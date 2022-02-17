use super::SVGFile;

#[derive(Debug, Clone)]
pub struct SVGDocument(String);

impl SVGDocument {
    pub fn content(&self) -> &str {
        return self.0.as_str();
    }
}

impl TryFrom<SVGFile> for SVGDocument {
    type Error = std::io::Error;

    fn try_from(item: SVGFile) -> Result<Self, Self::Error> {
        let source = std::fs::read(item.path())?;
        Ok(SVGDocument(String::from_utf8_lossy(&source).to_string()))
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
