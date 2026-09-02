# Plano Mestre — Presorting Algorithm (Auditoria e Correções)

Fontes: código-fonte (crate `algoritmo`, Rust), `benches/benchmark.rs`, `examples/exportar_resultados.rs`, `artigo/main.tex`, artefatos em `artigo/resultados/`, dados atuais em `presorting-algorithm/target/criterion/experimentos_ordenacao/`.
Data da auditoria: 2026-09-01. Ambiente: Windows (PowerShell). Todos os testes passam (`cargo test --release` -> 4/4 OK).

---

## 1. Objetivo

Restaurar a integridade científica do experimento (dados, artefatos e artigo em sincronia com o código atual), tornar o pipeline de medição/exportação confiável e reprodutível, e reescrever as conclusões do artigo que foram refutadas pela implementação atual do Quicksort (particionamento de três vias, DNF).

## 2. Resumo Executivo

- **Os dados atuais do Criterion (n=1000–100000) são confiáveis**: foram reproduzidos localmente com os mesmos seeds (ver §6.3). O problema NÃO é o harness de medição.
- **Os artefatos e o artigo estão desatualizados em relação ao código (era pré-DNF)**: a Tabela 4 do artigo reporta Quicksort/Tartarugas ≈ 3,24 ms e Quicksort/Duplicados ≈ 9,82 ms; a implementação atual mede ≈ 149 µs e ≈ 40 µs. As conclusões de "ganhos expressivos" do Quicksort (+75,3 % Tartarugas, +53,7 % Zigue-zague, +48,9 % Invertido) **são refutadas** pelos dados atuais: o pré-processamento agora **degrada** o Quick nos 6 tipos (até −34 %).
- **n=1.000.000 nunca foi medido** e a Tabela 5 do artigo contém valores de pré-custo @100k e @1M obtidos por **extrapolação linear exata** (×10, ×100 de @10k), sem nenhum artefato que os sustente.
- **O pipeline de exportação já destruiu dados no histórico** (commit `570ed17` esvaziou `inversoes.csv`, `pre_custo.csv`, `fig_inversoes.csv`, `fig_precusto.csv`, `cli_cross.csv` – ainda vazios no working tree); `tabelas.tex` tem Tabelas 3 e 5 sem corpo. O exportador não falha quando faltam entradas.

Decisão importante (D3, §9): se mantido o Quick DNF, o artigo precisa enquadrar um resultado cientificamente interessante — a redução de inversões (até 100 %) **não se traduz em ganho de tempo** neste algoritmo, pois o Quick DNF é mais lento em entrada ordenada (≈ 480 µs em `quick(sorted@10k)`) do que em invertida (≈ 358 µs) ou aleatória (≈ 357 µs); ver §6.2.

## 3. Riscos à Validade (140 palavras)

Os resultados reportados no artigo são incompatíveis com o código atual: métricas de Quicksort vêm de uma implementação anterior (Lomuto) e a Tabela 5 contém extrapolações não declaradas. O pipeline de exportação sobrescreve dados sem validação, gerando artefatos vazios já commitados. Nenhum dado existe para n=1.000.000, embora o artigo afirme cobertura até 10⁶. Falta validação de corretude no harness Criterion e as estatísticas de inversão (CLI, seed 42) usam arrays diferentes dos usados nas medições de tempo (seed com XOR). Os ganhos não têm análise de significância; variâncias heterogêneas por tipo não são reportadas; sem registro de ambiente, concordância entre medição e código foi restaurada por verificação manual direta, não por infraestrutura.

## 4. Metodologia Corrente (o que o sistema faz hoje)

### 4.1 Algoritmos (`src/core/sort/`)
`bubble.rs`, `insertion.rs`, `selection.rs` — O(n²) clássicos. `merge.rs` — mergesort com otimização de pular a recópia da metade exaurida (corretude verificada). `quick.rs` — quicksort recursivo com partição de três vias (DNF/bandeira holandesa) + pivô mediana-de-3 + iteração na partição maior (tail-call); índices `i32` (`quick.rs:4,7`).

