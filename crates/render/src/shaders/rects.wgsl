// renders rectangles

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vert_main(
    @builtin(vertex_index) vert_i: u32,
    @location(0) rect_pos: vec2<f32>,
    @location(1) rect_size: vec2<f32>,
    @location(2) rect_color: vec4<f32>, // rgba
) -> VertexOut {
    let u = f32(vert_i & 1u);
    let v = f32((vert_i >> 1u) & 1u);
    // let uv = vec2<f32>(f32(vert_i & 1u), f32((vert_i >> 1u) & 1u));
    //let pix_pos = (uv * rect_size) + rect_pos;
    let pix_pos = vec2<f32>(u * 100.0, v * 100.0);

    var out: VertexOut;
    out.pos = vec4<f32>(pix_pos, 1.0, 1.0);
    out.color = rect_color;
    return out;
}

@fragment
fn frag_main(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 0.0);
}
