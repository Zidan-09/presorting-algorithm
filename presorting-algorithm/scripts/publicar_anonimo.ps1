# Monta a arvore anonimizada p/ revisores a partir da WORKING TREE
# (nao exige commit; inclui arquivos novos ainda nao commitados).
# Rodar a partir da RAIZ do repo:
#   powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/publicar_anonimo.ps1 [-Destino ..\anonimo]
# Depois (lado usuario, logado em servico anonimo, ex. anonymous.4open.science):
#   1. criar repositorio vazio ANONIMO (sem nome/e-mail);
#   2. no diretorio de destino: git remote add origin <URL-anonima>; git push -u origin main
#   3. colar a URL no artigo ([14]) ou seguir a regra vigente do PEvaluation.
param([string]$Destino = "..\anonimo")

$ErrorActionPreference = "Stop"
$Raiz = (Get-Location).Path
if ($Raiz -ne (Get-Item $Raiz).FullName) { $Raiz = (Get-Item $Raiz).FullName }

if (Test-Path -LiteralPath $Destino) { throw "destino ja existe: $Destino" }
New-Item -ItemType Directory -Path $Destino | Out-Null
function CopyTree($origem, $destino) {
    if ((Get-Item -LiteralPath $origem -Force) -is [System.IO.DirectoryInfo]) {
        robocopy $origem $destino /E /NFL /NDL /NJH /NJS | Out-Null
        if ($LASTEXITCODE -ge 8) { throw "robocopy falhou: $origem" }
    } else {
        New-Item -ItemType Directory -Path (Split-Path $destino) -Force | Out-Null
        Copy-Item -LiteralPath $origem -Destination $destino -Force
    }
}

# --- codigo (lista explicita; sem target/, bench_logs/, docs internos) ---
CopyTree "$Raiz\presorting-algorithm\Cargo.toml" "$Destino\presorting-algorithm\Cargo.toml"
CopyTree "$Raiz\presorting-algorithm\Cargo.lock" "$Destino\presorting-algorithm\Cargo.lock"
CopyTree "$Raiz\presorting-algorithm\rust-toolchain.toml" "$Destino\presorting-algorithm\rust-toolchain.toml"
CopyTree "$Raiz\presorting-algorithm\src" "$Destino\presorting-algorithm\src"
CopyTree "$Raiz\presorting-algorithm\benches" "$Destino\presorting-algorithm\benches"
CopyTree "$Raiz\presorting-algorithm\examples" "$Destino\presorting-algorithm\examples"
CopyTree "$Raiz\presorting-algorithm\tests" "$Destino\presorting-algorithm\tests"
CopyTree "$Raiz\presorting-algorithm\dados_reais" "$Destino\presorting-algorithm\dados_reais"
CopyTree "$Raiz\presorting-algorithm\scripts" "$Destino\presorting-algorithm\scripts"
CopyTree "$Raiz\README_ANON.md" "$Destino\README.md"
CopyTree "$Raiz\.gitignore" "$Destino\.gitignore"

# --- artigo: bench2026 + resultados (sem .aux/.log/.out/synctex) ---
CopyTree "$Raiz\artigo\bench2026\main.tex" "$Destino\artigo\bench2026\main.tex"
CopyTree "$Raiz\artigo\bench2026\main.pdf" "$Destino\artigo\bench2026\main.pdf"
CopyTree "$Raiz\artigo\bench2026\reference.bib" "$Destino\artigo\bench2026\reference.bib"
CopyTree "$Raiz\artigo\bench2026\TBench-template.cls" "$Destino\artigo\bench2026\TBench-template.cls"
CopyTree "$Raiz\artigo\bench2026\TBench.bst" "$Destino\artigo\bench2026\TBench.bst"
CopyTree "$Raiz\artigo\bench2026\img" "$Destino\artigo\bench2026\img"
CopyTree "$Raiz\artigo\bench2026\resultados" "$Destino\artigo\bench2026\resultados"
New-Item -ItemType Directory -Path "$Destino\artigo\resultados" -Force | Out-Null
Get-ChildItem -Path "$Raiz\artigo\resultados\*.csv" -Force |
    ForEach-Object { CopyTree $_.FullName "$Destino\artigo\resultados\$($_.Name)" }
CopyTree "$Raiz\artigo\resultados\cli" "$Destino\artigo\resultados\cli"
Get-ChildItem -Path $Destino -Recurse -Force -Include @(
    "*.aux", "*.log", "*.out", "*.synctex.gz", "*.bbl", "*.blg", "*.tmp") |
    Remove-Item -Force -ErrorAction SilentlyContinue

# --- dados brutos: new/estimates.json (seeds 42-46) + companions ---
$Dados = Join-Path $Destino "dados_brutos"
New-Item -ItemType Directory -Path $Dados | Out-Null
$mapa = @{
    "seed_42" = "presorting-algorithm/target/criterion/experimentos_ordenacao"
    "seed_43" = "presorting-algorithm/target/criterion_archive/seed_43/experimentos_ordenacao"
    "seed_44" = "presorting-algorithm/target/criterion_archive/seed_44/experimentos_ordenacao"
    "seed_45" = "presorting-algorithm/target/criterion_archive/seed_45/experimentos_ordenacao"
    "seed_46" = "presorting-algorithm/target/criterion_archive/seed_46/experimentos_ordenacao"
}
foreach ($S in $mapa.Keys) {
    $origem = Join-Path $Raiz $mapa[$S]
    if (-not (Test-Path -LiteralPath $origem)) { throw "origem ausente: $origem" }
    $dest = Join-Path $Dados $S
    Get-ChildItem -Path $origem -Recurse -Force -Filter "estimates.json" |
        Where-Object { $_.FullName -match "\\new\\estimates\.json$" } |
        ForEach-Object {
            $rel = $_.FullName.Substring($origem.Length)
            $alvo = Join-Path $dest ($rel.TrimStart("\"))
            New-Item -ItemType Directory -Path (Split-Path $alvo) -Force | Out-Null
            Copy-Item -LiteralPath $_.FullName -Destination $alvo
        }
}
CopyTree "$Raiz\presorting-algorithm\target\criterion\experimentos_ordenacao\_companion" `
    "$Dados\companions"

# --- manifesto SHA-256 (dados) ---
$alag = Join-Path $Dados "SHA256SUMS.txt"
Get-ChildItem -Path @($Dados, (Join-Path $Destino "artigo/resultados")) `
    -Recurse -Force -File -Include "*.json", "*.csv" |
    ForEach-Object {
        $h = (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash
        $rel = $_.FullName.Substring($Destino.Length).TrimStart("\").Replace("\", "/")
        "$h  $rel"
    } | Sort-Object | Set-Content -LiteralPath $alag -Encoding ascii

# --- git fresco, identidade generica ---
Set-Location $Destino
git init -b main
git add -A
git -c user.name="Bench 2026 Authors" -c user.email="noreply@example.com" `
    commit -m "Anonymized artifact for double-blind review (Bench 2026)"
Set-Location $Raiz
"OK: arvore anonimizada em $Destino"
"PROXIMO (usuario): criar repo anonimo vazio, git remote add origin <URL>, git push -u origin main"
