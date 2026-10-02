//! F4 (docs/plan2.md): experimento pareado offline por pool-slot.
//!
//! Para cada célula planejada (sort, tipo, tamanho) e cada seed, mede
//! puro vs. pré+sort **nos mesmos vetores** (slots 0..R-1 do pool idêntico ao
//! do bench: `ChaCha8Rng::seed_from_u64(seed ^ n ^ tipo)`), funde a ordem dos
//! braços por repetição (controle de deriva, cf. `service.rs`) e estima o
//! efeito como média de log-razões por vetor, com IC percentílico por
//! bootstrap (B reamostragens) — substituindo a regra "ICs não sobrepostos".
//!
//! Uso (a partir da raiz do crate):
//!   rm -f ../artigo/resultados/efeito_pareado.csv
//!   FP_SEEDS="42,43,44,45,46" FP_TAMS="10000" FP_R="50" FP_BOOT="10000" \
//!     cargo run --release --example efeito_pareado
//!
//! Saída: CSV em `FP_OUT` (default `../artigo/resultados/efeito_pareado.csv`);
//! cabeçalho escrito só se o arquivo estiver ausente/vazio (acumula seeds).

use algoritmo::core::sort::pre_proc::pre_processamento_simetrico;
use algoritmo::services::BenchmarkServiceBench;
use algoritmo::utils::gerador::REAL_NATIVE_LEN;
use algoritmo::utils::{generate_test_array, ArrayType, SortType};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

const SORTS: [(SortType, &str); 8] = [
    (SortType::Merge, "merge"),
    (SortType::Quick, "quick"),
    (SortType::Insertion, "insertion"),
    (SortType::Bubble, "bubble"),
    (SortType::Selection, "selection"),
    (SortType::StdUnstable, "stdunstable"),
    (SortType::StdStable, "stdstable"),
    (SortType::DescReverse, "descreverse"),
];

const TIPOS: [(ArrayType, &str); 10] = [
    (ArrayType::Random, "random"),
    (ArrayType::Turtles, "turtles"),
    (ArrayType::Zigzag, "zigzag"),
    (ArrayType::AlmostSorted, "almostsorted"),
    (ArrayType::Duplicates, "duplicates"),
    (ArrayType::Inverted, "inverted"),
    (ArrayType::Sawtooth, "sawtooth"),
    (ArrayType::OrganPipe, "organpipe"),
    (ArrayType::FewRuns, "fewruns"),
    (ArrayType::Real, "real"),
];

fn e_quadratico(s: SortType) -> bool {
    matches!(
        s,
        SortType::Insertion | SortType::Bubble | SortType::Selection | SortType::DescReverse
    )
}

/// Espelho de `celula_planejada` em benches/benchmark.rs.
fn celula_planejada(s: SortType, tipo: ArrayType, tamanho: usize) -> bool {
    if e_quadratico(s) && tamanho >= 1_000_000 {
        return false;
    }
    if tamanho >= 10_000_000 {
        return matches!(
            s,
            SortType::Merge | SortType::Quick | SortType::StdUnstable | SortType::StdStable
        );
    }
    if tipo == ArrayType::Real && tamanho > REAL_NATIVE_LEN {
        return false;
    }
    true
}

fn env_lista_u64(chave: &str, padrao: &str) -> Vec<u64> {
    std::env::var(chave)
        .unwrap_or_else(|_| padrao.to_string())
        .split(',')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect()
}

fn env_usize_list(chave: &str, padrao: &str) -> Vec<usize> {
    std::env::var(chave)
        .unwrap_or_else(|_| padrao.to_string())
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect()
}

fn env_usize(chave: &str, padrao: usize) -> usize {
    std::env::var(chave)
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(padrao)
}

fn permutacao_preservada(original: &[i32], ordenado: &[i32]) -> bool {
    let mut a = original.to_vec();
    let mut b = ordenado.to_vec();
    a.sort_unstable();
    b.sort_unstable();
    a == b
}

