use crate::{Measurable, Result};
use std::path::Path;

pub trait Plotter {
    fn plot<P>(&self, data: &Vec<Box<Measurable>>, path: P) -> Result<()>
    where
        P: AsRef<Path>;
}
