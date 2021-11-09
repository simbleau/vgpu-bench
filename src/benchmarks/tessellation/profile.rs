use csv::Writer;
use std::path::PathBuf;
use tessellation::{
    backends::Tessellator,
    benching::{error::Result, output::SVGProfile},
};

pub struct SVGProfiler<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    writer: Option<Writer<W>>,
}

impl<W> SVGProfiler<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        SVGProfiler {
            backends: Vec::new(),
            assets: Vec::new(),
            writer: None,
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
        for asset in assets {
            self.assets.push(asset.into());
        }
        self
    }

    pub fn writer(mut self, writer: Writer<W>) -> Self {
        self.writer = Some(writer);
        self
    }
}

pub fn profile<W>(profiler: SVGProfiler<W>) -> Result<()>
where
    W: std::io::Write,
{
    // Collect results
    let mut results: Vec<SVGProfile> = Vec::new();
    for mut backend in profiler.backends {
        let backend: &mut dyn Tessellator = backend.as_mut(); // Coerce & shadow
        for file_path in &profiler.assets {
            let x = tessellation::benching::profiling::get_profile(backend, file_path)?;
            results.push(x);
        }
    }

    // Write results
    if let Some(mut writer) = profiler.writer {
        for result in results {
            writer.serialize(result)?;
        }
        writer.flush()?;
    }

    Ok(())
}
