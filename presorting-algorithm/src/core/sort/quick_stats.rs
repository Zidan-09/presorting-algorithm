//! Diagnóstico F2 (docs/plan2.md) — CÓPIA INSTRUMENTADA do `quick.rs`.
//!
//! O algoritmo padrão (`quick_sort`) NÃO é alterado. Este módulo duplica a
//! lógica exata (mesmos pivôs, mesmas partições, mesma recursão) adicionando
//! apenas contadores, para responder com números: a escala ~n^1.45 em
//! entradas ordenadas/invertidas/zigzag vem de particionamento degenerado,
//! de profundidade de recursão, ou de outro gargalo?
//! Uso exclusivo em examples/diag_f2.rs. NÃO usar nos benchmarks.

#[derive(Debug, Default)]
pub struct QuickStats {
    /// Comparações escalares `<` / `>` (inclui as da mediana de três).
    pub comparisons: u64,
    /// Chamadas a `swap` (inclui a troca do pivô).
    pub swaps: u64,
    /// Chamadas de particionamento.
    pub partitions: u64,
    /// Profundidade máxima de recursão.
    pub max_depth: u32,
    /// Soma de |L-R|/(L+R) sobre partições com L+R > 0 (0 = balanceada).
    pub imbal_sum: f64,
    /// Pior (mais degenerada) partição observada.
    pub imbal_worst: f64,
    /// Primeiras divisões rasas (n, L, R), depth <= 2, máx. 12 registros.
    pub splits_top: Vec<(usize, usize, usize)>,
}

pub fn quick_sort_stats(array: &mut [i32]) -> QuickStats {
    let mut st = QuickStats::default();
    let len = array.len();
    if len < 2 {
        return st;
    }
    rec(array, 0, len - 1, 1, &mut st);
    st
}

fn rec(array: &mut [i32], mut inicio: usize, mut fim: usize, depth: u32, st: &mut QuickStats) {
    if depth > st.max_depth {
        st.max_depth = depth;
    }
    while inicio < fim {
        let (lt, gt) = particao(array, inicio, fim, st);

        let left = lt.saturating_sub(inicio);
        let right = fim.saturating_sub(gt);
        let tot = left + right;
        if tot > 0 {
            let imb = ((left as i64 - right as i64).abs() as f64) / (tot as f64);
            st.imbal_sum += imb;
            if imb > st.imbal_worst {
                st.imbal_worst = imb;
            }
        }
        if depth <= 2 && st.splits_top.len() < 12 {
            st.splits_top.push((tot + 1, left, right));
        }

        if left < right {
            if left > 0 {
                rec(array, inicio, lt - 1, depth + 1, st);
            }
            if gt == usize::MAX {
                break;
            }
            inicio = gt + 1;
            if inicio > fim {
                break;
            }
        } else {
            if right > 0 {
                rec(array, gt + 1, fim, depth + 1, st);
            }
            if lt == 0 {
                break;
            }
            fim = lt - 1;
        }
    }
}

/// Réplica exata do fluxo de curto-circuito da mediana de três original:
///
/// ```text
/// if (a <= b && b <= c) || (c <= b && b <= a) { meio }
/// else if (b <= a && a <= c) || (c <= a && a <= b) { inicio }
/// else { fim }
/// ```
fn mediana(array: &[i32], inicio: usize, meio: usize, fim: usize, st: &mut QuickStats) -> usize {
    let a = array[inicio];
    let b = array[meio];
    let c = array[fim];

    st.comparisons += 1;
    if a <= b {
        st.comparisons += 1;
        if b <= c {
            return meio;
        }
    }
    st.comparisons += 1;
    if c <= b {
        st.comparisons += 1;
        if b <= a {
            return meio;
        }
    }
    st.comparisons += 1;
    if b <= a {
        st.comparisons += 1;
        if a <= c {
            return inicio;
        }
    }
    st.comparisons += 1;
    if c <= a {
        st.comparisons += 1;
        if a <= b {
            return inicio;
        }
    }
    fim
}

fn particao(array: &mut [i32], inicio: usize, fim: usize, st: &mut QuickStats) -> (usize, usize) {
    st.partitions += 1;
    let meio_u = inicio + ((fim - inicio) >> 1);

    let indice_pivo = mediana(array, inicio, meio_u, fim, st);
    array.swap(indice_pivo, inicio);
    st.swaps += 1;

    let pivo = array[inicio];
    let mut lt = inicio;
    let mut i = inicio + 1;
    let mut gt = fim;

    while i <= gt {
        st.comparisons += 1;
        if array[i] < pivo {
            array.swap(lt, i);
            st.swaps += 1;
            lt += 1;
            i += 1;
        } else {
            st.comparisons += 1;
            if array[i] > pivo {
                array.swap(i, gt);
                st.swaps += 1;
                if gt == 0 {
                    break;
                }
                gt -= 1;
            } else {
                i += 1;
            }
        }
    }

    (lt, gt)
}
