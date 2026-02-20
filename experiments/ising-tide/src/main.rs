use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey},
};
use wgpu::util::DeviceExt;
use cgmath::prelude::*;

const GRID_SIZE: u32 = 64;
const WORKGROUP_SIZE: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    width: u32,
    height: u32,
    depth: u32,
    _pad1: u32,

    time: f32,
    seed: f32,
    temperature: f32,
    field: f32,

    lattice_type: u32,
    brush_radius: f32,
    brush_strength: f32,
    _pad2: u32,

    brush_pos: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    position: [f32; 4],
}

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let correction = cgmath::Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        );
        correction * proj * view
    }
}

struct CameraController {
    speed: f32,
    is_drag_rotate: bool,
    lat: f32,
    lon: f32,
    radius: f32,
}

impl CameraController {
    fn new(radius: f32) -> Self {
        Self {
            speed: 0.2,
            is_drag_rotate: false,
            lat: 0.0,
            lon: 0.0,
            radius,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        let lat_rad = self.lat.to_radians();
        let lon_rad = self.lon.to_radians();

        let x = self.radius * lat_rad.cos() * lon_rad.sin();
        let y = self.radius * lat_rad.sin();
        let z = self.radius * lat_rad.cos() * lon_rad.cos();

        camera.eye = cgmath::Point3::new(x, y, z);
    }

    fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { state, button: MouseButton::Right, .. } => {
                self.is_drag_rotate = *state == ElementState::Pressed;
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.radius -= y * self.speed * 2.0;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.radius -= pos.y as f32 * self.speed * 0.1;
                    }
                }
                if self.radius < 1.0 { self.radius = 1.0; }
                true
            }
            _ => false,
        }
    }

    fn process_device_event(&mut self, event: &DeviceEvent) {
        if self.is_drag_rotate {
            if let DeviceEvent::MouseMotion { delta } = event {
                self.lon -= delta.0 as f32 * self.speed;
                self.lat -= delta.1 as f32 * self.speed;

                if self.lat > 89.0 { self.lat = 89.0; }
                if self.lat < -89.0 { self.lat = -89.0; }
            }
        }
    }
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: std::sync::Arc<winit::window::Window>,

    render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,

    cell_buffers: [wgpu::Buffer; 2],
    compute_bind_groups: [wgpu::BindGroup; 2],
    render_cell_bind_groups: [wgpu::BindGroup; 2],

    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_compute: wgpu::BindGroup,
    uniform_bind_group_render: wgpu::BindGroup,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,

    uniforms: Uniforms,

    frame_count: u64,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    num_instances: u32,
}

