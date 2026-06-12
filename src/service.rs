use std::time::{Duration, Instant};
use crate::tipos::{ArrayType, SortType};
use crate::gerador::generate_test_array;

use crate::algoritmos::{
    insertion::insertion_sort,
    bubble::bubble_sort,
    selection::selection_sort,
    quick::quick_sort,
    merge::merge_sort,
};
use crate::algoritmos::pre_proc::pre_processamento_simetrico;

pub struct BenchmarkResult {
    pub tempo_puro: Duration,
    pub tempo_pre_processamento: Duration,
    pub tempo_ordenacao_com_pre: Duration,
    pub tempo_total_com_pre: Duration,
    pub ordenacao_pura_valida: bool,
    pub ordenacao_pre_valida: bool,
}

pub struct BenchmarkService;

impl BenchmarkService {
    pub fn executar_teste(size: usize, array_type: ArrayType, sort_type: SortType) -> BenchmarkResult {
        let vetor_original = generate_test_array(size, array_type);

        let mut vetor_puro = vetor_original.clone();
        let mut vetor_com_pre = vetor_original; 

        let inicio_puro = Instant::now();
        Self::aplicar_ordenacao(sort_type, &mut vetor_puro);
        let tempo_puro = inicio_puro.elapsed();

        let inicio_pre = Instant::now();
        pre_processamento_simetrico(&mut vetor_com_pre);
        let tempo_pre_processamento = inicio_pre.elapsed();

        let inicio_sort_com_pre = Instant::now();
        Self::aplicar_ordenacao(sort_type, &mut vetor_com_pre);
        let tempo_ordenacao_com_pre = inicio_sort_com_pre.elapsed();

        let tempo_total_com_pre = tempo_pre_processamento + tempo_ordenacao_com_pre;

        let ordenacao_pura_valida = vetor_puro.windows(2).all(|w| w[0] <= w[1]);
        let ordenacao_pre_valida = vetor_com_pre.windows(2).all(|w| w[0] <= w[1]);

        BenchmarkResult {
            tempo_puro,
            tempo_pre_processamento,
            tempo_ordenacao_com_pre,
            tempo_total_com_pre,
            ordenacao_pura_valida,
            ordenacao_pre_valida,
        }
    }

    fn aplicar_ordenacao(sort_type: SortType, array: &mut [i32]) {
        match sort_type {
            SortType::Insertion => insertion_sort(array),
            SortType::Bubble => bubble_sort(array),
            SortType::Selection => selection_sort(array),
            SortType::Quick => quick_sort(array),
            SortType::Merge => merge_sort(array),
        }
    }
}