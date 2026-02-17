#[cfg(feature = "nova")]
use crate::ast::{Gene, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use std::str::FromStr;

#[cfg(feature = "nova")]
pub fn exec_absorb_geometry(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., radius, start_y, start_x ]
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for absorb_geometry".to_string());
        return None;
    }

    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    let r_val = vm.stack.pop().unwrap();

    if let (Value::Int(start_x), Value::Int(start_y), Value::Int(radius)) = (x_val, y_val, r_val) {
        if radius < 0 {
            vm.output
                .push("Error: Negative radius for absorb_geometry".to_string());
            return None;
        }

        let side = 2 * radius + 1;
        let count = (side * side) as usize;
        let coords = get_spiral_coords(start_y, start_x, count);

        let mut genes = Vec::new();

        for (y, x) in coords {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let val = &vm.grid[ny][nx];
                match val {
                    Value::Int(n) => {
                        genes.push(Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(*n)],
                        });
                    }
                    Value::Str(s) => {
                        // Try to parse as OpCode
                        // Handle snake_case vs PascalCase if needed, but EnumString handles PascalCase usually.
                        // Our opcodes in .dna are snake_case usually? No, `OpCode::from_str` derives `EnumString` with `serialize_all = "snake_case"`.
                        // So "push" works. "Push" might not unless case insensitive?
                        // `strum` is usually strict unless configured.
                        // Let's assume the string on grid matches the string format.
                        if let Ok(op) = OpCode::from_str(s) {
                            genes.push(Gene {
                                op,
                                args: vec![],
                            });
                        } else {
                            // Fallback to Push String
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::String(s.clone())],
                            });
                        }
                    }
                    _ => {
                        // Ignore complex types for now
                    }
                }
            }
        }

        vm.dna.helix.strands.push(Strand { genes });
        // Add default metadata
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        let new_idx = vm.dna.helix.strands.len() - 1;
        vm.stack.push(Value::Int(new_idx as i64));

        vm.output.push(format!("ABSORB: Created strand {} from geometry", new_idx));
        vm.energy = vm.energy.saturating_sub(10 + (count as i64 / 2));

    } else {
        vm.output
            .push("Error: Invalid arguments for absorb_geometry".to_string());
    }

    None
}

#[cfg(feature = "nova")]
pub fn exec_project_geometry(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., strand_idx, start_y, start_x ]
    if vm.stack.len() < 3 {
        vm.output
            .push("Error: Stack underflow for project_geometry".to_string());
        return None;
    }

    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    let s_val = vm.stack.pop().unwrap();

    if let (Value::Int(start_x), Value::Int(start_y), Value::Int(s_idx)) = (x_val, y_val, s_val) {
        if s_idx < 0 || (s_idx as usize) >= vm.dna.helix.strands.len() {
             vm.output
                .push("Error: Invalid strand index for project_geometry".to_string());
            return None;
        }

        let strand = &vm.dna.helix.strands[s_idx as usize];
        let count = strand.genes.len();
        let coords = get_spiral_coords(start_y, start_x, count);

        for (i, (y, x)) in coords.into_iter().enumerate() {
            if i >= strand.genes.len() { break; }
            let gene = &strand.genes[i];

            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                // Convert Gene to Value
                let val = match gene.op {
                    OpCode::Push => {
                        if let Some(arg) = gene.args.first() {
                            match arg {
                                Nucleotide::Number(n) => Value::Int(*n),
                                Nucleotide::String(s) => Value::Str(s.clone()),
                                _ => Value::Str("?".to_string()),
                            }
                        } else {
                            Value::Int(0)
                        }
                    },
                    _ => Value::Str(gene.op.to_string()),
                };

                vm.grid[ny][nx] = val;
            }
        }

        vm.output.push(format!("PROJECT: Inscribed strand {} onto geometry", s_idx));
        vm.energy = vm.energy.saturating_sub(10 + (count as i64 / 2));

    } else {
        vm.output
            .push("Error: Invalid arguments for project_geometry".to_string());
    }

    None
}

#[cfg(feature = "nova")]
fn get_spiral_coords(start_y: i64, start_x: i64, count: usize) -> Vec<(i64, i64)> {
    let mut coords = Vec::with_capacity(count);
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;

    for _ in 0..count {
        coords.push((start_y + y, start_x + x));

        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }

        x += dx;
        y += dy;
    }

    coords
}
