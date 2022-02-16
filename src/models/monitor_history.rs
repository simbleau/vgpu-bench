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

    pub fn push(&mut self, measurable: Measurable) {
        self.history.push(measurable);
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn write(self, path: &Path) -> Result<()> {
        // TODO this fn doesn't need to consume self
        let rows: Vec<Box<dyn Serialize>> = self
            .history
            .into_iter()
            .map(|x| -> Box<dyn Serialize> { Box::new(x) })
            .collect();
        Ok(util::write_csv(&path, &rows)?)
    }
}
