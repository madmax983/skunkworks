use macroquad::prelude::*;
use chimera_lang::prelude::*;
use ::rand::Rng;

mod pbd;
mod mesh;

use mesh::ChimeraMesh;

#[macroquad::main("Chimera-Fold")]
async fn main() {
    // 1. Create DNA: A simple Oscillator
    // Logic:
    //   t = GRead(0, 3)  (Time)
    //   id = GRead(0, 2) (Column ID)
    //   phase = t + id * 5
    //   val = (phase % 20)
    //   if val < 10 { target = 0 } else { target = 100 }
    //   GWrite(0, 1, target)
    //   Jump(0)

    let genes = vec![
        // Get Time
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] }, // x
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y
        Gene { op: OpCode::GRead, args: vec![] },

        // Get ID
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // x
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y
        Gene { op: OpCode::GRead, args: vec![] },

        // Offset phase by ID * 5
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Mul, args: vec![] },
        Gene { op: OpCode::Add, args: vec![] },

        // Modulo 20
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("%".to_string())] }, // Using ribosome op
        Gene { op: OpCode::Consume, args: vec![] }, // Consume string to put on stack? No, execute it.
        // Wait, "%" is a Ribosome op, but default OpCode::Unknown parses string.
        // Or I can use Push("%") then Call? Or just use "Modulo" logic manually?
        // Let's use manual modulo: a - (a/b)*b
        // Stack: [phase, 20]
        Gene { op: OpCode::Dup, args: vec![] }, // [phase, 20, 20]
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // Rotate stack? No.
        // Let's keep it simple: Just use Division to make a square wave.
        // val = (phase / 10) % 2
        // If even -> 0, if odd -> 100
        Gene { op: OpCode::Drop, args: vec![] }, // Drop 20

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Div, args: vec![] }, // phase / 10

        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        // Manual Modulo 2: x % 2 = x & 1
        // But we don't have bitwise AND easily.
        // Let's just multiply by 50 to get 0 or 50 or 100 etc?
        // No, let's just write (phase / 10) * 10
        // That creates a ramp.

        // Let's try to use the string "%" op via Ribosome if available, but Ribosome is an Organelle.
        // The VM main loop doesn't execute string ops directly unless it's an Unknown OpCode that matches.

        // Alternative: Just use SIN/COS? Not available.
        // Just make it move: target = 100
        Gene { op: OpCode::Drop, args: vec![] }, // Drop 2
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        // Target = 100

        // Write Target
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // x
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // y
        Gene { op: OpCode::GWrite, args: vec![] },

        // Jump back
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];

    // Better DNA: Oscillator using Subtraction and Brz
    // 0: Read State (0,4) initialized to 0.
    // 1: Add 1
    // 2: Write State
    // 3: Check if > 20.
    //    Push 20, Sub, Brz(reset)
    // 4: Jump(0)
    // reset: Write 0 to State, Write 0 to Target.
    // else: Write 100 to Target.

    // Let's stick to a simpler "Breathing" DNA
    // 0: Push 100
    // 1: Write Target
    // 2: Consume (Delay)
    // 3: Push 0
    // 4: Write Target
    // 5: Consume (Delay)
    // 6: Jump 0
    let breathing_genes = vec![
        // Expand
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Delay (waste energy/time)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] }, // Loop 20 times
        // Loop Start (Idx 6)
        Gene { op: OpCode::Dup, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Sub, args: vec![] },
        Gene { op: OpCode::Dup, args: vec![] },
        Gene { op: OpCode::Brz, args: vec![Nucleotide::Number(12)] }, // Exit loop
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(6)] }, // Continue loop

        // Loop End (Idx 12)
        Gene { op: OpCode::Drop, args: vec![] }, // Drop counter
        Gene { op: OpCode::Drop, args: vec![] }, // Drop 0

        // Contract
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Delay
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
        // Loop Start (Idx 19)
        Gene { op: OpCode::Dup, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Sub, args: vec![] },
        Gene { op: OpCode::Dup, args: vec![] },
        Gene { op: OpCode::Brz, args: vec![Nucleotide::Number(25)] },
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(19)] },

        // Loop End (Idx 25)
        Gene { op: OpCode::Drop, args: vec![] },
        Gene { op: OpCode::Drop, args: vec![] },

        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: breathing_genes }],
        },
    };

    let mut creature = ChimeraMesh::new(6, 12, dna);

    loop {
        clear_background(BLACK);

        // Camera
        set_camera(&Camera3D {
            position: vec3(0.0, -25.0, 25.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        // Mouse Interaction (Perturb)
        if is_mouse_button_down(MouseButton::Left) {
            let mut rng = ::rand::thread_rng();
            let idx = rng.gen_range(0..creature.system.particles.len());
            creature.system.particles[idx].pos.z += 1.0;
        }

        creature.update(0.016);
        creature.draw();

        set_default_camera();

        // HUD
        draw_text("CHIMERA-FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text("Origami Mesh driven by Genetic Code", 10.0, 50.0, 20.0, GRAY);

        // Visualize Brain State (Grid of first brain)
        if let Some(brain) = creature.brains.first() {
            let start_x = 10.0;
            let start_y = 80.0;
            draw_text(&format!("Energy: {}", brain.energy), start_x, start_y, 20.0, GOLD);
            draw_text(&format!("IP: {:?}", brain.ip), start_x, start_y + 20.0, 20.0, WHITE);
            draw_text(&format!("Strain: {}", brain.grid[0][0]), start_x, start_y + 40.0, 20.0, RED);
            draw_text(&format!("Target: {}", brain.grid[0][1]), start_x, start_y + 60.0, 20.0, GREEN);
        }

        next_frame().await
    }
}
