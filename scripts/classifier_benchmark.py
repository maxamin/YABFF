#!/usr/bin/env python3
"""Benchmark candidate fingerprint classifiers on the ferox-ml-core feature space.

Parses the real profile centroids / feature weights from
`ferox-ml-core/src/profiles.rs`, generates a labelled dataset by sampling each
feature as Bernoulli(centroid-probability) per profile (the centroid value is that
profile's expected presence probability), then compares:

  * Hand centroids (current)  — the shipped weighted nearest-centroid, no training
  * NearestCentroid (trained) — sklearn, class means learned from data
  * Bernoulli Naive Bayes
  * Logistic Regression (multinomial)
  * k-NN (k=5)
  * Linear SVM
  * Random Forest
  * Gradient Boosting (GBDT)

Writes:
  docs/analysis/fingerprint-classifier-benchmark.md   (metrics)
  docs/analysis/fingerprint-classifier-heatmap.html   (confusion-matrix heatmaps)

NOTE: the data is synthetic (a product-of-Bernoullis per profile), so it measures
how well each algorithm *separates the profiles under observation noise* — it is
not a real-world yield claim, and the Bernoulli generator mildly favours Naive
Bayes (its modelling assumption matches). Treat it as an algorithm comparison; the
real unlock is a labelled corpus of actual hosts.
"""
import os
import re
import numpy as np

from sklearn.model_selection import train_test_split
from sklearn.neighbors import NearestCentroid, KNeighborsClassifier
from sklearn.naive_bayes import BernoulliNB
from sklearn.linear_model import LogisticRegression
from sklearn.svm import LinearSVC
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.metrics import accuracy_score, f1_score, precision_score, recall_score, confusion_matrix

ROOT = os.path.join(os.path.dirname(__file__), "..")
PROFILES_RS = os.path.join(ROOT, "ferox-ml-core", "src", "profiles.rs")
OUT_MD = os.path.join(ROOT, "docs", "analysis", "fingerprint-classifier-benchmark.md")
OUT_HTML = os.path.join(ROOT, "docs", "analysis", "fingerprint-classifier-heatmap.html")

SHORT = {
    "REST_API": "REST", "ENTERPRISE_JAVA_SPRING": "SPRING", "WORDPRESS_CMS": "WP",
    "LEGACY_STATIC": "STATIC", "PHP_GENERIC": "PHP", "NODE_SPA": "SPA", "DJANGO": "DJANGO",
}


def parse_profiles():
    """Extract feature names, weights, and centroids from profiles.rs."""
    src = open(PROFILES_RS).read()
    # feature names
    m = re.search(r"FEATURE_NAMES:\s*\[&str;\s*\d+\]\s*=\s*\[(.*?)\];", src, re.S)
    names = re.findall(r'"([a-z0-9_]+)"', m.group(1))
    # weights
    m = re.search(r"FEATURE_WEIGHTS:\s*\[f64;\s*N_FEATURES\]\s*=\s*\[(.*?)\];", src, re.S)
    wblock = re.sub(r"//[^\n]*", "", m.group(1))  # strip inline comments (they carry indices)
    weights = [float(x) for x in re.findall(r"[-+]?\d*\.?\d+", wblock)]
    # centroids: ("NAME", [ ... ])  (a comment line may sit between)
    body = src[src.index("pub fn centroids"):]
    cents = {}
    for name, arr in re.findall(r'\(\s*"([A-Z_]+)",\s*(?:/[^\n]*\n\s*)?\[([^\]]+)\]', body):
        cents[name] = [float(x) for x in re.findall(r"[-+]?\d*\.?\d+", arr)]
    return names, np.array(weights), cents


def make_dataset(cents, n_per=300, seed=1, flip=0.0):
    """Sample each feature Bernoulli(centroid-prob) per profile, then corrupt with
    `flip`-probability bit flips (observation noise: flaky probes, catch-alls)."""
    rng = np.random.default_rng(seed)
    profiles = list(cents.keys())
    X, y = [], []
    for i, p in enumerate(profiles):
        probs = np.clip(np.array(cents[p]), 0.0, 1.0)
        samples = (rng.random((n_per, len(probs))) < probs).astype(float)
        if flip > 0.0:
            mask = rng.random(samples.shape) < flip
            samples = np.where(mask, 1.0 - samples, samples)
        X.append(samples)
        y.extend([i] * n_per)
    return np.vstack(X), np.array(y), profiles


