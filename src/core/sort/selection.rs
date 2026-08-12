pub fn selection_sort(array: &mut [i32]) {
    let n = array.len();
    if n < 2 { return; }

    for i in 0..n - 1 {
        let mut indice_minimo = i;
        
        for j in i + 1..n {
            if array[j] < array[indice_minimo] {
                indice_minimo = j;
            }
        }

        if indice_minimo != i {
            array.swap(i, indice_minimo);
        }
    }
}