use super::error::Result;
use super::types::SceneGlobals;
use crate::{
    artifacts::{FlatRenderTimeResult, TessellationData},
    renderer::error::RendererError::FatalRenderingError,
    renderer::state::State,
};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub struct Renderer {
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    state: Option<State>,
}

impl Renderer {
    pub fn new() -> Self {
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

    pub fn run(&mut self, frames: usize) -> Result<FlatRenderTimeResult> {
        let state = self.state.as_mut().unwrap();
        let window = self.window.as_mut().unwrap();
        let event_loop = self.event_loop.as_mut().unwrap();

        let mut frame_count = 0;
        let frame_times = Arc::from(Mutex::from(Vec::<Duration>::new()));
        let frame_times_arc = frame_times.clone();
        event_loop.run_return(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    let t1 = Instant::now();
                    match state.render() {
                        Ok(_) => {
                            let t2 = Instant::now();
                            let dur = t2.duration_since(t1);
                            {
                                let mut frame_times = frame_times_arc.lock().unwrap();
                                frame_times.push(dur);
                            }
                            frame_count += 1
                        }
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::Resized(size) => state.resize(size),
                    _ => {}
                },
                _ => {}
            }

            // Return if frames have been drawn
            if frame_count == frames {
                *control_flow = ControlFlow::Exit;
            }
        });
        // Unwrap
        let frame_times = Mutex::into_inner(Arc::try_unwrap(frame_times).unwrap()).unwrap();

        // Ensure all frames were rendered.
        if frame_times.len() != frames {
            return Err(FatalRenderingError);
        }

        // Collect results
        let triangles = (&self.state.as_ref().unwrap().data.indices.len() / 3) as u32;
        Ok(FlatRenderTimeResult {
            triangles,
            frame_times,
        })
    }
}
