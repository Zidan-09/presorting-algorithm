#!/usr/bin/env bash
# Espelho Linux de pipeline_f7.ps1. Rodar a partir da RAIZ do repo:
#   bash presorting-algorithm/scripts/pipeline_f7.sh [smoke|full]
# Requer: cargo (toolchain de rust-toolchain.toml), python3, pdflatex, bibtex.
set -euo pipefail
STAGE="${1:-smoke}"
cd "$(dirname "$0")/../.."

echo "===== F0 ambiente ====="
for c in cargo rustc python3 pdflatex bibtex; do command -v "$c" >/dev/null || { echo "dependência ausente: $c"; exit 1; }; done
git status --short
cat presorting-algorithm/rust-toolchain.toml
rustc --version -v
cargo --version
test -f presorting-algorithm/Cargo.lock

if [ "$STAGE" = smoke ]; then
  echo "===== smoke ====="
  ( cd presorting-algorithm && cargo test )
  ( cd presorting-algorithm && BN_SEED=42 BN_SORTS=merge BN_TIPOS=random BN_TAMANHOS=1000 \
      cargo bench --bench benchmark -- --noplot )
  ( cd presorting-algorithm && rm -f bench_logs/smoke_f7.csv && \
      FP_SEEDS=42 FP_TAMS=16 FP_R=10 FP_BOOT=500 FP_OUT=bench_logs/smoke_f7.csv \
      cargo run --release --example efeito_pareado | tail -2 )
elif [ "$STAGE" = full ]; then
  echo "===== full: matriz F3 (longo) ====="
  ( cd presorting-algorithm && for S in 43 44 45 46; do
      export BN_SEED=$S
      BN_SORTS=merge,quick BN_TIPOS=random,turtles,zigzag,almostsorted,duplicates BN_TAMANHOS=1000000 \
        cargo bench --bench benchmark -- --noplot
      BN_TIPOS=inverted,sawtooth,organpipe,fewruns \
        cargo bench --bench benchmark -- --noplot
      BN_SORTS=stdunstable,stdstable BN_TIPOS= \
        cargo bench --bench benchmark -- --noplot
      mkdir -p "target/criterion_archive/seed_$S/experimentos_ordenacao"
      cp -r target/criterion/experimentos_ordenacao/. \
        "target/criterion_archive/seed_$S/experimentos_ordenacao/"
    done )
  echo "seed 42 restante + F4 + export: ver pipeline_f7.ps1 (mesma sequência)"
  echo "NÃO automatizado aqui: rode os blocos BN_* do .ps1 adaptados ao shell."
else
  echo "Stage inválido: $STAGE (smoke|full)"; exit 1
fi

echo "===== F5 figuras ====="
python3 presorting-algorithm/scripts/gerar_figuras_f5.py | tail -3

echo "===== PDF ====="
( cd artigo/bench2026 && pdflatex -interaction=nonstopmode main.tex && \
    bibtex main && pdflatex -interaction=nonstopmode main.tex && \
    pdflatex -interaction=nonstopmode main.tex )
grep -m5 "^!" artigo/bench2026/main.log || true
pdfinfo artigo/bench2026/main.pdf | grep -E "Author|Title|Pages|File size"
echo "===== PIPELINE $STAGE OK ====="