### 4.2 Pré-processamento (`pre_proc.rs`)
Um único passe simétrico (i do início, j do fim): troca par mais externo (i,j) quando `a[i]>a[j]`, mais trocas adjacentes locais (i,i+1) e (j,j−1) dentro de limites. Custo ~O(n). **Invariante**: não aumenta inversões (verificado por força bruta §6.4; sem prova formal).

### 4.3 Datasets (`src/utils/gerador.rs`)
`Random` (permutação embaralhada), `Inverted` (`size−i`), `Zigzag` (i par→i; i ímpar→`size−i`), `Turtles` (1ª metade `size+i`, 2ª `i%10`), `Duplicates` (valores 0..3), `AlmostSorted` (0..size com `size/100` trocas). Inverted/Zigzag/Turtles **não usam rng** → os 50 arrays do pool são idênticos (variância só de ruído).

### 4.4 CLI (`src/services/service.rs`, `src/main.rs`)
`executar_teste(size, array_type, sort_type, seed=42, repeticoes=7)`: 2 iterações de warm-up; por repetição gera 1 array, clona para puro e com_pre, alterna ordem; conta inversões apenas na rep. 0; mede tempo com `Instant` (fora do cronômetro ficam geração, inversões e validação `windows(2)`); métricas de heap via `stats_alloc::Region` (cobrem o trecho inteiro, incluindo pós-cronômetro) e cache-misses via `perf_event` (apenas Linux; 0 no Windows). Saída `CSV_PURO|COM_PRE|PRE_SOZINHO|...` → `artigo/resultados/cli/*.txt`.

### 4.5 Harness Criterion (`benches/benchmark.rs`, `src/services/servicebench.rs`)
Uma função por (algoritmo, tipo, tamanho, ramo) → 5×6×5×2 = 300 IDs de benchmark. Pool de `AMOSTRAS_POR_POOL=50` arrays por (tamanho, tipo), seeds `42 ^ tamanho ^ tipo`, compartilhado entre puro/com_pre e entre os 5 algoritmos (comparação "pareada"). Config atualmente **uniforme** (working copy não commitado): `sample_size=50`, `warm_up_time=5s`, `measurement_time` default; `LargeInput` p/ 1M. `medir_puro/medir_com_pre` **não validam** corretude.

### 4.6 Exportação (`examples/exportar_resultados.rs`)
Lê `target/criterion/experimentos_ordenacao/...` (caminho minúsculo × grupo "Experimentos_Ordenacao") e `artigo/resultados/cli/*.txt`; escreve `benchmark_consolidado.csv`, `ganho.csv`, `fig_*.csv`, `tabelas.tex`. **Não valida presença de dados**: se faltam arquivos, gera tabelas/CSVs vazios sem aviso (ver P03).

### 4.7 Artigo e artefatos (`artigo/`)
`main.tex` embute números manualmente (Tabelas 3–5, Figuras, discussão); `tabelas.tex` deve ser gerado pelo exportador mas está parcialmente vazio; `resultados/cli/*.txt` possuem dados finos (CLI 1000–20000); `benchmark_consolidado.csv` (180 linhas: só 1000/5000/10000) tem dados da era Lomuto para o Quicksort.

## 5. Problemas Identificados

Severidades: **[CRÍTICO]** invalida publicação; **[ALTO]** afeta conclusões/confiança; **[MÉDIO]** metodologia/código; **[BAIXO]/[OPCIONAL]** manutenibilidade.

