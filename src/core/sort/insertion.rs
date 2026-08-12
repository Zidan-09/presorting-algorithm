pub fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let valor_atual = array[i];
        let mut j = i;

        while j > 0 && array[j - 1] > valor_atual {
            array[j] = array[j - 1];
            j -= 1;
        }
        array[j] = valor_atual;
    }
}