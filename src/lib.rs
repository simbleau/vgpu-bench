#![feature(trait_alias)]
#![feature(iter_intersperse)]
#![feature(div_duration)]
#![feature(duration_consts_float)]

pub extern crate nvtx;

pub use anyhow::Error;
pub use anyhow::Result;

pub mod macros;
pub mod models;
pub mod monitors;
pub mod util;

pub mod prelude {
    pub extern crate nvtx;
    pub use crate::macros::*;
    pub use crate::models::*;
    pub use anyhow::Error;
    pub use anyhow::Result;
}
