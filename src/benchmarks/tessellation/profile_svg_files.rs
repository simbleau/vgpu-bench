use crate::{
    benchmarks::{Benchmark, BenchmarkBuilder, BenchmarkData, BenchmarkFn},
    log_assert, util, Result,
};
use benchmark_macro_derive::BenchmarkData;
use erased_serde::Serialize;
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use tessellation_util::{
    backends::Tessellator, benching::output::SVGFileProfile,
};

#[derive(Debug, BenchmarkData)]
pub struct ProfileSVGFiles {
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    csv_output: Option<&'static str>,
    plot_output: Option<&'static str>,
}

impl ProfileSVGFiles {
    pub fn new() -> Self {
        ProfileSVGFiles {
            backends: Vec::new(),
            assets: Vec::new(),
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

    pub fn to_csv(mut self, path: &'static str) -> Self {
        self.csv_output = Some(path);
        self
    }

    pub fn to_plot(mut self, path: &'static str) -> Self {
        self.plot_output = Some(path);
        self
    }
}

impl BenchmarkBuilder for ProfileSVGFiles {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
        // Sanitize input assets
        let assets = util::files_with_extension(&self.assets, "svg");
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

        // Write benchmark
        BenchmarkFn::from(move |options| {
            trace!("commencing SVG file profiling");
            debug!("options: {:?}", self);

            // Collect results
            let mut results: Vec<SVGFileProfile> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut(); // Coerce & shadow
                for file_path in &assets {
                    results.push(
                        tessellation_util::benching::profiling::get_file_profile(
                            backend, file_path,
                        )?,
                    );
                }
            }

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
                        "tools/plotter/plot_profile_svg_files.py",
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

            trace!("completed SVG file profiling");
            Ok(())
        })
    }
}
