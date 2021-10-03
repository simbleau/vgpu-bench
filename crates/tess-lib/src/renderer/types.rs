use serde::Serialize;
use wgpu::{BindGroup, BindGroupLayout, Buffer};
use winit::dpi::PhysicalSize;

#[derive(Copy, Clone, Debug)]
pub struct SceneGlobals {
    pub zoom: f32,
    pub pan: [f32; 2],
    pub window_size: PhysicalSize<u32>,
    pub wireframe: bool,
    pub size_changed: bool,
}

#[derive(Debug)]
pub struct Buffers {
    pub ibo: Buffer,
    pub vbo: Buffer,
    pub prims_ubo: Buffer,
    pub transforms_ubo: Buffer,
    pub globals_ubo: Buffer,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub prim_id: u32,
}

// A 2x3 matrix (last two members of data1 unused).
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct GpuTransform {
    pub data0: [f32; 4],
    pub data1: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct GpuPrimitive {
    pub transform: u32,
    pub color: u32,
    pub _pad: [u32; 2],
}

impl GpuPrimitive {
    pub fn new(transform_idx: u32, color: usvg::Color, alpha: f32) -> Self {
        GpuPrimitive {
            transform: transform_idx,
            color: ((color.red as u32) << 24)
                + ((color.green as u32) << 16)
                + ((color.blue as u32) << 8)
                + (alpha * 255.0) as u32,
            _pad: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct VertexCtor {
    pub prim_id: u32,
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}
