use crate::benchmarks::{Benchmark, BenchmarkBuilder};
use crate::driver::dictionary::*;
use crate::util;
use csv::Writer;
use log::{debug, info, trace, warn};
use rendering_util::benching::output::SVGNaiveRenderTime;
use rendering_util::benching::Result;
use std::path::PathBuf;
use tessellation_util::backends::Tessellator;

pub struct NaiveSVGRenderingBuilder<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    frames: usize,
    writer: Option<Writer<W>>,
}
impl<W> std::fmt::Debug for NaiveSVGRenderingBuilder<W>
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

impl<W> NaiveSVGRenderingBuilder<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        NaiveSVGRenderingBuilder {
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
}

impl<W> BenchmarkBuilder for NaiveSVGRenderingBuilder<W>
where
    W: std::io::Write + 'static,
{
    fn build(self) -> Benchmark {
        // Input check and sanitizing
        assert!(self.backends.len() > 0, "No backends were provided");
        assert!(self.assets.len() > 0, "No assets were found or provided");
        assert!(self.frames > 0, "Frames must be greater than 0");
        if self.writer.is_none() {
            warn!("No writer was provided")
        };

        // Write benchmark
        Benchmark::from(move |options| {
            trace!("Commencing naive SVG file rendering frametime capture");
            debug!("Options: {:?}", self);

            let output_path: PathBuf = options.output_dir.join(
                [
                    DATA_DIR_NAME,
                    EXAMPLES_DIR_NAME,
                    SVG_DIR_NAME,
                    "naive_frametimes.csv", // TODO make this an option
                ]
                .iter()
                .collect::<PathBuf>(),
            );

            // Bandaid fix: wgpu uses the same logger - Disable logging
            // temporarily
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            // Collect results
            let mut results: Vec<SVGNaiveRenderTime> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for file_path in &self.assets {
                    results.extend(
                        rendering_util::benching::timing::time_naive_svg(
                            backend,
                            file_path,
                            self.frames,
                        )?,
                    );
                }
            }
            // Bandaid removal
            log::set_max_level(prev_level);

            // Write results
            if let Some(mut writer) = self.writer {
                for result in results {
                    writer.serialize(result)?;
                }
                writer.flush()?;
            }

            // Return results
            Ok(Vec::new())
        })
    }
}

pub fn frametimes<W>(options: NaiveSVGRenderingBuilder<W>) -> Result<()>
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
