use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SVGProfile {
    pub tessellator: String,
    pub filename: String,
    pub vertices: u32,
    pub indices: u32,
    pub triangles: u32,
}

#[derive(Debug, Serialize)]
pub struct SVGTessellationTime {
    pub tessellator: String,
    pub filename: String,
    pub init_time: u32,
    pub tess_time: u32,
}

#[derive(Debug, Serialize)]
pub struct PrimitiveTime {
    pub tessellator: String,
    pub primitive: String,
    pub amount: u32,
    pub init_time: u128,
    pub tess_time: u128,
}
