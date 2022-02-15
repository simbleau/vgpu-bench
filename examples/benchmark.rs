use std::rc::Rc;
use std::sync::Mutex;

use log::LevelFilter;
use rand::Rng;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::Benchmark;
use vgpu_bench::Driver;

pub fn main() {
    let items = Rc::new(Mutex::new(vec![]));

    let items_ref = items.clone();
    let mut generator = rand::thread_rng();
    let insert_benchmark = Benchmark::from("Bm-Push", move |_| {
        let mut items = items_ref.try_lock().unwrap();
        // Benchmark Vec's push algorithm
        for _ in 0..10_000_000 {
            items.push(generator.gen::<u64>());
        }
        Ok(())
    });

    let items_ref = items.clone();
    let sort_benchmark = Benchmark::from("Bm-Sort", move |_| {
        let mut items = items_ref.try_lock().unwrap();
        // Benchmark Vec's sorting algorithm
        items.sort();
        Ok(())
    });

    Driver::builder()
        .logger(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .add(insert_benchmark)
        .add(sort_benchmark)
        .build()
        .run();
}
