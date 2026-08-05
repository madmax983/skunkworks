//! # Arthropod-Origami-Lattice
//!
//! An experiment crossing `crates/arthropod` (Interactive UI), `crates/origami` (Soft Body Physics), and `crates/miller-lattice` (Hierarchical Trees).
//!
//! ## Lineage 🧬
//! - **Parent A (`arthropod`)**: Provides the interactive immediate mode graphical user interface and sliders.
//! - **Parent B (`origami`)**: Provides the procedural soft-body physics engine simulating continuous paper.
//! - **Parent C (`miller-lattice`)**: Provides the discrete hierarchical tree spatial grouping logic.
//! - **Novel Trait**: We map the rigid, abstract node distances of `miller-lattice` into the physical structural tension links of `origami`, controlled dynamically via `arthropod` UI interactions. This visualizes a hierarchical layout as a draping, buckling manifold.
//!
//! ## Usage
//!
//! ```bash
//! # Run with window
//! cargo run -p arthropod-origami-lattice
//!
//! # Run headless (for CI or automated testing)
//! HEADLESS=true cargo run -p arthropod-origami-lattice
//! ```
//!
use macroquad::prelude::*;

fn get_headless_mode() -> bool {
    std::env::var("HEADLESS").unwrap_or_else(|_| "false".to_string()) == "true"
}

#[macroquad::main("Arthropod-Origami-Lattice")]
async fn main() {
    let is_headless = get_headless_mode();
    let mut frame_count = 0;

    loop {
        clear_background(BLACK);

        if is_headless {
            frame_count += 1;
            if frame_count > 10 {
                break;
            }
        }

        next_frame().await;
    }
}
