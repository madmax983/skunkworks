mod math;
mod renderer;

use anyhow::Result;
use cgmath::{InnerSpace, Point3, Vector2, MetricSpace};
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
    pollster::block_on(run())
}

async fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Genesis: Penrose Rogue")
        .build(&event_loop)?;

    let window = Arc::new(window);

    let mut state = State::new(Some(window.clone()), 800, 600).await;

    // Game State
    let mut current_tile_idx = 0;

    // Find a nice starting tile (e.g. center)
    // Center is usually at index 0?
    // math::generate_sun centers at (0,0).
    // Let's verify.
    if !state.rhombuses.is_empty() {
        // Find closest to (0,0)
        let mut min_dist = f32::MAX;
        for (i, r) in state.rhombuses.iter().enumerate() {
            let d = r.center.distance2(cgmath::Point2::new(0.0, 0.0));
            if d < min_dist {
                min_dist = d;
                current_tile_idx = i;
            }
        }
    }

    state.player_pos = [state.rhombuses[current_tile_idx].center.x, state.rhombuses[current_tile_idx].center.y];
    state.camera.position = Point3::new(state.player_pos[0], state.player_pos[1], 10.0);
    state.update();

    info!("Starting game loop. Tile count: {}", state.rhombuses.len());

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
                        // Movement
                        let mut direction = Vector2::new(0.0, 0.0);
                        match keycode {
                            KeyCode::KeyW | KeyCode::ArrowUp => direction.y += 1.0,
                            KeyCode::KeyS | KeyCode::ArrowDown => direction.y -= 1.0,
                            KeyCode::KeyA | KeyCode::ArrowLeft => direction.x -= 1.0,
                            KeyCode::KeyD | KeyCode::ArrowRight => direction.x += 1.0,
                            // Zoom
                            KeyCode::KeyQ => state.camera.zoom *= 1.1,
                            KeyCode::KeyE => state.camera.zoom /= 1.1,
                            _ => {}
                        }

                        if direction.magnitude() > 0.0 {
                            direction = direction.normalize();

                            // Find best neighbor
                            let current_center = state.rhombuses[current_tile_idx].center;
                            let mut best_neighbor = None;
                            let mut max_dot = -1.0; // Angle threshold?

                            for &neighbor_idx in &state.adjacency[current_tile_idx] {
                                let neighbor_center = state.rhombuses[neighbor_idx].center;
                                let dir_to_neighbor = (neighbor_center - current_center).normalize();
                                let dot = direction.dot(dir_to_neighbor);

                                if dot > max_dot {
                                    max_dot = dot;
                                    best_neighbor = Some(neighbor_idx);
                                }
                            }

                            if let Some(n_idx) = best_neighbor {
                                // Threshold to ensure we don't move backwards or sideways weirdly
                                // 45 degrees -> dot > 0.707
                                if max_dot > 0.5 {
                                    current_tile_idx = n_idx;
                                    let new_center = state.rhombuses[current_tile_idx].center;
                                    state.player_pos = [new_center.x, new_center.y];

                                    // Smooth camera follow? Instant for now.
                                    state.camera.position.x = new_center.x;
                                    state.camera.position.y = new_center.y;

                                    info!("Moved to tile {}", current_tile_idx);
                                }
                            }
                        }

                        window.request_redraw();
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
                // window.request_redraw(); // Continuous redraw? No, save battery.
                // Redraw only on input.
            }
            _ => {}
        }
    })?;

    Ok(())
}
