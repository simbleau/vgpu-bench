use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{
    artifacts::{FlattenedRenderResult, TessellationData, TessellationProfile},
    renderer::state::State,
};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

use super::types::SceneGlobals;
use super::Result;

pub struct Renderer {
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    state: Option<State>,
}

impl Renderer {
    pub fn new() -> Self {
        env_logger::init();
        Renderer {
            window: None,
            event_loop: None,
            state: None,
        }
    }

    pub fn init(&mut self, scene: SceneGlobals, data: TessellationData) -> Result<()> {
        let event_loop_thread: EventLoop<()> =
            winit::platform::unix::EventLoopExtUnix::new_any_thread();
        let window = WindowBuilder::new().build(&event_loop_thread)?;
        window.set_resizable(true);
        let state = pollster::block_on(State::new(&window, scene, data));

        self.window = Some(window);
        self.event_loop = Some(event_loop_thread);
        self.state = Some(state);

        Ok(())
    }

    pub fn run(&mut self, frames: u32) -> Result<FlattenedRenderResult> {
        let state = self.state.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();
        let event_loop = self.event_loop.as_mut().unwrap();
        let mut frame_count = 0;
        let durations = Arc::from(Mutex::from(Vec::<Duration>::new()));
        let dur_clone = durations.clone();
        event_loop.run_return(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    let t1 = Instant::now();
                    match state.render() {
                        Ok(_) => {
                            let t2 = Instant::now();
                            let dur = t2.duration_since(t1);
                            {
                                let mut data = dur_clone.lock().unwrap();
                                data.push(dur);
                            }
                            frame_count += 1
                        }
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
                    WindowEvent::Resized(size) => state.resize(size),
                    _ => {}
                },
                _ => {} // Do nothing
            }

            // Return if frames have been drawn
            if frame_count == frames {
                *control_flow = ControlFlow::Exit;
            }
        });
        let durations = Mutex::into_inner(Arc::try_unwrap(durations).unwrap()).unwrap();

        println!("Done");
        let profile = TessellationProfile {
            vertices: 0,
            indices: 0,
        };
        Ok(FlattenedRenderResult {
            profile,
            frame_times: durations,
        })
    }
}
