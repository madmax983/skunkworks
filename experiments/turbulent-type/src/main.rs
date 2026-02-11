mod state;
mod simulation;
mod renderer;
mod text;

use anyhow::Result;
use log::{error, info};
use state::State;
use simulation::Simulation;
use renderer::Renderer;
use text::TextGenerator;
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
            .with_title("Genesis: Turbulent Type")
            .build(&event_loop)?,
    );

    let mut state = pollster::block_on(State::new(window.clone()));
    let mut simulation = Simulation::new(&state.device, &state.queue, state.size.width, state.size.height);
    let renderer = Renderer::new(&state.device, state.config.format);

    let text_gen = TextGenerator::new(state.size.width, state.size.height);
    let mut current_text = String::from("Genesis");
    // Initial update
    let data = text_gen.generate_texture(&current_text);
    simulation.update_text(&state.queue, &data);

    info!("Turbulent Type initialized.");
    info!("Type to add text. Scroll to swirl. Click to inject dye.");

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            match event.physical_key {
                                PhysicalKey::Code(KeyCode::Escape) => elwt.exit(),
                                PhysicalKey::Code(KeyCode::Backspace) => {
                                    current_text.pop();
                                    let data = text_gen.generate_texture(&current_text);
                                    simulation.update_text(&state.queue, &data);
                                }
                                _ => {
                                    if let Some(txt) = &event.text {
                                         if !txt.chars().any(|c| c.is_control()) {
                                             current_text.push_str(txt);
                                             let data = text_gen.generate_texture(&current_text);
                                             simulation.update_text(&state.queue, &data);
                                         }
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        simulation.params.mouse_x = (position.x / state.size.width as f64) as f32;
                        simulation.params.mouse_y = (position.y / state.size.height as f64) as f32;
                    }
                    WindowEvent::MouseInput { state: mstate, button, .. } => {
                        match button {
                            MouseButton::Left => simulation.params.mouse_active = if *mstate == ElementState::Pressed { 1 } else { 0 },
                            MouseButton::Right => simulation.params.mouse_active = if *mstate == ElementState::Pressed { 2 } else { 0 },
                            _ => {}
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                        // Simulation doesn't resize, but we update text generator just in case we want to support it later?
                        // For now fixed size.
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
                        simulation.step(&mut encoder, &state.device);
                        simulation.finish_step(&mut encoder);

                        // Render Step
                        let current_sim_view = simulation.get_current_view();
                        renderer.render(&state.device, &view, &mut encoder, Some(current_sim_view));

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
