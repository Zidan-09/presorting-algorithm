use crate::utils::tipos::ArrayType;
use rand::Rng;

pub fn generate_test_array(size: usize, array_type: ArrayType) -> Vec<i32> {
    let mut rng = rand::thread_rng();

    match array_type {
        ArrayType::Inverted => (0..size).map(|i| (size - i) as i32).collect(),
        ArrayType::Zigzag => (0..size)
            .map(|i| if i % 2 == 0 { i as i32 } else { (size - i) as i32 })
            .collect(),
        ArrayType::Turtles => {
            let metade = size / 2;
            (0..size)
                .map(|i| if i < metade { (i + size) as i32 } else { (i % 10) as i32 })
                .collect()
        }
        ArrayType::Duplicates => (0..size).map(|_| rng.gen_range(0..3)).collect(),
        ArrayType::AlmostSorted => {
            let mut arr: Vec<i32> = (0..size).map(|i| i as i32).collect();
            let trocas = size / 100;
            for _ in 0..trocas {
                let i = rng.gen_range(0..size);
                let j = rng.gen_range(0..size);
                arr.swap(i, j);
            }
            arr
        }
        ArrayType::Random => (0..size).map(|_| rng.gen_range(1..=1_000_000)).collect(),
    }
}