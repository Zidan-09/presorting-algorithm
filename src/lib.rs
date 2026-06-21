pub mod tipos;
pub mod gerador;
pub mod algoritmos;
pub mod service;
pub mod servicebench;

use stats_alloc::StatsAlloc;
use std::alloc::System;

#[global_allocator]
pub static GLOBAL: StatsAlloc<System> = StatsAlloc::system();