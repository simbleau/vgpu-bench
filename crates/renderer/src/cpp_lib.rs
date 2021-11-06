use crate::artifacts::RenderTimeResult;
use crate::error::CppRendererError::{Compilation, Initialization, Rendering, Runtime, Staging};
use crate::error::Result;
use crate::targets::SVGDocument;
use crate::Renderer;
use libloading::{Library, Symbol};
use std::path::PathBuf;
use std::time::Duration;

type Init = fn() -> ::std::os::raw::c_int;
type Stage = fn(input: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
type Render = fn(
    frame_times: *mut ::std::os::raw::c_ulong,
    frames: ::std::os::raw::c_size_t,
) -> ::std::os::raw::c_int;

pub struct CppRenderer {
    library: Library,
}

impl CppRenderer {
    pub unsafe fn from(lib_path: PathBuf) -> Result<Self> {
        let library = Library::new(lib_path).map_err(|err| Compilation(err))?;
        Ok(Self { library })
    }
}

impl Renderer for CppRenderer {
    fn init(&mut self) -> Result<()> {
        unsafe {
            let init: Symbol<Init> = self
                .library
                .get(b"init")
                .map_err(|err| Initialization(err))?;

            let return_code = init.call(());
            match return_code {
                0 => Ok(()),
                _ => Err(Runtime(return_code).into()),
            }
        }
    }

    fn stage(&mut self, svg: &SVGDocument) -> Result<()> {
        unsafe {
            let stage: Symbol<Stage> = self.library.get(b"stage").map_err(|err| Staging(err))?;

            let input: std::ffi::CString = str_to_cstring(svg.content());
            let return_code = stage.call((input.as_ptr(),));
            match return_code {
                0 => Ok(()),
                _ => Err(Runtime(return_code).into()),
            }
        }
    }

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult> {
        unsafe {
            let render: Symbol<Render> =
                self.library.get(b"render").map_err(|err| Rendering(err))?;

            let buffer: Vec<u64> = vec![0; frames];
            let (buffer_ptr, len, cap) = buffer.into_raw_parts();
            let return_code = render.call((buffer_ptr, frames));
            match return_code {
                0 => {
                    // Re-assemble the buffer
                    let buffer = Vec::from_raw_parts(buffer_ptr, len, cap);
                    // Convert result
                    let frame_times: Vec<Duration> = buffer
                        .into_iter()
                        .map(|nanos| Duration::from_nanos(nanos))
                        .collect();
                    // Return result
                    Ok(RenderTimeResult { frame_times })
                }
                _ => Err(Runtime(return_code).into()),
            }
        }
    }
}

// Helper function to reduce code repetition
fn str_to_cstring(input: &str) -> std::ffi::CString {
    return std::ffi::CString::new(input)
        .expect(&format!("Creation of CString failed from {}", input));
}
