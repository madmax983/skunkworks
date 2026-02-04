use macroquad::prelude::*;
use klein_life::life::LifeGrid;

const WIDTH: usize = 60;
const HEIGHT: usize = 60;

fn parametric_klein(u: f32, v: f32) -> Vec3 {
    let r = 2.5;
    let cos_u = u.cos();
    let sin_u = u.sin();
    let cos_u2 = (u / 2.0).cos();
    let sin_u2 = (u / 2.0).sin();
    let sin_v = v.sin();
    let sin_2v = (2.0 * v).sin();

    // Figure-8 Immersion
    let term = r + cos_u2 * sin_v - sin_u2 * sin_2v;

    let x = term * cos_u;
    let y = term * sin_u;
    let z = sin_u2 * sin_v + cos_u2 * sin_2v;

    // Rotate to make it look nicer (Vertical Figure-8)
    vec3(x, z, y)
}

#[macroquad::main("Klein Life")]
async fn main() {
    let mut grid = LifeGrid::new(WIDTH, HEIGHT);

    let mut last_update = get_time();
    let mut cam_angle = 0.0f32;
    let mut paused = false;

    loop {
        if is_key_pressed(KeyCode::R) {
             grid = LifeGrid::new(WIDTH, HEIGHT);
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if !paused && get_time() - last_update > 0.05 {
             grid.update();
             last_update = get_time();
        }

        // Input for camera
        if is_key_down(KeyCode::Left) {
            cam_angle -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle += 0.02;
        }
        // Auto rotate if no input
        if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
             cam_angle += 0.005;
        }

        let cam_pos = vec3(cam_angle.cos() * 10.0, 4.0, cam_angle.sin() * 10.0);

        clear_background(BLACK);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // Draw active cells
        for y in 0..HEIGHT {
            let u = (y as f32 / HEIGHT as f32) * std::f32::consts::PI * 2.0;
            for x in 0..WIDTH {
                if grid.cells[y * WIDTH + x] {
                    let v = (x as f32 / WIDTH as f32) * std::f32::consts::PI * 2.0;
                    let pos = parametric_klein(u, v);

                    // Color gradient based on U (Along the tube length)
                    let color = Color::new(
                        0.2 + 0.8 * (u / (2.0 * std::f32::consts::PI)),
                        1.0 - 0.5 * (u / (2.0 * std::f32::consts::PI)),
                        0.5,
                        1.0
                    );
                    draw_cube(pos, vec3(0.08, 0.08, 0.08), None, color);
                }
            }
        }

        set_default_camera();
        draw_text("Klein Life", 10.0, 30.0, 30.0, WHITE);
        draw_text("Space: Pause | R: Reset | Arrows: Rotate", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 70.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
