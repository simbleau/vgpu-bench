mod error;
pub use error::{RenderingError, Result};

pub mod naive_primitive_rendering;

mod time_naive_svg_file_rendering;
pub use time_naive_svg_file_rendering::TimeNaiveSVGFileRendering;
