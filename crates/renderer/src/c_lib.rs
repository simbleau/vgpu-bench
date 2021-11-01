use crate::{artifacts::RenderTimeResult, targets::SVGDocument, AnyError, Renderer};
use std::{error::Error, fmt, path::Path, time::Duration};

#[derive(Debug)]
struct CError {
    return_code: i32,
}
impl fmt::Display for CError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.return_code)
    }
}
impl Error for CError {}

pub struct CRenderer<P>
where
    P: AsRef<Path>,
{
    path: P,
}

impl<P> CRenderer<P>
where
    P: AsRef<Path>,
{
    pub fn from(path: P) -> Self {
        // TODO Check exists
        CRenderer { path }
    }

    pub fn build(&self) {
        cc::Build::new().file(&self.path).compile("c-renderer");
    }
}

impl<P> Renderer for CRenderer<P>
where
    P: AsRef<Path>,
{
    fn init(&mut self) -> Result<(), AnyError> {
        #[link(name = "c-renderer")]
        extern "C" {
            fn init() -> ::std::os::raw::c_int;
        }
        let return_code = unsafe { init() };
        match return_code {
            0 => Ok(()),
            _ => Err(Box::new(CError { return_code })),
        }
    }

    fn stage(&mut self, svg: &SVGDocument) -> Result<(), AnyError> {
        #[link(name = "c-renderer")]
        extern "C" {
            fn stage(input: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
        }
        let input: std::ffi::CString = str_to_cstring(svg.content());
        let return_code = unsafe { stage(input.as_ptr()) };
        match return_code {
            0 => Ok(()),
            _ => Err(Box::new(CError { return_code })),
        }
    }

    fn render(&mut self, frames: u64) -> Result<RenderTimeResult, AnyError> {
        #[link(name = "c-renderer")]
        extern "C" {
            fn render(frames: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_ulong;
        }
        let frames = frames as usize;
        let frame_times =
            unsafe { std::slice::from_raw_parts(render(frames as i32), frames).to_vec() };

        // Convert result
        let frame_times: Vec<Duration> = frame_times
            .into_iter()
            .map(|ft| Duration::from_nanos(ft))
            .collect();
        // Return result
        if frame_times.len() == frames {
            Ok(RenderTimeResult {
                triangles: 0_u32, // TODO implement triangle count
                frame_times,
            })
        } else {
            Err(Box::new(CError { return_code: 0 }))
        }
    }
}

// Helper function to reduce code repetition
fn str_to_cstring(input: &str) -> std::ffi::CString {
    return std::ffi::CString::new(input)
        .expect(&format!("Creation of CString failed from {}", input));
}
