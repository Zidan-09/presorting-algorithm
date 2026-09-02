pub fn pre_processamento_simetrico(array: &mut [i32]) {    
    let len = array.len();
    if len < 2 { return; }

    let ultimo = len - 1;
    let meio = len >> 1;

    for i in 0..meio {
        let j = ultimo - i;

        if array[i] > array[j] {
            array.swap(i, j);
        }
        if i + 1 < meio {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
            }
        }
        if (j - 1) > meio {
            if array[j] < array[j - 1] {
                array.swap(j, j - 1);
            }
        }
    }
}