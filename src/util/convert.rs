use crate::Result;
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;

pub fn path_to_svg<P>(path: P) -> Result<SVGDocument>
where
    P: Into<PathBuf>,
{
    let file = SVGFile::from(&path.into());
    Ok(SVGDocument::try_from(file)?)
}

pub fn to_serializable<T: 'static>(
    vec: Vec<T>,
) -> Vec<Box<dyn erased_serde::Serialize>>
where
    T: serde::Serialize,
{
    vec.into_iter()
        .map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) })
        .collect()
}
