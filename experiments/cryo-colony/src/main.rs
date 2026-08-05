//! # Cryo-Colony ❄️🐜
//!
//! **Genesis: Cryo-Colony** is a hybrid experiment simulating agents ("Thermites") that manipulate the phase state of their environment.
//!
//! ## Lineage 🧬
//!
//! *   **Parent A**: `phase-engine` (Genesis/Crystallographer)
//!     *   *Inheritance*: WGPU Instanced Rendering, Langevin Dynamics, Phase Transition Physics (Solid <-> Liquid).
//! *   **Parent B**: `thermo-termites` (Genesis/Mycelium)
//!     *   *Inheritance*: Agent-based logic, modification of local environment variables (Temperature).
//!
//! ## Concept
//!
//! Agents inhabit a crystalline lattice.
//! *   **Coolers (Cyan)**: Lower local temperature, freezing liquid into solid (restoring lattice structure).
//! *   **Heaters (Yellow)**: Raise local temperature, melting solid into liquid (allowing flow and tunneling).
//!
//! The environment reacts to temperature:
//! *   **Solid (Blue)**: Particles are tethered to their lattice positions by strong springs.
//! *   **Liquid (Red)**: Particles are loosely tethered and move chaotically due to thermal noise.
//!
//! ## Controls
//!
//! *   **Arrow Up/Down**: Increase/Decrease global ambient temperature.
//! *   **R**: Reset simulation.
//! *   **W/A/S/D/Space/Shift**: Camera movement.
//!
//! ## Implementation Details
//!
//! *   **Simulation**: `src/simulation.rs` - Parallel update of particles and agents using `rayon`.
//! *   **Rendering**: `src/renderer.rs` - WGPU instanced rendering. Instances are colored based on their state (Phase) or type (Agent).
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
        .with_title("Genesis: Cryo-Colony (Arrow Up/Down: Temp, R: Reset)")
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
                                info!("Ambient Temp: {:.2}", temperature);
                                window.set_title(&format!(
                                    "Genesis: Cryo-Colony (Ambient: {:.2})",
                                    temperature
                                ));
                            }
                            KeyCode::ArrowDown => {
                                temperature = (temperature - 0.05).max(0.0);
                                info!("Ambient Temp: {:.2}", temperature);
                                window.set_title(&format!(
                                    "Genesis: Cryo-Colony (Ambient: {:.2})",
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
