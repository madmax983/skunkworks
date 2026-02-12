mod state;
mod audio;

use std::sync::Arc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{KeyCode, PhysicalKey},
};
use state::State;
use audio::AudioSystem;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Cymatic Ocean - Use Arrows to control Freq/Amp")
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap());

    // Audio
    let audio_sys = match AudioSystem::new() {
        Ok(sys) => Some(sys),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    // State
    // Using pollster to block on async init
    // Pass a clone of the Arc<Window>
    let mut state = pollster::block_on(State::new(window.clone()));

    let mut last_render_time = std::time::Instant::now();
    let mut total_time = 0.0;

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            if let PhysicalKey::Code(key) = event.physical_key {
                                match key {
                                    KeyCode::Escape => elwt.exit(),
                                    KeyCode::ArrowUp => {
                                        if let Some(audio) = &audio_sys {
                                            let mut p = audio.params.lock().unwrap();
                                            p.freq += 1.0;
                                            println!("Freq: {:.1}", p.freq);
                                        }
                                    }
                                    KeyCode::ArrowDown => {
                                        if let Some(audio) = &audio_sys {
                                            let mut p = audio.params.lock().unwrap();
                                            p.freq -= 1.0;
                                            if p.freq < 1.0 { p.freq = 1.0; }
                                            println!("Freq: {:.1}", p.freq);
                                        }
                                    }
                                    KeyCode::ArrowRight => {
                                        if let Some(audio) = &audio_sys {
                                            let mut p = audio.params.lock().unwrap();
                                            p.amp *= 1.1;
                                            println!("Amp: {:.2}", p.amp);
                                        }
                                    }
                                    KeyCode::ArrowLeft => {
                                        if let Some(audio) = &audio_sys {
                                            let mut p = audio.params.lock().unwrap();
                                            p.amp *= 0.9;
                                            println!("Amp: {:.2}", p.amp);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        let now = std::time::Instant::now();
                        let dt = (now - last_render_time).as_secs_f32();
                        last_render_time = now;
                        total_time += dt;

                        // Update Params
                        let mut current_params = audio::Params::default();
                        if let Some(audio) = &audio_sys {
                            let p = audio.params.lock().unwrap();
                            current_params = *p;
                        }
                        current_params.time = total_time;

                        state.update(current_params);

                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => eprintln!("{:?}", e),
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
    }).unwrap();
}
