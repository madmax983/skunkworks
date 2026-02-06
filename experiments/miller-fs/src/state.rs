use crate::camera::{Camera, CameraController};
use crate::lattice::Crystal;
use cgmath::prelude::*;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
}

const VERTICES: &[Vertex] = &[
    // Front face
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    // Back face
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    // Top face
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    // Bottom face
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    // Right face
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, -0.5],
    },
    Vertex {
        position: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.5],
    },
    // Left face
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, -0.5],
    },
    Vertex {
        position: [-0.5, 0.5, 0.5],
    },
    Vertex {
        position: [-0.5, 0.5, -0.5],
    },
];

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model_pos: [f32; 3],
    color: [f32; 4],
    scale: [f32; 3],
    rotation: [f32; 4],
}

impl InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    // model_pos
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // color
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    // scale
                    offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // rotation
                    offset: (std::mem::size_of::<[f32; 3]>() * 2 + std::mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Arc<Window>,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,

    camera: Camera,
    camera_controller: CameraController,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,

    // Miller indices
    pub miller_indices: [i32; 3],
    plane_buffer: wgpu::Buffer,
}

impl State {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub async fn new(window: Arc<Window>, crystal: Crystal) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

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

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("depth_texture"),
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // --- Camera Setup ---
        let camera = Camera {
            eye: (0.0, 5.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
        };
        let camera_controller = CameraController::new(0.5);

        let camera_uniform = CameraUniform {
            view_proj: camera.build_view_projection_matrix().into(),
        };

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

        // --- Instance Data ---
        let instances = crystal
            .atoms
            .iter()
            .map(|atom| {
                let color = if atom.is_dir {
                    [0.0, 0.8, 0.8, 1.0]
                } else {
                    [0.8, 0.2, 0.2, 1.0]
                };
                let scale = if atom.is_dir {
                    [0.4, 0.4, 0.4]
                } else {
                    [0.2, 0.2, 0.2]
                };

                InstanceRaw {
                    model_pos: [
                        atom.position.x as f32,
                        atom.position.y as f32,
                        atom.position.z as f32,
                    ],
                    color,
                    scale,
                    rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
                }
            })
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_instances = instances.len() as u32;

        // --- Miller Plane ---
        let plane_instance = InstanceRaw {
            model_pos: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0], // Invisible initially
            scale: [100.0, 100.0, 0.05],
            rotation: [0.0, 0.0, 0.0, 1.0],
        };
        let plane_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Plane Buffer"),
            contents: bytemuck::cast_slice(&[plane_instance]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        // --- Pipeline ---
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Disable culling for plane visibility from both sides
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Self::DEPTH_FORMAT,
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            instance_buffer,
            num_instances,
            camera,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            depth_texture,
            depth_view,
            miller_indices: [0, 0, 0],
            plane_buffer,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: new_size.width,
                    height: new_size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("depth_texture"),
                view_formats: &[],
            });
            self.depth_view = self
                .depth_texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: key_event, ..
            } => {
                if key_event.state == ElementState::Pressed {
                    match key_event.physical_key {
                        PhysicalKey::Code(KeyCode::KeyU) => self.miller_indices[0] += 1,
                        PhysicalKey::Code(KeyCode::KeyI) => self.miller_indices[0] -= 1,
                        PhysicalKey::Code(KeyCode::KeyJ) => self.miller_indices[1] += 1,
                        PhysicalKey::Code(KeyCode::KeyK) => self.miller_indices[1] -= 1,
                        PhysicalKey::Code(KeyCode::KeyN) => self.miller_indices[2] += 1,
                        PhysicalKey::Code(KeyCode::KeyM) => self.miller_indices[2] -= 1,
                        _ => return self.camera_controller.process_events(key_event),
                    }
                    println!("Miller Indices: {:?}", self.miller_indices);
                    true
                } else {
                    self.camera_controller.process_events(key_event)
                }
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        let camera_uniform = CameraUniform {
            view_proj: self.camera.build_view_projection_matrix().into(),
        };
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // Update Plane
        let h = self.miller_indices[0] as f32;
        let k = self.miller_indices[1] as f32;
        let l = self.miller_indices[2] as f32;
        let normal = cgmath::Vector3::new(h, k, l);

        let (rotation, color) = if normal.magnitude() < 0.001 {
            ([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 0.0])
        } else {
            let normal = normal.normalize();
            let up = cgmath::Vector3::unit_z(); // Initial plane normal
                                                // Rotation from up to normal
            let rotation = if normal.dot(up).abs() > 0.999 {
                if normal.z > 0.0 {
                    cgmath::Quaternion::one()
                } else {
                    cgmath::Quaternion::from_axis_angle(
                        cgmath::Vector3::unit_x(),
                        cgmath::Deg(180.0),
                    )
                }
            } else {
                let axis = up.cross(normal).normalize();
                let angle = up.dot(normal).acos();
                cgmath::Quaternion::from_axis_angle(axis, cgmath::Rad(angle))
            };

            (
                [rotation.v.x, rotation.v.y, rotation.v.z, rotation.s],
                [1.0, 1.0, 1.0, 0.3],
            )
        };

        let plane_instance = InstanceRaw {
            model_pos: [0.0, 0.0, 0.0],
            color,
            scale: [50.0, 50.0, 0.05],
            rotation,
        };
        self.queue.write_buffer(
            &self.plane_buffer,
            0,
            bytemuck::cast_slice(&[plane_instance]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
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
                            r: 0.1,
                            g: 0.1,
                            b: 0.2,
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
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

            // Draw Atoms
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..self.num_instances);

            // Draw Plane
            render_pass.set_vertex_buffer(1, self.plane_buffer.slice(..));
            render_pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
