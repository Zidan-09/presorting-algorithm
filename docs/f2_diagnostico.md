# Diagnóstico F2 — Quicksort e Selection (1 página)

Ferramentas: `src/core/sort/quick_stats.rs` e `selection_stats.rs` (cópias
fiéis instrumentadas; algoritmo padrão intocado), `examples/diag_f2.rs`
(pools idênticos ao bench: seed `42^n^tipo`). Contadores de hardware
indisponíveis (sem AMD uProf; `perf-event` é Linux-only) — limitação mantida.

## Q1. Quicksort: de onde vem o ~n¹·⁴⁵ em Reversed/ordenado/Zigzag?

Resposta: **particionamento degenerado (não profundidade, não branches).**
Contagens (5 vetores do pool por célula):

| input | n | cmps | cmps/(n·ln n) | swaps | partições | maxdepth | imb médio |
|---|---|---|---|---|---|---|---|
| random | 10⁴/10⁵/10⁶ | 0.23M/2.9M/34M | **2.5/2.5/2.5** | 0.14M/1.8M/21M | 5.7k/57k/571k | 10/12/14 | 0.46 |
| reversed | 10⁴/10⁵/10⁶ | 1.1M/31.7M/976M | **11.4/27.6/70.6** | 0.5M/16M/490M | 5.8k/57k/577k | 7/8/9 | 0.50–0.55 |
| sorted (rev+pré) | 10⁴/10⁵/10⁶ | 1.4M/44M/1370M | **15.7/38.4/99.2** | — | 5.8k/57k/578k | 7/8/8 | 0.50–0.54 |
| zigzag | 10⁴/10⁵/10⁶ | 1.2M/34.6M/1067M | **12.7/30.1/77.3** | — | —/57k/570k | 8/9/10 | 0.50–0.51 |

Anatomia (primeiras divisões `(n,L,R)` em reversed@10⁵):
`(100000,50000,49999)` raiz balanceada, depois espinha
`(49999,49997,1)`, `(24998,1,24996)`, `(24996,2,24993)`,
`(24993,3,24989)`… — a mediana-de-três escolhe **sistematicamente
pivôs quase-extremos** nos subintervalos produzidos pelo DNF
(lado minoritário 1,2,3,…). Profundidade rasa (7–9) exclui
patologia de recursão: o lado grande é iterado (tail-call) e o
trabalho total explode (~n¹·⁴⁸). A teoria (mediana-de-três favorece
ordenados) prevê o oposto do medido → **degenerescência específica
desta implementação** (interação pivô/DNF em runs), não propriedade geral.
Referência no mesmo pool (mediana de 5 reps, 3 vetores), razão
quick/std: random ~5× (10⁴/10⁵/10⁶); reversed 30×/141×/**465×**;
sorted 43×/209×/**749×**.

**Decisão p/ §4.3:** remover a frase da mediana-de-três; reenquadrar os
−30…−42% como escopo-da-implementação; declarar a anomalia com os números
acima; gatilho exato da degenerescência = trabalho futuro (F6).

## Q2. Selection: os −12% em Reversed são reais? É preditor de desvios?

Resposta: **efeito real (~10–13%), intercalado, mas a história do preditor
precisa de refinamento.** Medição round-robin (5 rounds × mediana de 5):

| n | random | reversed puro | reversed+pré |
|---|---|---|---|
| 10⁴ | 20.69 | **18.00 (−13%)** | 20.03 |
| 10⁵ | 2056.5 | **1791.7 (−12.9%)** | 2034.3 |

Replica o artigo (18.26/21.49/20.52) → **não é deriva temporal**.
Contagens exatas (n=10⁴): cmps 50M iguais; updates 78k (random) /
25M (desc) / 0 (sorted); swaps 9989/5000/0. Fato novo: em entrada
descendente o Selection **ordena em n/2 swaps** (cada troca fixa 2
posições) e updates ≈ (n/2)² — verificado também em 10⁵ (2.5B/50k).
Assembly (`-C opt-level=3`, probe standalone — forma do loop robusta):
a comparação interna é um **branch condicional real (`jl`)** com custos
de caminho assimétricos (taken = salto ao topo; not-taken = mov + jmp).
Logo há dependência genuína de input no custo por iteração (~0.36 vs
0.40 ns) — família microarquitetural **confirmada**, mas "preditor erra
mais" é impreciso: ambos os padrões são estáveis; o refinamento
(taken-mais-barato-que-not-taken neste layout) e a quantificação com
contadores ficam para Linux/F6.

**Decisão p/ §4.3:** manter os números; apresentar as contagens
(25M updates + 5k swaps vs 0/0) e o paradoxo (mais trabalho, menos tempo);
trocar "atribuído a predição" por mecanismo de branch/layout evidenciado
pelo assembly, com caveats (probe standalone, sem contadores).

## Q3. Pendente p/ F6
Texto final do §4.3 conforme decisões acima; gatilho DNF exato, contadores
Linux e bootstrap pareado seguem em F3/F4/F6.
