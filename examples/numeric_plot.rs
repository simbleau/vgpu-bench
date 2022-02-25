use vgpu_bench::{
    BooleanPlotType, BooleanPlotter, Measurable, Measurements, NumericPlotType,
    NumericPlotter, Plotter,
};

pub fn main() {
    let mut data = Measurements::new();
    data.push(Measurable::Bool(true));
    data.push(Measurable::Bool(false));
    data.push(Measurable::Bool(false));
    let plotter = NumericPlotter {
        plot_type: NumericPlotType::Line,
        title: "Numeric Plot".to_string(),
        x_label: "Time".to_string(),
        y_label: "Magnitude".to_string(),
        sort_by: None,
        sort_ascending: true,
        show_stats: true,
        show_stats_table: true,
    };
    plotter.show_plot(&data).unwrap();
}
