use super::SVGFile;

#[derive(Debug, Clone)]
pub struct SVGDocument(String);

impl SVGDocument {
    pub fn content(&self) -> &str {
        return self.0.as_str();
    }
}

impl From<SVGFile> for SVGDocument {
    fn from(item: SVGFile) -> Self {
        let source = std::fs::read(item.path()).unwrap();
        SVGDocument(String::from_utf8_lossy(&source).to_string())
    }
}

impl<T> From<T> for SVGDocument
where
    T: Into<String>,
{
    fn from(item: T) -> Self {
        SVGDocument(item.into())
    }
}
