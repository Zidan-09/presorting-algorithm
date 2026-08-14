use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use algoritmo::utils::{generate_test_array, ArrayType, SortType};
use algoritmo::core::sort::pre_proc::pre_processamento_simetrico;
use algoritmo::core::sort::contar_inversoes::contar_inversoes;
use algoritmo::core::sort::{
    bubble::bubble_sort, insertion::insertion_sort, selection::selection_sort,
    quick::quick_sort, merge::merge_sort,
};

const TIPOS: [ArrayType; 6] = [
    ArrayType::Random,
    ArrayType::Turtles,
    ArrayType::Zigzag,
    ArrayType::AlmostSorted,
    ArrayType::Duplicates,
    ArrayType::Inverted,
];

const SORTS: [SortType; 5] = [
    SortType::Merge,
    SortType::Quick,
    SortType::Insertion,
    SortType::Bubble,
    SortType::Selection,
];

fn ordenar(sort_type: SortType, array: &mut [i32]) {
    match sort_type {
        SortType::Insertion => insertion_sort(array),
        SortType::Bubble => bubble_sort(array),
        SortType::Selection => selection_sort(array),
        SortType::Quick => quick_sort(array),
        SortType::Merge => merge_sort(array),
    }
}

#[test]
fn ordenacoes_geram_permutacao_ordenada() {
    for sort in SORTS {
        for tipo in TIPOS {
            for &tamanho in &[10usize, 100, 1000, 5000] {
                let mut rng = ChaCha8Rng::seed_from_u64(12345 + tamanho as u64 * 31);
                for _ in 0..3 {
                    let original = generate_test_array(tamanho, tipo, &mut rng);
                    let mut vetor = original.clone();
                    ordenar(sort, &mut vetor);

                    assert!(
                        vetor.windows(2).all(|w| w[0] <= w[1]),
                        "não ordenado: sort={:?} tipo={:?} tamanho={}",
                        sort,
                        tipo,
                        tamanho
                    );

                    let mut original_ord = original.clone();
                    let mut vetor_ord = vetor.clone();
                    original_ord.sort_unstable();
                    vetor_ord.sort_unstable();
                    assert_eq!(
                        original_ord, vetor_ord,
                        "multiset alterado (não é permutação): sort={:?} tipo={:?} tamanho={}",
                        sort,
                        tipo,
                        tamanho
                    );
                }
            }
        }
    }
}

#[test]
fn pre_processamento_nao_aumenta_inversoes() {
    for tipo in TIPOS {
        for &tamanho in &[10usize, 50, 200, 1000] {
            let mut rng = ChaCha8Rng::seed_from_u64(999 + tamanho as u64 * 17);
            for _ in 0..3 {
                let mut vetor = generate_test_array(tamanho, tipo, &mut rng);
                let antes = contar_inversoes(&vetor);
                pre_processamento_simetrico(&mut vetor);
                let depois = contar_inversoes(&vetor);
                assert!(
                    depois <= antes,
                    "inversões aumentaram: tipo={:?} tamanho={} antes={} depois={}",
                    tipo,
                    tamanho,
                    antes,
                    depois
                );
            }
        }
    }
}

#[test]
fn random_gera_valores_distintos() {
    let mut rng = ChaCha8Rng::seed_from_u64(7);
    let vetor = generate_test_array(5000, ArrayType::Random, &mut rng);
    let mut ord = vetor.clone();
    ord.sort_unstable();
    ord.dedup();
    assert_eq!(ord.len(), vetor.len(), "Random deve conter valores distintos");
}

#[test]
fn quase_ordenado_esta_realmente_quase_ordenado() {
    let mut rng = ChaCha8Rng::seed_from_u64(7);
    let tamanho = 10_000usize;
    let vetor = generate_test_array(tamanho, ArrayType::AlmostSorted, &mut rng);
    let inversoes = contar_inversoes(&vetor);
    let total_possiveis = (tamanho * (tamanho - 1) / 2) as u64;
    assert!(
        inversoes < total_possiveis / 10,
        "AlmostSorted com muitas inversões: {} ({}%)",
        inversoes,
        inversoes as f64 / total_possiveis as f64 * 100.0
    );
}
