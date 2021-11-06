#![feature(vec_into_raw_parts)]
#![feature(c_size_t)]
#![feature(fn_traits)]

extern crate libloading;

use artifacts::RenderTimeResult;
use targets::SVGDocument;

pub mod artifacts;
pub mod cpp_lib;
pub mod error;
pub mod targets;

use crate::error::Result;
pub trait Renderer {
    fn init(&mut self) -> Result<()>;

    fn stage(&mut self, svg: &SVGDocument) -> Result<()>;

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult>;
}
