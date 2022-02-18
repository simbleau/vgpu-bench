use std::path::Path;

use erased_serde::Serialize;

use crate::util;
use crate::Measurable;
use crate::Result;

pub struct MonitorHistory {
    history: Vec<Measurable>,
}

impl MonitorHistory {
    pub fn new() -> Self {
        MonitorHistory {
            history: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn push(&mut self, measurable: Measurable) {
        self.history.push(measurable);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let rows: Vec<Box<dyn Serialize>> = self
            .history
            .iter()
            .map(|x| -> Box<dyn Serialize> { Box::new(x.clone()) })
            .collect();
        Ok(util::io::write_csv(path, &rows)?)
    }
}
