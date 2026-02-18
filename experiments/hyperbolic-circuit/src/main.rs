mod bio;
mod circuit;

use bio::BioAgent;
use circuit::HyperCircuit;
use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};

const DISK_SCALE: f32 = 0.45;

#[macroquad::main("Hyperbolic Circuit")]
async fn main() {
    let mut circuit = HyperCircuit::generate("hyperbolic-circuit");

    // Spawn agents
    let mut agents = Vec::new();
    for _ in 0..20 {
        if circuit.pads.is_empty() { break; }
        // Start at random pad
        let start_pad = circuit.pads[macroquad::rand::gen_range(0, circuit.pads.len())];
        // DNA: Simple seeker
        use chimera_lang::prelude::*;
        let genes = vec![
            // Read Dist (0, 0) -> Push
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::GRead, args: vec![] },

            // Read Angle (0, 1) -> Turn (15, 0)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::GRead, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // Write Speed (15, 1) = 2
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] },
            Gene { op: OpCode::GWrite, args: vec![] },
        ];
        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        agents.push(BioAgent::new(start_pad, dna));
    }

    loop {
        // Update
        if is_key_pressed(KeyCode::R) {
             circuit = HyperCircuit::generate(&format!("seed-{}", get_time()));
             // Reset agents (respawn at new pads)
             for agent in &mut agents {
                 if !circuit.pads.is_empty() {
                    agent.pos = circuit.pads[macroquad::rand::gen_range(0, circuit.pads.len())];
                    agent.target_idx = None;
                 }
             }
        }

        for agent in &mut agents {
            agent.update(&circuit);
        }

        // Draw
        clear_background(BLACK);
        let w = screen_width();
        let h = screen_height();
        let min_dim = w.min(h);
        let screen_center = Vec2::new(w / 2.0, h / 2.0);
        let disk_radius = min_dim * DISK_SCALE;

        // Disk
        draw_circle(screen_center.x, screen_center.y, disk_radius, Color::new(0.05, 0.05, 0.05, 1.0));
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 2.0, DARKGRAY);

        // Traces
        for &(i, j) in &circuit.traces {
            if i < circuit.pads.len() && j < circuit.pads.len() {
                let p1 = circuit.pads[i];
                let p2 = circuit.pads[j];
                draw_geodesic(p1, p2, screen_center, disk_radius, Color::new(0.0, 0.5, 0.0, 0.5));
            }
        }

        // Pads
        for pad in &circuit.pads {
            let pos = to_screen(*pad, screen_center, disk_radius);
            draw_circle(pos.x, pos.y, 3.0, GOLD);
        }

        // Agents
        for agent in &agents {
            let pos = to_screen(agent.pos, screen_center, disk_radius);
            draw_circle(pos.x, pos.y, 4.0, RED);
        }

        // UI
        draw_text("Hyperbolic Circuit", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", agents.len()), 20.0, 60.0, 20.0, GRAY);
        draw_text("R: Regenerate", 20.0, 90.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius,
    )
}

fn draw_geodesic(p1: Point, p2: Point, center: Vec2, radius: f32, color: Color) {
    let steps = 20;
    // Map p1 to 0. Target is mobius_sub(p2, p1).
    // Interpolate radially from 0 to Target.
    // Map back using mobius_add(z, p1).

    let target = mobius_sub(p2, p1);
    let mut points = Vec::new();

    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let p_local = target * t; // Straight line in local tangent space at p1
        let p_world = mobius_add(p_local, p1);
        points.push(to_screen(p_world, center, radius));
    }

    for i in 0..points.len()-1 {
        draw_line(points[i].x, points[i].y, points[i+1].x, points[i+1].y, 2.0, color);
    }
}
