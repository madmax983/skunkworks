use crate::state::SharedState;
use chimera_lang::prelude::*;
use rand::Rng;

pub fn init_vm() -> ChimeraVM {
    let mut rng = rand::thread_rng();
    let mut genes = Vec::new();

    // Create a random initial genome
    for _ in 0..64 {
        // Random OpCode from a small set of "builder" enzymes
        let op_choice = rng.gen_range(0..10);
        let op = match op_choice {
            0 => OpCode::Push,
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::GWrite, // Write to grid (shape the vocal tract)
            4 => OpCode::GRead,
            5 => OpCode::Jump,
            6 => OpCode::Brz,
            7 => OpCode::Photosynthesize, // Energy
            8 => OpCode::Dup,
            9 => OpCode::Consume,
            _ => OpCode::Nop,
        };

        let mut args = Vec::new();
        if matches!(op, OpCode::Push | OpCode::Jump | OpCode::Brz) {
            args.push(Nucleotide::Number(rng.gen_range(0..16)));
        }

        genes.push(Gene { op, args });
    }

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    // Give initial energy
    vm.energy = 1000;
    vm
}

pub fn update_state(vm: &mut ChimeraVM, state: &SharedState) {
    let mut params = state.params.write();
    let num_areas = params.areas.len();

    // Map Grid Columns to Areas
    // Grid is 16x16. Vocal tract is ~44.
    // We can map each column (0..15) to a region of the tract.
    // Interpolation: index i in tract maps to grid column i * (16 / 44).

    for i in 0..num_areas {
        let col_idx = (i as f32 * (16.0 / num_areas as f32)) as usize;
        if col_idx < 16 {
            // Find "center of mass" or highest value in this column
            let mut total_val = 0;
            let mut weighted_y = 0;

            for y in 0..16 {
                if let Value::Int(v) = vm.grid[y][col_idx] {
                    let val = v.abs().clamp(0, 100) as i64;
                    total_val += val;
                    weighted_y += val * (y as i64);
                }
            }

            let area = if total_val > 0 {
                // Average Y position (0..15)
                let avg_y = weighted_y as f32 / total_val as f32;
                // Map Y to area: 0 = Closed (0.1), 15 = Open (3.0)
                0.1 + (avg_y / 15.0) * 3.0
            } else {
                // Default neutral area
                1.0
            };

            params.areas[i] = area;
        }
    }

    // Map Energy/Stack to Frequency
    // Use last item on stack as pitch modulator if available
    if let Some(Value::Int(n)) = vm.stack.last() {
        let target_freq = 110.0 + (*n as f32).clamp(-50.0, 200.0);
        // Smooth transition
        params.frequency = params.frequency * 0.9 + target_freq * 0.1;
    }

    // Mutate occasionally
    if vm.tick_counter % 100 == 0 {
        vm.mutate();
    }
}
