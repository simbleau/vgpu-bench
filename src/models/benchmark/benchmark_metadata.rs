pub struct BenchmarkMetadata {
    name: &'static str,
}

impl BenchmarkMetadata {
    pub fn new(name: &'static str) -> Self {
        BenchmarkMetadata { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
