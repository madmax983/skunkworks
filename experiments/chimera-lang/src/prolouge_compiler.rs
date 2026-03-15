use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::path::Path;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{Value, GRID_SIZE};

#[derive(Parser)]
#[grammar = "prolouge_lang.pest"]
pub struct ProlougeParser;

pub fn compile(
    source: &str,
    _base_path: Option<&Path>,
) -> Result<(
    Dna,
    Option<Vec<Vec<Value>>>,
    Option<bool>,
    HashMap<String, usize>,
)> {
    let mut pairs = ProlougeParser::parse(Rule::program, source)?;

    let mut grid = None;
    let mut orca_mode = None;
    let custom_runes = HashMap::new();
    let mut strands = Vec::new();
    let mut strand_map: HashMap<String, usize> = HashMap::new();

    let program = pairs.next().ok_or_else(|| anyhow!("Empty program"))?;

    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::section => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::config_section => {
                        for entry in inner.into_inner() {
                            let mut entry_inner = entry.into_inner();
                            let key = entry_inner.next().unwrap().as_str();
                            let val = entry_inner.next().unwrap().as_str();
                            if key == "mode" {
                                if val == "Orca" {
                                    orca_mode = Some(true);
                                } else if val == "Normal" {
                                    orca_mode = Some(false);
                                }
                            }
                        }
                    }
                    Rule::grid_section => {
                        let mut parsed_grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];
                        let content = inner.into_inner().next().unwrap().as_str().trim();
                        let mut row = 0;

                        for line in content.lines() {
                            if row >= GRID_SIZE { break; }
                            let trimmed_line = line.trim();
                            if trimmed_line.is_empty() { continue; }

                            let mut col = 0;
                            // Values are space-separated
                            for val_str in trimmed_line.split_whitespace() {
                                if col >= GRID_SIZE { break; }

                                if val_str != "." {
                                    if let Ok(num) = val_str.parse::<i64>() {
                                        parsed_grid[row][col] = Value::Int(num);
                                    } else if val_str.starts_with('"') {
                                        let unquoted = val_str.trim_matches('"').to_string();
                                        parsed_grid[row][col] = Value::Str(unquoted);
                                    } else {
                                        parsed_grid[row][col] = Value::Str(val_str.to_string());
                                    }
                                }
                                col += 1;
                            }
                            row += 1;
                        }
                        grid = Some(parsed_grid);
                    }
                    Rule::dna_section => {
                        for strand_def in inner.into_inner() {
                            let mut inner_strand = strand_def.into_inner();
                            let name = inner_strand.next().unwrap().as_str().to_string();
                            let idx = strands.len();
                            strand_map.insert(name.clone(), idx);

                            let mut genes = Vec::new();
                            for instruction in inner_strand {
                                genes.extend(parse_instruction(instruction)?);
                            }
                            strands.push(Strand { genes });
                        }
                    }
                    _ => {}
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    let dna = Dna {
        helix: Helix { strands },
        evolution_config: None,
    };

    Ok((dna, grid, orca_mode, custom_runes))
}

