struct Uniforms {
    view_proj: mat4x4<f32>,
    selected: f32,
    _padding: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) smooth_normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    return out;
}

@vertex
fn vs_outline(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let clip_pos = uniforms.view_proj * vec4<f32>(in.position, 1.0);
    let clip_normal = uniforms.view_proj * vec4<f32>(in.smooth_normal, 0.0);
    let offset = normalize(clip_normal.xy) * 0.005 * clip_pos.w;
    out.clip_position = vec4<f32>(clip_pos.xy + offset, clip_pos.zw);
    out.normal = in.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light = dot(normalize(in.normal), normalize(vec3<f32>(1.0, 2.0, 1.0))) * 0.5 + 0.5;
    let base = vec4<f32>(0.4 * light, 0.8 * light, 0.4 * light, 1.0);
    let tint = vec4<f32>(1.0, 0.5, 0.1, 1.0);
    return mix(base, tint, uniforms.selected * 0.25);
}

@fragment
fn fs_outline(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.5, 0.0, 1.0);
}