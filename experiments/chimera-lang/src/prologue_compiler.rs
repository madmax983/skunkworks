use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::path::Path;

use crate::ast::Dna;
use crate::vm::{Value, GRID_SIZE};

#[derive(Parser)]
#[grammar = "prologue_grammar.pest"]
pub struct PrologueParser;

pub fn compile(
    source: &str,
    base_path: Option<&Path>,
) -> Result<(Dna, Option<Vec<Vec<Value>>>, Option<bool>, HashMap<String, usize>)> {
    let mut pairs = PrologueParser::parse(Rule::program, source)?;

    let mut grid = None;
    let mut dna: Option<Dna> = None;
    let mut orca_mode = None;
    let mut custom_runes = HashMap::new();

    let program = pairs.next().ok_or_else(|| anyhow!("Empty program"))?;

    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::section => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::config_section => {
                        // "config" ~ "{" ~ config_entry* ~ "}"
                        for entry in inner.into_inner() {
                            let mut entry_inner = entry.into_inner();
                            let key = entry_inner.next().unwrap().as_str();
                            let val = entry_inner.next().unwrap().as_str();
                            match key {
                                "mode" => {
                                    if val == "Orca" {
                                        orca_mode = Some(true);
                                    } else if val == "Normal" {
                                        orca_mode = Some(false);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Rule::grid_section => {
                        let content = inner.into_inner().next().unwrap().as_str();
                        let mut new_grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];

                        // Trim leading/trailing newlines to handle brace placement
                        let trimmed = content.trim_matches(|c| c == '\n' || c == '\r');
                        let lines: Vec<&str> = trimmed.lines().collect();

                        for (y, line) in lines.iter().enumerate() {
                            if y >= GRID_SIZE {
                                break;
                            }
                            // Only process non-empty lines if we want to support blank lines as spacers?
                            // But usually ASCII grid is contiguous.
                            // If line is empty, it's an empty row in grid.

                            let tokens: Vec<&str> = line.split_whitespace().collect();
                            for (x, token) in tokens.iter().enumerate() {
                                if x >= GRID_SIZE {
                                    break;
                                }
                                let val = parse_grid_value(token);
                                new_grid[y][x] = val;
                            }
                        }
                        grid = Some(new_grid);
                    }
                    Rule::dna_section => {
                        let content = inner.into_inner().next().unwrap().as_str();
                        let compiled_dna = crate::compiler::compile(content, base_path)?;
                        if let Some(existing_dna) = dna.as_mut() {
                            existing_dna.helix.strands.extend(compiled_dna.helix.strands);
                            if existing_dna.evolution_config.is_none() {
                                existing_dna.evolution_config = compiled_dna.evolution_config;
                            }
                        } else {
                            dna = Some(compiled_dna);
                        }
                    }
                    Rule::definitions_section => {
                        for entry in inner.into_inner() {
                            let mut entry_inner = entry.into_inner();
                            let rune_char = entry_inner.next().unwrap().as_str();
                            let definition_body = entry_inner.next().unwrap();
                            // definition_body -> nested_text -> str
                            // We need the string content inside the braces
                            let content = definition_body.into_inner().next().unwrap().as_str();

                            // Wrap content in a strand definition for the compiler
                            let wrapped_content = format!("strand rune_{} {{ {} }}", custom_runes.len(), content);
                            let compiled_def = crate::compiler::compile(&wrapped_content, base_path)?;

                            if dna.is_none() {
                                dna = Some(Dna { evolution_config: None, helix: crate::ast::Helix { strands: vec![] } });
                            }

                            if let Some(main_dna) = dna.as_mut() {
                                let start_idx = main_dna.helix.strands.len();
                                // Merge strands
                                main_dna.helix.strands.extend(compiled_def.helix.strands);
                                // Map rune to start index of its definition
                                custom_runes.insert(rune_char.to_string(), start_idx);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    let final_dna = dna.ok_or_else(|| anyhow!("No DNA section found"))?;

    Ok((final_dna, grid, orca_mode, custom_runes))
}

fn parse_grid_value(s: &str) -> Value {
    if let Ok(i) = s.parse::<i64>() {
        Value::Int(i)
    } else if s == "." {
        Value::Int(0)
    } else {
        Value::Str(s.to_string())
    }
}
