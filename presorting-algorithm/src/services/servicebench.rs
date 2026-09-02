use std::time::{Duration, Instant};
use std::hint::black_box;
use crate::utils::tipos::SortType;
use crate::core::sort::{
    insertion::insertion_sort, bubble::bubble_sort, selection::selection_sort,
    quick::quick_sort, merge::merge_sort, pre_proc::pre_processamento_simetrico,
};

pub struct BenchmarkServiceBench;

impl BenchmarkServiceBench {
    pub fn medir_puro(sort_type: SortType, array: &mut [i32]) -> Duration {
        let inicio = Instant::now();
        Self::aplicar_ordenacao(sort_type, array);
        let tempo = inicio.elapsed();
        
        black_box(array);
        tempo
    }

    pub fn medir_com_pre(sort_type: SortType, array: &mut [i32]) -> Duration {
        let inicio = Instant::now();
        
        pre_processamento_simetrico(array);
        
        Self::aplicar_ordenacao(sort_type, array);
        
        let tempo = inicio.elapsed();
        
        black_box(array);
        tempo
    }

    pub fn aplicar_ordenacao(sort_type: SortType, array: &mut [i32]) {
        match sort_type {
            SortType::Insertion => insertion_sort(array),
            SortType::Bubble => bubble_sort(array),
            SortType::Selection => selection_sort(array),
            SortType::Quick => quick_sort(array),
            SortType::Merge => merge_sort(array),
        }
    }
}