#[allow(clippy::too_many_arguments)]
fn experimento_celula(
    seed: u64,
    sort: SortType,
    tipo: ArrayType,
    tamanho: usize,
    r: usize,
    boot: usize,
) -> (Vec<String>, bool) {
    // Pool idêntico ao do bench (mesma semente composta, mesmo stream).
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ (tamanho as u64) ^ (tipo as u64));
    let pool: Vec<Vec<i32>> = (0..r)
        .map(|_| generate_test_array(tamanho, tipo, &mut rng))
        .collect();

    // Aquecimento fora do timing (espelha service.rs).
    for k in 0..2 {
        let mut a = pool[k % r].clone();
        let mut b = pool[k % r].clone();
        if k % 2 == 0 {
            BenchmarkServiceBench::aplicar_ordenacao(sort, &mut a);
            pre_processamento_simetrico(&mut b);
            BenchmarkServiceBench::aplicar_ordenacao(sort, &mut b);
        } else {
            pre_processamento_simetrico(&mut b);
            BenchmarkServiceBench::aplicar_ordenacao(sort, &mut b);
            BenchmarkServiceBench::aplicar_ordenacao(sort, &mut a);
        }
    }

    let mut t_puro = Vec::with_capacity(r);
    let mut t_pre = Vec::with_capacity(r);
    let mut valido = true;

    // Sonda p/ calibrar repetições internas: alvo ~50 µs por medição, longe
    // da granularidade do relógio (evita 0 ns e NaN em n pequeno).
    let sonda = {
        let mut a = pool[0].clone();
        BenchmarkServiceBench::medir_puro(sort, &mut a).as_nanos()
    };
    let inner = (50_000u128 / sonda.max(1)).clamp(1, 50_000) as usize;

    for (rep, orig) in pool.iter().enumerate() {
        let (dp_total, dc_total);
        let (mut va, mut vb) = (Vec::new(), Vec::new());
        if rep % 2 == 0 {
            let mut acc_p = 0u128;
            for k in 0..inner {
                let mut a = orig.clone();
                acc_p += BenchmarkServiceBench::medir_puro(sort, &mut a).as_nanos();
                if k == 0 {
                    va = a;
                }
            }
            let mut acc_c = 0u128;
            for k in 0..inner {
                let mut b = orig.clone();
                acc_c += BenchmarkServiceBench::medir_com_pre(sort, &mut b).as_nanos();
                if k == 0 {
                    vb = b;
                }
            }
            dp_total = acc_p;
            dc_total = acc_c;
        } else {
            let mut acc_c = 0u128;
            for k in 0..inner {
                let mut b = orig.clone();
                acc_c += BenchmarkServiceBench::medir_com_pre(sort, &mut b).as_nanos();
                if k == 0 {
                    vb = b;
                }
            }
            let mut acc_p = 0u128;
            for k in 0..inner {
                let mut a = orig.clone();
                acc_p += BenchmarkServiceBench::medir_puro(sort, &mut a).as_nanos();
                if k == 0 {
                    va = a;
                }
            }
            dp_total = acc_p;
            dc_total = acc_c;
        }
        assert!(
            dp_total > 0 && dc_total > 0,
            "relógio retornou 0 (inner={} insuficiente)",
            inner
        );
        // Validação fora do timing: ordenação (todo rep) + permutação (rep 0).
        let ok_a = va.windows(2).all(|w| w[0] <= w[1]);
        let ok_b = vb.windows(2).all(|w| w[0] <= w[1]);
        if rep == 0 && (!permutacao_preservada(orig, &va) || !permutacao_preservada(orig, &vb)) {
            valido = false;
        }
        valido &= ok_a && ok_b;
        t_puro.push(dp_total as f64 / inner as f64);
        t_pre.push(dc_total as f64 / inner as f64);
    }

    // Log-razão por vetor: estatística pareada (simétrica, aditiva).
    let lr: Vec<f64> = t_puro
        .iter()
        .zip(t_pre.iter())
        .map(|(p, c)| (c / p).ln())
        .collect();
    let lr_media = lr.iter().sum::<f64>() / r as f64;
    let media_puro = t_puro.iter().sum::<f64>() / r as f64;
    let media_pre = t_pre.iter().sum::<f64>() / r as f64;
    let ganho = (media_puro - media_pre) / media_puro * 100.0;

    // Bootstrap percentílico da média das log-razões (RNG determinística).
    let sidx = SORTS.iter().position(|(s, _)| *s == sort).unwrap_or(0) as u64;
    let mut brng = ChaCha8Rng::seed_from_u64(
        seed.wrapping_add((tamanho as u64).wrapping_mul(0x9E3779B1))
            .wrapping_add((tipo as u64) << 20)
            .wrapping_add(sidx << 40)
            .wrapping_add(0xC2B2AE3D27D4EB4F),
    );
    let mut medias = Vec::with_capacity(boot);
    for _ in 0..boot {
        let mut s = 0.0;
        for _ in 0..r {
            s += lr[brng.gen_range(0..r)];
        }
        medias.push(s / r as f64);
    }
    medias.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    assert!(
        lr_media.is_finite(),
        "log-razão não finita (tempos zerados?)"
    );
    let idx_lo = ((boot as f64 * 0.025).floor() as usize).min(boot - 1);
    let idx_hi = ((boot as f64 * 0.975).floor() as usize).min(boot - 1);
    let lr_lo = medias[idx_lo];
    let lr_hi = medias[idx_hi];
    let p_melhora = medias.iter().filter(|&&m| m < 0.0).count() as f64 / boot as f64;

    // Ganho% com IC (transformação monotônica: lr menor = ganho maior).
    let ganho_lo = (1.0 - lr_hi.exp()) * 100.0;
    let ganho_hi = (1.0 - lr_lo.exp()) * 100.0;
    let sig = lr_hi < 0.0 || lr_lo > 0.0;
    let sinal = if lr_media < 0.0 {
        "+"
    } else if lr_media > 0.0 {
        "-"
    } else {
        "0"
    };

    let linha = vec![
        seed.to_string(),
        SORTS.iter().find(|(s, _)| *s == sort).unwrap().1.to_string(),
        TIPOS.iter().find(|(t, _)| *t == tipo).unwrap().1.to_string(),
        tamanho.to_string(),
        r.to_string(),
        format!("{:.0}", media_puro),
        format!("{:.0}", media_pre),
        format!("{:.2}", ganho),
        format!("{:.2}", ganho_lo),
        format!("{:.2}", ganho_hi),
        format!("{:.4}", lr_media),
        format!("{:.4}", lr_lo),
        format!("{:.4}", lr_hi),
        format!("{:.4}", p_melhora),
        if sig { "1" } else { "0" }.to_string(),
        sinal.to_string(),
        if valido { "1" } else { "0" }.to_string(),
    ];
    (linha, valido)
}

