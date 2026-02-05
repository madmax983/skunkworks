use bevy::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

#[derive(Component)]
pub struct ChimeraBrain {
    pub vm: ChimeraVM,
}

impl Default for ChimeraBrain {
    fn default() -> Self {
        // Default Genome: Constant gait
        // Writes Speed=50 to (0,1), Bounce=5 to (0,2), StepHeight=20 to (0,3)
        // Order for GWrite: Push(val) -> Push(y) -> Push(x) -> GWrite
        let genes = vec![
            // Speed = 50
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // Bounce = 5
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // Step Height = 20
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
            Gene { op: OpCode::GWrite, args: vec![] },

            // Jump back to 0
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];

        let dna = Dna {
            helix: Helix { strands: vec![Strand { genes }] },
        };

        Self {
            vm: ChimeraVM::new(dna),
        }
    }
}

pub fn brain_input_system(mut query: Query<&mut ChimeraBrain>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyM) {
        println!("🧬 Mutating Brains...");
        for mut brain in &mut query {
            // Force 5 mutations
            for _ in 0..5 {
                brain.vm.mutate();
            }
        }
    }

    if input.just_pressed(KeyCode::KeyR) {
        println!("♻️ Resetting Brains...");
        for mut brain in &mut query {
             *brain = ChimeraBrain::default();
        }
    }
}
