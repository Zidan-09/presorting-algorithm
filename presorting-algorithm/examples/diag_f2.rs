//! Diagnóstico F2 (docs/plan2.md). NÃO faz parte da matriz do artigo.
//!
//! - Pools idênticos ao bench: seed `42 ^ n ^ tipo`, mesmos geradores.
//! - Quicksort instrumentado (`quick_stats`, cópia fiel) em
//!   Reversed puro / Reversed+pré (= ordenado) / Random / Zigzag,
//!   n = 10k, 100k, 1M: contagens + tempo vs `sort_unstable`.
//! - Selection instrumentado em Random vs Reversed+pré, n = 10k, 100k.
//!
//! Uso: `cargo run --release --example diag_f2`

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::hint::black_box;
use std::time::{Duration, Instant};

use algoritmo::core::sort::pre_proc::pre_processamento_simetrico;
use algoritmo::core::sort::quick_stats::quick_sort_stats;
use algoritmo::core::sort::selection_stats::selection_sort_stats;
use algoritmo::utils::{generate_test_array, ArrayType};

fn pool(tipo: ArrayType, n: usize, k: usize) -> Vec<Vec<i32>> {
    let mut rng = ChaCha8Rng::seed_from_u64(42 ^ (n as u64) ^ (tipo as u64));
    (0..k).map(|_| generate_test_array(n, tipo, &mut rng)).collect()
}

fn mediana(mut v: Vec<Duration>) -> Duration {
    v.sort();
    v[v.len() / 2]
}

fn cronometra(reps: usize, mut f: impl FnMut()) -> Duration {
    mediana((0..reps).map(|_| {
        let t0 = Instant::now();
        f();
        black_box(t0.elapsed())
    }).collect())
}

fn ordenado(a: &[i32]) -> bool {
    a.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    // ---------- A. contagens do quicksort ----------
    println!("== A. quicksort instrumentado (5 vetores do pool por célula) ==");
    println!("tipo_var,n,cmps,cmps_por_n_ln_n,swaps,particoes,maxdepth,imb_medio,imb_pior");
    for (nome, tipo, pre) in [
        ("reversed_puro", ArrayType::Inverted, false),
        ("reversed_pre", ArrayType::Inverted, true),
        ("random", ArrayType::Random, false),
        ("zigzag", ArrayType::Zigzag, false),
    ] {
        for n in [10_000usize, 100_000, 1_000_000] {
            let (mut c, mut s, mut p, mut d, mut ib, mut iw): (u64, u64, u64, u32, f64, f64) =
                (0, 0, 0, 0, 0.0, 0.0);
            for mut v in pool(tipo, n, 5) {
                if pre {
                    pre_processamento_simetrico(&mut v);
                }
                let st = quick_sort_stats(&mut v);
                assert!(ordenado(&v), "clone divergiu: {nome} n={n}");
                c += st.comparisons;
                s += st.swaps;
                p += st.partitions;
                d = d.max(st.max_depth);
                ib += st.imbal_sum / st.partitions.max(1) as f64;
                iw = iw.max(st.imbal_worst);
            }
            let nln = n as f64 * (n as f64).ln();
            println!(
                "{nome},{n},{},{:.2},{},{},{},{:.4},{:.4}",
                c / 5,
                (c / 5) as f64 / nln,
                s / 5,
                p / 5,
                d,
                ib / 5.0,
                iw
            );
            if n == 100_000 && (nome == "reversed_puro" || nome == "random") {
                // Anatomia das primeiras divisões (n, L, R).
                let mut v: Vec<i32> = pool(tipo, n, 1).pop().unwrap();
                if pre {
                    pre_processamento_simetrico(&mut v);
                }
                let st0 = quick_sort_stats(&mut v);
                println!("  splits {nome}: {:?}", st0.splits_top);
            }
        }
    }

    // ---------- B. tempo quick instrumentado vs sort_unstable ----------
    println!("== B. tempo: quick_clone vs sort_unstable (mediana de 5 reps, 3 vetores) ==");
    println!("tipo_var,n,quick_ms,std_ms,razao");
    for (nome, tipo, pre) in [
        ("reversed_puro", ArrayType::Inverted, false),
        ("reversed_pre", ArrayType::Inverted, true),
        ("random", ArrayType::Random, false),
    ] {
        for n in [10_000usize, 100_000, 1_000_000] {
            let (mut tq, mut ts) = (Duration::ZERO, Duration::ZERO);
            for v0 in pool(tipo, n, 3) {
                let mut v = v0.clone();
                if pre {
                    pre_processamento_simetrico(&mut v);
                }
                let base = v.clone();
                tq += cronometra(5, || {
                    let mut w = base.clone();
                    quick_sort_stats(&mut w);
                    black_box(w);
                });
                ts += cronometra(5, || {
                    let mut w = base.clone();
                    w.sort_unstable();
                    black_box(w);
                });
            }
            println!(
                "{nome},{n},{:.3},{:.3},{:.2}",
                tq.as_secs_f64() * 1000.0 / 3.0,
                ts.as_secs_f64() * 1000.0 / 3.0,
                tq.as_secs_f64() / ts.as_secs_f64().max(1e-12)
            );
        }
    }

    // ---------- C. selection: contagens ----------
    println!("== C. selection instrumentado (5 vetores por célula) ==");
    println!("tipo_var,n,cmps,updates,swaps");
    for (nome, tipo, pre) in [
        ("random", ArrayType::Random, false),
        ("reversed_puro", ArrayType::Inverted, false),
        ("reversed_pre", ArrayType::Inverted, true),
    ] {
        for n in [10_000usize, 100_000] {
            let (mut c, mut u, mut s) = (0u64, 0u64, 0u64);
            for mut v in pool(tipo, n, 5) {
                if pre {
                    pre_processamento_simetrico(&mut v);
                }
                let st = selection_sort_stats(&mut v);
                assert!(ordenado(&v), "clone divergiu: {nome} n={n}");
                c += st.comparisons;
                u += st.min_updates;
                s += st.swaps;
            }
            println!("{nome},{n},{},{},{}", c / 5, u / 5, s / 5);
        }
    }

    // ---------- D. selection intercalado: descarta deriva temporal ----------
    // O bench mede puro antes do com_pre dentro da célula; rounds
    // round-robin isolam deriva de frequência/temperatura do efeito de input.
    println!("== D. selection intercalado (5 rounds x mediana de 5 reps, ms) ==");
    for n in [10_000usize, 100_000] {
        let pools = [
            ("random", pool(ArrayType::Random, n, 5)),
            ("reversed_puro", pool(ArrayType::Inverted, n, 5)),
            ("reversed_pre", {
                let mut pp = pool(ArrayType::Inverted, n, 5);
                for v in &mut pp {
                    pre_processamento_simetrico(v);
                }
                pp
            }),
        ];
        print!("n={n}");
        for (nome, pv) in &pools {
            let mut rounds = Vec::new();
            for r in 0..5 {
                let base = pv[r].clone();
                let t = cronometra(5, || {
                    let mut w = base.clone();
                    selection_sort_stats(&mut w);
                    black_box(w);
                });
                rounds.push(t.as_secs_f64() * 1000.0);
            }
            let media = rounds.iter().sum::<f64>() / rounds.len() as f64;
            print!(" | {nome}: [{}] media={:.2}", {
                rounds.iter().map(|x| format!("{x:.2}")).collect::<Vec<_>>().join(",")
            }, media);
        }
        println!();
    }
}
