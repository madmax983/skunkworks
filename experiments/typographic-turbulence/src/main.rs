mod sim;

use std::sync::Arc;
use sim::SimState;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new()
        .with_title("Typographic Turbulence")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 1024.0))
        .build(&event_loop)
        .unwrap());

    // Initialize WGPU async
    let mut state = pollster::block_on(SimState::new(window.clone()));

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
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
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { scale_factor: _, inner_size_writer: _ } => {
                    // new_inner_size is not directly available in 0.29 event?
                    // It says "inner_size_writer" is a MutexGuard.
                    // Usually we just handle Resized.
                    // But ScaleFactorChanged might require resize.
                    // For winit 0.29, ScaleFactorChanged event structure changed.
                    // Just rely on Resized which usually follows.
                }
                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    state.mouse_pos = [position.x as f32, position.y as f32];
                }
                WindowEvent::MouseInput { state: button_state, button: MouseButton::Left, .. } => {
                    state.mouse_pressed = *button_state == ElementState::Pressed;
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
