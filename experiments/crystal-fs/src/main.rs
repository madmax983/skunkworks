use cgmath::prelude::*;
use std::iter;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crystal_fs::scanner;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    rot_4d: [[f32; 4]; 4],
}

struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: std::sync::Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,

    // Camera
    camera_pos: cgmath::Point3<f32>,
    camera_yaw: f32,
    camera_pitch: f32,

    // 4D Rotation angles
    rot_xw: f32,
    rot_yw: f32,
    rot_zw: f32,

    // Input State
    keys_down: std::collections::HashSet<KeyCode>,
    mouse_pressed: bool,
}

impl State {
    async fn new(window: std::sync::Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
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

        let path = std::env::current_dir().unwrap();
        let points = scanner::scan(&path);
        let vertices: Vec<Vertex> = points
            .iter()
            .map(|p| Vertex {
                position: p.position,
                color: p.color,
            })
            .collect();

        let num_vertices = vertices.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let uniform_bind_group_layout =
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
                label: Some("uniform_bind_group_layout"),
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let uniforms = Uniforms {
            view_proj: cgmath::Matrix4::identity().into(),
            rot_4d: cgmath::Matrix4::identity().into(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            uniform_buffer,
            uniform_bind_group,
            camera_pos: cgmath::Point3::new(0.0, 0.0, 5.0),
            camera_yaw: -90.0, // Face -Z
            camera_pitch: 0.0,
            rot_xw: 0.0,
            rot_yw: 0.0,
            rot_zw: 0.0,
            keys_down: std::collections::HashSet::new(),
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

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = *state == ElementState::Pressed;
                if pressed {
                    self.keys_down.insert(*key);
                } else {
                    self.keys_down.remove(key);
                }
                true
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        let dt = 0.016; // Assume 60fps
        let speed = 2.0 * dt;
        let rot_speed = 1.0 * dt;

        // Camera Rotation (Simple, fixed keys for now: Arrows)
        if self.keys_down.contains(&KeyCode::ArrowLeft) {
            self.camera_yaw -= rot_speed * 50.0;
        }
        if self.keys_down.contains(&KeyCode::ArrowRight) {
            self.camera_yaw += rot_speed * 50.0;
        }
        if self.keys_down.contains(&KeyCode::ArrowUp) {
            self.camera_pitch += rot_speed * 50.0;
        }
        if self.keys_down.contains(&KeyCode::ArrowDown) {
            self.camera_pitch -= rot_speed * 50.0;
        }

        // Clamp pitch
        if self.camera_pitch > 89.0 {
            self.camera_pitch = 89.0;
        }
        if self.camera_pitch < -89.0 {
            self.camera_pitch = -89.0;
        }

        let (yaw_sin, yaw_cos) = cgmath::Rad::from(cgmath::Deg(self.camera_yaw)).0.sin_cos();
        let (pitch_sin, pitch_cos) = cgmath::Rad::from(cgmath::Deg(self.camera_pitch))
            .0
            .sin_cos();

        let front =
            cgmath::Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();

        let right = front.cross(cgmath::Vector3::unit_y()).normalize();
        let up = right.cross(front).normalize();

        // Camera Movement
        if self.keys_down.contains(&KeyCode::KeyW) {
            self.camera_pos += front * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyS) {
            self.camera_pos -= front * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyA) {
            self.camera_pos -= right * speed;
        }
        if self.keys_down.contains(&KeyCode::KeyD) {
            self.camera_pos += right * speed;
        }
        if self.keys_down.contains(&KeyCode::Space) {
            self.camera_pos += cgmath::Vector3::unit_y() * speed;
        }
        if self.keys_down.contains(&KeyCode::ShiftLeft) {
            self.camera_pos -= cgmath::Vector3::unit_y() * speed;
        }

        // 4D Rotation (Q/E for XW, R/F for YW, T/G for ZW)
        if self.keys_down.contains(&KeyCode::KeyQ) {
            self.rot_xw += rot_speed;
        }
        if self.keys_down.contains(&KeyCode::KeyE) {
            self.rot_xw -= rot_speed;
        }

        if self.keys_down.contains(&KeyCode::KeyR) {
            self.rot_yw += rot_speed;
        }
        if self.keys_down.contains(&KeyCode::KeyF) {
            self.rot_yw -= rot_speed;
        }

        if self.keys_down.contains(&KeyCode::KeyT) {
            self.rot_zw += rot_speed;
        }
        if self.keys_down.contains(&KeyCode::KeyG) {
            self.rot_zw -= rot_speed;
        }

        // Construct View Matrix
        let view = cgmath::Matrix4::look_at_rh(self.camera_pos, self.camera_pos + front, up);
        let aspect = self.config.width as f32 / self.config.height as f32;
        let proj = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 1000.0);
        let view_proj = proj * view;

        // Construct 4D Rotation Matrix
        // We accumulate rotations.
        // Rot(xw) * Rot(yw) * Rot(zw)

        let cxw = self.rot_xw.cos();
        let sxw = self.rot_xw.sin();
        let cyw = self.rot_yw.cos();
        let syw = self.rot_yw.sin();
        let czw = self.rot_zw.cos();
        let szw = self.rot_zw.sin();

        // R_xw: Rotates x and w
        let mut r_xw = cgmath::Matrix4::identity();
        r_xw.x.x = cxw;
        r_xw.x.w = -sxw;
        r_xw.w.x = sxw;
        r_xw.w.w = cxw;

        // R_yw: Rotates y and w
        let mut r_yw = cgmath::Matrix4::identity();
        r_yw.y.y = cyw;
        r_yw.y.w = -syw;
        r_yw.w.y = syw;
        r_yw.w.w = cyw;

        // R_zw: Rotates z and w
        let mut r_zw = cgmath::Matrix4::identity();
        r_zw.z.z = czw;
        r_zw.z.w = -szw;
        r_zw.w.z = szw;
        r_zw.w.w = czw;

        let rot_4d = r_xw * r_yw * r_zw;

        let uniforms = Uniforms {
            view_proj: view_proj.into(),
            rot_4d: rot_4d.into(),
        };

        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
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
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = std::sync::Arc::new(
        WindowBuilder::new()
            .with_title("Genesis Crystal FS")
            .build(&event_loop)
            .unwrap(),
    );

    let mut state = pollster::block_on(State::new(window.clone()));

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                        WindowEvent::RedrawRequested => {
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                state.window.request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
