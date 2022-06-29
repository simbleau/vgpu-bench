use std::{collections::HashMap, path::Path};

use crate::models::{Measurable, Measurement, Measurements};
use crate::Result;

/// HashMap of <String, Measurements> types
#[derive(Debug)]
pub struct MonitorBundle {
    pub monitor_measurements: HashMap<String, Measurements<Measurement>>,
}

impl MonitorBundle {
    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        for (name, monitor) in &self.monitor_measurements {
            let mut data_path = path.join(name);
            data_path.set_extension("csv");
            monitor.write(data_path)?;
        }

        Ok(())
    }
}

/// Unifies Measurements and MonitorBundles for further processing.
#[derive(Debug)]
pub struct BenchmarkBundle<T>
where
    T: Measurable,
{
    pub measurements: Measurements<T>,
    pub monitor_bundle: MonitorBundle,
}

impl<T> BenchmarkBundle<T>
where
    T: Measurable,
{
    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let mut data_path = path.to_owned();
        data_path.push("measurements");
        data_path.set_extension("csv");
        self.measurements.write(&data_path)?;

        // Write monitor measurements
        let mon_path = path.join("monitors");
        self.monitor_bundle.write(mon_path)?;

        Ok(())
    }
}

/// HashMap of <String, BenchmarkBundle> types
#[derive(Debug)]
pub struct DriverBundle<T>
where
    T: Measurable,
{
    pub benchmark_bundles: HashMap<String, BenchmarkBundle<T>>,
}

impl<T> DriverBundle<T>
where
    T: Measurable,
{
    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        for (name, bundle) in &self.benchmark_bundles {
            let bm_path = path.join(name);
            bundle.write(bm_path)?;
        }

        Ok(())
    }
}
