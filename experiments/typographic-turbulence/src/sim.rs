use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use rusttype::{Font, Scale};
use image::{ImageBuffer, Luma};
use rand::Rng;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct FluidUniforms {
    dt: f32,
    width: u32,
    height: u32,
    dx: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Impulse {
    pos: [f32; 2],
    radius: f32,
    pad: f32,
    val: [f32; 2],
    pad2: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Particle {
    pos: [f32; 2],
    vel: [f32; 2],
    life: f32,
    char_idx: u32,
    pad: [f32; 2],
}

pub struct SimState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    fluid: FluidSim,
    particles: ParticleSystem,
    renderer: Renderer,

    pub mouse_pos: [f32; 2],
    pub mouse_pressed: bool,
}

impl SimState {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            required_limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
        }, None).await.unwrap();

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
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let width = 512;
        let height = 512;

        let fluid = FluidSim::new(&device, width, height);
        let particles = ParticleSystem::new(&device, 1_000_000);
        let renderer = Renderer::new(&device, &queue, &config, width, height, &particles);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            fluid,
            particles,
            renderer,
            mouse_pos: [0.0, 0.0],
            mouse_pressed: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn update(&mut self) {
        if self.mouse_pressed {
            let imp = Impulse {
                pos: [self.mouse_pos[0] / self.size.width as f32, self.mouse_pos[1] / self.size.height as f32],
                radius: 0.05,
                pad: 0.0,
                val: [100.0, 0.0], // Impulse direction? Just push right for now or use delta.
                pad2: [0.0, 0.0],
            };
            self.queue.write_buffer(&self.fluid.impulse_buffer, 0, bytemuck::cast_slice(&[imp]));
        } else {
             let imp = Impulse {
                pos: [-1.0, -1.0],
                radius: 0.0,
                pad: 0.0,
                val: [0.0, 0.0],
                pad2: [0.0, 0.0],
            };
            self.queue.write_buffer(&self.fluid.impulse_buffer, 0, bytemuck::cast_slice(&[imp]));
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.fluid.dispatch(&mut encoder);
        self.particles.dispatch(&mut encoder, &self.device, &self.fluid.vel_tex_view, &self.fluid.uniforms_buffer);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer.draw(&mut render_pass, &self.particles.particle_buffer, self.particles.count);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

struct FluidSim {
    width: u32,
    height: u32,

    vel_tex: wgpu::Texture,
    vel_temp: wgpu::Texture,
    vel_tex_view: wgpu::TextureView,
    vel_temp_view: wgpu::TextureView,

    bg_param: wgpu::BindGroup,
    bg_impulse: wgpu::BindGroup,
    bg_advect: wgpu::BindGroup,
    bg_divergence: wgpu::BindGroup,
    bg_pressure_0: wgpu::BindGroup,
    bg_pressure_1: wgpu::BindGroup,
    bg_subtract: wgpu::BindGroup,
    bg_impulse_sim: wgpu::BindGroup,

    pipelines: [wgpu::ComputePipeline; 5],

    uniforms_buffer: wgpu::Buffer,
    impulse_buffer: wgpu::Buffer,
}

impl FluidSim {
    fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let create_tex = |label, format| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            })
        };

        let vel_tex = create_tex("Vel", wgpu::TextureFormat::Rg32Float);
        let vel_temp = create_tex("VelTemp", wgpu::TextureFormat::Rg32Float);
        let pressure_tex = create_tex("Pressure", wgpu::TextureFormat::R32Float);
        let pressure_temp = create_tex("PressureTemp", wgpu::TextureFormat::R32Float);
        let divergence_tex = create_tex("Divergence", wgpu::TextureFormat::R32Float);
        let obstacles_tex = create_tex("Obstacles", wgpu::TextureFormat::R32Float); // Should init to 0

        let vel_view = vel_tex.create_view(&Default::default());
        let vel_temp_view = vel_temp.create_view(&Default::default());
        let press_view = pressure_tex.create_view(&Default::default());
        let press_temp_view = pressure_temp.create_view(&Default::default());
        let div_view = divergence_tex.create_view(&Default::default());
        let obs_view = obstacles_tex.create_view(&Default::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            ..Default::default()
        });

        let uniforms_data = FluidUniforms { dt: 0.016, width, height, dx: 1.0 };
        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Fluid Uniforms"),
            contents: bytemuck::cast_slice(&[uniforms_data]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let impulse_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Impulse Uniform"),
            size: std::mem::size_of::<Impulse>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Layouts
        let param_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Param Layout"),
            entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
        });

        let impulse_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
             label: Some("Impulse Layout"),
             entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
        });

        let sim_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Sim Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::Rg32Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ],
        });

        let div_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Div Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::R32Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            ]
        });

        let press_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Press Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::R32Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            ]
        });

        let sub_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Sub Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::Rg32Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
            ]
        });

        let imp_sim_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
             label: Some("Impulse Sim Layout"),
              entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: false }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::Rg32Float, view_dimension: wgpu::TextureViewDimension::D2 }, count: None },
             ]
        });

        // Pipelines
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fluid Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/fluid.wgsl").into()),
        });

        let create_pipeline = |label, layout, entry| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(label),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: layout,
                    push_constant_ranges: &[],
                })),
                module: &shader,
                entry_point: entry,
            })
        };

        let advect_layouts = [&param_layout, &sim_layout];
        let advect_pipeline = create_pipeline("Advect", &advect_layouts, "advect");

        let div_layouts = [&param_layout, &div_layout];
        let divergence_pipeline = create_pipeline("Divergence", &div_layouts, "divergence");

        let press_layouts = [&param_layout, &press_layout];
        let pressure_pipeline = create_pipeline("Pressure", &press_layouts, "pressure");

        let sub_layouts = [&param_layout, &sub_layout];
        let subtract_pipeline = create_pipeline("Subtract", &sub_layouts, "subtract");

        let imp_layouts = [&param_layout, &imp_sim_layout, &impulse_layout];
        let impulse_pipeline = create_pipeline("Impulse", &imp_layouts, "add_impulse");

        // Bind Groups
        let bg_param = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Param BG"),
            layout: &param_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniforms_buffer.as_entire_binding() }],
        });

        let bg_impulse = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Impulse BG"),
            layout: &impulse_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: impulse_buffer.as_entire_binding() }],
        });

        let bg_advect = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Advect BG"),
            layout: &sim_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&vel_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&vel_temp_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&obs_view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        let bg_divergence = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Div BG"),
            layout: &div_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&vel_temp_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&div_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&obs_view) },
            ],
        });

        let create_press_bg = |src: &wgpu::TextureView, dst: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Pressure BG"),
                layout: &press_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(src) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(dst) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&div_view) },
                    wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&obs_view) },
                ]
            })
        };
        let bg_press_0 = create_press_bg(&press_view, &press_temp_view);
        let bg_press_1 = create_press_bg(&press_temp_view, &press_view);

        let bg_subtract = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Subtract BG"),
            layout: &sub_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&vel_temp_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&press_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&vel_view) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&obs_view) },
            ],
        });

        let bg_impulse_sim = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Impulse Sim BG"),
            layout: &imp_sim_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&vel_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&vel_temp_view) },
            ],
        });

        Self {
            width, height,
            vel_tex, vel_temp,
            vel_tex_view: vel_view, vel_temp_view,
            bg_param, bg_impulse, bg_advect, bg_divergence, bg_pressure_0: bg_press_0, bg_pressure_1: bg_press_1, bg_subtract, bg_impulse_sim,
            pipelines: [advect_pipeline, divergence_pipeline, pressure_pipeline, subtract_pipeline, impulse_pipeline],
            uniforms_buffer, impulse_buffer,
        }
    }

    fn dispatch(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let x_groups = (self.width + 15) / 16;
        let y_groups = (self.height + 15) / 16;

        // 1. Impulse: Vel -> VelTemp
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Impulse Pass") , timestamp_writes: None });
            cpass.set_pipeline(&self.pipelines[4]);
            cpass.set_bind_group(0, &self.bg_param, &[]);
            cpass.set_bind_group(1, &self.bg_impulse_sim, &[]);
            cpass.set_bind_group(2, &self.bg_impulse, &[]);
            cpass.dispatch_workgroups(x_groups, y_groups, 1);
        }

        // Copy VelTemp -> Vel (for Advect to use as source)
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture { texture: &self.vel_temp, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::ImageCopyTexture { texture: &self.vel_tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 }
        );

        // 2. Advect: Vel (Source) -> VelTemp (Dest)
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Advect Pass") , timestamp_writes: None });
            cpass.set_pipeline(&self.pipelines[0]);
            cpass.set_bind_group(0, &self.bg_param, &[]);
            cpass.set_bind_group(1, &self.bg_advect, &[]);
            cpass.dispatch_workgroups(x_groups, y_groups, 1);
        }

        // 3. Divergence: VelTemp -> Div
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Div Pass") , timestamp_writes: None });
            cpass.set_pipeline(&self.pipelines[1]);
            cpass.set_bind_group(0, &self.bg_param, &[]);
            cpass.set_bind_group(1, &self.bg_divergence, &[]);
            cpass.dispatch_workgroups(x_groups, y_groups, 1);
        }

        // 4. Pressure: Ping-Pong 20 times
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Pressure Pass") , timestamp_writes: None });
            cpass.set_pipeline(&self.pipelines[2]);
            cpass.set_bind_group(0, &self.bg_param, &[]);
            for i in 0..20 {
                if i % 2 == 0 {
                    cpass.set_bind_group(1, &self.bg_pressure_0, &[]);
                } else {
                    cpass.set_bind_group(1, &self.bg_pressure_1, &[]);
                }
                cpass.dispatch_workgroups(x_groups, y_groups, 1);
            }
        }

        // 5. Subtract: VelTemp - P -> Vel
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Subtract Pass") , timestamp_writes: None });
            cpass.set_pipeline(&self.pipelines[3]);
            cpass.set_bind_group(0, &self.bg_param, &[]);
            cpass.set_bind_group(1, &self.bg_subtract, &[]);
            cpass.dispatch_workgroups(x_groups, y_groups, 1);
        }
    }
}

