pub fn contar_inversoes(array: &[i32]) -> u64 {
    if array.len() < 2 {
        return 0;
    }
    let mut auxiliar = vec![0; array.len()];
    let mut arr_copy = array.to_vec();
    contar_inversoes_merge(&mut arr_copy, &mut auxiliar, 0, array.len())
}

fn contar_inversoes_merge(array: &mut [i32], auxiliar: &mut [i32], inicio: usize, fim: usize) -> u64 {
    if fim - inicio < 2 {
        return 0;
    }

    let meio = inicio + ((fim - inicio) >> 1);
    let mut inversoes = 0;

    inversoes += contar_inversoes_merge(array, auxiliar, inicio, meio);
    inversoes += contar_inversoes_merge(array, auxiliar, meio, fim);
    inversoes += intercalar_contar(array, auxiliar, inicio, meio, fim);

    inversoes
}

fn intercalar_contar(array: &mut [i32], auxiliar: &mut [i32], inicio: usize, meio: usize, fim: usize) -> u64 {
    auxiliar[inicio..fim].copy_from_slice(&array[inicio..fim]);

    let mut i = inicio;
    let mut j = meio;
    let mut k = inicio;
    let mut inversoes = 0u64;

    while i < meio && j < fim {
        if auxiliar[i] <= auxiliar[j] {
            array[k] = auxiliar[i];
            i += 1;
        } else {
            array[k] = auxiliar[j];
            inversoes += (meio - i) as u64;
            j += 1;
        }
        k += 1;
    }

    while i < meio {
        array[k] = auxiliar[i];
        i += 1;
        k += 1;
    }

    while j < fim {
        array[k] = auxiliar[j];
        j += 1;
        k += 1;
    }

    inversoes
}