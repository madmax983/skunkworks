use winit::{
    event::*,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
    keyboard::{KeyCode, PhysicalKey},
};
use wgpu::util::DeviceExt;
use std::sync::Arc;
use rand::Rng;

const GRID_WIDTH: u32 = 512;
const GRID_HEIGHT: u32 = 512;
const WORKGROUP_SIZE: u32 = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct FluidUniforms {
    dt: f32,
    grid_scale: f32,
    width: f32,
    height: f32,
    viscosity: f32,
    _pad1: f32,
    mouse_pos: [f32; 2],
    mouse_active: f32,
    _pad2: f32,
    mouse_vel: [f32; 2],
    color_shift: f32,
    _pad3: [f32; 3],
}

struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    window: Arc<Window>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniforms: FluidUniforms,
    views: Vec<wgpu::TextureView>,
    textures: Vec<wgpu::Texture>,

    idx_density_read: usize,
    idx_density_write: usize,
    idx_velocity_read: usize,
    idx_velocity_write: usize,
    idx_divergence: usize,
    idx_pressure_read: usize,
    idx_pressure_write: usize,
    idx_glyph: usize,

    pipeline_advect: wgpu::ComputePipeline,
    pipeline_divergence: wgpu::ComputePipeline,
    pipeline_jacobi: wgpu::ComputePipeline,
    pipeline_subtract: wgpu::ComputePipeline,
    pipeline_inject_vel: wgpu::ComputePipeline,
    pipeline_inject_den: wgpu::ComputePipeline,
    pipeline_render: wgpu::RenderPipeline,

    compute_bind_group_layout: wgpu::BindGroupLayout,
    render_bind_group_layout: wgpu::BindGroupLayout,

    last_mouse_pos: Option<winit::dpi::PhysicalPosition<f64>>,
    start_time: std::time::Instant,

    font: rusttype::Font<'static>,
    glyph_data: Vec<u8>,
    injection_active: bool,
}

impl<'a> State<'a> {
    async fn new(window: Arc<Window>) -> State<'a> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
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