struct ParticleSystem {
    particle_buffer: wgpu::Buffer,
    count: u32,
    pipeline: wgpu::ComputePipeline,
    bg_layout: wgpu::BindGroupLayout,
}

impl ParticleSystem {
    fn new(device: &wgpu::Device, count: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut data = Vec::with_capacity(count as usize);
        for _ in 0..count {
            data.push(Particle {
                pos: [rng.gen(), rng.gen()],
                vel: [0.0, 0.0],
                life: rng.gen(),
                char_idx: rng.gen_range(0..256),
                pad: [0.0, 0.0],
            });
        }

        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Particles"),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE | wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ]
        });

        let param_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Param Layout"),
            entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&param_layout, &bg_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Particle Update"),
            layout: Some(&layout),
            module: &device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Particle Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/particles.wgsl").into()),
            }),
            entry_point: "update_particles",
        });

        Self { particle_buffer, count, pipeline, bg_layout }
    }

    fn dispatch(&mut self, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device, vel_view: &wgpu::TextureView, param_buffer: &wgpu::Buffer) {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bg_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: self.particle_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(vel_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        let param_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Param Layout"),
            entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }],
        });
        let param_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &param_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: param_buffer.as_entire_binding() }],
        });

        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("Particle Pass"), timestamp_writes: None });
        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, &param_bg, &[]);
        cpass.set_bind_group(1, &bg, &[]);
        cpass.dispatch_workgroups((self.count + 63) / 64, 1, 1);
    }
}

