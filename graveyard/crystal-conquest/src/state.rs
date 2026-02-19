use crate::renderer::Renderer;
use crate::simulation::Simulation;
use anyhow::Result;
use std::sync::Arc;
use winit::window::Window;

pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub _window: Arc<Window>,
    pub simulation: Simulation,
    pub renderer: Renderer,

    // Camera / Interaction State
    pub mouse_pressed: bool,
    pub mouse_pos: (f64, f64),
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub camera_dist: f32,
    pub _temperature: f32,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("No adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
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

        let simulation = Simulation::new(200); // 200 particles
        let renderer = Renderer::new(&device, &config, &simulation);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            _window: window,
            simulation,
            renderer,
            mouse_pressed: false,
            mouse_pos: (0.0, 0.0),
            camera_yaw: 0.0,
            camera_pitch: 0.0,
            camera_dist: 3.0,
            _temperature: 0.0,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.resize(&self.device, &self.config); // Delegate to renderer if needed
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        match event {
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x, position.y);
                false
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                if *button == winit::event::MouseButton::Left {
                    self.mouse_pressed = *state == winit::event::ElementState::Pressed;
                }
                false
            }
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        self.camera_dist -= y * 0.1;
                    }
                    winit::event::MouseScrollDelta::PixelDelta(pos) => {
                        self.camera_dist -= pos.y as f32 * 0.01;
                    }
                }
                self.camera_dist = self.camera_dist.clamp(1.5, 10.0);
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        // Handle input for camera rotation if mouse pressed
        // For simplicity, just auto-rotate or assume input handler does it
        // Or implement simple drag logic here?
        // Let's assume input updates state, but mouse drag logic:
        // Proper mouse drag needs `DeviceEvent` or storing previous mouse pos.
        // For now, let's keep it simple: WASD or arrows in main loop?
        // Or just `camera_yaw += 0.01` auto rotate.
        self.camera_yaw += 0.005;

        // Update simulation
        // Temperature logic
        // Inject random noise based on temp

        self.simulation.update(0.016);

        // Update buffers
        self.renderer.update(
            &self.queue,
            &self.simulation,
            self.camera_yaw,
            self.camera_pitch,
            self.camera_dist,
            self.size,
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

        self.renderer.render(&mut encoder, &view);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
