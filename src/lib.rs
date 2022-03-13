#![feature(trait_alias)]
#![feature(iter_intersperse)]
#![feature(div_duration)]
#![feature(duration_consts_float)]

extern crate nvtx;
extern crate rendering_util;
extern crate svg_generator;
extern crate tessellation_util;

pub use anyhow::Result;

pub mod macros;
pub mod monitors;
pub mod util;

mod models;
pub use models::*;
