use super::error::{NaiveRendererError, Result};
use super::state::State;
use super::types::SceneGlobals;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::SVGDocument;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tessellation_util::artifacts::TessellationData;
use tessellation_util::backends::Tessellator;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub struct TriangleRenderer {
    window: Option<Window>,
    event_loop: Option<EventLoop<()>>,
    state: Option<State>,
}

impl TriangleRenderer {
    pub fn new() -> Self {
        TriangleRenderer {
            window: None,
            event_loop: None,
            state: None,
        }
    }

    pub fn init_with_svg(
        &mut self,
        tessellator: &mut dyn Tessellator,
        svg: &SVGDocument,
    ) -> Result<()> {
        // Get global scene space
        let scene = super::util::get_globals(svg);

        // Tessellate the data
        tessellator.init(svg);
        let data = tessellator
            .get_tessellation_data()
            .map_err(|err| NaiveRendererError::FatalTessellationError(err))?;

        self.init(scene, data)
    }

    pub fn init(
        &mut self,
        scene: SceneGlobals,
        data: TessellationData,
    ) -> Result<()> {
        let event_loop_thread: EventLoop<()>;
        if cfg!(windows) {
            event_loop_thread =
                winit::platform::windows::EventLoopExtWindows::new_any_thread();
        } else if cfg!(unix) {
            event_loop_thread =
                winit::platform::unix::EventLoopExtUnix::new_any_thread();
        } else {
            unimplemented!();
        }
        let window = WindowBuilder::new().build(&event_loop_thread)?;
        window.set_resizable(true);
        let state = pollster::block_on(State::new(&window, scene, data));

        self.window = Some(window);
        self.event_loop = Some(event_loop_thread);
        self.state = Some(state);

        Ok(())
    }

    pub fn toggle_wireframe(&mut self) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.toggle_wireframe();
        } else {
            return Err(NaiveRendererError::RendererNotInitialized);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if let (Some(state), Some(window), Some(event_loop)) = (
            self.state.as_mut(),
            self.window.as_mut(),
            self.event_loop.as_mut(),
        ) {
            event_loop.run_return(move |event, _, control_flow| match event {
                Event::RedrawRequested(_) => match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = ControlFlow::Exit
                    }
                    Err(e) => eprintln!("{:?}", e),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::Resized(size) => state.resize(size),
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = ControlFlow::Exit
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    } => state.input(key),
                    _ => {}
                },
                _ => {}
            });
        } else {
            return Err(NaiveRendererError::RendererNotInitialized);
        }

        Ok(())
    }

    pub fn time(&mut self, frames: usize) -> Result<RenderTimeResult> {
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
                                let mut frame_times =
                                    frame_times_arc.lock().unwrap();
                                frame_times.push(dur);
                            }
                            frame_count += 1
                        }
                        Err(wgpu::SurfaceError::Lost) => {
                            state.resize(state.size)
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            *control_flow = ControlFlow::Exit
                        }
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
        let frame_times =
            Mutex::into_inner(Arc::try_unwrap(frame_times).unwrap()).unwrap();

        // Ensure all frames were rendered.
        if frame_times.len() != frames {
            return Err(NaiveRendererError::FatalRenderingError);
        }

        // Collect results
        Ok(RenderTimeResult { frame_times })
    }
}
