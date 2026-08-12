pub fn bubble_sort(array: &mut [i32]) {
    let n = array.len();
    if n < 2 { return; }

    for i in 0..n {
        let mut trocou = false;
        for j in 0..n - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                trocou = true;
            }
        }
        if !trocou {
            break;
        }
    }
}