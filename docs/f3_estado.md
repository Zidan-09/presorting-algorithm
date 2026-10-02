# Estado da F3 — ledger de rodadas (fonte de verdade)

Regra de proveniência: cada célula-ID (`Sort_Tipo/Tamanho_*`) vale pela
**última seed cuja rodada a cobriu**. Arquivos em
`target/criterion_archive/seed_*/` são snapshots do diretório vivo no
momento do arquivo — podem conter caronas de outras seeds (@16/@1000 da
F1 são seed 42 em todos os snapshots). Para F4, ler sempre
`*/new/estimates.json` e conferir a seed neste ledger.

## Log de rodadas (ordem cronológica)

| # | seed | filtros (SORTS/TIPOS/TAM) | duração | arquivo | notas |
|---|---|---|---|---|---|
| F1a | 42 | novos sorts × novos tipos @16,1000 | — | — | smoke F1; `merge_real_16` **sobrescrito depois** (ver种子43-teste) |
| F1b | 42 | insertion,merge × sawtooth,real @10k,100k | — | — | smoke F1 (seed 42, keepers) |
| F3p | 42 | merge,quick × random @10⁷ | 6 min | — (vivo) | piloto; keepers: `*_Random/Tamanho_10000000_*`, `Pre_Random` |
| F3t | 43 | merge × real @16 (teste BN_SEED) | ~1 min | — | validou gating (companion intacto); **sobrescreveu `merge_real_16` (era 42)** |
| F3a | 43 | tudo @10⁴ | 23 min | `seed_43` (969 est.) | |
| F3b | 44 | tudo @10⁴ | 23 min | `seed_44` (1069 est.) | |
| F3c | 45 | tudo @10⁴ | 23 min | `seed_45` (1069 est.) | |
| F3d | 46 | tudo @10⁴ | 23 min | `seed_46` (1069 est.) | |
| F3e | 43 | merge,quick × 5 tipos + 4 tipos @10⁶ | 7+20 min | — | **NÃO arquivado; sobrescrito por F3f. DADO PERDIDO (ver abaixo)** |
| F3f | 43 | stdunstable,stdstable × 9 tipos @10⁶ | 12 min | — | idem |
| F3g | 44 | merge,quick × 5 tipos + 4 tipos @10⁶ | 7+20 min | `seed_44` re-arquivado (1264 est.: @10⁴-44 + @10⁶-44) | organpipe confirma patologia (~4,3 s/sort) |
| F3h | 44 | stdunstable,stdstable × 9 tipos @10⁶ | 12 min | incluído no re-arquivo acima | |
| F3i | 45 | merge,quick × 5 tipos + 4 tipos @10⁶; std × 9 tipos @10⁶ | ~40 min | `seed_45` re-arquivado (504 new-only: @10⁴-45 + @10⁶-45) | 02/10/2026 |
| F3j | 46 | idem blocos com BN_SEED=46 | ~40 min | `seed_46` re-arquivado (504 new-only) | 02/10/2026 |
| F3k | 43 | re-run F3e+F3f perdida: mesmos 3 blocos @10⁶ | ~40 min | `seed_43` re-arquivado como superset (504 new-only) — perda RESOLVIDA, 5 pools @10⁶ fechados (42–46) | 02/10/2026 |
| F3l | 42 | restante seed 42 (3a: std/desc×6 tipos×1k–1M; 3b: tudo×4 novos tipos×16–1M em 6 sub-blocos; 3c: tudo×6 tipos antigos×16–512 em 2 blocos; 3d: merge/quick/std×7–8 tipos @10⁷, organpipe-quick ~7,5 s/sort, sem exclusão) | ~6 h | dir vivo = matriz seed 42 completa | 02/10/2026 |
| F3m | 42 | export | — | `exportar_resultados` OK sem panic: 872 células, `benchmark_consolidado.csv` (1745 lin), `ganho.csv` (873), `cpre_consolidado.csv` (119), `inversoes.csv` (59); saída em `artigo/resultados/` | 02/10/2026 |

## Mapa atual do diretório vivo (F3 CONCLUÍDA em 02/10/2026)

| células | seed vigente |
|---|---|
| matriz completa (8 sorts × 10 tipos × 12 tamanhos + Pre, c/ gating Real/celula_planejada) | 42 |
| companions `_companion/*` | 42 (118 arquivos = 10×12 − real@1M/10M) |
| archives `seed_43/44/45/46` | 504 new-only cada (@10⁴ + @10⁶; 5 pools p/ F4 c/ seed 42) |

