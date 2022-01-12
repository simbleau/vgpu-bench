#![feature(trait_alias)]
#![feature(iter_intersperse)]

extern crate nvtx;
extern crate rendering_util;
extern crate svg_generator;
extern crate tessellation_util;

pub use anyhow::Result;
pub mod benchmarks;
pub mod driver;
pub mod util;

mod macros;
