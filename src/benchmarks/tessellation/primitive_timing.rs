use csv::Writer;
use svg_generator::Primitive;
use tessellation_util::{
    backends::Tessellator,
    benching::{output::PrimitiveTessellationTime, Result},
};

pub struct PrimitiveTessellationTimingOptions<W>
where
    W: std::io::Write,
{
    backends: Vec<Box<dyn Tessellator>>,
    primitives: Vec<Primitive>,
    primitive_counts: Vec<u32>,
    trials: u32,
    writer: Option<Writer<W>>,
}
impl<W> std::fmt::Debug for PrimitiveTessellationTimingOptions<W>
where
    W: std::io::Write,
{
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "PrimitiveTessellationTimingOptions {{ backends: {:?}, primitives: {:?}, primitive_counts: {:?}, trials: {:?} }}",
            self.backends, self.primitives, self.primitive_counts, self.trials,
        )
    }
}

impl<W> PrimitiveTessellationTimingOptions<W>
where
    W: std::io::Write + 'static,
{
    pub fn new() -> Self {
        PrimitiveTessellationTimingOptions {
            backends: Vec::new(),
            primitives: Vec::new(),
            primitive_counts: Vec::new(),
            trials: 0,
            writer: None,
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

    pub fn trials(mut self, trials: u32) -> Self {
        self.trials = trials;
        self
    }

    pub fn writer(mut self, writer: Writer<W>) -> Self {
        self.writer = Some(writer);
        self
    }
}

pub fn write_tessellation_times<W>(
    options: PrimitiveTessellationTimingOptions<W>,
) -> Result<()>
where
    W: std::io::Write,
{
    // Collect results
    let mut results: Vec<PrimitiveTessellationTime> = Vec::new();
    for mut backend in options.backends {
        for primitive_count in &options.primitive_counts {
            for primitive in &options.primitives {
                let result =
                    tessellation_util::benching::tessellating::time_primitive(
                        backend.as_mut(),
                        primitive.clone(),
                        primitive_count.clone(),
                        options.trials,
                    );
                results.extend(result?);
            }
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