Notas de fechamento:
- Export escreve/lê em `artigo/resultados/` (raiz do repo; RAIZ_SAIDA/RAIZ_CLI =
  `../artigo/resultados[/cli]` a partir do crate). CLI: base copiada de
  `artigo/resultados/cli/` (66 arqs) + 23 gerados p/ novos tipos
  (`sawtooth/organpipe/fewruns × 1k–1M`, `real × 1k–100k`; seed 42, sort merge —
  só `CSV_PRE_SOZINHO` é lido). Cuidado: redirect `>` do PS 5.1 gera UTF-16 e
  quebra `read_to_string` do export — converter p/ UTF-8 sem BOM.
- Patch F3 em `examples/exportar_resultados.rs` (seção 3): pular `real` acima do
  nativo em TAMANHOS_CLI (gera assert no CLI; sem tiling).
- `quick_organpipe @10⁷` medido (~7,5 s/sort; ganho +0,65%): nenhuma exclusão
  necessária (orçamento de 30 min/célula respeitado).
- `cargo test`: 4/4 OK. Segunda máquina/Linux (F3.3): pendente, lado usuário.

## Perda registrada — RESOLVIDA em 02/10/2026 (F3k: seed 43 @10⁶ re-rodada,
arquivada como superset; pools @10⁶ = {42,43,44,45,46}, @10⁴ = {42,43,44,45,46})

## Fila de resume — CONCLUÍDA (02/10/2026). Próximo: F4 (bootstrap pareado;
pools em `target/criterion_archive/seed_*/` + matriz seed 42 no dir vivo).

Comandos executados (histórico, a partir da raiz do repo):

```powershell
# 1. seeds 45, 46 @10⁶ (3 blocos cada, ~40 min/seed); arquivar ao fim de cada seed:
#  (repetir p/ S in 45,46)
$env:BN_SEED="<S>"
$env:BN_SORTS="merge,quick"; $env:BN_TIPOS="random,turtles,zigzag,almostsorted,duplicates"; $env:BN_TAMANHOS="1000000"
cargo bench --bench benchmark -- --noplot   # (c/ workdir presorting-algorithm)
$env:BN_TIPOS="inverted,sawtooth,organpipe,fewruns"   # repete
$env:BN_SORTS="stdunstable,stdstable"; $env:BN_TIPOS=""  # repete (todos os 9 tipos)
powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/arquivar_seed.ps1 -Seed <S>

# 2. seed 43 @10⁶ (opcional, ver Perda): mesmos 3 blocos com BN_SEED=43 (NÃO arquivar
#   por cima de seed_43 sem renomear antes — o snapshot atual é @10⁴; se re-rodar,
#   arquivar em seed_43b ou aceitar o superset).

# 3. seed 42 restante (restaura o dir vivo p/ export; tudo seed 42):
$env:BN_SEED="42"
# 3a. novos sorts × tipos/tamanhos antigos:
$env:BN_SORTS="stdunstable,stdstable,descreverse"
$env:BN_TIPOS="random,turtles,zigzag,almostsorted,duplicates,inverted"
$env:BN_TAMANHOS="1000,5000,10000,100000,1000000"; cargo bench ...
# 3b. todos sorts × novos tipos × todos tamanhos:
$env:BN_SORTS=""; $env:BN_TIPOS="sawtooth,organpipe,fewruns,real"; $env:BN_TAMANHOS=""
# 3c. tamanhos pequenos, tudo:
$env:BN_TIPOS=""; $env:BN_TAMANHOS="16,32,64,128,256,512"
# 3d. 10⁷ restante: BN_SORTS="merge,quick,stdunstable,stdstable"
#     BN_TIPOS="turtles,zigzag,almostsorted,duplicates,inverted,sawtooth,organpipe,fewruns"
#     BN_TAMANHOS="10000000"  (~2h; pular organpipe-quick se >30 min/célula)

# 4. export (aceite F3): só após 3.
cargo run --release --example exportar_resultados
# esperado: sem panic; benchmark_consolidado.csv, inversoes.csv,
#           cpre_consolidado.csv, ganho.csv regenerados.

# 5. segunda máquina/Linux: lado usuário (roteiro em docs/plan2.md F3.3).
```

## Notas
- `merge_real_16` no dir vivo é seed 43; o passo 3b o restaura p/ 42.
- OrganPipe @10⁶ no quick da casa (4,3 s/sort): manter no protocolo, mas
  pular a célula se estourar o orçamento na passada 3d.
- Snapshots crescem ~100 arquivos/passada (histórico do Criterion); só
  `*/new/estimates.json` é leitura válida.
