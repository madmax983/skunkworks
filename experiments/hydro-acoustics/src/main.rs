mod audio;
mod sim;

use std::sync::Arc;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};
use sim::Simulation;
use audio::AudioEngine;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    let window = Arc::new(WindowBuilder::new()
        .with_title("Genesis: Hydro-Acoustics")
        .build(&event_loop)?);

    let mut sim = pollster::block_on(Simulation::new(window.clone()));
    let mut cursor_pos = winit::dpi::PhysicalPosition::new(0.0, 0.0);

    // Audio Setup
    let audio_engine = AudioEngine::new();
    let _stream = match audio_engine.start_stream() {
        Ok(s) => Some(s),
        Err(e) => {
            log::warn!("Audio initialization failed: {}. Running in visual-only mode.", e);
            None
        }
    };
    let audio_params = audio_engine.get_params();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CursorMoved { position, .. } => {
                    cursor_pos = *position;
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    // Normalized coords 0..1
                    let size = window.inner_size();
                    let x = cursor_pos.x as f32 / size.width as f32;
                    let y = cursor_pos.y as f32 / size.height as f32;
                    sim.add_drop(x, y);
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Escape),
                            ..
                        },
                    ..
                } => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    sim.resize(*physical_size);
                }
                WindowEvent::RedrawRequested => {
                    sim.update();

                    // Update Audio
                    {
                        let mut params = audio_params.lock().unwrap();
                        let center_val = sim.last_hydrophone_data[0];
                        let corner_val = sim.last_hydrophone_data[1];

                        // Modulate
                        // Center value usually oscillates around 0.
                        params.frequency = 220.0 + center_val * 400.0;
                        params.amplitude = (0.1 + corner_val.abs() * 5.0).clamp(0.0, 1.0);
                    }

                    match sim.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => sim.resize(sim.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
