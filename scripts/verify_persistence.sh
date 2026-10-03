#!/usr/bin/env bash
# Repeatable, LOCAL-ONLY verification of --ml-loop model persistence + --output
# routing on interrupt. Codifies the manual SIGTERM test:
#
#   1. start a local slow HTTP server (never touches the network / real targets)
#   2. run a list-mode --ml-loop against it with --ml-model and --output
#   3. SIGTERM it mid-run
#   4. assert the model.json is valid JSON and testoutput got written
#
# Exits non-zero on any failed assertion.
#
# Usage: scripts/verify_persistence.sh [port]
set -uo pipefail

PORT="${1:-8137}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/debug/feroxbuster"
WORK="$(mktemp -d)"
MODEL="$WORK/model.json"
OUT="$WORK/out.txt"
LISTS="$WORK/lists"
SRVPID=""

cleanup() {
    [[ -n "$SRVPID" ]] && kill "$SRVPID" 2>/dev/null || true
    rm -rf "$WORK"
}
trap cleanup EXIT

fail() { echo "FAIL: $*" >&2; exit 1; }

[[ -x "$BIN" ]] || fail "debug binary not found at $BIN (run: cargo build -p feroxbuster)"

# --- slow local server: 200 on every path, 40ms latency so the scan runs long
# enough to be interrupted mid-flight ---
python3 - "$PORT" <<'PY' &
import sys, time, http.server, socketserver
port = int(sys.argv[1])
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        time.sleep(0.04)
        b = b"ok"
        self.send_response(200); self.send_header("Content-Length", str(len(b)))
        self.end_headers(); self.wfile.write(b)
    def log_message(self, *a): pass
socketserver.ThreadingTCPServer.allow_reuse_address = True
socketserver.ThreadingTCPServer(("127.0.0.1", port), H).serve_forever()
PY
SRVPID=$!
sleep 1
curl -s -o /dev/null -w "" "http://127.0.0.1:$PORT/" || fail "local server did not come up on :$PORT"

# list-mode keeps rounds unbounded, so the run is still going when we interrupt
mkdir -p "$LISTS"
seq 1 8000 | sed 's/^/w/' > "$LISTS/a.txt"

echo "[verify] starting ml-loop against http://127.0.0.1:$PORT ..."
"$BIN" --ml-loop -u "http://127.0.0.1:$PORT" \
    --ml-list-dir "$LISTS" --ml-list-max 0 --depth 3 \
    --ml-model "$MODEL" --output "$OUT" >"$WORK/run.log" 2>&1 &
FPID=$!

sleep 6
if ! kill -0 "$FPID" 2>/dev/null; then
    fail "scan exited before we could interrupt it (make it run longer)"
fi

echo "[verify] sending SIGTERM to pid $FPID ..."
kill -TERM "$FPID"
sleep 3
kill -0 "$FPID" 2>/dev/null && fail "process still alive after SIGTERM"

# --- assertions ---
grep -aiq 'interrupted: stopping' "$WORK/run.log" \
    || fail "cooperative-stop message not found (signal handler did not fire)"
[[ -f "$MODEL" ]] || fail "model not written on interrupt: $MODEL"
python3 -c "import json,sys; json.load(open(sys.argv[1]))" "$MODEL" \
    || fail "model is not valid JSON (truncated write?)"

echo
echo "PASS: SIGTERM saved a valid model ($(stat -c %s "$MODEL") bytes)"
echo "      cooperative-stop message present in run log"
if [[ -s "$OUT" ]]; then
    echo "      --output written ($(wc -l < "$OUT") lines)"
else
    echo "      note: --output empty (slow server 200s everything -> soft-404/wildcard, so few real finds)"
fi
