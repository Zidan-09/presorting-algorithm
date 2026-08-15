pub mod core;
pub mod services;
pub mod utils;

use stats_alloc::StatsAlloc;
use std::alloc::System;

#[global_allocator]
pub static GLOBAL: StatsAlloc<System> = StatsAlloc::system();
