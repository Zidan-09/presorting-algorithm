# Arquiva o grupo Criterion de uma rodada multi-seed (F3, docs/plan2.md).
#
# Uso (na raiz do repo), APÓS cada rodada com BN_SEED=S e ANTES da próxima:
#   powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/arquivar_seed.ps1 -Seed 43
#
# Copia (não move, por segurança) target/criterion/experimentos_ordenacao
# para target/criterion_archive/seed_<S>/. A próxima rodada sobrescreve os
# mesmos IDs no diretório vivo — por isso o arquivo ANTES de cada nova seed.
# A seed 42 (matriz principal) roda por último e permanece no lugar para o
# script de exportação.
param([Parameter(Mandatory = $true)][string]$Seed)
$ErrorActionPreference = "Stop"
$origem = "presorting-algorithm/target/criterion/experimentos_ordenacao"
$destino = "presorting-algorithm/target/criterion_archive/seed_$Seed/experimentos_ordenacao"
if (-not (Test-Path -LiteralPath $origem)) {
    throw "Origem não encontrada: $origem (rode o bench com BN_SEED=$Seed antes)"
}
New-Item -ItemType Directory -Path $destino -Force | Out-Null
# -Path (não -LiteralPath): o curinga precisa expandir.
Copy-Item -Path "$origem/*" -Destination $destino -Recurse -Force
# NB: -Filter retorna 0 neste layout (quirk PS 5.1); usar -Include.
$SPH = Get-ChildItem -Path $destino -Recurse -Force -Include "estimates.json" |
    Measure-Object | Select-Object -ExpandProperty Count
if ($SPH -eq 0) { throw "falha no arquivo seed=$Seed (0 estimates.json copiados)" }
"arquivado seed=$Seed -> $destino ($SPH estimates.json)"
