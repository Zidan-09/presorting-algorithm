use std::time::{Duration, Instant};
use stats_alloc::Region;
use std::hint::black_box;

use crate::utils::tipos::{ArrayType, SortType};
use crate::utils::gerador::generate_test_array;
use crate::core::sort::{
    insertion::insertion_sort, bubble::bubble_sort, selection::selection_sort,
    quick::quick_sort, merge::merge_sort, pre_proc::pre_processamento_simetrico,
    contar_inversoes::contar_inversoes
};

pub struct MetricasDados {
    pub tempo: Duration,
    pub memoria_alocada_bytes: usize,
    pub cache_misses: u64,
    pub valido: bool,
}

pub struct BenchmarkCompletoResult {
    pub puro: MetricasDados,
    pub com_pre: MetricasDados,
    pub tempo_so_pre_processamento: Duration,
    pub inversoes_iniciais: u64,
    pub inversoes_pos_pre_processamento: u64
}

pub struct BenchmarkService;

impl BenchmarkService {
    pub fn executar_teste(size: usize, array_type: ArrayType, sort_type: SortType) -> BenchmarkCompletoResult {
        let vetor_original = generate_test_array(size, array_type);
        let mut vetor_puro = vetor_original.clone();
        let mut vetor_com_pre = vetor_original;

        let inversoes_inicial = contar_inversoes(&vetor_puro);

        let regiao_memoria_pura = Region::new(&crate::GLOBAL);
        let monitor_cache_puro = MonitorCache::iniciar();
        
        let inicio_puro = Instant::now();
        Self::aplicar_ordenacao(sort_type, &mut vetor_puro);
        let tempo_puro = inicio_puro.elapsed();

        black_box(&vetor_puro);
        
        let cache_puro = monitor_cache_puro.finalizar();
        let memoria_pura = regiao_memoria_pura.change();

        let puro_valido = vetor_puro.windows(2).all(|w| w[0] <= w[1]);

        let inicio_so_pre = Instant::now();
        pre_processamento_simetrico(&mut vetor_com_pre);
        let tempo_so_pre = inicio_so_pre.elapsed();

        let depois = contar_inversoes(&vetor_com_pre);

        black_box(&vetor_com_pre);

        let regiao_memoria_pre = Region::new(&crate::GLOBAL);
        let monitor_cache_pre = MonitorCache::iniciar();

        let inicio_sort_pre = Instant::now();
        Self::aplicar_ordenacao(sort_type, &mut vetor_com_pre);
        let tempo_sort_pre = inicio_sort_pre.elapsed();

        black_box(&vetor_com_pre);

        let cache_pre = monitor_cache_pre.finalizar();
        let memoria_pre = regiao_memoria_pre.change();

        let pre_valido = vetor_com_pre.windows(2).all(|w| w[0] <= w[1]);

        BenchmarkCompletoResult {
            puro: MetricasDados {
                tempo: tempo_puro,
                memoria_alocada_bytes: memoria_pura.bytes_allocated,
                cache_misses: cache_puro,
                valido: puro_valido,
            },
            com_pre: MetricasDados {
                tempo: tempo_sort_pre,
                memoria_alocada_bytes: memoria_pre.bytes_allocated,
                cache_misses: cache_pre,
                valido: pre_valido,
            },
            tempo_so_pre_processamento: tempo_so_pre,
            inversoes_iniciais: inversoes_inicial,
            inversoes_pos_pre_processamento: depois
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

struct MonitorCache {
    #[cfg(target_os = "linux")]
    counter: Option<perf_event::Counter>,
}

impl MonitorCache {
    #[allow(unused_variables)]
    fn iniciar() -> Self {
        #[cfg(target_os = "linux")]
        {
            use perf_event::events::Hardware;
            let mut group = perf_event::Group::new().into_ok();
            let counter = perf_event::Builder::new()
                .group(&mut group)
                .kind(Hardware::CACHE_MISSES)
                .build()
                .ok();
            if let Some(mut c) = counter {
                c.enable().ok();
                return MonitorCache { counter: Some(c) };
            }
        }
        MonitorCache {
            #[cfg(target_os = "linux")]
            counter: None,
        }
    }

    fn finalizar(self) -> u64 {
        #[cfg(target_os = "linux")]
        {
            if let Some(mut c) = self.counter {
                c.disable().ok();
                return c.read().unwrap_or(0);
            }
        }
        0
    }
}