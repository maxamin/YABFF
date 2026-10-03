#!/usr/bin/env bash
# =============================================================================
# test_workflow.sh — end-to-end test workflow for the ML + WAF-detection work.
#
# Exercises every major code block against local labs and two throwaway
# adversarial fixtures, on top of the unit/integration suites:
#
#   UNIT   both crates' test suites (orchestrator, algos, config, parser,
#          scanner::waf, requester, heuristics, filters, ...)
#   ML     the --ml-loop orchestrator path (list mode, every algo + scheduler,
#          BM25 on/off, model persist+reload, --output streaming, multi-target
#          progression, depth bound, soft-404 suppression)
#   WAF    the normal-scan requester path + WAF detector (plain scan, auto-tune
#          with no false positive, ban->bail on a 403 wall, transient->backoff
#          on 429+Retry-After)
#
# Local-only. No proxychains. Safe to re-run. Exits non-zero on any failure.
#
# Usage:  scripts/test_workflow.sh [--no-unit] [--no-ml] [--no-waf]
# =============================================================================
set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT" || exit 2
BIN="$ROOT/target/debug/feroxbuster"
WL_DIR="$(mktemp -d)"
TMP="$(mktemp -d)"
CODES="200,201,204,301,302,401,403,405,429"

RUN_UNIT=1 RUN_ML=1 RUN_WAF=1
for a in "$@"; do case "$a" in
  --no-unit) RUN_UNIT=0 ;; --no-ml) RUN_ML=0 ;; --no-waf) RUN_WAF=0 ;;
esac; done

PASS=0 FAIL=0 SKIP=0
FIX_PIDS=()
cleanup() { for p in "${FIX_PIDS[@]:-}"; do kill "$p" 2>/dev/null; done; rm -rf "$WL_DIR" "$TMP"; }
trap cleanup EXIT

green() { printf '\033[32m%s\033[0m' "$1"; }
red()   { printf '\033[31m%s\033[0m' "$1"; }
ok()   { PASS=$((PASS+1)); echo "  [$(green PASS)] $1"; }
bad()  { FAIL=$((FAIL+1)); echo "  [$(red FAIL)] $1"; [ -n "${2:-}" ] && echo "         $2"; }
skip() { SKIP=$((SKIP+1)); echo "  [SKIP] $1"; }
hdr()  { echo; echo "=== $1 ==="; }

