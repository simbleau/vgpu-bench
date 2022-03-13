pub struct TimeFileRendering {
    renderer: Option<dyn Renderer>,
    assets: Vec<PathBuf>,
    frames: usize,
}

impl TimeFileRendering {
    pub fn new() -> Self {
        TimeFileRendering {
            renderer: None,
            assets: Vec::new(),
            frames: 0,
        }
    }

    pub fn renderer(mut self, backend: Box<dyn Renderer>) -> Self {
        self.renderer = Some(backend.into_inner());
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
            let mut measurements = Measurements2D::new(&[
                "tessellator",
                "filename",
                "frame",
                "frame_time",
                "triangles",
            ]);
            for mut backend in self.backends {
                let backend: &mut dyn Tessellator = backend.as_mut();
                for file_path in &assets {
                    let results =
                        rendering_util::benching::timing::time_naive_svg(
                            backend,
                            file_path,
                            self.frames,
                        )?;
                    for result in results {
                        measurements.insert(
                            "tessellator",
                            Box::new(result.tessellator),
                        );
                        measurements
                            .insert("filename", Box::new(result.filename));
                        measurements.insert("frame", Box::new(result.frame));
                        measurements
                            .insert("frame_time", Box::new(result.frame_time));
                        measurements
                            .insert("triangles", Box::new(result.triangles));
                    }
                }
            }
            // Bandaid removal
            log::set_max_level(prev_level);

            trace!("completed naive SVG file rendering frametime capture");
            Ok(measurements.into())
        });

        Ok(bfn)
    }
}
