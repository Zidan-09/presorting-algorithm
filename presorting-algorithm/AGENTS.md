# AGENTS.md

## Overview
Rust binary+library crate (`algoritmo`, edition 2024) that benchmarks sorting algorithms with and without a symmetric pre-processing pass. It is a TCC (undergrad thesis) experiment: all code, output, and names are in Brazilian Portuguese. There is no README, CI, test suite, or lint config.

## Commands
- Build: `cargo build` (compiles on stable; no nightly features needed)
- Run benchmark: `cargo run -- -s 10000 -a random -o insertion --seed 42 --repeticoes 7`
  - `-s/--size` (default 10000), `-a/--array`, `-o/--sort`
  - `--seed` (default 42; reproduzível), `--repeticoes` (default 7; medições intercaladas puro/com_pre com warm-up)
  - Array types: `random|inverted|zigzag|turtles|duplicates|almostSorted`
  - Sort types: `bubble|insertion|merge|quick|selection`
- Testes de validade: `cargo test` (verifica permutação ordenada em todos os sorts/tipos e que o pré-processamento não aumenta inversões).
- Criterion bench: `cargo bench --bench benchmark` (custom harness, `harness = false`). This runs 180 scenarios (5 sorts × 6 array types × 3 sizes [1000/5000/10000] × puro/com-pre) — expect it to take a long time. There is no way to filter by algorithm/array type from the CLI (all scenarios are hardcoded in `benches/benchmark.rs`). The harness uses seeded pools (`ChaCha8Rng`) and both branches clone from the SAME pool (pares casados).

## Gotchas
- `contar_inversoes` (`src/core/sort/contar_inversoes.rs`) is a brute-force O(n²) double loop. In `BenchmarkService::executar_teste` it runs only ONCE per experiment (na primeira repetição, fora dos loops cronometrados), so keep CLI sizes small (10k is ~10⁸ iterations; 100k+ will crawl). Criterion does NOT use it.
- Module layout is `src/core/sort/` (algorithms), `src/services/` (benchmark services), `src/utils/` (`tipos.rs`, `gerador.rs`), all wired in `src/lib.rs`. `main.rs` is the only file at `src/` root. Re-exports in each `mod.rs` (e.g. `crate::services::BenchmarkService`, `crate::core::sort::insertion_sort`) keep imports short — prefer them over long module paths.
- Cache misses (`cache_misses`) are only measured on Linux: `perf-event` is a linux-only dependency and the `MonitorCache` code is `cfg(target_os = "linux")`. On Windows it reports 0 — not a bug.
- Heap usage is measured via the `#[global_allocator]` `StatsAlloc` in `src/lib.rs` (`stats_alloc`). Don't replace or reorder the global allocator or `Region::new(&crate::GLOBAL)` calls in `service.rs` will break.
- `main.rs` (not `lib.rs`) is the binary entrypoint; `lib.rs` exposes the API consumed by the bench harness.

## Conventions
- Keep names, identifiers, comments, and CLI text in Portuguese (e.g., `pre_processamento_simetrico`, `executar_teste`, `contar_inversoes`).
- The research subject is `pre_processamento_simetrico` (`src/core/sort/pre_proc.rs`); experiments compare "pure" sort vs pre-processing + sort. Changes should preserve that two-branch structure.
