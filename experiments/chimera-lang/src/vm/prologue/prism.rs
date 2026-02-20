use super::normalize_coords;
use crate::vm::Value;

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
                    Value::Str(s) => s
                        .chars()
                        .map(|c| Value::Str(c.to_string()))
                        .collect(),
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
                    next_signals[y][x] = Some(Value::Junction(
                        crate::ast::JunctionType::Any,
                        collected,
                    ));
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
                        next_signals[y][x] = Some(Value::Junction(
                            crate::ast::JunctionType::Any,
                            gene_strs,
                        ));
                        changes = true;
                    }
                }
            }
        }
        "⚛" => {
            // Decompose: West (Gene String) -> North (OpCode), South (Args)
            if let Some(Value::Str(s)) = w_sig {
                // Simple parsing: Name(Arg1, Arg2)
                if let Some(paren_idx) = s.find('(') {
                    let op_name = &s[..paren_idx];
                    // Safer parsing for closing paren
                    let end_idx = if s.ends_with(')') { s.len() - 1 } else { s.len() };
                    let args_str = if paren_idx + 1 < end_idx {
                        &s[paren_idx + 1..end_idx]
                    } else {
                        ""
                    };

                    // Parse args? "1, 2" -> [1, 2]
                    let args: Vec<Value> = if args_str.trim().is_empty() {
                        Vec::new()
                    } else {
                        args_str.split(',')
                            .map(|a| {
                                let a = a.trim();
                                if let Ok(n) = a.parse::<i64>() {
                                    Value::Int(n)
                                } else {
                                    Value::Str(a.to_string())
                                }
                            })
                            .collect()
                    };

                    // Output OpCode to North
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(Value::Str(op_name.to_string()));
                            changes = true;
                        }
                    }

                    // Output Args to South
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(Value::Junction(
                                crate::ast::JunctionType::Any,
                                args,
                            ));
                            changes = true;
                        }
                    }
                } else {
                    // No parens? Just OpCode
                     if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(Value::Str(s));
                            changes = true;
                        }
                    }
                }
            }
        }
        _ => {}
    }

    changes
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
        format!("{}()", g.op)
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
