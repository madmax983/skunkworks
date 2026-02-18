use macroquad::prelude::*;
use macroquad::models::draw_mesh;
use gray_scott::GrayScott;
use ::rand::Rng;

const WIDTH: usize = 255;
const HEIGHT: usize = 255;

// Coral parameters
const DEFAULT_FEED: f32 = 0.0545;
const DEFAULT_KILL: f32 = 0.062;
const DT: f32 = 1.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Terra Phage ⚛️".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gs = GrayScott::new(WIDTH, HEIGHT);

    // Seed with random noise/rectangles
    let mut rng = ::rand::thread_rng();
    for _ in 0..50 {
        let cx = rng.gen_range(20..WIDTH-20);
        let cy = rng.gen_range(20..HEIGHT-20);
        for y in cy-5..cy+5 {
            for x in cx-5..cx+5 {
                gs.add_chemical(x, y, 0.5);
            }
        }
    }

    let mut feed = DEFAULT_FEED;
    let mut kill = DEFAULT_KILL;

    // Initialize Mesh
    let mut mesh = init_mesh(WIDTH, HEIGHT);

    // Camera State
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 300.0;
    let target = vec3(WIDTH as f32 / 2.0, 0.0, HEIGHT as f32 / 2.0);

    // Material for lighting (optional, but good for 3D)
    // We'll stick to default material which uses vertex colors if no texture

    loop {
        // --- Input ---
        // Camera Orbit
        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 5.0;
            cam_pitch += delta.y * 5.0;
            cam_pitch = cam_pitch.clamp(0.1, 1.5);
        }
        cam_dist -= mouse_wheel().1 * 10.0;
        cam_dist = cam_dist.clamp(50.0, 600.0);

        // Feed/Kill Controls
        if is_key_down(KeyCode::Up) { feed += 0.0001; }
        if is_key_down(KeyCode::Down) { feed -= 0.0001; }
        if is_key_down(KeyCode::Right) { kill += 0.0001; }
        if is_key_down(KeyCode::Left) { kill -= 0.0001; }

        // Interaction: Rain (Space)
        if is_key_down(KeyCode::Space) {
             let mut rng = ::rand::thread_rng();
             for _ in 0..10 {
                 let cx = rng.gen_range(5..WIDTH-5);
                 let cy = rng.gen_range(5..HEIGHT-5);
                 for y in cy-2..cy+2 {
                     for x in cx-2..cx+2 {
                         gs.add_chemical(x, y, 0.5);
                     }
                 }
             }
        }

        // Interaction: Bomb (Enter)
        if is_key_pressed(KeyCode::Enter) {
             let mut rng = ::rand::thread_rng();
             for _ in 0..50 {
                 let cx = rng.gen_range(10..WIDTH-10);
                 let cy = rng.gen_range(10..HEIGHT-10);
                 for y in cy-5..cy+5 {
                     for x in cx-5..cx+5 {
                         gs.add_chemical(x, y, 0.8);
                     }
                 }
             }
        }

        // Interaction: Reset (R)
        if is_key_pressed(KeyCode::R) {
            gs = GrayScott::new(WIDTH, HEIGHT);
            let mut rng = ::rand::thread_rng();
            for _ in 0..50 {
                let cx = rng.gen_range(20..WIDTH-20);
                let cy = rng.gen_range(20..HEIGHT-20);
                for y in cy-5..cy+5 {
                    for x in cx-5..cx+5 {
                        gs.add_chemical(x, y, 0.5);
                    }
                }
            }
        }

        // --- Update ---
        // Run multiple steps per frame for speed
        for _ in 0..8 {
            gs.update(feed, kill, DT);
        }

        update_mesh(&mut mesh, &gs);

        // --- Draw ---
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let cam_pos = vec3(
            target.x + cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            target.y + cam_dist * cam_pitch.sin(),
            target.z + cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        // Draw water plane (transparent blue quad)
        // Adjust height as needed (e.g., 20.0)
        let water_level = 20.0;
        draw_cube(vec3(WIDTH as f32/2.0, water_level - 5.0, HEIGHT as f32/2.0),
                  vec3(WIDTH as f32, 10.0, HEIGHT as f32),
                  None,
                  Color::new(0.0, 0.3, 0.8, 0.5));

        set_default_camera();

        // UI
        draw_text("Terra Phage ⚛️", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Feed: {:.4} | Kill: {:.4}", feed, kill), 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("ARROWS: Adjust Params | SPACE: Rain | ENTER: Bomb | R: Reset", 20.0, 70.0, 16.0, GRAY);
        draw_text("Right Click + Drag: Rotate Camera | Scroll: Zoom", 20.0, 90.0, 16.0, GRAY);

        next_frame().await
    }
}

