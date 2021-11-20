#![feature(vec_into_raw_parts)]
#![feature(c_size_t)]
#![feature(fn_traits)]

extern crate libloading;

mod error;
pub use error::{CRendererError, RendererError, Result};

pub mod artifacts;
pub mod c;
pub mod rust;
pub mod targets;
