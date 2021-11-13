use crate::{
    benchmarks::{Benchmark, BenchmarkFn},
    log_assert, util, Result,
};
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use tessellation_util::{
    backends::Tessellator, benching::output::SVGFileProfile,
};

#[derive(Debug)]
pub struct ProfileSVGFiles {
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    output: Option<&'static str>,
}

impl ProfileSVGFiles {
    pub fn new() -> Self {
        ProfileSVGFiles {
            backends: Vec::new(),
            assets: Vec::new(),
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
}

impl Benchmark for ProfileSVGFiles {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
        // Sanitize input assets
        let assets = util::files_with_extension(&self.assets, "svg");
        // Input check
        if let Some(path) = self.output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{} is not a relative path",
                path
            );
        } else {
            warn!("no output path was provided; results will be dropped");
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
            if let Some(path) = self.output {
                let output_path = options.output_dir.join(path);
                let mut writer = util::csv_writer_relative(&output_path)?;
                for result in results {
                    writer.serialize(result)?;
                }
                writer.flush()?;
                info!(
                    "output SVG file profiling to '{}'",
                    &output_path.display()
                );
            }

            trace!("completed SVG file profiling");
            Ok(())
        })
    }
}