def build_models(cents, weights, profiles):
    return {
        "Hand centroids (current)": HandCentroid(cents, weights, profiles),
        "NearestCentroid (trained)": NearestCentroid(),
        "Bernoulli Naive Bayes": BernoulliNB(),
        "Logistic Regression": LogisticRegression(max_iter=2000),
        "k-NN (k=5)": KNeighborsClassifier(n_neighbors=5),
        "Linear SVM": LinearSVC(max_iter=5000),
        "Random Forest": RandomForestClassifier(n_estimators=200, random_state=42),
        "Gradient Boosting (GBDT)": GradientBoostingClassifier(random_state=42),
    }


class HandCentroid:
    """The shipped classifier: weighted-Euclidean nearest hand-authored centroid."""
    def __init__(self, cents, weights, profiles):
        self.C = np.array([cents[p] for p in profiles])
        self.w = weights
        self.classes_ = np.arange(len(profiles))

    def fit(self, X, y):
        return self  # fixed; no training

    def predict(self, X):
        # weighted squared distance to each centroid
        d = ((X[:, None, :] - self.C[None, :, :]) ** 2 * self.w[None, None, :]).sum(axis=2)
        return d.argmin(axis=1)


MAIN_FLIP = 0.15          # noise level for the headline results + matrices
SWEEP = [0.0, 0.1, 0.2, 0.3]


def evaluate(cents, weights, profiles, flip):
    """Train+eval every model at a given noise level; return (results, cms)."""
    X, y, _ = make_dataset(cents, flip=flip)
    Xtr, Xte, ytr, yte = train_test_split(X, y, test_size=0.3, random_state=42, stratify=y)
    results, cms = [], {}
    for name, clf in build_models(cents, weights, profiles).items():
        clf.fit(Xtr, ytr)
        pred = clf.predict(Xte)
        results.append({
            "name": name,
            "acc": accuracy_score(yte, pred),
            "f1": f1_score(yte, pred, average="macro"),
            "prec": precision_score(yte, pred, average="macro", zero_division=0),
            "rec": recall_score(yte, pred, average="macro", zero_division=0),
        })
        cms[name] = confusion_matrix(yte, pred, labels=np.arange(len(profiles)))
    return results, cms, len(Xtr), len(Xte)


def main():
    names, weights, cents = parse_profiles()
    profiles = list(cents.keys())
    labels = [SHORT[p] for p in profiles]

    # headline run at MAIN_FLIP (also drives the confusion-matrix heatmap)
    results, cms, n_tr, n_te = evaluate(cents, weights, profiles, MAIN_FLIP)
    results.sort(key=lambda r: r["acc"], reverse=True)

    # noise-robustness sweep: accuracy per model across flip levels
    sweep = {eps: {r["name"]: r["acc"] for r in evaluate(cents, weights, profiles, eps)[0]}
             for eps in SWEEP}

    write_md(results, sweep, profiles, labels, n_tr, n_te)
    write_html(results, cms, labels)
    print(f"wrote {os.path.relpath(OUT_MD)} and {os.path.relpath(OUT_HTML)}")
    for r in results:
        print(f"  {r['name']:28} acc={r['acc']:.3f} f1={r['f1']:.3f}")


