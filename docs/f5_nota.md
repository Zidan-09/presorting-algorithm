# F5 — Regeneração de figuras/tabelas (nota de fechamento)

Data: 02/10/2026. `main.pdf`: 12 pp., 0 erros, 0 refs indefinidas
(`pdflatex+bibtex+2×pdflatex`, MiKTeX). Overfulls restantes: 2,9 pt
preexistente (linha de algoritmo) + 11× aviso de 253 pt da classe em
shipout de floats largos — o baseline original (9 pp.) já emitia 8× o
mesmo aviso; verificado via rebuild com `git stash` (comportamento da
classe, não regressão).

## Scripts (automatização F5)

- `presorting-algorithm/scripts/gerar_figuras_f5.py` — lê
  `artigo/resultados/*.csv` e escreve em `artigo/bench2026/resultados/`:
  `fig_inversoes.csv`, `fig_tempos_<8 sorts>.csv`, `fig_precusto.csv`,
  `fig_gain_vs_n.csv` (9 curvas, `nan` onde não planejado), fragmentos
  `tab3/tab4/tab4b/tab4c/tab5/tab6/tabcpre_tabular.tex` (= tabular*
  completo, pois o TBench-template quebra `\noalign` após `\input`
  dentro de alignment — testado e confirmado).
- `presorting-algorithm/scripts/valores_inline_f5.py` — caderno de valores
  p/ números inline do §4.

## Mudanças no main.tex

Tab2 +4 topologias; Tab3 10 linhas; Tab4 30 linhas refrescadas + marcas
`*/†` (pareada s42 / robusta 5 pools); NOVAS Tab4b (baselines 18 lin),
Tab4c (novas topologias 32 lin), Tab Cpre (10 lin, `tab:cpre`); Tab5/Tab6
refrescadas (Tab6: 36 lin c/ `\midrule`); Fig.1 10 barras; Fig.3 10
categorias; Fig. gain×n 9 curvas (eixo até 10⁷); §3.1/§3.3 (protocolo +
critério pareado F4)/§3.5 reescritos factualmente; Conclusão e Abstract
com números novos; future work (ii)/(iii) atualizados (baselines, Cpre,
pareada e multiseed saem da lista). Apêndice de medianas/outliers:
adiado (quartis disponíveis em `benchmark_consolidado.csv`).

## Deixado p/ F6 (interpretação, sem números pendentes)

Reenquadrar Abstract/§4.3/Conclusão (OrganPipe/FewRuns imunes, baselines
vencem o método, anomaly quick, §2.4 vs. medido, threats residuais) e
decidir marcação final das células `instavel` nas tabelas.
