#!/usr/bin/env python3
"""Summarize a --ml-loop --output results file.

Auto-detects the two formats run_ml_loop writes:
  * plaintext : "<status> <url>" per line
  * NDJSON    : {"type":"response","url":...,"path":...,"status":...} per line (--json)

Reports totals, status-code breakdown, per-host hit counts, and duplicate URLs,
so you can sanity-check that --output routing is working and see what a run
actually found.

Usage:
    scripts/inspect_output.py [testoutput] [--top N]
"""
import argparse
import json
import os
import sys
from collections import Counter
from urllib.parse import urlparse


def parse_line(line):
    """Return (status:int|None, url:str) or None if unparseable."""
    line = line.strip()
    if not line:
        return None
    if line[0] == "{":
        try:
            obj = json.loads(line)
            return (obj.get("status"), obj.get("url", ""))
        except json.JSONDecodeError:
            return None
    parts = line.split(None, 1)
    if len(parts) != 2:
        return None
    status = int(parts[0]) if parts[0].isdigit() else None
    return (status, parts[1])


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("path", nargs="?", default="testoutput")
    ap.add_argument("--top", type=int, default=20)
    args = ap.parse_args()

    if not os.path.exists(args.path):
        sys.exit(f"no such file: {args.path}")

    total = 0
    bad = 0
    status_counts = Counter()
    host_counts = Counter()
    url_counts = Counter()

    with open(args.path, errors="replace") as fh:
        for line in fh:
            parsed = parse_line(line)
            if parsed is None:
                if line.strip():
                    bad += 1
                continue
            status, url = parsed
            total += 1
            status_counts[status] += 1
            url_counts[url] += 1
            host = urlparse(url).netloc or "<no-host>"
            host_counts[host] += 1

    dupes = {u: c for u, c in url_counts.items() if c > 1}

    print(f"path          : {args.path}")
    print(f"size          : {os.path.getsize(args.path):,} bytes")
    print(f"result lines  : {total:,}")
    print(f"unparseable   : {bad:,}")
    print(f"unique urls   : {len(url_counts):,}")
    print(f"duplicate urls: {len(dupes):,}")
    print(f"unique hosts  : {len(host_counts):,}")

    print("\nstatus codes:")
    for status, c in status_counts.most_common():
        print(f"  {status}: {c:,}")

    print(f"\ntop {args.top} hosts by hits:")
    for host, c in host_counts.most_common(args.top):
        print(f"  {c:>6}  {host}")

    if dupes:
        print(f"\ntop {args.top} duplicated urls:")
        for url, c in sorted(dupes.items(), key=lambda x: -x[1])[: args.top]:
            print(f"  {c:>4}x  {url}")


if __name__ == "__main__":
    main()
