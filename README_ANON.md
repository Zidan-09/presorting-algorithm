# Anonymized artifact — symmetric preprocessing for sorting (Bench 2026)

Double-blind review package. No author information is contained in this
repository, the PDF, or its metadata (verified via `pdfinfo`: empty
Author/Title; Creator/Producer carry only the toolchain).

## Layout

- `presorting-algorithm/` — Rust 1.96.0 crate (`rust-toolchain.toml`,
  `Cargo.lock` committed): `src/` (algorithms, harness), `benches/`,
  `examples/` (export, paired-effect), `tests/`, `scripts/`,
  `dados_reais/` (frozen real-byte asset + generator + SHA-256).
- `artigo/bench2026/` — submitted paper (`main.tex` + `main.pdf`,
  TBench template) and its figure/table inputs (`resultados/`).
- `artigo/resultados/` — consolidated machine-readable results
  (`benchmark_consolidado.csv`, `ganho.csv`, `inversoes.csv`,
  `cpre_consolidado.csv`, `efeito_pareado.csv`,
  `efeito_pareado_resumo.csv`, CLI series in `cli/`).
- `dados_brutos/` — raw Criterion data: per-seed `new/estimates.json`
  (seeds 42–46; seed 42 is the paper matrix), `_companion` inversion
  files, and `SHA256SUMS.txt` covering every data file.

## Reproduce (Windows PowerShell; Linux: `pipeline_f7.sh`)

```powershell
powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/pipeline_f7.ps1 -Stage smoke  # minutes
powershell -ExecutionPolicy Bypass -File presorting-algorithm/scripts/pipeline_f7.ps1 -Stage full   # ~15-20 h machine time
```

`smoke` proves the full chain (tests, one Criterion cell, minimal paired
experiment, figures, PDF). `full` re-runs the entire measurement matrix,
the paired analysis, the export, and the paper build.
