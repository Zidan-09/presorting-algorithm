# F4 — Estatística pareada (nota de fechamento)

Data: 02/10/2026. Escopo executado: células das Tabelas 4 (n=10⁴, todas) e 6
(n=10⁶, quasilineares) — 116 células × 5 pools (seeds 42–46). Demais tamanhos
mantêm a regra conservadora antiga até F5/F6 decidir estender.

## Método (texto-base p/ §3.3 em F6)

Para cada célula e cada seed, os mesmos R=50 vetores do pool do bench
(`ChaCha8Rng::seed_from_u64(seed ^ n ^ tipo)`, slots 0–49 idênticos aos do
Criterion) são ordenados nos dois braços (puro vs. pré+sort), alternando a
ordem dos braços por repetição (controle de deriva, cf. `service.rs`).
Estatística: média das log-razões por vetor `mean(ln(T_pre/T_puro))`;
IC 95% por bootstrap percentílico (B=10.000 reamostragens, RNG
determinística por célula). Ganho% = (1−exp(lr))×100 com IC transformado.
Repetições internas adaptativas por célula (alvo ~50 µs/medição) eliminam
zeros de granularidade do relógio em n pequeno. Validação fora do timing:
ordenação (toda rep.) + permutação vs. `sort_unstable` (rep 0).

Decisão por célula (5 pools): `ganho` (5/5 sig +), `perda` (5/5 sig −),
`neutro` (0/5 sig), `instavel` (sig parcial ou flip de sinal entre pools).

## Resultados

- 116 células: ganho 32, perda 45, neutro 17, instavel 22. Flips de sinal: 23.
- Aderência pareada-s42 × Criterion: pearson 0,9766, MAD 4,50 pp (estimadores
  distintos: média de log-razões vs. razão das médias; diferenças esperadas,
  ex.: merge/inverted @10⁶ +12,9 vs. +6,0; selection/inverted @10⁴ −11,8 vs.
  −9,1 — F6 reporta os ICs pareados como primários p/ significância).
- Tab4: merge/duplicates, selection random/almostsorted/duplicates →
  `neutro` (confirma regra antiga). **selection/zigzag → `neutro` pela
  pareada** (regra antiga dizia sig): resolve a violação §8.4-item 4 — o texto
  "neutro" estava certo, a regra estava errada. selection/turtles: sig em s42
  (+3,58 [2,18;4,98]) mas `instavel` entre pools → rebaixar p/ "pequeno e
  pool-dependente". selection/inverted: `perda` robusta (−11,79).
- Tab6: merge/random → `neutro` (confirma). merge/zigzag e merge/inverted →
  `ganho` robusto. merge/duplicates: sig s42 (+8,26) mas `instavel` → caveat.
  quick/random, quick/duplicates, quick/inverted → `perda` robusta.
- **Quicksort/AlmostSorted @10⁶: `neutro` + flip** ([+3,8;−2,9;+3,7;−5,5;−5,4],
  s42 +3,81 [−4,08;+10,21], p=0,82) — o "+11,1%" pontual não é robusto; F6
  deve remover a exceção e declarar instabilidade entre pools (responde F4.3).
  quick/zigzag @10⁴ também flipa ([−3,5;−3,5;−2,6;+2,8;+1,5]).
- Flips restantes são efeitos ≈0 (selection ±2%, insertion/bubble em
  organpipe/fewruns ±3%) — ruído em torno do zero, consistente c/ neutralidade.

## Recomendação p/ F5/F6

Tabelas 4/6 mantêm médias±IC do Criterion (já exportadas) + marcadores de
significância pareada de `efeito_pareado_resumo.csv` (colunas `sig_s42`,
`decisao`); §3.3 reescrito com o procedimento acima; §4.3 e Conclusão
atualizados nos pontos marcados.

## Artefatos (aceite F4)

- `presorting-algorithm/examples/efeito_pareado.rs` — experimento (FP_SEEDS,
  FP_TAMS, FP_R=50, FP_BOOT=10000, FP_OUT).
- `presorting-algorithm/scripts/resumir_pareado.py` — agregação + confronto.
- `artigo/resultados/efeito_pareado.csv` (581 lin: 400 @10⁴ + 180 @10⁶, 0 inválidas).
- `artigo/resultados/efeito_pareado_resumo.csv` (116 células + `decisao`).

Reprodução: `rm -f ../artigo/resultados/efeito_pareado.csv` e rodar o example
por tamanho (`FP_TAMS="10000"` ~4 min; `FP_TAMS="1000000"` ~45 min p/ 5 seeds),
depois o script python.
