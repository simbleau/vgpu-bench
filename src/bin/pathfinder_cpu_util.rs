#[allow(unused_imports)]
use naive_renderer::NaiveRenderer;
use pathfinder_vgpu_glue::PathfinderRenderer;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::env;
use std::path::PathBuf;
use vgpu_bench::macros::measurement;
use vgpu_bench::monitors::CpuUtilizationMonitor;
use vgpu_bench::prelude::*;

#[measurement]
struct RenderTime {
    filename: String,
    frame: usize,
    time_ns: u128,
}
pub struct PathfinderImpl {
    pathfinder: PathfinderRenderer,
    asset: PathBuf,
}

impl PathfinderImpl {
    pub fn new<P: Into<PathBuf>>(asset: P) -> Self {
        PathfinderImpl {
            pathfinder: PathfinderRenderer::new(),
            asset: asset.into(),
        }
    }
}

impl Renderer for PathfinderImpl {
    fn init(&mut self) -> renderer::Result<()> {
        self.pathfinder.init(self.asset.clone());
        Ok(())
    }

    fn stage(&mut self, _svg: &SVGDocument) -> renderer::Result<()> {
        Ok(())
    }

    fn render(&mut self, frames: usize) -> renderer::Result<RenderTimeResult> {
        let pathfinder = &mut self.pathfinder;
        let result = pathfinder.render(frames);
        let rt = RenderTimeResult {
            frame_times: result,
        };
        Ok(rt)
    }
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let bm_fn = move |_files| {
        let files: Vec<PathBuf> = _files;
        println!("{files:?}");
        // Benchmarking
        for file in files.iter() {
            let f = SVGFile::from(file);
            let d = SVGDocument::try_from(f).unwrap();
            for _ in 0..1 {
                let prev_level = log::max_level();
                log::set_max_level(log::LevelFilter::Off);
                let mut renderer = Box::new(PathfinderImpl::new(file));
                renderer.init().unwrap();
                renderer.stage(&d).unwrap();
                renderer.render(20).unwrap();
                drop(renderer);
                log::set_max_level(prev_level);
            }
        }
        Ok(Measurements::<RenderTime>::new())
    };

    let args: Vec<_> = env::args().collect();
    let file = PathBuf::from(args[1].to_owned());
    let files = vec![file];
    let bm_fn = BenchmarkFn::new(move || bm_fn(files));
    let mut bm_ = Benchmark::from(bm_fn).monitor(CpuUtilizationMonitor {
        name: "cpu_util",
        frequency: MonitorFrequency::Hertz(1),
    });
    let bundle1 = bm_.run(&DriverOptions::default()).unwrap();
    bundle1.write("output").unwrap();

    Ok(())
}
