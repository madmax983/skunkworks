use super::normalize_coords;
use crate::ast::{Gene, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, MAX_STRANDS};

/// The Weaver Agent (🕷) traverses the grid and synthesizes DNA from Blueprints.
///
/// It looks for a "Blueprint" signal from the West (usually provided by a `B` rune or manually).
/// It compiles the blueprint's grid structure into a linear sequence of genes (a Strand).
///
/// # DNA Synthesis Logic
/// The Weaver scans the blueprint in reading order (Row by Row, Left to Right).
/// It maps recognized Runes to their `OpCode` equivalents.
///
/// - Logic: `&` -> `BitAnd`, `|` -> `BitOr`, `+` -> `BitXor`
/// - Math: `A` -> `Add`, `S` -> `Sub`, `M` -> `Mul`, `D` -> `Div`, `%` -> `Mod`
/// - Values: `Integer(n)` -> `Push(n)`
/// - Flow: `!` (Input) -> `Nop` (Assumes args on stack), `?` (Output) -> `Nop`
///
pub fn process_weaver_agent(
    vm: &mut ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let updated_agent = agent.clone();

    // 1. Scan West for Blueprint Signal
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if let Some(Value::Int(n)) = w_sig {
        // Reverse Translation: Decompile Strand n to Grid (East)
        let strand_idx = n as usize;
        if strand_idx < vm.dna.helix.strands.len() {
            let genes = vm.dna.helix.strands[strand_idx].genes.clone();
            vm.output
                .push(format!("WEAVER: Decompiling Strand {}", strand_idx));

            // Write genes linearly to East
            let mut write_offset = 1;
            for gene in genes {
                if let Some((ty, tx)) = normalize_coords(y as i64, x as i64 + write_offset) {
                    let val = match gene.op {
                        OpCode::Push => {
                            if let Some(arg) = gene.args.first() {
                                match arg {
                                    Nucleotide::Number(num) => Value::Int(*num),
                                    Nucleotide::String(s) => Value::Str(s.clone()),
                                    _ => Value::Int(0),
                                }
                            } else {
                                Value::Int(0)
                            }
                        }
                        OpCode::BitAnd => Value::Str("&".to_string()),
                        OpCode::BitOr => Value::Str("|".to_string()),
                        OpCode::BitXor => Value::Str("+".to_string()),
                        OpCode::Add => Value::Str("A".to_string()),
                        OpCode::Sub => Value::Str("S".to_string()),
                        OpCode::Mul => Value::Str("M".to_string()),
                        OpCode::Div => Value::Str("D".to_string()),
                        OpCode::Mod => Value::Str("%".to_string()),
                        _ => Value::Str(format!("{:?}", gene.op).to_lowercase()),
                    };

                    vm.grid[ty][tx] = val;
                    write_offset += 1;
                } else {
                    break; // End of grid
                }
            }

            // Signal completion
            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

            // Do not move
            return Some((updated_agent, None));
        }
    } else if let Some(Value::Junction(JunctionType::Dish, rows)) = w_sig {
        // Found a Blueprint (Junction of Junctions)!
        let mut genes = Vec::new();
        let mut valid_synthesis = false;

        // Interpret the blueprint as a 2D pattern
        // We can look for special patterns or just linearize rows
        // Let's implement a smarter "Loom" logic:
        // - Reads rows sequentially
        // - If it sees "!" (Input), it binds a value from the physical grid North of the Weaver
        // - If it sees "?" (Output), it adds a Print op

        for row in rows {
            if let Value::Junction(JunctionType::Dish, cells) = row {
                for cell in cells {
                    match cell {
                        Value::Int(n) => {
                            if n != 0 {
                                genes.push(Gene {
                                    op: OpCode::Push,
                                    args: vec![Nucleotide::Number(n)],
                                });
                                valid_synthesis = true;
                            }
                        }
                        Value::Str(s) => {
                            let op = match s.as_str() {
                                "&" => Some(OpCode::BitAnd),
                                "|" => Some(OpCode::BitOr),
                                "+" => Some(OpCode::BitXor), // XOR in Prologue
                                "A" => Some(OpCode::Add),
                                "S" => Some(OpCode::Sub),
                                "M" => Some(OpCode::Mul),
                                "D" => Some(OpCode::Div),
                                "%" => Some(OpCode::Mod),
                                "!" => {
                                    // Bind from North
                                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64)
                                    {
                                        let bound_val = match &grid_snapshot[ny][nx] {
                                            Value::Int(n) => Nucleotide::Number(*n),
                                            Value::Str(s) => Nucleotide::String(s.clone()),
                                            _ => Nucleotide::Number(0),
                                        };
                                        genes.push(Gene {
                                            op: OpCode::Push,
                                            args: vec![bound_val],
                                        });
                                        valid_synthesis = true;
                                    }
                                    None
                                }
                                "?" => Some(OpCode::Print),
                                "~" => None, // Wire (Implicit)
                                _ => None,
                            };

                            if let Some(opcode) = op {
                                genes.push(Gene {
                                    op: opcode,
                                    args: vec![],
                                });
                                valid_synthesis = true;
                            } else if let Ok(parsed_op) = s.parse::<OpCode>() {
                                // Direct OpCode support (e.g. "push", "dup")
                                genes.push(Gene {
                                    op: parsed_op,
                                    args: vec![],
                                });
                                valid_synthesis = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if valid_synthesis {
            if vm.dna.helix.strands.len() < MAX_STRANDS {
                vm.dna.helix.strands.push(Strand { genes });
                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.output
                    .push(format!("WEAVER: Synthesized Strand {}", new_idx));

                // Emitting success signal to Self
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(new_idx as i64));

                // Do not move
                return Some((updated_agent, None));
            } else {
                vm.output.push("WEAVER: Strand limit reached".to_string());
            }
        }
    }

    // 2. Move Logic (Wander or Seek Blueprint)
    // If we just synthesized, maybe rest?
    // For now, random walk or stay.
    // Let's move randomly to find more work.
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();
    use rand::Rng;
    let (dy, dx) = neighbors[rng.gen_range(0..4)];

    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        if let Value::Int(0) = &grid_snapshot[ny][nx] {
            return Some((updated_agent, Some((ny, nx))));
        }
    }

    Some((updated_agent, None))
}
