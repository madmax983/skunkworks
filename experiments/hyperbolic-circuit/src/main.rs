mod bio;
mod circuit;

use bio::BioAgent;
use chimera_lang::prelude::*;
use circuit::HyperbolicCircuit;
use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};

const DISK_SCALE: f32 = 0.45;

fn generate_router_dna() -> Dna {
    // A simple genome that takes sensor input and produces a routing decision
    let genes = vec![
        // Read neighbor count at (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Read random noise at (0,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Combine: (Neighbors + Random) * 31 % Neighbors (handled by agent logic)
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(31)],
        },
        Gene {
            op: OpCode::Mul,
            args: vec![],
        },
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[macroquad::main("Hyperbolic Circuit")]
async fn main() {
    let mut circuit = HyperbolicCircuit::new();
    circuit.generate(50, 2.0); // Generate 50 pads, connect if dist < 2.0

    let mut agents = Vec::new();
    for _ in 0..20 {
        let start_pad = rand::gen_range(0, circuit.pads.len());
        agents.push(BioAgent::new(start_pad, generate_router_dna()));
    }

    loop {
        // Handle Input
        if is_key_pressed(KeyCode::R) {
            circuit.generate(50, 2.0);
            agents.clear();
            for _ in 0..20 {
                let start_pad = rand::gen_range(0, circuit.pads.len());
                agents.push(BioAgent::new(start_pad, generate_router_dna()));
            }
        }

        // Update Agents
        for agent in &mut agents {
            agent.tick(&circuit);
        }

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let w = screen_width();
        let h = screen_height();
        let min_dim = w.min(h);
        let screen_center = Vec2::new(w / 2.0, h / 2.0);
        let disk_radius = min_dim * DISK_SCALE;

        // Draw Boundary
        draw_circle(
            screen_center.x,
            screen_center.y,
            disk_radius,
            Color::new(0.0, 0.0, 0.0, 1.0),
        );
        draw_circle_lines(
            screen_center.x,
            screen_center.y,
            disk_radius,
            2.0,
            DARKGRAY,
        );

        // Draw Traces
        for trace in &circuit.traces {
            let p1 = circuit.pads[trace.start].pos;
            let p2 = circuit.pads[trace.end].pos;
            draw_geodesic(p1, p2, screen_center, disk_radius, Color::new(0.2, 0.8, 0.8, 0.3));
        }

        // Draw Pads
        for pad in &circuit.pads {
            let pos = to_screen(pad.pos, screen_center, disk_radius);
            draw_circle(pos.x, pos.y, 3.0, Color::new(0.4, 0.4, 0.4, 0.8));
        }

        // Draw Agents
        for agent in &agents {
            let start_pos = circuit.pads[agent.current_pad].pos;
            let pos = if let Some(target_idx) = agent.target_pad {
                // Interpolate along geodesic
                let end_pos = circuit.pads[target_idx].pos;
                let relative_end = mobius_sub(end_pos, start_pos);
                let relative_current = relative_end * agent.progress;
                mobius_add(relative_current, start_pos)
            } else {
                start_pos
            };

            let screen_pos = to_screen(pos, screen_center, disk_radius);
            draw_circle(screen_pos.x, screen_pos.y, 4.0, Color::new(1.0, 0.2, 0.2, 1.0));
        }

        // UI
        draw_text("Hyperbolic Circuit", 20.0, 30.0, 30.0, WHITE);
        draw_text("Press 'R' to Regenerate", 20.0, 50.0, 20.0, GRAY);

        next_frame().await;
    }
}

fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius, // Flip Y for screen
    )
}

fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    let steps = 15;
    let m_p2 = mobius_sub(p2, p1);

    // Collect points first
    let mut points = Vec::with_capacity(steps + 1);
    points.push(to_screen(p1, screen_center, radius));

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t;
        let world_pos = mobius_add(q, p1); // Map back
        points.push(to_screen(world_pos, screen_center, radius));
    }

    // Draw lines
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, color);
    }
}
