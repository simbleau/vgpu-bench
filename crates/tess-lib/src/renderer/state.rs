use super::types::Buffers;
use super::types::GpuGlobals;
use super::types::SceneGlobals;
use crate::artifacts::TessellationData;
use crate::renderer::util;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub buffers: Buffers,
    pub scene: SceneGlobals,
    pub data: TessellationData,
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
            present_mode: wgpu::PresentMode::Immediate,
        };

        // Make surface
        surface.configure(&device, &config);

        // Make buffers
        let buffers = util::build_buffers(&device, &data);

        // Choose pipeline
        let render_pipeline = util::build_pipeline(&device, &buffers);

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
            data,
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
        // Input handling not supported.
        false
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

            pass.draw_indexed(0..(self.data.indices.len() as u32), 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
