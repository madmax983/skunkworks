use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SimulationParams {
    pub width: u32,
    pub height: u32,
    pub dt: f32,
    pub dx: f32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_active: u32,
    pub time: f32,
}

pub struct Simulation {
    // Textures
    vel_texture: [wgpu::Texture; 2],
    vel_view: [wgpu::TextureView; 2],
    rho_texture: [wgpu::Texture; 2],
    rho_view: [wgpu::TextureView; 2],
    press_texture: [wgpu::Texture; 2],
    press_view: [wgpu::TextureView; 2],
    div_texture: wgpu::Texture,
    div_view: wgpu::TextureView,
    text_texture: wgpu::Texture,
    text_view: wgpu::TextureView,

    // Uniforms
    pub params: SimulationParams,
    uniform_buffer: wgpu::Buffer,
    common_bind_group: wgpu::BindGroup,

    // Pipelines
    advect_vel_pipeline: wgpu::ComputePipeline,
    advect_rho_pipeline: wgpu::ComputePipeline,
    divergence_pipeline: wgpu::ComputePipeline,
    jacobi_pipeline: wgpu::ComputePipeline,
    subtract_pipeline: wgpu::ComputePipeline,
    inject_pipeline: wgpu::ComputePipeline,

    // Bind Groups
    // We create bind groups on the fly or store them?
    // Storing all permutations is tedious.
    // Creating them every frame is slow? No, `create_bind_group` is cheap enough for 6 calls per frame?
    // Actually, `wgpu` recommends caching bind groups.
    // Let's cache them.
    // But maintaining 2 versions of each is painful.
    // I'll create a helper to create bind groups dynamically for now.
    // If it's slow, I'll cache.
    // Update: I'll store the layouts and create bind groups in `step`.

    bind_group_layouts: BindGroupLayouts,

    pub frame_count: u64,
}

struct BindGroupLayouts {
    advect_vel: wgpu::BindGroupLayout,
    advect_rho: wgpu::BindGroupLayout,
    divergence: wgpu::BindGroupLayout,
    jacobi: wgpu::BindGroupLayout,
    subtract: wgpu::BindGroupLayout,
    inject: wgpu::BindGroupLayout,
}

impl Simulation {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> Self {
        let (vel_a, vel_view_a) = create_texture(device, width, height, wgpu::TextureFormat::Rg16Float, "Vel A");
        let (vel_b, vel_view_b) = create_texture(device, width, height, wgpu::TextureFormat::Rg16Float, "Vel B");
        let (rho_a, rho_view_a) = create_texture(device, width, height, wgpu::TextureFormat::R16Float, "Rho A");
        let (rho_b, rho_view_b) = create_texture(device, width, height, wgpu::TextureFormat::R16Float, "Rho B");
        let (press_a, press_view_a) = create_texture(device, width, height, wgpu::TextureFormat::R16Float, "Press A");
        let (press_b, press_view_b) = create_texture(device, width, height, wgpu::TextureFormat::R16Float, "Press B");
        let (div, div_view) = create_texture(device, width, height, wgpu::TextureFormat::R16Float, "Divergence");

        let (text, text_view) = create_texture(device, width, height, wgpu::TextureFormat::R8Unorm, "Text");

        let params = SimulationParams {
            width,
            height,
            dt: 0.1,
            dx: 1.0,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_active: 0,
            time: 0.0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let common_bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Common Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let common_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Common Bind Group"),
            layout: &common_bg_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });

        // Load Shader
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/fluid.wgsl"));

