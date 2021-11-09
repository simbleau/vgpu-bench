use log::LevelFilter;

pub struct RunOptions {
    functions: Vec<Box<dyn Fn()>>,
}
impl RunOptions {
    // This method will help users to discover the builder
    pub fn builder() -> RunOptionsBuilder {
        RunOptionsBuilder::new()
    }

    pub fn functions(&self) -> &Vec<Box<dyn Fn()>> {
        &self.functions
    }
}

pub struct RunOptionsBuilder {
    functions: Vec<Box<dyn Fn()>>,
}
impl RunOptionsBuilder {
    fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl RunOptionsBuilder {
    pub fn logging(mut self, level: LevelFilter) -> Self {
        env_logger::builder().filter_level(level).init();
        self
    }

    pub fn add<F: Fn() + 'static>(mut self, f: F) -> Self {
        self.functions.push(Box::new(f));
        self
    }

    pub fn build(self) -> RunOptions {
        RunOptions {
            functions: self.functions,
        }
    }
}
