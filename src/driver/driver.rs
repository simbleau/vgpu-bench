use super::RunOptions;

pub struct Driver {
    options: RunOptions,
}

impl Driver {
    pub fn from(options: RunOptions) -> Self {
        Driver { options }
    }

    pub fn run(self) {
        for func in self.options.functions() {
            func.call(());
        }
    }
}
