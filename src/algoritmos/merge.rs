pub fn merge_sort(array: &mut [i32]) {
    let len = array.len();
    if len < 2 { return; }
    
    let mut auxiliar = vec![0; len];
    merge_sort_recursivo(array, &mut auxiliar, 0, len);
}

fn merge_sort_recursivo(array: &mut [i32], auxiliar: &mut [i32], inicio: usize, fim: usize) {
    if fim - inicio < 2 { return; }

    let meio = inicio + ((fim - inicio) >> 1);
    
    merge_sort_recursivo(array, auxiliar, inicio, meio);
    merge_sort_recursivo(array, auxiliar, meio, fim);
    
    intercalar(array, auxiliar, inicio, meio, fim);
}

fn intercalar(array: &mut [i32], auxiliar: &mut [i32], inicio: usize, meio: usize, fim: usize) {
    auxiliar[inicio..fim].copy_from_slice(&array[inicio..fim]);

    let mut i = inicio;
    let mut j = meio;
    let mut k = inicio;

    while i < meio && j < fim {
        if auxiliar[i] <= auxiliar[j] {
            array[k] = auxiliar[i];
            i += 1;
        } else {
            array[k] = auxiliar[j];
            j += 1;
        }
        k += 1;
    }

    while i < meio {
        array[k] = auxiliar[i];
        i += 1;
        k += 1;
    }
}