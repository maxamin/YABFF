#!/usr/bin/env bash
# =============================================================================
# live_scan_workflow.sh — the LIVE --ml-loop scan workflow.
#
# Codifies the real-world invocation (the local-only counterpart is
# test_workflow.sh, which never touches the network):
#
#   shuf <domains> | head -n <count> | proxychains ./target/debug/feroxbuster \
#       --ml-loop --stdin --ml-list <list> --depth <depth> --ml-model <model> \
#       --output <output> --threads <threads> -s <codes>
#
# Unlike test_workflow.sh this DOES hit the network (through proxychains) against
# whatever hosts land in the sampled domain list — run it only against targets
# you are authorized to scan.
#
# Preflight guards (build the debug binary if missing, verify the domain list,
# the ML wordlist and proxychains) fail fast with a clear message rather than
# letting the pipe swallow a missing-input error. stderr is tee'd to a log so a
# second terminal can follow it with:  scripts/watch_ml_loop.sh <log> <model> <output>
# (off a TTY the scan prints periodic `[ml-loop] rounds=…` lines that script reads).
#
# Everything is overridable by env var; defaults mirror the documented command.
#
#   DOMAINS   domain list to sample            (default: ../jsintel/domains.txt)
#   COUNT     hosts to sample from it          (default: 100)
#   LIST      --ml-list wordlist               (default: raft-large-directories-lowercase.txt)
#   DEPTH     --depth                          (default: 5)
#   MODEL     --ml-model (persisted, shared)   (default: model.json)
#   OUTPUT    --output                         (default: ./test)
#   THREADS   --threads                        (default: 200)
#   CODES     -s status codes                  (default: 200-205,301-305)
#   LOG       stderr tee target                (default: <scratch>/live_scan.<pid>.log)
#   NO_PROXY  set to 1 to drop proxychains     (default: unset -> proxychains used)
#
# Usage:  scripts/live_scan_workflow.sh [extra feroxbuster args...]
# Exits non-zero if any preflight guard fails.
# =============================================================================
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/target/debug/feroxbuster"

DOMAINS="${DOMAINS:-$ROOT/../jsintel/domains.txt}"
COUNT="${COUNT:-100}"
LIST="${LIST:-/usr/share/seclists/Discovery/Web-Content/raft-large-directories-lowercase.txt}"
DEPTH="${DEPTH:-5}"
MODEL="${MODEL:-$ROOT/model.json}"
OUTPUT="${OUTPUT:-$ROOT/test}"
THREADS="${THREADS:-200}"
CODES="${CODES:-200,201,202,203,204,205,301,302,303,304,305}"
LOG="${LOG:-${TMPDIR:-/tmp}/live_scan.$$.log}"

die() { echo "live_scan_workflow: $*" >&2; exit 1; }

# ---- preflight -------------------------------------------------------------
# Build the debug binary if it isn't there (the documented command uses the
# debug build); a build failure stops here rather than mid-pipe.
if [[ ! -x "$BIN" ]]; then
    echo "[preflight] $BIN missing — building (cargo build -p feroxbuster)..." >&2
    ( cd "$ROOT" && cargo build -p feroxbuster --bin feroxbuster ) || die "build failed"
fi

[[ -r "$DOMAINS" ]] || die "domain list not found/readable: $DOMAINS (set DOMAINS=...)"
[[ -r "$LIST" ]]    || die "ml-list wordlist not found/readable: $LIST (set LIST=...)"
[[ "$COUNT" =~ ^[0-9]+$ && "$COUNT" -gt 0 ]] || die "COUNT must be a positive integer: $COUNT"

# proxychains: resolve the binary (proxychains4 on many distros) unless opted out.
PROXY=()
if [[ "${NO_PROXY:-0}" != "1" ]]; then
    if   command -v proxychains  >/dev/null 2>&1; then PROXY=(proxychains)
    elif command -v proxychains4 >/dev/null 2>&1; then PROXY=(proxychains4)
    else die "proxychains not found (install it, or set NO_PROXY=1 to scan directly)"
    fi
fi

# ---- summary ---------------------------------------------------------------
echo "=== live --ml-loop scan workflow =========================================" >&2
echo " domains : $DOMAINS  (sampling $COUNT)" >&2
echo " ml-list : $LIST" >&2
echo " model   : $MODEL" >&2
echo " output  : $OUTPUT" >&2
echo " depth=$DEPTH threads=$THREADS codes=$CODES proxy=${PROXY[*]:-<none>}" >&2
echo " log     : $LOG" >&2
echo "==========================================================================" >&2

# ---- run -------------------------------------------------------------------
# shuf | head feeds a random sample of hosts to --stdin. stderr (banner +
# periodic progress) is tee'd to $LOG; stdout (the machine-readable summary)
# stays on stdout. PIPESTATUS[2] is feroxbuster's own exit code, so a broken
# proxy or missing input surfaces here instead of being masked by tee.
shuf "$DOMAINS" | head -n "$COUNT" | "${PROXY[@]}" "$BIN" \
    --ml-loop --stdin \
    --ml-list "$LIST" \
    --depth "$DEPTH" \
    --ml-model "$MODEL" \
    --output "$OUTPUT" \
    --threads "$THREADS" \
    -s "$CODES" \
    "$@" \
    2> >(tee "$LOG" >&2)

rc="${PIPESTATUS[2]}"
echo "=== scan exited rc=$rc (log: $LOG) ===" >&2
exit "$rc"