| ID | Severidade | Local | Resumo |
|---|---|---|---|
| P01 | CRÍTICO | artigo + artefatos | Resultados/artigo fora de fase com o Quick DNF atual (Tabela 4, Figuras, discussão) |
| P02 | CRÍTICO | main.tex Tabela 5 | n=1.000.000 nunca medido; pré-custo @100k/@1M = extrapolação linear apresentada como medição |
| P03 | CRÍTICO | exportar_resultados.rs + histórico | Exportador sobrescreve/omite dados silenciosamente; artefatos vazios commitados (570ed17) |
| P04 | CRÍTICO | main.tex Resultados/Discussão | Conclusões do Quicksort refutadas por dados atuais (pre agora prejudica Quick; ver §6.2) |
| P05 | ALTO | servicebench.rs | Harness Criterion sem validação de corretude (ordenado + permutação) |
| P06 | ALTO | benchmark.rs | Config uniforme inadequada p/ O(n²)@100k/1M; rodada completa (300 IDs) pode ser inviável |
| P07 | ALTO | CLI vs Criterion | Inversões (seed 42) medidas em arrays diferentes dos de timing (seed XOR) |
| P08 | ALTO | pre_proc.rs | Invariante sem prova formal e sem validação em escalas grandes |
| P09 | MÉDIO | artigo | Ganhos sem análise de significância (ICs sobrepostos em vários casos) |
| P10 | MÉDIO | repo | Sem manifesto de ambiente (CPU/OS/rustc/commit) para reprodutibilidade |
| P11 | MÉDIO | quick.rs | Índices `i32` (idiomaticamente `usize`; risco teórico de overflow) |
| P12 | MÉDIO | exportar_resultados.rs | Caminho do Criterion com case inconsistente (Windows ok / Linux quebra) |
| P13 | MÉDIO | service.rs | Métricas memória/cache cobrem setup fora do cronômetro; ok/no-op e não reportadas |
| P14 | MÉDIO | benchmark.rs | Variância heterogênea: 50 arrays idênticos para os tipos determinísticos |
| P15 | MÉDIO | tipos.rs / mod.rs | Enums em inglês (`AlmostSorted`, `Turtles`) vs artigo em PT; `contar_inversoes` sem re-export |
| P16 | BAIXO | benchmark.rs | `warm_up 5s` pode ser insuficiente p/ cenários lentos (estabilização térmica) |
| P17 | BAIXO | benchmark.rs | Seeds por XOR (`42^tamanho^tipo`) com risco de colisão |
| P18 | OPCIONAL | Cargo.toml | Perfil release não pinado (LTO/codegen-units) — linkagem pode variar tempos entre builds |

### P01 — Artefatos de resultados fora de fase com o código atual
**Arquivos**: `artigo/resultados/benchmark_consolidado.csv`, `fig_tempos_quick.csv`, `fig_ganho.csv`, `ganho.csv`, `tabelas.tex`, `artigo/main.tex`.
**Evidência** (n=10000): artefatos (era Lomuto) → Quick Tartarugas puro 3.240.293 ns, Duplicados 9.815.640 ns, Zigue-zague 683.598 ns, Invertido 144.616 ns, Aleatório 334.509 ns. Dados atuais reproduzidos (§6.2): Tartarugas ≈ 149 µs, Duplicados ≈ 40 µs, Zigue-zague ≈ 397 µs, Invertido ≈ 352 µs, Aleatório ≈ 370 µs (diferença de até ~240×).
**Impacto**: Tabela 4 (colunas Quick), Figura 2 (curvas Quick) e toda a discussão associada estão errados para a implementação atual. Contribui para o parecer do avaliador (`parecer_artigo.md`).
**Correção**: rodar benchmark completo (T4), reexportar artefatos a partir dos dados novos (T5), reescrever análise (T7).
**Validação**: cada célula da Tabela 4 rastreável ao `estimates.json` correspondente (§10).

### P02 — n=1.000.000 nunca medido; Tabela 5 com extrapolação linear
**Arquivos**: `main.tex` Tabela 5 (linhas 100.000 = 121,67 µs → 1.000.000 = 1.216,7 µs); `target/criterion/...` (sem `tamanho_1000000` para quaisquer dos 30 grupos × 2 ramos).
**Evidência**: média CLI @10000 = 12.166,67 ns (média dos 6 tipos: (27,3+24,5+4,6+4,8+4,8+7,0) µs/6); Tabela 5 = ×10, ×100 exatos. Não há dados de CLI ≥ 100.000 nem estimativas do Criterion em 1M; `benchmark_consolidado.csv` só tem 1000/5000/10000.
**Impacto**: afirmação "experimentos até 10⁶ elementos" insustentada; extrapolação apresentada sem ressalva — risco à integridade (azul da pesquisa).
**Correção**: T4 mede 1M onde viável (merge/quick); O(n²)@1M segue decisão D1 (§9). Linhas não medidas devem ser removidas ou rotuladas explicitamente como "estimativa linear" — nada de números fabricados.
**Validação**: existência de artefato para cada célula reportada (teste T6).

