#![feature(vec_into_raw_parts)]
#![feature(c_size_t)]
#![feature(fn_traits)]

extern crate libloading;

mod error;
pub use error::{ExternalRendererError, RendererError, Result};

pub mod artifacts;
pub mod ffi;
pub mod targets;

mod renderer;
pub use renderer::Renderer;
