use crate::math::{self, Rhombus, TriangleType};
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Point3, Vector3};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct PlayerUniform {
    position: [f32; 2],
    radius: f32,
    padding: f32,
}

pub struct Camera {
    pub position: Point3<f32>,
    pub width: f32,
    pub height: f32,
    pub zoom: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        // Orthographic projection
        let left = -self.width / 2.0 * self.zoom;
        let right = self.width / 2.0 * self.zoom;
        let bottom = -self.height / 2.0 * self.zoom;
        let top = self.height / 2.0 * self.zoom;
        let near = -100.0;
        let far = 100.0;

        let proj = cgmath::ortho(left, right, bottom, top, near, far);

        // LookAt matrix: Camera at (x, y, 10), looking at (x, y, 0), Up (0, 1, 0)
        let view = Matrix4::look_at_rh(
            self.position,
            Point3::new(self.position.x, self.position.y, 0.0),
            Vector3::unit_y(),
        );

        // Standard WGPU correction for NDC depth range 0..1
        let correction = Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );

        correction * proj * view
    }
}

pub struct State {
    surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_uniform: CameraUniform,
    pub camera: Camera,
    player_buffer: wgpu::Buffer,
    player_bind_group: wgpu::BindGroup,
    player_uniform: PlayerUniform,
    pub player_pos: [f32; 2], // Current player position
    pub rhombuses: Vec<Rhombus>, // Store tiling data
    pub adjacency: Vec<Vec<usize>>, // Store graph
    depth_view: wgpu::TextureView,
}

impl State {
    pub async fn new(window: Option<Arc<Window>>, width: u32, height: u32) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = window.as_ref().map(|w| instance.create_surface(w.clone()).unwrap());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = if let Some(surface) = &surface {
            let caps = surface.get_capabilities(&adapter);
            let format = caps.formats[0];
            Some(wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            })
        } else {
            None
        };

        if let (Some(surface), Some(config)) = (&surface, &config) {
            surface.configure(&device, config);
        }

        // Camera
        let camera = Camera {
            position: (0.0, 0.0, 10.0).into(),
            width: width as f32,
            height: height as f32,
            zoom: 0.1, // Zoom level (units per pixel?) 0.1 means 10 units fit in screen? No.
                       // With ortho(left=-W/2 * zoom ...), width in world units = W * zoom.
                       // So if W=800, zoom=0.1 -> World Width = 80.
        };

        let mut camera_uniform = CameraUniform {
            view_proj: [[0.0; 4]; 4],
        };
        camera_uniform.view_proj = camera.build_view_projection_matrix().into();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Player Uniform
        let player_uniform = PlayerUniform {
            position: [0.0, 0.0],
            radius: 50.0, // Initial Fog Radius
            padding: 0.0,
        };

        let player_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Player Buffer"),
            contents: bytemuck::cast_slice(&[player_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let player_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("player_bind_group_layout"),
            });

        let player_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &player_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: player_buffer.as_entire_binding(),
            }],
            label: Some("player_bind_group"),
        });

        // Pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &player_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config
                        .as_ref()
                        .map(|c| c.format)
                        .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Disable culling for 2D just in case winding is flipped
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Generate Tiling
        let (rhombuses, adjacency) = math::generate_tiling(5); // 5 iterations -> ~3k tiles?

        let mut vertices = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut index_counter = 0;

        for r in &rhombuses {
            // Rhombus vertices are [A, B, C, D] (cyclic)
            // Triangles: (A, B, C) and (A, C, D)?
            // math::Rhombus vertices order: [t1.A, t1.B, t2.A, t1.C]
            // Wait, I put logic in math.rs: [t1.v0, t1.v1, t2.v0, t1.v2]
            // t1 is Triangle(A, B, C). t2 is Triangle(A, B, C).
            // Shared edge is B-C.
            // If t1 vertices are A, B, C.
            // t2 vertices are A', B', C'. (Actually A', C', B' if mirrored?)
            // This is tricky.
            // Let's rely on `r.vertices`.
            // Just assume they form a convex quad.
            // Triangulate as (0, 1, 2) and (0, 2, 3) or (0, 1, 3) and (1, 2, 3).
            // Since it's a Rhombus, it's convex. Any diagonal works.

            // Color based on type
            let color = match r.r_type {
                TriangleType::Acute => [0.2, 0.4, 0.8], // Thick Blue
                TriangleType::Obtuse => [0.8, 0.2, 0.2], // Thin Red
            };

            for v in &r.vertices {
                vertices.push(Vertex {
                    position: [v.x, v.y, 0.0],
                    color,
                });
            }

            // Indices for quad (0, 1, 2, 3) -> (0, 1, 2), (2, 3, 0)
            indices.push(index_counter + 0);
            indices.push(index_counter + 1);
            indices.push(index_counter + 2);

            indices.push(index_counter + 2);
            indices.push(index_counter + 3);
            indices.push(index_counter + 0);

            index_counter += 4;
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Depth Texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("Depth Texture"),
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            camera,
            player_buffer,
            player_bind_group,
            player_uniform,
            player_pos: [0.0, 0.0],
            rhombuses,
            adjacency,
            depth_view,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.camera.width = new_size.0 as f32;
            self.camera.height = new_size.1 as f32;

            if let Some(config) = &mut self.config {
                config.width = new_size.0;
                config.height = new_size.1;
                if let Some(surface) = &self.surface {
                    surface.configure(&self.device, config);
                }
            }

            // Recreate depth view
            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: new_size.0,
                    height: new_size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("Depth Texture"),
                view_formats: &[],
            });
            self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn update(&mut self) {
        // Update Camera
        self.camera_uniform.view_proj = self.camera.build_view_projection_matrix().into();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // Update Player Uniform (Fog Center)
        self.player_uniform.position = self.player_pos;
        self.queue.write_buffer(
            &self.player_buffer,
            0,
            bytemuck::cast_slice(&[self.player_uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = if let Some(surface) = &self.surface {
            surface.get_current_texture()?
        } else {
            return Ok(());
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.player_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
