//! # Miller FS ⚛️💎
//!
//! **Miller FS** is an experimental 3D file system visualizer that renders directory structures as crystallographic lattices. It explores the concept of "frozen mathematics" by mapping file hierarchy depth and names to Miller indices $(h, k, l)$, creating complex, intersecting planes of "atoms" (files).
//!
//! ## Concept
//!
//! - **Directories** act as nucleation sites, spawning new lattice planes.
//! - **Files** are atoms populated on these planes in a spiral or grid pattern.
//! - **Miller Indices**: The orientation of each directory's plane is deterministically derived from its name hash, mapping to standard crystallographic axes (e.g., `(1,0,0)`, `(1,1,1)`, `(1,1,0)`).
//! - **Bonds**: Connections between parent directories and their children form the "crystal lattice" structure.
//!
//! ## Controls
//!
//! | Key | Action |
//! | --- | --- |
//! | `W` / `ArrowUp` | Move Forward |
//! | `S` / `ArrowDown` | Move Backward |
//! | `A` / `ArrowLeft` | Strafe Left |
//! | `D` / `ArrowRight` | Strafe Right |
//! | `Space` | Move Up |
//! | `Shift` | Move Down |
//! | `Q` | Rotate View Left (Yaw) |
//! | `E` | Rotate View Right (Yaw) |
//! | `U` / `I` | Adjust Miller Index `h` (+/-) |
//! | `J` / `K` | Adjust Miller Index `k` (+/-) |
//! | `N` / `M` | Adjust Miller Index `l` (+/-) |
//! | `Esc` | Exit |
//!
//! ## Technical Details
//!
//! - **Stack**: Rust, `wgpu`, `cgmath`, `winit`.
//! - **Rendering**: Custom `wgpu` renderer using instanced meshes (cubes for atoms, stretched cubes for bonds).
//! - **Lattice Generation**: Recursive traversal of the file system, generating basis vectors for each directory's plane and placing files in a spiral pattern to minimize overlap.
//!
//! ## Usage
//!
//! Run from the root of the repository:
//!
//! ```bash
//! cargo run -p miller-fs -- <path_to_scan>
//! ```
//!
//! If no path is provided, it scans the current directory.
//!
//! ## Status
//!
//! **HIGH Concentration**. Active experiment exploring the intersection of crystallography and data visualization.
//!
mod camera;

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
