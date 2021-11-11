use simplelog::SharedLogger;

pub struct RunOptions {
    pub loggers: Vec<Box<dyn SharedLogger>>,
    pub functions: Vec<Box<dyn Fn()>>,
}
impl RunOptions {
    pub fn builder() -> RunOptionsBuilder {
        RunOptionsBuilder::new()
    }
}

pub struct RunOptionsBuilder {
    loggers: Vec<Box<dyn SharedLogger>>,
    functions: Vec<Box<dyn Fn()>>,
}
impl RunOptionsBuilder {
    fn new() -> Self {
        Self {
            loggers: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl RunOptionsBuilder {
    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.functions.push(Box::new(f));
        self
    }

    pub fn build(self) -> RunOptions {
        RunOptions {
            loggers: self.loggers,
            functions: self.functions,
        }
    }
}
