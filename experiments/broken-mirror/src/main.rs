mod state;
mod simulation;
mod renderer;

use anyhow::Result;
use log::{error, info};
use state::State;
use simulation::Simulation;
use renderer::Renderer;
use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Genesis: Broken Mirror")
            .build(&event_loop)?,
    );

    let mut state = pollster::block_on(State::new(window.clone()));
    let mut simulation = Simulation::new(&state.device, &state.queue, state.size.width, state.size.height);
    let renderer = Renderer::new(&state.device, state.config.format);

    info!("Broken Mirror initialized.");
    info!("Controls: Scroll (Temp), Left Click (Align), Right Click (Heat), Space (Pause)");

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
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
                    WindowEvent::CursorMoved { position, .. } => {
                        // Map mouse to simulation coordinate space [0, 1]
                        // Note: Simulation size is fixed at initial window size
                        // But we want mouse relative to current window size.
                        simulation.params.mouse_x = (position.x / state.size.width as f64) as f32;
                        simulation.params.mouse_y = (position.y / state.size.height as f64) as f32;
                    }
                    WindowEvent::MouseInput { state: mstate, button, .. } => {
                         match button {
                            MouseButton::Left => {
                                simulation.params.mouse_active = if *mstate == ElementState::Pressed { 1 } else { 0 };
                            }
                            MouseButton::Right => {
                                simulation.params.mouse_active = if *mstate == ElementState::Pressed { 2 } else { 0 };
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            MouseScrollDelta::LineDelta(_, y) => {
                                simulation.params.temperature = (simulation.params.temperature + y * 0.1).max(0.0).min(10.0);
                                info!("Temperature: {:.2}", simulation.params.temperature);
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        simulation.update_uniforms(&state.queue);

                        let output = match state.surface.get_current_texture() {
                            Ok(output) => output,
                            Err(wgpu::SurfaceError::Lost) => {
                                state.resize(state.size);
                                window.request_redraw();
                                return;
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                elwt.exit();
                                return;
                            }
                            Err(e) => {
                                error!("{:?}", e);
                                return;
                            }
                        };

                        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render Encoder"),
                        });

                        // Simulation Step
                        simulation.step(&mut encoder);

                        // Render Step
                        let current_sim_view = simulation.get_current_view();
                        // Render pipeline uses bind group which borrows view.
                        // The renderer.render call creates transient bind group.
                        renderer.render(&state.device, &view, &mut encoder, current_sim_view);

                        state.queue.submit(std::iter::once(encoder.finish()));
                        output.present();
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
