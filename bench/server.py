#!/usr/bin/env python3
"""Local benchmark target. Serves a known set of hidden paths so we can score
each fuzzer on discovery accuracy and speed. Everything else returns 404."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import sys

# Ground-truth set of discoverable paths (status shown in parentheses)
VALID = {
    "admin": 301, "admin/": 200, "login": 200, "config": 200,
    "backup": 200, "api": 200, "api/": 200, "robots.txt": 200,
    "dashboard": 200, "uploads": 301, "uploads/": 200, "test": 200,
    "hidden": 200, "secret": 403, "server-status": 403, "index.php": 200,
    "config.php": 200, "db.php": 200, "old": 301, "old/": 200,
}

class H(BaseHTTPRequestHandler):
    def log_message(self, *a):  # silence per-request logging
        pass
    def _handle(self, body=True):
        p = self.path.lstrip("/").split("?")[0]
        code = VALID.get(p)
        if code is None:
            self.send_response(404); self.send_header("Content-Length", "9")
            self.end_headers()
            if body: self.wfile.write(b"Not Found")
            return
        self.send_response(code)
        payload = (f"OK {p}").encode()
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        if body: self.wfile.write(payload)
    def do_GET(self):  self._handle(True)
    def do_HEAD(self): self._handle(False)
    def do_POST(self): self._handle(True)

if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    ThreadingHTTPServer(("127.0.0.1", port), H).serve_forever()
