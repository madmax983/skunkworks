use arthropod::Button;
use macroquad::prelude::*;
use miller_lattice::Crystal;
use std::path::Path;

// When implementing the --headless bypass in macroquad experiments,
// avoid using the #[macroquad::main] attribute macro, as it initializes X11
// before the function code executes and causes panics in CI. Instead,
// manually parse arguments and initialize the window conditionally
// using a custom main wrapper and window_conf().

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Lattice".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

async fn run() {
    let mut camera = Camera3D {
        position: vec3(0.0, 50.0, 100.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let btn_grow = Button::new("Grow Crystal", 10.0, 10.0, 150.0, 30.0).with_colors(BLUE, SKYBLUE, DARKBLUE);

    // Default crystal from current dir
    let mut crystals: Vec<Crystal> = vec![];
    if let Ok(c) = Crystal::build_from_path(Path::new(".")) {
        crystals.push(c);
    }

    let mut rotation: f32 = 0.0;

    loop {
        clear_background(BLACK);

        rotation += 0.01;
        camera.position = vec3(rotation.sin() * 100.0, 50.0, rotation.cos() * 100.0);

        set_camera(&camera);

        // Draw crystals
        for (i, crystal) in crystals.iter().enumerate() {
            let offset = vec3((i as f32) * 50.0, 0.0, 0.0);

            for atom in &crystal.atoms {
                let pos = vec3(atom.position.x as f32, atom.position.y as f32, atom.position.z as f32) + offset;

                let color = if atom.is_dir { GREEN } else { WHITE };

                draw_sphere(pos, 1.0, None, color);
            }

            for bond in &crystal.bonds {
                let p1 = &crystal.atoms[bond.0];
                let p2 = &crystal.atoms[bond.1];

                let pos1 = vec3(p1.position.x as f32, p1.position.y as f32, p1.position.z as f32) + offset;
                let pos2 = vec3(p2.position.x as f32, p2.position.y as f32, p2.position.z as f32) + offset;

                draw_line_3d(pos1, pos2, GRAY);
            }
        }

        set_default_camera();

        // UI Layer
        if btn_grow.draw() {
            if let Ok(c) = Crystal::build_from_path(Path::new(".")) {
                crystals.push(c);
            }
        }

        next_frame().await;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Exiting immediately.");
        return;
    }

    macroquad::Window::from_config(window_conf(), run());
}
