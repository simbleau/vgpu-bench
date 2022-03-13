use crate::{Measurement, Result};
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;

pub fn path_to_svg<P>(path: P) -> Result<SVGDocument>
where
    P: Into<PathBuf>,
{
    let file = SVGFile::from(&path.into());
    Ok(SVGDocument::try_from(file)?)
}

pub fn erase<T: 'static>(y: T) -> Box<dyn erased_serde::Serialize>
where
    T: serde::Serialize,
{
    let x: Box<dyn erased_serde::Serialize> = { Box::new(y) };
    x
}

pub fn to_measurement<T: 'static>(y: T) -> Measurement
where
    T: serde::Serialize,
{
    Measurement {
        inner: Box::new(erase(y)),
    }
}

pub fn to_serializable<T>(vec: Vec<T>) -> Vec<Box<dyn erased_serde::Serialize>>
where
    T: serde::Serialize + 'static,
{
    vec.into_iter()
        .map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) })
        .collect()
}
