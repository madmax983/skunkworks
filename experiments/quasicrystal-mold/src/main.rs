mod math;
mod simulation;

use macroquad::prelude::*;
use simulation::Simulation;

#[macroquad::main("Quasicrystal Mold")]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(0.0, 0.0, 15.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let lattice = math::generate_icosahedral_lattice(2);
    let mut sim = Simulation::new(lattice, 2000);

    let mut orbit_angle = 0.0;

    loop {
        // Physics
        sim.step();

        // Input
        let dt = get_frame_time();
        if is_key_down(KeyCode::Left) {
            orbit_angle += dt;
        }
        if is_key_down(KeyCode::Right) {
            orbit_angle -= dt;
        }

        let radius = 15.0;
        camera.position.x = radius * orbit_angle.sin();
        camera.position.z = radius * orbit_angle.cos();
        camera.target = vec3(0.0, 0.0, 0.0);

        clear_background(BLACK);

        set_camera(&camera);

        // Draw Edges
        for (p1, p2) in &sim.world.lattice.edges {
            // Can we color edges by pheromone level?
            // Need to map p1/p2 back to indices or just draw grey.
            // For performance, just draw grey.
            draw_line_3d(*p1, *p2, GRAY);
        }

        // Draw Cities (Nodes)
        for &city_idx in &sim.world.cities {
            let pos = sim.world.lattice.atoms[city_idx];
            draw_sphere(pos, 0.2, None, YELLOW);
        }

        // Draw Agents
        for agent in &sim.agents {
            draw_sphere(agent.pos, 0.05, None, Color::new(agent.color[0], agent.color[1], agent.color[2], 1.0));
        }

        // Draw Pheromones (Nodes)
        // Only draw if level > threshold to avoid clutter
        for (i, &level) in sim.world.pheromone_levels.iter().enumerate() {
            if level > 0.1 {
                let pos = sim.world.lattice.atoms[i];
                let intensity = (level / 10.0).min(1.0);
                draw_sphere(pos, 0.08 * intensity, None, Color::new(0.0, 1.0, 1.0, intensity));
            }
        }

        set_default_camera();

        // UI
        draw_text(&format!("Agents: {}", sim.agents.len()), 10.0, 20.0, 30.0, WHITE);
        draw_text("Controls: Arrows to Orbit", 10.0, 50.0, 20.0, WHITE);

        next_frame().await;
    }
}