struct Renderer {
    pipeline: wgpu::RenderPipeline,
    font_atlas: wgpu::Texture,
    font_sampler: wgpu::Sampler,
    bg_layout: wgpu::BindGroupLayout,
    bg: wgpu::BindGroup,
}

impl Renderer {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration, width: u32, height: u32, particles: &ParticleSystem) -> Self {
        let font_data = include_bytes!("../assets/font.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
        let atlas_size = 1024u32;
        let char_size = 64.0;
        let scale = Scale::uniform(char_size);

        let mut image = ImageBuffer::<Luma<u8>, Vec<u8>>::new(atlas_size, atlas_size);

        let chars_per_row = 16;
        for i in 0..256u32 {
            let c = std::char::from_u32(i).unwrap_or('?');
            let col = i % chars_per_row;
            let row = i / chars_per_row;

            let v_metrics = font.v_metrics(scale);
            let offset = rusttype::point(
                (col as f32) * (atlas_size as f32 / 16.0),
                (row as f32) * (atlas_size as f32 / 16.0) + v_metrics.ascent
            );

            let glyph = font.glyph(c).scaled(scale).positioned(offset);
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let px = x + bb.min.x as u32;
                    let py = y + bb.min.y as u32;
                    if px < atlas_size && py < atlas_size {
                         image.put_pixel(px, py, Luma([(v * 255.0) as u8]));
                    }
                });
            }
        }

        let font_atlas = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("Font Atlas"),
                size: wgpu::Extent3d { width: atlas_size, height: atlas_size, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &image,
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render BG Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None },
            ]
        });

        let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render BG"),
            layout: &bg_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: particles.particle_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&font_atlas.create_view(&Default::default())) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bg_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
             label: None,
             layout: Some(&pipeline_layout),
             vertex: wgpu::VertexState {
                 module: &device.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(include_str!("shaders/render.wgsl").into()) }),
                 entry_point: "vs_main",
                 buffers: &[]
             },
             fragment: Some(wgpu::FragmentState {
                 module: &device.create_shader_module(wgpu::ShaderModuleDescriptor { label: None, source: wgpu::ShaderSource::Wgsl(include_str!("shaders/render.wgsl").into()) }),
                 entry_point: "fs_main",
                 targets: &[Some(wgpu::ColorTargetState {
                     format: config.format,
                     blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                     write_mask: wgpu::ColorWrites::ALL,
                 })],
             }),
             primitive: wgpu::PrimitiveState {
                 topology: wgpu::PrimitiveTopology::TriangleStrip,
                 ..Default::default()
             },
             depth_stencil: None,
             multisample: wgpu::MultisampleState::default(),
             multiview: None,
        });

        Self { pipeline, font_atlas, font_sampler: sampler, bg_layout, bg }
    }

    fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, _particle_buffer: &'a wgpu::Buffer, count: u32) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bg, &[]);
        rpass.draw(0..4, 0..count);
    }
}
