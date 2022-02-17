use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::models::driver::DriverOptions;
use crate::{log_assert, util, MonitorHistory};

use super::benchmark_metadata::BenchmarkMetadata;
use super::{monitor::Monitor, BenchmarkFn};
use anyhow::{anyhow, Result};
use crossbeam::thread::ScopedJoinHandle;
use log::{debug, error, info, trace, warn};

pub struct Benchmark {
    pub data: BenchmarkMetadata,
    pub func: Option<BenchmarkFn>,
    pub monitors: Vec<Box<dyn Monitor + Send + Sync>>,
}

impl Benchmark {
    pub fn new(data: BenchmarkMetadata, func: BenchmarkFn) -> Self {
        Benchmark {
            data,
            func: Some(func),
            monitors: vec![],
        }
    }

    pub fn from<F>(name: &'static str, func: F) -> Self
    where
        F: FnOnce(&DriverOptions) -> Result<()> + 'static,
    {
        let bfn = BenchmarkFn::from(func);
        let metadata = BenchmarkMetadata { name };
        Benchmark {
            data: metadata,
            func: Some(bfn),
            monitors: Vec::new(),
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
        // Check conditions for run
        log_assert!(self.func.is_some(), "this benchmark has already run");
        let bm_dir = options.benchmark_dir();
        log_assert!(
            util::io::dir_is_empty(&bm_dir)
                || util::io::dir_create_all(&bm_dir).is_ok(),
            "{bm_dir:?} is not permissive or empty"
        );

        // Start run
        let func = self.func.take().unwrap();
        let bm_name = self.metadata().name.to_owned();
        let num_mon = self.monitors.len();
        info!("{bm_name}: starting with {num_mon} monitors");

        // Lifecycle hook - 'on_start'
        self.monitor_lifecycle_hook("on_start", |mon| Ok(mon.on_start()))?;
        trace!("{bm_name}: started all monitors");

        // Prepare buffers for measurables
        let barrier = Barrier::new(self.monitors.len() + 1);
        let complete = AtomicBool::new(false);
        let start_time = Instant::now();
        crossbeam::scope(|scope| {
            for mon in self.monitors.iter_mut() {
                scope.spawn(|_| {
                    let mut history = MonitorHistory::new();
                    let mon_name = mon.metadata().name.clone();
                    let freq_nanos = mon.metadata().frequency.as_duration().as_nanos();

                    trace!("{mon_name}: waiting to poll");
                    barrier.wait();
                    trace!("{mon_name}: starting polling");

                    // Spinlock on completion of Benchmark
                    loop {
                        // Sleep until next poll time
                        let time_since_start = (Instant::now() - start_time).as_nanos();
                        let time_till_next = freq_nanos - (time_since_start % freq_nanos);
                        let sleep_time = Duration::from_nanos(time_till_next as u64);
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
                            warn!("{mon_name}: missed {missed_polls} poll trigger(s)" );
                        }

                        if complete.load(Ordering::Relaxed) == true {
                            break;
                        } else {
                            match measurement {
                                Ok(measurable) => {
                                    debug!("{mon_name}: polled {measurable:?} in {elapsed:?}");
                                    history.push(measurable);
                                },
                                Err(e) => error!("{mon_name}: failed to poll with error '{e}'")
                            }
                        }
                    }

                    // Write results
                    // TODO fix me
                    history.write(Path::new("ex"))
                }
            );
}
            trace!("{bm_name}: waiting to execute");
            barrier.wait();
            trace!("{bm_name}: starting execution");
            func.call(options).unwrap();
            trace!("{bm_name}: completed execution");
            complete.store(true, Ordering::Release);
        })
        .map_err(|thread_ex| anyhow!("Unit thread exception: {thread_ex:?}"))?;

        // Lifecycle hook - 'on_stop'
        self.monitor_lifecycle_hook("on_stop", |mon| Ok(mon.on_stop()))?;
        trace!("{bm_name}: stopped all monitors");

        info!("{bm_name}: finished execution");
        Ok(())
    }

    fn monitor_lifecycle_hook<F, T>(
        &mut self,
        lifecycle_name: &'static str,
        func: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(&mut Box<dyn Monitor + Send + Sync + 'static>) -> Result<T>
            + 'static
            + Send
            + Sync,
        T: Send,
        T: std::fmt::Debug,
    {
        // Buffer for monitor lifecycle hook results
        let results = Arc::new(Mutex::new(Vec::new()));

        // Run monitor lifecycle hook
        let results_ref = results.clone();
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            // Spawn threads
            for mon in self.monitors.iter_mut() {
                let _: ScopedJoinHandle<'_, Result<(), anyhow::Error>> = scope.spawn(|_| {
                    let mon_name = &mon.metadata().name;
                    // Wait for all threads
                    trace!(
                        "{mon_name}: blocking on '{lifecycle_name}' lifecycle barrier"
                    );
                    barrier.wait();
                    trace!(
                        "{mon_name}: released from '{lifecycle_name}' lifecycle barrier"
                    );
                    // Get result from given logic
                    let result = func(mon)?;
                    // Append results
                    let mut results_lock = results_ref.lock().unwrap();
                    results_lock.push(result);
                    Ok(())
                });
            }
        })
        .map_err(|ex| anyhow!("lifecycle hook exception: {ex:?}"))?;

        // Release the last reference
        drop(results_ref);

        // SAFETY: No one has a reference to results anymore.
        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        Ok(results)
    }
}
