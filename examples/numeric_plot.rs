use vgpu_bench::{
    Measurable, Measurements, NumericPlotType, NumericPlotter, Plotter,
};

pub fn main() {
    // Loading of data
    let mut data = Measurements::new();
    data.insert("x", Measurable::Integer(1));
    data.insert("x", Measurable::Integer(2));
    data.insert("x", Measurable::Integer(3));
    data.insert("x", Measurable::Integer(3));
    data.insert("x", Measurable::Integer(4));

    data.insert("y", Measurable::Integer(5));
    data.insert("y", Measurable::Integer(6));
    data.insert("y", Measurable::Integer(8));
    data.insert("y", Measurable::Integer(5));
    data.insert("y", Measurable::Integer(4));

    // Plot settings
    let plotter = NumericPlotter {
        plot_type: NumericPlotType::Line,
        title: "Numeric Plot".to_string(),
        x_column: "x".to_string(),
        y_column: "y".to_string(),
        x_label: "Time".to_string(),
        y_label: "Magnitude".to_string(),
        plot_by: None,
        sort_by: None,
        sort_ascending: true,
        show_stats: true,
        show_stats_table: false,
    };

    // Plot
    plotter.show_plot(&data).unwrap();
}
