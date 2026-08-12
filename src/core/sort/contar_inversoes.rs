pub fn contar_inversoes(array: &[i32]) -> u64 {
    let mut inversoes = 0u64;

    for i in 0..array.len() {
        for j in (i + 1)..array.len() {
            if array[i] > array[j] {
                inversoes += 1;
            }
        }
    }

    inversoes
}