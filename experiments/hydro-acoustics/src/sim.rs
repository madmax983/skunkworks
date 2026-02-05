use winit::window::Window;

const TEXTURE_SIZE: u32 = 512;
const WORKGROUP_SIZE: u32 = 16;

pub struct Simulation {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub compute_pipeline: wgpu::ComputePipeline,

    pub texture_a: wgpu::Texture,
    pub texture_b: wgpu::Texture,
    pub texture_c: wgpu::Texture,

    pub bind_groups: Vec<wgpu::BindGroup>, // [ABC, BCA, CAB]
    pub frame_count: usize,

    pub render_bind_groups: Vec<wgpu::BindGroup>, // To visualize A, B, or C

    // Hydrophones
    pub hydrophone_buffer: wgpu::Buffer,
    pub hydrophone_coords: Vec<(u32, u32)>,
    pub last_hydrophone_data: Vec<f32>,
    mapped_ready: std::sync::Arc<std::sync::atomic::AtomicBool>,
    map_pending: bool,
}

impl Simulation {
    pub async fn new(window: std::sync::Arc<Window>) -> Self {
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
                    required_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
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

        // --- Textures ---
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Wave Texture"),
            size: wgpu::Extent3d {
                width: TEXTURE_SIZE,
                height: TEXTURE_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };

        let texture_a = device.create_texture(&texture_desc);
        let texture_b = device.create_texture(&texture_desc);
        let texture_c = device.create_texture(&texture_desc);

        let view_a = texture_a.create_view(&wgpu::TextureViewDescriptor::default());
        let view_b = texture_b.create_view(&wgpu::TextureViewDescriptor::default());
        let view_c = texture_c.create_view(&wgpu::TextureViewDescriptor::default());

        // --- Shader ---
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // --- Compute Pipeline ---
        let compute_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Compute Bind Group Layout"),
                entries: &[
                    // Binding 0: Current (Read)
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::ReadOnly,
                            format: wgpu::TextureFormat::R32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    // Binding 1: Prev (Read)
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::ReadOnly,
                            format: wgpu::TextureFormat::R32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    // Binding 2: Next (Write)
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::R32Float,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                ],
            });

        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compute Pipeline Layout"),
                bind_group_layouts: &[&compute_bind_group_layout],
                push_constant_ranges: &[],
            });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &shader,
            entry_point: "compute_main",
        });

        // Create 3 bind groups for cycling
        // Cycle:
        // 0: Curr=A, Prev=B, Next=C
        // 1: Curr=C, Prev=A, Next=B
        // 2: Curr=B, Prev=C, Next=A
        // Note: Prev must be the one BEFORE Curr.
        // If we start A, B(prev), C(next) -> Calc C from A, B.
        // Next step: Curr=C, Prev=A. Calc B from C, A.
        // Next step: Curr=B, Prev=C. Calc A from B, C.

        let create_compute_bg = |label, curr: &wgpu::TextureView, prev: &wgpu::TextureView, next: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &compute_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(curr),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(prev),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(next),
                    },
                ],
            })
        };

        let bg0 = create_compute_bg("BG0: A, B -> C", &view_a, &view_b, &view_c); // A=Curr, B=Prev, C=Next
        let bg1 = create_compute_bg("BG1: C, A -> B", &view_c, &view_a, &view_b); // C=Curr, A=Prev, B=Next
        let bg2 = create_compute_bg("BG2: B, C -> A", &view_b, &view_c, &view_a); // B=Curr, C=Prev, A=Next

        // --- Render Pipeline ---
        let render_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Render Bind Group Layout"),
                entries: &[
                    // Texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let create_render_bg = |label, tex: &wgpu::TextureView| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(label),
                layout: &render_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(tex),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    },
                ],
            })
        };

        // We render the "Current" state.
        // Frame 0: Curr=A. Render BG0 (using A).
        // Frame 1: Curr=C. Render BG1 (using C).
        // Frame 2: Curr=B. Render BG2 (using B).

        let rbg0 = create_render_bg("Render A", &view_a);
        let rbg1 = create_render_bg("Render C", &view_c);
        let rbg2 = create_render_bg("Render B", &view_b);

        // Hydrophones Setup
        // Place them in corners and center
        let hydrophone_coords = vec![
            (TEXTURE_SIZE / 2, TEXTURE_SIZE / 2),     // Center
            (TEXTURE_SIZE / 4, TEXTURE_SIZE / 4),     // Top-Left
            (TEXTURE_SIZE * 3 / 4, TEXTURE_SIZE / 4), // Top-Right
            (TEXTURE_SIZE / 2, TEXTURE_SIZE * 3 / 4), // Bottom
        ];

        // Align to 256 bytes per hydrophone
        let buffer_size = (hydrophone_coords.len() * 256) as wgpu::BufferAddress;
        let hydrophone_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Hydrophone Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            compute_pipeline,
            texture_a,
            texture_b,
            texture_c,
            bind_groups: vec![bg0, bg1, bg2],
            render_bind_groups: vec![rbg0, rbg1, rbg2],
            frame_count: 0,
            hydrophone_buffer,
            hydrophone_coords,
            last_hydrophone_data: vec![0.0; 4],
            mapped_ready: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            map_pending: false,
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

    pub fn add_drop(&self, x: f32, y: f32) {
        // x, y are normalized 0..1
        let tex_x = (x * TEXTURE_SIZE as f32) as u32;
        let tex_y = (y * TEXTURE_SIZE as f32) as u32;

        // Draw a 5x5 blob
        let radius = 5;
        let width = 2 * radius + 1;
        let mut data = Vec::with_capacity((width * width) as usize * 4);

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let dist = (dx * dx + dy * dy) as f32;
                let val = (-dist / 10.0).exp(); // Gaussian
                data.extend_from_slice(bytemuck::bytes_of(&val));
            }
        }

        // Current texture is the one at index frame_count % 3
        // BG0: A(0), B(1), C(2). Current = A.
        // BG1: C(0), A(1), B(2). Current = C.
        // BG2: B(0), C(1), A(2). Current = B.

        let target_texture = match self.frame_count % 3 {
            0 => &self.texture_a,
            1 => &self.texture_c,
            2 => &self.texture_b,
            _ => unreachable!(),
        };

        // Clip coords
        let origin_x = tex_x.saturating_sub(radius as u32);
        let origin_y = tex_y.saturating_sub(radius as u32);

        // Ensure we don't write OOB.
        // For simplicity, just ignore edge cases or let wgpu clip (it doesn't clip automatically, returns error).
        if origin_x + width as u32 >= TEXTURE_SIZE || origin_y + width as u32 >= TEXTURE_SIZE {
             return;
        }

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: target_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: origin_x,
                    y: origin_y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width as u32 * 4),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: width as u32,
                height: width as u32,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn update(&mut self) {
        // 0. Poll device to trigger callbacks
        self.device.poll(wgpu::Maintain::Poll);

        // 1. Check Hydrophone Readback
        if self.mapped_ready.load(std::sync::atomic::Ordering::Relaxed) {
            {
                let buffer_slice = self.hydrophone_buffer.slice(..);
                let view = buffer_slice.get_mapped_range();
                let data: &[f32] = bytemuck::cast_slice(&view);

                for (i, val) in self.last_hydrophone_data.iter_mut().enumerate() {
                    // Index = (i * 256) / 4
                    *val = data[i * 64];
                }
            }
            self.hydrophone_buffer.unmap();
            self.mapped_ready.store(false, std::sync::atomic::Ordering::Relaxed);
            self.map_pending = false; // Ready for next request
        }

        // 2. Dispatch Compute
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Wave Compute Pass"),
                timestamp_writes: None,
            });
            cpass.set_pipeline(&self.compute_pipeline);
            // Cycle bind groups
            let idx = self.frame_count % 3;
            cpass.set_bind_group(0, &self.bind_groups[idx], &[]);
            cpass.dispatch_workgroups(TEXTURE_SIZE / WORKGROUP_SIZE, TEXTURE_SIZE / WORKGROUP_SIZE, 1);
        }

        // 3. Initiate Readback for *Result* (Current Texture)
        // Note: The compute pass wrote to "Next". We cycle frame_count after.
        // So "Next" becomes "Current" for next frame. We want to read "Next".
        // "Next" is Binding 2 of current bind group.
        // BG0: Next=C. BG1: Next=B. BG2: Next=A.

        let target_texture = match self.frame_count % 3 {
            0 => &self.texture_c,
            1 => &self.texture_b,
            2 => &self.texture_a,
            _ => unreachable!(),
        };

        // Only issue copy if not currently mapped OR pending mapping
        if !self.map_pending {
            for (i, (x, y)) in self.hydrophone_coords.iter().enumerate() {
                let offset = (i * 256) as u64;
                encoder.copy_texture_to_buffer(
                    wgpu::ImageCopyTexture {
                        texture: target_texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d { x: *x, y: *y, z: 0 },
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::ImageCopyBuffer {
                        buffer: &self.hydrophone_buffer,
                        layout: wgpu::ImageDataLayout {
                            offset,
                            bytes_per_row: Some(256), // Must be multiple of 256
                            rows_per_image: None,
                        },
                    },
                    wgpu::Extent3d {
                        width: 1,
                        height: 1,
                        depth_or_array_layers: 1,
                    },
                );
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        // Map Async if we issued copy
        // Actually, `copy_texture_to_buffer` requires bytes_per_row alignment (256).
        // My buffer is small (16 bytes).
        // Error: "bytes_per_row (4) is less than 256".
        // I must copy to a bigger temp buffer?
        // OR copy 1 pixel from texture to a 1x1 buffer?
        // bytes_per_row applies to texture copy.
        // If width=1, bytes_per_row must still be aligned? Yes.
        // So I can't pack them easily in one copy command unless I copy a rect.
        // Or I use separate copies to separate aligned offsets in a bigger buffer.

        // Simpler approach: Read 1 pixel to a 256-byte buffer.
        // 4 hydrophones -> 4 * 256 bytes buffer.
        // Read from offset i*256.

        if !self.map_pending {
            let slice = self.hydrophone_buffer.slice(..);
            let ready = self.mapped_ready.clone();
            slice.map_async(wgpu::MapMode::Read, move |res| {
                if res.is_ok() {
                    ready.store(true, std::sync::atomic::Ordering::Relaxed);
                } else {
                    log::error!("Buffer map failed");
                }
            });
            self.map_pending = true;
        }

        self.frame_count += 1;
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

            // Render the *Result* of the PREVIOUS update.
            // If frame_count was just incremented in update(),
            // Frame 0: Updated using bg0 (A,B->C). Next state is C.
            // frame_count is now 1.
            // We want to display C.
            // render_bind_groups layout:
            // 0: A
            // 1: C
            // 2: B

            // Wait, let's trace:
            // Start: frame_count = 0.
            // Update: idx = 0. Use bg0 (A, B -> C). Result in C. frame_count -> 1.
            // Render: We want C. C is at index 1 in my vector `render_bind_groups`?
            // rbg0 = A, rbg1 = C, rbg2 = B.
            // So if frame_count=1, we want index 1.
            // Next Update: idx = 1. Use bg1 (C, A -> B). Result in B. frame_count -> 2.
            // Render: We want B. B is at index 2.
            // Next Update: idx = 2. Use bg2 (B, C -> A). Result in A. frame_count -> 3.
            // Render: We want A. A is at index 0.

            // So logic: `let idx = self.frame_count % 3;`
            // If frame_count=1, idx=1 -> C. Correct.
            // If frame_count=3, idx=0 -> A. Correct.

            let idx = self.frame_count % 3;
            rpass.set_bind_group(0, &self.render_bind_groups[idx], &[]);
            rpass.draw(0..3, 0..1); // 1 triangle covering screen
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
