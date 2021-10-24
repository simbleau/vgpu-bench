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
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
