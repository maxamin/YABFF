#!/usr/bin/env python3
"""Capture feroxbuster-ml's fingerprint probe against the live jsintel lab estate.

Writes two fixtures under `feroxbuster-ml/tests/fixtures/`:

* `jsintel_labs.json` — `{lab: [ {url, status, headers} ]}` for the discriminating
  probe paths (used by the fingerprint tests).
* `jsintel_labs_full.json` — `{lab: {probes:[...], random:[...], crawled:[...]}}`
  where each response also carries `content_length` / `word_count` / `line_count`
  (for E2 soft-404 scoring) and `random` holds probes to absent paths (the soft-404
  baseline). Used by the end-to-end `lab_features.rs` test that drives every engine
  function against the labs.

Re-run only when the labs change; needs the jsintel labs running on loopback.

    python3 scripts/probe_jsintel_labs.py
"""
import json
import os
import socket
import urllib.error
import urllib.request

LABS = {
    "juice-shop": "http://127.0.0.1:3000",
    "dvwa": "http://127.0.0.3:8081",
    "webgoat": "http://127.0.0.2:8082",
    "wordpress": "http://127.0.0.8:8083",
    "django": "http://127.0.0.11:8092",
}

# must mirror ferox_ml_core::profiles::PROBE_PATHS (plus "" for the root)
PROBE = ["", "wp-json", "wp-login.php", "actuator", "actuator/health", "api",
         "api/v1", "api/v2", "rest", "graphql", "swagger-ui.html", "openapi.json",
         "v2/api-docs", ".git/HEAD", "robots.txt", "xmlrpc.php", "index.php",
         "index.jsp", "server-status", "manifest.webmanifest", "_next", "assets",
         "static/admin"]

# random, almost-certainly-absent paths -> the server's soft-404 baseline (E2)
RANDOM = ["zz-absent-a1b2c3d4", "zz-absent-e5f6a7b8", "zz-absent-99x0y1z2"]

KEEP = {
    "content-type", "server", "x-powered-by", "set-cookie",
    # security headers → the sec_headers richer feature
    "content-security-policy", "strict-transport-security",
    "x-frame-options", "x-content-type-options",
}
FIXDIR = os.path.join(os.path.dirname(__file__), "..", "feroxbuster-ml", "tests", "fixtures")


def fetch(url):
    try:
        req = urllib.request.Request(url, method="GET", headers={"User-Agent": "ferox-probe"})
        with urllib.request.urlopen(req) as r:
            status, src, body = r.status, r.headers, r.read()
    except urllib.error.HTTPError as e:
        status, src, body = e.code, (e.headers or {}), (e.read() if hasattr(e, "read") else b"")
    except Exception:
        return {"status": 0, "headers": {}, "content_length": 0, "word_count": 0, "line_count": 0}
    try:
        text = body.decode("utf-8", "replace")
    except Exception:
        text = ""
    hdrs = {k.lower(): v for k, v in (src.items() if src else []) if k.lower() in KEEP}
    return {
        "status": status,
        "headers": hdrs,
        "content_length": len(body),
        "word_count": len(text.split()),
        "line_count": text.count("\n") + (1 if text else 0),
    }


def main():
    socket.setdefaulttimeout(4)
    simple = {}
    full = {}
    for name, base in LABS.items():
        probes = [{"url": f"{base}/{p}", **fetch(f"{base}/{p}")} for p in PROBE]
        random = [{"url": f"{base}/{p}", **fetch(f"{base}/{p}")} for p in RANDOM]
        answered = sum(1 for r in probes if r["status"])
        print(f"{name:12} answered={answered}/{len(PROBE)}")
        simple[name] = [{"url": r["url"], "status": r["status"], "headers": r["headers"]} for r in probes]
        full[name] = {"probes": probes, "random": random}
    with open(os.path.join(FIXDIR, "jsintel_labs.json"), "w") as fh:
        json.dump(simple, fh, indent=1)
    with open(os.path.join(FIXDIR, "jsintel_labs_full.json"), "w") as fh:
        json.dump(full, fh, indent=1)
    print(f"wrote jsintel_labs.json and jsintel_labs_full.json to {os.path.relpath(FIXDIR)}")


if __name__ == "__main__":
    main()
