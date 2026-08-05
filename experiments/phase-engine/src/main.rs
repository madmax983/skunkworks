//! # Phase Engine
//!
//! **"Crystallization is just a loss of entropy."**
//!
//! A real-time simulation of phase transitions (Solid <-> Liquid) using Langevin Dynamics.
//! Particles are simulated on the CPU (using `rayon` for parallelism) and rendered on the GPU (using `wgpu` instancing).
//!
//! ## Concept
//!
//! Each particle is tethered to a lattice position by a spring force (simulating solid state) but perturbed by random thermal noise (simulating temperature).
//!
//! - **Low Temperature**: Spring forces dominate. Particles vibrate around lattice points (Crystal/Solid).
//! - **High Temperature**: Thermal noise dominates. Particles break free and move randomly (Liquid/Gas).
//!
//! Equation of Motion (Langevin):
//! $$ F = -k(x - x_{lattice}) - \gamma v + \eta(t) $$
//! Where $\eta(t)$ is white noise proportional to Temperature.
//!
//! ## Tech Stack
//! - **wgpu**: Graphics API.
//! - **rayon**: Parallel CPU simulation.
//! - **cgmath**: Vector math.
//! - **winit**: Windowing.
//!
//! ## Controls
//! - **Arrow Up**: Increase Temperature (Melt).
//! - **Arrow Down**: Decrease Temperature (Freeze).
//! - **R**: Reset Simulation.
//! - **W/A/S/D**: Move Camera.
//! - **Space/Shift**: Move Up/Down.
//!
mod renderer;
mod simulation;

use anyhow::Result;
use cgmath::InnerSpace;
use log::{error, info};
use renderer::State;
use simulation::Simulation;
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
    // Grid size 20 -> 8000 atoms
    let grid_size = 20;
    let mut sim = Simulation::new(grid_size);
    info!("Generated {} particles", sim.particles.len());

    let mut state = State::new(None, 800, 600, &sim).await;

    // Position camera
    state.camera.eye = (30.0, 30.0, 30.0).into();
    state.camera.target = (0.0, 0.0, 0.0).into();

    // Run a few updates to warm up
    for _ in 0..100 {
        sim.update(0.016, 0.5); // Warm up
    }

    state.update(&sim);

    let image = state.render_headless().await;
    image.save("output.png")?;
    info!("Saved output.png");
    Ok(())
}

async fn run_window() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Genesis: Phase Engine (Arrow Up/Down: Temp)")
        .build(&event_loop)?;

    let window = Arc::new(window);

    info!("Initializing Simulation...");
    let grid_size = 30; // 27000 particles
    let mut sim = Simulation::new(grid_size);
    info!("Generated {} particles", sim.particles.len());

    let mut state = State::new(Some(window.clone()), 800, 600, &sim).await;

    let mut temperature: f32 = 0.05; // Start cold (crystalline)
    let mut last_render_time = std::time::Instant::now();

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
                        let speed = 2.0;

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
                            KeyCode::ArrowUp => {
                                temperature += 0.05;
                                info!("Temperature: {:.2}", temperature);
                                window.set_title(&format!(
                                    "Genesis: Phase Engine (Temp: {:.2})",
                                    temperature
                                ));
                            }
                            KeyCode::ArrowDown => {
                                temperature = (temperature - 0.05).max(0.0);
                                info!("Temperature: {:.2}", temperature);
                                window.set_title(&format!(
                                    "Genesis: Phase Engine (Temp: {:.2})",
                                    temperature
                                ));
                            }
                            KeyCode::KeyR => {
                                info!("Resetting simulation");
                                sim = Simulation::new(grid_size);
                                temperature = 0.05;
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = (now - last_render_time).as_secs_f32();
                        last_render_time = now;

                        // Clamp dt to avoid explosion if lag
                        let sim_dt = dt.min(0.05);

                        sim.update(sim_dt, temperature);
                        state.update(&sim);

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
