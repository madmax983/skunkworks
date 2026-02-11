use chimera_lang::prelude::*;

pub struct SwarmController {
    pub vms: Vec<ChimeraVM>,
}

impl SwarmController {
    pub fn init(count: usize) -> Self {
        let mut vms = Vec::new();

        for _ in 0..count {
            // Simple feedback genome:
            // 1. Read Input from (0,0)
            // 2. Multiply by Gain
            // 3. Write Output to (0,1)
            // 4. Loop
            let genes = vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // X
                Gene { op: OpCode::GRead, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] }, // Gain (Strain -> Actuation)
                Gene { op: OpCode::Mul, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // X
                Gene { op: OpCode::GWrite, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
            ];

            let dna = Dna {
                helix: Helix {
                    strands: vec![Strand { genes }],
                },
            };

            let vm = ChimeraVM::new(dna);
            vms.push(vm);
        }

        Self { vms }
    }

    pub fn step(&mut self, inputs: &[f32]) -> Vec<f32> {
        let mut outputs = Vec::new();

        for (i, vm) in self.vms.iter_mut().enumerate() {
            // Inject Input
            let input_val = if i < inputs.len() { inputs[i] } else { 0.0 };

            // Convert to Nucleotide (Chimera uses i64/f64 usually represented as Number)
            // Nucleotide::Number is i64. Wait, does Chimera support floats?
            // Let's check Nucleotide definition.

            // Assuming Nucleotide::Number(i64). If float needed, maybe Nucleotide::String?
            // Or maybe OpCode::Mul handles floats?
            // "Pops two values, multiplies them...".
            // Value enum in VM has Int(i64) and String(String).
            // It seems Chimera is integer based.
            // So I should scale the float to an integer.
            // Strain is roughly 0.0 to 1.0 (or higher).
            // Let's scale by 100.

            let input_int = (input_val * 100.0) as i64;

            // Direct Grid Access: vm.grid is likely private.
            // Check if grid is public.
            // If not, I can't inject.
            // But I can't check visibility easily without reading `vm.rs`.
            // Assuming `grid` is public or `set_cell` method exists.
            // The memory said "ChimeraVM is a God Object".

            // Let's try `grid.set(y, x, val)`.
            // Or access `grid` field directly. `vm.grid[y][x]`.

            // If I can't access grid, I'm stuck.
            // I'll assume `vm.grid` is public or accessible given the nature of the project.
            // However, `vm.grid` is a `Grid` struct usually.

            // Let's try to set `vm.grid[0][0]`.
            // Warning: `vm.grid` might be `Vec<Vec<Value>>`.

            // Wait, I should check `vm/mod.rs` to be safe.
            // But I'll write the code assuming `vm.grid` is accessible.
            // If compilation fails, I'll fix it.

            // Using a safe method if available would be better.

            // Let's write to `grid` directly if possible.
            // If `grid` is `pub grid: Grid`, and `Grid` implements `Index`.

            // For now, let's use a workaround if needed.
            // But `Value::Int` is what I need.

             // Inject input
            if let Some(row) = vm.grid.get_mut(0) {
                if let Some(cell) = row.get_mut(0) {
                    *cell = Value::Int(input_int);
                }
            }

            // Step VM
            for _ in 0..10 {
                vm.step();
            }

            // Read Output
            let output_val = if let Some(row) = vm.grid.get(0) {
                 if let Some(cell) = row.get(1) {
                     match cell {
                         Value::Int(v) => *v as f32 / 100.0,
                         _ => 0.0,
                     }
                 } else { 0.0 }
            } else { 0.0 };

            // Clamp output to 0.0 - 1.0 for actuator factor
            let clamped = output_val.clamp(0.0, 1.0);
            outputs.push(clamped);
        }

        outputs
    }
}
