//! # Miller Reaction
//!
//! **Parents**: `experiments/miller-fs` + `experiments/reaction-monitor`
//!
//! ## Concept
//! A "Living Crystal Filesystem". The file system structure is visualized as a 3D crystal lattice (using Miller indices for growth direction), where the surface texture of every crystal face displays a living Reaction-Diffusion simulation (Gray-Scott model).
//!
//! The simulation parameters (Feed/Kill rates) are driven by the system's real-time load (CPU/RAM usage), creating a feedback loop where the computer's effort to render the visualization changes the visualization itself.
//!
//! ## Phenotype
//! - **Structure**: 3D Crystal Lattice (from `miller-fs`).
//! - **Texture**: Gray-Scott Reaction Diffusion (from `reaction-monitor`).
//! - **Dynamics**:
//!   - Crystals grow based on directory depth/content.
//!   - Surface patterns evolve organically.
//!   - High CPU activity alters the "Feed" rate, changing pattern stability.
//!   - High RAM usage alters the "Kill" rate.
//!
//! ## Controls
//! - **W/A/S/D**: Move camera.
//! - **Space/Shift**: Up/Down.
//! - **U/I/J/K/N/M**: Adjust Miller Indices (Plane visualization).
//!
//! ## Lineage
//! - `miller-fs`: Provided the WGPU instance rendering and crystallography logic.
//! - `reaction-monitor`: Provided the Compute Shader implementation of the Gray-Scott model and system monitoring logic.
//! - **Hybridization**: The compute shader output is used as a texture for the crystal instances in the render pass.
//!
mod camera;

mod reaction;
mod state;

use anyhow::Result;
use log::{error, info};
use miller_lattice::Crystal;
use state::State;
use std::path::Path;
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
    let path = if args.len() > 1 { &args[1] } else { "." };

    info!("Scanning crystal structure from: {}", path);
    let crystal = Crystal::build_from_path(Path::new(path))?;
    info!("Crystal built with {} atoms.", crystal.atoms.len());

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Genesis: Miller FS")
            .build(&event_loop)?,
    );

    // State needs to be created async
    let mut state = pollster::block_on(State::new(window.clone(), crystal));

    event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
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
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(e) => error!("{:?}", e),
                        }
                    }
                    _ => {}
                }
            }
        }
        Event::AboutToWait => {
            state.window.request_redraw();
        }
        _ => {}
    })?;

    Ok(())
}
