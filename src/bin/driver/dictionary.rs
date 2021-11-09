use const_format::concatcp;

pub const OUTPUT_DIR: &'static str = "output/data/";
pub const SVG_OUTPUT_DIR: &'static str = concatcp![OUTPUT_DIR, "svg/"];
pub const PRIMITIVES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "primitives/"];
pub const EXAMPLES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "examples/"];

pub const ASSETS_DIR: &'static str = "assets/";
pub const SVG_ASSETS_DIR: &'static str = concatcp![ASSETS_DIR, "svg/"];
pub const PRIMITIVES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "primitives/"];
pub const EXAMPLES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "examples/"];