fn main() {
    let seeds = env_lista_u64("FP_SEEDS", "42");
    let tamanhos = env_usize_list("FP_TAMS", "10000,1000000");
    let r = env_usize("FP_R", 50).max(2);
    let boot = env_usize("FP_BOOT", 10000).max(100);
    let out = std::env::var("FP_OUT").unwrap_or_else(|_| "../artigo/resultados/efeito_pareado.csv".to_string());

    let precisa_cabecalho = std::fs::metadata(&out).map(|m| m.len() == 0).unwrap_or(true);
    let mut arq = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&out)
        .unwrap();
    if precisa_cabecalho {
        writeln!(arq, "seed,sort,tipo,tamanho,R,media_puro_ns,media_pre_ns,ganho_pct,ganho_lo,ganho_hi,lr_media,lr_lo,lr_hi,p_melhora,sig,sinal,valido").unwrap();
    }

    let mut celulas = 0usize;
    let mut invalidas = 0usize;
    let t0 = Instant::now();
    for seed in seeds {
        for &tamanho in &tamanhos {
            for (tipo, _) in TIPOS {
                for (sort, snome) in SORTS {
                    if !celula_planejada(sort, tipo, tamanho) {
                        continue;
                    }
                    let tnome = TIPOS.iter().find(|(t, _)| *t == tipo).unwrap().1;
                    eprintln!("[F4] seed={} {} {} n={} R={}", seed, snome, tnome, tamanho, r);
                    let (linha, valido) =
                        experimento_celula(seed, sort, tipo, tamanho, r, boot);
                    if !valido {
                        invalidas += 1;
                        eprintln!("[F4] VALIDAÇÃO FALHOU: {} {} n={} seed={}", snome, tnome, tamanho, seed);
                    }
                    writeln!(arq, "{}", linha.join(",")).unwrap();
                    arq.flush().unwrap();
                    celulas += 1;
                }
            }
        }
    }
    eprintln!(
        "[F4] OK: {} células em {:.0}s, inválidas={} -> {}",
        celulas,
        t0.elapsed().as_secs_f64(),
        invalidas,
        out
    );
    if invalidas > 0 {
        panic!("VALIDAÇÃO FALHOU em {} célula(s)", invalidas);
    }
}
