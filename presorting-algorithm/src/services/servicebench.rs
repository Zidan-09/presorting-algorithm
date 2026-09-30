use std::time::{Duration, Instant};
use std::hint::black_box;
use crate::utils::tipos::SortType;
use crate::core::sort::{
    insertion::insertion_sort, bubble::bubble_sort, selection::selection_sort,
    quick::quick_sort, merge::merge_sort, pre_proc::pre_processamento_simetrico,
    baselines::desc_reverse_insertion,
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
            SortType::StdUnstable => array.sort_unstable(),
            SortType::StdStable => array.sort(),
            SortType::DescReverse => desc_reverse_insertion(array),
        }
    }

    /// F1 (docs/plan2.md): cronometra SOMENTE o pré-processamento, no mesmo
    /// pool e com o mesmo `iter_batched` dos outros braços. Permite
    /// DeltaCsort = Ctotal - Cpre por célula (tipo, tamanho).
    pub fn medir_pre(array: &mut [i32]) -> Duration {
        let inicio = Instant::now();
        pre_processamento_simetrico(array);
        let tempo = inicio.elapsed();

        black_box(array);
        tempo
    }
}