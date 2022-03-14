pub struct BenchmarkMetadata {
    name: &'static str,
}

impl BenchmarkMetadata {
    pub(crate) fn new(name: &'static str) -> Self {
        BenchmarkMetadata { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
