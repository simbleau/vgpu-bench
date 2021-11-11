use std::path::Path;

use simplelog::SharedLogger;

pub struct RunOptions<'a> {
    pub output_dir: &'a Path,
    pub loggers: Vec<Box<dyn SharedLogger>>,
    pub functions: Vec<Box<dyn Fn(&'a Path)>>,
}
impl<'a> RunOptions<'a> {
    pub fn builder() -> RunOptionsBuilder<'a> {
        RunOptionsBuilder::new()
    }
}

pub struct RunOptionsBuilder<'a> {
    output_dir: &'a Path,
    loggers: Vec<Box<dyn SharedLogger>>,
    functions: Vec<Box<dyn Fn(&'a Path)>>,
}
impl<'a> RunOptionsBuilder<'a> {
    fn new() -> Self {
        Self {
            output_dir: &Path::new("."),
            loggers: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl<'a> RunOptionsBuilder<'a> {
    pub fn output_dir(mut self, output_dir: &'a Path) -> Self {
        self.output_dir = output_dir;
        self
    }

    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add<F: Fn(&'a Path) + 'static>(mut self, f: F) -> Self {
        self.functions.push(Box::new(f));
        self
    }

    pub fn build(self) -> RunOptions<'a> {
        // Build loggers
        RunOptions {
            output_dir: self.output_dir,
            loggers: self.loggers,
            functions: self.functions,
        }
    }
}
