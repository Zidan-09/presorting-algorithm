use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Selection,
    /// Baseline competitiva: `slice::sort_unstable` do std (F1, docs/plan2.md).
    StdUnstable,
    /// Baseline competitiva: `slice::sort` do std (F1, docs/plan2.md).
    StdStable,
    /// Baseline O(n)+insertion: se estritamente decrescente, reverte e aplica
    /// insertion; senão, insertion direto (F1, docs/plan2.md).
    DescReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ArrayType {
    Random,
    Inverted,
    Zigzag,
    Turtles,
    Duplicates,
    AlmostSorted,
    /// Dente-de-serra: `a[i] = i % 16` (F1, docs/plan2.md).
    Sawtooth,
    /// Tubo de órgão: sobe até o meio e desce (F1, docs/plan2.md).
    OrganPipe,
    /// 4 runs descendentes em blocos nos quartis (F1, docs/plan2.md).
    FewRuns,
    /// Bytes reais congelados (`dados_reais/real_bytes_v1.bin`, F1).
    Real,
}