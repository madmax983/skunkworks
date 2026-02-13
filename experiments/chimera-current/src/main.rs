mod agent;
mod lbm;
mod renderer;

use agent::Agent;
use lbm::{FluidSim, WIDTH, HEIGHT};
use renderer::render_ascii;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use ::rand::Rng;

#[macroquad::main("Chimera Current")]
async fn main() {
    let mut sim = FluidSim::new();
    let mut agents = Vec::new();

    // Spawn Initial Population
    for i in 0..50 {
        // Create random DNA: Simple Swimmer
        let mut genes = Vec::new();
        let mut rng = ::rand::thread_rng();

        // Genome:
        // 0: Push Action (Eat)
        // 1: Push AccelX (Random)
        // 2: Push AccelY (Random)
        // 3: Jump to 0
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }); // Action
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-100..100))] }); // AccelX
        genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-100..100))] }); // AccelY
        genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }); // Loop

        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };

        agents.push(Agent::new(
            i,
            dna,
            Vec2::new(rng.gen_range(0.0..WIDTH as f32), rng.gen_range(0.0..HEIGHT as f32))
        ));
    }

    loop {
        // Input: Add fluid with mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            let gx = (mx / sw * WIDTH as f32) as usize;
            let gy = (my / sh * HEIGHT as f32) as usize;

            sim.add_density(gx, gy, 5.0);

            // Add velocity towards mouse movement?
            // Simpler: Just add density.
        }

        // Step Fluid
        sim.step();

        // Step Agents
        for agent in &mut agents {
            agent.update(&mut sim);
        }

        // Reproduction / Death
        let mut new_agents = Vec::new();
        let current_count = agents.len();
        agents.retain_mut(|agent| {
            if agent.energy <= 0.0 {
                return false; // Die
            }
            if agent.energy > 150.0 {
                // Reproduce
                agent.energy /= 2.0;
                let mut child = Agent::new(current_count + new_agents.len() + 1000, agent.dna.clone(), agent.pos);
                child.energy = agent.energy;
                // Mutate child logic could go here (but ChimeraVM usually mutates via opcodes or external force)
                // Let's implement simple mutation: Flip a bit in args?
                // Or just clone for now.
                new_agents.push(child);
            }
            true
        });
        agents.append(&mut new_agents);

        // Ensure population doesn't explode
        if agents.len() > 200 {
            agents.truncate(200);
        }
        // Ensure population doesn't go extinct
        if agents.len() < 10 {
             // Respawn
             let mut rng = ::rand::thread_rng();
             let mut genes = Vec::new();
             genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }); // Action
             genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-100..100))] }); // AccelX
             genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(rng.gen_range(-100..100))] }); // AccelY
             genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }); // Loop
             let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
             agents.push(Agent::new(
                agents.len(),
                dna,
                Vec2::new(rng.gen_range(0.0..WIDTH as f32), rng.gen_range(0.0..HEIGHT as f32))
            ));
        }

        clear_background(BLACK);

        // Render Fluid
        let fluid_ascii = render_ascii(&sim);
        let screen_h = screen_height();
        let screen_w = screen_width();

        let char_h = screen_h / HEIGHT as f32;
        // Render ASCII line by line
        let mut y_pos = 0.0;
        for line in fluid_ascii.lines() {
             draw_text(line, 0.0, y_pos + char_h, char_h, BLUE);
             y_pos += char_h;
        }

        // Render Agents
        for agent in &agents {
            let sx = agent.pos.x / WIDTH as f32 * screen_w;
            let sy = agent.pos.y / HEIGHT as f32 * screen_h;

            // Color based on Energy
            let color = if agent.energy > 100.0 { GREEN }
                       else if agent.energy > 50.0 { YELLOW }
                       else { RED };

            // Draw
            draw_text("@", sx, sy, char_h * 1.5, color);
        }

        draw_text(&format!("Agents: {}", agents.len()), 10.0, 30.0, 30.0, WHITE);
        draw_text("Evolutionary Hydrodynamics", 10.0, 60.0, 20.0, GRAY);

        next_frame().await
    }
}
