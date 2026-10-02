#!/usr/bin/env python3
"""F5 auxiliar: imprime valores inline restantes p/ revisão do main.tex."""
import csv
import os

RAIZ = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                    "..", "..", "artigo", "resultados")


def ler(nome):
    with open(os.path.join(RAIZ, nome), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


bench = {(r["sort"], r["tipo"], r["tamanho"], r["branch"]): r
         for r in ler("benchmark_consolidado.csv")}
ganho = {(r["sort"], r["tipo"], r["tamanho"]): float(r["ganho_pct"])
         for r in ler("ganho.csv")}
inv = {(r["tipo"], r["tamanho"]): r for r in ler("inversoes.csv")}


def cell(s, t, n):
    p = bench[(s, t, str(n), "puro")]
    c = bench[(s, t, str(n), "com_pre")]
    mp, cp = float(p["mean_ns"]) / 1000.0, float(c["mean_ns"]) / 1000.0
    hp = (float(p["ci_hi_ns"]) - float(p["ci_lo_ns"])) / 2.0 / 1000.0
    hc = (float(c["ci_hi_ns"]) - float(c["ci_lo_ns"])) / 2.0 / 1000.0
    return mp, hp, cp, hc, ganho[(s, t, str(n))]


CLASS = ["random", "turtles", "zigzag", "almostsorted", "duplicates",
         "inverted"]
print("== adaptive @10^4 (ranges) ==")
gi = [ganho[("insertion", t, "10000")] for t in CLASS]
gb = [ganho[("bubble", t, "10000")] for t in CLASS]
print("insertion gains:", [f"{x:.1f}" for x in gi])
print("bubble gains:", [f"{x:.1f}" for x in gb])
print("== speedups alinhados ==")
for s, t in [("insertion", "zigzag"), ("bubble", "zigzag"),
             ("insertion", "inverted"), ("bubble", "inverted")]:
    mp, hp, cp, hc, g = cell(s, t, 10000)
    print(f"{s}/{t}: {mp:.2f}/{cp:.2f} = x{mp/cp:.0f}")
print("== ns/inversao insertion @10^4 ==")
for t in ["random", "turtles", "zigzag", "inverted", "duplicates",
          "almostsorted"]:
    mp, hp, cp, hc, g = cell("insertion", t, 10000)
    ini = float(inv[(t, "10000")]["inversoes_iniciais"])
    print(f"{t}: {mp*1000.0/ini:.3f} ns/inv")
print("== bubble tempos @10^4 ==")
for t in CLASS:
    mp, hp, cp, hc, g = cell("bubble", t, 10000)
    print(f"{t}: {mp:.0f} us")
mp, hp, cp, hc, g = cell("bubble", "almostsorted", 10000)
mp2, hp2, cp2, hc2, g2 = cell("insertion", "almostsorted", 10000)
print(f"almostsorted bubble/insertion: {mp:.2f} / {mp2:.2f} = x{mp/mp2:.0f}")
print("== quick @10^4 ==")
for t in CLASS:
    mp, hp, cp, hc, g = cell("quick", t, 10000)
    print(f"{t}: {mp:.2f} -> {cp:.2f} ({g:+.1f}%)")
print("== quick @10^6 escala ==")
for t in ["random", "inverted", "zigzag", "turtles"]:
    a = bench[("quick", t, "10000", "com_pre")]
    b = bench[("quick", t, "1000000", "com_pre")]
    r = float(b["mean_ns"]) / float(a["mean_ns"])
    import math
    print(f"{t}: x{r:.0f} (expoente {math.log(r)/math.log(100):.2f})")
mp, hp, cp, hc, g = cell("quick", "inverted", 1000000)
mp2, hp2, cp2, hc2, g2 = cell("quick", "random", 1000000)
print(f"quick10^6 pre: inverted={cp:.2f}ms random-pre={cp2:.2f}ms")
print("== selection ==")
for t in CLASS + ["sawtooth", "organpipe", "fewruns", "real"]:
    mp, hp, cp, hc, g = cell("selection", t, 10000)
    print(f"{t}: {mp:.2f}±{hp:.2f} -> {cp:.2f}±{hc:.2f} ({g:+.1f}%)")
mp, hp, cp, hc, g = cell("selection", "inverted", 100000)
print(f"sel inv 100k: {mp/1000:.2f}±{hp/1000:.2f} -> {cp/1000:.2f}±{hc/1000:.2f}ms ({g:+.1f}%)")
print("== merge @10^6 ==")
for t in CLASS:
    mp, hp, cp, hc, g = cell("merge", t, 1000000)
    print(f"{t}: {mp/1000:.2f} -> {cp/1000:.2f}ms ({g:+.1f}%)")
print("== series ganho vs n ==")
for s, t in [("insertion", "random"), ("bubble", "random"),
              ("merge", "zigzag"), ("merge", "inverted"),
              ("quick", "random"), ("quick", "almostsorted"),
              ("stdunstable", "random"), ("insertion", "sawtooth"),
              ("insertion", "organpipe")]:
    serie = [(n, ganho.get((s, t, str(n)))) for n in
             [16, 32, 64, 128, 256, 512, 1000, 5000, 10000, 100000,
              1000000, 10000000]]
    print(f"{s}/{t}: " + " ".join(
        f"{n}={v:.1f}" if v is not None else f"{n}=--" for n, v in serie))
print("== baselines destaque @10^4 ==")
for s, t in [("stdunstable", "random"), ("stdunstable", "inverted"),
              ("stdstable", "random"), ("descreverse", "random"),
              ("descreverse", "inverted")]:
    mp, hp, cp, hc, g = cell(s, t, 10000)
    print(f"{s}/{t}: {mp:.2f} -> {cp:.2f} ({g:+.1f}%)")
mp, hp, cp, hc, g = cell("insertion", "random", 10000)
ms = bench[("quick", "random", "10000", "puro")]
print(f"insertion+pre random vs quick puro: {cp:.2f} vs "
      f"{float(ms['mean_ns'])/1000.0:.2f} = x{cp/(float(ms['mean_ns'])/1000.0):.1f}")
print("== tab4c selecao (novos tipos @10^4) ==")
for t in ["sawtooth", "organpipe", "fewruns", "real"]:
    row = []
    for s in ["merge", "quick", "insertion", "bubble", "selection",
              "stdunstable", "stdstable", "descreverse"]:
        row.append(f"{s}={ganho[(s,t,'10000')]:+.1f}")
    print(t + ": " + " ".join(row))
print("== zigzag footnote ==")
print("pos-pre:", inv[("zigzag", "10000")]["inversoes_pos_pre"])
