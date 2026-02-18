mod bio;
mod circuit;

use bio::BioAgent;
use circuit::CircuitGenerator;
use chimera_lang::prelude::*;
use macroquad::prelude::*;

fn generate_seeker_dna() -> Dna {
    // A simple genome that reads the angle to target and steers towards it
    // Logic:
    // 1. Read Angle Diff from (0, 2)
    // 2. Write Angle Diff to Turn (15, 0)
    // 3. Write Speed 2 to (15, 1)
    let genes = vec![
        // Read Angle (0, 2)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // X
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
        Gene { op: OpCode::GRead, args: vec![] },

        // Write Turn (15, 0)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] }, // X
        Gene { op: OpCode::GWrite, args: vec![] },

        // Write Speed (15, 1)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // Value
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] }, // X
        Gene { op: OpCode::GWrite, args: vec![] },
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[macroquad::main("Chimera Circuit")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    // 1. Generate Circuit
    let generator = CircuitGenerator::new(512, 512);
    let seed = "chimera-circuit"; // TODO: Use git hash
    let (mut img, mut pads) = generator.generate(seed);

    // 2. Create Texture
    let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16, img.as_raw());

    // 3. Spawn Agents
    let mut agents = Vec::new();
    for _ in 0..50 {
        // Pick random pad
        let start_pad_idx = ::rand::Rng::gen_range(&mut rng, 0..pads.len());
        let (px, py) = pads[start_pad_idx];
        let pos = vec2(px as f32, py as f32);

        agents.push(BioAgent::new(pos, generate_seeker_dna()));
    }

    let camera = Camera2D {
        zoom: vec2(1.0 / 512.0 * 2.0, -1.0 / 512.0 * 2.0),
        target: vec2(256.0, 256.0),
        ..Default::default()
    };

    loop {
        // Handle Input
        if is_key_pressed(KeyCode::R) {
             let (new_img, new_pads) = generator.generate(&format!("seed-{}", get_time()));
             // Update texture
             texture.update(&Image {
                 width: new_img.width() as u16,
                 height: new_img.height() as u16,
                 bytes: new_img.as_raw().clone(),
             });

             // Update Physics Data
             img = new_img;
             pads = new_pads;

             // Reset agents
             agents.clear();
             for _ in 0..50 {
                let start_pad_idx = ::rand::Rng::gen_range(&mut rng, 0..pads.len());
                let (px, py) = pads[start_pad_idx];
                let pos = vec2(px as f32, py as f32);
                agents.push(BioAgent::new(pos, generate_seeker_dna()));
             }
        }

        // Update
        for agent in &mut agents {
            agent.update(&img, &pads);
        }

        // Remove dead agents
        agents.retain(|a| !a.vm.halted);

        // Respawn if low
        if agents.len() < 10 {
             let start_pad_idx = ::rand::Rng::gen_range(&mut rng, 0..pads.len());
             let (px, py) = pads[start_pad_idx];
             let pos = vec2(px as f32, py as f32);
             agents.push(BioAgent::new(pos, generate_seeker_dna()));
        }

        // Draw
        clear_background(BLACK);

        set_camera(&camera);

        draw_texture(&texture, 0.0, 0.0, WHITE);

        for agent in &agents {
            draw_circle(agent.pos.x, agent.pos.y, 3.0, RED);
            let end = agent.pos + agent.dir * 5.0;
            draw_line(agent.pos.x, agent.pos.y, end.x, end.y, 1.0, YELLOW);
        }

        set_default_camera();

        draw_text("Chimera Circuit", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 50.0, 20.0, WHITE);
        draw_text("Press R to Regenerate", 10.0, 80.0, 20.0, GRAY);

        next_frame().await;
    }
}
