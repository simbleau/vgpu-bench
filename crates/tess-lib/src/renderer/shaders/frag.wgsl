struct FragmentOutput {
    [[location(0)]] out_color: vec4<f32>;
};

var<private> v_color1: vec4<f32>;
var<private> out_color: vec4<f32>;

fn main1() {
    let e2: vec4<f32> = v_color1;
    out_color = e2;
    return;
}

[[stage(fragment)]]
fn main([[location(0)]] v_color: vec4<f32>) -> FragmentOutput {
    v_color1 = v_color;
    main1();
    let e7: vec4<f32> = out_color;
    return FragmentOutput(e7);
}
