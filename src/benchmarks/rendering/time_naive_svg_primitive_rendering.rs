use crate::benchmarks::{Benchmark, BenchmarkFn};
use crate::Result;
use crate::{log_assert, util};
use erased_serde::Serialize;
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
    fn build(self: Box<Self>) -> Result<BenchmarkFn> {
        // Input check
        if let Some(path) = self.output {
            log_assert!(
                PathBuf::from(path).is_relative(),
                "{path} is not a relative path"
            );
        } else {
            warn!("no output path was provided; results will be dropped");
        }
        log_assert!(self.backends.len() > 0, "no backends were provided");
        log_assert!(self.primitives.len() > 0, "no primitive were provided");
        log_assert!(
            self.primitive_count > 0,
            "primitive count must be greater than 0"
        );
        log_assert!(self.frames > 0, "frames must be greater than 0");

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
            if let Some(path) = self.output {
                let path = options.output_dir.join(path);
                let rows: Vec<Box<dyn Serialize>> = results
                    .into_iter()
                    .map(|x| -> Box<dyn Serialize> { Box::new(x) })
                    .collect();
                util::write_csv(&path, &rows)?;
                info!("output CSV data to '{}'", &path.display());
            }

            trace!("completed naive SVG primitive rendering frametime capture");
            Ok(())
        })
    }
}