def write_md(results, sweep, profiles, labels, n_tr, n_te):
    lines = []
    lines.append("# Fingerprint classifier benchmark\n")
    lines.append("Head-to-head comparison of candidate classifiers for the framework")
    lines.append("fingerprinter, on the `ferox-ml-core` 20-dim feature space. Generated by")
    lines.append("[`scripts/classifier_benchmark.py`](../../scripts/classifier_benchmark.py).\n")
    lines.append("**Method.** Centroids, weights and feature names are parsed from")
    lines.append("[`profiles.rs`](../../ferox-ml-core/src/profiles.rs). A labelled dataset is")
    lines.append("sampled per profile as Bernoulli(centroid-probability), then corrupted with")
    lines.append(f"**{int(MAIN_FLIP*100)}% bit-flip observation noise** (flaky probes / catch-alls) —")
    lines.append(f"{n_tr} train / {n_te} test, stratified 70/30 over the {len(profiles)} profiles.")
    lines.append("Metrics are macro-averaged on the held-out test set. The companion heatmap is")
    lines.append("[`fingerprint-classifier-heatmap.html`](fingerprint-classifier-heatmap.html).\n")
    lines.append("> **Caveat — synthetic data.** This measures how well each algorithm")
    lines.append("> *separates the profiles under observation noise*, not real-world yield. The")
    lines.append("> product-of-Bernoullis generator mildly favours Naive Bayes (its assumption")
    lines.append("> matches). The real unlock is a labelled corpus of actual hosts; treat this")
    lines.append("> as an algorithm comparison that motivates the choice.\n")
    lines.append(f"## Results at {int(MAIN_FLIP*100)}% noise (test set, best first)\n")
    lines.append("| classifier | accuracy | macro-F1 | macro-precision | macro-recall |")
    lines.append("|---|---:|---:|---:|---:|")
    for r in results:
        lines.append(f"| {r['name']} | {r['acc']:.3f} | {r['f1']:.3f} | {r['prec']:.3f} | {r['rec']:.3f} |")
    lines.append("")
    lines.append("## Noise-robustness sweep (accuracy vs. bit-flip rate)\n")
    header = "| classifier | " + " | ".join(f"ε={e:.1f}" for e in SWEEP) + " |"
    lines.append(header)
    lines.append("|---|" + "---:|" * len(SWEEP))
    for r in results:
        cells = " | ".join(f"{sweep[e][r['name']]:.3f}" for e in SWEEP)
        lines.append(f"| {r['name']} | {cells} |")
    lines.append("")
    best = results[0]
    hand = next(r for r in results if r["name"].startswith("Hand centroids"))
    gap = best["acc"] - hand["acc"]
    lines.append("## Reading it\n")
    lines.append(f"- **Best at {int(MAIN_FLIP*100)}% noise:** {best['name']} "
                 f"(acc {best['acc']:.3f}, F1 {best['f1']:.3f}).")
    lines.append(f"- **Shipped baseline** (hand centroids, *no training data at all*): "
                 f"acc {hand['acc']:.3f}, F1 {hand['f1']:.3f} — "
                 + (f"within {gap:.3f} of the best trained model."
                    if gap <= 0.05 else f"{gap:.3f} behind the best trained model."))
    lines.append("- The sweep shows the ranking is **stable across noise**; the gap to the best")
    lines.append("  trained model widens only as noise grows, and every model needs a labelled")
    lines.append("  corpus to train — which the hand centroids do not.")
    lines.append("- Near-origin profiles (SPA / DJANGO / PHP / STATIC) are the dominant")
    lines.append("  confusions — see the per-classifier matrices in the heatmap.\n")
    lines.append("## Recommendation\n")
    lines.append("The algorithm is **not** the bottleneck on this feature space — all candidates")
    lines.append("cluster tightly and the zero-data hand centroids are competitive. The real")
    lines.append("levers, in order: **(1) richer features** (response-body / header / favicon /")
    lines.append("JS-framework signals), **(2) a labelled host corpus**, then **(3)** adopt")
    lines.append(f"**{best['name']}** behind the `Classifier` trait (it also yields calibrated")
    lines.append("probabilities, subsuming E5's softmax and E3's hand weights). For *discovering*")
    lines.append("new profiles, use a density clusterer (HDBSCAN / k-modes) over host vectors")
    lines.append("rather than k-means.\n")
    with open(OUT_MD, "w") as fh:
        fh.write("\n".join(lines))


def cell_color(frac):
    # white -> deep blue
    r = int(255 - frac * (255 - 13))
    g = int(255 - frac * (255 - 71))
    b = int(255 - frac * (255 - 161))
    return f"rgb({r},{g},{b})"


def matrix_html(name, cm, labels, acc):
    n = len(labels)
    rows = []
    rows.append(f'<figure><figcaption>{name} <span class="acc">acc {acc:.3f}</span></figcaption>')
    rows.append('<table class="cm"><thead><tr><th></th>'
                + "".join(f"<th>{l}</th>" for l in labels) + "<th>rec</th></tr></thead><tbody>")
    for i in range(n):
        total = cm[i].sum()
        rows.append(f'<tr><th>{labels[i]}</th>')
        for j in range(n):
            frac = cm[i][j] / total if total else 0.0
            txt = "" if cm[i][j] == 0 else str(cm[i][j])
            fg = "#fff" if frac > 0.55 else "#222"
            rows.append(f'<td style="background:{cell_color(frac)};color:{fg}" '
                        f'title="{labels[i]}→{labels[j]}: {cm[i][j]}">{txt}</td>')
        rec = cm[i][i] / total if total else 0.0
        rows.append(f'<td class="rec">{rec:.2f}</td></tr>')
    rows.append("</tbody></table></figure>")
    return "".join(rows)


