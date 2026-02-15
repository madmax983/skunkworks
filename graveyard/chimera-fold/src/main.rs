mod boid;
mod pbd;

use boid::OrigamiBoid;
use macroquad::prelude::*;

#[macroquad::main("Chimera Fold")]
async fn main() {
    let mut boids: Vec<OrigamiBoid> = Vec::new();
    let num_boids = 50;

    // Spawn
    for i in 0..num_boids {
        let pos = vec3(
            rand::gen_range(-20.0, 20.0),
            rand::gen_range(-20.0, 20.0),
            rand::gen_range(-20.0, 20.0),
        );
        boids.push(OrigamiBoid::new(i, pos));
    }

    loop {
        clear_background(BLACK);

        // Camera
        set_camera(&Camera3D {
            position: vec3(0.0, -80.0, 40.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        draw_grid(20, 5.0, Color::new(0.2, 0.2, 0.2, 1.0), GRAY);

        let positions: Vec<Vec3> = boids.iter().map(|b| b.center_of_mass).collect();
        let velocities: Vec<Vec3> = boids.iter().map(|b| b.velocity).collect();
        // Just use dummy phases for now, as phase coupling is handled internally or ignored
        let phases: Vec<f32> = vec![0.0; num_boids];

        let bounds_min = vec3(-50.0, -50.0, -50.0);
        let bounds_max = vec3(50.0, 50.0, 50.0);

        for i in 0..boids.len() {
            boids[i].update_flocking(&positions, &velocities, &phases, bounds_min, bounds_max);
        }

        for boid in &mut boids {
            boid.update_internal(0.016);
        }

        for boid in &boids {
            boid.draw();
        }

        set_default_camera();

        draw_text("CHIMERA FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            format!("Boids: {}", num_boids).as_str(),
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            for boid in &mut boids {
                boid.vm.mutate();
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            draw_text("MUTATING...", 10.0, 70.0, 20.0, RED);
        }

        next_frame().await
    }
}
