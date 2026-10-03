#!/usr/bin/env python3
"""Inspect a persisted --ml-loop / --ml model file (model.json).

Reports the serialized shape and summary statistics so you can confirm a run is
actually learning (and that the periodic flush / interrupt-save is writing a
sane model), without eyeballing 24 MB of JSON.

Handles the Markov DTO ({"rows": [[ctx, [[tok, weight], ...]], ...], max_order,
alpha, threshold}) in detail; for the DynSDT / FreqTrie / TST engines it falls
back to a generic key/size summary.

Usage:
    scripts/inspect_model.py [model.json] [--top N]
"""
import argparse
import datetime as dt
import json
import os
import sys
from collections import Counter


def human_bytes(n):
    for unit in ("B", "KB", "MB", "GB"):
        if n < 1024 or unit == "GB":
            return f"{n:.1f}{unit}" if unit != "B" else f"{n}B"
        n /= 1024


def inspect_markov(dto, top):
    rows = dto.get("rows", [])
    print("format           : markov")
    print(f"max_order        : {dto.get('max_order')}")
    print(f"alpha            : {dto.get('alpha')}")
    print(f"threshold        : {dto.get('threshold')}")
    print(f"contexts (rows)  : {len(rows):,}")

    total_transitions = 0
    total_mass = 0.0
    order_hist = Counter()
    outdeg = []  # (outdegree, context)
    transitions = []  # (weight, context, token)
    for ctx, tos in rows:
        order_hist[len(ctx)] += 1
        total_transitions += len(tos)
        outdeg.append((len(tos), "/".join(ctx)))
        for tok, w in tos:
            total_mass += w
            transitions.append((w, "/".join(ctx), tok))

    print(f"transitions      : {total_transitions:,}")
    print(f"observation mass : {total_mass:,.0f}")
    print(
        "avg out-degree   : "
        f"{(total_transitions / len(rows)) if rows else 0:.2f}"
    )

    print("\ncontext-order histogram (context length -> #contexts):")
    for order in sorted(order_hist):
        print(f"  order {order}: {order_hist[order]:,}")

    print(f"\ntop {top} contexts by out-degree:")
    for deg, ctx in sorted(outdeg, reverse=True)[:top]:
        print(f"  {deg:>5}  {ctx or '<root>'}")

    print(f"\ntop {top} transitions by weight:")
    for w, ctx, tok in sorted(transitions, key=lambda x: x[0], reverse=True)[:top]:
        print(f"  {w:>8.1f}  {(ctx or '<root>')} -> {tok}")


def inspect_generic(data, top):
    print("format           : non-markov (generic summary)")
    if isinstance(data, dict):
        print(f"top-level keys   : {list(data.keys())}")
        for k, v in data.items():
            if isinstance(v, (list, dict)):
                print(f"  {k}: {type(v).__name__} len={len(v)}")
            else:
                print(f"  {k}: {v!r}")
    else:
        print(f"top-level type   : {type(data).__name__}")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("path", nargs="?", default="model.json")
    ap.add_argument("--top", type=int, default=15)
    args = ap.parse_args()

    if not os.path.exists(args.path):
        sys.exit(f"no such file: {args.path}")

    st = os.stat(args.path)
    age = dt.datetime.now() - dt.datetime.fromtimestamp(st.st_mtime)
    print(f"path             : {args.path}")
    print(f"size             : {human_bytes(st.st_size)}")
    print(
        "modified         : "
        f"{dt.datetime.fromtimestamp(st.st_mtime):%Y-%m-%d %H:%M:%S} "
        f"({int(age.total_seconds())}s ago)"
    )

    try:
        with open(args.path) as fh:
            data = json.load(fh)
    except json.JSONDecodeError as e:
        sys.exit(f"invalid JSON (truncated write / interrupted flush?): {e}")

    print()
    if isinstance(data, dict) and "rows" in data and "max_order" in data:
        inspect_markov(data, args.top)
    else:
        inspect_generic(data, args.top)


if __name__ == "__main__":
    main()
