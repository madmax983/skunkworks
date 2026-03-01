use super::{normalize_coords, PrologueAgent};
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

pub fn process_forth_agent(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // 1. Extract State: [dy, dx, underfoot]
    let (mut dy, mut dx, underfoot) = match &updated_agent.state {
        Value::Junction(_, list) if list.len() >= 3 => {
            if let (Value::Int(y), Value::Int(x)) = (&list[0], &list[1]) {
                ((*y), (*x), list[2].clone())
            } else {
                (0, 1, Value::Int(0)) // Default East, Empty Underfoot
            }
        }
        _ => (0, 1, Value::Int(0)), // Default
    };

    // 2. Execute Logic (Process Underfoot)
    let cell = &underfoot;

    match cell {
        Value::Int(n) => {
            if *n != 0 {
                updated_agent.stack.push(Value::Int(*n));
            }
        }
        Value::Str(s) => {
            match s.as_str() {
                // Movement Control
                "N" => {
                    dy = -1;
                    dx = 0;
                }
                "S" => {
                    dy = 1;
                    dx = 0;
                }
                "E" => {
                    dy = 0;
                    dx = 1;
                }
                "W" => {
                    dy = 0;
                    dx = -1;
                }

                // Stack Manipulation
                "\"" | "dup" => {
                    if let Some(val) = updated_agent.stack.last() {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "_" | "drop" => {
                    updated_agent.stack.pop();
                }
                "\\" | "swap" => {
                    let len = updated_agent.stack.len();
                    if len >= 2 {
                        updated_agent.stack.swap(len - 1, len - 2);
                    }
                }
                "o" | "over" => {
                    // ( a b -- a b a )
                    let len = updated_agent.stack.len();
                    if len >= 2 {
                        let val = updated_agent.stack[len - 2].clone();
                        updated_agent.stack.push(val);
                    }
                }
                "rot" => {
                    // ( a b c -- b c a )
                    let len = updated_agent.stack.len();
                    if len >= 3 {
                        let c = updated_agent.stack.remove(len - 1); // Top
                        let b = updated_agent.stack.remove(len - 2);
                        let a = updated_agent.stack.remove(len - 3); // Bottom of 3
                        updated_agent.stack.push(b);
                        updated_agent.stack.push(c);
                        updated_agent.stack.push(a);
                    }
                }

                // Math
                "+" | "add" => binary_op(&mut updated_agent.stack, |a, b| a + b),
                "-" | "sub" => binary_op(&mut updated_agent.stack, |a, b| a - b),
                "*" | "mul" => binary_op(&mut updated_agent.stack, |a, b| a * b),
                "/" | "div" => binary_op(
                    &mut updated_agent.stack,
                    |a, b| if b != 0 { a / b } else { 0 },
                ),
                "%" | "mod" => binary_op(
                    &mut updated_agent.stack,
                    |a, b| if b != 0 { a % b } else { 0 },
                ),

                // IO
                "!" | "emit" => {
                    // Emit: Pop stack -> Signal Grid
                    if let Some(val) = updated_agent.stack.pop() {
                        vm.prologue_state.delayed_signals[y][x] = Some(val);
                    }
                }
                "?" | "read" => {
                    // Consume: Read Signal -> Push Stack
                    if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
                        updated_agent.stack.push(val.clone());
                    }
                }
                "." | "print" => {
                    // Log: Pop -> Output
                    if let Some(val) = updated_agent.stack.pop() {
                        vm.output
                            .push(format!("₣ {}: {}", updated_agent.stack.len(), val));
                    }
                }

                // Host Operations (Genetic Engineering)
                "r" | "gene_read" => {
                    // Read Gene: [strand, gene] -> [op, arg]
                    if updated_agent.stack.len() >= 2 {
                        let gene_idx_val = updated_agent.stack.pop().unwrap();
                        let strand_idx_val = updated_agent.stack.pop().unwrap();
                        if let (Value::Int(si), Value::Int(gi)) = (strand_idx_val, gene_idx_val) {
                            if si >= 0 && (si as usize) < vm.dna.helix.strands.len() {
                                let strand = &vm.dna.helix.strands[si as usize];
                                if gi >= 0 && (gi as usize) < strand.genes.len() {
                                    let gene = &strand.genes[gi as usize];
                                    let op_str = gene.op.to_string();
                                    let arg_val = if let Some(arg) = gene.args.first() {
                                        match arg {
                                            Nucleotide::Number(n) => *n,
                                            _ => 0,
                                        }
                                    } else {
                                        0
                                    };
                                    updated_agent.stack.push(Value::Str(op_str));
                                    updated_agent.stack.push(Value::Int(arg_val));
                                } else {
                                    updated_agent.stack.push(Value::Int(0)); // Fail
                                    updated_agent.stack.push(Value::Int(0));
                                }
                            } else {
                                updated_agent.stack.push(Value::Int(0));
                                updated_agent.stack.push(Value::Int(0));
                            }
                        }
                    }
                }
                "w" | "gene_write" => {
                    // Write Gene: [strand, gene, op, arg] -> []
                    if updated_agent.stack.len() >= 4 {
                        let arg_val = updated_agent.stack.pop().unwrap();
                        let op_val = updated_agent.stack.pop().unwrap();
                        let gene_idx_val = updated_agent.stack.pop().unwrap();
                        let strand_idx_val = updated_agent.stack.pop().unwrap();

                        if let (
                            Value::Int(si),
                            Value::Int(gi),
                            Value::Str(op_s),
                            Value::Int(arg_i),
                        ) = (strand_idx_val, gene_idx_val, op_val, arg_val)
                        {
                            if si >= 0 && (si as usize) < vm.dna.helix.strands.len() {
                                let strand = &mut vm.dna.helix.strands[si as usize];
                                // Auto-extend strand if needed? Or strict? Strict for now.
                                if gi >= 0 && (gi as usize) < strand.genes.len() {
                                    if let Ok(op) = op_s.parse::<OpCode>() {
                                        strand.genes[gi as usize] = Gene {
                                            op,
                                            args: vec![Nucleotide::Number(arg_i)],
                                        };
                                        vm.output.push(format!(
                                            "₣ Scribe: Replaced strand {} gene {}",
                                            si, gi
                                        ));
                                    }
                                } else if gi as usize == strand.genes.len() {
                                    // Append
                                    if let Ok(op) = op_s.parse::<OpCode>() {
                                        strand.genes.push(Gene {
                                            op,
                                            args: vec![Nucleotide::Number(arg_i)],
                                        });
                                        vm.output
                                            .push(format!("₣ Scribe: Appended to strand {}", si));
                                    }
                                }
                            }
                        }
                    }
                }
                "x" | "exec" => {
                    // Execute Strand: [strand] -> [] (Trigger Interrupt)
                    if let Some(Value::Int(si)) = updated_agent.stack.pop() {
                        if si >= 0 {
                            vm.interrupt(si as usize);
                        }
                    }
                }
                "n" | "new" => {
                    // New Strand: [] -> [new_strand_idx]
                    vm.dna.helix.strands.push(Strand { genes: vec![] });
                    let idx = vm.dna.helix.strands.len() - 1;
                    updated_agent.stack.push(Value::Int(idx as i64));
                    vm.output.push(format!("₣ Genesis: Created strand {}", idx));
                }
                "l" | "len" => {
                    // Length: [strand] -> [len] (-1 for helix len)
                    if let Some(Value::Int(idx)) = updated_agent.stack.pop() {
                        if idx < 0 {
                            updated_agent
                                .stack
                                .push(Value::Int(vm.dna.helix.strands.len() as i64));
                        } else if (idx as usize) < vm.dna.helix.strands.len() {
                            let len = vm.dna.helix.strands[idx as usize].genes.len();
                            updated_agent.stack.push(Value::Int(len as i64));
                        } else {
                            updated_agent.stack.push(Value::Int(-1));
                        }
                    }
                }
                "skip" => {
                    // Conditional Skip: Pop (cond). If 0, skip next move (stay put? No, that's delay).
                    // Skip usually means "Skip next instruction".
                    // Since instructions are spatial, we need to skip the next cell.
                    // This means jumping over it (moving 2 steps).
                    if let Some(Value::Int(0)) = updated_agent.stack.pop() {
                        // Move an extra step in current direction
                        if let Some((_ny, _nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            // Update current pos logic so the final move is from (ny, nx)
                            // But we are in "process underfoot". The move happens at step 4.
                            // We can just add to dy/dx? No, that changes direction.
                            // We need to change the *starting point* of the next move or the *magnitude*.
                            // Let's multiply dy, dx by 2 for this turn only.
                            dy *= 2;
                            dx *= 2;
                        }
                    }
                }

                // Literals (Numbers in strings)
                _ => {
                    if let Ok(n) = s.parse::<i64>() {
                        updated_agent.stack.push(Value::Int(n));
                    } else if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
                        updated_agent
                            .stack
                            .push(Value::Str(s[1..s.len() - 1].to_string()));
                    } else {
                        // Push as string literal if unknown
                        updated_agent.stack.push(Value::Str(s.to_string()));
                    }
                }
            }
        }
        _ => {}
    }

    // 4. Calculate Move Target
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        // Check if destination is blocked by another agent
        if let Value::Str(s) = &grid_snapshot[ny][nx] {
            if matches!(s.as_str(), "@" | "K" | "H" | "C" | "♻" | "♬" | "₣") {
                // Blocked - Stay put
                updated_agent.state = Value::Junction(
                    crate::ast::JunctionType::All,
                    vec![
                        Value::Int(if dy.abs() > 1 { dy / 2 } else { dy }),
                        Value::Int(if dx.abs() > 1 { dx / 2 } else { dx }),
                        underfoot,
                    ],
                );
                return Some((updated_agent, None));
            }
        }

        // 1. Restore underfoot at current pos
        vm.grid[y][x] = underfoot;

        // 2. Capture new underfoot
        let next_val = vm.grid[ny][nx].clone();

        // 3. Update state (Reset dy/dx to normal 1-step if skipped)
        let saved_dy = if dy.abs() > 1 { dy / 2 } else { dy };
        let saved_dx = if dx.abs() > 1 { dx / 2 } else { dx };

        updated_agent.state = Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(saved_dy), Value::Int(saved_dx), next_val],
        );

        return Some((updated_agent, Some((ny, nx))));
    }

    // If blocked/no move, we stay.
    let saved_dy = if dy.abs() > 1 { dy / 2 } else { dy };
    let saved_dx = if dx.abs() > 1 { dx / 2 } else { dx };

    updated_agent.state = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Int(saved_dy), Value::Int(saved_dx), underfoot],
    );

    Some((updated_agent, None))
}

fn binary_op<F>(stack: &mut Vec<Value>, op: F)
where
    F: Fn(i64, i64) -> i64,
{
    if stack.len() >= 2 {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        if let (Value::Int(ia), Value::Int(ib)) = (a, b) {
            stack.push(Value::Int(op(ia, ib)));
        } else {
            // Restore? Or consume and fail? Consuming is standard for type errors in loose langs.
        }
    }
}
