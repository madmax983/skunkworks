use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::path::Path;

use crate::ast::Dna;
use crate::vm::{Value, GRID_SIZE};

#[derive(Parser)]
#[grammar = "prologue_grammar.pest"]
pub struct PrologueParser;

pub fn compile(
    source: &str,
    base_path: Option<&Path>,
) -> Result<(Dna, Option<Vec<Vec<Value>>>, Option<bool>)> {
    let mut pairs = PrologueParser::parse(Rule::program, source)?;

    let mut grid = None;
    let mut dna = None;
    let mut orca_mode = None;

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
                        dna = Some(compiled_dna);
                    }
                    _ => {}
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    let final_dna = dna.ok_or_else(|| anyhow!("No DNA section found"))?;

    Ok((final_dna, grid, orca_mode))
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