### P03 — Exportador destrói/omite dados silenciosamente
**Arquivos**: `examples/exportar_resultados.rs`; commit `570ed17` (esvaziou `inversoes.csv`, `fig_inversoes.csv`, `fig_precusto.csv`, `pre_custo.csv`, `cli_cross.csv` — 24–30 linhas removidas, arquivos vazios commitados); `tabelas.tex` §§3/5 sem corpo.
**Evidência**: dados legítimos existem em `resultados/cli/*.txt` (ex.: `random_10000.txt` → `CSV_PRE_SOZINHO,27300`) e no Criterion, porém CSVs estão vazios. O exportador não emite erro quando entradas faltam.
**Impacto**: perda de dados no histórico; impossibilidade de conferir Tabelas 3/5/Figuras; qualquer reprodução pode repetir o problema.
**Correção**: T5 — validar entradas obrigatórias (panic/exit ≠ 0), escrita atômica (temp→rename), validação de shape (linhas > 0, contagens esperadas), `manifest.json` (commit, rustc, data, CPU), e teste automático T6.
**Validação**: `cargo run --example exportar_resultados` → nenhum CSV vazio; T6 falha se artefato for mais antigo que o código-fonte.

### P04 — Conclusões do Quicksort refutadas pelos dados atuais
**Arquivos**: `main.tex` (discussão, ~linha 458): "Quicksort obtém ganhos expressivos em entradas estruturadas (Tartarugas: +75,3 %, Zigue-zague: +53,7 %, Invertido: +48,9 %)...".
**Evidência** (§6.2): com DNF, pré-processamento **degrada** o Quick: Aleatório −5,2 %, Tartarugas −4,3 %, Zigue-zague −4,2 %, Quase ordenado −3,7 %, Duplicados −30,2 %, Invertido −34,3 %.
**Causa raiz**: o pré deixa o vetor invertido totalmente ordenado (0 inversões) e o Quick DNF é mais lento em entrada ordenada (≈ 480 µs) que em invertida (≈ 358 µs) ou quase ordenada (≈ 245 µs) — ver §6.2.
**Impacto**: mensagem central do artigo muda; resumo/abstract a descrevem.
**Correção**: T7 reescreve o enquadramento (redução de inversões ≠ ganho de tempo neste algoritmo) — ver decisão D3.
**Validação**: os números citados no artigo devem bater com os artefatos regenerados.

### P05 — Harness Criterion sem validação de corretude
`servicebench.rs` não verifica ordenação nem preservação de permutação (sinalizado também no `AGENTS.md`).
**Correção**: T2 — no setup, validar cada array do pool (sorted + permutação vs `sort_unstable`) para puro e com_pre; falha aborta a rodada com mensagem clara. Fecha também P07 ao computar inversões dos **mesmos** arrays do pool.

### P06 — Config de medição inadequada e rodada potencialmente inviável
O working copy removerá a configuração especial por algoritmo/tamanho (amostras/duração menores para O(n²), `BatchSize::PerIteration`); hoje tudo é `sample_size=50`, `warm_up 5s`, `measurement_time` default. Medido: `selection_random@100000` ≈ 2,13 s/iter → 50 amostras ≈ 2 min (só puro; a corrida de 2026-09-01 foi interrompida justamente no selection@100k — 4 cenários de selection@100k ausentes). Projeção @1M O(n²): ~3 min/iter → ~2,7 h/cenário → inviável.
**Correção**: T3 — matriz de config por perfil (O(n·logn) vs O(n²) e por tamanho) documentada, rodadas em lotes retomáveis (filtro por cenário), relatório de avisos do Criterion ("unable to complete N samples") que invalida a célula; D1 §9.

