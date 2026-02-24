use macroquad::prelude::*;
use chimera_lang::prelude::*;
use physics_pbd::PbdSystem;

mod logistic;
mod agent;

use logistic::LogisticMap;
use agent::Crawler;

fn create_simple_dna() -> Dna {
    // Simple oscillator like in chimera-tissue
    let genes = vec![
        Gene { op: OpCode::Dup, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
        Gene { op: OpCode::Gt, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Swap, args: vec![] },
        Gene { op: OpCode::Sub, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        Gene { op: OpCode::Mul, args: vec![] },
    ];
    Dna { helix: Helix { strands: vec![Strand { genes }] }, evolution_config: None }
}

#[macroquad::main("Bifurcation Crawler")]
async fn main() {
    let mut system = PbdSystem::new();
    let logistic_map = LogisticMap::new(512, 512); // Texture size

    // Start at r=2.9 (Stable)
    // World Coords: X=0 corresponds to r=2.8.
    // r=2.9 -> delta=0.1.
    // r = 2.8 + (x/12)*1.2 -> 0.1 = (x/12)*1.2 -> 0.1/0.1 = x/12 -> 1 = x/12 -> x=1.0?
    // 0.1 = x/10. No. 1.2/12 = 0.1. So x=1.0 -> r=2.9.
    // y=5.0 -> x=0.5.
    let mut crawler = Crawler::new(vec2(1.0, 5.0), &mut system, create_simple_dna());

    // Camera Setup
    let world_width = 12.0;
    let world_height = 10.0;

    loop {
        // Input: Move Head?
        if is_key_down(KeyCode::Right) {
             system.particles[crawler.particle_indices[0]].vel.x += 2.0;
        }
        if is_key_down(KeyCode::Left) {
             system.particles[crawler.particle_indices[0]].vel.x -= 2.0;
        }
        if is_key_down(KeyCode::Up) {
             system.particles[crawler.particle_indices[0]].vel.y += 2.0;
        }
        if is_key_down(KeyCode::Down) {
             system.particles[crawler.particle_indices[0]].vel.y -= 2.0;
        }

        // Physics Step
        system.step(0.016, 5);

        // Bounds Check & Friction
        for p in &mut system.particles {
            p.vel *= 0.90; // Drag

            // Keep in bounds
            if p.pos.x < 0.0 { p.pos.x = 0.0; p.vel.x = 0.0; }
            if p.pos.x > world_width { p.pos.x = world_width; p.vel.x = 0.0; }
            if p.pos.y < 0.0 { p.pos.y = 0.0; p.vel.y = 0.0; }
            if p.pos.y > world_height { p.pos.y = world_height; p.vel.y = 0.0; }
        }

        // Agent Update
        // Map agent head position to (r, x)
        let head_pos = system.particles[crawler.particle_indices[0]].pos;
        let r = 2.8 + (head_pos.x / world_width) * 1.2;
        let x = head_pos.y / world_height;

        // Chaos Damage
        let dist = logistic_map.distance_to_attractor(r, x);
        if dist > 0.05 {
            crawler.energy -= 0.1;
        } else {
            crawler.energy += 0.2;
        }
        crawler.energy = crawler.energy.clamp(0.0, 100.0);

        crawler.update(&mut system, dist); // Pass dist as chaos factor

        // Draw
        clear_background(BLACK);

        // Draw Logistic Map Background
        draw_texture_ex(
            &logistic_map.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Particles (mapped to screen)
        for i in 0..system.particles.len() {
             let p = system.particles[i];
             let sx = (p.pos.x / world_width) * screen_width();
             let sy = (1.0 - (p.pos.y / world_height)) * screen_height();
             draw_circle(sx, sy, 5.0, WHITE);
        }

        // Draw Constraints
        for c in &system.constraints {
            if let physics_pbd::Constraint::Actuator { p1, p2, .. } = c {
                 let pos1 = system.particles[*p1].pos;
                 let pos2 = system.particles[*p2].pos;
                 let sx1 = (pos1.x / world_width) * screen_width();
                 let sy1 = (1.0 - (pos1.y / world_height)) * screen_height();
                 let sx2 = (pos2.x / world_width) * screen_width();
                 let sy2 = (1.0 - (pos2.y / world_height)) * screen_height();
                 draw_line(sx1, sy1, sx2, sy2, 3.0, crawler.color);
            }
        }

        // UI
        draw_text(&format!("Energy: {:.1}", crawler.energy), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("R: {:.4}", r), 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
