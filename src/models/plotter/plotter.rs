use plotters::prelude::ChartContext;
use std::path::Path;

use crate::models::{Measurable, Measurements};
use crate::Result;

pub trait Plotter {
    fn plot<T, DB, CT>(
        &self,
        data: &Measurements<T>,
    ) -> Result<ChartContext<DB, CT>>
    where
        T: Measurable,
        DB: plotters::prelude::DrawingBackend,
        CT: plotters::prelude::CoordTranslate;

    fn save_plot<P, T>(&self, _data: &Measurements<T>, _path: P) -> Result<()>
    where
        P: AsRef<Path>,
        T: Measurable,
    {
        todo!("Save plot");
    }

    fn show_plot<T>(&self, _data: &Measurements<T>) -> Result<()>
    where
        T: Measurable,
    {
        todo!("Show plot");
    }
}
