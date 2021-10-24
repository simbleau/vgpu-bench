struct FragmentOutput {
    [[location(0)]] out_color: vec4<f32>;
};

[[stage(fragment)]]
fn main([[location(0)]] color: vec4<f32>) -> FragmentOutput {
    return FragmentOutput(color);
}