### P07 — Inversões medidas em arrays diferentes dos de timing
CLI usa seed 42 direto; Criterion usa `42 ^ tamanho ^ tipo`. Aleatório@10000: inversões iniciais 24.993.860 (CLI, seed 42) ≠ pool do Criterion (outra permutação). **Correção**: T2 (inversões por array do pool) + T5/T7 usam esse dataset para as Tabelas 3/Figura.

### P08 — Invariante do pré sem prova formal
Força bruta aqui realizada: 0 violações em 496.494 casos (n≤9 permutações; n≤8 com duplicatas); testes chegam a n=1000. **Correção**: T2 valida em todos os pools/tamanhos do benchmark; T7 enquadra como verificação empírica (ou prova sintética opcional).

### P09 — Sem significância estatística dos ganhos
Relatórios usam média/quartis. Vários ganhos são pequenos (ex.: quick quase ordenado 246 µs→255 µs; merge ±7 µs). **Correção**: T5 — estatística pareada por pool (os 50 pares, indexados por pool), bootstrap da diferença + Wilcoxon signed-rank; regra: "ganho" só se CI da diferença exclui 0 (e prático ≥ 3 %).

### P10 — Sem manifesto de ambiente
**Correção**: T5 grava `env.json` (OS, CPU, rustc, commit, seeds, versões) junto aos artefatos; artigo descreve limitação de frequência/térmicas Windows.

### P11–P18 — Técnica/manutenibilidade
P11 (`i32`→`usize`, scaffold com testes), P12 (case do caminho), P13 (documentar/eliminar métricas), P14 (documentar variância heterogênea), P15 (nomes PT + re-exports), P16/P17/P18 (config/seeds/perfil release) → tarefa T8.

## 6. Evidências Verificadas (dados em mãos)

### 6.1 Datasets determinísticos
`Inverted`@10000 via crate: 49.995.000 inversões → após pré **0** (`pre_sorted=true`). Invariante mantido.

### 6.2 Quick DNF atual (n=10000; local, mesma máquina/seeds; µs)
| Entrada | puro | com_pre | Δ pré |
|---|---|---|---|
| Aleatório | 369,952 | 389,299 | −5,2 % |
| Invertido | 351,993 | 472,730 | −34,3 % |
| Zigue-zague | 396,652 | 413,207 | −4,2 % |
| Tartarugas | 149,124 | 155,603 | −4,3 % |
| Duplicados | 40,403 | 52,612 | −30,2 % |
| Quase ordenado | 246,489 | 255,543 | −3,7 % |

Anomalia reproduzida localmente: `quick(sorted)` ≈ 480 µs > `quick(inverted)` ≈ 358 µs ≈ `quick(random)` ≈ 357 µs > `quick(quase ordenado)` ≈ 245 µs. O Criterion (estimates.json, dados de 2026-09-01 19:39) reproduz exatamente esses valores → **o harness mede o que o código faz**; o problema é do artigo/artefatos.

### 6.3 O(n²) e merge atuais (n=10000, ns por célula)
`bubble_inverted` 19.970.237 → **8.436** (pré ordena → 1 passe); `insertion_inverted` 10.016.836 → **9.361**; `bubble_zigzag` 19.013.024 → **16.131**; `insertion_zigzag` 5.182.861 → **10.722**; `bubble_random` 26.518.702 → 21.494.984; `insertion_random` 5.182.082 → 2.915.727; `insertion_duplicates` 3.416.423 → 1.397.369; `selection_*` ≈ 20 ms inalterado pelo pré; `merge_*` ~±7 µs; `quick_*`: ver §6.2. Conclusão: claims de bubble/insertion (ganhos de 60–99 %) **permanecem válidos**; selection "sem ganho" **permanece**; merge "misto" **permanece**; **somente o Quick muda** (e muda de sinal).

### 6.4 Inventário do Criterion (2026-09-01 19:35–19:46)
Completo e fresco em 1000/5000/10000 (60/60 células por ramo). `100000`: falta selection (4 células: `selection_almostsorted_com_pre`, `selection_duplicates_*`, `selection_inverted_*`). `1000000`: **ausente para os 60 IDs** (nenhum grupo tem `tamanho_1000000`).

## 7. Metodologia Recomendada

