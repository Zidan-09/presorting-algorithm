use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::cell::Cell;
use std::collections::HashSet;
use std::fs;
use std::time::Duration;

use algoritmo::core::sort::contar_inversoes::contar_inversoes;
use algoritmo::core::sort::pre_proc::pre_processamento_simetrico;
use algoritmo::services::BenchmarkServiceBench;
use algoritmo::utils::{generate_test_array, ArrayType, SortType};

const SEED: u64 = 42;
const AMOSTRAS_POR_POOL: usize = 50;
const MAX_VALIDACAO_CARA: usize = 5;

const RAIZ_COMPANION: &str = "target/criterion/experimentos_ordenacao/_companion";
const RAIZ_VALIDACAO: &str = "target/criterion/experimentos_ordenacao/_validacao";

fn nome_sort(s: SortType) -> &'static str {
    match s {
        SortType::Merge => "merge",
        SortType::Quick => "quick",
        SortType::Insertion => "insertion",
        SortType::Bubble => "bubble",
        SortType::Selection => "selection",
    }
}

fn nome_tipo(t: ArrayType) -> &'static str {
    match t {
        ArrayType::Random => "random",
        ArrayType::Turtles => "turtles",
        ArrayType::Zigzag => "zigzag",
        ArrayType::AlmostSorted => "almostsorted",
        ArrayType::Duplicates => "duplicates",
        ArrayType::Inverted => "inverted",
    }
}

fn e_quadratico(s: SortType) -> bool {
    matches!(s, SortType::Insertion | SortType::Bubble | SortType::Selection)
}

/// Matriz de medição: algoritmos O(n²) não são medidos em n = 1.000.000
/// (a unidade de execução é de minutos — impraticável para 36 células
/// independentes); a classe O(n²) é coberta até n = 100.000.
fn celula_planejada(s: SortType, tamanho: usize) -> bool {
    !(e_quadratico(s) && tamanho >= 1_000_000)
}

/// (amostras, warm_up, tempo de medição por amostra)
fn config_mediacao(s: SortType, tamanho: usize) -> (usize, Duration, Duration) {
    if e_quadratico(s) && tamanho >= 100_000 {
        (30, Duration::from_secs(1), Duration::from_secs(5))
    } else if tamanho >= 1_000_000 {
        (50, Duration::from_secs(2), Duration::from_secs(10))
    } else {
        (50, Duration::from_secs(2), Duration::from_secs(5))
    }
}

/// Filtros por variável de ambiente (execução em lotes / retomável).
///   BN_SORTS   = "merge,quick" (nomes minúsculos)
///   BN_TIPOS   = "random,inverted"
///   BN_TAMANHOS= "1000,100000"
fn filtros_env() -> (HashSet<String>, HashSet<String>, HashSet<usize>) {
    let ler = |chave: &str| -> HashSet<String> {
        std::env::var(chave)
            .ok()
            .map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_ascii_lowercase())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    };
    let sorts = ler("BN_SORTS");
    let tipos = ler("BN_TIPOS");
    let tamanhos = std::env::var("BN_TAMANHOS")
        .ok()
        .map(|v| {
            v.split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect()
        })
        .unwrap_or_default();
    (sorts, tipos, tamanhos)
}

fn validar_ordenacao(sort: SortType, array: &mut Vec<i32>) -> bool {
    BenchmarkServiceBench::aplicar_ordenacao(sort, array);
    if !array.windows(2).all(|w| w[0] <= w[1]) {
        return false;
    }
    let mut referencia = array.clone();
    referencia.sort_unstable();
    *array == referencia
}

/// Escreve (uma vez) o companion de inversões por (tipo, tamanho), fora de
/// qualquer região cronometrada, usando exatamente os arrays do pool.
fn gerar_companion_inversoes(tipo: ArrayType, tamanho: usize, pool: &[Vec<i32>]) {
    fs::create_dir_all(RAIZ_COMPANION).unwrap();
    let path = format!("{RAIZ_COMPANION}/inversoes_{}_{}.csv", nome_tipo(tipo), tamanho);
    if fs::metadata(&path).is_ok() {
        return;
    }
    let mut out = String::from("indice,inversoes_iniciais,inversoes_pos_pre\n");
    for (i, vet) in pool.iter().enumerate() {
        let ini = contar_inversoes(vet);
        let mut sonda = vet.clone();
        pre_processamento_simetrico(&mut sonda);
        let pos = contar_inversoes(&sonda);
        out.push_str(&format!("{i},{ini},{pos}\n"));
    }
    fs::write(&path, out).unwrap();
}

