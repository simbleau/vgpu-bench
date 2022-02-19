use crate::{Measurable, Plotter};

pub enum BooleanPlotType {
    Pie,
    Stepper,
}

pub struct BooleanPlotter {
    plot_type: BooleanPlotType,
    title: String,
    true_label: String,
    false_label: String,
}

impl BooleanPlotter {
    pub fn new(title: String) -> Self {
        BooleanPlotter {
            plot_type: BooleanPlotType::Stepper,
            title,
            true_label: "True".to_string(),
            false_label: "False".to_string(),
        }
    }
}

impl Plotter for BooleanPlotter {
    fn plot<P>(
        &self,
        data: &Vec<Box<Measurable>>,
        path: P,
    ) -> anyhow::Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        match self.plot_type {
            BooleanPlotType::Pie => unimplemented!(),
            BooleanPlotType::Stepper => todo!(),
        }
    }
}

fn plot_stepper() {
    //
}
