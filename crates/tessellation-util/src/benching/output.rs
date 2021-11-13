use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SVGFileProfile {
    pub tessellator: String,
    pub filename: String,
    pub vertices: u32,
    pub indices: u32,
    pub triangles: u32,
}

#[derive(Debug, Serialize)]
pub struct SVGPrimitiveProfile {
    pub tessellator: String,
    pub primitive: String,
    pub primitive_count: u32,
    pub vertices: u32,
    pub indices: u32,
    pub triangles: u32,
}

#[derive(Debug, Serialize)]

pub struct SVGFileTessellationTime {
    pub tessellator: String,
    pub filename: String,
    pub init_time: u128,
    pub tess_time: u128,
}

#[derive(Debug, Serialize)]
pub struct SVGDocumentTessellationTime {
    pub tessellator: String,
    pub init_time: u128,
    pub tess_time: u128,
}

#[derive(Debug, Serialize)]
pub struct PrimitiveTessellationTime {
    pub tessellator: String,
    pub primitive: String,
    pub amount: u32,
    pub init_time: u128,
    pub tess_time: u128,
}
