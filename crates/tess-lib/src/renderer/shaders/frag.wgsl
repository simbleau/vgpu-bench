struct FragmentOutput {
    [[location(0)]] out_color: vec4<f32>;
};

var<private> v_color: vec4<f32>;
var<private> out_color: vec4<f32>;

[[stage(fragment)]]
fn main([[location(0)]] v_color: vec4<f32>) -> FragmentOutput {
    out_color = v_color;
    return FragmentOutput(out_color);
}