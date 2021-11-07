pub struct RunOptions {
    functions: Vec<Box<dyn Fn()>>,
}
impl RunOptions {
    // This method will help users to discover the builder
    pub fn builder() -> RunBuilder {
        RunBuilder::default()
    }

    pub fn functions(&self) -> &Vec<Box<dyn Fn()>> {
        &self.functions
    }
}

pub struct RunBuilder {
    functions: Vec<Box<dyn Fn()>>,
}
impl Default for RunBuilder {
    fn default() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl RunBuilder {
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
