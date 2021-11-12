use csv::Writer;
use rendering_util::benching::output::SVGNaiveRenderTime;
use rendering_util::benching::Result;
use std::io::Write;
use std::path::PathBuf;
use tessellation_util::backends::Tessellator;

use crate::benchmarks::Benchmark;
use crate::driver::DriverOptions;

pub struct SVGNaiveRenderingOptions<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    frames: usize,
    writer: Option<Writer<W>>,
}
impl<W> std::fmt::Debug for SVGNaiveRenderingOptions<W>
where
    W: std::io::Write,
{
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "SVGNaiveRenderingOptions {{ backends: {:?}, assets: {:?}, frames: {:?} }}",
            self.backends, self.assets, self.frames
        )
    }
}

impl<W> SVGNaiveRenderingOptions<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        SVGNaiveRenderingOptions {
            backends: Vec::new(),
            assets: Vec::new(),
            frames: 0,
            writer: None,
        }
    }

    pub fn backend(mut self, backend: Box<dyn Tessellator>) -> Self {
        self.backends.push(backend);
        self
    }

    pub fn asset<P>(mut self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.assets.push(path.into());
        self
    }

    pub fn assets<I>(mut self, assets: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<PathBuf>,
    {
        self.assets.extend(assets.into_iter().map(|a| a.into()));
        self
    }

    pub fn writer(mut self, writer: Writer<W>) -> Self {
        self.writer = Some(writer);
        self
    }

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }

    pub fn build(self) -> Benchmark {
        let b = Benchmark::from(move |x| {
            let x = &self;
            Vec::new()
        });

        b
    }
}

pub fn frametimes<W>(options: SVGNaiveRenderingOptions<W>) -> Result<()>
where
    W: std::io::Write,
{
    // TODO remove this bandaid
    // Bandaid fix: wgpu uses the same logger - Disable logging temporarily
    let prev_level = log::max_level();
    log::set_max_level(log::LevelFilter::Off);
    // Collect results
    let mut results: Vec<SVGNaiveRenderTime> = Vec::new();
    for mut backend in options.backends {
        let backend: &mut dyn Tessellator = backend.as_mut();
        for file_path in &options.assets {
            results.extend(rendering_util::benching::timing::time_naive_svg(
                backend,
                file_path,
                options.frames,
            )?);
        }
    }
    // Bandaid removal
    log::set_max_level(prev_level);

    // Write results
    if let Some(mut writer) = options.writer {
        for result in results {
            writer.serialize(result)?;
        }
        writer.flush()?;
    }

    Ok(())
}
