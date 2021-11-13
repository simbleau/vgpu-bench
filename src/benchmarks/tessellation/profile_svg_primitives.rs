use crate::{
    benchmarks::{Benchmark, BenchmarkFn},
    log_assert, util, Result,
};
use log::{debug, info, trace, warn};
use svg_generator::Primitive;
use std::path::PathBuf;
use tessellation_util::{backends::Tessellator, benching::output::SVGPrimitiveProfile};

#[derive(Debug)]
pub struct ProfileSVGPrimitives {
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_counts: Vec<u32>,
    output: Option<&'static str>,
}

impl ProfileSVGPrimitives {
    pub fn new() -> Self {
        ProfileSVGPrimitives {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_counts: Vec::new(),
            output: None,
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

    pub fn primitives_counts<I>(mut self, primitive_counts: I) -> Self
    where
        I: IntoIterator<Item = u32>,
    {
        self.primitive_counts.extend(primitive_counts);
        self
    }

    pub fn to_file(mut self, path: &'static str) -> Self {
        self.output = Some(path);
        self
    }
}

impl Benchmark for ProfileSVGPrimitives {
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
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
        log_assert!(self.primitives.len() > 0, "no primitive were provided");
        log_assert!(
            self.primitive_counts.len() > 0,
            "no primitive counts were provided"
        );
        log_assert!(self.backends.len() > 0, "no backends were provided");

        // Write benchmark
        BenchmarkFn::from(move |options| {
            trace!("commencing SVG primitive profiling");
            debug!("options: {:?}", self);

            // Collect results
            let mut results: Vec<SVGPrimitiveProfile> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for primitive_count in &self.primitive_counts {
                    for primitive in &self.primitives {
                        let result =
                    tessellation_util::benching::profiling::get_primitive_profile(backend, 
                        *primitive, *primitive_count)?;
                        results.push(result);
                    }
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
                    "output SVG primitive profiling to '{}'",
                    &output_path.display()
                );
            }

            trace!("completed SVG primitive profiling");
            Ok(())
        })
    }
}