fn parse_instruction(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::prolog_fact => {
            let mut args = Vec::new();
            for arg_pair in inner.into_inner().next().unwrap().into_inner() {
                args.push(parse_argument(arg_pair)?);
            }
            let fact = Nucleotide::Junction(JunctionType::Any, args);
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![fact],
            });
            genes.push(Gene {
                op: OpCode::Assert,
                args: vec![],
            });
        }
        Rule::prolog_query => {
            // Simplified query logic
            let query_str = inner.into_inner().next().unwrap().as_str().to_string();
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(query_str)],
            });
            #[cfg(feature = "oracle")]
            genes.push(Gene {
                op: OpCode::PrologCall,
                args: vec![],
            });
        }
        Rule::raku_hyper_op => {
            let op_sym = inner.into_inner().next().unwrap().as_str();
            let op_code = match op_sym {
                "+" => OpCode::Add,
                "-" => OpCode::Sub,
                "*" => OpCode::Mul,
                "/" => OpCode::Div,
                "%" => OpCode::Mod,
                "|" => OpCode::BitOr,
                "&" => OpCode::BitAnd,
                "^" => OpCode::BitXor,
                _ => return Err(anyhow!("Unsupported hyper operator: {}", op_sym)),
            };
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(op_code.to_string())],
            });
            #[cfg(feature = "nova")]
            genes.push(Gene {
                op: OpCode::Map, // Close enough equivalent to Hyper in Chimera OpCodes
                args: vec![],
            });
        }
        Rule::raku_reduce_op => {
            let op_sym = inner.into_inner().next().unwrap().as_str();
            let op_code = match op_sym {
                "+" => OpCode::Add,
                "-" => OpCode::Sub,
                "*" => OpCode::Mul,
                "/" => OpCode::Div,
                "%" => OpCode::Mod,
                "|" => OpCode::BitOr,
                "&" => OpCode::BitAnd,
                "^" => OpCode::BitXor,
                _ => return Err(anyhow!("Unsupported reduce operator: {}", op_sym)),
            };
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(op_code.to_string())],
            });
            #[cfg(feature = "nova")]
            genes.push(Gene {
                op: OpCode::Fold,
                args: vec![],
            });
        }
        Rule::genetic_splice => {
            let mut iter = inner.into_inner();
            let parent_a = iter.next().unwrap().as_str().to_string();
            let parent_b = iter.next().unwrap().as_str().to_string();
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(parent_b)],
            });
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(parent_a)],
            });
            #[cfg(feature = "nova")]
            genes.push(Gene {
                op: OpCode::Splice,
                args: vec![],
            });
        }
        Rule::forth_op => {
            let op_str = inner.as_str();
            let op_code = match op_str {
                "dup" => OpCode::Dup,
                "drop" => OpCode::Drop,
                "swap" => OpCode::Swap,
                "over" => OpCode::Swap, // Over isn't there, approximate with swap
                "add" => OpCode::Add,
                "sub" => OpCode::Sub,
                "mul" => OpCode::Mul,
                "div" => OpCode::Div,
                "mod" => OpCode::Mod,
                _ => return Err(anyhow!("Unsupported forth operator: {}", op_str)),
            };
            genes.push(Gene {
                op: op_code,
                args: vec![],
            });
        }
        Rule::literal => {
            let arg = parse_argument(inner)?;
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![arg],
            });
        }
        Rule::identifier => {
            let ident = inner.as_str();
            if let Ok(op) = ident.parse::<OpCode>() {
                genes.push(Gene {
                    op,
                    args: vec![],
                });
            } else {
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(ident.to_string())],
                });
                genes.push(Gene {
                    op: OpCode::Call,
                    args: vec![],
                });
            }
        }
        _ => return Err(anyhow!("Unexpected instruction rule: {:?}", inner.as_rule())),
    }
    Ok(genes)
}

fn parse_argument(pair: pest::iterators::Pair<Rule>) -> Result<Nucleotide> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => {
            let lit = inner.into_inner().next().unwrap();
            match lit.as_rule() {
                Rule::number => {
                    let num = lit.as_str().parse::<i64>()?;
                    Ok(Nucleotide::Number(num))
                }
                Rule::string_literal => {
                    let unquoted = lit.as_str().trim_matches('"').to_string();
                    Ok(Nucleotide::String(unquoted))
                }
                _ => Err(anyhow!("Unexpected literal type")),
            }
        }
        Rule::identifier | Rule::variable => Ok(Nucleotide::String(inner.as_str().to_string())),
        Rule::number => {
            let num = inner.as_str().parse::<i64>()?;
            Ok(Nucleotide::Number(num))
        }
        Rule::string_literal => {
            let unquoted = inner.as_str().trim_matches('"').to_string();
            Ok(Nucleotide::String(unquoted))
        }
        _ => Err(anyhow!("Unexpected argument type: {:?}", inner.as_rule())),
    }
}
