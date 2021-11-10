#![feature(format_args_capture)]
#![feature(fn_traits)]

mod dictionary;
mod naive_rendering_benchmarks;
mod tessellation_benchmarks;

mod driver;
use dictionary::EXAMPLES_ASSETS_DIR;
use driver::Driver;
mod run_options;
use run_options::RunOptions;

use log::LevelFilter;

pub fn main() {
    Driver::from(
        RunOptions::builder()
            .logging(LevelFilter::Trace)
            // TODO add bench_tessellation_examples()
            .add(|| tessellation_benchmarks::bench_tessellation_primitives())
            .add(|| tessellation_benchmarks::profile_svg_examples(EXAMPLES_ASSETS_DIR))
            .add(|| tessellation_benchmarks::profile_svg_primitives())
            .add(|| naive_rendering_benchmarks::frametimes_svg_examples(EXAMPLES_ASSETS_DIR))
            .add(|| naive_rendering_benchmarks::frametimes_svg_primitives())
            .build(),
    )
    .run();
}
