use std::sync::Arc;
use std::time::{Duration, Instant};
use sysinfo::System;
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalPosition,
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 1024;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    feed: f32,
    kill: f32,
    dt: f32,
    diff_u: f32,
    diff_v: f32,
    _padding1: f32,
    _padding2: f32,
    _padding3: f32,
}

struct SystemMonitor {
    sys: System,
    last_update: Instant,
}

impl SystemMonitor {
    fn new() -> Self {
        Self {
            sys: System::new_all(),
            last_update: Instant::now(),
        }
    }

    fn refresh(&mut self) -> (f32, f32) {
        if self.last_update.elapsed() >= Duration::from_millis(500) {
            self.sys.refresh_all();
            self.last_update = Instant::now();
        }

        let cpu_usage = self.sys.global_cpu_info().cpu_usage(); // 0.0 to 100.0
        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let ram_usage = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };

        (cpu_usage, ram_usage)
    }
}

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>, // Hold Arc
    cursor_pos: PhysicalPosition<f64>,

    // Pipelines
    compute_pipeline: wgpu::ComputePipeline,
    render_pipeline: wgpu::RenderPipeline,

    // Resources
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    texture_a: wgpu::Texture,
    texture_b: wgpu::Texture,

    bind_group_a: wgpu::BindGroup, // Read A, Write B
    bind_group_b: wgpu::BindGroup, // Read B, Write A

    display_bind_group_a: wgpu::BindGroup, // Display A
    display_bind_group_b: wgpu::BindGroup, // Display B

    frame_count: u64,
    monitor: SystemMonitor,

    // Simulation Params
    uniforms: Uniforms,
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Surface creation using Arc<Window>
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
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    required_limits: wgpu::Limits::default(),
                    label: None,
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

        // --- Resources ---

        // 1. Textures (Ping Pong)
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Simulation Texture"),
            size: wgpu::Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        let texture_a = device.create_texture(&texture_desc);
        let texture_b = device.create_texture(&texture_desc);
        let texture_view_a = texture_a.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_view_b = texture_b.create_view(&wgpu::TextureViewDescriptor::default());

        // 2. Uniforms
        let uniforms = Uniforms {
            feed: 0.055,
            kill: 0.062,
            dt: 1.0,
            diff_u: 1.0,
            diff_v: 0.5,
            _padding1: 0.0,
            _padding2: 0.0,
            _padding3: 0.0,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 3. Shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // --- Pipelines ---

        // Compute Pipeline
        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Compute Bind Group Layout"),
                entries: &[
                    // Uniforms
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let storage_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Storage Bind Group Layout"),
                entries: &[
                    // Current State (Read)
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    // Next State (Write)
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_bind_group_layout, &storage_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "update",
        });

        // Render Pipeline
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout"),
                entries: &[
                    // Uniforms
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // Texture to display
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    // Sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                        count: None,
                    },
                ],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&render_bind_group_layout],
                push_constant_ranges: &[],
            });

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
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // --- Bind Groups ---
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &compute_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let bind_group_a = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group A"),
            layout: &storage_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view_a),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view_b),
                },
            ],
        });

        let bind_group_b = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group B"),
            layout: &storage_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view_b),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view_a),
                },
            ],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let display_bind_group_a = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Display Bind Group A"),
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view_a),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let display_bind_group_b = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Display Bind Group B"),
            layout: &render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view_b),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        // Seed initial state
        // Create initial data with some noise
        let mut initial_data = vec![0u8; (WIDTH * HEIGHT * 4 * 4) as usize];
        // RGBA32Float = 16 bytes per pixel.
        // We want to set some pixels to have v=1.0 (Green channel)
        // Format is R32 G32 B32 A32 floats.

        // Use a simple seeded loop for now
        for y in (HEIGHT / 2 - 20)..(HEIGHT / 2 + 20) {
            for x in (WIDTH / 2 - 20)..(WIDTH / 2 + 20) {
                let idx = ((y * WIDTH + x) * 16) as usize;
                // Set G (v) to 1.0
                // F32 as bytes
                let val: f32 = 1.0;
                let bytes = val.to_ne_bytes();
                // R=0, G=1, B=0, A=0
                initial_data[idx + 4] = bytes[0];
                initial_data[idx + 5] = bytes[1];
                initial_data[idx + 6] = bytes[2];
                initial_data[idx + 7] = bytes[3];
            }
        }

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture_a,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &initial_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(WIDTH * 16),
                rows_per_image: Some(HEIGHT),
            },
            wgpu::Extent3d {
                width: WIDTH,
                height: HEIGHT,
                depth_or_array_layers: 1,
            },
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            cursor_pos: PhysicalPosition::new(0.0, 0.0),
            compute_pipeline,
            render_pipeline,
            uniform_buffer,
            uniform_bind_group,
            texture_a,
            texture_b,
            bind_group_a,
            bind_group_b,
            display_bind_group_a,
            display_bind_group_b,
            frame_count: 0,
            monitor: SystemMonitor::new(),
            uniforms,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = *position;
                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *state == ElementState::Pressed && *button == MouseButton::Left {
                    self.add_catalyst();
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn add_catalyst(&mut self) {
        // Map cursor pos to texture coords
        let w = self.size.width as f64;
        let h = self.size.height as f64;

        if w == 0.0 || h == 0.0 {
            return;
        }

        // Clamp cursor to window
        let cx = self.cursor_pos.x.clamp(0.0, w - 1.0);
        let cy = self.cursor_pos.y.clamp(0.0, h - 1.0);

        let tx = (cx / w * WIDTH as f64) as u32;
        let ty = (cy / h * HEIGHT as f64) as u32;

        let radius = 16;
        let diameter = radius * 2;

        // Create data for a square (easier than circle for now)
        let mut data = vec![0u8; (diameter * diameter * 16) as usize];

        for dy in 0..diameter {
            for dx in 0..diameter {
                let val: f32 = 0.9; // Add V
                let bytes = val.to_ne_bytes();
                let idx = ((dy * diameter + dx) * 16) as usize;

                // R=0, G=0.9, B=0, A=0
                data[idx + 4] = bytes[0];
                data[idx + 5] = bytes[1];
                data[idx + 6] = bytes[2];
                data[idx + 7] = bytes[3];
            }
        }

        let origin_x = tx.saturating_sub(radius);
        let origin_y = ty.saturating_sub(radius);

        // Clip to bounds
        let copy_width = diameter.min(WIDTH - origin_x);
        let copy_height = diameter.min(HEIGHT - origin_y);

        if copy_width == 0 || copy_height == 0 {
            return;
        }

        // Write to BOTH textures to ensure it sticks
        for texture in [&self.texture_a, &self.texture_b] {
            self.queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: origin_x,
                        y: origin_y,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &data, // This buffer is technically too large if clipped, but write_texture ignores extra
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(diameter * 16),
                    rows_per_image: Some(diameter),
                },
                wgpu::Extent3d {
                    width: copy_width,
                    height: copy_height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }

    fn update(&mut self) {
        // Read System
        let (cpu, ram) = self.monitor.refresh();

        // Map to Feed/Kill
        // Gray Scott Spots: F=0.055, K=0.062
        // Chaos/Stripes: F=0.03..0.06, K=0.05..0.07

        // CPU (0-100) maps to Feed (Energy). High CPU -> High Feed.
        // Base F = 0.055. Variation +/- 0.02.
        // F = 0.035 + (cpu / 100.0) * 0.04
        self.uniforms.feed = 0.035 + (cpu / 100.0) * 0.04;

        // RAM (0-100) maps to Kill (Damping). High RAM -> Low Kill (Overgrowth) or High Kill (Death)?
        // Let's say High RAM = High Kill (Constraint).
        // Base K = 0.062. Variation +/- 0.02.
        // K = 0.045 + (ram / 100.0) * 0.04
        self.uniforms.kill = 0.045 + (ram / 100.0) * 0.04;

        // Update Uniform Buffer
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Compute Pass (Ping Pong)
        // If frame is even, read A write B. If odd, read B write A.
        let (compute_bind_group, display_bind_group) = if self.frame_count % 2 == 0 {
            (&self.bind_group_a, &self.display_bind_group_b) // Write B, Display B
        } else {
            (&self.bind_group_b, &self.display_bind_group_a) // Write A, Display A
        };

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            cpass.set_bind_group(1, compute_bind_group, &[]);
            cpass.dispatch_workgroups(WIDTH / 16, HEIGHT / 16, 1);
        }

        // Render Pass
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, display_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.frame_count += 1;

        Ok(())
    }
}

pub fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Reaction Monitor")
            .build(&event_loop)
            .unwrap(),
    );

    // We clone window for state
    let mut state = pollster::block_on(State::new(window.clone()));

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
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
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => {}
    });
}
