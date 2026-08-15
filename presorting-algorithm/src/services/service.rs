use std::time::{Duration, Instant};
use stats_alloc::Region;
use std::hint::black_box;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::utils::tipos::{ArrayType, SortType};
use crate::utils::gerador::generate_test_array;
use crate::core::sort::{
    insertion::insertion_sort, bubble::bubble_sort, selection::selection_sort,
    quick::quick_sort, merge::merge_sort, pre_proc::pre_processamento_simetrico,
    contar_inversoes::contar_inversoes
};

pub struct MetricasDados {
    pub tempo: Duration,
    pub tempo_mediana: Duration,
    pub tempos: Vec<Duration>,
    pub memoria_alocada_bytes: usize,
    pub cache_misses: u64,
    pub valido: bool,
}

pub struct BenchmarkCompletoResult {
    pub puro: MetricasDados,
    pub com_pre: MetricasDados,
    pub tempo_so_pre_processamento: Duration,
    pub tempos_so_pre_processamento: Vec<Duration>,
    pub inversoes_iniciais: u64,
    pub inversoes_pos_pre_processamento: u64,
    pub seed: u64,
    pub repeticoes: usize,
}

pub struct BenchmarkService;

impl BenchmarkService {
    pub fn executar_teste(
        size: usize,
        array_type: ArrayType,
        sort_type: SortType,
        seed: u64,
        repeticoes: usize,
    ) -> BenchmarkCompletoResult {
        assert!(repeticoes >= 1, "repeticoes deve ser >= 1");

        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        for _ in 0..2 {
            let mut vetor = generate_test_array(size, array_type, &mut rng);
            Self::aplicar_ordenacao(sort_type, &mut vetor);
            black_box(&vetor);
        }

        let mut tempos_puro = Vec::with_capacity(repeticoes);
        let mut tempos_com_pre = Vec::with_capacity(repeticoes);
        let mut tempos_so_pre = Vec::with_capacity(repeticoes);
        let mut memorias_puro = Vec::with_capacity(repeticoes);
        let mut memorias_com_pre = Vec::with_capacity(repeticoes);
        let mut caches_puro = Vec::with_capacity(repeticoes);
        let mut caches_com_pre = Vec::with_capacity(repeticoes);
        let mut validos_puro = Vec::with_capacity(repeticoes);
        let mut validos_com_pre = Vec::with_capacity(repeticoes);

        let mut inversoes_iniciais = 0u64;
        let mut inversoes_pos_pre_processamento = 0u64;

        for rep in 0..repeticoes {
            let vetor_original = generate_test_array(size, array_type, &mut rng);

            if rep == 0 {
                inversoes_iniciais = contar_inversoes(&vetor_original);
                let mut sonda = vetor_original.clone();
                pre_processamento_simetrico(&mut sonda);
                inversoes_pos_pre_processamento = contar_inversoes(&sonda);
            }

            let mut vetor_puro = vetor_original.clone();
            let mut vetor_com_pre = vetor_original;

            if rep % 2 == 0 {
                let (tempo, memoria, cache, valido) =
                    Self::medir_puro(sort_type, &mut vetor_puro);
                tempos_puro.push(tempo);
                memorias_puro.push(memoria);
                caches_puro.push(cache);
                validos_puro.push(valido);

                let (tempo_pre, tempo_total, memoria, cache, valido) =
                    Self::medir_com_pre(sort_type, &mut vetor_com_pre);
                tempos_so_pre.push(tempo_pre);
                tempos_com_pre.push(tempo_total);
                memorias_com_pre.push(memoria);
                caches_com_pre.push(cache);
                validos_com_pre.push(valido);
            } else {
                let (tempo_pre, tempo_total, memoria, cache, valido) =
                    Self::medir_com_pre(sort_type, &mut vetor_com_pre);
                tempos_so_pre.push(tempo_pre);
                tempos_com_pre.push(tempo_total);
                memorias_com_pre.push(memoria);
                caches_com_pre.push(cache);
                validos_com_pre.push(valido);

                let (tempo, memoria, cache, valido) =
                    Self::medir_puro(sort_type, &mut vetor_puro);
                tempos_puro.push(tempo);
                memorias_puro.push(memoria);
                caches_puro.push(cache);
                validos_puro.push(valido);
            }
        }

        BenchmarkCompletoResult {
            puro: Self::agregar(&tempos_puro, &memorias_puro, &caches_puro, &validos_puro),
            com_pre: Self::agregar(&tempos_com_pre, &memorias_com_pre, &caches_com_pre, &validos_com_pre),
            tempo_so_pre_processamento: Self::media_duration(&tempos_so_pre),
            tempos_so_pre_processamento: tempos_so_pre,
            inversoes_iniciais,
            inversoes_pos_pre_processamento,
            seed,
            repeticoes,
        }
    }

    fn agregar(tempos: &[Duration], memorias: &[usize], caches: &[u64], validos: &[bool]) -> MetricasDados {
        let n = tempos.len().max(1);
        let soma_memoria = memorias.iter().copied().fold(0usize, usize::saturating_add);
        let soma_cache = caches.iter().copied().fold(0u64, u64::saturating_add);

        MetricasDados {
            tempo: Self::media_duration(tempos),
            tempo_mediana: Self::mediana_duration(tempos),
            tempos: tempos.to_vec(),
            memoria_alocada_bytes: soma_memoria / n,
            cache_misses: soma_cache / n as u64,
            valido: validos.iter().all(|&v| v),
        }
    }

    fn media_duration(tempos: &[Duration]) -> Duration {
        if tempos.is_empty() {
            return Duration::ZERO;
        }
        let soma: u128 = tempos.iter().map(|d| d.as_nanos()).sum();
        Duration::from_nanos((soma / tempos.len() as u128) as u64)
    }

    fn mediana_duration(tempos: &[Duration]) -> Duration {
        if tempos.is_empty() {
            return Duration::ZERO;
        }
        let mut ord = tempos.to_vec();
        ord.sort();
        let n = ord.len();
        if n % 2 == 1 {
            ord[n / 2]
        } else {
            let soma = ord[n / 2 - 1] + ord[n / 2];
            Duration::from_nanos((soma.as_nanos() / 2) as u64)
        }
    }

    fn medir_puro(sort_type: SortType, array: &mut [i32]) -> (Duration, usize, u64, bool) {
        let regiao_memoria = Region::new(&crate::GLOBAL);
        let monitor_cache = MonitorCache::iniciar();

        let inicio = Instant::now();
        Self::aplicar_ordenacao(sort_type, array);
        let tempo = inicio.elapsed();

        black_box(&mut *array);

        let cache = monitor_cache.finalizar();
        let memoria = regiao_memoria.change();
        let valido = array.windows(2).all(|w| w[0] <= w[1]);

        (tempo, memoria.bytes_allocated, cache, valido)
    }

    fn medir_com_pre(sort_type: SortType, array: &mut [i32]) -> (Duration, Duration, usize, u64, bool) {
        let regiao_memoria = Region::new(&crate::GLOBAL);
        let monitor_cache = MonitorCache::iniciar();

        let inicio = Instant::now();
        pre_processamento_simetrico(array);
        let tempo_so_pre = inicio.elapsed();

        Self::aplicar_ordenacao(sort_type, array);
        let tempo_total = inicio.elapsed();

        black_box(&mut *array);

        let cache = monitor_cache.finalizar();
        let memoria = regiao_memoria.change();
        let valido = array.windows(2).all(|w| w[0] <= w[1]);

        (tempo_so_pre, tempo_total, memoria.bytes_allocated, cache, valido)
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
