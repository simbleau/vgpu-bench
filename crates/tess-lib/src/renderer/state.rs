use wgpu::include_spirv;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::renderer::util;
use crate::targets::TessellationData;

use super::util::SceneGlobals;
use super::Buffers;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub buffers: Buffers,
    pub scene: SceneGlobals,
    pub indices: usize,
}

impl State {
    pub async fn new(window: &Window, scene: SceneGlobals, data: TessellationData) -> Self {
        // The instance is a handle to our GPU
        // Backends::all() => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        // The surface is part of the window to draw on
        let surface = unsafe { instance.create_surface(window) };
        // The adapter is a handle to an actual graphics card
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // Device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // GPU Features - None needed for triangle rendering...
                    features: wgpu::Features::empty(),
                    // Limits the resources we can create - Default for better cross-platform support
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        // Config
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        surface.configure(&device, &config);

        // Make pipeline
        let frag_spv = include_spirv!("shaders/geometry.frag.spv").source;
        let _frag_wgsl = wgpu::ShaderSource::Wgsl(include_str!("shaders/frag.wgsl").into());
        let frag_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Frag Shader"),
            source: frag_spv,
        });

        let vert_spv = include_spirv!("shaders/geometry.vert.spv").source;
        let _vert_wgsl = wgpu::ShaderSource::Wgsl(include_str!("shaders/vert.wgsl").into());
        let vert_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vert Shader"),
            source: vert_spv,
        });

        // Get buffers
        let buffers = util::get_buffers(&device, &data);

        // Make pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&buffers.bind_group_layout],
            push_constant_ranges: &[],
            label: None,
        });

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_module,
                entry_point: "main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            format: wgpu::VertexFormat::Uint32,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &frag_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                front_face: wgpu::FrontFace::Ccw,
                strip_index_format: None,
                cull_mode: None,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        };

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        queue.write_buffer(
            &buffers.transforms_ubo,
            0,
            bytemuck::cast_slice(&data.transforms),
        );
        queue.write_buffer(
            &buffers.prims_ubo,
            0,
            bytemuck::cast_slice(&data.primitives),
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            buffers,
            scene,
            indices: data.mesh.indices.len(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        // Input handling currently not supported.
        false
    }

    pub fn update(&mut self) {
        // Currently does nothing.
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_frame()?.output;
        // The view texture
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // Encoder sends commands to the GPU
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.queue.write_buffer(
            &self.buffers.globals_ubo,
            0,
            bytemuck::cast_slice(&[GpuGlobals {
                aspect_ratio: self.size.width as f32 / self.size.height as f32,
                zoom: [self.scene.zoom, self.scene.zoom],
                pan: self.scene.pan,
                _pad: 0.0,
            }]),
        );

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                    resolve_target: None,
                }],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.render_pipeline);
            pass.set_bind_group(0, &self.buffers.bind_group, &[]);
            pass.set_index_buffer(self.buffers.ibo.slice(..), wgpu::IndexFormat::Uint32);
            pass.set_vertex_buffer(0, self.buffers.vbo.slice(..));

            pass.draw_indexed(0..(self.indices as u32), 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 2],
    pub prim_id: u32,
}

// A 2x3 matrix (last two members of data1 unused).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GpuTransform {
    pub data0: [f32; 4],
    pub data1: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct GpuGlobals {
    pub zoom: [f32; 2],
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub _pad: f32,
}

pub struct VertexCtor {
    pub prim_id: u32,
}

unsafe impl bytemuck::Pod for GpuGlobals {}
unsafe impl bytemuck::Zeroable for GpuGlobals {}
unsafe impl bytemuck::Pod for GpuVertex {}
unsafe impl bytemuck::Zeroable for GpuVertex {}
