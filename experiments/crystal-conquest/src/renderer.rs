use crate::simulation::Simulation;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GlobalUniform {
    view_proj: [[f32; 4]; 4],
    inv_view_proj: [[f32; 4]; 4],
    view_pos: [f32; 4],
    time: f32,
    _pad: f32,
    resolution: [f32; 2],
}

pub struct Renderer {
    render_pipeline: wgpu::RenderPipeline,
    particle_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group_0: wgpu::BindGroup,
    bind_group_1: wgpu::BindGroup,
    start_time: std::time::Instant,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        simulation: &Simulation,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Uniform Buffer
        let uniform_size = std::mem::size_of::<GlobalUniform>() as wgpu::BufferAddress;
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: uniform_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Particle Buffer

        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particle Buffer"),
            contents: bytemuck::cast_slice(&simulation.particles),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        // Bind Group Layouts
        let bind_group_layout_0 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let bind_group_layout_1 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("particle_bind_group_layout"),
            });

        // Pipeline Layout
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1],
                push_constant_ranges: &[],
            });

        // Render Pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // No culling for full screen triangle
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Bind Groups
        let bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout_0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout_1,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: particle_buffer.as_entire_binding(),
            }],
            label: Some("particle_bind_group"),
        });

        Self {
            render_pipeline,
            particle_buffer,
            uniform_buffer,
            bind_group_0,
            bind_group_1,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn resize(&mut self, _device: &wgpu::Device, _config: &wgpu::SurfaceConfiguration) {
        // Handle resize if needed (e.g. depth buffer)
    }

    pub fn update(
        &mut self,
        queue: &wgpu::Queue,
        simulation: &Simulation,
        yaw: f32,
        pitch: f32,
        dist: f32,
        size: winit::dpi::PhysicalSize<u32>,
    ) {
        // Update particles
        queue.write_buffer(
            &self.particle_buffer,
            0,
            bytemuck::cast_slice(&simulation.particles),
        );

        // Camera Math
        let target = Point3::new(0.0, 0.0, 0.0);
        // Spherical coords for camera pos
        let cam_x = dist * pitch.cos() * yaw.sin();
        let cam_y = dist * pitch.sin();
        let cam_z = dist * pitch.cos() * yaw.cos();
        let eye = Point3::new(cam_x, cam_y, cam_z);

        let view = Matrix4::look_at_rh(eye, target, Vector3::unit_y());
        let aspect = size.width as f32 / size.height as f32;
        let proj = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);
        let view_proj = proj * view;
        let inv_view_proj = view_proj.invert().unwrap_or(Matrix4::identity());

        let time = self.start_time.elapsed().as_secs_f32();

        let uniform = GlobalUniform {
            view_proj: view_proj.into(),
            inv_view_proj: inv_view_proj.into(),
            view_pos: [eye.x, eye.y, eye.z, 1.0],
            time,
            _pad: 0.0,
            resolution: [size.width as f32, size.height as f32],
        };

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn render(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
        render_pass.set_bind_group(1, &self.bind_group_1, &[]);
        render_pass.draw(0..3, 0..1); // Draw 3 vertices (full screen triangle)
    }
}
