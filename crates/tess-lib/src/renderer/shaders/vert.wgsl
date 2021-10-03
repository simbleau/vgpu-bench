struct VertexOutput {
    [[location(0)]] v_color: vec4<f32>;
    [[builtin(position)]] member: vec4<f32>;
};

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
struct primitives_container {
    primitives: [[stride(16)]] array<Primitive,512>;
};
[[block]]
struct transforms_container {
    transforms: [[stride(32)]] array<Transform,512>;
};

[[group(0), binding(0)]]
var<uniform> global: Globals;
[[group(0), binding(1)]]
var<uniform> u_primitives: primitives_container;
[[group(0), binding(2)]]
var<uniform> u_transforms: transforms_container;

var<private> v_color: vec4<f32>;
var<private> v_position: vec4<f32>;

[[stage(vertex)]]
fn main([[location(0)]] a_position: vec2<f32>, [[location(1)]] a_prim_id: u32) -> VertexOutput {
    var prim: Primitive = u_primitives.primitives[a_prim_id];
    
    var t: Transform = u_transforms.transforms[prim.transform];
    var transform: mat3x3<f32> = mat3x3<f32>(
        vec3<f32>(t.data0_.x, t.data0_.y, 0.0), 
        vec3<f32>(t.data0_.z, t.data0_.w, 0.0), 
        vec3<f32>(t.data1_.x, t.data1_.y, 1.0)
    );
    
    var invert_y: vec2<f32> = vec2<f32>(1.0, -1.0);

    var pos: vec2<f32> = (transform * vec3<f32>(a_position, 1.0)).xy;
    v_position = vec4<f32>((((pos.xy + global.u_pan) * global.u_zoom) * invert_y), 0.0, 1.0);
    v_position.x = (v_position.x / global.u_aspect_ratio);


    var mask: u32 = 255u;
    var color = prim.color;

    v_color = vec4<f32>(
        f32(((color >> u32(24)) & mask)), 
        f32(((color >> u32(16)) & mask)), 
        f32(((color >> u32(8)) & mask)), 
        f32((color & mask))
        ) / vec4<f32>(255.0);

    return VertexOutput(v_color, v_position);
}
