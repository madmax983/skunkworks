use macroquad::prelude::*;
use flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocusVec2;
use origami::{generate_miura_mesh, MiuraParams, Orientation};

const NUM_BOIDS: usize = 150;
const FLOCKING_PARAMS: FlockingParams = FlockingParams {
    view_radius: 50.0,
    separation_radius: 15.0,
    max_speed: 100.0,
    max_force: 50.0,
    separation_weight: 1.5,
    alignment_weight: 1.0,
    cohesion_weight: 1.0,
};

#[macroquad::main("Origami Flock")]
async fn main() {
    let rows = 20;
    let cols = 20;

    let miura_params = MiuraParams {
        a: 10.0,
        b: 10.0,
        gamma: 60.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    let extension = 0.5;
    let mut mesh_data = generate_miura_mesh(miura_params, (cols, rows), extension);

    let mut mq_positions = vec![Vec2::ZERO; NUM_BOIDS];
    let mut mq_velocities = vec![Vec2::ZERO; NUM_BOIDS];

    let mut flock_positions = vec![LocusVec2::zero(); NUM_BOIDS];
    let mut flock_velocities = vec![LocusVec2::zero(); NUM_BOIDS];


    // Initialize boids
    for i in 0..NUM_BOIDS {
        mq_positions[i] = vec2(
            rand::gen_range(-200.0, 200.0),
            rand::gen_range(-200.0, 200.0),
        );
        mq_velocities[i] = vec2(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ).normalize_or_zero() * FLOCKING_PARAMS.max_speed as f32;

        flock_positions[i] = LocusVec2::new(mq_positions[i].x as f64, mq_positions[i].y as f64);
        flock_velocities[i] = LocusVec2::new(mq_velocities[i].x as f64, mq_velocities[i].y as f64);
    }

    let mut camera = Camera3D {
        position: vec3(0., 300., 400.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    loop {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Right) {
            camera.position.x += 100.0 * dt;
        }
        if is_key_down(KeyCode::Left) {
            camera.position.x -= 100.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            camera.position.y += 100.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            camera.position.y -= 100.0 * dt;
        }

        // Update flocking
        let mut new_mq_velocities = mq_velocities.clone();
        for i in 0..NUM_BOIDS {
            let acc = compute_force(&flock_positions, &flock_velocities, i, &FLOCKING_PARAMS);
            let mq_acc = vec2(acc.x as f32, acc.y as f32);

            new_mq_velocities[i] += mq_acc * dt;

            let speed = new_mq_velocities[i].length();
            if speed > FLOCKING_PARAMS.max_speed as f32 {
                new_mq_velocities[i] = (new_mq_velocities[i] / speed) * FLOCKING_PARAMS.max_speed as f32;
            }

            mq_positions[i] += new_mq_velocities[i] * dt;

            // Simple wrap-around
            if mq_positions[i].x > 200.0 { mq_positions[i].x = -200.0; }
            if mq_positions[i].x < -200.0 { mq_positions[i].x = 200.0; }
            if mq_positions[i].y > 200.0 { mq_positions[i].y = -200.0; }
            if mq_positions[i].y < -200.0 { mq_positions[i].y = 200.0; }

            flock_positions[i] = LocusVec2::new(mq_positions[i].x as f64, mq_positions[i].y as f64);
            flock_velocities[i] = LocusVec2::new(new_mq_velocities[i].x as f64, new_mq_velocities[i].y as f64);
        }
        mq_velocities = new_mq_velocities;


        // Recalculate base mesh geometry to get base heights
        let base_mesh = generate_miura_mesh(miura_params, (cols, rows), extension);

        // Deform mesh based on boid proximity
        for i in 0..mesh_data.vertices.len() {
            let mut v = base_mesh.vertices[i].pos;
            let mesh_pos = vec2(v.x, v.y);

            let mut influence = 0.0;
            for j in 0..NUM_BOIDS {
                let boid_pos = mq_positions[j];
                let dist_sq = mesh_pos.distance_squared(boid_pos);
                let effect_radius = 50.0;
                if dist_sq < effect_radius * effect_radius {
                    influence += (1.0 - (dist_sq.sqrt() / effect_radius)) * 20.0; // Raise vertices near boids
                }
            }

            v.z -= influence; // Note: z is often negative for "up" depending on camera, adjusting it down for visually "up"
            mesh_data.vertices[i].pos = v;
        }

        // Draw
        clear_background(DARKGRAY);

        set_camera(&camera);

        // Draw Mesh
        let mut mq_vertices = Vec::new();
        for v in &mesh_data.vertices {
            mq_vertices.push(Vertex {
                position: v.pos,
                uv: v.uv,
                color: Color::new(0.5, 0.7, 0.9, 1.0).into(),
                normal: vec4(0., 0., -1., 0.), // macroquad models vertex needs vec4 for normals in some versions or vec3 depending, checking it
            });
        }

        let mut mq_indices = Vec::new();
        for idx in &mesh_data.indices {
            mq_indices.push(*idx);
        }

        draw_mesh(&Mesh {
            vertices: mq_vertices,
            indices: mq_indices,
            texture: None,
        });

        // Draw Wireframe (optional for better visual clarity)
        for i in (0..mesh_data.indices.len()).step_by(3) {
            let v1 = mesh_data.vertices[mesh_data.indices[i] as usize].pos;
            let v2 = mesh_data.vertices[mesh_data.indices[i+1] as usize].pos;
            let v3 = mesh_data.vertices[mesh_data.indices[i+2] as usize].pos;

            draw_line_3d(v1, v2, GREEN);
            draw_line_3d(v2, v3, GREEN);
            draw_line_3d(v3, v1, GREEN);
        }

        // Draw Boids
        for pos in &mq_positions {
            // Draw a small red sphere or cube for each boid
            draw_sphere(vec3(pos.x, pos.y, 10.0), 2.0, None, RED);
        }

        set_default_camera();

        draw_text("Origami Flock Morphogenesis", 10.0, 20.0, 30.0, WHITE);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 50.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
