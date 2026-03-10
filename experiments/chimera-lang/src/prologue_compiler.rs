use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::path::Path;

use crate::ast::Dna;
use crate::vm::prologue::AlchemyRule;
use crate::vm::{Value, GRID_SIZE};

#[derive(Parser)]
#[grammar = "prologue_grammar.pest"]
pub struct PrologueParser;

pub type PrologueCompilerResult = Result<(
    Dna,
    Option<Vec<Vec<Value>>>,
    Option<bool>,
    HashMap<String, usize>,
    Vec<AlchemyRule>,
)>;

pub fn compile(
    source: &str,
    base_path: Option<&Path>,
) -> PrologueCompilerResult {
    let mut pairs = PrologueParser::parse(Rule::program, source)?;

    let mut grid = None;
    let mut dna: Option<Dna> = None;
    let mut orca_mode = None;
    let mut custom_runes = HashMap::new();
    let mut alchemy_book = Vec::new();

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
                        let content = inner.into_inner().next().unwrap().as_str();
                        let mut new_grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];

                        // Trim leading/trailing newlines to handle brace placement
                        let trimmed = content.trim_matches(|c| c == '\n' || c == '\r');
                        let lines: Vec<&str> = trimmed.lines().collect();

                        for (y, line) in lines.iter().enumerate() {
                            if y >= GRID_SIZE {
                                break;
                            }
                            let row = parse_grid_line(line);
                            for (x, val) in row.into_iter().enumerate() {
                                if x >= GRID_SIZE {
                                    break;
                                }
                                new_grid[y][x] = val;
                            }
                        }
                        grid = Some(new_grid);
                    }
                    Rule::dna_section => {
                        let content = inner.into_inner().next().unwrap().as_str();
                        let compiled_dna = crate::compiler::compile(content, base_path)?;
                        if let Some(existing_dna) = dna.as_mut() {
                            existing_dna
                                .helix
                                .strands
                                .extend(compiled_dna.helix.strands);
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
                            let wrapped_content =
                                format!("strand rune_{} {{ {} }}", custom_runes.len(), content);
                            let compiled_def =
                                crate::compiler::compile(&wrapped_content, base_path)?;

                            if dna.is_none() {
                                dna = Some(Dna {
                                    evolution_config: None,
                                    helix: crate::ast::Helix { strands: vec![] },
                                });
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
                    Rule::alchemy_section => {
                        for rule in inner.into_inner() {
                            if let Rule::alchemy_rule = rule.as_rule() {
                                let mut rule_inner = rule.into_inner();
                                let ingredients_pair = rule_inner.next().unwrap();
                                let result_pair = rule_inner.next().unwrap();

                                let mut ingredients = Vec::new();
                                for term in ingredients_pair.into_inner() {
                                    let inner = term.into_inner().next().unwrap();
                                    match inner.as_rule() {
                                        Rule::string_literal => {
                                            let s = inner.as_str();
                                            ingredients
                                                .push(Value::Str(s[1..s.len() - 1].to_string()));
                                        }
                                        Rule::number_literal => {
                                            if let Ok(n) = inner.as_str().parse::<i64>() {
                                                ingredients.push(Value::Int(n));
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                let result = {
                                    let inner = result_pair.into_inner().next().unwrap();
                                    match inner.as_rule() {
                                        Rule::string_literal => {
                                            let s = inner.as_str();
                                            Value::Str(s[1..s.len() - 1].to_string())
                                        }
                                        Rule::number_literal => {
                                            if let Ok(n) = inner.as_str().parse::<i64>() {
                                                Value::Int(n)
                                            } else {
                                                Value::Int(0)
                                            }
                                        }
                                        _ => Value::Int(0),
                                    }
                                };

                                alchemy_book.push(AlchemyRule {
                                    ingredients,
                                    result,
                                });
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

    Ok((final_dna, grid, orca_mode, custom_runes, alchemy_book))
}

fn parse_grid_line(line: &str) -> Vec<Value> {
    let mut row = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '"' {
            // Parse String
            chars.next(); // consume opening quote
            let mut s = String::new();
            let mut escaped = false;
            while let Some(&next_c) = chars.peek() {
                if escaped {
                    s.push(chars.next().unwrap());
                    escaped = false;
                } else if next_c == '\\' {
                    chars.next(); // consume backslash
                    escaped = true;
                } else if next_c == '"' {
                    chars.next(); // consume closing quote
                    break;
                } else {
                    s.push(chars.next().unwrap());
                }
            }
            row.push(Value::Str(s));
        } else if c.is_ascii_digit() || c == '-' {
            // Parse Number or potential single char '-' rune
            // To distinguish '-' (math) from -5 (number), we peek ahead.
            // If '-' is followed by digit, it's a number.

            let mut is_number = false;
            if c == '-' {
                let mut temp = chars.clone();
                temp.next(); // skip '-'
                if let Some(nc) = temp.peek() {
                    if nc.is_ascii_digit() {
                        is_number = true;
                    }
                }
            } else {
                is_number = true;
            }

            if is_number {
                let mut s = String::new();
                s.push(chars.next().unwrap());
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        s.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(n) = s.parse::<i64>() {
                    row.push(Value::Int(n));
                } else {
                    row.push(Value::Str(s));
                }
            } else {
                // It's a '-' rune
                row.push(Value::Str(chars.next().unwrap().to_string()));
            }
        } else if c.is_alphabetic() {
            // Identifier (might be multi-char like "func")
            // BUT: Single char runes are common (A, S, M, O).
            // Logic: If it looks like a word (>1 char), treat as String.
            // If single char, treat as String (Rune).
            let mut s = String::new();
            s.push(chars.next().unwrap());
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    s.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            row.push(Value::Str(s));
        } else {
            // Single Char Rune (Symbols like !, ?, ~, &)
            let c = chars.next().unwrap();
            if c == '.' {
                row.push(Value::Int(0));
            } else {
                row.push(Value::Str(c.to_string()));
            }
        }
    }
    row
}
