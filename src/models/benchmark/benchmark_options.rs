use std::path::{Path, PathBuf};

pub struct BenchmarkOptions {
    pub(crate) output_dir: PathBuf,
}

impl BenchmarkOptions {
    pub(crate) fn new<P, S>(benchmark_dir: P, benchmark_name: S) -> Self
    where
        P: AsRef<Path>,
        S: Into<String>,
    {
        let output_dir = benchmark_dir.as_ref().join(benchmark_name.into());
        BenchmarkOptions { output_dir }
    }
}

impl BenchmarkOptions {
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
}
