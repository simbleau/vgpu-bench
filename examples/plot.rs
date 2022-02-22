use vgpu_bench::{
    BooleanPlotType, BooleanPlotter, Measurable, Measurements, Plotter,
};

pub fn main() {
    let mut data = Measurements::new();
    data.push(Measurable::Bool(true));
    data.push(Measurable::Bool(false));
    data.push(Measurable::Bool(false));
    let plotter = BooleanPlotter {
        plot_type: BooleanPlotType::Pie,
        title: "True vs. False".to_string(),
        true_label: "True".to_string(),
        false_label: "False".to_string(),
    };
    plotter.show_plot(&data).unwrap();
}
