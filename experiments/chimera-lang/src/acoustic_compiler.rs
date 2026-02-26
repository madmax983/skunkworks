#![cfg(feature = "resonance")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "acoustic.pest"]
pub struct AcousticParser;

pub fn compile(source: &str) -> Result<Dna> {
    let pairs = AcousticParser::parse(Rule::score, source)?;

    let mut strands = Vec::new();
    let mut symbol_table: HashMap<String, usize> = HashMap::new();

    // Pass 1: Collect Track/Instrument Names and Assign Indices
    // We need to iterate over the parsed structure to find definitions.
    // The structure is score -> item -> (instrument_def | track_def)

    let mut strand_counter = 0;

    // Iterate manually to scan for names first.
    // Since pest iterators consume, we might need to parse twice or collect pairs.
    // Let's parse, collect pairs into a Vec, then iterate.

    // Actually, let's just do it in one pass if we can, but forward references require 2 passes.
    // We can use a placeholder and fix up later, but 'Strand' holds 'Genes' which hold 'OpCodes'.
    // Arguments to Call are indices.

    // Let's do a quick scan of the source again or just re-parse? No, re-parsing is wasteful.
    // We can clone the pairs iterator?

    // Better: Just traverse and build a list of (Name, BlockPair).

    let score_pair = pairs.clone().next().ok_or_else(|| anyhow!("Empty score"))?;

    let mut pending_tracks: Vec<(String, pest::iterators::Pair<Rule>)> = Vec::new();

    for item in score_pair.into_inner() {
        match item.as_rule() {
            Rule::item => {
                let inner = item.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::track_def => {
                        let mut parts = inner.into_inner();
                        let name = parts.next().unwrap().as_str().to_string();
                        let block = parts.next().unwrap();
                        symbol_table.insert(name.clone(), strand_counter);
                        pending_tracks.push((name, block));
                        strand_counter += 1;
                    }
                    Rule::instrument_def => {
                        let mut parts = inner.into_inner();
                        let inst_name = parts.next().unwrap().as_str().to_string();
                        // Methods
                        for method in parts {
                            if method.as_rule() == Rule::method_def {
                                let mut m_parts = method.into_inner();
                                let m_name = m_parts.next().unwrap().as_str().to_string();
                                let block = m_parts.next().unwrap();
                                let full_name = format!("{}.{}", inst_name, m_name);
                                symbol_table.insert(full_name.clone(), strand_counter);
                                pending_tracks.push((full_name, block));
                                strand_counter += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    // Pass 2: Compile Bodies
    for (name, block_pair) in pending_tracks {
        let genes = compile_block(block_pair, &symbol_table, name.as_str())?;
        strands.push(Strand { genes });
    }

    Ok(Dna {
        evolution_config: None,
        helix: Helix { strands },
    })
}

fn compile_block(
    block_pair: pest::iterators::Pair<Rule>,
    symbols: &HashMap<String, usize>,
    current_strand_name: &str,
) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();

    for stmt in block_pair.into_inner() {
        match stmt.as_rule() {
            Rule::stmt => {
                let inner = stmt.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::call_stmt => {
                        // call Name(Args)
                        let mut parts = inner.into_inner();
                        let target_name = parts.next().unwrap().as_str();

                        // Push Args first (if any)
                        if let Some(args_pair) = parts.next() {
                            for arg in args_pair.into_inner() {
                                genes.push(compile_arg_push(arg, symbols)?);
                            }
                        }

                        // Find target index
                        let target_idx = symbols
                            .get(target_name)
                            .ok_or_else(|| anyhow!("Unknown track/method: {}", target_name))?;

                        // Use Call opcode (if exists) or Jump logic?
                        // OpCode::Call is [Nova] feature. Assuming it exists.
                        // Or Jump if it's a tail call?
                        // Let's use OpCode::Call.

                        genes.push(Gene {
                            op: OpCode::Call,
                            args: vec![Nucleotide::Number(*target_idx as i64)],
                        });
                    }
                    Rule::op_stmt => {
                        // op(Name, Args...)
                        let mut parts = inner.into_inner();
                        let op_name = parts.next().unwrap().as_str();
                        let mut op_code = OpCode::from_str(op_name)
                            .map_err(|_| anyhow!("Failed to parse OpCode: {}", op_name))?;

                        if let OpCode::Unknown(_) = op_code {
                            if let Ok(lower_op) = OpCode::from_str(&op_name.to_lowercase()) {
                                if !matches!(lower_op, OpCode::Unknown(_)) {
                                    op_code = lower_op;
                                }
                            }
                        }

                        let mut args = Vec::new();
                        if let Some(args_pair) = parts.next() {
                            for arg in args_pair.into_inner() {
                                args.push(compile_arg(arg)?);
                            }
                        }

                        genes.push(Gene { op: op_code, args });
                    }
                    Rule::play_stmt => {
                        // play(note, duration, velocity?)
                        // Mapping: Push args, OpCode::Note
                        if let Some(args_pair) = inner.into_inner().next() {
                            for arg in args_pair.into_inner() {
                                genes.push(compile_arg_push(arg, symbols)?);
                            }
                        }
                        genes.push(Gene {
                            op: OpCode::Note,
                            args: vec![],
                        });
                    }
                    Rule::rest_stmt => {
                        // rest(duration)
                        if let Some(args_pair) = inner.into_inner().next() {
                            for arg in args_pair.into_inner() {
                                genes.push(compile_arg_push(arg, symbols)?);
                            }
                        }
                        genes.push(Gene {
                            op: OpCode::Rest,
                            args: vec![],
                        });
                    }
                    Rule::cmd_stmt => {
                        // Generic command: name(args)
                        // Treat as OpCode if valid, else error
                        let mut parts = inner.into_inner();
                        let cmd_name = parts.next().unwrap().as_str();

                        // Try parsing as OpCode (Case insensitive or sensitive?)
                        // OpCode::from_str expects PascalCase usually (e.g. "Push").
                        // Let's try to capitalize or use as is.
                        // But wait, user might use "push" or "Push".
                        // Let's assume user uses correct casing or we map standard ones.

                        let op_code = OpCode::from_str(cmd_name).or_else(|_| {
                            // Try Capitalized
                            let mut c = cmd_name.chars();
                            match c.next() {
                                None => Err(anyhow!("Empty command")),
                                Some(f) => {
                                    let cap = f.to_uppercase().collect::<String>() + c.as_str();
                                    OpCode::from_str(&cap).map_err(|_| {
                                        anyhow!("Unknown command/OpCode: {}", cmd_name)
                                    })
                                }
                            }
                        })?;

                        // Arguments handling:
                        // Some OpCodes take arguments in Gene (Immediate), others take from Stack.
                        // Acoustic syntax `cmd(arg)` usually implies pushing args to stack OR immediate args.
                        // Strategy: If OpCode takes arguments in definition (like Push takes 1 arg), we put it in Gene.
                        // But OpCode enum definitions in `opcode.rs` show:
                        // Push: Args: [Nucleotide...].
                        // Add: Stack: [a, b].

                        // We need to know if the OpCode expects immediate arguments.
                        // Heuristic: If parsing args succeeds and OpCode supports them.
                        // But `Gene` struct always has `args`. The VM decides usage.
                        // Most OpCodes (Add, Sub, Note) use Stack.
                        // Push uses Immediate.
                        // Jump uses Immediate.

                        // Rule:
                        // If OpCode is `Push`, `Jump`, `Brz`, `Call`, `OpCode::Note` (Wait, Note uses stack usually?)
                        // Let's check `opcode.rs` docs again.
                        // `Note`: Stack: `[ ..., velocity, duration, pitch ]`.
                        // `Push`: Args: `[Nucleotide]`.

                        // So `play(60)` -> Push(60), Note.
                        // `push(60)` -> Push(60) (Immediate).

                        // This implies we need a mapping of "Stack-based" vs "Immediate-based" Ops.
                        // Or we force explicit `op(Push, 60)` for immediate.
                        // And `cmd(arg)` -> `Push(arg), Cmd` for stack-based.

                        // For generic `cmd_stmt`, we will assume STACK-BASED arguments unless it's a known Immediate Op.
                        // Known Immediates: Push, Jump, Brz, Call.

                        let is_immediate = matches!(
                            op_code,
                            OpCode::Push
                                | OpCode::Jump
                                | OpCode::Brz
                                | OpCode::Call
                                | OpCode::Harmonize
                                | OpCode::Choir
                        );

                        if is_immediate {
                            let mut args = Vec::new();
                            if let Some(args_pair) = parts.next() {
                                for arg in args_pair.into_inner() {
                                    args.push(compile_arg(arg)?);
                                }
                            }
                            genes.push(Gene { op: op_code, args });
                        } else {
                            // Stack based: Push args first
                            if let Some(args_pair) = parts.next() {
                                for arg in args_pair.into_inner() {
                                    genes.push(compile_arg_push(arg, symbols)?);
                                }
                            }
                            genes.push(Gene {
                                op: op_code,
                                args: vec![],
                            });
                        }
                    }
                    Rule::repeat_stmt => {
                        // repeat(N) { block }
                        let mut parts = inner.into_inner();
                        let count_pair = parts.next().unwrap();
                        let count_str = count_pair.as_str(); // "repeat" ~ "(" ~ int ~ ")"
                                                             // Wait, parse tree structure for repeat_stmt is: "repeat", "(", int, ")", block.
                                                             // Pest structure: repeat_stmt -> [int, block].

                        let count: usize = count_str.parse()?;
                        let block_pair = parts.next().unwrap();

                        let block_genes = compile_block(block_pair, symbols, current_strand_name)?;

                        // Unroll
                        for _ in 0..count {
                            genes.extend(block_genes.clone());
                        }
                    }
                    Rule::loop_stmt => {
                        // loop { block }
                        // Infinite loop.
                        // Implementation: Block... Jump(SelfStart).
                        // But we can't jump to SelfStart easily without knowing our own index.
                        // We DO know our own index from `symbols` and `current_strand_name`.

                        let block_pair = inner.into_inner().next().unwrap();
                        let block_genes = compile_block(block_pair, symbols, current_strand_name)?;
                        genes.extend(block_genes);

                        if let Some(&my_idx) = symbols.get(current_strand_name) {
                            genes.push(Gene {
                                op: OpCode::Jump,
                                args: vec![Nucleotide::Number(my_idx as i64)],
                            });
                        } else {
                            // Should not happen if logic is correct
                            return Err(anyhow!("Current strand not found in symbol table"));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(genes)
}

fn compile_arg(pair: pest::iterators::Pair<Rule>) -> Result<Nucleotide> {
    match pair.as_rule() {
        Rule::arg => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::int => {
                    let v: i64 = inner.as_str().parse()?;
                    Ok(Nucleotide::Number(v))
                }
                Rule::float => {
                    // Chimera uses Int or String mostly. Float support in Nucleotide?
                    // Value::Float exists? Value enum in `value.rs`.
                    // Nucleotide only has Number(i64).
                    // We must convert float to scaled int (fixed point) or string?
                    // `acoustic_compiler` original used scaled int (x 100).
                    let v: f64 = inner.as_str().parse()?;
                    Ok(Nucleotide::Number((v * 100.0) as i64))
                }
                Rule::string => {
                    let s = inner.as_str();
                    // Remove quotes
                    Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
                }
                Rule::identifier => Ok(Nucleotide::String(inner.as_str().to_string())),
                _ => Err(anyhow!("Unknown arg type")),
            }
        }
        _ => Err(anyhow!("Expected arg")),
    }
}

fn compile_arg_push(
    pair: pest::iterators::Pair<Rule>,
    symbols: &HashMap<String, usize>,
) -> Result<Gene> {
    // Compile an argument into a Push instruction.
    // If identifier, check if it's a track name -> Push Index.

    let n = compile_arg(pair.clone())?;

    // Check identifier resolution
    if let Nucleotide::String(ref s) = n {
        if let Some(&idx) = symbols.get(s) {
            return Ok(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(idx as i64)],
            });
        }
    }

    Ok(Gene {
        op: OpCode::Push,
        args: vec![n],
    })
}
