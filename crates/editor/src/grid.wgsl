struct Uniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec3<f32>,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
};

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_proj * vec4<f32>(position, 1.0);
    out.world_pos = position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = in.world_pos.x;
    let z = in.world_pos.z;

    // Distance from camera for fade
    let dist = length(in.world_pos - uniforms.camera_pos);
    let fade = 1.0 - smoothstep(40.0, 80.0, dist);

    // Minor grid lines - every 1 unit
    let minor_x = abs(fract(x - 0.5) - 0.5) / fwidth(x);
    let minor_z = abs(fract(z - 0.5) - 0.5) / fwidth(z);
    let minor_line = min(minor_x, minor_z);

    // Major grid lines - every 10 units
    let major_x = abs(fract(x / 10.0 - 0.5) - 0.5) / fwidth(x / 10.0);
    let major_z = abs(fract(z / 10.0 - 0.5) - 0.5) / fwidth(z / 10.0);
    let major_line = min(major_x, major_z);

    // Base colour - dark background
    var colour = vec4<f32>(0.12, 0.12, 0.12, 0.0);

    // Minor lines - dark grey
    if minor_line < 1.0 {
        colour = vec4<f32>(0.25, 0.25, 0.25, (1.0 - minor_line) * fade);
    }

    // Major lines - lighter grey, override minor
    if major_line < 1.0 {
        colour = vec4<f32>(0.4, 0.4, 0.4, (1.0 - major_line) * fade);
    }

    return colour;
}