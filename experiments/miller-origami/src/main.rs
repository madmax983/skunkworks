use macroquad::prelude::*;
use miller_lattice::Crystal;
use origami::{generate_miura_grid, MiuraParams, Orientation};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string())
        || (std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux"))
    {
        println!("Headless execution completed successfully.");
        return;
    }
    macroquad::Window::from_config(window_conf(), amain());
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Miller-Origami: Codebase Morphogenesis".to_owned(),
        window_width: 1000,
        window_height: 800,
        ..Default::default()
    }
}

async fn amain() {
    // 1. Initialize Miller Lattice (Read Directory Structure)
    let path = std::path::Path::new(".");
    let crystal = Crystal::build_from_path(path).unwrap_or_else(|_| Crystal {
        atoms: Vec::new(),
        bonds: Vec::new(),
        lookup: rustc_hash::FxHashMap::default(),
    });

    let atoms_len = crystal.atoms.len();
    if atoms_len == 0 {
        println!("Empty crystal generated.");
        return;
    }

    // 2. Initialize Origami Mesh based on Crystal size
    let side_len = (atoms_len as f32).sqrt().ceil() as usize;
    let cols = side_len;
    let rows = side_len;

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let mut camera = Camera3D {
        position: vec3(0.0, 15.0, 15.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Animate extension factor using time to create a "breathing" effect
        let extension_factor = ((get_time() as f32) * 0.5).sin() * 0.4 + 0.5;

        // Generate the continuous origami grid positions
        let points = generate_miura_grid(params, (cols, rows), extension_factor);

        if points.is_empty() {
            next_frame().await;
            continue;
        }

        // Orbit camera around the mesh
        let rotation = get_time() as f32 * 0.2;
        let radius = (side_len as f32) * 1.5;
        camera.position = vec3(rotation.sin() * radius, radius, rotation.cos() * radius);
        set_camera(&camera);

        // Draw structural bonds as tissue connecting atoms
        for &(parent_idx, child_idx) in &crystal.bonds {
            let p_idx = parent_idx.min(points.len().saturating_sub(1));
            let c_idx = child_idx.min(points.len().saturating_sub(1));

            let p0 = points[p_idx];
            let p1 = points[c_idx];

            draw_line_3d(p0, p1, Color::new(0.2, 0.8, 0.5, 1.0));
        }

        // Draw nodes (files/directories)
        for i in 0..atoms_len {
            let idx = i.min(points.len().saturating_sub(1));
            let p = points[idx];
            let atom = &crystal.atoms[i];

            let color = if atom.is_dir {
                Color::new(1.0, 0.2, 0.2, 1.0) // Directories are Red
            } else {
                Color::new(0.2, 0.5, 1.0, 1.0) // Files are Blue
            };

            // Draw a vertical spike representing the node
            draw_line_3d(p, p + vec3(0.0, 0.3, 0.0), color);
        }

        set_default_camera();

        // UI text overlays
        draw_text(
            "Miller-Origami: Codebase Soft-Body Morphogenesis",
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Atoms (Files/Dirs): {}", atoms_len),
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}