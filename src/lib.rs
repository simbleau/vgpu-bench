#![feature(trait_alias)]
#![feature(iter_intersperse)]
#![feature(div_duration)]

extern crate nvtx;
extern crate rendering_util;
extern crate svg_generator;
extern crate tessellation_util;

pub use anyhow::Result;
pub mod benchmarks;
pub mod driver;
pub mod models;
pub mod monitors;
pub mod util;

mod macros;
