use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use std::cell::Cell;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use algoritmo::services::BenchmarkServiceBench;
use algoritmo::utils::{generate_test_array, ArrayType, SortType};

const SEED: u64 = 42;
const AMOSTRAS_POR_POOL: usize = 50;

fn executar_benchmarks_tcc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Experimentos_Ordenacao");

    let tamanhos: [usize; 3] = [1000, 5000, 10000];
    let tipos_array = [ArrayType::Random, ArrayType::Turtles, ArrayType::Zigzag, ArrayType::AlmostSorted, ArrayType::Duplicates, ArrayType::Inverted];
    let algoritmos = [SortType::Merge, SortType::Quick, SortType::Insertion, SortType::Bubble, SortType::Selection];

    for tamanho in &tamanhos {
        for tipo in tipos_array {
            let mut rng = ChaCha8Rng::seed_from_u64(SEED ^ (*tamanho as u64) ^ (tipo as u64));

            let pool: Vec<Vec<i32>> = (0..AMOSTRAS_POR_POOL)
                .map(|_| generate_test_array(*tamanho, tipo, &mut rng))
                .collect();

            for algoritmo in algoritmos {
                let id_cenario = format!("{:?}_{:?}", algoritmo, tipo);
                let parametro_tamanho_puro = format!("Tamanho_{}_Puro", tamanho);
                let parametro_tamanho_pre = format!("Tamanho_{}_Com_Pre", tamanho);

                group.bench_with_input(
                    BenchmarkId::new(&id_cenario, &parametro_tamanho_puro),
                    &pool,
                    |b, pool| {
                        let contador = Cell::new(0usize);
                        b.iter_batched(
                            || {
                                let i = contador.get() % pool.len();
                                contador.set(contador.get() + 1);
                                pool[i].clone()
                            },
                            |mut vetor| {
                                let tempo = BenchmarkServiceBench::medir_puro(algoritmo.clone(), &mut vetor);
                                black_box(tempo);
                            },
                            BatchSize::SmallInput,
                        );
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new(&id_cenario, &parametro_tamanho_pre),
                    &pool,
                    |b, pool| {
                        let contador = Cell::new(0usize);
                        b.iter_batched(
                            || {
                                let i = contador.get() % pool.len();
                                contador.set(contador.get() + 1);
                                pool[i].clone()
                            },
                            |mut vetor| {
                                let tempo = BenchmarkServiceBench::medir_com_pre(algoritmo.clone(), &mut vetor);
                                black_box(tempo);
                            },
                            BatchSize::SmallInput,
                        );
                    },
                );
            }
        }
    }

    group.finish();
}

criterion_group!(benches, executar_benchmarks_tcc);
criterion_main!(benches);
