# Prompt — Adaptive ML-Driven Fuzzing Orchestrator for feroxbuster

## Role

You are a senior offensive-security tooling engineer and applied ML engineer. You write clean, production-grade Python 3.11+ that other professionals will run in real engagements. You prefer correct, verifiable designs over impressive-sounding ones, and you never assume a dependency behaves a certain way without matching its real interface.

## Objective

Build **`feroxml`**, a Python orchestration layer that wraps the real `feroxbuster` binary and turns it into an adaptive, feedback-driven fuzzer using two cooperating engines:

1. a **K-Means target-fingerprinting engine** (scikit-learn) that classifies the target into a framework profile, and
2. a **Markov-chain path predictor** that, from each discovered path, proposes the most probable next path tokens,

wired together by an **orchestrator** that consumes feroxbuster's live NDJSON output and launches follow-up scans seeded with the predicted paths.

Deliver a fully realized, modular implementation — no placeholders, no `TODO`s, no stubbed functions. Every profile's seed data (feature centroids and transition matrices) must be written out in full so the tool runs end to end on first launch.

## Ground truth about feroxbuster — build to these facts, not assumptions

Before writing code, honor how feroxbuster **actually** works (verify against `feroxbuster --help` and its NDJSON output; do not invent flags):

- **Output:** `--json` emits **newline-delimited JSON (NDJSON)**, one object per line, each tagged by a `"type"` field. The types are `configuration`, `response`, `statistics`, and `log`. Parse **only `type == "response"`** objects for findings. Their relevant fields are: `url`, `path`, `status` (int), `content_length`, `line_count`, `word_count`, `method`, and `headers` (object). Write output to a file with `-o <file> --json` or read it from the process's stdout; parse **line by line** and tolerate partial/interleaved lines.
- **Directory vs. file:** treat a `response` as a directory worth expanding when `status` is 2xx and `url` ends in `/`, or when `status` is 3xx with a `location` header pointing to `url + "/"` (this mirrors feroxbuster's own `is_directory` logic).
- **CRITICAL feedback limitation:** feroxbuster does **not** support injecting new wordlist entries into a *running* scan. Its `--stdin` is read **once at startup** as a list of target **URLs** (not words), and the interactive menu's "add URL" is manual, not programmatic. **Therefore the adaptive loop must work by spawning new, short feroxbuster scans** (a round-based feedback loop), each targeting a newly discovered directory with a freshly generated, Markov-predicted wordlist (`-u <discovered_dir> -w <predicted_wordlist>`), or by feeding a batch of predicted **full URLs** into a new `feroxbuster --stdin` process. Do **not** design around live stdin word injection — it will not work. State this limitation in the code's module docstring.
- **Useful flags to rely on:** `-u`, `--stdin`, `-w`, `-x/--extensions`, `-s/--status-codes`, `-o`, `--json`, `-t/--threads`, `--rate-limit`, `-d/--depth`, `-n/--no-recursion`, `-q/--quiet`, `--time-limit`, `-k/--insecure`, `-H/--headers`, `--dont-scan`. Use `--rate-limit` and `-t` for safety. Use `-n` on prediction-round scans so your orchestrator controls recursion rather than feroxbuster.

## Architecture & required modules

Produce a package with this layout (each file complete and runnable):

```
feroxml/
  __init__.py
  config.py          # dataclasses + YAML/CLI loading of all hyperparameters
  profiles.py        # the 4 framework profiles: centroids + seed transition matrices (FULL data)
  fingerprint.py     # Phase 1: probe + feature vector + KMeans classifier
  markov.py          # Phase 2: MarkovTransitionMatrix + top-N prediction
  orchestrator.py    # Phase 3: subprocess control, NDJSON stream parsing, feedback loop
  wordlists.py       # generate per-target predicted wordlists; dedupe across rounds
  cli.py             # argparse entry point (`python -m feroxml ...`)
tests/
  test_fingerprint.py
  test_markov.py
  test_orchestrator.py
requirements.txt
README.md
```

### Phase 1 — Target Clustering Engine (`fingerprint.py`)
- Run a **rapid, bounded probe** (a small fixed set of ~15–25 discriminating requests using `requests` or `httpx`, with timeout, retry, and TLS-verify toggle) against the target root.
- Build a **numerical feature vector** from observable signals, each documented. Include at minimum: presence/absence (1/0) of `wp-json`, `wp-login.php`, `/actuator`, `/actuator/health`, `/api`, `/api/v1`, `/swagger`/`/openapi.json`, `/.git/HEAD`, `/robots.txt`; response signals for the root (status class, has `Content-Type: application/json`, presence of `X-Powered-By`, `Server` family, `Set-Cookie` names like `JSESSIONID`/`wordpress_`/`PHPSESSID`); and counts of discovered static extensions (`.php`, `.jsp`, `.html`, `.aspx`).
- Classify into exactly one of four **profiles**: `REST_API`, `ENTERPRISE_JAVA_SPRING`, `WORDPRESS_CMS`, `LEGACY_STATIC`.
- Use `sklearn.cluster.KMeans` **seeded with the four profile centroids** defined in `profiles.py` (`init=<centroids array>`, `n_clusters=4`, `n_init=1`, fixed `random_state`) so classification is deterministic and each cluster maps to a known profile — then assign the target to the nearest centroid. Also expose a pure-NumPy nearest-centroid fallback used automatically when scikit-learn is unavailable, so the tool degrades gracefully.
- Return the profile plus the distances to all four centroids (a confidence signal).

### Phase 2 — Markov Chain State Predictor (`markov.py`)
- Implement `class MarkovTransitionMatrix` storing `P(next_token | current_token)` as a nested dict or sparse structure, with `add_transition`, `fit(paths)`, `normalize`, `predict_next(token, top_n)`, `save`/`load` (JSON), and Laplace smoothing controlled by a hyperparameter.
- In `profiles.py`, provide **complete, structurally accurate seed matrices per profile** — real tokens with sensible weights, e.g. `REST_API`: `api → {v1: .4, v2: .25, v3: .1, users: .1, ...}`, `v1 → {users: .3, auth: .2, health: .1, ...}`; `WORDPRESS_CMS`: `wp-content → {plugins: .5, themes: .3, uploads: .2}`, `wp-json → {wp: .6, ...}`; `ENTERPRISE_JAVA_SPRING`: `actuator → {health: .3, env: .2, mappings: .15, ...}`; `LEGACY_STATIC`: `admin → {login: .3, index.php: .2, ...}`. Fill every profile out fully, not one example each.
- `predict_next` takes the **last path segment** of a discovered URL, looks it up, and returns the top-N next tokens by probability, above a configurable probability threshold; when the token is unknown, fall back to the profile's global high-frequency tokens.
- Continuously **learn online**: every discovered path updates the live matrix so predictions improve during the run.

### Phase 3 — Dynamic Interaction Layer (`orchestrator.py`)
- Launch feroxbuster via `subprocess.Popen` with `--json`, stream and parse NDJSON **line by line** from stdout as it arrives (non-blocking read; never `communicate()` on an unbounded stream).
- On each `response` with a success-class status (configurable set, default `{200, 204, 301, 302, 307, 401, 403, 405}`), parse the URL, feed the last segment to the Markov engine, generate top-N predicted child paths, and **enqueue a follow-up scan round**: build a per-target predicted wordlist (`wordlists.py`) and spawn a bounded feroxbuster scan (`-u <dir> -w <predicted> -n`). Maintain a global **seen-URL set** to prevent re-scanning and infinite loops, and a **max-rounds / max-depth / global-request budget** guard.
- Run rounds concurrently up to a worker cap; drain all child processes cleanly on completion or interrupt.

## Non-functional requirements

- **Safety/authorization:** print and log a scope banner; refuse to run unless the operator passes `--i-have-authorization` (or a scope file). Enforce an in-scope host/domain check on every generated URL and honor `--rate-limit`. Never scan hosts outside the provided scope.
- **Error handling:** detect a missing `feroxbuster` binary (check `shutil.which`) and missing Python deps with actionable messages; handle `KeyboardInterrupt`/`SIGINT` by terminating all children and flushing state; tolerate malformed JSON lines; time out and reap zombie processes.
- **Determinism & persistence:** fixed random seeds; `save`/`load` for both the fitted KMeans state and the Markov matrix (resume a campaign).
- **Config & hyperparameters:** expose via `config.py` dataclass + YAML + CLI overrides — at least: `top_n`, `probability_threshold`, `laplace_alpha`, `max_rounds`, `max_depth`, `request_budget`, `threads`, `rate_limit`, `success_codes`, `probe_timeout`, `kmeans_random_state`, `tls_verify`, `extensions`.
- **Logging:** structured logging (`logging` module) with levels; a concise run summary at the end (profile chosen, rounds run, paths discovered, predictions that hit).
- **Tests:** unit tests for the feature vectorizer, the KMeans/nearest-centroid classifier (assert each seed centroid classifies to its own profile), the Markov normalization and `predict_next` ordering, and an orchestrator test that parses a **captured sample NDJSON fixture** (provide the fixture) without needing a live target.
- **Docs:** `README.md` with install, an authorized-use warning, a quickstart, a diagram of the two-engine feedback loop, and an explicit note about the feroxbuster live-injection limitation and why the design uses round-based re-scanning.

## Constraints

- Clean, typed (`from __future__ import annotations`, type hints throughout), PEP 8, docstrings on public APIs.
- Standard, current libraries only: `scikit-learn`, `numpy`, `requests` or `httpx`, `pyyaml`; standard library for everything else. Pin versions in `requirements.txt`.
- No placeholders, no pseudo-code, no "fill this in later." Every profile's centroid vector and transition matrix must be present and internally consistent so the tool is functional immediately.
- Do not fabricate feroxbuster flags or behaviors; if unsure, choose the mechanism consistent with the ground-truth section above.

## Advanced algorithms (v2 upgrade path)

Design the engines from the start with clean interfaces (a `Predictor` protocol, a `Classifier` protocol, a `Scheduler` protocol) so the components below can be swapped in without rewriting the orchestrator. Implement the **Tier 1** items now; scaffold **Tier 2/3** behind the same interfaces with feature flags in `config.py` (default off) so they are drop-in later. Order the roadmap by value ÷ effort, and note in the README that in real engagements the bottleneck is network round-trips and target rate-limits — so spending *fewer, better-ordered* requests (the scheduler and ranker) beats a fancier generator.

### Tier 1 — implement now (high value, low cost)
- **Subword tokenization (BPE/WordPiece)** for paths, so the predictor generalizes across `user` / `users` / `userId` instead of treating them as unrelated tokens. Apply it everywhere a path is split into tokens.
- **Variable-order Markov / PPM (prediction by partial matching)** to replace the fixed 1st-order chain: use the longest context with evidence (`api/v1/` not just `v1`), backing off to shorter contexts, with Kneser–Ney (or at least proper back-off) smoothing instead of flat Laplace. Keep the 1st-order model as the fallback level.
- **Labeled nearest-centroid classifier** as the primary Phase-1 path (the four profiles are *known*, so this is more correct than unsupervised K-Means). Keep the seeded-KMeans route as an alternative, selectable in config; both must classify each seed centroid to its own profile.
- **Thompson-Sampling multi-armed bandit** as the request scheduler — the single highest-value ML upgrade. Model each directory (or wordlist segment) as an arm with a Beta(hits, misses) posterior; sample to choose which branch gets the next scan round. This replaces naive round-robin expansion and directly maximizes hits-per-request under the global request budget. Expose `scheduler: {round_robin | thompson | ucb1}` in config.
- **BM25 (or TF-IDF) wordlist ranking** so each generated candidate list is *ordered* by relevance to the target's observed vocabulary before it is handed to feroxbuster; truncate to the request budget by rank.
- **SimHash / MinHash + LSH soft-404 and near-duplicate detection** on response bodies to drop template/noise responses before they pollute the bandit's reward signal. (feroxbuster itself uses SimHash — mirror its approach; `--filter-similar-to` can assist.)

### Tier 2 — scaffold behind interfaces (higher effort, needs data)
- **Contextual bandit (LinUCB)** that uses the target's Phase-1 feature vector as context, so a learned budgeting policy transfers across targets.
- **Gradient-boosted-tree fingerprinting (LightGBM/XGBoost)** to replace centroid classification once labeled site data exists; surface feature importances so the operator sees which signals discriminate. Ship a small pretrained model artifact or a training script.
- **Gaussian Mixture Model** for *soft* profile membership (e.g. 70% Spring / 30% REST), then **blend the corresponding Markov matrices** weighted by membership instead of committing to one profile.
- **Learning-to-Rank (LambdaMART)** for wordlist ordering, trained on "which paths hit on sites like this."
- **Isolation Forest / One-Class SVM** anomaly detection to flag the single *interesting* outlier response among thousands, distinct from simple dedup.

### Tier 3 — research / optional (highest effort, diminishing returns)
- **Monte Carlo Tree Search (UCT)** over the path tree, balancing exploration of new branches against exploitation of productive ones — an elegant fit for recursive discovery and a natural generalization of the bandit.
- **Pretrained char-level LSTM or small Transformer** trained on SecLists to *generate* novel plausible paths (learns morphology), shipped as a model artifact so no training is needed at run time. Gate behind an optional extra dependency; fall back to the Markov/PPM engine when unavailable.
- **HDBSCAN** to discover *new* framework clusters beyond the predefined four (e.g. GraphQL, headless CMS) and propose adding them as profiles.
- **Reinforcement learning (Q-learning / PPO)** modeling the crawl as an MDP — document it as a possibility but note it is usually beaten by bandits/MCTS here for far more effort.

### Requirements for every advanced component
- Each lives behind its protocol and is selected via `config.py`; the default configuration must run using only Tier 1.
- Anything needing extra/heavy libraries (LightGBM, torch, hdbscan) goes in an optional `requirements-advanced.txt` and imports lazily, with a graceful fallback to the Tier 1 equivalent when the dependency is absent — never a hard crash.
- Every swap-in must keep determinism (fixed seeds) and the offline-testability requirement: add a unit test per component that runs without a live target.

## Acceptance criteria (the implementation is done when)

1. `python -m feroxml --url https://TARGET --i-have-authorization` runs a probe, prints the detected profile with centroid distances, then runs the adaptive feedback loop and prints a final summary.
2. All unit tests pass offline (no live target required), including the NDJSON-fixture orchestrator test.
3. Each of the four seed centroids classifies to its own profile, and each profile's Markov matrix rows sum to 1.0 after normalization.
4. Missing binary/deps and Ctrl-C are all handled with clear messages and clean shutdown.
5. Every generated URL is scope-checked; out-of-scope candidates are dropped and logged.
6. Tier 1 advanced components (subword tokenization, variable-order Markov/PPM, nearest-centroid classifier, Thompson-Sampling scheduler, BM25 ranking, SimHash soft-404 filter) are implemented, selectable via config, and each has an offline unit test; Tier 2/3 components are scaffolded behind their protocols and default off.