1. **Fonte única de verdade**: `target/criterion/…` + `resultados/cli/*.txt`; `exportar_resultados` regenera todos os CSVs/tabelas/figuras; `main.tex` **não** embute números manualmente (usa `\input{tabelas.tex}` e as figuras dos CSVs).
2. **Design pareado**: puro e com_pre sobre os MESMOS 50 arrays (já é assim por pool); **análise pareada** (diferenças por pool-index), não teste não-pareado.
3. **Validação sempre**: corretude + soma de inversões por array **fora** do cronômetro (T2); rodada abortada se qualquer célula falhar ou tiver aviso do Criterion.
4. **Config por perfil** (T3): documentar amostras/duração por (algoritmo, tamanho); rejeitar rodadas com `unable to complete … samples`.
5. **Declaração de escopo**: reportar explicitamente o que foi medido, o que foi excluído (O(n²)@1M) e o que é estimativa (nada, por padrão).
6. **Reprodutibilidade**: `env.json` com commit+máquina; rodadas a partir de um baseline commitado e limpo (git).

## 8. Plano de Implementação (ordem de dependência)

Tarefas executáveis em sequência; T8/T9 podem ser 8-paralelas, mas **T9 (mudança de algoritmo) precisa ser decidida antes de T4**.

- **T1 — Baseline e manifesto** (baixo esforço, destrava tudo). Commit do working copy de `benchmarks/benchmark.rs`, `AGENTS.md` movido e limpeza de temporários. Criar `scripts/README` + `MANIFEST_TEMPLATE.json`. Validação: `git status` limpo; `cargo test` 4/4.
- **T2 — Validação + inversões no harness** (P05, P07, P08). Em `servicebench.rs`/`benchmark.rs`: para cada array do pool, antes do timing, verificar sortedness (puro e com_pre) e preservação de permutação (`sort_unstable` como referência); gravar `inversoes_antes/depois` por array em `companion.csv`. Validação: rodada de 1 cenário com e sem bug induzido → falha detectada.
- **T3 — Configuração por perfil + lotes retomáveis** (P06, P16). Matriz: O(n·logn): `sample_size≈100` p/ ≤100k, `≈50` p/ 1M; O(n²): `≈50` p/ ≤10k, `≈30`+`measurement 300s` p/ 100k; 1M O(n²) conforme D1. Suporte a filtro por cenário (env `BENCH_SCOPE`) e detector de warnings no relatório. Validação: executar `selection_random@100000` → sem warnings, ≥30 amostras.
- **T4 — Rodada científica completa** (P01, P02, P04). Baseline limpo (T1), `cargo bench --bench benchmark` em lotes; 300 células + companions. Validação: §10 checa cada célula com amostra válida; todas as células reportadas têm artefato.
- **T5 — Estatística pareada e exportador robusto** (P03, P09, P10, P14). Análise pareada por pool (bootstrap da diferença, Wilcoxon); reescrita do exportador (validações, escrita atômica, `manifest.json`, `env.json`, tabelas/figuras 100 % preenchidas). Validação: reexportação produz arquivos não-vazios e manifest íntegro.
- **T6 — Testes de integridade dos artefatos** (P03). Teste (ou script CI) que falha se: artefato vazio, data < último change no código, células sem correspondência em `estimates.json`, figura com dados ausentes. Validação: `cargo test` inclui o teste; rodada "quebrada" do exportador deve reprovar.
- **T7 — Atualização do artigo** (P01, P02, P04, P07, P09). Regenerar Tabelas 3–5 e Figuras; reescrever Discussão do Quick (enquadrar pré × DNF, anomalia ordenada, significância); revisar resumo/abstract; declarar limitações (O(n²)@1M, Windows, variância por tipo, invariante empírico); remover/rotular extrapolações da Tabela 5. Validação: cada número no LaTeX rastreável a artefato (§10).
- **T8 — Limpeza técnica** (P11–P15, P17, P18): `usize` no quick, corrigir case do caminho do Criterion, re-export `contar_inversoes`, renomear enums PT (ou mapear no artigo), hah/sembrar seeds, pinar perfil release (LTO), propagar re-exports. Validação: `cargo build/test/clippy` ok; rodada de 1 cenário idêntica à da matriz.
- **T9 — [OPCIONAL] Micro-otimização do Quick DNF** (somente se decidir manter DNF e quiser melhorar o caso ordenado): evitar self-swap em `particao_tres_vias` (quando `lt==i`/`i==gt`) e reavaliar. Se aceita, FAZER ANTES de T4.
- **T10 — QA final e higiene do git** (T1–T9). Verificação cruzada celular (spot-check 20 células), revisão final do artigo × artefatos, remoção de temporários, atualização do `AGENTS.md` (issues resolvidas). Validação: checklist §10; `cargo test`; `git status` limpo.

