pub fn quick_sort(array: &mut [i32]) {
    let len = array.len();
    if len < 2 { return; }
    quick_sort_recursivo(array, 0, (len - 1) as i32);
}

fn quick_sort_recursivo(array: &mut [i32], inicio: i32, fim: i32) {
    if inicio < fim {
        let (lt, gt) = particao_tres_vias(array, inicio, fim);
        quick_sort_recursivo(array, inicio, lt - 1);
        quick_sort_recursivo(array, gt + 1, fim);
    }
}

fn mediana_de_tres(array: &[i32], inicio: usize, meio: usize, fim: usize) -> usize {
    let a = array[inicio];
    let b = array[meio];
    let c = array[fim];

    if (a <= b && b <= c) || (c <= b && b <= a) {
        meio
    } else if (b <= a && a <= c) || (c <= a && a <= b) {
        inicio
    } else {
        fim
    }
}

fn particao_tres_vias(array: &mut [i32], inicio: i32, fim: i32) -> (i32, i32) {
    let inicio_u = inicio as usize;
    let fim_u = fim as usize;
    let meio_u = inicio_u + ((fim_u - inicio_u) >> 1);

    let indice_pivo = mediana_de_tres(array, inicio_u, meio_u, fim_u);
    array.swap(indice_pivo, fim_u);

    let pivo = array[fim_u];
    let mut lt = inicio;
    let mut i = inicio;
    let mut gt = fim - 1;

    while i <= gt {
        let val = array[i as usize];
        if val < pivo {
            array.swap(lt as usize, i as usize);
            lt += 1;
            i += 1;
        } else if val > pivo {
            array.swap(i as usize, gt as usize);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    array.swap(i as usize, fim_u);
    (lt, gt)
}