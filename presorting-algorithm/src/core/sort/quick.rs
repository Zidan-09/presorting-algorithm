pub fn quick_sort(array: &mut [i32]) {
    let len = array.len();
    if len < 2 {
        return;
    }
    quick_sort_recursivo(array, 0, len - 1);
}

fn quick_sort_recursivo(array: &mut [i32], mut inicio: usize, mut fim: usize) {
    while inicio < fim {
        let (lt, gt) = particao_tres_vias(array, inicio, fim);

        let left_len = lt.saturating_sub(inicio);
        let right_len = fim.saturating_sub(gt);

        if left_len < right_len {
            if left_len > 0 {
                quick_sort_recursivo(array, inicio, lt - 1);
            }
            if gt == usize::MAX {
                break;
            }
            inicio = gt + 1;
            if inicio > fim {
                break;
            }
        } else {
            if right_len > 0 {
                quick_sort_recursivo(array, gt + 1, fim);
            }
            if lt == 0 {
                break;
            }
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

fn particao_tres_vias(array: &mut [i32], inicio: usize, fim: usize) -> (usize, usize) {
    let meio_u = inicio + ((fim - inicio) >> 1);

    let indice_pivo = mediana_de_tres(array, inicio, meio_u, fim);
    array.swap(indice_pivo, inicio);

    let pivo = array[inicio];
    let mut lt = inicio;
    let mut i = inicio + 1;
    let mut gt = fim;

    while i <= gt {
        if array[i] < pivo {
            array.swap(lt, i);
            lt += 1;
            i += 1;
        } else if array[i] > pivo {
            array.swap(i, gt);
            if gt == 0 {
                break;
            }
            gt -= 1;
        } else {
            i += 1;
        }
    }

    (lt, gt)
}
