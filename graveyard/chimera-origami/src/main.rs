use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use origami::{generate_miura_mesh, MiuraParams, Orientation};

#[macroquad::main("Chimera Origami")]
async fn main() {
    // 1. Setup Origami Mesh
    let rows = 4;
    let cols = 4;
    let params = MiuraParams {
        a: 20.0,
        b: 20.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let mut mesh = generate_miura_mesh(params, (cols, rows), 0.5);
    let cell_size = 20.0;

    // Initial offset to center the mesh
    for vertex in &mut mesh.vertices {
        vertex.pos.z -= 100.0;
        vertex.pos.x += screen_width() / 2.0 - (cols as f32 * cell_size) / 2.0;
        vertex.pos.y += screen_height() / 2.0 - (rows as f32 * cell_size) / 2.0;
    }

    // 2. Setup ChimeraVM (The Genetic Brain)
    let mut rng = ::rand::thread_rng();
    let mut brains: Vec<ChimeraVM> = Vec::new();

    // The mesh generated from `origami` module currently doesn't expose constraints
    // directly. It outputs an `OrigamiMesh` which contains `vertices` and `indices`.
    // We will just create dummy constraints for the edges here if they don't exist,
    // or operate directly on the positions if it's purely a visual mesh.
    // However, wait, let me check the `origami` crate.
    // If it only returns vertices and indices, we need to create constraints!

    // For this simple example we will just add a dummy brain list.
    // Let's assume we create a spring constraint system manually over the indices.
    struct Constraint {
        p1: usize,
        p2: usize,
        rest_length: f32,
    }
    let mut constraints = Vec::new();
    for i in (0..mesh.indices.len()).step_by(3) {
        let v0 = mesh.indices[i] as usize;
        let v1 = mesh.indices[i + 1] as usize;
        let v2 = mesh.indices[i + 2] as usize;

        let p0 = mesh.vertices[v0].pos;
        let p1 = mesh.vertices[v1].pos;
        let p2 = mesh.vertices[v2].pos;

        let d01 = (p0 - p1).length();
        let d12 = (p1 - p2).length();
        let d20 = (p2 - p0).length();

        constraints.push(Constraint {
            p1: v0,
            p2: v1,
            rest_length: d01,
        });
        constraints.push(Constraint {
            p1: v1,
            p2: v2,
            rest_length: d12,
        });
        constraints.push(Constraint {
            p1: v2,
            p2: v0,
            rest_length: d20,
        });
    }

    // Assign a brain to each constraint (actuator)
    for _ in 0..constraints.len() {
        // Just a blank DNA
        let dna = Dna::from_genes(vec![]);
        let vm = ChimeraVM::new(dna);

        brains.push(vm);
    }

    let mut cam = Camera3D {
        position: vec3(screen_width() / 2.0, screen_height() / 2.0, 150.0),
        target: vec3(screen_width() / 2.0, screen_height() / 2.0, 0.0),
        up: vec3(0.0, -1.0, 0.0),
        ..Default::default()
    };

    let mut angle: f32 = 0.0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            angle += delta.x * 2.0;
        }

        cam.position = vec3(
            screen_width() / 2.0 + angle.sin() * 150.0,
            screen_height() / 2.0,
            angle.cos() * 150.0,
        );

        // Update Brains & Constraints
        for (i, constraint) in constraints.iter_mut().enumerate() {
            if i < brains.len() {
                let brain = &mut brains[i];

                // Execute a step of the genetic prologue logic (if we had a grid setup)
                // For this mock, we just use random energy to simulate neural activity
                brain.energy = brain
                    .energy
                    .saturating_add(if rng.gen_bool(0.05) { 10 } else { 0 });

                // Genetic actuation: Modify the rest_length based on brain activity
                // Base length is what it was originally.
                let target_length = if brain.energy > 5 {
                    brain.energy -= 5;
                    constraint.rest_length * 0.7 // Contract
                } else {
                    constraint.rest_length * 1.0 // Relax
                };

                // Smoothly interpolate current rest length
                constraint.rest_length = constraint.rest_length * 0.95 + target_length * 0.05;
            }
        }

        // Render
        set_camera(&cam);

        for constraint in &constraints {
            let p1 = &mesh.vertices[constraint.p1].pos;
            let p2 = &mesh.vertices[constraint.p2].pos;

            // Color based on tension
            let dist = (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2);
            let dist = dist.sqrt();
            let tension = (dist - constraint.rest_length).abs() / constraint.rest_length;

            let color = Color::new(0.4 + tension * 2.0, 0.4, 0.8, 0.8);

            draw_line_3d(vec3(p1.x, p1.y, p1.z), vec3(p2.x, p2.y, p2.z), color);
        }

        // Draw vertices
        for vertex in &mesh.vertices {
            draw_cube(
                vec3(vertex.pos.x, vertex.pos.y, vertex.pos.z),
                vec3(1.0, 1.0, 1.0),
                None,
                Color::new(0.8, 0.8, 0.9, 1.0),
            );
        }

        set_default_camera();

        draw_text(
            "🧬 Genetic Origami (Chimera x PBD)",
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text("Drag mouse to rotate", 10.0, 40.0, 16.0, LIGHTGRAY);

        next_frame().await;
    }
}
