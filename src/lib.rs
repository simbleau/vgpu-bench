#![feature(fn_traits)]

extern crate rendering_util;
extern crate svg_generator;
extern crate tessellation_util;

mod error;
pub use error::Result;

pub mod benchmarks;
pub mod driver;
pub mod util;
// TODO: Option builder for analysis.
