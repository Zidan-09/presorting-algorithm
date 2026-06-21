use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use algoritmo::servicebench::BenchmarkServiceBench;
use algoritmo::gerador::generate_test_array;
use algoritmo::tipos::{ArrayType, SortType};

fn executar_benchmarks_tcc(c: &mut Criterion) {
    let mut group = c.benchmark_group("Experimentos_Ordenacao");

    let tamanhos: [usize; 3] = [1000, 5000, 10000];
    let tipos_array = [ArrayType::Random, ArrayType::Turtles, ArrayType::Zigzag, ArrayType::AlmostSorted, ArrayType::Duplicates, ArrayType::Inverted];
    let algoritmos = [SortType::Merge, SortType::Quick, SortType::Insertion, SortType::Bubble, SortType::Selection];

    for algoritmo in algoritmos {
        for tipo in tipos_array {
            for tamanho in &tamanhos {
                let id_cenario = format!("{:?}_{:?}", algoritmo, tipo);
                let parametro_tamanho_puro = format!("Tamanho_{}_Puro", tamanho);
                let parametro_tamanho_pre = format!("Tamanho_{}_Com_Pre", tamanho);

                group.bench_with_input(
                    BenchmarkId::new(&id_cenario, &parametro_tamanho_puro),
                    tamanho,
                    |b, &size| {
                        b.iter_batched(
                            || generate_test_array(size, tipo.clone()),
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
                    tamanho,
                    |b, &size| {
                        b.iter_batched(
                            || generate_test_array(size, tipo.clone()),
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