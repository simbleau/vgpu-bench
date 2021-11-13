use crate::benchmarks::{Benchmark, BenchmarkFn};
use crate::driver::dictionary::*;
use crate::util;
use log::{debug, info, trace, warn};
use rendering_util::benching::output::PrimitiveNaiveRenderTime;
use std::path::PathBuf;
use svg_generator::Primitive;
use tessellation_util::backends::Tessellator;

#[derive(Debug)]
pub struct TimeNaiveSVGPrimitiveRendering {
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_count: u32,
    frames: usize,
    output: Option<&'static str>,
}

impl TimeNaiveSVGPrimitiveRendering {
    pub fn new() -> Self {
        TimeNaiveSVGPrimitiveRendering {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_count: 0,
            frames: 0,
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
        self.primitive_count = primitive_count;
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

impl Benchmark for TimeNaiveSVGPrimitiveRendering {
    fn build(self: Box<Self>) -> BenchmarkFn {
        // Input check
        assert!(self.backends.len() > 0, "no backends were provided");
        assert!(self.primitives.len() > 0, "no primitive were provided");
        assert!(
            self.primitive_count > 0,
            "primitive count must be greater than 0"
        );
        assert!(self.frames > 0, "frames must be greater than 0");
        if self.output.is_none() {
            warn!("no output path was provided; results will be dropped")
        };

        // Write benchmark
        BenchmarkFn::from(move |options| {
            trace!(
                "commencing naive SVG primitive rendering frametime capture"
            );
            debug!("options: {:?}", self);

            // Bandaid fix: wgpu uses the same logger - Disable logging
            // temporarily
            let prev_level = log::max_level();
            log::set_max_level(log::LevelFilter::Off);
            // Collect results
            let mut results: Vec<PrimitiveNaiveRenderTime> = Vec::new();
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for primitive in &self.primitives {
                    results.extend(
                        rendering_util::benching::timing::time_naive_primitive(
                            backend,
                            primitive.to_owned(),
                            self.primitive_count,
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
                    "output naive SVG primitive rendering frametime capture to '{}'",
                    output_path.display()
                );
            }

            trace!("completed naive SVG primitive rendering frametime capture");
            Ok(())
        })
    }
}
