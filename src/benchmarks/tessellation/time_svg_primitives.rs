use crate::{
    log_assert,
    models::{Benchmark, BenchmarkBuilder, BenchmarkData, BenchmarkFn},
    util, Result,
};
use benchmark_macro_derive::BenchmarkData;
use erased_serde::Serialize;
use log::{debug, info, trace, warn};
use std::path::PathBuf;
use svg_generator::Primitive;
use tessellation_util::{
    backends::Tessellator, benching::output::PrimitiveTessellationTime,
};

#[derive(Debug, BenchmarkData)]
pub struct TimeSVGPrimitiveTessellation {
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_counts: Vec<u32>,
    trials: u32,
    csv_output: Option<&'static str>,
    plot_output: Option<&'static str>,
}

impl TimeSVGPrimitiveTessellation {
    pub fn new() -> Self {
        TimeSVGPrimitiveTessellation {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_counts: Vec::new(),
            trials: 0,
            csv_output: None,
            plot_output: None,
        }
    }

    pub fn backend(mut self, backend: Box<dyn Tessellator>) -> Self {
        self.backends.push(backend);
        self
    }

    pub fn primitive(mut self, primitive: Primitive) -> Self {
        self.primitives.push(primitive);
        self
    }

    pub fn primitives<I>(mut self, primitives: I) -> Self
    where
        I: IntoIterator<Item = Primitive>,
    {
        self.primitives.extend(primitives);
        self
    }

    pub fn primitive_count(mut self, primitive_count: u32) -> Self {
        self.primitive_counts.push(primitive_count);
        self
    }

    pub fn trials(mut self, trials: u32) -> Self {
        self.trials = trials;
        self
    }

    pub fn primitives_counts<I>(mut self, primitive_counts: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.primitive_counts.extend(primitive_counts);
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

impl BenchmarkBuilder for TimeSVGPrimitiveTessellation {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
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
        log_assert!(self.primitives.len() > 0, "no primitive were provided");
        log_assert!(
            self.primitive_counts.len() > 0,
            "no primitive counts were provided"
        );
        log_assert!(self.trials > 0, "trials must be greater than 0");
        log_assert!(self.backends.len() > 0, "no backends were provided");

        // Write benchmark
        let bfn = BenchmarkFn::from(move |options| {
            trace!("commencing SVG primitive tessellation timing");
            debug!("options: {:?}", self);

            // Collect results
            let mut results: Vec<PrimitiveTessellationTime> = Vec::new();
            for mut backend in self.backends {
                for primitive_count in &self.primitive_counts {
                    for primitive in &self.primitives {
                        let result =
                    tessellation_util::benching::tessellating::time_primitive(
                        backend.as_mut(),
                        primitive.clone(),
                        primitive_count.clone(),
                        self.trials,
                    );
                        results.extend(result?);
                    }
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
                        "tools/plotter/plot_tessellation_time_svg_primitives.py",
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

            trace!("completed SVG primitive tessellation timing");
            Ok(())
        });

        Ok(bfn)
    }
}
