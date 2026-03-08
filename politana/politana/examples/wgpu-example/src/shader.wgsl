struct Uniforms {
    red_comp: f32,
    green_comp: f32
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexData {
    pos: vec2f,
    color: vec3f
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f
}

@vertex fn vertex(
    @builtin(vertex_index) index: u32
) -> VertexOutput {
    let vertex_data = array<VertexData, 4>(
        VertexData(vec2f(-1, -1), vec3f(uniforms.red_comp, 0, 1)),
        VertexData(vec2f(-1, 1), vec3f(uniforms.red_comp, 0, 1)),
        VertexData(vec2f(1, -1), vec3f(1, uniforms.green_comp, 0)),
        VertexData(vec2f(1, 1), vec3f(1, uniforms.green_comp, 0))
    );
    let v = vertex_data[index];
    return VertexOutput(vec4f(v.pos, 0, 1), vec4f(v.color, 1));
}

@fragment fn fragment(
    in: VertexOutput
) -> @location(0) vec4f {
    return in.color;
}
