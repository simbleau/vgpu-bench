mod svg_results;
pub use svg_results::{SVGFlatRenderTimeResult, SVGProfileResult, SVGTessellationTimeResult};

mod primitive_results;
pub use primitive_results::PrimitiveTimeResult;

mod tessellation_results;
pub use tessellation_results::{TessellationData, TessellationProfile, TessellationTimeResult};

mod rendering_results;
pub use rendering_results::FlatRenderTimeResult;
