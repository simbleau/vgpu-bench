use crate::models::{Benchmark, BenchmarkFn, BenchmarkMetadata};
use crate::Result;
use crate::{log_assert, util};
use log::{debug, info, trace, warn};
use rendering_util::benching::output::NaiveSVGFileRenderTime;
use std::path::PathBuf;
use tessellation_util::backends::Tessellator;

#[derive(Debug)]
pub struct TimeNaiveSVGFileRendering {
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    frames: usize,
    csv_output: Option<&'static str>,
    plot_output: Option<&'static str>,
}

impl TimeNaiveSVGFileRendering {
    pub fn new() -> Self {
        TimeNaiveSVGFileRendering {
            backends: Vec::new(),
            assets: Vec::new(),
            frames: 0,
            csv_output: None,
            plot_output: None,
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

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }

    pub fn to_csv(mut self, path: &'static str) -> Self {
        self.csv_output = Some(path);
        self
    }

    pub fn to_plot(mut self, path: &'static str) -> Self {
        self.plot_output = Some(path);
        self
    }
}

pub const DEFAULT_METADATA: BenchmarkMetadata = BenchmarkMetadata {
    name: "Time Naive SVG File Rendering",
};
impl TryFrom<TimeNaiveSVGFileRendering> for Benchmark {
    type Error = anyhow::Error;

    fn try_from(value: TimeNaiveSVGFileRendering) -> Result<Self, Self::Error> {
        Ok(Benchmark::new(DEFAULT_METADATA, value.build()?))
    }
}

impl TimeNaiveSVGFileRendering {
    pub fn build(self) -> Result<BenchmarkFn> {
        // Sanitize input assets
        let assets = util::io::files_with_extension(&self.assets, "svg");
        // Input check
        if let Some(path) = self.csv_output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{} is not a relative path",
                path
            );
        } else {
            warn!("no output path was provided; results will be dropped");
        }
        if let Some(path) = self.plot_output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{} is not a relative path",
                path
            );
            log_assert!(
                self.csv_output.is_some(),
                "you cannot save a plot without an output path set"
            )
        }
        log_assert!(assets.len() > 0, "no assets were found or provided");
        log_assert!(self.backends.len() > 0, "no backends were provided");
        log_assert!(self.frames > 0, "frames must be greater than 0");

        // Write benchmark
        let bfn = BenchmarkFn::from(move |options| {
            trace!("commencing naive SVG file rendering frametime capture");
            debug!("options: {:?}", self);

            // Bandaid fix: wgpu uses the same logger - Disable logging
            // temporarily
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            // Collect results
            let mut results: Vec<NaiveSVGFileRenderTime> = Vec::new();
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
            if let Some(path) = self.csv_output {
                let path = options.output_dir().join(path);
                let rows = util::convert::to_serializable(results);
                util::io::write_csv(&path, &rows)?;
                info!("output CSV data to '{}'", &path.display());
            }

            // Plot results
            if let Some(plot_output) = self.plot_output {
                let mut csv_path =
                    options.output_dir().join(self.csv_output.unwrap());
                csv_path.set_extension("csv");

                let _proc_output = util::exec::call_program(
                    "python3",
                    [
                        "tools/plotter/plot_naive_frametimes_files.py",
                        csv_path.to_str().unwrap(),
                        options.output_dir().to_str().unwrap(),
                        plot_output,
                    ],
                )?;
                info!(
                    "output plot to '{}'",
                    options.output_dir().join(plot_output).display()
                );
            }

            trace!("completed naive SVG file rendering frametime capture");
            Ok(())
        });

        Ok(bfn)
    }
}
