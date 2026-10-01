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

## Mapa atual do diretório vivo (pós-F3h)

| células | seed vigente |
|---|---|
| @10⁴ tudo (8 sorts × 10 tipos + Pre) | 46 |
| @10⁶ merge,quick,stdunstable,stdstable × 9 tipos + Pre | 44 |
| @16,@1000 novos sorts/tipos (F1) | 42, **exceto `merge_real_16` (= 43)** |
| @10⁷ merge,quick × random + Pre (piloto) | 42 (keepers) |
| matriz antiga (5×6×1k–1M, pré-F1) | 42 (intocada) |
| companions `_companion/*` | 42 (gating BN_SEED; F1 + matriz antiga) |

## Perda registrada
Seed-43 @10⁶ (F3e+F3f, ~40 min de máquina) não foi arquivada antes da
seed-44 sobrescrever os mesmos IDs. Opções: (a) re-rodar 43 @10⁶ no fim
(~40 min) para fechar 5 pools; (b) seguir com pools {42,44,45,46} @10⁶
(4 pools — suficiente p/ variância entre pools na F4). Decidir na F4.

## Fila de resume (comandos a partir da raiz do repo)

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