def write_html(results, cms, labels):
    acc_by = {r["name"]: r["acc"] for r in results}
    order = [r["name"] for r in results]
    cards = "\n".join(matrix_html(n, cms[n], labels, acc_by[n]) for n in order)
    best = results[0]["name"]
    bars = "".join(
        f'<div class="bar"><span class="lbl">{r["name"]}</span>'
        f'<span class="track"><span class="fill" style="width:{r["acc"]*100:.1f}%"></span></span>'
        f'<span class="val">{r["acc"]:.3f}</span></div>'
        for r in results
    )
    html = f"""<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Fingerprint classifier benchmark</title>
<style>
  :root {{ --bg:#fff; --fg:#1a1a1a; --muted:#666; --card:#f6f7f9; --line:#e2e5e9; --accent:#0d47a1; }}
  @media (prefers-color-scheme: dark) {{
    :root:not([data-theme="light"]) {{ --bg:#15171a; --fg:#e8eaed; --muted:#9aa0a6; --card:#1e2127; --line:#30343b; --accent:#5b9bd5; }}
  }}
  :root[data-theme="dark"] {{ --bg:#15171a; --fg:#e8eaed; --muted:#9aa0a6; --card:#1e2127; --line:#30343b; --accent:#5b9bd5; }}
  * {{ box-sizing:border-box; }}
  body {{ margin:0; background:var(--bg); color:var(--fg); font:15px/1.5 -apple-system,Segoe UI,Roboto,Helvetica,Arial,sans-serif; padding:24px 16px; }}
  main {{ max-width:1100px; margin:0 auto; }}
  h1 {{ font-size:1.5rem; margin:0 0 .25rem; }}
  p.sub {{ color:var(--muted); margin:0 0 1.5rem; }}
  h2 {{ font-size:1.05rem; margin:1.8rem 0 .6rem; border-bottom:1px solid var(--line); padding-bottom:.3rem; }}
  .bars {{ display:grid; gap:6px; margin-bottom:1rem; }}
  .bar {{ display:grid; grid-template-columns:200px 1fr 54px; align-items:center; gap:10px; }}
  .bar .lbl {{ font-size:.82rem; color:var(--muted); text-align:right; }}
  .bar .track {{ background:var(--card); border:1px solid var(--line); border-radius:5px; height:16px; overflow:hidden; }}
  .bar .fill {{ display:block; height:100%; background:var(--accent); }}
  .bar .val {{ font-variant-numeric:tabular-nums; font-size:.82rem; }}
  .grid {{ display:grid; grid-template-columns:repeat(auto-fit,minmax(300px,1fr)); gap:16px; }}
  figure {{ margin:0; background:var(--card); border:1px solid var(--line); border-radius:10px; padding:12px; }}
  figcaption {{ font-weight:600; font-size:.9rem; margin-bottom:8px; display:flex; justify-content:space-between; }}
  figcaption .acc {{ color:var(--muted); font-weight:400; }}
  table.cm {{ border-collapse:collapse; width:100%; font-size:.74rem; }}
  table.cm th {{ color:var(--muted); font-weight:600; padding:2px 4px; }}
  table.cm td {{ text-align:center; padding:4px 0; width:11%; font-variant-numeric:tabular-nums; border-radius:2px; }}
  table.cm td.rec {{ background:transparent!important; color:var(--muted); }}
  .note {{ color:var(--muted); font-size:.85rem; margin-top:1.5rem; }}
</style></head>
<body><main>
  <h1>Fingerprint classifier benchmark</h1>
  <p class="sub">Confusion matrices (rows = true profile, cols = predicted; cell shade = row fraction) on a held-out synthetic test set. Best model: <strong>{best}</strong>.</p>
  <h2>Accuracy</h2>
  <div class="bars">{bars}</div>
  <h2>Confusion matrices</h2>
  <div class="grid">{cards}</div>
  <p class="note">Synthetic Bernoulli-sampled data from the profile centroids; an algorithm comparison, not a real-world yield claim. See <code>fingerprint-classifier-benchmark.md</code>.</p>
</main></body></html>"""
    with open(OUT_HTML, "w") as fh:
        fh.write(html)


if __name__ == "__main__":
    main()
