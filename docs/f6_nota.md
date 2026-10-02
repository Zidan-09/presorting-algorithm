# F6 — Reescrita interpretativa (nota + trilha anti-invenção)

Data: 02/10/2026. `main.pdf`: 12 pp., 0 erros, 0 refs indefinidas.

## O que mudou (só prosa/interpretação; números já eram F5)

- Formalização (§3.4): derivação de primeira ordem E[ΔI]=n²/12+O(n)
  (condicional na troca espelhada: par remove 1 + 2/3 por k interior;
  ×P(troca)=1/2; soma nos n/2 pares) vs. 45,19% medido (resto = ajustes
  adjacentes, em aberto); regra de breakeven ΔI ≳ 5n
  (Cpre≈αn, α≈1ns/el, c≈0,21ns/inv) + breakevens por topologia via
  Tab `tab:cpre`; verificado contra todas as células adaptativas @10⁴.
- §4.3 Selection: hipótese de preditor descartada; mecanismo F2
  (branch/layout `jl` assimétrico, 0,36 vs 0,40 ns/it; contagens
  50M/78k-25M-0 updates, 9989/5000/0 swaps); contadores seguem p/ Linux.
- §4.3 Quicksort: degenerescência de particionamento (cmps/(n ln n)
  2,5 vs 11,4→70,6; spine DNF; depth 7–9; quick/std até 749×);
  −31–−38% como escopo-da-implementação; gatilho exato em aberto.
- Bubble: revalidação do deslocamento nas novas topologias.
- Abstract/Conclusão: 5 pools + bootstrap pareado; n≳10²; veredito
  baselines (8× vs quick, 36× vs sort_unstable, run-detection captura
  Reversed); future work sem os itens cumpridos.

## Trilha (número novo → fonte)

- 45,19%, ΔIs (11,3M/12,5M/10,3M/10k/5k), 0,21ns, Cpre 30,36/27,63/
  5,86/5,49µs, breakevens 144k/28k → `inversoes.csv`,
  `benchmark_consolidado.csv`, `cpre_consolidado.csv`, `ganho.csv`.
- Contagens F2 (50M, 78k/25M/0, 9989/5000/0, 2,5, 11,4→70,6,
  15,7→99,2, 12,7→77,3, depth 7–9, spine, 5×/30–465×/43–749×,
  0,36/0,40ns, `jl`) → `docs/f2_diagnostico.md` (gerável por
  `examples/diag_f2.rs`).
- n²/12, ≳5n → derivação analítica no próprio texto (não medição).
- Resto dos números do §4/Conclusão → inalterados desde F5 (rastreio
  em `scripts/valores_inline_f5.py` + `bench_logs/f5_vals.log`).
