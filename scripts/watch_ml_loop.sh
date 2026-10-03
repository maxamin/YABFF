#!/usr/bin/env bash
# Snapshot the health of a running --ml-loop: latest progress, discovery rate,
# and whether the model + output files are actually being written (confirms the
# periodic flush and --output routing are live).
#
# One-shot by default; pass an interval (seconds) as $4 to repeat.
#
# Usage:
#   scripts/watch_ml_loop.sh [progress.log] [model.json] [testoutput] [interval]
#
# With no progress log given, picks the newest *.output under the session tasks
# dir if that env is discoverable, else requires an explicit path.
set -euo pipefail

LOG="${1:-}"
MODEL="${2:-model.json}"
OUT="${3:-testoutput}"
INTERVAL="${4:-0}"

if [[ -z "$LOG" ]]; then
    echo "usage: $0 <progress.log> [model.json] [testoutput] [interval]" >&2
    exit 2
fi

file_age_secs() { echo $(( $(date +%s) - $(stat -c %Y "$1" 2>/dev/null || echo 0) )); }

snapshot() {
    echo "=== $(date '+%H:%M:%S') ============================================"
    if [[ -f "$LOG" ]]; then
        last=$(grep -a '\[ml-loop\]' "$LOG" | tail -1 || true)
        echo "progress : ${last:-<no [ml-loop] lines yet>}"
        # discovery rate from the last line: found per 1k requests
        if [[ "$last" =~ requests=([0-9]+).*found=([0-9]+) ]]; then
            req="${BASH_REMATCH[1]}"; found="${BASH_REMATCH[2]}"
            if (( req > 0 )); then
                rate=$(awk "BEGIN{printf \"%.3f\", $found*1000/$req}")
                echo "rate     : ${found} found / ${req} requests  (${rate} per 1k)"
            fi
        fi
        grep -aiq 'interrupted: stopping' "$LOG" && \
            echo "signal   : cooperative stop fired (interrupt saved the model)"
    else
        echo "progress : <log not found: $LOG>"
    fi

    if [[ -f "$MODEL" ]]; then
        echo "model    : $(stat -c %s "$MODEL") bytes, last written $(file_age_secs "$MODEL")s ago"
    else
        echo "model    : <not written yet: $MODEL>"
    fi

    if [[ -f "$OUT" ]]; then
        echo "output   : $(wc -l < "$OUT") lines, $(stat -c %s "$OUT") bytes, last written $(file_age_secs "$OUT")s ago"
    else
        echo "output   : <not written yet: $OUT>"
    fi
}

if (( INTERVAL > 0 )); then
    while true; do snapshot; sleep "$INTERVAL"; done
else
    snapshot
fi
