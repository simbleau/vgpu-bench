use simplelog::{CombinedLogger, SharedLogger};

use super::RunOptions;

pub struct Driver {
    options: RunOptions,
}

impl Driver {
    pub fn from(options: RunOptions) -> Self {
        Driver { options }
    }

    pub fn run(self) {
        // Initialize logger
        CombinedLogger::init(self.options.loggers).unwrap();

        // Run all benchmarks
        for func in self.options.functions {
            func.call(());
        }
    }
}
