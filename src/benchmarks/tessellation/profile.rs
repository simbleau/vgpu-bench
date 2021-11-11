use csv::Writer;
use std::path::PathBuf;
use tessellation_util::{
    backends::Tessellator,
    benching::{output::SVGProfile, Result},
};

pub struct SVGProfilingOptions<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    assets: Vec<PathBuf>,
    writer: Option<Writer<W>>,
}
impl<W> std::fmt::Debug for SVGProfilingOptions<W>
where
    W: std::io::Write,
{
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "SVGNaiveRenderingOptions {{ backends: {:?}, assets: {:?} }}",
            self.backends, self.assets
        )
    }
}

impl<W> SVGProfilingOptions<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        SVGProfilingOptions {
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
        self.assets.extend(assets.into_iter().map(|a| a.into()));
        self
    }

    pub fn writer(mut self, writer: Writer<W>) -> Self {
        self.writer = Some(writer);
        self
    }
}

pub fn write_profiles<W>(options: SVGProfilingOptions<W>) -> Result<()>
where
    W: std::io::Write,
{
    // Collect results
    let mut results: Vec<SVGProfile> = Vec::new();
    for mut backend in options.backends {
        let backend: &mut dyn Tessellator = backend.as_mut(); // Coerce & shadow
        for file_path in &options.assets {
            results.push(tessellation_util::benching::profiling::get_profile(
                backend, file_path,
            )?);
        }
    }

    // Write results
    if let Some(mut writer) = options.writer {
        for result in results {
            writer.serialize(result)?;
        }
        writer.flush()?;
    }

    Ok(())
}
