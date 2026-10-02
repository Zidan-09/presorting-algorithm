#!/usr/bin/env python3
"""F5 (docs/plan2.md): gera figuras e corpos de tabela p/ artigo/bench2026.

Lê os CSVs consolidados de artigo/resultados/ (matriz F3 + resumo F4) e
escreve em artigo/bench2026/resultados/:
  fig_inversoes.csv, fig_tempos_<sort>.csv (x8), fig_precusto.csv,
  fig_gain_vs_n.csv (estendido: +stdunstable_random, +insertion_sawtooth,
  +insertion_organpipe, n pequenos, nan onde não planejado),
  tab3_body.tex, tab4_body.tex, tab4b_body.tex (baselines),
  tab4c_body.tex (novas topologias @10^4), tab5_body.tex, tab6_body.tex,
  tabcpre_body.tex.
Rótulos em inglês (coords simbólicas do pgfplots). Ganhos marcados:
  * = pareada significativa (seed 42); † = + robusta nos 5 pools.
Uso: python3 presorting-algorithm/scripts/gerar_figuras_f5.py
"""
import csv
import os

RAIZ = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                    "..", "..", "artigo", "resultados")
DEST = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                    "..", "..", "artigo", "bench2026", "resultados")

TIPOS_EN = {"random": "Random", "turtles": "Turtles", "zigzag": "Zigzag",
            "almostsorted": "AlmostSorted", "duplicates": "Duplicates",
            "inverted": "Reversed", "sawtooth": "Sawtooth",
            "organpipe": "OrganPipe", "fewruns": "FewRuns", "real": "Real"}
TIPOS_DISP = dict(TIPOS_EN)
TIPOS_DISP["almostsorted"] = "Almost Sorted"
SORTS_DISP = {"merge": "Merge Sort", "quick": "Quicksort",
              "insertion": "Insertion Sort", "bubble": "Bubble Sort",
              "selection": "Selection Sort",
              "stdunstable": "\\texttt{sort\\_unstable}",
              "stdstable": "\\texttt{sort}",
              "descreverse": "\\texttt{desc\\_reverse}"}
CLASSICOS = ["random", "turtles", "zigzag", "almostsorted", "duplicates",
             "inverted"]
NOVOS = ["sawtooth", "organpipe", "fewruns", "real"]
SORTS5 = ["merge", "quick", "insertion", "bubble", "selection"]
BASES = ["stdunstable", "stdstable", "descreverse"]
SORTS8 = SORTS5 + BASES
LIN4 = ["merge", "quick", "stdunstable", "stdstable"]


