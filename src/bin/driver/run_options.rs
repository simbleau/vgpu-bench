use std::path::PathBuf;

use chrono::Local;
use simplelog::SharedLogger;

pub struct RunOptions {
    pub output_dir: PathBuf,
    pub loggers: Vec<Box<dyn SharedLogger>>,
    pub functions: Vec<Box<dyn Fn(&PathBuf)>>,
}
impl RunOptions {
    pub fn builder() -> RunOptionsBuilder {
        RunOptionsBuilder::new()
    }
}

pub struct RunOptionsBuilder {
    output_dir: PathBuf,
    loggers: Vec<Box<dyn SharedLogger>>,
    functions: Vec<Box<dyn Fn(&PathBuf)>>,
}
impl RunOptionsBuilder {
    fn new() -> Self {
        Self {
            output_dir: PathBuf::from(crate::dictionary::OUTPUT_DIR)
                .join(Local::now().format("%d%m%Y_%H-%M-%S").to_string()),
            loggers: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl RunOptionsBuilder {
    pub fn output_dir<P>(mut self, output_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.output_dir = output_dir.into();
        self
    }

    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add<F: Fn(&PathBuf) + 'static>(mut self, f: F) -> Self {
        self.functions.push(Box::new(f));
        self
    }

    pub fn build(self) -> RunOptions {
        RunOptions {
            output_dir: self.output_dir,
            loggers: self.loggers,
            functions: self.functions,
        }
    }
}
