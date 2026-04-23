use wgpu::util::DeviceExt;
use map_format::brush::{Brush, Plane};
use map_format::types::Vec3;
use map_format::geometry::brush_to_mesh;
use uuid::Uuid;

pub struct ViewportRenderer {
    pipeline: wgpu::RenderPipeline,
    outline_pipeline: wgpu::RenderPipeline,
    pub uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl ViewportRenderer {
    pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("viewport_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&[0f32; 24]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create a test box brush
        let mut brush = Brush::new(Uuid::new_v4());
        brush.planes = vec![
            Plane { normal: Vec3 { x:  1.0, y: 0.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: Vec3 { x: -1.0, y: 0.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: Vec3 { x: 0.0, y:  1.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: Vec3 { x: 0.0, y: -1.0, z: 0.0 }, distance:  1.0 },
            Plane { normal: Vec3 { x: 0.0, y: 0.0, z:  1.0 }, distance:  1.0 },
            Plane { normal: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, distance:  1.0 },
        ];

        let mesh = brush_to_mesh(&brush);

        // Interleave: [x, y, z, nx, ny, nz, snx, sny, snz]
        let interleaved: Vec<[f32; 9]> = mesh.vertices.iter()
            .zip(mesh.normals.iter())
            .zip(mesh.smooth_normals.iter())
            .map(|((v, n), sn)| [
                v[0], v[1], v[2],
                n[0], n[1], n[2],
                sn[0], sn[1], sn[2],
            ])
            .collect();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(&interleaved),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(&mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = mesh.indices.len() as u32;

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: 36,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("viewport_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buffer_layout.clone()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let outline_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("outline_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_outline"),
                buffers: &[vertex_buffer_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_outline"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            outline_pipeline,
            uniform_buffer,
            bind_group,
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }

    pub fn paint(&self, render_pass: &mut wgpu::RenderPass, selected: bool) {
        if selected {
            render_pass.set_pipeline(&self.outline_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.index_count, 0, 0..1);
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

pub struct ViewportCallback {
    pub view_proj: glam::Mat4,
    pub camera_pos: glam::Vec3,
    pub selected: bool,
    pub selected_center: Option<glam::Vec3>,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        if let Some(renderer) = callback_resources.get::<ViewportRenderer>() {
            let mut data = [0f32; 24];
            let vp = self.view_proj.to_cols_array();
            data[..16].copy_from_slice(&vp);
            data[16] = if self.selected { 1.0 } else { 0.0 };
            queue.write_buffer(
                &renderer.uniform_buffer,
                0,
                bytemuck::cast_slice(&data),
            );
        }

        if let Some(grid) = callback_resources.get::<crate::grid::GridRenderer>() {
            let mut data = [0f32; 20];
            let vp = self.view_proj.to_cols_array();
            data[..16].copy_from_slice(&vp);
            data[16] = self.camera_pos.x;
            data[17] = self.camera_pos.y;
            data[18] = self.camera_pos.z;
            data[19] = 0.0;
            queue.write_buffer(&grid.uniform_buffer, 0, bytemuck::cast_slice(&data));
        }

        Vec::new()
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        callback_resources: &egui_wgpu::CallbackResources,
    ) {
        if let Some(grid) = callback_resources.get::<crate::grid::GridRenderer>() {
            grid.paint(render_pass);
        }
        if let Some(renderer) = callback_resources.get::<ViewportRenderer>() {
            renderer.paint(render_pass, self.selected);
        }
    }
}