def ler(nome):
    with open(os.path.join(RAIZ, nome), newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def mil(x):
    s = f"{round(x):.0f}"
    out = ""
    for i, c in enumerate(s):
        if i > 0 and (len(s) - i) % 3 == 0:
            out += "."
        out += c
    return out


def dec(x, casas=2):
    return f"{x:.{casas}f}".replace(".", ",")


def us(ns):
    return float(ns) / 1000.0


bench = {(r["sort"], r["tipo"], r["tamanho"], r["branch"]): r
         for r in ler("benchmark_consolidado.csv")}
ganho = {(r["sort"], r["tipo"], r["tamanho"]): float(r["ganho_pct"])
         for r in ler("ganho.csv")}
inv = {(r["tipo"], r["tamanho"]): r for r in ler("inversoes.csv")}
cpre = {(r["tipo"], r["tamanho"]): r for r in ler("cpre_consolidado.csv")}
par = {(r["sort"], r["tipo"], r["tamanho"]): r
       for r in ler("efeito_pareado_resumo.csv")}


def marcador(sort, tipo, tam, g):
    """Símbolos de significância pareada na célula de ganho."""
    p = par.get((sort, tipo, tam))
    if p is None:
        return ""
    if p["decisao"] in ("ganho", "perda"):
        return "$^{\\dagger}$"
    if p["sig_s42"] == "1":
        return "$^{*}$"
    return ""


def celula_tempo(sort, tipo, tam):
    p = bench[(sort, tipo, str(tam), "puro")]
    c = bench[(sort, tipo, str(tam), "com_pre")]
    mp, cp = us(p["mean_ns"]), us(c["mean_ns"])
    hp = (us(p["ci_hi_ns"]) - us(p["ci_lo_ns"])) / 2.0
    hc = (us(c["ci_hi_ns"]) - us(c["ci_lo_ns"])) / 2.0
    return mp, hp, cp, hc


def linha_tempo(sort, tipo, tam, unidade="\\mu{}s"):
    mp, hp, cp, hc = celula_tempo(sort, tipo, tam)
    g = ganho[(sort, tipo, str(tam))]
    sinal = "+" if g >= 0 else "-"
    mk = marcador(sort, tipo, str(tam), g)
    return (f"{SORTS_DISP[sort]} & {TIPOS_DISP[tipo]} & "
            f"{mp:.2f} $\\pm$ {hp:.2f} & {cp:.2f} $\\pm$ {hc:.2f} & "
            f"{sinal}{abs(g):.1f}\\%{mk} \\\\")


def gravar(nome, conteudo):
    with open(os.path.join(DEST, nome), "w", encoding="utf-8",
              newline="") as f:
        f.write(conteudo)
    print(f"OK -> {nome}")


def tabular(cols, spec, cabecalho, corpos, small=True):
    """Fragmento = ambiente tabular* completo (o TBench-template quebra
    \\noalign após \\input dentro de alignment; o tabular inteiro precisa
    vir de um único arquivo)."""
    out = []
    if small:
        out.append("\\small")
    out.append(f"\\begin{{tabular*}}{{{cols}}}{{{spec}}}")
    out.append("\\toprule")
    out.append(cabecalho)
    out.append("\\midrule")
    out.extend(corpos)
    out.append("\\botrule")
    out.append("\\end{tabular*}")
    return "\n".join(out) + "\n"


# --- figuras ---
ordem_fig = CLASSICOS + NOVOS
with open(os.path.join(DEST, "fig_inversoes.csv"), "w", encoding="utf-8",
          newline="") as f:
    f.write("tipo,iniciais,pospre\n")
    for t in ordem_fig:
        r = inv[(t, "10000")]
        f.write(f"{TIPOS_EN[t]},{float(r['inversoes_iniciais']):.0f},"
                f"{float(r['inversoes_pos_pre']):.0f}\n")
print("OK -> fig_inversoes.csv")

for s in SORTS8:
    with open(os.path.join(DEST, f"fig_tempos_{s}.csv"), "w",
              encoding="utf-8", newline="") as f:
        f.write("tipo,puro_us,com_pre_us\n")
        for t in ordem_fig:
            p = bench[(s, t, "10000", "puro")]
            c = bench[(s, t, "10000", "com_pre")]
            f.write(f"{TIPOS_EN[t]},{us(p['mean_ns']):.3f},"
                    f"{us(c['mean_ns']):.3f}\n")
print("OK -> fig_tempos_x8.csv")

with open(os.path.join(RAIZ, "pre_custo.csv"), encoding="utf-8") as f:
    prec = {r["tamanho"]: float(r["pre_ns"]) for r in
            csv.DictReader(f, fieldnames=["tamanho", "pre_ns"])}
with open(os.path.join(DEST, "fig_precusto.csv"), "w", encoding="utf-8",
          newline="") as f:
    f.write("tamanho,pre_us\n")
    for t in sorted(prec, key=int):
        f.write(f"{t},{prec[t]/1000.0:.3f}\n")
print("OK -> fig_precusto.csv")

CURVAS = [("insertion", "random"), ("bubble", "random"),
          ("merge", "zigzag"), ("merge", "inverted"),
          ("quick", "random"), ("quick", "almostsorted"),
          ("stdunstable", "random"), ("insertion", "sawtooth"),
          ("insertion", "organpipe")]
NOMES_CURVAS = {"insertion/random": "insertion_random",
                "bubble/random": "bubble_random",
                "merge/zigzag": "merge_zigzag",
                "merge/inverted": "merge_inverted",
                "quick/random": "quick_random",
                "quick/almostsorted": "quick_almostsorted",
                "stdunstable/random": "stdunstable_random",
                "insertion/sawtooth": "insertion_sawtooth",
                "insertion/organpipe": "insertion_organpipe"}
TAMS_GAIN = [16, 32, 64, 128, 256, 512, 1000, 5000, 10000, 100000,
             1000000, 10000000]
with open(os.path.join(DEST, "fig_gain_vs_n.csv"), "w", encoding="utf-8",
          newline="") as f:
    f.write("n," + ",".join(NOMES_CURVAS[f"{s}/{t}"]
                            for s, t in CURVAS) + "\n")
    for n in TAMS_GAIN:
        vals = []
        for s, t in CURVAS:
            v = ganho.get((s, t, str(n)))
            vals.append(f"{v:.2f}" if v is not None else "nan")
        f.write(f"{n}," + ",".join(vals) + "\n")
print("OK -> fig_gain_vs_n.csv")

# --- tabelas (fragmentos = tabular* completo) ---
H_TEMPO = "Algorithm & Type & Pure & With pre & Gain\\\\"
H_INV = "Type & Inversions & Post-pre & Reduction\\\\"
H_PRE = "Size & Pre ($\\mu$s) & ns/element\\\\"
H_CPRE = "Type & $C_{\\text{pre}}$ at $10^4$ & $C_{\\text{pre}}$ at $10^6$\\\\"

linhas = []
for t in ordem_fig:
    r = inv[(t, "10000")]
    ini, pos = float(r["inversoes_iniciais"]), float(r["inversoes_pos_pre"])
    red = (ini - pos) / ini * 100.0 if ini > 0 else 0.0
    mk = "$^{*}$" if t == "zigzag" else ("$^{**}$" if t == "inverted" else "")
    linhas.append(f"{TIPOS_DISP[t]} & {mil(ini)} & {mil(pos)} & "
                 f"{dec(red)}\\%{mk} \\\\")
gravar("tab3_tabular.tex",
       tabular("\\columnwidth",
               "@{\\extracolsep\\fill}lrrr@{\\extracolsep\\fill}",
               H_INV, linhas))

linhas = [linha_tempo(s, t, 10000) for s in SORTS5 for t in CLASSICOS]
gravar("tab4_tabular.tex",
       tabular("\\textwidth",
               "@{\\extracolsep\\fill}llrrr@{\\extracolsep\\fill}",
               H_TEMPO, linhas, small=False))

linhas = [linha_tempo(s, t, 10000) for s in BASES for t in CLASSICOS]
gravar("tab4b_tabular.tex",
       tabular("\\textwidth",
               "@{\\extracolsep\\fill}llrrr@{\\extracolsep\\fill}",
               H_TEMPO, linhas, small=False))

linhas = [linha_tempo(s, t, 10000) for s in SORTS8 for t in NOVOS]
gravar("tab4c_tabular.tex",
       tabular("\\textwidth",
               "@{\\extracolsep\\fill}llrrr@{\\extracolsep\\fill}",
               H_TEMPO, linhas, small=False))

with open(os.path.join(RAIZ, "pre_custo.csv"), encoding="utf-8") as f:
    rows = [(int(r["tamanho"]), float(r["pre_ns"])) for r in
            csv.DictReader(f, fieldnames=["tamanho", "pre_ns"])]
linhas = [f"{mil(t)} & {dec(v/1000.0)} & {dec(v/t)} \\\\"
          for t, v in sorted(rows)]
gravar("tab5_tabular.tex",
       tabular("\\columnwidth",
               "@{\\extracolsep\\fill}rrr@{\\extracolsep\\fill}",
               H_PRE, linhas))


def linha_ms(sort, tipo):
    p = bench[(sort, tipo, "1000000", "puro")]
    c = bench[(sort, tipo, "1000000", "com_pre")]
    mp, cp = us(p["mean_ns"]) / 1000.0, us(c["mean_ns"]) / 1000.0
    hp = (us(p["ci_hi_ns"]) - us(p["ci_lo_ns"])) / 2.0 / 1000.0
    hc = (us(c["ci_hi_ns"]) - us(c["ci_lo_ns"])) / 2.0 / 1000.0
    g = ganho[(sort, tipo, "1000000")]
    sinal = "+" if g >= 0 else "-"
    mk = marcador(sort, tipo, "1000000", g)
    return (f"{SORTS_DISP[sort]} & {TIPOS_DISP[tipo]} & "
            f"{mp:.2f} $\\pm$ {hp:.2f} & {cp:.2f} $\\pm$ {hc:.2f} & "
            f"{sinal}{abs(g):.1f}\\%{mk} \\\\")


linhas = [linha_ms(s, t) for s in LIN4 for t in CLASSICOS]
linhas.append("\\midrule")
linhas += [linha_ms(s, t) for s in LIN4
           for t in ["sawtooth", "organpipe", "fewruns"]]
gravar("tab6_tabular.tex",
       tabular("\\textwidth",
               "@{\\extracolsep\\fill}llrrr@{\\extracolsep\\fill}",
               H_TEMPO, linhas, small=False))

linhas = []
for t in ordem_fig:
    r4 = cpre[(t, "10000")]
    v4 = f"{us(r4['pre_only_ns']):.2f}"
    if (t, "1000000") in cpre:
        v6 = f"{us(cpre[(t, '1000000')]['pre_only_ns']):.2f}"
    else:
        v6 = "{---}"
    linhas.append(f"{TIPOS_DISP[t]} & {v4} & {v6} \\\\")
gravar("tabcpre_tabular.tex",
       tabular("\\columnwidth",
               "@{\\extracolsep\\fill}lrr@{\\extracolsep\\fill}",
               H_CPRE, linhas))

# --- caderno de valores p/ números inline do §4 ---
print("=== VALORES-F5 ===")
for t in ordem_fig:
    r = inv[(t, "10000")]
    ini, pos = float(r["inversoes_iniciais"]), float(r["inversoes_pos_pre"])
    red = (ini - pos) / ini * 100.0 if ini > 0 else 0.0
    print(f"inv10k.{t} = {mil(ini)} -> {mil(pos)} ({dec(red)}%)")
for s, t, n in [("insertion", "random", 10000), ("bubble", "random", 10000),
                ("insertion", "zigzag", 10000), ("bubble", "zigzag", 10000),
                ("insertion", "inverted", 10000),
                ("bubble", "inverted", 10000)]:
    mp, hp, cp, hc = celula_tempo(s, t, n)
    print(f"t10k.{s}.{t} = {mp:.2f}±{hp:.2f} -> {cp:.2f}±{hc:.2f} "
          f"(+{ganho[(s,t,str(n))]:.2f}%)")
for s, t, n in [("merge", "random", 10000), ("quick", "duplicates", 10000),
                ("quick", "inverted", 10000), ("selection", "inverted", 10000),
                ("merge", "zigzag", 10000), ("merge", "inverted", 10000)]:
    mp, hp, cp, hc = celula_tempo(s, t, n)
    print(f"t10k.{s}.{t} = {mp:.2f}±{hp:.2f} -> {cp:.2f}±{hc:.2f} "
          f"({ganho[(s,t,str(n))]:+.2f}%)")
print("precusto:", "; ".join(
    f"{mil(t)}: {dec(v/1000.0)}us/{dec(v/t)}ns-el"
    for t, v in sorted(
        {int(k): vv for k, vv in prec.items()}.items())))
print("cpre10k:", "; ".join(
    f"{t}={us(cpre[(t,'10000')]['pre_only_ns']):.2f}us"
    for t in ordem_fig))
