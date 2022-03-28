#[allow(unused_imports)]
use naive_renderer::NaiveRenderer;
use pathfinder_vgpu_glue::PathfinderRenderer;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::env;
use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use usvg::ScreenSize;
use vgpu_bench::macros::measurement;
use vgpu_bench::prelude::*;

#[measurement]
struct FirstFrameTime {
    filename: String,
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

    let bm_fn = move |_name, _renderer, _file| {
        let file: PathBuf = _file;
        let mut renderer: Box<dyn Renderer> = _renderer;
        println!("{file:?}");
        let mut measurements: Measurements<FirstFrameTime> =
            Measurements::new();
        // Benchmarking
        let f = SVGFile::from(file.clone());
        let d = SVGDocument::try_from(f).unwrap();
        let prev_level = log::max_level();
        log::set_max_level(log::LevelFilter::Off);
        renderer.init().unwrap();
        let start = Instant::now();
        renderer.stage(&d).unwrap();
        let dur = Instant::now().duration_since(start);
        let results = renderer.render(1).unwrap().frame_times.clone();
        drop(renderer);
        log::set_max_level(prev_level);

        for frame in 0..results.len() {
            measurements.push(FirstFrameTime {
                filename: file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                time_ns: (results.get(frame).unwrap().add(dur)).as_nanos(),
            })
        }

        Ok(measurements)
    };

    let args: Vec<_> = env::args().collect();
    let file = PathBuf::from(args[1].to_owned());
    let renderers: Vec<(&'static str, Box<dyn Renderer>)> = vec![
        ("Resvg", Box::new(Resvg::new())),
        ("Pathfinder", Box::new(PathfinderImpl::new(file.clone()))),
        ("Render-Kit", Box::new(NaiveRenderer::new())),
    ];

    for (r_name, r) in renderers {
        let f_copy = file.clone();
        let bm_fn = BenchmarkFn::new(move || bm_fn(r_name, r, f_copy));
        let mut bm_ = Benchmark::from(bm_fn);
        let bundle1 = bm_.run(&DriverOptions::default()).unwrap();
        bundle1.write(format!("output/{r_name}")).unwrap();
    }

    Ok(())
}
