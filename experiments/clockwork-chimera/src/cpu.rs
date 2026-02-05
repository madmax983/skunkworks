use bevy::prelude::*;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::{ChimeraVM, Value},
};

// Event triggered by the escapement tick
#[derive(Event)]
pub struct TickEvent;

#[derive(Component)]
pub struct ChimeraState {
    pub vm: ChimeraVM,
}

impl Default for ChimeraState {
    fn default() -> Self {
        // Simple "Clockwork Life" Genome
        // 0: Photosynthesize (Gain Energy from the "Sun" / Mainspring)
        // 1: Push 1
        // 2: Add (Increment counter)
        // 3: Jump 0 (Loop)

        // Note: We need to ensure the stack has at least one value for Add.
        // We will prime it in the constructor logic or use a check.
        // Actually, let's make the genome self-starting.
        // [ SLen, Push 0, Eq (Check if empty) ] -> No, too complex.

        // Let's just prime it.
        let genes = vec![
            Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let strand = Strand { genes };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Prime the stack with an initial counter value
        vm.stack.push(Value::Int(0));

        Self { vm }
    }
}

pub fn chimera_tick_system(
    mut query: Query<&mut ChimeraState>,
    mut events: EventReader<TickEvent>,
) {
    for _ in events.read() {
        for mut state in &mut query {
            if !state.vm.halted {
                state.vm.step();
                // info!("Chimera Tick. Energy: {}, IP: {:?}, Stack: {:?}", state.vm.energy, state.vm.ip, state.vm.stack);
            } else {
                // Optional: Resurrection or Restart?
            }
        }
    }
}
