use std::{
    error::Error,
    time::{Duration, Instant},
};

pub struct TessellationTarget {
    pub path: String,
}

impl TessellationTarget {
    pub fn time_tessellation(&mut self, mut t: Box<dyn Tessellator>) -> (Duration, Duration) {
        // Time pre-processing
        let t1 = Instant::now();
        t.preprocess(&self);
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

pub trait Tessellator {
    fn preprocess(&mut self, t: &TessellationTarget);
    fn tessellate(&mut self) -> Result<(i32, i32), Box<dyn Error>>;
}
