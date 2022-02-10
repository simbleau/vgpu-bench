use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Barrier;
use std::thread;

use crate::driver::DriverOptions;
use crate::log_assert;

use super::benchmark_metadata::BenchmarkMetadata;
use super::{monitor::Monitor, BenchmarkFn};
use anyhow::{anyhow, Result};
use log::{debug, trace};

pub struct Unit {
    pub data: BenchmarkMetadata,
    pub func: Option<BenchmarkFn>,
    pub monitors: Vec<Box<dyn Monitor + Send + Sync>>,
}

impl Unit {
    pub fn new(data: BenchmarkMetadata, func: BenchmarkFn) -> Self {
        Unit {
            data,
            func: Some(func),
            monitors: vec![],
        }
    }

    pub fn monitors(&self) -> &Vec<Box<dyn Monitor + Send + Sync>> {
        &self.monitors
    }

    pub fn monitors_mut(&mut self) -> &mut Vec<Box<dyn Monitor + Send + Sync>> {
        &mut self.monitors
    }

    pub fn metadata(&self) -> &BenchmarkMetadata {
        &self.data
    }

    pub fn run(&mut self, options: &DriverOptions) -> Result<()> {
        // Lifecycle hook - Before run
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    trace!(
                        "{mon_name}: waiting on lifecycle 'before' barrier",
                        mon_name = mon.metadata().name
                    );
                    barrier.wait();
                    trace!(
                        "{mon_name}: released from lifecycle 'before' barrier",
                        mon_name = mon.metadata().name
                    );
                    mon.before();
                    trace!(
                        "{mon_name}: began monitoring",
                        mon_name = mon.metadata().name
                    );
                });
            }
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        // Prepare buffers for measurables
        let barrier = Barrier::new(self.monitors.len() + 1);
        let complete = AtomicBool::new(false);
        log_assert!(self.func.is_some(), "This benchmark has already run");
        let func = self.func.take().unwrap();
        crossbeam::scope(|scope| {
            let bm_name = self.metadata().name.to_owned();
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    trace!("{mon_name}: waiting on execution barrier", mon_name = mon.metadata().name);
                    barrier.wait();
                    trace!("{mon_name}: released from execution barrier", mon_name = mon.metadata().name);
                    // Spinlock on completion of Benchmark
                    loop {
                        thread::sleep(mon.metadata().frequency.as_duration());
                        // TODO record start and stop time of tick for accuracy
                        mon.tick();
                        let measurement = mon.poll();
                        if complete.load(Ordering::Relaxed) == true {
                            break;
                        } else {
                            // TODO save measurement
                            debug!("Monitor {{{mon_name}}}: Poll {{{measurement:?}}}", mon_name = mon.metadata().name);
                        }
                    }
                    trace!("{mon_name}: broke execution spinlock", mon_name = mon.metadata().name);
                });
            }
            trace!("{bm_name}: waiting on execution barrier");
            barrier.wait();
            trace!("{bm_name}: released from execution barrier");
            func.call(options).unwrap();
            complete.store(true, Ordering::Release);
            trace!("{bm_name}: finished execution");
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        // Lifecycle hook - After completion
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    trace!(
                        "{mon_name}: waiting on lifecycle 'after' barrier",
                        mon_name = mon.metadata().name
                    );
                    barrier.wait();
                    trace!(
                        "{mon_name}: released from lifecycle 'after' barrier",
                        mon_name = mon.metadata().name
                    );
                    mon.after();
                    trace!(
                        "{mon_name}: finished monitoring",
                        mon_name = mon.metadata().name
                    );
                });
            }
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        Ok(())
    }
}
