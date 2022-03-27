#[allow(unused_imports)]
use naive_renderer::NaiveRenderer;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use usvg::ScreenSize;
use vgpu_bench::macros::measurement;
use vgpu_bench::prelude::*;

#[measurement]
struct RenderTime {
    filename: String,
    frame: usize,
    time_ns: u128,
}

struct Resvg {
    opt: usvg::Options,
    svg_data: Vec<u8>,
    rtree: Option<usvg::Tree>,
    pixmap_size: Option<ScreenSize>,
}
impl Resvg {
    fn new() -> Self {
        Resvg {
            opt: usvg::Options::default(),
            svg_data: Vec::new(),
            rtree: None,
            pixmap_size: None,
        }
    }
}

impl Renderer for Resvg {
    fn init(&mut self) -> renderer::Result<()> {
        self.opt = usvg::Options::default();
        self.opt.fontdb.load_system_fonts();
        self.svg_data = Vec::new();
        self.rtree = None;
        self.pixmap_size = None;

        Ok(())
    }

    fn stage(&mut self, svg: &SVGDocument) -> renderer::Result<()> {
        let data: Vec<u8> = svg.content().into();
        self.svg_data = data;
        self.rtree = Some(
            usvg::Tree::from_data(&self.svg_data, &self.opt.to_ref()).unwrap(),
        );
        self.pixmap_size = Some(
            self.rtree
                .as_ref()
                .unwrap()
                .svg_node()
                .size
                .to_screen_size(),
        );

        Ok(())
    }

    fn render(
        &mut self,
        frames: usize,
    ) -> renderer::Result<renderer::artifacts::RenderTimeResult> {
        let pixmap_size = self.pixmap_size.unwrap();
        let mut pixmap =
            tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
                .unwrap();

        let mut frame_times: Vec<Duration> = Vec::new();
        for _ in 0..frames {
            let t1 = Instant::now();
            resvg::render(
                &self.rtree.as_ref().unwrap(),
                usvg::FitTo::Original,
                tiny_skia::Transform::default(),
                pixmap.as_mut(),
            )
            .unwrap();
            let dur = Instant::now().duration_since(t1);
            frame_times.push(dur);
            println!("finished in {dur:?}");
        }

        let x = RenderTimeResult { frame_times };

        Ok(x)
    }
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let bm_fn = move |_files| {
        let files: Vec<PathBuf> = _files;
        println!("{files:?}");
        let mut measurements: Measurements<RenderTime> = Measurements::new();
        // Benchmarking
        for file in files.iter() {
            let f = SVGFile::from(file);
            let d = SVGDocument::try_from(f).unwrap();
            for _ in 0..1 {
                let prev_level = log::max_level();
                log::set_max_level(log::LevelFilter::Off);
                let mut renderer = Box::new(Resvg::new());
                renderer.init().unwrap();
                renderer.stage(&d).unwrap();
                let results = renderer.render(50).unwrap().frame_times.clone();
                drop(renderer);
                log::set_max_level(prev_level);

                for frame in 0..results.len() {
                    measurements.push(RenderTime {
                        filename: file
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string(),
                        frame: frame + 1,
                        time_ns: results.get(frame).unwrap().as_nanos(),
                    })
                }
            }
        }
        Ok(measurements)
    };

    let files = vgpu_bench::util::io::get_files_with_extension(
        "assets/svg/examples",
        false,
        "svg",
    );
    let bm_fn = BenchmarkFn::new(move || bm_fn(files));
    let mut bm_ = Benchmark::from(bm_fn);
    let bundle1 = bm_.run(&DriverOptions::default()).unwrap();
    bundle1.write("output").unwrap();

    Ok(())
}
