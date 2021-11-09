#![feature(fn_traits)]

extern crate rendering;
extern crate svg_generator;
extern crate tessellation;

mod error;
pub use error::Result;

pub mod benchmarks;
pub mod driver;
pub mod util;
// TODO: Option builder for analysis.
