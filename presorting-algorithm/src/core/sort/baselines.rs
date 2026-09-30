use super::insertion::insertion_sort;

/// Baseline competitiva (F1, docs/plan2.md): detecção O(n) de run
/// descendente. Se o vetor é estritamente decrescente, reverte in-place
/// (ficando ordenado, caso O(n) do insertion) e aplica insertion sort;
/// caso contrário, aplica insertion sort direto.
pub fn desc_reverse_insertion(array: &mut [i32]) {
    if array.len() > 1 && array.windows(2).all(|w| w[0] > w[1]) {
        array.reverse();
    }
    insertion_sort(array);
}
