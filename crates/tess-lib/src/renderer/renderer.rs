use std::thread;

use crate::{renderer::state::State, targets::TessellationData};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

use super::util::SceneGlobals;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        env_logger::init();
        Renderer {}
    }

    pub fn run(&self, scene: SceneGlobals, data: TessellationData, frames: u32) {
        let event_loop_thread: EventLoop<()> =
            winit::platform::unix::EventLoopExtUnix::new_any_thread();
        let mut event_loop = Box::new(event_loop_thread);
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        window.set_resizable(false);
        let mut state = pollster::block_on(State::new(&window, scene, data));
        event_loop.run_return(move |event, _, control_flow| {
            let mut frame_count = 0;
            match event {
                Event::RedrawRequested(_) => {
                    state.update();
                    match state.render() {
                        Ok(_) => frame_count += 1,
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                },
                _ => {} // Do nothing
            }
            if frame_count == frames {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
