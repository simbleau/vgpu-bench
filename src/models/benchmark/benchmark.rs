use anyhow::anyhow;
use crossbeam::thread::ScopedJoinHandle;
use log::{debug, error, trace, warn};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::models::{
    BenchmarkBundle, BenchmarkFn, BenchmarkMetadata, DriverOptions, Measurable,
    Measurement, Measurements, Monitor, MonitorBundle,
};
use crate::util;
use crate::Result;

/// Benchmark
pub struct Benchmark<T>
where
    T: Measurable,
{
    metadata: BenchmarkMetadata,
    func: Option<BenchmarkFn<T>>,
    monitors: Vec<Box<dyn Monitor>>,
}

impl<T> From<BenchmarkFn<T>> for Benchmark<T>
where
    T: Measurable,
{
    fn from(func: BenchmarkFn<T>) -> Self {
        let metadata = BenchmarkMetadata::new("Unnamed");
        Benchmark::new(metadata, func)
    }
}

impl<T> Benchmark<T>
where
    T: Measurable,
{
    pub fn new(data: BenchmarkMetadata, func: BenchmarkFn<T>) -> Self {
        Benchmark {
            metadata: data,
            func: Some(func),
            monitors: vec![],
        }
    }

    /// Add a static lifetime Monitor type to current Benchmarks.
    pub fn monitor<M>(mut self, monitor: M) -> Self
    where
        M: Monitor + 'static,
    {
        self.monitors.push(Box::new(monitor));
        self
    }

    /// Read only reference for metadata of current Benchmark instance.
    pub fn metadata(&self) -> &BenchmarkMetadata {
        &self.metadata
    }

    /// Multithreaded Arc Spinlock implementation for executing measurements of
    /// Benchmark's FnOnce type alongside their assigned Monitors.
    //  The results of the measurements are bundled up inside a Result<BenchmarkBundle<T>> upon completion.
    pub fn run(
        &mut self,
        options: &DriverOptions,
    ) -> Result<BenchmarkBundle<T>> {
        // Collect info
        let bm_name = self.metadata().name().to_owned();
        let bm_dir = options.output_dir().join(&bm_name);
        let num_mon = self.monitors.len();

        // Check conditions for run
        util::io::create_data_landing(bm_dir)?;

        // Start run
        debug!("{bm_name}: augmented with {num_mon} monitors");

        // Lifecycle hook - 'on_start'
        self.monitor_lifecycle_hook("on_start", |mon| Ok(mon.on_start()))?;
        trace!("{bm_name}: started all monitors");

        // Prepare buffers for measurables
        let barrier = Barrier::new(self.monitors.len() + 1);
        let complete = AtomicBool::new(false);
        let start_time = Instant::now();

        // Collect monitor measurements
        let monitor_measurement_map =
            HashMap::<String, Measurements<Measurement>>::new();
        let mmm_arc = Arc::new(Mutex::new(monitor_measurement_map));
        let scope: Result<Measurements<T>, anyhow::Error> =
            crossbeam::scope(|scope| {
                for mon in self.monitors.iter_mut() {
                    scope.spawn(|_| {
                    let mon_name = mon.name().to_owned();
                    let freq_nanos = mon.frequency().as_duration().as_nanos();
                    let mut monitor_measurements = Measurements::new();

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
                        let measurable = mon.poll();
                        let poll_end_time = Instant::now();
                        let elapsed = poll_end_time - poll_start_time;

                        // Check runtime elapsed time
                        if elapsed > mon.frequency().as_duration() {
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
                            match measurable {
                                Ok(measurable) => {
                                    debug!("{mon_name}: polled in {elapsed:?}");
                                    monitor_measurements.push(measurable);
                                },
                                Err(e) => error!("{mon_name}: failed to poll with error '{e}'")
                            }
                        }
                    }

                    let mut mmm_lock = mmm_arc.lock().unwrap();
                    mmm_lock.insert(mon_name, monitor_measurements);
                });
                }

                trace!("{bm_name}: waiting to execute");
                barrier.wait();
                trace!("{bm_name}: starting execution");
                let func = self.func.take().expect("How was this taken?");
                let measurements = func.run(&bm_name)?;
                trace!("{bm_name}: completed execution");
                complete.store(true, Ordering::Release);

                // Return results
                Ok(measurements)
            }).map_err(|thread_ex| {
            anyhow!("Unit thread exception: {thread_ex:?}")
        })?;
        let measurements = scope?;
        let monitor_measurements = Arc::try_unwrap(mmm_arc)
            .expect("No one should hold this arc!")
            .into_inner()
            .expect("No one should hold this mutex!");
        let monitor_bundle = MonitorBundle {
            monitor_measurements,
        };

        // Lifecycle hook - 'on_stop'
        self.monitor_lifecycle_hook("on_stop", |mon| Ok(mon.on_stop()))?;
        trace!("{bm_name}: stopped all monitors");

        // Package bundle
        let bundle = BenchmarkBundle {
            measurements,
            monitor_bundle,
        };

        Ok(bundle)
    }

    /// Execute Multithreaded Blocking Monitor lifecycle hooks,
    /// gather results and evaluate to a thread-safe Result<HashMap<String, Any>>.
    fn monitor_lifecycle_hook<F, Any>(
        &mut self,
        lifecycle_name: &'static str,
        func: F,
    ) -> Result<HashMap<String, Any>>
    where
        F: Fn(&mut Box<dyn Monitor + 'static>) -> Result<Any>
            + 'static
            + Send
            + Sync,
        Any: Send,
        Any: std::fmt::Debug,
    {
        // Buffer for monitor lifecycle hook results
        let results = Arc::new(Mutex::new(HashMap::new()));

        // Run monitor lifecycle hook
        let results_ref = results.clone();
        let barrier = Barrier::new(self.monitors.len());
        crossbeam::scope(|scope| {
            // Spawn threads
            for mon in self.monitors.iter_mut() {
                let _: ScopedJoinHandle<'_, Result<(), anyhow::Error>> = scope.spawn(|_| {
                    let mon_name = mon.name().to_owned();
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
                    results_lock.insert(mon_name, result);
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
