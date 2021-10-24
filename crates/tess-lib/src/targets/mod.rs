mod svg_target;
use crate::{
    artifacts::{TessellationProfile, TessellationTimeResult},
    backends::Tessellator,
};
pub use svg_target::SVGTarget;

pub trait TessellationTarget {
    fn get_data(
        &self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationProfile, Box<dyn std::error::Error>>;

    fn time(
        &mut self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationTimeResult, Box<dyn std::error::Error>>;
}
