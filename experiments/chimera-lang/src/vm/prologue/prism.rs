use super::normalize_coords;
use crate::vm::Value;
use crate::ast::JunctionType;

pub fn apply_prism_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    dna: &crate::ast::Dna,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "▲" => {
            // Disperse: West (List/Str) -> N, E, S
            if let Some(val) = w_sig {
                let parts: Vec<Value> = match val {
                    Value::Junction(_, list) => list,
                    Value::Str(s) => s.chars().map(|c| Value::Str(c.to_string())).collect(),
                    _ => vec![val],
                };

                let targets = [
                    (-1, 0), // North
                    (0, 1),  // East
                    (1, 0),  // South
                ];

                for (i, (dy, dx)) in targets.iter().enumerate() {
                    if i < parts.len() {
                        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if next_signals[ny][nx].is_none() {
                                next_signals[ny][nx] = Some(parts[i].clone());
                                changes = true;
                            }
                        }
                    }
                }
            }
        }
        "▼" => {
            // Converge: N, E, S -> Self (List)
            let mut collected = Vec::new();
            let sources = [
                (-1, 0), // North
                (0, 1),  // East
                (1, 0),  // South
            ];

            for (dy, dx) in sources {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(val) = &current_signals[ny][nx] {
                        if !is_empty(val) {
                            collected.push(val.clone());
                        }
                    }
                }
            }

            if !collected.is_empty() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] =
                        Some(Value::Junction(JunctionType::Any, collected));
                    changes = true;
                }
            }
        }
        "🧬" => {
            // Sequence: West (StrandIdx) -> Self (List of Gene Strings)
            if let Some(Value::Int(idx)) = w_sig {
                let idx = idx as usize;
                if idx < dna.helix.strands.len() {
                    let strand = &dna.helix.strands[idx];
                    let gene_strs: Vec<Value> = strand
                        .genes
                        .iter()
                        .map(|g| Value::Str(gene_to_string(g)))
                        .collect();

                    if next_signals[y][x].is_none() {
                        next_signals[y][x] =
                            Some(Value::Junction(JunctionType::Any, gene_strs));
                        changes = true;
                    }
                }
            }
        }
        "⚛" => {
            // Decompose: West (Gene String) -> North (OpCode), South (Args)
            if let Some(Value::Str(s)) = w_sig {
                // Simple parsing: Name(Arg1, Arg2)
                let (op_name, args_str) = if let Some(paren_idx) = s.find('(') {
                    let op = &s[..paren_idx];
                    let end_idx = if s.ends_with(')') {
                        s.len() - 1
                    } else {
                        s.len()
                    };
                    let args = if paren_idx + 1 < end_idx {
                        &s[paren_idx + 1..end_idx]
                    } else {
                        ""
                    };
                    (op, args)
                } else {
                    (s.as_str(), "")
                };

                // Output OpCode to North
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(Value::Str(op_name.to_string()));
                        changes = true;
                    }
                }

                // Output Args to South
                if !args_str.trim().is_empty() {
                    let args: Vec<Value> = args_str
                        .split(',')
                        .map(|a| {
                            let a = a.trim();
                            if let Ok(n) = a.parse::<i64>() {
                                Value::Int(n)
                            } else {
                                if a.starts_with('"') && a.ends_with('"') && a.len() >= 2 {
                                    Value::Str(a[1..a.len() - 1].to_string())
                                } else {
                                    Value::Str(a.to_string())
                                }
                            }
                        })
                        .collect();

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] =
                                Some(Value::Junction(JunctionType::Any, args));
                            changes = true;
                        }
                    }
                }
            }
        }
        "⚒" => {
            // Compose: West (OpCode), North (Args) -> Self (Gene String)
            // Example: West="add", North=[5, 3] -> Self="add(5, 3)"
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let Some(Value::Str(op)) = w_sig {
                let args_str = match n_sig {
                    Some(Value::Junction(_, list)) => {
                        let parts: Vec<String> = list.iter().map(value_to_arg_string).collect();
                        parts.join(", ")
                    }
                    Some(val) => value_to_arg_string(&val),
                    None => "".to_string(),
                };

                let gene_str = if args_str.is_empty() {
                    op.clone()
                } else {
                    format!("{}({})", op, args_str)
                };

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Str(gene_str));
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}

pub fn apply_prism_sinks(vm: &mut crate::vm::ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "🧶" => {
            // Synthesize: West (List of Gene Strings) -> Create Strand -> Output Index South
            if let Some(Value::Junction(_, list)) = w_sig {
                // Construct source code
                let mut source = String::new();
                source.push_str("strand synthesized_gene {\n");
                for val in list {
                    if let Value::Str(s) = val {
                        source.push_str("    ");
                        source.push_str(&s);
                        source.push('\n');
                    }
                }
                source.push_str("}\n");

                match crate::compiler::compile(&source, None) {
                    Ok(mut new_dna) => {
                        if let Some(strand) = new_dna.helix.strands.pop() {
                            vm.dna.helix.strands.push(strand);
                            let new_idx = vm.dna.helix.strands.len() - 1;

                            // Output to South
                            if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                vm.grid[sy][sx] = Value::Int(new_idx as i64);
                                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                // Light up
                            }
                        }
                    }
                    Err(e) => {
                        vm.output.push(format!("PROLOGUE: Synthesis failed: {}", e));
                    }
                }
            }
        }
        "💉" => {
            // Splice: West (StrandIdx), North (TargetIdx) -> Insert Strand
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else {
                None
            };

            if let (Some(Value::Int(idx)), Some(Value::Int(target))) = (w_sig, n_sig) {
                let idx = idx as usize;
                let target = target as usize;

                if idx < vm.dna.helix.strands.len() {
                    let strand = vm.dna.helix.strands[idx].clone();
                    if target <= vm.dna.helix.strands.len() {
                        vm.dna.helix.strands.insert(target, strand);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "✂" => {
            // Excise: West (StrandIdx) -> Remove Strand
            if let Some(Value::Int(idx)) = w_sig {
                let idx = idx as usize;
                if idx < vm.dna.helix.strands.len() {
                    vm.dna.helix.strands.remove(idx);
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        _ => {}
    }
}

fn is_empty(v: &Value) -> bool {
    match v {
        Value::Int(0) => true,
        Value::Str(s) => s.is_empty(),
        _ => false,
    }
}

fn gene_to_string(g: &crate::ast::Gene) -> String {
    let args: Vec<String> = g.args.iter().map(nucleotide_to_string).collect();
    if args.is_empty() {
        format!("{}", g.op)
    } else {
        format!("{}({})", g.op, args.join(", "))
    }
}

fn nucleotide_to_string(n: &crate::ast::Nucleotide) -> String {
    match n {
        crate::ast::Nucleotide::Number(i) => i.to_string(),
        crate::ast::Nucleotide::String(s) => format!("\"{}\"", s),
        crate::ast::Nucleotide::Identifier(s) => s.clone(),
        crate::ast::Nucleotide::Junction(_, list) => {
            let inner: Vec<String> = list.iter().map(nucleotide_to_string).collect();
            format!("[{}]", inner.join(", "))
        }
    }
}

fn value_to_arg_string(v: &Value) -> String {
    v.to_string()
}
