pub fn quick_sort(array: &mut [i32]) {
    let len = array.len();
    if len < 2 { return; }
    quick_sort_recursivo(array, 0, (len - 1) as i32);
}

fn quick_sort_recursivo(array: &mut [i32], inicio: i32, fim: i32) {
    if inicio < fim {
        let p = particao(array, inicio, fim);
        quick_sort_recursivo(array, inicio, p - 1);
        quick_sort_recursivo(array, p + 1, fim);
    }
}

fn particao(array: &mut [i32], inicio: i32, fim: i32) -> i32 {
    let pivo = array[fim as usize];
    let mut i = inicio - 1;

    for j in inicio..fim {
        if array[j as usize] <= pivo {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }
    
    array.swap((i + 1) as usize, fim as usize);
    i + 1
}