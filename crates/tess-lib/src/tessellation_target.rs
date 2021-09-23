use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::Tessellator;
pub struct TessellationTarget {
    pub path: PathBuf,
}

impl TessellationTarget {
    pub fn time_tessellation(&mut self, t: Box<&mut dyn Tessellator>) -> (Duration, Duration) {
        // Time pre-processing
        let t1 = Instant::now();
        t.init(&self);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time tessellation
        let t1 = Instant::now();
        t.tessellate().unwrap();
        let t2 = Instant::now();
        let dur2 = t2.duration_since(t1);

        // Return duration passed
        (dur1, dur2)
    }
}
