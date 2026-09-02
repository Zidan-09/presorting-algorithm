# AGENTS.md

## Project Structure
- **Cargo project**: `presorting-algorithm/` (Rust 1.96.0)
- **Core modules** (`src/core/sort/`):
  - `quick.rs` – three-way partitioning quicksort with median-of-three pivot
  - `bubble.rs`, `insertion.rs`, `merge.rs`, `selection.rs` – classic sorts
  - `pre_proc.rs` – symmetric pre-processing (swap operation) to reduce inversions
- **Services** (`src/services/`):
  - `service.rs` – orchestrates benchmark runs
  - `servicebench.rs` – Criterion harness for timing comparisons
- **Utilities** (`src/utils/`):
  - `types.rs` – `ArrayType` (Random, Inverted, Zigzag, Turtles, Duplicates, AlmostSorted) and `SortType` (Merge, Quick, Insertion, Bubble, Selection)
  - `gerador.rs` – test data generator
- **Main entrypoint**: `src/main.rs` – CLI with `clap` (sizes: 1K–1M, types: 6, sorts: 5)

## Key Implementation Details
- **Quick sort** uses Dutch National Flag (three-way partitioning) + median-of-three pivot + tail-call optimization (larger partition iterated)
- **Pre-processing** (`pre_proc.rs`) performs symmetric swaps to reduce inversions; validated in `service.rs`
- **Benchmarking** uses Criterion with seeded `ChaCha8Rng`; separates "pure" vs "with pre-processing"
- **Validation** – `service.rs` asserts sorted output and permutation preservation; `tests/validacao.rs` covers these

## Development Commands
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Benchmark**: `cargo bench --bench benchmark` (300 scenarios: 5 sorts × 6 types × 5 sizes × 2 branches)
- **Run all tests**: `cargo test`
- **Run test by name**: `cargo test pre_processamento_nao_aumenta_inversoes`

## Critical Issues (Do Not Ignore)
1. **Criterion benchmark lacks result validation** – it orders without checking correctness. Add inversion counting to `BenchmarkServiceBench`.
2. **Naming inconsistency** – enums use English (`AlmostSorted`, `Turtles`) but article uses Portuguese (`QuaseOrdenado`, `Tartarugas`). Align all enum variants to Portuguese.
3. **Index types** – `quick.rs` uses `i32` for array indices; should be `usize` for idiomatic Rust.
4. **Missing `pub use`** – `contar_inversoes` is not re-exported in `core/sort/mod.rs`.

## Architecture Notes
- Single crate (`presorting-algorithm`) with clear separation: core algorithms, benchmarking, utilities
- Tests live in `tests/validacao.rs` (4 tests covering sorting correctness, pre-processing, permutations)
- Article in `artigo/` (LaTeX) – reference for expected results
- No generated code, migrations, or CI/CD pipelines beyond standard Cargo

## Testing Quirks
- All tests pass; benchmark validation is currently absent
- Pre-processing invariants tested in `service.rs` but not in Criterion harness
- Sort correctness verified by comparing with `sort_unstable()`

## Constraints
- Criterion harness must report both "pure" and "with pre-processing" timings
- Quick sort must handle `AlmostSorted` and `Duplicates` efficiently (three-way partition helps)
