use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Barrier;
use std::thread;
use std::time::{Duration, Instant};

use crate::driver::DriverOptions;
use crate::log_assert;

use super::benchmark_metadata::BenchmarkMetadata;
use super::{monitor::Monitor, BenchmarkFn};
use anyhow::{anyhow, Result};
use log::{debug, info, trace, warn};

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
        // Lifecycle hook - 'on_start'
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    trace!(
                        "{mon_name}: waiting on lifecycle 'on_start' barrier",
                        mon_name = mon.metadata().name
                    );
                    barrier.wait();
                    trace!(
                        "{mon_name}: released from lifecycle 'on_start' barrier",
                        mon_name = mon.metadata().name
                    );
                    mon.on_start();
                    trace!(
                        "{mon_name}: started monitor",
                        mon_name = mon.metadata().name
                    );
                });
            }
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        // Prepare buffers for measurables
        let barrier = Barrier::new(self.monitors.len() + 1);
        let complete = AtomicBool::new(false);
        log_assert!(self.func.is_some(), "this benchmark has already run");
        let func = self.func.take().unwrap();
        let start_time = Instant::now();
        crossbeam::scope(|scope| {
            let bm_name = self.metadata().name.to_owned();
            let num_mon = self.monitors.len();
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    let mon_name = mon.metadata().name.clone();
                    trace!("{mon_name}: waiting on execution barrier");
                    barrier.wait();
                    trace!("{mon_name}: released from execution barrier");
                    // Spinlock on completion of Benchmark
                    let freq_nanos = mon.metadata().frequency.as_duration().as_nanos();
                    loop {
                        // Sleep until next poll
                        let time_since_start = (Instant::now() - start_time).as_nanos();
                        let time_till_next = freq_nanos - (time_since_start % freq_nanos);
                        let sleep_time = Duration::from_nanos(time_till_next as u64);
                        //debug!("tss: {tss:?}, ttn: {ttn:?}, st: {st:?}", tss = Duration::from_nanos(time_since_start as u64), ttn = Duration::from_nanos(time_till_next as u64), st = sleep_time);
                        thread::sleep(sleep_time);

                        // Poll
                        let poll_start_time = Instant::now();
                        let measurement = mon.poll();
                        let poll_end_time = Instant::now();
                        let elapsed = poll_end_time - poll_start_time;

                        // Check runtime elapsed time
                        if elapsed > mon.metadata().frequency.as_duration() {
                            // Calculate amount of polls missed
                            let this_poll_id = (poll_start_time - start_time).as_nanos() / freq_nanos;
                            let next_poll_id = (poll_end_time - start_time).as_nanos() / freq_nanos;
                            let missed_polls = next_poll_id - this_poll_id;

                            // Processing time overflowed next poll
                            warn!("{mon_name} missed ~{missed_polls} poll(s) due to expensive computation overrunning frequency time allotment" );
                        }

                        if complete.load(Ordering::Relaxed) == true {
                            break;
                        } else {
                            // TODO save measurement
                            debug!("{mon_name}: Polled {measurement:?} in {elapsed:?}");
                        }
                    }
                    trace!("{mon_name}: broke execution spinlock");
                });
            }
            trace!("{bm_name}: waiting on execution barrier");
            barrier.wait();
            trace!("{bm_name}: released from execution barrier");
            info!("{bm_name} is starting with {num_mon} monitors");
            func.call(options).unwrap();
            complete.store(true, Ordering::Release);
            info!("{bm_name}: finished execution");
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        // Lifecycle hook - 'on_stop'
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    trace!(
                        "{mon_name}: waiting on lifecycle 'on_stop' barrier",
                        mon_name = mon.metadata().name
                    );
                    barrier.wait();
                    trace!(
                        "{mon_name}: released from lifecycle 'on_stop' barrier",
                        mon_name = mon.metadata().name
                    );
                    mon.on_stop();
                    trace!(
                        "{mon_name}: stopped monitor",
                        mon_name = mon.metadata().name
                    );
                });
            }
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        Ok(())
    }
}
