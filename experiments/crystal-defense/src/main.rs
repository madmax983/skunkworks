mod dungeon;
mod math;
mod renderer;
mod sim;

use anyhow::Result;
use cgmath::InnerSpace;
use cgmath::Vector3;
use dungeon::Dungeon;
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
    info!("Generated {} edges", qc.edges.len());

    let dungeon = Dungeon::new(qc);
    let mut state = State::new(None, 800, 600, dungeon).await;

    // Position camera inside the crystal
    state.camera.eye = (2.0, 2.0, 2.0).into();
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
        .with_title("Genesis: Crystal Defense")
        .build(&event_loop)?;

    let window = Arc::new(window);

    info!("Generating Quasicrystal Lattice...");
    let qc = math::generate_icosahedral_lattice(2); // Radius 2 for playable FPS with agents
    info!("Generated {} atoms", qc.atoms.len());
    info!("Generated {} edges", qc.edges.len());

    let dungeon = Dungeon::new(qc);
    let mut state = State::new(Some(window.clone()), 800, 600, dungeon).await;

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
                        let speed = 0.5;

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
                            // Cursor Controls
                            KeyCode::Tab => {
                                // Cycle neighbor
                                let current_player = state.dungeon.player_idx;
                                let neighbors = &state.dungeon.world.qc.adj[current_player];
                                if !neighbors.is_empty() {
                                    let next_sel =
                                        if let Some(current_sel) = state.selected_neighbor_idx {
                                            if let Some(pos) =
                                                neighbors.iter().position(|&n| n == current_sel)
                                            {
                                                neighbors[(pos + 1) % neighbors.len()]
                                            } else {
                                                neighbors[0]
                                            }
                                        } else {
                                            neighbors[0]
                                        };
                                    state.selected_neighbor_idx = Some(next_sel);
                                    // Visuals updated in loop
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(target) = state.selected_neighbor_idx {
                                    if state.dungeon.move_player(target) {
                                        info!("Moved Cursor to {}", target);

                                        // Spawn tower or interaction?
                                        // For now just move camera focus
                                        state.selected_neighbor_idx = None;

                                        let p = state.dungeon.world.qc.atoms[state.dungeon.player_idx];
                                        let offset = Vector3::new(2.0, 2.0, 2.0);
                                        state.camera.target = p;
                                        state.camera.eye = p + offset;
                                    }
                                }
                            }
                            // Debug Spawn
                            KeyCode::KeyT => {
                                info!("Spawning Termite wave");
                                // Manually trigger termite spawn via some debug flag?
                                // Actually, simulation handles it.
                            }
                            _ => {}
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        // Update Simulation
                        state.dungeon.update();

                        // Update Visuals (every frame? might be heavy)
                        state.update_dungeon_visuals();

                        state.update(); // Camera buffer
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