# lab reachability
lab_up() { [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 3 "$1" 2>/dev/null)" != "000" ]; }

# a curated, deterministic wordlist dir for the ML scenarios
mk_wordlist() {
  printf '%s\n' admin login wp-login.php readme.html index config robots.txt about profile \
    > "$WL_DIR/words.txt"
}

# ---------------------------------------------------------------- build
hdr "build"
if cargo build -p feroxbuster -p ferox-ml-core 2>"$TMP/build.err"; then
  ok "debug build"
else
  bad "debug build" "$(tail -3 "$TMP/build.err")"; echo; echo "aborting: build failed"; exit 1
fi
[ -x "$BIN" ] || { echo "missing $BIN"; exit 1; }

# ---------------------------------------------------------------- UNIT
if [ "$RUN_UNIT" = 1 ]; then
  hdr "UNIT — test suites (cover config/parser/scanner::waf/requester/orchestrator/algos)"
  if cargo test -p ferox-ml-core --lib >"$TMP/u1.out" 2>&1; then
    ok "ferox-ml-core lib: $(grep -oE '[0-9]+ passed' "$TMP/u1.out" | head -1)"
  else bad "ferox-ml-core lib" "$(grep -E 'test result|error' "$TMP/u1.out" | tail -3)"; fi
  if cargo test -p feroxbuster --lib >"$TMP/u2.out" 2>&1; then
    ok "feroxbuster lib: $(grep -oE '[0-9]+ passed' "$TMP/u2.out" | head -1)"
  else bad "feroxbuster lib" "$(grep -E 'test result|error' "$TMP/u2.out" | tail -3)"; fi
fi

# ---------------------------------------------------------------- ML LOOP
if [ "$RUN_ML" = 1 ]; then
  hdr "ML — --ml-loop orchestrator path (local labs, no proxychains)"
  mk_wordlist
  FAST="http://127.0.0.3:8081/"   # yields quick, distinct finds
  WP="http://127.0.0.8:8083/"     # WordPress catch-all -> soft-404 suppression
  ALT="http://127.0.0.5:5013/"

  run_ml() { # $1=label-of-model $@: extra args ; target on stdin
    local tgt="$1"; shift
    printf '%s\n' "$tgt" | timeout 60 "$BIN" --ml-loop --stdin \
      --ml-list-dir "$WL_DIR" --ml-list-max 0 --depth 1 \
      -s "$CODES" --threads 10 "$@" 2>&1
  }

  if lab_up "$FAST"; then
    # M1 list-mode discovery
    out=$(run_ml "$FAST" --ml-model "$TMP/m1.json" --output "$TMP/o1");
    found=$(echo "$out" | sed -n 's/^resources found  : //p')
    { [ -n "$found" ] && [ "$found" -ge 1 ]; } && ok "M1 list-mode discovers (found=$found)" \
      || bad "M1 list-mode discovers" "found=$found"

    # M7 --output streaming
    { [ -s "$TMP/o1" ] && grep -qE '^[0-9]{3} https?://' "$TMP/o1"; } \
      && ok "M7 --output streams discoveries ($(wc -l <"$TMP/o1") lines)" \
      || bad "M7 --output streams discoveries" "output empty/malformed"

    # M6 --ml-model persist + reload
    if [ -s "$TMP/m1.json" ]; then
      sz1=$(wc -c <"$TMP/m1.json")
      run_ml "$FAST" --ml-model "$TMP/m1.json" >/dev/null
      sz2=$(wc -c <"$TMP/m1.json")
      { [ -s "$TMP/m1.json" ] && [ "$sz2" -ge "$sz1" ]; } \
        && ok "M6 --ml-model persists & reloads (${sz1}B -> ${sz2}B)" \
        || bad "M6 --ml-model persist/reload" "sz1=$sz1 sz2=$sz2"
    else bad "M6 --ml-model persist/reload" "model not written"; fi

    # M3 every algo
    for algo in markov trie dynsdt tst auto; do
      out=$(run_ml "$FAST" --ml-algo "$algo" --ml-model "$TMP/alg_$algo.json")
      rc=$?
      want="$algo"; [ "$algo" = auto ] && want="dynsdt"  # auto -> dynsdt in list mode
      line=$(echo "$out" | sed -n 's/^algorithm        : //p' | head -1)
      { [ "$rc" = 0 ] && [ "$line" = "$want" ]; } \
        && ok "M3 algo=$algo (summary: $line)" \
        || bad "M3 algo=$algo" "rc=$rc algorithm='$line' want='$want'"
    done

    # M4 every scheduler
    for s in thompson ucb1 round_robin; do
      out=$(run_ml "$FAST" --ml-scheduler "$s" --ml-model "$TMP/sch_$s.json"); rc=$?
      f=$(echo "$out" | sed -n 's/^resources found  : //p')
      { [ "$rc" = 0 ] && [ -n "$f" ]; } && ok "M4 scheduler=$s (found=$f)" \
        || bad "M4 scheduler=$s" "rc=$rc"
    done

    # M5 --no-ml-rank
    out=$(run_ml "$FAST" --no-ml-rank --ml-model "$TMP/norank.json"); rc=$?
    f=$(echo "$out" | sed -n 's/^resources found  : //p')
    { [ "$rc" = 0 ] && [ -n "$f" ] && [ "$f" -ge 1 ]; } && ok "M5 --no-ml-rank discovers (found=$f)" \
      || bad "M5 --no-ml-rank" "rc=$rc found=$f"
  else
    skip "M1/M3/M4/M5/M6/M7 (fast lab $FAST down)"
  fi

  # M2 soft-404 suppression active on the WordPress lab (its catch-all paths
  # collapse onto one template and must be filtered; real distinct pages like
  # wp-login.php/readme.html may still legitimately surface, so the signal is
  # "filtered >= 1", not "found == 0").
  if lab_up "$WP"; then
    out=$(run_ml "$WP" --ml-model "$TMP/wp.json")
    f=$(echo "$out" | sed -n 's/^resources found  : //p')
    s4=$(echo "$out" | sed -n 's/^filtered soft404 : //p')
    { [ -n "$s4" ] && [ "$s4" -ge 1 ]; } \
      && ok "M2 soft-404 suppression active (filtered=$s4, found=$f real)" \
      || bad "M2 soft-404 suppression" "found=$f filtered=$s4"
  else skip "M2 soft-404 (WP lab down)"; fi

  # M8 multi-target progression
  if lab_up "$FAST" && lab_up "$ALT"; then
    out=$(printf '%s\n%s\n' "$FAST" "$ALT" | timeout 90 "$BIN" --ml-loop --stdin \
      --ml-list-dir "$WL_DIR" --ml-list-max 0 --depth 1 -s "$CODES" --threads 10 \
      --ml-model "$TMP/multi.json" 2>&1)
    echo "$out" | grep -qE '2/2 targets scanned' \
      && ok "M8 multi-target progression (2/2 scanned)" \
      || bad "M8 multi-target progression" "$(echo "$out" | grep -E 'targets scanned' | tail -1)"
  else skip "M8 multi-target (need 2 labs up)"; fi
fi

# ---------------------------------------------------------------- WAF / normal scan
if [ "$RUN_WAF" = 1 ]; then
  hdr "WAF — normal-scan requester path + detector (local labs + adversarial fixtures)"
  mk_wordlist
  FAST="http://127.0.0.3:8081/"

  # big wordlist so requests >= max(threads,50) and the status ratio dominates.
  # WAF assertions are BEHAVIORAL (did the scan bail early or keep going), which
  # is robust; feroxbuster's warn-level logs are not reliably surfaced to stderr.
  WAF_WORDS=200
  seq 1 "$WAF_WORDS" | sed 's#^#p#' > "$TMP/waf_words.txt"
  count_results() { grep -cE '^[0-9]{3} +GET' "$1"; }

  # N1 plain scan discovers
  if lab_up "$FAST"; then
    out=$(timeout 40 "$BIN" -u "$FAST" -w "$WL_DIR/words.txt" -k -q --no-state -n \
      -s "$CODES" --threads 10 2>&1); rc=$?
    { [ "$rc" = 0 ] && echo "$out" | grep -qE '^[0-9]{3} +GET'; } \
      && ok "N1 plain scan discovers resources" \
      || bad "N1 plain scan" "rc=$rc"

    # N2 auto-tune: no false WAF positive on a friendly lab
    out=$(timeout 40 "$BIN" -u "$FAST" -w "$WL_DIR/words.txt" -k -q --no-state -n \
      --auto-tune -s "$CODES" --threads 10 2>&1); rc=$?
    { [ "$rc" = 0 ] && ! echo "$out" | grep -qiE 'WAF +(Banned|Bailed)|bailing'; } \
      && ok "N2 auto-tune: no false WAF ban on friendly lab" \
      || bad "N2 auto-tune false positive" "rc=$rc $(echo "$out" | grep -i 'WAF' | tail -1)"
  else skip "N1/N2 (fast lab down)"; fi

  # ---- adversarial fixtures (throwaway, loopback only) ----
  start_fixture() { # $1=port $2=mode(403|429)
    local port="$1" mode="$2"
    cat > "$TMP/fix_$port.py" <<PY
import http.server, sys
MODE = "$mode"
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if MODE == "429":
            self.send_response(429); self.send_header("Retry-After", "1")
        else:
            self.send_response(403)
        self.send_header("Server", "cloudflare")
        self.send_header("CF-RAY", "deadbeef-TEST")
        self.end_headers(); self.wfile.write(b"blocked")
    def log_message(self, *a): pass
http.server.HTTPServer(("127.0.0.1", $port), H).serve_forever()
PY
    python3 "$TMP/fix_$port.py" & FIX_PIDS+=($!)
    for _ in $(seq 1 20); do
      [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 1 "http://127.0.0.1:$port/x")" = "$([ "$mode" = 429 ] && echo 429 || echo 403)" ] && return 0
      sleep 0.2
    done
    return 1
  }

  # N3 ban -> bail (403 wall, --auto-bail). The detector classifies a ~100% 403
  # ratio as a ban, so AutoBail cancels the scan shortly after the enforcement
  # floor (~50 requests) instead of walking the whole wordlist. -D so the 403
  # wall isn't auto-filtered before it can be tallied.
  if start_fixture 8461 403; then
    timeout 40 "$BIN" -u "http://127.0.0.1:8461/" -w "$TMP/waf_words.txt" -k -q --no-state -n -D \
      --auto-bail -s "$CODES" --threads 10 >"$TMP/n3.out" 2>&1
    n3=$(count_results "$TMP/n3.out")
    early=0; { [ "$n3" -ge 40 ] && [ "$n3" -lt $((WAF_WORDS * 3 / 4)) ]; } && early=1
    # the verdict line must be visible WITHOUT -v (the log-visibility fix)
    visible=0; grep -qiE 'WAF +(Banned|Bailed).*(403 wall|vendor)' "$TMP/n3.out" && visible=1
    { [ "$early" = 1 ] && [ "$visible" = 1 ]; } \
      && ok "N3 403-wall -> AutoBail cancels early + verdict visible ($n3/$WAF_WORDS)" \
      || bad "N3 403-wall -> bail" "scanned=$n3/$WAF_WORDS early=$early verdict_visible=$visible"
  else skip "N3 (could not start 403 fixture)"; fi

  # N4 transient 429 under --auto-bail does NOT bail on the first interval: the
  # detector calls it RateLimited (transient), so AutoBail backs off and keeps
  # the target rather than abandoning it — it processes well past N3's early
  # cancel. (A truly sustained 429 only escalates to a ban after several
  # intervals; the point here is it is NOT treated as an instant ban.)
  if start_fixture 8462 429; then
    timeout 40 "$BIN" -u "http://127.0.0.1:8462/" -w "$TMP/waf_words.txt" -k -q --no-state -n -D \
      --auto-bail -s "$CODES" --threads 10 >"$TMP/n4.out" 2>&1
    n4=$(count_results "$TMP/n4.out")
    { [ "$n4" -gt "${n3:-0}" ]; } \
      && ok "N4 transient 429 -> AutoBail backs off, not instant bail ($n4 > N3 $n3)" \
      || bad "N4 429 transient" "scanned $n4 (expected > N3's $n3)"
  else skip "N4 (could not start 429 fixture)"; fi

  # N5 AutoTune never bails, even on a 403 wall: it tunes/backs off and keeps
  # going (times out still running, or finishes), proving the policy contract.
  if start_fixture 8463 403; then
    timeout 25 "$BIN" -u "http://127.0.0.1:8463/" -w "$TMP/waf_words.txt" -k -q --no-state -n -D \
      --auto-tune -s "$CODES" --threads 10 >"$TMP/n5.out" 2>&1
    n5rc=$?; n5=$(count_results "$TMP/n5.out")
    { [ "$n5rc" = 124 ] || [ "$n5" -ge $((WAF_WORDS * 3 / 4)) ]; } \
      && ok "N5 AutoTune never bails on 403 wall (rc=$n5rc, $n5/$WAF_WORDS)" \
      || bad "N5 AutoTune no-bail" "rc=$n5rc scanned $n5/$WAF_WORDS (looks like an early cancel)"
  else skip "N5 (could not start 403 fixture)"; fi

  # ---- ml-loop path WAF wiring (the in-process runner, not the requester) ----
  MLW="$TMP/mlwl"; mkdir -p "$MLW"; seq 1 300 | sed 's#^#p#' > "$MLW/w.txt"

  # MW1 ml-loop vs 403 wall -> detector aborts THIS target; sweep would continue
  if start_fixture 8471 403; then
    printf 'http://127.0.0.1:8471/\n' | timeout 40 "$BIN" --ml-loop --stdin \
      --ml-list-dir "$MLW" --ml-list-max 0 --depth 1 --threads 10 \
      -s 200,301,302,403,429 >"$TMP/mw1.out" 2>&1
    vis=0;  grep -qiE 'WAF +(Banned|Bailed)' "$TMP/mw1.out" && vis=1
    abrt=0; grep -qiE 'target .*failed: WAF ban' "$TMP/mw1.out" && abrt=1
    { [ "$vis" = 1 ] && [ "$abrt" = 1 ]; } \
      && ok "MW1 ml-loop 403-wall -> WAF ban aborts target" \
      || bad "MW1 ml-loop WAF ban" "verdict_visible=$vis target_aborted=$abrt"
  else skip "MW1 (could not start 403 fixture)"; fi

  # MW2 ml-loop vs 429 -> transient RateLimited, backs off (no instant abort)
  if start_fixture 8472 429; then
    printf 'http://127.0.0.1:8472/\n' | timeout 20 "$BIN" --ml-loop --stdin \
      --ml-list-dir "$MLW" --ml-list-max 0 --depth 1 --threads 10 \
      -s 200,301,302,403,429 >"$TMP/mw2.out" 2>&1
    grep -qiE 'WAF +RateLimited' "$TMP/mw2.out" \
      && ok "MW2 ml-loop 429 -> transient RateLimited backoff" \
      || bad "MW2 ml-loop transient" "no RateLimited verdict seen"
  else skip "MW2 (could not start 429 fixture)"; fi
fi

# ---------------------------------------------------------------- summary
hdr "summary"
echo "  PASS=$PASS  FAIL=$FAIL  SKIP=$SKIP"
[ "$FAIL" = 0 ]
