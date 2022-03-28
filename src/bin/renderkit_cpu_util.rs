use log::info;
#[allow(unused_imports)]
use naive_renderer::NaiveRenderer;
use naive_renderer::TriangleRenderer;
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;
use tessellation_util::backends::LyonTessellator;
use vgpu_bench::macros::measurement;
use vgpu_bench::monitors::CpuUtilizationMonitor;
use vgpu_bench::prelude::*;

#[measurement]
struct RenderTime {
    filename: String,
    frame: usize,
    time_ns: u128,
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let bm_fn = move |_files| {
        let files: Vec<PathBuf> = _files;
        println!("{files:?}");
        // Benchmarking
        for file in files.iter() {
            info!("Loading file...");
            nvtx::range_push("IO");
            let f = SVGFile::from(file);
            let d = SVGDocument::try_from(f).unwrap();
            nvtx::range_pop();

            info!("Initializing renderer...");
            nvtx::range_push("Startup");
            let mut r = TriangleRenderer::new();
            nvtx::range_pop();

            info!("Tessellating file...");
            nvtx::range_push("Tessellating");
            r.init_with_svg(&mut LyonTessellator::new(), &d).unwrap();
            nvtx::range_pop();

            info!("Starting renderer...");
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            nvtx::range_push("Renderering");
            r.run().unwrap();
            nvtx::range_pop();
            log::set_max_level(prev_level);

            info!("Finished!");
        }

        Ok(Measurements::<RenderTime>::new())
    };

    let files = vec![PathBuf::from("assets/svg/examples/København_512.svg")];
    let files = vec![PathBuf::from("C:\\Users\\sterc\\OneDrive\\Desktop\\Spencer_thesis\\vgpu-bench\\assets\\svg\\complex\\Kobenhavn_512.svg")];
    let bm_fn = BenchmarkFn::new(move || bm_fn(files));
    let mut bm_ = Benchmark::new(BenchmarkMetadata::new("Renderkit"), bm_fn)
        .monitor(CpuUtilizationMonitor {
            name: "cpu_util",
            frequency: MonitorFrequency::Hertz(1),
        });
    let bundle1 = bm_.run(&DriverOptions::default()).unwrap();
    bundle1.write("output").unwrap();

    Ok(())
}
