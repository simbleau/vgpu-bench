use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    time::{Duration, Instant},
};

pub struct TessellationTarget {
    pub tessellator: Rc<RefCell<dyn Tessellator>>,
    pub path: String,
}

impl TessellationTarget {
    pub fn time_tessellation(&mut self) -> (Duration, Duration) {
        // Time pre-processing
        let t1 = Instant::now();
        self.tessellator.borrow_mut().preprocess(&self);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time tessellation
        let t1 = Instant::now();
        self.tessellator.borrow_mut().tessellate().unwrap();
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
