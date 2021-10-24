use renderer::Renderer;
use std::path::PathBuf;
use svg_gen::Primitive;

pub fn write_frametimes_svgs<P>(
    renderer: &mut dyn Renderer,
    svg_dir: P,
    output: P,
    frames: usize,
) -> rendering::benching::error::Result<()>
where
    P: Into<PathBuf>,
{
    // TODO : Surely I shouldn't be re-exporting an identical function?
    Ok(rendering::benching::rendering::write_frametimes_svgs(
        renderer, svg_dir, output, frames,
    )?)
}

pub fn write_frametimes_primitives<P>(
    renderer: &mut dyn Renderer,
    primitives: &Vec<(String, Primitive)>,
    count: u32,
    output: P,
    frames: usize,
) -> rendering::benching::error::Result<()>
where
    P: Into<PathBuf>,
{
    // TODO : Surely I shouldn't be re-exporting an identical function?
    Ok(rendering::benching::rendering::write_frametimes_primitives(
        renderer, primitives, count, output, frames,
    )?)
}
