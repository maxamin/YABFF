#!/usr/bin/env python3
"""Structured benchmark target for the ML A/B.

Unlike the flat `server.py`, this models a REST-ish app with *nested* resources
whose paths follow common conventions (`/api/v1/users/me`, ...). The point of the
A/B is that the nested convention tokens (`v1`, `v2`, `me`, `search`, ...) are
deliberately NOT in the benchmark wordlist (`ab-wordlist.txt`) — only the entry
directories are. So:

  * stock feroxbuster discovers the entry dirs and recurses, but cannot reach the
    nested resources (their path tokens aren't in the wordlist);
  * `feroxbuster --ml` fingerprints the target as REST_API and injects
    profile-seeded / online-learned predictions (`v1`, `users`, `me`, ...),
    reaching the nested chain.

This isolates the ML layer's contribution on the *same* wordlist. It is a
synthetic illustration of the mechanism, not a claim about real-world yield.
Fingerprint markers (/swagger-ui.html, /openapi.json, /api/v1) make the profile
unambiguous. Everything not listed returns 404.
"""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import sys

# Ground-truth discoverable paths. Entry dirs (also in ab-wordlist.txt) are marked
# [entry]; everything else is reachable only by predicting the next token.
VALID = {
    # fingerprint markers (probed by the ML layer, not scored as "nested")
    "swagger-ui.html": 200, "openapi.json": 200, "robots.txt": 200,
    # --- entry directories (present in ab-wordlist.txt) ---
    "api": 200, "api/": 200,                      # [entry]
    "admin": 301, "admin/": 200,                  # [entry]
    "users": 200, "users/": 200,                  # [entry]
    "blog": 200, "blog/": 200,                    # [entry]
    # --- nested, convention-following (NOT in the wordlist) ---
    "api/v1": 200, "api/v1/": 200,
    "api/v1/users": 200, "api/v1/users/": 200,
    "api/v1/users/me": 200,
    "api/v1/users/search": 200,
    "api/v1/users/login": 200,
    "api/v2": 200, "api/v2/": 200,
    "api/v2/users": 200,
    "admin/config": 200, "admin/login": 200, "admin/dashboard": 200,
    "users/me": 200, "users/login": 200, "users/profile": 200,
    "blog/archive": 200, "blog/search": 200,
}

# the subset that stock cannot reach from the wordlist alone (used for scoring)
NESTED = {p for p in VALID if p.count("/") >= 1 and not p.endswith("/")} - {
    "swagger-ui.html", "openapi.json", "robots.txt",
}


class H(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _handle(self, body=True):
        p = self.path.lstrip("/").split("?")[0]
        code = VALID.get(p)
        if code is None:
            self.send_response(404)
            self.send_header("Content-Length", "9")
            self.end_headers()
            if body:
                self.wfile.write(b"Not Found")
            return
        self.send_response(code)
        # a JSON-ish content-type on /api* nudges the REST_API fingerprint
        if p == "" or p.startswith("api"):
            self.send_header("Content-Type", "application/json")
        payload = (f'{{"ok":"{p}"}}').encode()
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        if body:
            self.wfile.write(payload)

    def do_GET(self):
        self._handle(True)

    def do_HEAD(self):
        self._handle(False)

    def do_POST(self):
        self._handle(True)


if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8001
    ThreadingHTTPServer(("127.0.0.1", port), H).serve_forever()
