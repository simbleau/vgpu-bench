use log::{debug, error, info, trace};
use std::path::PathBuf;
use vgpu_bench::benchmarks::tessellation::primitive_timing::PrimitiveTessellationTimingOptions;
use vgpu_bench::driver::dictionary::*;
use vgpu_bench::driver::DriverOptions;
use vgpu_bench::{benchmarks, util};

pub fn bench_tessellation_primitives(options: &DriverOptions) {
    let output_path = options.output_dir.join(
        [
            DATA_DIR_NAME,
            PRIMITIVES_DIR_NAME,
            SVG_DIR_NAME,
            "tessellation.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives::default();
    let trials = 1;
    let options = PrimitiveTessellationTimingOptions::new()
        .writer(writer)
        .backend(backend)
        .primitives(primitives)
        .primitive_count(10)
        .primitives_counts((100..=500).step_by(100 as usize))
        .trials(trials);
    debug!("Options: {:?}", options);

    trace!("Commencing SVG primitive tessellation time capture");
    match benchmarks::tessellation::primitive_timing::write_tessellation_times(
        options,
    ) {
        Ok(_) => {
            trace!("Completed SVG primitive tessellation time capture");
            info!(
                "Completed SVG primitive tessellation time capture. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}
