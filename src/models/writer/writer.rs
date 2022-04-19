use std::path::Path;

use crate::models::{Measurable, Measurements};
use crate::Result;

pub trait Writer {
    fn append<T, P>(&self, data: &Measurements<T>, path: P) -> Result<()>
    where
        T: Measurable,
        P: AsRef<Path>;

    fn write<T, P>(&self, data: &Measurements<T>, path: P) -> Result<()>
    where
        T: Measurable,
        P: AsRef<Path>;
}
