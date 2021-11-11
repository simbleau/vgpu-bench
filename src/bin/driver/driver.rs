use super::RunOptions;
use simplelog::CombinedLogger;

pub struct Driver<'a> {
    options: RunOptions<'a>,
}

impl<'a> Driver<'a> {
    pub fn from(options: RunOptions<'a>) -> Self {
        Driver { options }
    }

    pub fn run(self) {
        // Initialize logger
        CombinedLogger::init(self.options.loggers).unwrap();

        // Run all benchmarks
        for func in self.options.functions {
            func(self.options.output_dir);
        }
    }
}
