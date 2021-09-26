mod lyon_tessellator;
pub use lyon_tessellator::LyonTessellator;

use crate::Tessellator;

pub fn backends() -> Vec<Box<dyn Tessellator>> {
    let mut tessellators: Vec<Box<dyn Tessellator>> = vec![];
    tessellators.push(Box::new(LyonTessellator::new()));
    tessellators
}
