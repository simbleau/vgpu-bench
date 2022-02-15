use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::driver::Driver;
use vgpu_bench::models::Unit;

use std::thread;
use std::time::Duration;

pub fn main() {
    let benchmark = Unit::from("Benchmark-1", |_| {
        // Some expensive operation...
        Ok(thread::sleep(Duration::from_secs(5)))
    });

    Driver::builder()
        .logger(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .add(benchmark)
        .build()
        .run();
}
