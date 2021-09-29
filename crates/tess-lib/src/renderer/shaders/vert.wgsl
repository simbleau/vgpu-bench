[[block]]
struct Globals {
    u_zoom: vec2<f32>;
    u_pan: vec2<f32>;
    u_aspect_ratio: f32;
};

struct Primitive {
    transform: u32;
    color: u32;
    pad: vec2<u32>;
};

struct Transform {
    data0_: vec4<f32>;
    data1_: vec4<f32>;
};

[[block]]
struct u_primitives {
    primitives: [[stride(16)]] array<Primitive,512>;
};

[[block]]
struct u_transforms {
    transforms: [[stride(32)]] array<Transform,512>;
};

struct VertexOutput {
    [[location(0)]] v_color: vec4<f32>;
    [[builtin(position)]] member: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> global: Globals;
[[group(0), binding(1)]]
var<uniform> global1: u_primitives;
[[group(0), binding(2)]]
var<uniform> global2: u_transforms;
var<private> a_position1: vec2<f32>;
var<private> a_prim_id1: u32;
var<private> v_color: vec4<f32>;
var<private> gl_Position: vec4<f32>;

fn main1() {
    var prim: Primitive;
    var t: Transform;
    var transform: mat3x3<f32>;
    var invert_y: vec2<f32> = vec2<f32>(1.0, -1.0);
    var pos: vec2<f32>;
    var mask: u32 = 255u;
    var color: u32;

    let e13: u32 = a_prim_id1;
    let e15: Primitive = global1.primitives[e13];
    prim = e15;
    let e17: Primitive = prim;
    let e20: Transform = global2.transforms[e17.transform];
    t = e20;
    let e22: Transform = t;
    let e25: Transform = t;
    let e29: Transform = t;
    let e32: Transform = t;
    let e36: Transform = t;
    let e39: Transform = t;
    transform = mat3x3<f32>(vec3<f32>(e22.data0_.x, e25.data0_.y, 0.0), vec3<f32>(e29.data0_.z, e32.data0_.w, 0.0), vec3<f32>(e36.data1_.x, e39.data1_.y, 1.0));
    let e53: mat3x3<f32> = transform;
    let e54: vec2<f32> = a_position1;
    pos = (e53 * vec3<f32>(e54, 1.0)).xy;
    let e61: vec2<f32> = pos;
    let e63: vec2<f32> = global.u_pan;
    let e65: vec2<f32> = global.u_zoom;
    let e67: vec2<f32> = invert_y;
    gl_Position = vec4<f32>((((e61.xy + e63) * e65) * e67), 0.0, 1.0);
    let e73: vec4<f32> = gl_Position;
    let e75: f32 = global.u_aspect_ratio;
    gl_Position.x = (e73.x / e75);
    let e79: Primitive = prim;
    color = e79.color;
    let e82: u32 = color;
    let e86: u32 = mask;
    let e89: u32 = color;
    let e93: u32 = mask;
    let e96: u32 = color;
    let e100: u32 = mask;
    let e103: u32 = color;
    let e104: u32 = mask;
    v_color = (vec4<f32>(f32(((e82 >> u32(24)) & e86)), f32(((e89 >> u32(16)) & e93)), f32(((e96 >> u32(8)) & e100)), f32((e103 & e104))) / vec4<f32>(255.0));
    return;
}

[[stage(vertex)]]
fn main([[location(0)]] a_position: vec2<f32>, [[location(1)]] a_prim_id: u32) -> VertexOutput {
    a_position1 = a_position;
    a_prim_id1 = a_prim_id;
    main1();
    let e21: vec4<f32> = v_color;
    let e23: vec4<f32> = gl_Position;
    return VertexOutput(e21, e23);
}
