import numpy as np, itertools

features = ["Dir/File","Recurse","Depth","DNS","Vhost","API","FUZZ-pos","Body","Header",
"Multi-WL","Encoders","Methods","Crawl","robots","DirList","Wildcard","F:Status","F:Size",
"F:Regex","F:Similar","Rate-lim","Auto-tune","Proxy","Replay","SOCKS","mTLS","Rand-UA",
"Resume","Interact","JSON-out","Ext-col","Bak-col","Word-col","Multi-proto","GUI"]

P = {
"ffuf":       [2,2,2,0,2,0,2,2,2,2,0,2,0,0,0,2,2,2,2,1,2,0,2,2,2,2,0,0,2,2,0,0,0,0,0],
"feroxbuster":[2,2,2,0,1,0,0,1,0,1,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,0,0],
"wfuzz":      [2,2,2,0,2,0,2,2,2,2,2,2,1,1,0,1,2,2,2,0,1,0,2,0,2,1,1,0,0,2,0,0,0,0,0],
"dirsearch":  [2,2,2,0,0,0,1,1,0,1,0,2,2,0,0,2,2,2,2,1,2,0,2,0,2,2,2,2,1,2,0,0,0,0,0],
"gobuster":   [2,0,0,2,2,0,2,1,1,0,0,1,0,0,0,2,2,1,1,0,2,0,2,0,2,0,0,0,0,0,0,0,0,0,0],
"kiterunner": [2,1,1,0,0,2,0,2,2,0,0,2,0,0,0,2,2,1,0,0,2,0,2,0,1,0,1,0,0,2,0,0,0,0,0],
"patator":    [2,0,0,2,1,0,2,2,2,2,2,2,0,0,0,1,2,2,2,0,2,0,1,0,1,1,0,1,1,1,0,0,0,2,0],
"dirbuster":  [2,2,1,0,0,0,0,0,0,0,0,1,2,0,0,2,1,0,0,0,1,0,2,0,0,0,0,0,2,1,0,0,0,0,2],
"dirb":       [2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,1,0,2,0,0,0,0,0,0,0,0,0,0,0,0],
}
projects=list(P)
M=np.array([P[p] for p in projects],dtype=float)/2.0   # 9x35 in {0,0.5,1}
var=M.var(axis=0)
keep=[i for i in range(len(features)) if var[i]>1e-9]
dropped=[features[i] for i in range(len(features)) if i not in keep]
fk=[features[i] for i in keep]
Mk=M[:,keep]
C=np.corrcoef(Mk.T)   # feature x feature Pearson
# guard nan
C=np.nan_to_num(C)

# full matrix CSV
with open("/root/YABFF/feature-correlation-matrix.csv","w") as fh:
    fh.write("feature,"+",".join(fk)+"\n")
    for i,f in enumerate(fk):
        fh.write(f+","+",".join(f"{C[i,j]:.3f}" for j in range(len(fk)))+"\n")

# top pairs
pairs=[]
for i,j in itertools.combinations(range(len(fk)),2):
    pairs.append((C[i,j],fk[i],fk[j]))
pos=sorted(pairs,reverse=True)
neg=sorted(pairs)

# co-occurrence counts (# projects where BOTH have >=partial, using binary has=val>0)
B=(Mk>0).astype(int)
def cooc(i,j): return int((B[:,i]&B[:,j]).sum())

print("KEPT features:",len(fk),"  DROPPED (zero variance):",dropped)
print("\n=== TOP 20 POSITIVE feature correlations ===")
seen=set()
cnt=0
for r,a,b in pos:
    if r>0.9999 and (a,b): pass
    print(f"{r:+.3f}  {a:<11} ~ {b:<11}  (co-occur in {cooc(fk.index(a),fk.index(b))}/9)")
    cnt+=1
    if cnt>=20: break
print("\n=== TOP 12 NEGATIVE feature correlations ===")
for r,a,b in neg[:12]:
    print(f"{r:+.3f}  {a:<11} ~ {b:<11}")

# perfectly collinear groups (r==1)
import collections
groups=collections.defaultdict(set)
adj=collections.defaultdict(set)
for i,j in itertools.combinations(range(len(fk)),2):
    if C[i,j]>0.9999:
        adj[fk[i]].add(fk[j]); adj[fk[j]].add(fk[i])
# connected components
visited=set(); comps=[]
for f in fk:
    if f in visited: continue
    stack=[f]; comp=set()
    while stack:
        x=stack.pop()
        if x in visited: continue
        visited.add(x); comp.add(x); stack+=list(adj[x])
    if len(comp)>1: comps.append(sorted(comp))
print("\n=== PERFECTLY COLLINEAR groups (r=1.000) ===")
for c in comps: print("  {", ", ".join(c), "}")

# ---- build markdown doc ----
counts = (Mk>0).sum(axis=0)  # presence count per kept feature
# core subset: features present in 3..7 projects (genuinely variable, multi-project)
core_idx=[i for i in range(len(fk)) if 3<=counts[i]<=7]
core=[fk[i] for i in core_idx]
# short codes for heatmap
def shade(r):
    if r>=0.8: return "▓▓"
    if r>=0.5: return "▒▒"
    if r>=0.2: return "░░"
    if r>-0.2: return "··"
    if r>-0.5: return "--"
    if r>-0.8: return "=="
    return "##"
lines=[]
# heatmap header (codes)
codes={f:f"{i+1:02d}" for i,f in enumerate(core)}
hdr="| # feature | "+" | ".join(codes[f] for f in core)+" |"
sep="|"+"---|"*(len(core)+1)
lines.append(hdr); lines.append(sep)
for i,fi in enumerate(core):
    row=[]
    for fj in core:
        r=C[fk.index(fi),fk.index(fj)]
        row.append(shade(r) if fi!=fj else "▓▓")
    lines.append(f"| `{codes[fi]}` {fi} | "+" | ".join(row)+" |")
heat="\n".join(lines)
codelegend="\n".join(f"- `{codes[f]}` = {f} (in {int(counts[fk.index(f)])}/9)" for f in core)

# numeric core matrix
nlines=["| feature | "+" | ".join(codes[f] for f in core)+" |","|"+"---|"*(len(core)+1)]
for fi in core:
    vals=[f"{C[fk.index(fi),fk.index(fj)]:+.2f}" for fj in core]
    nlines.append(f"| `{codes[fi]}` | "+" | ".join(vals)+" |")
numeric="\n".join(nlines)

# robust pairs: both features present in >=4 projects, |r|>=0.5, exclude self
robust=[]
for i,j in itertools.combinations(range(len(fk)),2):
    if counts[i]>=4 and counts[j]>=4 and abs(C[i,j])>=0.5:
        robust.append((C[i,j],fk[i],fk[j],cooc(i,j)))
robust_pos=sorted([r for r in robust if r[0]>0],reverse=True)
robust_neg=sorted([r for r in robust if r[0]<0])

def tbl(rows):
    out=["| r | feature A | feature B | co-occur |","|---|---|---|---|"]
    for r,a,b,c in rows:
        out.append(f"| {r:+.3f} | {a} | {b} | {c}/9 |")
    return "\n".join(out)

import json
open("/tmp/claude-0/corr_parts.json","w").write(json.dumps({
 "heat":heat,"codelegend":codelegend,"numeric":numeric,
 "robust_pos":tbl(robust_pos),"robust_neg":tbl(robust_neg),
 "ncore":len(core),"nkept":len(fk)
}))
print("core features:",len(core))
print(heat[:300])