const VERTICES: &[f32] = &[
    // Front face
    -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
     0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
    // Back face
    -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
    -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
     0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
    // Top face
    -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
    // Bottom face
    -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
    // Right face
     0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
    // Left face
    -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
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
    async fn new(window: std::sync::Arc<winit::window::Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
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

        let num_cells = (GRID_SIZE * GRID_SIZE * GRID_SIZE) as usize;

        let mut rng = rand::thread_rng();
        use rand::Rng;
        let mut initial_cells = Vec::with_capacity(num_cells);
        for _ in 0..num_cells {
            let spin = if rng.gen_bool(0.5) { 1.0f32 } else { -1.0f32 };
            initial_cells.push(spin);
        }

        let cell_buffer_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cell Buffer A"),
            contents: bytemuck::cast_slice(&initial_cells),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let cell_buffer_b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cell Buffer B"),
            contents: bytemuck::cast_slice(&initial_cells),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let cell_buffers = [cell_buffer_a, cell_buffer_b];

        let uniforms = Uniforms {
            width: GRID_SIZE,
            height: GRID_SIZE,
            depth: GRID_SIZE,
            _pad1: 0,
            time: 0.0,
            seed: 0.0,
            temperature: 4.5, // Start near Critical Temp
            field: 0.0,
            lattice_type: 0,
            brush_radius: 5.0,
            brush_strength: 0.0, // Start with brush OFF
            _pad2: 0,
            brush_pos: [0.0, 0.0, 0.0, 0.0],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera = Camera {
            eye: (GRID_SIZE as f32 * 1.5, GRID_SIZE as f32 * 1.5, GRID_SIZE as f32 * 1.5).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 1000.0,
        };

        let mut camera_uniform = CameraUniform {
            view_proj: [[0.0; 4]; 4],
            position: [0.0; 4],
        };
        camera_uniform.view_proj = camera.build_view_projection_matrix().into();
        camera_uniform.position = [camera.eye.x, camera.eye.y, camera.eye.z, 1.0];

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // --- BIND GROUPS ---

        let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("compute_bind_group_layout"),
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let compute_bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: cell_buffers[0].as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: cell_buffers[1].as_entire_binding() },
            ],
            label: Some("compute_bind_group_0"),
        });

        let compute_bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &compute_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: cell_buffers[1].as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: cell_buffers[0].as_entire_binding() },
            ],
            label: Some("compute_bind_group_1"),
        });

        let uniform_bind_group_compute = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
            ],
            label: Some("uniform_bind_group_compute"),
        });

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() },
            ],
            label: Some("camera_bind_group"),
        });

        let render_cell_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("render_cell_bind_group_layout"),
        });

        let render_cell_bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_cell_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: cell_buffers[0].as_entire_binding() },
            ],
            label: Some("render_cell_bind_group_0"),
        });

        let render_cell_bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &render_cell_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: cell_buffers[1].as_entire_binding() },
            ],
            label: Some("render_cell_bind_group_1"),
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&compute_bind_group_layout, &uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "compute_main",
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &render_cell_bind_group_layout,
                &uniform_bind_group_layout,
            ],
            push_constant_ranges: &[],
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

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: 6 * 4,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: 3 * 4,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                ],
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
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let uniform_bind_group_render = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
            ],
            label: Some("uniform_bind_group_render"),
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            compute_pipeline,
            cell_buffers,
            compute_bind_groups: [compute_bind_group_0, compute_bind_group_1],
            render_cell_bind_groups: [render_cell_bind_group_0, render_cell_bind_group_1],
            uniform_buffer,
            uniform_bind_group_compute,
            uniform_bind_group_render,
            camera_buffer,
            camera_bind_group,
            camera,
            camera_controller: CameraController::new(GRID_SIZE as f32 * 2.0),
            camera_uniform,
            uniforms,
            frame_count: 0,
            vertex_buffer,
            index_buffer,
            num_indices: INDICES.len() as u32,
            num_instances: num_cells as u32,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        if self.camera_controller.process_events(event) {
            return true;
        }

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(keycode),
                        ..
                    },
                ..
            } => {
                match keycode {
                    KeyCode::ArrowUp => {
                        self.uniforms.temperature += 0.1;
                        println!("Temperature: {:.1}", self.uniforms.temperature);
                        true
                    }
                    KeyCode::ArrowDown => {
                        self.uniforms.temperature -= 0.1;
                        if self.uniforms.temperature < 0.0 { self.uniforms.temperature = 0.0; }
                        println!("Temperature: {:.1}", self.uniforms.temperature);
                        true
                    }
                    KeyCode::ArrowRight => {
                        self.uniforms.field += 0.1;
                        println!("Field: {:.1}", self.uniforms.field);
                        true
                    }
                    KeyCode::ArrowLeft => {
                        self.uniforms.field -= 0.1;
                        println!("Field: {:.1}", self.uniforms.field);
                        true
                    }
                    KeyCode::Digit1 => {
                        self.uniforms.lattice_type = 0;
                        println!("Lattice: Simple Cubic");
                        true
                    }
                    KeyCode::Digit2 => {
                        self.uniforms.lattice_type = 1;
                        println!("Lattice: BCC (Topology)");
                        true
                    }
                    KeyCode::Digit3 => {
                        self.uniforms.lattice_type = 2;
                        println!("Lattice: FCC (Topology)");
                        true
                    }
                    KeyCode::Space => {
                        if self.uniforms.brush_strength > 0.0 {
                            self.uniforms.brush_strength = 0.0;
                            println!("Brush: OFF");
                        } else {
                            self.uniforms.brush_strength = 5.0;
                            println!("Brush: ON");
                        }
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        self.uniforms.time += 0.016;
        self.uniforms.seed = self.frame_count as f32;

        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.view_proj = self.camera.build_view_projection_matrix().into();
        self.camera_uniform.position = [self.camera.eye.x, self.camera.eye.y, self.camera.eye.z, 1.0];

        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
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

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            let idx = (self.frame_count % 2) as usize;
            compute_pass.set_bind_group(0, &self.compute_bind_groups[idx], &[]);
            compute_pass.set_bind_group(1, &self.uniform_bind_group_compute, &[]);

            let num_cells = GRID_SIZE * GRID_SIZE * GRID_SIZE;
            let workgroups = (num_cells + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
            compute_pass.dispatch_workgroups(workgroups, 1, 1);
        }

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
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
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

            let result_idx = ((self.frame_count % 2) ^ 1) as usize;
            render_pass.set_bind_group(1, &self.render_cell_bind_groups[result_idx], &[]);
            render_pass.set_bind_group(2, &self.uniform_bind_group_render, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = std::sync::Arc::new(WindowBuilder::new().build(&event_loop).unwrap());

    let mut state = pollster::block_on(State::new(window.clone()));

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => elwt.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::DeviceEvent { event, .. } => {
                state.camera_controller.process_device_event(&event);
            }
            Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
