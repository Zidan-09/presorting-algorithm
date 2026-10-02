# Alvo do PEvaluation / Bench 2026: deve ser executado a partir da RAIZ do repo.
# Uso:
#   powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/pipeline_f7.ps1 [-Stage smoke|full]
# Stages:
#   smoke (default): prova a cadeia ponta a ponta em minutos, sem o bench
#     longo - cargo test, 1 celula Criterion, efeito pareado minimo,
#     figuras a partir dos CSVs commitados e build do PDF.
#   full: reproduce TUDO (matriz F3 ~15-20h de maquina + F4 + export +
#     figuras + PDF). Rode em maquina dedicada, sem sleep.
param([string]$Stage = "smoke")

$ErrorActionPreference = "Stop"

function Banner($m) { Write-Host ""; Write-Host "===== $m =====" -ForegroundColor Cyan }
function Need($cmd) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        throw "dependencia ausente: $cmd"
    }
}

# --- F0: ambiente ---
Banner "F0 ambiente"
Need cargo; Need rustc; Need python3
git status --short
Get-Content presorting-algorithm/rust-toolchain.toml
rustc --version -v
cargo --version
if (-not (Test-Path presorting-algorithm/Cargo.lock)) { throw "sem Cargo.lock" }

if ($Stage -eq "smoke") {
    Banner "smoke: testes + 1 celula bench"
    Set-Location presorting-algorithm
    cargo test
    $env:BN_SEED = "42"; $env:BN_SORTS = "merge"; $env:BN_TIPOS = "random"
    $env:BN_TAMANHOS = "1000"
    cargo bench --bench benchmark -- --noplot
    foreach ($v in "BN_SEED", "BN_SORTS", "BN_TIPOS", "BN_TAMANHOS") {
        Remove-Item "Env:\$v" -ErrorAction SilentlyContinue
    }
    $env:FP_SEEDS = "42"; $env:FP_TAMS = "16"; $env:FP_R = "10"
    $env:FP_BOOT = "500"; $env:FP_OUT = "bench_logs/smoke_f7.csv"
    Remove-Item "bench_logs/smoke_f7.csv" -ErrorAction SilentlyContinue
    cargo run --release --example efeito_pareado | Select-Object -Last 2
    foreach ($v in "FP_SEEDS", "FP_TAMS", "FP_R", "FP_BOOT", "FP_OUT") {
        Remove-Item "Env:\$v" -ErrorAction SilentlyContinue
    }
    Set-Location ..
} elseif ($Stage -eq "full") {
    Banner "full: matriz F3 (longo) - ver docs/f3_estado.md"
    Set-Location presorting-algorithm
    # Multi-seed @10^6 (3 blocos/seed, ~40min/seed).
    foreach ($S in @("43", "44", "45", "46")) {
        $env:BN_SEED = $S
        $env:BN_SORTS = "merge,quick"
        $env:BN_TIPOS = "random,turtles,zigzag,almostsorted,duplicates"
        $env:BN_TAMANHOS = "1000000"
        cargo bench --bench benchmark -- --noplot
        $env:BN_TIPOS = "inverted,sawtooth,organpipe,fewruns"
        cargo bench --bench benchmark -- --noplot
        $env:BN_SORTS = "stdunstable,stdstable"; $env:BN_TIPOS = ""
        cargo bench --bench benchmark -- --noplot
        powershell -ExecutionPolicy Bypass -File scripts/arquivar_seed.ps1 -Seed $S
    }
    # Seed 42 restante (3a, 3b fatiado, 3c, 3d).
    $env:BN_SEED = "42"
    $env:BN_SORTS = "stdunstable,stdstable,descreverse"
    $env:BN_TIPOS = "random,turtles,zigzag,almostsorted,duplicates,inverted"
    $env:BN_TAMANHOS = "1000,5000,10000,100000,1000000"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = ""
    $env:BN_TIPOS = "sawtooth,organpipe,fewruns,real"
    foreach ($T in @("16,32,64,128,256,512,1000", "5000", "10000")) {
        $env:BN_TAMANHOS = $T
        cargo bench --bench benchmark -- --noplot
    }
    $env:BN_SORTS = "insertion,bubble,selection,descreverse"
    $env:BN_TAMANHOS = "100000"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = "merge,quick,stdunstable,stdstable"
    cargo bench --bench benchmark -- --noplot
    $env:BN_TAMANHOS = "1000000"
    cargo bench --bench benchmark -- --noplot
    $env:BN_TIPOS = "random,turtles,zigzag"
    $env:BN_TAMANHOS = "16,32,64,128,256,512"
    cargo bench --bench benchmark -- --noplot
    $env:BN_TIPOS = "almostsorted,duplicates,inverted"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = "merge,quick,stdunstable,stdstable"
    $env:BN_TIPOS = "turtles,zigzag,almostsorted,duplicates,inverted"
    $env:BN_TAMANHOS = "10000000"
    cargo bench --bench benchmark -- --noplot
    $env:BN_TIPOS = "sawtooth,fewruns"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = "merge,stdunstable,stdstable"; $env:BN_TIPOS = "organpipe"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = "quick"
    cargo bench --bench benchmark -- --noplot
    $env:BN_SORTS = "stdunstable,stdstable"; $env:BN_TIPOS = "random"
    cargo bench --bench benchmark -- --noplot
    foreach ($v in "BN_SEED", "BN_SORTS", "BN_TIPOS", "BN_TAMANHOS") {
        Remove-Item "Env:\$v" -ErrorAction SilentlyContinue
    }
    # F4 pareado (n=10^4 todas seeds; n=10^6 por seed).
    Remove-Item "../artigo/resultados/efeito_pareado.csv" -ErrorAction SilentlyContinue
    $env:FP_SEEDS = "42,43,44,45,46"; $env:FP_TAMS = "10000"
    $env:FP_R = "50"; $env:FP_BOOT = "10000"
    cargo run --release --example efeito_pareado
    foreach ($S in @("42", "43", "44", "45", "46")) {
        $env:FP_SEEDS = $S; $env:FP_TAMS = "1000000"
        cargo run --release --example efeito_pareado
    }
    foreach ($v in "FP_SEEDS", "FP_TAMS", "FP_R", "FP_BOOT") {
        Remove-Item "Env:\$v" -ErrorAction SilentlyContinue
    }
    python3 scripts/resumir_pareado.py
    # Export (aceite F3/F4: sem panic).
    cargo run --release --example exportar_resultados
    Set-Location ..
} else {
    throw "Stage invalido: $Stage (smoke|full)"
}

# --- F5: figuras EN + fragmentos (a partir dos CSVs) ---
Banner "F5 figuras"
Need pdflatex
python3 presorting-algorithm/scripts/gerar_figuras_f5.py | Select-Object -Last 3

# --- PDF ---
Banner "PDF"
Set-Location artigo/bench2026
pdflatex -interaction=nonstopmode main.tex
bibtex main
pdflatex -interaction=nonstopmode main.tex
pdflatex -interaction=nonstopmode main.tex
Select-String -Path main.log -Pattern "^!" | Select-Object -First 5
& "$env:LOCALAPPDATA\Programs\MiKTeX\miktex\bin\x64\pdfinfo.exe" main.pdf |
    Select-String -Pattern "Author|Title|Pages|File size"
Set-Location ../..
Banner "PIPELINE $Stage OK"
