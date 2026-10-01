#!/usr/bin/env python3
"""Capture feroxbuster-ml's fingerprint probe against the live jsintel lab estate.

Replays `feroxbuster::ml::probe_paths()` against each lab and writes the responses
(status + the headers the fingerprinter reads) to
`feroxbuster-ml/tests/fixtures/jsintel_labs.json`, which the deterministic
`tests/lab_fingerprint.rs` replays. Re-run this only when the labs change; it
needs the jsintel labs running on the loopback estate.

    python3 scripts/probe_jsintel_labs.py
"""
import json
import os
import socket
import urllib.error
import urllib.request

# jsintel's web_default_labs (tests/lab/labs_index.py)
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
         "index.jsp", "server-status"]

# headers the fingerprint feature-vector consults
KEEP = {"content-type", "server", "x-powered-by", "set-cookie"}

FIXTURE = os.path.join(os.path.dirname(__file__), "..", "feroxbuster-ml",
                       "tests", "fixtures", "jsintel_labs.json")


def probe(base):
    out = []
    for p in PROBE:
        url = f"{base}/{p}"
        try:
            req = urllib.request.Request(url, method="GET",
                                         headers={"User-Agent": "ferox-probe"})
            with urllib.request.urlopen(req) as r:
                status, src = r.status, r.headers
        except urllib.error.HTTPError as e:
            status, src = e.code, (e.headers or {})
        except Exception:
            status, src = 0, {}
        hdrs = {k.lower(): v for k, v in (src.items() if src else [])
                if k.lower() in KEEP}
        out.append({"url": url, "status": status, "headers": hdrs})
    return out


def main():
    socket.setdefaulttimeout(4)
    data = {}
    for name, base in LABS.items():
        resps = probe(base)
        answered = sum(1 for r in resps if r["status"])
        print(f"{name:12} answered={answered}/{len(PROBE)}")
        data[name] = resps
    with open(FIXTURE, "w") as fh:
        json.dump(data, fh, indent=1)
    print(f"wrote {os.path.relpath(FIXTURE)}")


if __name__ == "__main__":
    main()
