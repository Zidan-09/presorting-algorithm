use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortType {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Selection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ArrayType {
    Random,
    Inverted,
    Zigzag,
    Turtles,
    Duplicates,
    AlmostSorted,
}