/// Valida corretude (ordenação + preservação de permutação) fora da região
/// cronometrada e grava um marcador de sucesso por (algoritmo, tipo, tamanho).
fn validar_cenario(sort: SortType, tipo: ArrayType, tamanho: usize, pool: &[Vec<i32>]) {
    fs::create_dir_all(RAIZ_VALIDACAO).unwrap();
    let n_validar = if e_quadratico(sort) && tamanho >= 100_000 {
        MAX_VALIDACAO_CARA
    } else {
        pool.len()
    };
    let mut validados = 0usize;
    for vet in pool.iter().take(n_validar) {
        let mut a = vet.clone();
        let mut b = vet.clone();
        pre_processamento_simetrico(&mut b);
        if !validar_ordenacao(sort, &mut a) || !validar_ordenacao(sort, &mut b) {
            panic!(
                "VALIDAÇÃO FALHOU: sort={:?} tipo={:?} n={} saída não ordenada ou permutação alterada",
                sort, tipo, tamanho
            );
        }
        validados += 1;
    }
    let marker = format!(
        "{RAIZ_VALIDACAO}/{}_{}_{}.ok",
        nome_sort(sort),
        nome_tipo(tipo),
        tamanho
    );
    fs::write(&marker, format!("{validados}")).unwrap();
}

fn executar_benchmarks_tcc(c: &mut Criterion) {
    let (filtro_sorts, filtro_tipos, filtro_tamanhos) = filtros_env();

    let mut group = c.benchmark_group("Experimentos_Ordenacao");
    group.confidence_level(0.95);
    group.significance_level(0.05);

    let tamanhos: [usize; 5] = [1_000, 5_000, 10_000, 100_000, 1_000_000];

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

    for tamanho in tamanhos {
        if !filtro_tamanhos.is_empty() && !filtro_tamanhos.contains(&tamanho) {
            continue;
        }
        for tipo in tipos_array {
if !filtro_tipos.is_empty() && !filtro_tipos.contains(nome_tipo(tipo)) {
                    continue;
                }

            // Pool único por (tipo, tamanho): mesmas 50 entradas para todos os
            // algoritmas e para as duas variantes (puro e com pré-processamento).
            let mut rng =
                ChaCha8Rng::seed_from_u64(SEED ^ (tamanho as u64) ^ (tipo as u64));
            let pool: Vec<Vec<i32>> = (0..AMOSTRAS_POR_POOL)
                .map(|_| generate_test_array(tamanho, tipo, &mut rng))
                .collect();

            gerar_companion_inversoes(tipo, tamanho, &pool);

            for algoritmo in &algoritmos {
                if !celula_planejada(*algoritmo, tamanho) {
                    continue;
                }
                if !filtro_sorts.is_empty() && !filtro_sorts.contains(nome_sort(*algoritmo)) {
                    continue;
                }

                validar_cenario(*algoritmo, tipo, tamanho, &pool);

                let (amostras, warm_up, medicao) = config_mediacao(*algoritmo, tamanho);
                eprintln!(
                    "[BENCH] {} {} n={} amostras={} warm_up={:?} medicao={:?}",
                    nome_sort(*algoritmo),
                    nome_tipo(tipo),
                    tamanho,
                    amostras,
                    warm_up,
                    medicao
                );

                let id_cenario = format!("{:?}_{:?}", algoritmo, tipo);
                let parametro_tamanho_puro = format!("Tamanho_{}_Puro", tamanho);
                let parametro_tamanho_pre = format!("Tamanho_{}_Com_Pre", tamanho);
                let batch_size = if tamanho >= 100_000 {
                    BatchSize::LargeInput
                } else {
                    BatchSize::SmallInput
                };

                // Configuração por perfil: aplicada a todos os benchmarks do
                // grupo criados a partir daqui (Criterion 0.5).
                group.sample_size(amostras);
                group.warm_up_time(warm_up);
                group.measurement_time(medicao);

                // PURO
                group
                    .bench_with_input(
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
                                        *algoritmo,
                                        &mut vetor,
                                    );
                                    black_box(tempo);
                                },
                                batch_size,
                            );
                        },
                    );

                // COM PRÉ-PROCESSAMENTO
                group
                    .bench_with_input(
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
                                        *algoritmo,
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