        let uniforms = FluidUniforms {
            dt: 0.016, grid_scale: 1.0 / GRID_WIDTH as f32, width: GRID_WIDTH as f32, height: GRID_HEIGHT as f32,
            viscosity: 0.0001, _pad1: 0.0, mouse_pos: [0.0; 2], mouse_active: 0.0, _pad2: 0.0, mouse_vel: [0.0; 2],
            color_shift: 0.0, _pad3: [0.0; 3],
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"), contents: bytemuck::cast_slice(&[uniforms]), usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge, address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Linear, mipmap_filter: wgpu::FilterMode::Nearest, ..Default::default()
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None }
            ], label: Some("uniform_layout"),
        });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &uniform_bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) }], label: Some("uniform_bg") });

        let compute_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::Rgba16Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
            ], label: Some("compute_layout"),
        });
        let render_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            ], label: Some("render_layout"),
        });

        let mut textures = Vec::new();
        let mut views = Vec::new();
        let tex_desc = wgpu::TextureDescriptor {
            label: None, size: wgpu::Extent3d { width: GRID_WIDTH, height: GRID_HEIGHT, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        };
        for i in 0..8 {
            let tex = device.create_texture(&tex_desc);
            views.push(tex.create_view(&wgpu::TextureViewDescriptor::default()));
            textures.push(tex);
        }

        let shader = device.create_shader_module(wgpu::include_wgsl!("fluid.wgsl"));
        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("Compute PL"), bind_group_layouts: &[&uniform_bind_group_layout, &compute_bind_group_layout], push_constant_ranges: &[] });
        let create_pipeline = |entry: &str| device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor { label: Some(entry), layout: Some(&compute_pipeline_layout), module: &shader, entry_point: entry, compilation_options: Default::default() });

        let pipeline_advect = create_pipeline("advect");
        let pipeline_divergence = create_pipeline("divergence");
        let pipeline_jacobi = create_pipeline("jacobi");
        let pipeline_subtract = create_pipeline("subtract_gradient");
        let pipeline_inject_vel = create_pipeline("inject_velocity");
        let pipeline_inject_den = create_pipeline("inject_density");

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("Render PL"), bind_group_layouts: &[&uniform_bind_group_layout, &render_bind_group_layout], push_constant_ranges: &[] });
        let pipeline_render = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(wgpu::ColorTargetState { format: config.format, blend: Some(wgpu::BlendState::REPLACE), write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState::default(), depth_stencil: None, multisample: wgpu::MultisampleState::default(), multiview: None,
        });

        let font_data = include_bytes!("../assets/font.ttf");
        let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        Self {
            surface, device, queue, config, size, window, uniform_buffer, uniform_bind_group, uniforms, textures, views,
            idx_density_read: 0, idx_density_write: 1, idx_velocity_read: 2, idx_velocity_write: 3, idx_divergence: 4, idx_pressure_read: 5, idx_pressure_write: 6, idx_glyph: 7,
            pipeline_advect, pipeline_divergence, pipeline_jacobi, pipeline_subtract, pipeline_inject_vel, pipeline_inject_den, pipeline_render,
            compute_bind_group_layout, render_bind_group_layout, last_mouse_pos: None, start_time: std::time::Instant::now(),
            font, glyph_data: vec![0u8; (GRID_WIDTH * GRID_HEIGHT * 8) as usize], // 16-bit float * 4 channels = 8 bytes per pixel
            injection_active: false,
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
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
                self.last_mouse_pos = Some(*position);
                true
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let Some(txt) = &event.text {
                        self.inject_text(txt);
                        self.injection_active = true;
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn inject_text(&mut self, text: &str) {
        self.glyph_data.fill(0);
        let mut rng = rand::thread_rng();
        for c in text.chars() {
            let scale = rusttype::Scale::uniform(128.0);
            let _v_metrics = self.font.v_metrics(scale);
            let x = rng.gen_range(50.0..GRID_WIDTH as f32 - 50.0);
            let y = rng.gen_range(50.0..GRID_HEIGHT as f32 - 50.0);
            let point = rusttype::point(x, y);
            let glyph = self.font.glyph(c).scaled(scale).positioned(point);
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let px = gx as i32 + bb.min.x;
                    let py = gy as i32 + bb.min.y;
                    if px >= 0 && px < GRID_WIDTH as i32 && py >= 0 && py < GRID_HEIGHT as i32 {
                         let idx = ((py as u32 * GRID_WIDTH + px as u32) as usize) * 8;
                         if v > 0.1 {
                             self.glyph_data[idx] = 0x00;
                             self.glyph_data[idx+1] = 0x3C;
                         }
                    }
                });
            }
        }
        self.queue.write_texture(
            wgpu::ImageCopyTexture { texture: &self.textures[self.idx_glyph], mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            &self.glyph_data,
            wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(GRID_WIDTH * 8), rows_per_image: Some(GRID_HEIGHT) },
            wgpu::Extent3d { width: GRID_WIDTH, height: GRID_HEIGHT, depth_or_array_layers: 1 }
        );
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        let time = (now - self.start_time).as_secs_f32();
        self.uniforms.dt = 0.016;
        self.uniforms.color_shift = time;
        if let Some(pos) = self.last_mouse_pos {
            let x = (pos.x as f32 / self.size.width as f32) * GRID_WIDTH as f32;
            let y = (pos.y as f32 / self.size.height as f32) * GRID_HEIGHT as f32;
            let dx = x - self.uniforms.mouse_pos[0];
            let dy = y - self.uniforms.mouse_pos[1];
            self.uniforms.mouse_pos = [x, y];
            self.uniforms.mouse_vel = [dx * 10.0, dy * 10.0];
            self.uniforms.mouse_active = 1.0;
        } else {
            self.uniforms.mouse_active = 0.0;
        }
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Render Encoder") });

        let mut idx_vel_r = self.idx_velocity_read;
        let mut idx_vel_w = self.idx_velocity_write;
        let mut idx_den_r = self.idx_density_read;
        let mut idx_den_w = self.idx_density_write;
        let idx_pres_r = self.idx_pressure_read;
        let idx_pres_w = self.idx_pressure_write;

        let create_bg = |ia, ib, out| {
            self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.views[ia]) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&self.views[ib]) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&self.views[out]) },
                ],
                label: None,
            })
        };

        let bg_advect_vel = create_bg(idx_vel_r, idx_vel_r, idx_vel_w);
        let bg_advect_den = create_bg(idx_vel_r, idx_den_r, idx_den_w);

        std::mem::swap(&mut idx_vel_r, &mut idx_vel_w);
        std::mem::swap(&mut idx_den_r, &mut idx_den_w);

        let bg_inject_vel = create_bg(idx_vel_r, self.idx_glyph, idx_vel_w);
        let bg_inject_den = create_bg(idx_den_r, self.idx_glyph, idx_den_w);

        let mut next_vel_r = idx_vel_r;
        let mut next_vel_w = idx_vel_w;
        std::mem::swap(&mut next_vel_r, &mut next_vel_w);

        let mut next_den_r = idx_den_r;
        let mut next_den_w = idx_den_w;
        if self.injection_active {
            std::mem::swap(&mut next_den_r, &mut next_den_w);
        }

        let bg_div = create_bg(next_vel_r, next_vel_r, self.idx_divergence);
        let bg_jacobi_0 = create_bg(idx_pres_r, self.idx_divergence, idx_pres_w);
        let bg_jacobi_1 = create_bg(idx_pres_w, self.idx_divergence, idx_pres_r);
        let bg_sub = create_bg(next_vel_r, idx_pres_r, next_vel_w);
        let final_vel_r = next_vel_w;
        let final_vel_w = next_vel_r;

        let bg_render = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.render_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.views[next_den_r]) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&self.views[final_vel_r]) },
            ],
            label: None,
        });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("CP"), timestamp_writes: None });
            cpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            let w = GRID_WIDTH / WORKGROUP_SIZE;
            let h = GRID_HEIGHT / WORKGROUP_SIZE;

            cpass.set_pipeline(&self.pipeline_advect);
            cpass.set_bind_group(1, &bg_advect_vel, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            cpass.set_bind_group(1, &bg_advect_den, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            cpass.set_pipeline(&self.pipeline_inject_vel);
            cpass.set_bind_group(1, &bg_inject_vel, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            if self.injection_active {
                cpass.set_pipeline(&self.pipeline_inject_den);
                cpass.set_bind_group(1, &bg_inject_den, &[]);
                cpass.dispatch_workgroups(w, h, 1);
            }

            cpass.set_pipeline(&self.pipeline_divergence);
            cpass.set_bind_group(1, &bg_div, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            cpass.set_pipeline(&self.pipeline_jacobi);
            for i in 0..20 {
                if i % 2 == 0 {
                    cpass.set_bind_group(1, &bg_jacobi_0, &[]);
                } else {
                    cpass.set_bind_group(1, &bg_jacobi_1, &[]);
                }
                cpass.dispatch_workgroups(w, h, 1);
            }

            cpass.set_pipeline(&self.pipeline_subtract);
            cpass.set_bind_group(1, &bg_sub, &[]);
            cpass.dispatch_workgroups(w, h, 1);
        }

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RP"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            rpass.set_pipeline(&self.pipeline_render);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_bind_group(1, &bg_render, &[]);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.idx_velocity_read = final_vel_r;
        self.idx_velocity_write = final_vel_w;
        self.idx_density_read = next_den_r;
        self.idx_density_write = next_den_w;

        self.injection_active = false;
        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().with_title("Genesis: Fluid Corpus").with_inner_size(PhysicalSize::new(1024, 1024)).build(&event_loop).unwrap());
    let mut state = pollster::block_on(State::new(window.clone()));
    event_loop.run(move |event, elwt| {
        match event {
             Event::WindowEvent { window_id, event } if window_id == window.id() => {
                 if !state.input(&event) {
                     match event {
                         WindowEvent::CloseRequested => elwt.exit(),
                         WindowEvent::KeyboardInput { event, .. } => {
                             if event.state == ElementState::Pressed {
                                 if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key { elwt.exit(); }
                             }
                         },
                         WindowEvent::RedrawRequested => {
                             state.update();
                             match state.render() {
                                 Ok(_) => {}
                                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                 Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                 Err(e) => eprintln!("{:?}", e),
                             }
                         },
                         WindowEvent::Resized(ps) => state.resize(ps),
                         _ => {}
                     }
                 }
             },
             Event::AboutToWait => window.request_redraw(),
             _ => {}
        }
    }).unwrap();
}
