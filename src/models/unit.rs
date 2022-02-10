//TODO

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

use crate::driver::DriverOptions;

use super::benchmark_metadata::BenchmarkMetadata;
use super::monitor::Measurable;
use super::MonitorMetadata;
use super::{monitor::Monitor, BenchmarkFn};
use anyhow::Result;

pub struct Unit {
    pub data: &'static BenchmarkMetadata,
    pub func: &'static BenchmarkFn,
    pub monitors: Vec<Box<dyn Monitor + Send + Sync>>,
}

impl Unit {
    pub fn data(&self) -> &BenchmarkMetadata {
        self.data
    }

    pub fn run(&mut self, options: &DriverOptions) -> Result<()> {
        // Lifecycle hook - Before run
        let barrier = Arc::new(Barrier::new(self.monitors.len()));
        let mut handles = Arc::new(Vec::with_capacity(self.monitors.len()));
        for mut mon in &mut self.monitors {
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                mon.before();
            }));
        }
        // Wait for threads to finish.
        for handle in handles {
            handle.join().unwrap();
        }

        // Prepare buffers for measurables
        let complete = AtomicBool::new(true);

        // Run monitors and benchmark together
        let barrier = Arc::new(Barrier::new(self.monitors.len() + 1));
        let handles = Vec::with_capacity(self.monitors.len());
        for mon in self.monitors {
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                // Begin monitoring
                let buffer = Vec::<Measurable>::new();
                while complete.load(Ordering::Relaxed) {
                    // TODO sleep for poll rate
                }
                mon.before();
            }));
        }
        // Run benchmark
        barrier.wait();
        self.func.call(options)?;
        complete.store(true, Ordering::Release);
        // Wait for monitors to finish.
        for handle in handles {
            handle.join().unwrap();
        }

        // Lifecycle hook - After completion
        let barrier = Arc::new(Barrier::new(self.monitors.len()));
        let handles = Vec::with_capacity(self.monitors.len());
        for mon in self.monitors {
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                barrier.wait();
                mon.after();
            }));
        }
        // Wait for threads to finish.
        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }
}
