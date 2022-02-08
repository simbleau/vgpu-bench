use crate::benchmarks::rendering::output::FileRenderTime;
use crate::models::{Benchmark, BenchmarkBuilder, BenchmarkData, BenchmarkFn};
use crate::Result;
use crate::{log_assert, util};
use benchmark_macro_derive::BenchmarkData;
use erased_serde::Serialize;
use log::{debug, info, trace, warn};
use renderer::Renderer;
use std::path::PathBuf;

#[derive(Debug, BenchmarkData)]
pub struct TimeSVGFileRendering {
    renderer: Option<Box<dyn Renderer>>,
    assets: Vec<PathBuf>,
    frames: usize,
    csv_output: Option<&'static str>,
    plot_output: Option<&'static str>,
}

impl TimeSVGFileRendering {
    pub fn new() -> Self {
        TimeSVGFileRendering {
            renderer: None,
            assets: Vec::new(),
            frames: 0,
            csv_output: None,
            plot_output: None,
        }
    }

    pub fn renderer(mut self, renderer: Box<dyn Renderer>) -> Self {
        self.renderer = Some(renderer);
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

impl BenchmarkBuilder for TimeSVGFileRendering {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
        // Sanitize input assets
        let assets = util::files_with_extension(&self.assets, "svg");
        // Input check
        log_assert!(self.renderer.is_some(), "a renderer must be set");
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
        log_assert!(self.frames > 0, "frames must be greater than 0");

        // Bandaid fix: external loggers might use the same logger and affect
        // performance
        let prev_level = log::max_level();
        log::set_max_level(log::LevelFilter::Off);
        // Write benchmark
        let bfn = BenchmarkFn::from(move |options| {
            trace!("commencing file rendering frametime capture");
            debug!("options: {:?}", self);

            // Collect results
            let mut renderer = self.renderer.unwrap();
            let renderer = renderer.as_mut();
            let mut results: Vec<FileRenderTime> = Vec::new();
            for file_path in &assets {
                let result = rendering_util::benching::timing::time_svg(
                    renderer,
                    &util::path_to_svg(file_path),
                    self.frames,
                )?;
                for (frame, dur) in result.frame_times.iter().enumerate() {
                    results.push(FileRenderTime {
                        filename: file_path.display().to_string(),
                        frame,
                        frame_time: dur.as_nanos(),
                    })
                }
            }
            // Bandaid removal
            log::set_max_level(prev_level);

            // Write results
            if let Some(path) = self.csv_output {
                let path = options.benchmark_dir().join(path);
                let rows: Vec<Box<dyn Serialize>> = results
                    .into_iter()
                    .map(|x| -> Box<dyn Serialize> { Box::new(x) })
                    .collect();
                util::write_csv(&path, &rows)?;
                info!("output CSV data to '{}'", &path.display());
            }

            // Plot results
            if let Some(plot_output) = self.plot_output {
                let mut csv_path =
                    options.benchmark_dir().join(self.csv_output.unwrap());
                csv_path.set_extension("csv");

                let _proc_output = util::call_program(
                    "python3",
                    [
                        "tools/plotter/plot_frametimes_files.py",
                        csv_path.to_str().unwrap(),
                        options.benchmark_dir().to_str().unwrap(),
                        plot_output,
                    ],
                )?;
                info!(
                    "output plot to '{}'",
                    options.benchmark_dir().join(plot_output).display()
                );
            }

            trace!("completed SVG file rendering frametime capture");
            Ok(())
        });

        Ok(bfn)
    }
}
