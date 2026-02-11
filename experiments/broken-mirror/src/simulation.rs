use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};
use rand::Rng;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SimulationParams {
    pub temperature: f32,
    pub width: u32,
    pub height: u32,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_active: u32,
    pub time: f32,
    pub _padding: u32,
}

pub struct Simulation {
    pub texture_a: wgpu::Texture,
    pub texture_b: wgpu::Texture,
    pub view_a: wgpu::TextureView,
    pub view_b: wgpu::TextureView,
    bind_group_a_to_b: wgpu::BindGroup, // Read A, Write B
    bind_group_b_to_a: wgpu::BindGroup, // Read B, Write A
    pipeline: wgpu::ComputePipeline,
    uniform_buffer: wgpu::Buffer,
    pub params: SimulationParams,
    pub frame_count: u64,
}

impl Simulation {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> Self {
        // Create random initial data
        let mut rng = rand::thread_rng();
        let size = (width * height) as usize;
        let mut data = Vec::with_capacity(size);
        for _ in 0..size {
            data.push(rng.gen::<f32>() * std::f32::consts::PI * 2.0);
        }
        let data_bytes = bytemuck::cast_slice(&data);

        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Simulation Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R32Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        };

        // Initialize A with data
        let texture_a = device.create_texture_with_data(queue, &texture_desc, wgpu::util::TextureDataOrder::LayerMajor, data_bytes);
        // B is empty initially
        let texture_b = device.create_texture(&texture_desc);

        let view_a = texture_a.create_view(&wgpu::TextureViewDescriptor::default());
        let view_b = texture_b.create_view(&wgpu::TextureViewDescriptor::default());

        // Uniforms
        let params = SimulationParams {
            temperature: 0.5,
            width,
            height,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_active: 0,
            time: 0.0,
            _padding: 0,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simulation Uniform Buffer"),
            contents: bytemuck::cast_slice(&[params]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("compute.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                // Input Texture (Read)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Output Texture (Write)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        // Create Bind Groups
        // A -> B
        let bind_group_a_to_b = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group A -> B"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view_a),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view_b),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        // B -> A
        let bind_group_b_to_a = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group B -> A"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view_b),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view_a),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            texture_a,
            texture_b,
            view_a,
            view_b,
            bind_group_a_to_b,
            bind_group_b_to_a,
            pipeline,
            uniform_buffer,
            params,
            frame_count: 0,
        }
    }

    pub fn update_uniforms(&mut self, queue: &wgpu::Queue) {
        self.params.time += 0.016;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.params]));
    }

    pub fn step(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let bind_group = if self.frame_count % 2 == 0 {
            &self.bind_group_a_to_b
        } else {
            &self.bind_group_b_to_a
        };

        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Simulation Compute Pass"),
            timestamp_writes: None,
        });

        cpass.set_pipeline(&self.pipeline);
        cpass.set_bind_group(0, bind_group, &[]);

        let workgroup_size_x = 16;
        let workgroup_size_y = 16;
        let dispatch_x = (self.params.width + workgroup_size_x - 1) / workgroup_size_x;
        let dispatch_y = (self.params.height + workgroup_size_y - 1) / workgroup_size_y;

        cpass.dispatch_workgroups(dispatch_x, dispatch_y, 1);

        self.frame_count += 1;
    }

    pub fn get_current_view(&self) -> &wgpu::TextureView {
        if self.frame_count % 2 != 0 {
            &self.view_b
        } else {
            &self.view_a
        }
    }
}
