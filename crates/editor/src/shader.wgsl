struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec3<f32>, 3>(
        vec3<f32>(-0.5, -0.5, -2.0),
        vec3<f32>( 0.5, -0.5, -2.0),
        vec3<f32>( 0.0,  0.5, -2.0),
    );
    let pos = vec4<f32>(positions[in_vertex_index], 1.0);
    return uniforms.view_proj * pos;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.5, 0.0, 1.0);
}