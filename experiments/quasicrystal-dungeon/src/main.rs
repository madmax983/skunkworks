mod math;
mod renderer;

use anyhow::Result;
use cgmath::InnerSpace;
use log::{error, info};
use renderer::State;
use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if headless {
        pollster::block_on(run_headless())?;
    } else {
        pollster::block_on(run_window())?;
    }

    Ok(())
}

async fn run_headless() -> Result<()> {
    info!("Running in headless mode...");
    // Radius 3 -> ~100k atoms
    let qc = math::generate_icosahedral_lattice(3);
    info!("Generated {} atoms", qc.atoms.len());

    let mut state = State::new(None, 800, 600, &qc).await;

    // Position camera
    state.camera.eye = (10.0, 10.0, 10.0).into();
    state.camera.target = (0.0, 0.0, 0.0).into();
    state.update();

    let image = state.render_headless().await;
    image.save("output.png")?;
    info!("Saved output.png");
    Ok(())
}

async fn run_window() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Genesis: Quasicrystal Dungeon")
        .build(&event_loop)?;

    let window = Arc::new(window);

    info!("Generating Quasicrystal...");
    let qc = math::generate_icosahedral_lattice(3);
    info!("Generated {} atoms", qc.atoms.len());

    let mut state = State::new(Some(window.clone()), 800, 600, &qc).await;

    event_loop.run(move |event, target| {
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
                    } => target.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize((physical_size.width, physical_size.height));
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(keycode),
                                ..
                            },
                        ..
                    } => {
                        // Camera controls
                        let forward = (state.camera.target - state.camera.eye).normalize();
                        let right = forward.cross(state.camera.up).normalize();
                        let speed = 1.0;

                        match keycode {
                            KeyCode::KeyW => {
                                state.camera.eye += forward * speed;
                                state.camera.target += forward * speed;
                            }
                            KeyCode::KeyS => {
                                state.camera.eye -= forward * speed;
                                state.camera.target -= forward * speed;
                            }
                            KeyCode::KeyA => {
                                state.camera.eye -= right * speed;
                                state.camera.target -= right * speed;
                            }
                            KeyCode::KeyD => {
                                state.camera.eye += right * speed;
                                state.camera.target += right * speed;
                            }
                            KeyCode::Space => {
                                state.camera.eye.y += speed;
                                state.camera.target.y += speed;
                            }
                            KeyCode::ShiftLeft => {
                                state.camera.eye.y -= speed;
                                state.camera.target.y -= speed;
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
                            Err(e) => error!("{:?}", e),
                        }
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