## 9. Decisões Pendentes (usuário/orientador antes de T4)

- **D1 — O(n²) @ 1.000.000**: (a) medir com ~5–10 amostras e CI largo; (b) **restringir O(n²) a ≤ 100.000** (recomendado) e declarar no artigo; (c) usar 500k. Recomendação: (b), com (a) só p/ insertion/quase-ordenado se insistir em 1M.
- **D2 — Versionar resultados**: (a) fora do git, gerados por comando único (recomendado); (b) commitados ao lado do código com o teste T6 impedindo artefato velho.
- **D3 — Quick DNF**: **(a) manter e reescrever a discussão** (recomendado — honesto e interessante: "menos inversões não garante ganho"); (b) aplicar T9 e rediscutir.
- **D4 — Tabela 5**: remover linhas 100k/1M **ou** rotulá-las "estimativa linear (não medida)"; recomendado: **medir 100k via Criterion/CLI e excluir 1M do pré** (se D1-b).

## 10. Critérios de Aceitação

1. 100 % das células planejadas (após D2/D1) com ≥30 amostras válidas e **sem warnings** do Criterion; células excluídas constam explicitamente na Tabela/planilha de cobertura.
2. Todo cenário puro e com_pre validado (sorted + permutação) — zero falhas na rodada final.
3. Todos os CSVs, tabelas e figuras regenerados, **não-vazios**, com manifesto (`env.json`) e rastreáveis 1:1 aos `estimates.json`/`cli/*.txt`.
4. Nenhum número no artigo sem artefato correspondente (script de verificação T6 em CI).
5. Estatística pareada por pool com CI da diferença e p-valor para todos os ganhos de destaque; nenhuma afirmação de ganho sem CI excluir 0.
6. Reprodução: segunda rodada (mesmo commit, mesma máquina) com medianas dentro de 5 % nas células não-paramétricas (quick/merge).
7. `cargo build`, `cargo test`, `cargo clippy` sem erros; `git status` limpo ao final.

## 11. Checklists

**Pré-benchmark (T4)**: `git status` limpo e baseline commitado · `cargo test` ok · `cargo bench --no-run` compila · fechar apps pesados (browser, AV ativo ok) · registrar CPU/OS/clock (`env.json` pela T1/T5) · fixar tamanho/tipo/seed em config · rodar cada lote com `BENCH_SCOPE` e salvar log · conferir 0 warnings do Criterion.

**Pós-rodada**: conferir cobertura por lote (script) · reexecutar lotes com células ausentes · rodar `exportar_resultados` e conferir: nenhum CSV vazio, `tabelas.tex` com Tabelas 3–5 preenchidas, `manifest.json` presente · spot-check de 20 células (critério 5) · rodar T6.

**Pré-artigo (T7)**: cada tabela/figura regenerada dos artefatos · números de discussão cruzados com CSVs · declaração de limitações incluída · resumo/abstract sem claims refutados · referência ao `env.json` .

**Hygiene final (T10)**: remover exemplos/temporários de auditoria · atualizar `AGENTS.md` (Itens Críticos: validação adicionada, naming PT, `usize`, re-exports) · commit em partes lógicas · `git status` limpo.

---
Nota de auditoria: nenhum código-fonte foi alterado nesta fase (apenas leitura + reprodução empírica via exemplo temporário `examples/diag.rs`, removido; verificação direta com o crate).