fn init_mesh(width: usize, height: usize) -> Mesh {
    let mut vertices = Vec::with_capacity(width * height);
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);

    for z in 0..height {
        for x in 0..width {
            vertices.push(Vertex {
                position: vec3(x as f32, 0.0, z as f32),
                uv: vec2(x as f32 / width as f32, z as f32 / height as f32),
                color: [255, 255, 255, 255],
                normal: vec4(0.0, 1.0, 0.0, 0.0),
            });
        }
    }

    for z in 0..height - 1 {
        for x in 0..width - 1 {
            let i = (z * width + x) as u16;
            let next_row = (width) as u16;

            // Triangle 1
            indices.push(i);
            indices.push(i + next_row);
            indices.push(i + 1);

            // Triangle 2
            indices.push(i + 1);
            indices.push(i + next_row);
            indices.push(i + next_row + 1);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}

fn update_mesh(mesh: &mut Mesh, gs: &GrayScott) {
    let u = gs.u();
    let v = gs.v();

    // Safety check
    if mesh.vertices.len() != u.len() {
        return;
    }

    // Parallel update might be tricky with Mesh struct directly if not careful,
    // but we can iterate. Rayon for 65k items is good.
    // However, Mesh.vertices is a Vec, so we can use par_iter_mut.
    // We need to zip with u and v.

    // We'll just do sequential for now to avoid borrow checker complexity with GS and Mesh
    // unless performance is bad. 65k is small.

    for (i, vertex) in mesh.vertices.iter_mut().enumerate() {
        let val_u = u[i]; // 0.0 - 1.0 usually
        let val_v = v[i];

        // Map U to Height.
        // In Gray-Scott, U is high (1.0) in background, and drops in spots.
        // We want patterns to be mountains? Or valleys?
        // Let's make U=1.0 be base level (0), and U < 1.0 be mountains?
        // Or U=1.0 be mountains?
        // Let's try: Height = val_u * 60.0.
        // Usually U varies between 0.2 and 1.0 in patterns.

        let height = val_u * 100.0;
        vertex.position.y = height;

        // Color based on V (catalyst) and U (substrate)
        // High V usually means "Reaction Active".
        // Biomes:
        // High U, Low V -> "Peaceful" (Sand/Grass)
        // Low U, High V -> "Chaotic" (Rock/Magma)

        let color = if val_v > 0.3 {
            // High activity: Magma/Rock
             Color::new(0.8, 0.2 + val_v, 0.2, 1.0)
        } else if val_u > 0.6 {
            // High substrate: Grass/Forest
             Color::new(0.1, 0.5 + val_u * 0.4, 0.1, 1.0)
        } else if val_u > 0.3 {
            // Medium substrate: Sand
             Color::new(0.8, 0.8, 0.4, 1.0)
        } else {
             // Low everything: Water/Darkness
             Color::new(0.0, 0.1, 0.3, 1.0)
        };

        vertex.color = color.into();

        // Normals: Simplified (Up) or we could compute them.
        // For now, keep Up.
    }

    // Recalculate normals?
    // Doing it every frame is expensive (65k vertices * 4 neighbors).
    // Maybe later.
}
