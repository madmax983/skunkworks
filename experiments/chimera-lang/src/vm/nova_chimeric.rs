use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use std::str::FromStr;

#[cfg(feature = "nova")]
pub fn exec_chimeric_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let OpCode::Chimeric = op {
        if let Some(val) = vm.stack.pop() {
            let code = match val {
                Value::Str(s) => s,
                _ => format!("{}", val),
            };
            return exec_chimeric_source(vm, &code);
        } else {
            vm.output
                .push("Error: Chimeric op requires code string on stack".to_string());
        }
    }
    None
}

#[cfg(feature = "nova")]
fn exec_chimeric_source(vm: &mut ChimeraVM, source: &str) -> Option<(usize, usize)> {
    let tokens: Vec<&str> = source.split_whitespace().collect();
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        i += 1;

        match token {
            "def" => {
                if i < tokens.len() {
                    let name = tokens[i];
                    i += 1;
                    if i < tokens.len() && tokens[i] == "{" {
                        i += 1;
                        let mut genes = Vec::new();
                        while i < tokens.len() && tokens[i] != "}" {
                            let t = tokens[i];
                            // Try OpCode first
                            let mut op = None;
                            if let Ok(o) = OpCode::from_str(t) {
                                if !matches!(o, OpCode::Unknown(_)) {
                                    op = Some(o);
                                }
                            }
                            if op.is_none() {
                                if let Ok(o) = OpCode::from_str(&t.to_lowercase()) {
                                    if !matches!(o, OpCode::Unknown(_)) {
                                        op = Some(o);
                                    }
                                }
                            }

                            if let Some(o) = op {
                                genes.push(Gene {
                                    op: o,
                                    args: vec![],
                                });
                            } else if let Ok(n) = t.parse::<i64>() {
                                genes.push(Gene {
                                    op: OpCode::Push,
                                    args: vec![Nucleotide::Number(n)],
                                });
                            } else {
                                genes.push(Gene {
                                    op: OpCode::Push,
                                    args: vec![Nucleotide::String(t.to_string())],
                                });
                            }
                            i += 1;
                        }
                        if i < tokens.len() {
                            i += 1; // Skip }
                            let strand = Strand { genes };
                            vm.dna.helix.strands.push(strand);
                            let idx = vm.dna.helix.strands.len() - 1;
                            vm.dictionary.insert(name.to_string(), idx);
                            vm.output
                                .push(format!("CHIMERIC: Defined {} @ {}", name, idx));
                        }
                    }
                }
            }
            "run" => {
                if let Some(Value::Str(s)) = vm.stack.pop() {
                    if let Some(jump) = exec_chimeric_source(vm, &s) {
                        return Some(jump);
                    }
                }
            }
            "dump" => {
                vm.output.push(format!("STACK: {:?}", vm.stack));
            }
            "?" => {
                // Help / Intro
                vm.output
                    .push("CHIMERIC: Concatenative Meta-Language".to_string());
                vm.output
                    .push("Commands: def name { ... }, run, dump".to_string());
            }
            _ => {
                // Try parse as OpCode
                let mut op = None;
                if let Ok(o) = OpCode::from_str(token) {
                    if !matches!(o, OpCode::Unknown(_)) {
                        op = Some(o);
                    }
                }
                if op.is_none() {
                    if let Ok(o) = OpCode::from_str(&token.to_lowercase()) {
                        if !matches!(o, OpCode::Unknown(_)) {
                            op = Some(o);
                        }
                    }
                }

                if let Some(o) = op {
                    if let Some(jump) = vm.execute_gene_inner(o, &[]) {
                        return Some(jump);
                    }
                } else if let Ok(n) = token.parse::<i64>() {
                    vm.stack.push(Value::Int(n));
                } else if let Some(&idx) = vm.dictionary.get(token) {
                    if let Some(jump) =
                        vm.execute_gene_inner(OpCode::Call, &[Nucleotide::Number(idx as i64)])
                    {
                        return Some(jump);
                    }
                } else {
                    // String literal
                    vm.stack.push(Value::Str(token.to_string()));
                }
            }
        }
    }
    None
}
