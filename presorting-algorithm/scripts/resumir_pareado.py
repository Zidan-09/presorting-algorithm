#!/usr/bin/env python3
"""F4 (docs/plan2.md): agrega efeito_pareado.csv em efeito_pareado_resumo.csv.

Por célula (sort, tipo, tamanho), combina as 5 seeds (pools independentes):
- ganhos por seed (com IC bootstrap pareado), média/min/max entre pools;
- estabilidade de sinal: flip se alguma seed difere de sinal (entre sig.);
- decisão: ganho (5/5 sig +), perda (5/5 sig -), neutro (0/5 sig),
  instavel (qualquer outro caso: sig parcial ou flip de sinal).
- colunas seed-42 (ganho_s42, sig_s42) p/ confronto com as Tabelas 4/6.

Uso: python3 presorting-algorithm/scripts/resumir_pareado.py
Lê/escreve em artigo/resultados/ (raiz do repo).
"""
import csv
import os

RAIZ = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                    "..", "..", "artigo", "resultados")
ENTRADA = os.path.join(RAIZ, "efeito_pareado.csv")
SAIDA = os.path.join(RAIZ, "efeito_pareado_resumo.csv")

with open(ENTRADA, newline="", encoding="utf-8") as f:
    linhas = list(csv.DictReader(f))

celulas = {}
for l in linhas:
    chave = (l["sort"], l["tipo"], l["tamanho"])
    celulas.setdefault(chave, []).append(l)

resumo = []
flips = []
for (sort, tipo, tam), reps in sorted(celulas.items()):
    assert len(reps) == 5, f"{sort}/{tipo}/{tam}: {len(reps)} seeds (esperado 5)"
    ganhos = [float(x["ganho_pct"]) for x in reps]
    sigs = [x["sig"] == "1" for x in reps]
    sinais = [x["sinal"] for x in reps]
    n_plus = sum(1 for x in reps if x["sig"] == "1" and x["sinal"] == "+")
    n_minus = sum(1 for x in reps if x["sig"] == "1" and x["sinal"] == "-")
    n_sig = n_plus + n_minus
    flip = ("+" in sinais and "-" in sinais)
    if n_plus == 5:
        decisao = "ganho"
    elif n_minus == 5:
        decisao = "perda"
    elif n_sig == 0:
        decisao = "neutro"
    else:
        decisao = "instavel"
    if flip:
        flips.append((sort, tipo, tam, ganhos))
    s42 = next(x for x in reps if x["seed"] == "42")
    resumo.append({
        "sort": sort, "tipo": tipo, "tamanho": tam,
        "ganho_medio": f"{sum(ganhos)/len(ganhos):.2f}",
        "ganho_min": f"{min(ganhos):.2f}",
        "ganho_max": f"{max(ganhos):.2f}",
        "n_sig": str(n_sig),
        "n_plus_sig": str(n_plus),
        "n_minus_sig": str(n_minus),
        "flip_sinal": "1" if flip else "0",
        "decisao": decisao,
        "ganho_s42": s42["ganho_pct"],
        "ganho_lo_s42": s42["ganho_lo"],
        "ganho_hi_s42": s42["ganho_hi"],
        "sig_s42": s42["sig"],
        "p_melhora_s42": s42["p_melhora"],
    })

with open(SAIDA, "w", newline="", encoding="utf-8") as f:
    w = csv.DictWriter(f, fieldnames=list(resumo[0].keys()))
    w.writeheader()
    w.writerows(resumo)

tot = len(resumo)
from collections import Counter
cont = Counter(r["decisao"] for r in resumo)
print(f"células: {tot} {dict(cont)}")
print(f"flips de sinal entre pools: {len(flips)}")
for sort, tipo, tam, ganhos in flips:
    gs = ",".join(f"{g:.1f}" for g in ganhos)
    print(f"  FLIP {sort}/{tipo}/n={tam}: [{gs}]")
print(f"OK -> {SAIDA}")

# Confronto com as Tabelas 4 (n=10^4) e 6 (n=10^6): pareada seed-42 vs.
# Criterion (ganho.csv). Regra antiga: ICs não sobrepostos (ver docs/plan.md
# §8.4: em Tab4, não-sig = merge/duplicates, selection random/almostsorted/
# duplicates; em Tab6, não-sig = merge/random).
GANHO = os.path.join(RAIZ, "ganho.csv")
with open(GANHO, newline="", encoding="utf-8") as f:
    crit = {(r["sort"], r["tipo"], r["tamanho"]): float(r["ganho_pct"])
            for r in csv.DictReader(f)}
print("=== Tab4 (n=10000): pareada-s42 vs Criterion ===")
for c in ["merge/duplicates", "selection/random", "selection/almostsorted",
          "selection/duplicates", "selection/turtles", "selection/zigzag",
          "selection/inverted", "merge/random", "quick/almostsorted"]:
    s, t = c.split("/")
    r = next(x for x in resumo if x["sort"] == s and x["tipo"] == t
             and x["tamanho"] == "10000")
    print(f"  {c}: par gain={r['ganho_s42']} [{r['ganho_lo_s42']},{r['ganho_hi_s42']}]"
          f" sig={r['sig_s42']} p={r['p_melhora_s42']} dec={r['decisao']}"
          f" | crit={crit[(s, t, '10000')]:.2f}")
print("=== Tab6 (n=1000000): pareada-s42 vs Criterion ===")
for c in ["merge/random", "merge/zigzag", "merge/inverted",
          "merge/duplicates", "quick/random", "quick/almostsorted",
          "quick/duplicates", "quick/inverted"]:
    s, t = c.split("/")
    r = next(x for x in resumo if x["sort"] == s and x["tipo"] == t
             and x["tamanho"] == "1000000")
    print(f"  {c}: par gain={r['ganho_s42']} [{r['ganho_lo_s42']},{r['ganho_hi_s42']}]"
          f" sig={r['sig_s42']} p={r['p_melhora_s42']} dec={r['decisao']}"
          f" | crit={crit[(s, t, '1000000')]:.2f}")

# Aderencia global pareada-s42 vs. Criterion (mesma seed, estimadores
# distintos: media de log-razoes por vetor vs. razao das medias Criterion).
import math
pares = []
for r in resumo:
    c = crit.get((r["sort"], r["tipo"], r["tamanho"]))
    if c is not None:
        pares.append((float(r["ganho_s42"]), c))
n = len(pares)
mp = sum(p[0] for p in pares) / n
mc = sum(p[1] for p in pares) / n
cov = sum((p[0] - mp) * (p[1] - mc) for p in pares)
vp = sum((p[0] - mp) ** 2 for p in pares)
vc = sum((p[1] - mc) ** 2 for p in pares)
pearson = cov / math.sqrt(vp * vc)
mad = sum(abs(p[0] - p[1]) for p in pares) / n
print(f"aderencia pareada-s42 x Criterion: n={n} pearson={pearson:.4f} MAD={mad:.2f}pp")
