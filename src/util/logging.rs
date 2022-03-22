//Vec<Box<dyn SharedLogger>>

use log::{trace, LevelFilter};
use simplelog::{
    ColorChoice, CombinedLogger, Config, SharedLogger, TermLogger, TerminalMode,
};

pub fn init_default() {
    let default: Box<dyn SharedLogger> = TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::default(),
        ColorChoice::Auto,
    );
    init(vec![default]);
}

pub fn init(loggers: Vec<Box<dyn SharedLogger>>) {
    // Initialize logger
    match CombinedLogger::init(loggers) {
            Err(_) => eprintln!(
                "Logger failed to initialize... Was it already initialized by another driver?"
            ),
            Ok(_) => trace!("logging initialized"),
        };
}
