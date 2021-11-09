use csv::Writer;
use rendering_util::benching::output::PrimitiveNaiveRenderTime;
use rendering_util::benching::Result;
use std::path::PathBuf;
use svg_generator::Primitive;
use tessellation_util::backends::Tessellator;

pub struct PrimitiveNaiveRenderingOptions<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_count: u32,
    frames: usize,
    writer: Option<Writer<W>>,
}
impl<W> std::fmt::Debug for PrimitiveNaiveRenderingOptions<W>
where
    W: std::io::Write,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "PrimitiveNaiveRenderingOptions {{ backends: {:?}, primitives: {:?}, primitive_count: {:?}, frames: {:?} }}",
            self.backends, self.primitives, self.primitive_count, self.frames
        )
    }
}

impl<W> PrimitiveNaiveRenderingOptions<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        PrimitiveNaiveRenderingOptions {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_count: 0,
            frames: 0,
            writer: None,
        }
    }

    pub fn backend(mut self, backend: Box<dyn Tessellator>) -> Self {
        self.backends.push(backend);
        self
    }

    pub fn primitive<P>(mut self, primitive: Primitive) -> Self
    where
        P: Into<PathBuf>,
    {
        self.primitives.push(primitive);
        self
    }

    pub fn primitives<I>(mut self, primitives: I) -> Self
    where
        I: IntoIterator<Item = Primitive>,
    {
        for primitive in primitives {
            self.primitives.push(primitive);
        }
        self
    }

    pub fn primitive_count(mut self, primitive_count: u32) -> Self {
        self.primitive_count = primitive_count;
        self
    }

    pub fn writer(mut self, writer: Writer<W>) -> Self {
        self.writer = Some(writer);
        self
    }

    pub fn frames(mut self, frames: usize) -> Self {
        self.frames = frames;
        self
    }
}

pub fn write_frametimes<W>(options: PrimitiveNaiveRenderingOptions<W>) -> Result<()>
where
    W: std::io::Write,
{
    // TODO remove this bandaid
    // Bandaid fix: wgpu uses the same logger - Disable logging temporarily
    let prev_level = log::max_level();
    log::set_max_level(log::LevelFilter::Off);
    // Collect results
    let mut results: Vec<PrimitiveNaiveRenderTime> = Vec::new();
    for mut backend in options.backends {
        let backend: &mut dyn Tessellator = backend.as_mut();
        for primitive in &options.primitives {
            results.extend(rendering_util::benching::timing::time_naive_primitive(
                backend,
                primitive.to_owned(),
                options.primitive_count,
                options.frames,
            )?);
        }
    }
    // Bandaid removal
    log::set_max_level(prev_level);

    // Write results
    if let Some(mut writer) = options.writer {
        for result in results {
            writer.serialize(result)?;
        }
        writer.flush()?;
    }

    Ok(())
}
