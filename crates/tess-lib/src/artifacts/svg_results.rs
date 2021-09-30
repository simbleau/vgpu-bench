use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct SVGProfileResult {
    pub tessellator: String,
    pub filename: String,
    pub vertices: u32,
    pub indices: u32,
}

#[derive(Debug, Serialize)]
pub struct SVGTessellationTimeResult {
    pub tessellator: String,
    pub filename: String,
    pub init_time: u32,
    pub tess_time: u32,
}

#[derive(Debug)]
pub struct SVGFlatRenderTimeResult {
    pub tessellator: String,
    pub filename: String,
    pub triangles: u32,
    pub frame_times: Vec<Duration>,
}

impl Serialize for SVGFlatRenderTimeResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_struct("SVGFlatRenderTimeResult", 4)?;
        seq.serialize_field("tessellator", &self.tessellator)?;
        seq.serialize_field("filename", &self.filename)?;
        seq.serialize_field("triangles", &self.triangles)?;
        for e in &self.frame_times {
            seq.serialize_field("frame", &e.as_nanos())?;
        }
        seq.end()
    }
}