        // Helper to create layout and pipeline
        let create_pipeline = |entry_point: &str, entries: &[wgpu::BindGroupLayoutEntry]| -> (wgpu::BindGroupLayout, wgpu::ComputePipeline) {
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("{} Layout", entry_point)),
                entries,
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("{} Pipeline Layout", entry_point)),
                bind_group_layouts: &[&common_bg_layout, &layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(&format!("{} Pipeline", entry_point)),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point,
            });
            (layout, pipeline)
        };

        // Layouts
        let ro_texture = wgpu::BindingType::Texture { sample_type: wgpu::TextureSampleType::Float { filterable: true }, view_dimension: wgpu::TextureViewDimension::D2, multisampled: false };
        let wo_storage_r = wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::R16Float, view_dimension: wgpu::TextureViewDimension::D2 };
        let wo_storage_rg = wgpu::BindingType::StorageTexture { access: wgpu::StorageTextureAccess::WriteOnly, format: wgpu::TextureFormat::Rg16Float, view_dimension: wgpu::TextureViewDimension::D2 };

        // Advect Vel: In(0), Out(1)
        let (advect_vel_layout, advect_vel_pipeline) = create_pipeline("advect_velocity", &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_rg, count: None },
        ]);

        // Advect Rho: VelIn(0), RhoIn(2), RhoOut(3)
        let (advect_rho_layout, advect_rho_pipeline) = create_pipeline("advect_density", &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_r, count: None },
        ]);

        // Divergence: VelIn(0), DivOut(4)
        let (divergence_layout, divergence_pipeline) = create_pipeline("divergence", &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None }, // Using unfilterable for load? Shader uses load.
            wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_r, count: None },
        ]);
        // Note: Shader uses textureLoad, which works with Float { filterable: true } too, or { filterable: false }.
        // My create_texture uses floats.

        // Jacobi: PressIn(5), PressOut(6), DivIn(7)
        let (jacobi_layout, jacobi_pipeline) = create_pipeline("jacobi", &[
            wgpu::BindGroupLayoutEntry { binding: 5, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 6, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_r, count: None },
            wgpu::BindGroupLayoutEntry { binding: 7, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
        ]);

        // Subtract: PressIn(5), VelIn(0), VelOut(1)
        let (subtract_layout, subtract_pipeline) = create_pipeline("subtract_gradient", &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_rg, count: None },
            wgpu::BindGroupLayoutEntry { binding: 5, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
        ]);

        // Inject: VelIn(0), VelOut(1), RhoIn(2), RhoOut(3), TextIn(8)
        let (inject_layout, inject_pipeline) = create_pipeline("inject", &[
            wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_rg, count: None },
            wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
            wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE, ty: wo_storage_r, count: None },
            wgpu::BindGroupLayoutEntry { binding: 8, visibility: wgpu::ShaderStages::COMPUTE, ty: ro_texture, count: None },
        ]);

        Self {
            vel_texture: [vel_a, vel_b],
            vel_view: [vel_view_a, vel_view_b],
            rho_texture: [rho_a, rho_b],
            rho_view: [rho_view_a, rho_view_b],
            press_texture: [press_a, press_b],
            press_view: [press_view_a, press_view_b],
            div_texture: div,
            div_view: div_view,
            text_texture: text,
            text_view,
            params,
            uniform_buffer,
            common_bind_group,
            advect_vel_pipeline,
            advect_rho_pipeline,
            divergence_pipeline,
            jacobi_pipeline,
            subtract_pipeline,
            inject_pipeline,
            bind_group_layouts: BindGroupLayouts {
                advect_vel: advect_vel_layout,
                advect_rho: advect_rho_layout,
                divergence: divergence_layout,
                jacobi: jacobi_layout,
                subtract: subtract_layout,
                inject: inject_layout,
            },
            frame_count: 0,
        }
    }

    pub fn update_uniforms(&mut self, queue: &wgpu::Queue) {
        self.params.time += 0.016;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.params]));
    }

    pub fn update_text(&self, queue: &wgpu::Queue, data: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.text_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.params.width),
                rows_per_image: Some(self.params.height),
            },
            wgpu::Extent3d {
                width: self.params.width,
                height: self.params.height,
                depth_or_array_layers: 1,
            }
        );
    }

    pub fn step(&mut self, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device) {
        let w = (self.params.width + 15) / 16;
        let h = (self.params.height + 15) / 16;

        let cur = (self.frame_count % 2) as usize;
        let next = 1 - cur;

        // 1. Inject: Cur -> Next
        let bg_inject = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Inject BG"),
            layout: &self.bind_group_layouts.inject,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.vel_view[cur]) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&self.vel_view[next]) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&self.rho_view[cur]) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&self.rho_view[next]) },
                wgpu::BindGroupEntry { binding: 8, resource: wgpu::BindingResource::TextureView(&self.text_view) },
            ],
        });

        // 2. Advect Velocity: Next -> Cur (reuse Cur as temp target)
        let bg_advect_vel = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Advect Vel BG"),
            layout: &self.bind_group_layouts.advect_vel,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.vel_view[next]) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&self.vel_view[cur]) },
            ],
        });

        // 3. Advect Density: Next(Vel), Next(Rho) -> Cur(Rho)
        let bg_advect_rho = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Advect Rho BG"),
            layout: &self.bind_group_layouts.advect_rho,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.vel_view[next]) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&self.rho_view[next]) },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&self.rho_view[cur]) },
            ],
        });

        // 4. Divergence: Cur(Vel) -> Div
        let bg_div = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Divergence BG"),
            layout: &self.bind_group_layouts.divergence,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.vel_view[cur]) },
                wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&self.div_view) },
            ],
        });

        // 5. Jacobi Bind Groups (Ping-Pong)
        let bg_jacobi_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Jacobi BG 0"),
            layout: &self.bind_group_layouts.jacobi,
            entries: &[
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&self.press_view[0]) },
                wgpu::BindGroupEntry { binding: 6, resource: wgpu::BindingResource::TextureView(&self.press_view[1]) },
                wgpu::BindGroupEntry { binding: 7, resource: wgpu::BindingResource::TextureView(&self.div_view) },
            ],
        });
        let bg_jacobi_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Jacobi BG 1"),
            layout: &self.bind_group_layouts.jacobi,
            entries: &[
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&self.press_view[1]) },
                wgpu::BindGroupEntry { binding: 6, resource: wgpu::BindingResource::TextureView(&self.press_view[0]) },
                wgpu::BindGroupEntry { binding: 7, resource: wgpu::BindingResource::TextureView(&self.div_view) },
            ],
        });

        // 6. Subtract Gradient: Cur(Vel), P_Last -> Next(Vel)
        let bg_sub = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Subtract BG"),
            layout: &self.bind_group_layouts.subtract,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&self.vel_view[cur]) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&self.vel_view[next]) },
                wgpu::BindGroupEntry { binding: 5, resource: wgpu::BindingResource::TextureView(&self.press_view[0]) }, // Assuming 20 iter ends with result in 0
            ],
        });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Simulation Compute Pass"),
                timestamp_writes: None,
            });

            cpass.set_bind_group(0, &self.common_bind_group, &[]);

            // Inject
            cpass.set_pipeline(&self.inject_pipeline);
            cpass.set_bind_group(1, &bg_inject, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            // Advect Vel
            cpass.set_pipeline(&self.advect_vel_pipeline);
            cpass.set_bind_group(1, &bg_advect_vel, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            // Advect Rho
            cpass.set_pipeline(&self.advect_rho_pipeline);
            cpass.set_bind_group(1, &bg_advect_rho, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            // Divergence
            cpass.set_pipeline(&self.divergence_pipeline);
            cpass.set_bind_group(1, &bg_div, &[]);
            cpass.dispatch_workgroups(w, h, 1);

            // Jacobi
            cpass.set_pipeline(&self.jacobi_pipeline);
            for i in 0..20 {
                let bg = if i % 2 == 0 { &bg_jacobi_0 } else { &bg_jacobi_1 };
                cpass.set_bind_group(1, bg, &[]);
                cpass.dispatch_workgroups(w, h, 1);
            }

            // Subtract
            cpass.set_pipeline(&self.subtract_pipeline);
            cpass.set_bind_group(1, &bg_sub, &[]);
            cpass.dispatch_workgroups(w, h, 1);
        }
        // Wait, I want Result in Next(Vel) and Next(Rho).
        // I advected Rho to Cur in step 3.
        // I can just copy Cur(Rho) to Next(Rho) or just treat Cur as Next for Rho?
        // Let's just assume Next is the definitive state for the next frame.
        // So I need to move Cur(Rho) to Next(Rho).
        // Or I just swap expectations.

        // Simpler:
        // Frame start: state in `cur`.
        // Result: state in `next`.
        // Step 1 Inject: cur -> next (Vel & Rho).
        // Step 2 Advect Vel: next -> cur (temp).
        // Step 3 Advect Rho: next(Vel), next(Rho) -> cur(Rho) (temp). (Using original vel for advection or advected vel?)
        // Standard: Advect Vel first. Use advected velocity to advect density? No, usually use old velocity to advect both.
        // Or use Self-Advection for Velocity.

        // Let's simplify logic:
        // 1. Advect Vel: Cur -> Next.
        // 2. Advect Rho: Cur -> Next.
        // 3. Inject: Next -> Next (In-place? Needs ReadWrite or Copy).
        //    Can't do in-place easily.
        //    So: Cur -> Inject -> Temp. Temp -> Advect -> Next.

        // My current logic:
        // 1. Inject: Cur -> Next.
        // 2. Advect Vel: Next -> Cur. (Now Cur has Advected Vel).
        // 3. Advect Rho: Next -> Cur. (Now Cur has Advected Rho).
        // 4. Project Cur -> Next.

        // Final state is Next.
        // So I should increment frame_count such that next frame uses `next` as `cur`.
        // `cur` was 0. `next` was 1.
        // Result is in `vel_view[next]` (from Subtract) and `rho_view[cur]` (from Advect).
        // This is mismatched.

        // I need Rho to be in `next` too.
        // I can run a copy pass or just Advect Rho to `next`?
        // But Advect needs `source` and `dest`.
        // If I advect `Next(Rho)` (from Inject) to `Cur(Rho)`, I am back in Cur.

        // Let's fix Step 3:
        // Advect Rho: Next(Vel) [which is injected], Next(Rho) [injected] -> Next(Rho)? No, can't read/write same.
        // So Advect Rho: Next(Rho) -> Cur(Rho).

        // So Rho is in Cur. Vel is in Next.
        // I can add a Copy pass for Rho: Cur -> Next.
        // Or just realize that Rho doesn't participate in Pressure, so it's fine.
        // But for the next frame, `cur` becomes `next`.
        // So I expect `rho_view[next]` to have the data.

        // Use `encoder.copy_texture_to_texture`.
        // Cur(Rho) -> Next(Rho).
    }

    // Call this after step
    pub fn finish_step(&mut self, encoder: &mut wgpu::CommandEncoder) {
         let cur = (self.frame_count % 2) as usize;
         let next = 1 - cur;
         // Copy Rho from Cur to Next because my advection put it in Cur
         // Wait, step 3 put it in Cur.

         encoder.copy_texture_to_texture(
             wgpu::ImageCopyTexture {
                 texture: &self.rho_texture[cur],
                 mip_level: 0,
                 origin: wgpu::Origin3d::ZERO,
                 aspect: wgpu::TextureAspect::All,
             },
             wgpu::ImageCopyTexture {
                 texture: &self.rho_texture[next],
                 mip_level: 0,
                 origin: wgpu::Origin3d::ZERO,
                 aspect: wgpu::TextureAspect::All,
             },
             wgpu::Extent3d {
                 width: self.params.width,
                 height: self.params.height,
                 depth_or_array_layers: 1,
             }
         );

         self.frame_count += 1;
    }

    pub fn get_current_view(&self) -> &wgpu::TextureView {
        let cur = (self.frame_count % 2) as usize;
        // We just incremented frame_count. So the valid data is in `prev`?
        // If I increment at end of step, then `cur` points to the NEW state.
        // In `step` I used `cur` as input and `next` as output mostly.
        // `finish_step` increments.
        // So if I call get_current_view BEFORE finish_step, I see input.
        // If AFTER, I see output?
        // Let's say frame 0: Input 0. Output 1. Finish: count=1.
        // Next frame 1: Input 1.
        // So valid data is at `frame_count % 2`.
        &self.rho_view[cur]
    }
}

fn create_texture(device: &wgpu::Device, width: u32, height: u32, format: wgpu::TextureFormat, label: &str) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}
