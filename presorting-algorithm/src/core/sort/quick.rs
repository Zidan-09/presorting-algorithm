pub fn quick_sort(array: &mut [i32]) {
    let len = array.len();
    if len < 2 { return; }
    quick_sort_recursivo(array, 0, (len - 1) as i32);
}

fn quick_sort_recursivo(array: &mut [i32], mut inicio: i32, mut fim: i32) {
    while inicio < fim {
        let (lt, gt) = particao_tres_vias(array, inicio, fim);

        if (lt - 1 - inicio) < (fim - (gt + 1)) {
            quick_sort_recursivo(array, inicio, lt - 1);
            inicio = gt + 1;
        } else {
            quick_sort_recursivo(array, gt + 1, fim);
            fim = lt - 1;
        }
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
    array.swap(indice_pivo, inicio_u);

    let pivo = array[inicio_u];
    let mut lt = inicio;
    let mut i = inicio + 1;
    let mut gt = fim;

    while i <= gt {
        if array[i as usize] < pivo {
            array.swap(lt as usize, i as usize);
            lt += 1;
            i += 1;
        } else if array[i as usize] > pivo {
            array.swap(i as usize, gt as usize);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    (lt, gt)
}