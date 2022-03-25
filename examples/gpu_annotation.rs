use std::{thread, time::Duration};
use vgpu_bench::prelude::*;

#[measurement]
struct TessellationMeasurement {
    tessellation_time: f32,
}

pub fn main() -> Result<()> {
    BenchmarkFn::new(|| {
        let mut measurements = Measurements::new();
        // Annotating steps of a benchmark...
        nvtx::mark("Step 1 - Begin");
        thread::sleep(Duration::from_secs_f32(0.5));
        measurements.push(TessellationMeasurement {
            tessellation_time: 0.5,
        });
        nvtx::mark("Step 2 - Begin");
        thread::sleep(Duration::from_secs_f32(0.35));
        measurements.push(TessellationMeasurement {
            tessellation_time: 0.35,
        });
        // Benchmarking done!
        Ok(measurements)
    })
    .run("Benchmark Test")?;

    Ok(())
}
