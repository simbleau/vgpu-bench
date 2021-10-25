pub mod artifacts;
pub mod targets;

pub trait Renderer {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn stage(&mut self, svg: &targets::SVGDocument) -> Result<(), Box<dyn std::error::Error>>;

    fn render(
        &mut self,
        frames: u64,
    ) -> Result<artifacts::RenderTimeResult, Box<dyn std::error::Error>>;
}
