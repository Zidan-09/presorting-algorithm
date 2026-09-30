use crate::utils::tipos::ArrayType;
use rand::seq::SliceRandom;
use rand::Rng;

/// Asset real v1 congelado (`dados_reais/real_bytes_v1.bin`): bytes crus
/// concatenados de 40 arquivos-fonte do repo no commit c3b0bbe
/// (ver `dados_reais/README.md`; SHA-256 registrado lá).
/// Carregado em tempo de compilação: idêntico em qualquer clone.
pub const REAL_DATA: &[u8] = include_bytes!("../../dados_reais/real_bytes_v1.bin");
/// Tamanho nativo do asset real (n solicitado maior que isso é excluído
/// da matriz — sem tiling; ver `celula_planejada` no bench).
pub const REAL_NATIVE_LEN: usize = REAL_DATA.len();

pub fn generate_test_array(size: usize, array_type: ArrayType, rng: &mut impl Rng) -> Vec<i32> {
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
                let mut j = rng.gen_range(0..size);
                while j == i {
                    j = rng.gen_range(0..size);
                }
                arr.swap(i, j);
            }
            arr
        }
        ArrayType::Random => {
            let mut arr: Vec<i32> = (0..size).map(|i| i as i32).collect();
            arr.shuffle(rng);
            arr
        }
        ArrayType::Sawtooth => (0..size).map(|i| (i % 16) as i32).collect(),
        ArrayType::OrganPipe => (0..size)
            .map(|i| {
                if i < size / 2 {
                    i as i32
                } else {
                    (size - 1 - i) as i32
                }
            })
            .collect(),
        ArrayType::FewRuns => {
            let mut arr: Vec<i32> = (0..size).map(|i| i as i32).collect();
            // 4 blocos disjuntos revertidos nos quartis -> 4 runs descendentes.
            for b in 0..4 {
                let s = b * size / 4;
                let e = (s + size / 8).min(size);
                arr[s..e].reverse();
            }
            arr
        }
        ArrayType::Real => {
            assert!(
                size <= REAL_NATIVE_LEN,
                "Real: tamanho {} excede o nativo {} (célula não planejada)",
                size,
                REAL_NATIVE_LEN
            );
            REAL_DATA[..size].iter().map(|&b| b as i32).collect()
        }
    }
}
