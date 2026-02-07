use crate::simulation::{AgentType, Simulation, State as SimState};
use bytemuck::{Pod, Zeroable};
use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Point3, Vector3};
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct InstanceRaw {
    model_pos: [f32; 3],
    color: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let correction = Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
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
    instance_buffer: wgpu::Buffer,
    pub num_instances: u32,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_uniform: CameraUniform,
    pub camera: Camera,
    depth_view: wgpu::TextureView,
}

const VERTICES: &[Vertex] = &[
    // Cube vertices (same as before)
    Vertex {
        position: [-0.1, -0.1, 0.1],
        normal: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.1, -0.1, 0.1],
        normal: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.1, 0.1, 0.1],
        normal: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-0.1, 0.1, 0.1],
        normal: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-0.1, -0.1, -0.1],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [-0.1, 0.1, -0.1],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [0.1, 0.1, -0.1],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [0.1, -0.1, -0.1],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [-0.1, 0.1, -0.1],
        normal: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.1, 0.1, 0.1],
        normal: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.1, 0.1, 0.1],
        normal: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.1, 0.1, -0.1],
        normal: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [-0.1, -0.1, -0.1],
        normal: [0.0, -1.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1, -0.1],
        normal: [0.0, -1.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1, 0.1],
        normal: [0.0, -1.0, 0.0],
    },
    Vertex {
        position: [-0.1, -0.1, 0.1],
        normal: [0.0, -1.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1, -0.1],
        normal: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.1, 0.1, -0.1],
        normal: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.1, 0.1, 0.1],
        normal: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1, 0.1],
        normal: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.1, -0.1, -0.1],
        normal: [-1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.1, -0.1, 0.1],
        normal: [-1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.1, 0.1, 0.1],
        normal: [-1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.1, 0.1, -0.1],
        normal: [-1.0, 0.0, 0.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // Front
    4, 5, 6, 6, 7, 4, // Back
    8, 9, 10, 10, 11, 8, // Top
    12, 13, 14, 14, 15, 12, // Bottom
    16, 17, 18, 18, 19, 16, // Right
    20, 21, 22, 22, 23, 20, // Left
];

impl State {
    fn create_instances(sim: &Simulation) -> Vec<InstanceRaw> {
        let mut instances = Vec::with_capacity(sim.particles.len() + sim.agents.len());

        // Particles
        for p in &sim.particles {
            let t = p.temperature.clamp(0.0, 2.0) / 2.0;

            // Color based on State
            // Solid = Blue-ish, Liquid = Red-ish
            let (r, g, b) = match p.state {
                SimState::Solid => (0.1, 0.1 + t * 0.4, 0.8 + t * 0.2), // Icy Blue
                SimState::Liquid => (0.8 + t * 0.2, 0.1 + t * 0.5, 0.1), // Molten Red
            };

            instances.push(InstanceRaw {
                model_pos: [p.pos.x, p.pos.y, p.pos.z],
                color: [r, g, b],
            });
        }

        // Agents
        for agent in &sim.agents {
            let (r, g, b) = match agent.kind {
                AgentType::Cooler => (0.0, 1.0, 1.0), // Cyan
                AgentType::Heater => (1.0, 1.0, 0.0), // Yellow
            };

            instances.push(InstanceRaw {
                model_pos: [agent.pos.x, agent.pos.y, agent.pos.z],
                color: [r, g, b],
            });
        }

        instances
    }

    pub async fn new(
        window: Option<Arc<Window>>,
        width: u32,
        height: u32,
        sim: &Simulation,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = window
            .as_ref()
            .map(|w| instance.create_surface(w.clone()).unwrap());

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
            let surface_caps = surface.get_capabilities(&adapter);
            let format = surface_caps.formats[0];
            Some(wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: surface_caps.alpha_modes[0],
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
            eye: (40.0, 40.0, 40.0).into(), // Adjusted for grid visibility
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: width as f32 / height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 200.0,
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

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
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
        };

        let instance_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout, instance_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config
                        .as_ref()
                        .map(|c| c.format)
                        .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let instances = Self::create_instances(sim);
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

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
            num_indices: INDICES.len() as u32,
            instance_buffer,
            num_instances: instances.len() as u32,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            camera,
            depth_view,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.camera.aspect = new_size.0 as f32 / new_size.1 as f32;
            if let Some(config) = &mut self.config {
                config.width = new_size.0;
                config.height = new_size.1;
                if let Some(surface) = &self.surface {
                    surface.configure(&self.device, config);
                }
            }
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
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                label: Some("Depth Texture"),
                view_formats: &[],
            });
            self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    pub fn update(&mut self, sim: &Simulation) {
        self.camera_uniform.view_proj = self.camera.build_view_projection_matrix().into();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let instances = Self::create_instances(sim);
        // Important: this assumes instances count doesn't change, or buffer is big enough.
        // Simulation grid size is fixed, so this is safe.
        self.queue
            .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub async fn render_headless(&mut self) -> image::RgbaImage {
        let texture_size = wgpu::Extent3d {
            width: self.size.0,
            height: self.size.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Headless Texture"),
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Headless Encoder"),
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
                            g: 0.2,
                            b: 0.3,
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
        }

        let u32_size = std::mem::size_of::<u32>() as u32;
        let unpadded_bytes_per_row = u32_size * self.size.0;
        let align = 256;
        let padded_bytes_per_row_padding = (align - unpadded_bytes_per_row % align) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padded_bytes_per_row_padding;

        let output_buffer_size = (padded_bytes_per_row * self.size.1) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Output Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(self.size.1),
                },
            },
            texture_size,
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        let slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |v| {
            tx.send(v).unwrap();
        });
        self.device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().unwrap();

        let data = slice.get_mapped_range();
        let mut pixels: Vec<u8> =
            Vec::with_capacity((unpadded_bytes_per_row * self.size.1) as usize);
        for chunk in data.chunks(padded_bytes_per_row as usize) {
            pixels.extend_from_slice(&chunk[..unpadded_bytes_per_row as usize]);
        }

        let image = image::RgbaImage::from_raw(self.size.0, self.size.1, pixels).unwrap();
        drop(data);
        output_buffer.unmap();

        image
    }
}
