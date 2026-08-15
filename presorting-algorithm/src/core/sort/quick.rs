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
    let inicio_u = inicio as usize;
    let fim_u = fim as usize;
    let meio_u = inicio_u + ((fim_u - inicio_u) >> 1);

    let a = array[inicio_u];
    let b = array[meio_u];
    let c = array[fim_u];

    let indice_pivo = if (a <= b && b <= c) || (c <= b && b <= a) {
        meio_u
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        inicio_u
    } else {
        fim_u
    };

    array.swap(indice_pivo, fim_u);

    let pivo = array[fim_u];
    let mut i = inicio - 1;

    for j in inicio..fim {
        if array[j as usize] <= pivo {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }

    array.swap((i + 1) as usize, fim_u);
    i + 1
}