//! Diagnóstico F2 (docs/plan2.md) — CÓPIA INSTRUMENTADA do `selection.rs`.
//!
//! Lógica idêntica; conta comparações, atualizações de mínimo e trocas, para
//! decidir com números se a degradação de ~12% em Reversed é efeito de
//! preditor de desvios ou artefato (ex.: cmov sem desvio, custo das trocas).
//! Uso exclusivo em examples/diag_f2.rs. NÃO usar nos benchmarks.

#[derive(Debug, Default)]
pub struct SelStats {
    /// Comparações `array[j] < array[indice_minimo]`.
    pub comparisons: u64,
    /// Atribuições `indice_minimo = j`.
    pub min_updates: u64,
    /// Trocas efetivas (`indice_minimo != i`).
    pub swaps: u64,
}

pub fn selection_sort_stats(array: &mut [i32]) -> SelStats {
    let mut st = SelStats::default();
    let n = array.len();
    if n < 2 {
        return st;
    }

    for i in 0..n - 1 {
        let mut indice_minimo = i;

        for j in i + 1..n {
            st.comparisons += 1;
            if array[j] < array[indice_minimo] {
                indice_minimo = j;
                st.min_updates += 1;
            }
        }

        if indice_minimo != i {
            array.swap(i, indice_minimo);
            st.swaps += 1;
        }
    }
    st
}
