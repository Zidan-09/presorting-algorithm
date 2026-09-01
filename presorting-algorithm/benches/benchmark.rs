use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion,
};
use std::cell::Cell;
use std::time::Duration;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use algoritmo::services::BenchmarkServiceBench;
use algoritmo::utils::{generate_test_array, ArrayType, SortType};

const SEED: u64 = 42;
const AMOSTRAS_POR_POOL: usize = 50;

fn e_quadratico(algoritmo: &SortType) -> bool {
    matches!(
        algoritmo,
        SortType::Bubble | SortType::Selection | SortType::Insertion
    )
}

fn executar_benchmarks_tcc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Experimentos_Ordenacao");

    group.confidence_level(0.95);
    group.significance_level(0.05);

    let tamanhos: [usize; 5] = [1000, 5000, 10000, 100000, 1000000];
    let tipos_array = [
        ArrayType::Random,
        ArrayType::Turtles,
        ArrayType::Zigzag,
        ArrayType::AlmostSorted,
        ArrayType::Duplicates,
        ArrayType::Inverted,
    ];
    let algoritmos = [
        SortType::Merge,
        SortType::Quick,
        SortType::Insertion,
        SortType::Bubble,
        SortType::Selection,
    ];

    for tamanho in &tamanhos {
        for algoritmo in &algoritmos {
            let (sample_size, measurement_time, warm_up_time, batch_size) =
                match (*tamanho, e_quadratico(algoritmo)) {
                    (1_000_000, true) => (
                        10, 
                        Duration::from_secs(4700),
                        Duration::from_secs(15),
                        BatchSize::PerIteration,
                    ),
                    (100_000, true) => (
                        30,   
                        Duration::from_secs(350), 
                        Duration::from_secs(5),
                        BatchSize::LargeInput,
                    ),
                    (t, false) if t >= 100_000 => (
                        100,                       
                        Duration::from_secs(40),
                        Duration::from_secs(3),
                        BatchSize::LargeInput,
                    ),
                    _ => (
                        100,
                        Duration::from_secs(50),
                        Duration::from_secs(3),
                        BatchSize::SmallInput,
                    ),
                };

            group.sample_size(sample_size);
            group.measurement_time(measurement_time);
            group.warm_up_time(warm_up_time);

            for tipo in tipos_array {
                let mut rng =
                    ChaCha8Rng::seed_from_u64(SEED ^ (*tamanho as u64) ^ (tipo as u64));

                let pool: Vec<Vec<i32>> = (0..AMOSTRAS_POR_POOL)
                    .map(|_| generate_test_array(*tamanho, tipo, &mut rng))
                    .collect();

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
                                let tempo = BenchmarkServiceBench::medir_puro(
                                    algoritmo.clone(),
                                    &mut vetor,
                                );
                                black_box(tempo);
                            },
                            batch_size,
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
                                let tempo = BenchmarkServiceBench::medir_com_pre(
                                    algoritmo.clone(),
                                    &mut vetor,
                                );
                                black_box(tempo);
                            },
                            batch_size,
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