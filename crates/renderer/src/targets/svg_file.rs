use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SVGFile(PathBuf);

impl SVGFile {
    pub fn path(&self) -> &PathBuf {
        return &self.0;
    }
}

impl From<&PathBuf> for SVGFile {
    fn from(item: &PathBuf) -> Self {
        SVGFile(item.to_path_buf())
    }
}

impl From<PathBuf> for SVGFile {
    fn from(item: PathBuf) -> Self {
        SVGFile(item)
    }
}
