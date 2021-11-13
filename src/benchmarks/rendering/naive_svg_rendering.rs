use crate::benchmarks::{Benchmark, BenchmarkFn};
use crate::driver::dictionary::*;
use crate::util;
use crate::Result;
use log::{debug, info, trace, warn};
use rendering_util::benching::output::SVGNaiveRenderTime;
use std::ffi::OsStr;
use std::path::PathBuf;
use tessellation_util::backends::Tessellator;

#[derive(Debug)]
pub struct TimeNaiveSVGFileRendering {
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    frames: usize,
    output: Option<&'static str>,
}

impl TimeNaiveSVGFileRendering {
    pub fn new() -> Self {
        TimeNaiveSVGFileRendering {
            backends: Vec::new(),
            assets: Vec::new(),
            frames: 0,
            output: None,
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
        self.assets
            .extend(assets.into_iter().map(|path| path.into()));
        self
    }

    pub fn to_file(mut self, path: &'static str) -> Self {
        self.output = Some(path);
        self
    }

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }
}

impl Benchmark for TimeNaiveSVGFileRendering {
    fn build(self: Box<Self>) -> BenchmarkFn {
        // Sanitize assets
        let assets = self
            .assets
            .iter()
            .filter_map(|pb| {
                if pb.exists()
                    && pb.is_file()
                    && pb.extension() == Some(OsStr::new("svg"))
                {
                    Some(pb.to_owned())
                } else {
                    warn!(
                        "'{}' is not a .svg file; file dropped",
                        pb.display()
                    );
                    None
                }
            })
            .collect::<Vec<PathBuf>>();
        // Input check
        assert!(assets.len() > 0, "no assets were found or provided");
        assert!(self.backends.len() > 0, "no backends were provided");
        assert!(self.frames > 0, "frames must be greater than 0");
        if self.output.is_none() {
            warn!("no output path was provided; results will be dropped")
        };

        // Write benchmark
        BenchmarkFn::from(move |options| {
            trace!("commencing naive SVG file rendering frametime capture");
            debug!("options: {:?}", self);

            // Bandaid fix: wgpu uses the same logger - Disable logging
            // temporarily
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            // Collect results
            let mut results: Vec<SVGNaiveRenderTime> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for file_path in &assets {
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
            if let Some(output) = self.output {
                let output_path: PathBuf = options.output_dir.join(
                    [DATA_DIR_NAME, EXAMPLES_DIR_NAME, SVG_DIR_NAME, output]
                        .iter()
                        .collect::<PathBuf>(),
                );
                let mut writer = util::csv_writer(output_path.to_owned())?;
                for result in results {
                    writer.serialize(result)?;
                }
                writer.flush()?;
                info!(
                    "output naive SVG file rendering frametime capture to '{}'",
                    output_path.display()
                );
            }

            trace!("completed naive SVG file rendering frametime capture");
            Ok(())
        })
    }
}
