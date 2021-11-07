pub struct RunOptions {
    functions: Vec<Box<dyn Fn()>>,
}
impl RunOptions {
    // This method will help users to discover the builder
    pub fn builder() -> RunOptionsBuilder {
        RunOptionsBuilder::default()
    }

    pub fn functions(&self) -> &Vec<Box<dyn Fn()>> {
        &self.functions
    }
}

pub struct RunOptionsBuilder {
    functions: Vec<Box<dyn Fn()>>,
}
impl Default for RunOptionsBuilder {
    fn default() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

impl RunOptionsBuilder {
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
