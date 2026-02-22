//! # The Ribosome Compiler 🧬
//!
//! The `compiler` module acts as the cellular machinery that translates high-level
//! **ChimeraScript** (RNA) into executable **DNA** (OpCodes).
//!
//! ## Compilation Pipeline
//!
//! 1.  **Preprocessing**: Handling `#include` directives and ensuring no circular dependencies or path traversal attacks.
//! 2.  **Parsing**: Using `pest` to convert the source text into a Concrete Syntax Tree (CST).
//! 3.  **Gene Generation**: Walking the CST to produce `Gene` instructions, resolving macros, and linking jumps.
//!
//! ## Syntax Overview
//!
//! ChimeraScript is a concatenative, stack-oriented language.
//!
//! ```chimera
//! strand main {
//!     push(5)
//!     push(3)
//!     add
//!     print
//! }
//! ```
//!
//! See [`compile`] for usage details.

use crate::ast::{Dna, EvolutionConfig, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use strum::IntoEnumIterator;

/// The Pest parser for ChimeraScript.
///
/// Derived from `script_grammar.pest`. This handles the lexical analysis and parsing
/// of the source text into a CST (Concrete Syntax Tree).
#[derive(Parser)]
#[grammar = "script_grammar.pest"]
pub struct ScriptParser;

/// Maximum depth of `#include` directives to prevent stack overflow.
const MAX_INCLUDE_DEPTH: usize = 32;
/// Maximum depth of recursive parsing (e.g. nested Junctions).
const MAX_PARSE_DEPTH: usize = 256;
/// Maximum nesting level of brackets `{ [ (` in source code.
const MAX_NESTING_DEPTH: usize = 200;

/// Represents a defined grammar for parsing polyglot blocks.
#[derive(Debug, Clone)]
struct DefinedGrammar {
    rules: HashMap<String, Nucleotide>,
    entry_point: String,
}

/// Validates that the source code does not exceed the nesting limit.
///
/// This is a fast-fail check before parsing to prevent Pest from crashing on
/// extremely deep recursion (Stack Overflow Protection).
///
/// # Errors
///
/// Returns an error if nesting depth exceeds `limit`.
fn check_nesting_depth(source: &str, limit: usize) -> Result<()> {
    let mut depth = 0;
    for c in source.chars() {
        match c {
            '(' | '{' | '[' => {
                depth += 1;
                if depth > limit {
                    return Err(anyhow!(
                        "Recursion depth exceeded (nesting limit: {})",
                        limit
                    ));
                }
            }
            ')' | '}' | ']' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Recursively processes `#include` statements in the source code.
fn preprocess(
    source: &str,
    base_path: Option<&Path>,
    visited: &mut HashSet<PathBuf>,
    depth: usize,
) -> Result<String> {
    if depth > MAX_INCLUDE_DEPTH {
        return Err(anyhow!("Include recursion depth exceeded"));
    }

    let mut expanded = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("include") {
            expanded.push_str(line);
            expanded.push('\n');
            continue;
        }

        // Handle include
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(anyhow!("Invalid include statement: {}", trimmed));
        }

        let raw_filename = parts[1];
        let filename = raw_filename.trim_matches('"');

        let Some(bp) = base_path else {
            return Err(anyhow!("Cannot include files without a base path"));
        };

        let path = bp.join(filename);

        let abs_path = if path.exists() {
            path.canonicalize()?
        } else {
            path.clone()
        };

        let effective_base = if bp.as_os_str().is_empty() {
            Path::new(".")
        } else {
            bp
        };

        let canonical_base = effective_base.canonicalize().map_err(|e| {
            anyhow!(
                "Security Error: Failed to resolve base path {:?}: {}",
                effective_base,
                e
            )
        })?;

        if !abs_path.starts_with(&canonical_base) {
            return Err(anyhow!(
                "Security Error: Path traversal attempt detected. Access denied to {:?}",
                abs_path
            ));
        }

        if !visited.insert(abs_path.clone()) {
            return Err(anyhow!("Recursive include detected: {:?}", abs_path));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| anyhow!("Failed to include file {:?}: {}", path, e))?;

        let sub_expanded = preprocess(&content, Some(bp), visited, depth + 1)?;
        expanded.push_str(&sub_expanded);
        expanded.push('\n');

        visited.remove(&abs_path);
    }
    Ok(expanded)
}

/// Compiles ChimeraScript source code into DNA.
pub fn compile(source: &str, base_path: Option<&Path>) -> Result<Dna> {
    // Phase 1: Preprocessing (Includes)
    let mut visited = HashSet::new();
    let expanded_source = preprocess(source, base_path, &mut visited, 0)?;

    // Phase 2: Safety Checks
    check_nesting_depth(&expanded_source, MAX_NESTING_DEPTH)?;

    // Phase 3: Parsing (Text -> CST)
    let mut pairs = ScriptParser::parse(Rule::program, &expanded_source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    // Pass 1: Collect strand names and macros
    let mut strand_map: HashMap<String, usize> = HashMap::new();
    let mut macro_map: HashMap<String, pest::iterators::Pairs<Rule>> = HashMap::new();
    let mut grammar_map: HashMap<String, DefinedGrammar> = HashMap::new();
    let mut organelle_map: HashMap<String, usize> = HashMap::new();
    let mut grid_maps: HashMap<String, Vec<String>> = HashMap::new();
    let mut evolution_config: Option<EvolutionConfig> = None;

    for pair in program.clone().into_inner() {
        match pair.as_rule() {
            Rule::map_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let content_pair = inner.next().unwrap();
                let content = content_pair.as_str();

                let trimmed = content.trim_matches(|c| c == '\n' || c == '\r');
                let lines: Vec<&str> = trimmed.lines().collect();

                let min_indent = lines
                    .iter()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.chars().take_while(|c| *c == ' ').count())
                    .min()
                    .unwrap_or(0);

                let rows: Vec<String> = lines
                    .iter()
                    .map(|line| {
                        if line.len() >= min_indent {
                            line[min_indent..].trim_end().to_string()
                        } else {
                            line.trim_end().to_string()
                        }
                    })
                    .collect();

                grid_maps.insert(name.to_string(), rows);
            }
            Rule::strand_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let idx = strand_map.len();
                if strand_map.insert(name.to_string(), idx).is_some() {
                    return Err(anyhow!("Duplicate strand name: {}", name));
                }
            }
            Rule::organelle_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let strand_name = format!("{}_DNA", name);
                let idx = strand_map.len();
                if strand_map.insert(strand_name, idx).is_some() {
                    return Err(anyhow!("Duplicate strand name from organelle: {}", name));
                }
                organelle_map.insert(name.to_string(), idx);
            }
            Rule::chimera_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let strand_name = format!("{}_DNA", name);
                let idx = strand_map.len();
                if strand_map.insert(strand_name, idx).is_some() {
                    return Err(anyhow!("Duplicate strand name from chimera: {}", name));
                }
                organelle_map.insert(name.to_string(), idx);
            }
            Rule::macro_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                macro_map.insert(name.to_string(), inner);
            }
            Rule::grammar_def => {
                let mut inner = pair.into_inner();
                let name_pair = inner.next().ok_or(anyhow!("Missing grammar name"))?;
                let name = name_pair.as_str();
                let content_pair = inner.next().ok_or(anyhow!("Missing grammar content"))?;

                let grammar = parse_grammar_def(content_pair)?;
                grammar_map.insert(name.to_string(), grammar);
            }
            _ => {}
        }
    }

    let mut strands_ast = Vec::new();
    let mut anonymous_strands = Vec::new();

    // Pass 2: Generate Genes
    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::strand_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip name
                let mut genes = Vec::new();

                for instr in inner {
                    let generated = parse_instructions(
                        instr,
                        &strand_map,
                        &macro_map,
                        &grammar_map,
                        &organelle_map,
                        &grid_maps,
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            Rule::organelle_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip name
                let mut genes = Vec::new();

                for instr in inner {
                    let generated = parse_instructions(
                        instr,
                        &strand_map,
                        &macro_map,
                        &grammar_map,
                        &organelle_map,
                        &grid_maps,
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            Rule::chimera_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip name
                let grammar_arg_pair = inner.next().unwrap(); // grammar argument
                let boot_block_pair = inner.next().unwrap(); // block

                let grammar_val = parse_argument(grammar_arg_pair, &strand_map, 0)?;

                // Prepend push(grammar)
                let mut genes = vec![Gene {
                    op: OpCode::Push,
                    args: vec![grammar_val],
                }];

                // Compile block instructions
                for instr in boot_block_pair.into_inner() {
                    let generated = parse_instructions(
                        instr,
                        &strand_map,
                        &macro_map,
                        &grammar_map,
                        &organelle_map,
                        &grid_maps,
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            Rule::evolution_def => {
                let mut inner = pair.into_inner();
                let _name = inner.next().unwrap(); // skip identifier

                let mut pop_size = 50;
                let mut mut_rate = "0.1".to_string();
                let mut fitness_idx = None;
                let mut target = None;

                for prop in inner {
                    let mut prop_parts = prop.into_inner();
                    // Each part is (key ~ ":" ~ val)
                    // But pest returns flat children of prop
                    // e.g. "population", ":", "number"
                    // Wait, `evolution_prop` rule has alternatives.
                    // The first token is the keyword.
                    let key_token = prop_parts.next().unwrap();
                    match key_token.as_str() {
                        "population" => {
                            let _colon = prop_parts.next();
                            let val_str = prop_parts.next().unwrap().as_str();
                            pop_size = val_str.parse().unwrap_or(50);
                        }
                        "mutation_rate" => {
                            let _colon = prop_parts.next();
                            mut_rate = prop_parts.next().unwrap().as_str().to_string();
                        }
                        "target" => {
                            let _colon = prop_parts.next();
                            let val_str = prop_parts.next().unwrap().as_str();
                            target = val_str.parse().ok();
                        }
                        "fitness" => {
                            let block_pair = prop_parts.next().unwrap();
                            // Compile block.
                            // We need to parse this block into genes, then put it into anonymous strands.
                            // We can use parse_instructions on the block?
                            // But `instruction` rule has `block` as a choice.
                            // We need to create a `CompilerContext` and call `parse_block` directly or wrap it.
                            // Actually `block` is an instruction type.
                            // Let's manually invoke parse_block via parse_instructions context manually.

                            // Wait, `parse_instruction` expects an `instruction` pair.
                            // `block` is a rule inside `instruction`.
                            // But here we have `block` pair directly from `evolution_prop`.
                            // We need `CompilerContext::parse_block`.

                            let mut ctx = CompilerContext {
                                strand_map: &strand_map,
                                macro_map: &macro_map,
                                grammar_map: &grammar_map,
                                organelle_map: &organelle_map,
                                grid_maps: &grid_maps,
                                anonymous_strands: &mut anonymous_strands,
                                depth: 0,
                            };

                            // parse_block returns [Push(idx)].
                            // It takes `Rule::block` pair.
                            let genes = ctx.parse_block(block_pair)?;
                            if let Some(Gene {
                                op: OpCode::Push,
                                args,
                            }) = genes.first()
                            {
                                if let Some(Nucleotide::Number(idx)) = args.first() {
                                    fitness_idx = Some(*idx as usize);
                                }
                            }
                        }
                        _ => {}
                    }
                }

                evolution_config = Some(EvolutionConfig {
                    population_size: pop_size,
                    mutation_rate: mut_rate,
                    fitness_strand_idx: fitness_idx,
                    target_value: target,
                });
            }
            _ => {}
        }
    }

    // Append anonymous strands
    strands_ast.extend(anonymous_strands);

    Ok(Dna {
        helix: Helix {
            strands: strands_ast,
        },
        evolution_config,
    })
}

fn parse_grammar_def(pair: pest::iterators::Pair<Rule>) -> Result<DefinedGrammar> {
    let mut rules = HashMap::new();
    let mut entry_point = String::new();

    for rule_def in pair.into_inner() {
        if rule_def.as_rule() == Rule::grammar_rule_def {
            let mut parts = rule_def.into_inner();
            let _type = parts.next().ok_or(anyhow!("Missing rule type"))?.as_str();
            let name = parts
                .next()
                .ok_or(anyhow!("Missing rule name"))?
                .as_str()
                .to_string();
            let expr = parts.next().ok_or(anyhow!("Missing rule expression"))?;
            let transform = parts.next();

            if entry_point.is_empty() {
                entry_point = name.clone();
            }

            let parser_node = parse_rule_expr(expr)?;

            // If transform exists, wrap in Map
            let final_node = if let Some(t) = transform {
                let s = t.as_str();
                // Strip { and }
                let template_str = s[1..s.len() - 1].trim();
                Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("Map".to_string()),
                        parser_node,
                        Nucleotide::String(template_str.to_string()),
                    ],
                )
            } else {
                parser_node
            };

            rules.insert(name, final_node);
        }
    }

    Ok(DefinedGrammar { rules, entry_point })
}

fn parse_rule_expr(pair: pest::iterators::Pair<Rule>) -> Result<Nucleotide> {
    // rule_expr = { rule_choice }
    // rule_choice = { rule_seq ~ ( "|" ~ rule_seq )* }

    let choice_pair = pair.into_inner().next().unwrap(); // rule_choice
    let mut choices = Vec::new();

    for seq_pair in choice_pair.into_inner() {
        choices.push(parse_rule_seq(seq_pair)?);
    }

    if choices.len() == 1 {
        Ok(choices[0].clone())
    } else {
        let mut args = vec![Nucleotide::String("Alt".to_string())];
        args.extend(choices);
        Ok(Nucleotide::Junction(JunctionType::Any, args))
    }
}

fn parse_rule_seq(pair: pest::iterators::Pair<Rule>) -> Result<Nucleotide> {
    // rule_seq = { rule_term+ }
    let mut terms = Vec::new();
    for term_pair in pair.into_inner() {
        terms.push(parse_rule_term(term_pair)?);
    }

    if terms.len() == 1 {
        Ok(terms[0].clone())
    } else {
        let mut args = vec![Nucleotide::String("Seq".to_string())];
        args.extend(terms);
        Ok(Nucleotide::Junction(JunctionType::Any, args))
    }
}

fn parse_rule_term(pair: pest::iterators::Pair<Rule>) -> Result<Nucleotide> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            Ok(Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Match".to_string()),
                    Nucleotide::String(content),
                ],
            ))
        }
        Rule::regex_literal => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string(); // strip / /
            Ok(Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Regex".to_string()),
                    Nucleotide::String(content),
                ],
            ))
        }
        Rule::rule_ref => {
            let name = inner.as_str().to_string();
            Ok(Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Ref".to_string()),
                    Nucleotide::String(name),
                ],
            ))
        }
        Rule::group => {
            let expr = inner.into_inner().next().unwrap();
            parse_rule_expr(expr)
        }
        Rule::action_block => {
            // For now, treat action block as a Map on the preceding term?
            // But here it is a term itself.
            // Maybe it consumes nothing and produces a value?
            // Not supported yet.
            Ok(Nucleotide::Junction(
                JunctionType::Any,
                vec![
                    Nucleotide::String("Match".to_string()),
                    Nucleotide::String("".to_string()),
                ],
            ))
        }
        _ => Err(anyhow!("Unknown rule term: {:?}", inner.as_rule())),
    }
}

/// Internal state used during the gene generation phase.
struct CompilerContext<'a, 'i> {
    strand_map: &'a HashMap<String, usize>,
    macro_map: &'a HashMap<String, pest::iterators::Pairs<'i, Rule>>,
    grammar_map: &'a HashMap<String, DefinedGrammar>,
    organelle_map: &'a HashMap<String, usize>,
    grid_maps: &'a HashMap<String, Vec<String>>,
    anonymous_strands: &'a mut Vec<Strand>,
    depth: usize,
}

impl<'a, 'i> CompilerContext<'a, 'i> {
    fn parse_instruction(&mut self, pair: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        if self.depth > 50 {
            return Err(anyhow!("Macro recursion depth exceeded"));
        }

        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::block => self.parse_block(inner),
            Rule::literal => self.parse_literal_instruction(inner),
            Rule::junction => self.parse_junction_instruction(inner),
            Rule::simple_op => self.parse_simple_op(inner),
            Rule::arrow_jump => self.parse_arrow_jump(inner),
            Rule::question_branch => self.parse_question_branch(inner),
            Rule::call => self.parse_call(inner),
            Rule::polyglot_block => self.parse_polyglot_block(inner),
            #[cfg(feature = "nova")]
            Rule::crispr_block => self.parse_crispr_block(inner),
            Rule::chaos_block => self.parse_chaos_block(inner),
            Rule::apply_map_stmt => self.parse_apply_map(inner),
            #[cfg(feature = "oracle")]
            Rule::oracle_block => self.parse_oracle_block(inner),
            #[cfg(not(feature = "oracle"))]
            Rule::oracle_block => return Err(anyhow!("Oracle feature is disabled")),
            Rule::hyper_op => self.parse_hyper_op(inner),
            Rule::cross_op => self.parse_meta_op(inner, OpCode::Cross),
            Rule::reduce_op => self.parse_meta_op(inner, OpCode::Reduce),
            Rule::zip_op => self.parse_meta_op(inner, OpCode::ZipWith),
            _ => unreachable!("Unexpected instruction rule: {:?}", inner.as_rule()),
        }
    }

    fn parse_hyper_op(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let op_symbol = parts.next().unwrap().as_str();
        let op = match op_symbol {
            "+" => OpCode::HyperAdd,
            "-" => OpCode::HyperSub,
            "*" => OpCode::HyperMul,
            "/" => OpCode::HyperDiv,
            _ => return Err(anyhow!("Unsupported hyper operator: {}", op_symbol)),
        };
        Ok(vec![Gene { op, args: vec![] }])
    }

    fn parse_meta_op(
        &self,
        inner: pest::iterators::Pair<'i, Rule>,
        opcode: OpCode,
    ) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let op_symbol = parts.next().unwrap().as_str();
        Ok(vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(op_symbol.to_string())],
            },
            Gene {
                op: opcode,
                args: vec![],
            },
        ])
    }

    fn parse_apply_map(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let map_name = parts.next().unwrap().as_str().trim_matches('"');
        let x_val_str = parts.next().unwrap().as_str();
        let y_val_str = parts.next().unwrap().as_str();

        let base_x: i64 = x_val_str.parse()?;
        let base_y: i64 = y_val_str.parse()?;

        if let Some(rows) = self.grid_maps.get(map_name) {
            let mut genes = Vec::new();
            for (r, row) in rows.iter().enumerate() {
                for (c, ch) in row.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(ch as i64)],
                    });
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(base_y + r as i64)],
                    });
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(base_x + c as i64)],
                    });
                    #[cfg(feature = "nova")]
                    genes.push(Gene {
                        op: OpCode::Rune,
                        args: vec![],
                    });
                }
            }
            Ok(genes)
        } else {
            Err(anyhow!("Unknown map: {}", map_name))
        }
    }

    fn parse_junction_instruction(
        &self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let val = parse_junction(inner, self.strand_map, 0)?;
        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![val],
        }])
    }

    fn parse_chaos_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let block = inner.into_inner().next().unwrap();
        let mut genes = Vec::new();

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("0.1".to_string())],
        });
        genes.push(Gene {
            op: OpCode::HavocRate,
            args: vec![],
        });

        self.depth += 1;
        for instr in block.into_inner() {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        });
        genes.push(Gene {
            op: OpCode::HavocRate,
            args: vec![],
        });

        Ok(genes)
    }

    #[cfg(feature = "oracle")]
    fn parse_oracle_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut genes = Vec::new();
        for stmt in inner.into_inner() {
            let def = stmt.into_inner().next().unwrap();
            match def.as_rule() {
                Rule::fact_def => {
                    let arg_list = def.into_inner().next().unwrap();
                    let mut fact_terms = Vec::new();
                    for arg in arg_list.into_inner() {
                        fact_terms.push(parse_argument(arg, self.strand_map, 0)?);
                    }
                    let fact = Nucleotide::Junction(crate::ast::JunctionType::Any, fact_terms);
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![fact],
                    });
                    genes.push(Gene {
                        op: OpCode::Assert,
                        args: vec![],
                    });
                }
                Rule::rule_def => {
                    let mut parts = def.into_inner();
                    let head_args = parts.next().unwrap();
                    let query_expr = parts.next().unwrap();

                    let mut head_terms = Vec::new();
                    for arg in head_args.into_inner() {
                        head_terms.push(parse_argument(arg, self.strand_map, 0)?);
                    }
                    let head = Nucleotide::Junction(crate::ast::JunctionType::Any, head_terms);

                    let mut body_goals = Vec::new();
                    for pred in query_expr.into_inner() {
                        let mut pred_parts = pred.into_inner();
                        let pred_name = pred_parts.next().unwrap().as_str();
                        let pred_args = pred_parts.next().unwrap();

                        let mut term_args = vec![Nucleotide::String(pred_name.to_string())];
                        for arg in pred_args.into_inner() {
                            term_args.push(parse_argument(arg, self.strand_map, 0)?);
                        }
                        body_goals.push(Nucleotide::Junction(
                            crate::ast::JunctionType::Any,
                            term_args,
                        ));
                    }

                    let body = if body_goals.len() == 1 {
                        body_goals[0].clone()
                    } else {
                        Nucleotide::Junction(crate::ast::JunctionType::All, body_goals)
                    };

                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![head],
                    });
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![body],
                    });
                    genes.push(Gene {
                        op: OpCode::Rule,
                        args: vec![],
                    });
                }
                _ => {}
            }
        }
        Ok(genes)
    }

    fn parse_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut genes = Vec::new();
        self.depth += 1;
        for instr in inner.into_inner() {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        let index = self.strand_map.len() + self.anonymous_strands.len();
        self.anonymous_strands.push(Strand { genes });

        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(index as i64)],
        }])
    }

    fn parse_literal_instruction(
        &self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let val = parse_literal(inner, self.strand_map)?;
        Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![val],
        }])
    }

    fn parse_simple_op(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let name = inner.clone().into_inner().next().unwrap().as_str();
        if let Some(body) = self.macro_map.get(name) {
            let mut macro_genes = Vec::new();
            self.depth += 1;
            for instr in body.clone() {
                let sub = self.parse_instruction(instr)?;
                macro_genes.extend(sub);
            }
            self.depth -= 1;
            return Ok(macro_genes);
        }

        #[cfg(feature = "nova")]
        if let Some(stripped) = name.strip_prefix("Spawn") {
            if let Some(&idx) = self.organelle_map.get(stripped) {
                return Ok(vec![
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(idx as i64)],
                    },
                    Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(0)],
                    },
                    Gene {
                        op: OpCode::Spawn,
                        args: vec![],
                    },
                ]);
            }
        }

        if let Some(&idx) = self.strand_map.get(name) {
            return Ok(vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(idx as i64)],
            }]);
        }

        let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
        if let OpCode::Unknown(_) = &op {
            if let Some(suggestion) = suggest_opcode(name) {
                eprintln!(
                    "Warning: Unknown enzyme '{}'. Did you mean '{}'?",
                    name, suggestion
                );
            }
        }
        Ok(vec![Gene { op, args: vec![] }])
    }

    fn parse_arrow_jump(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();
        let arg = resolve_target(target_name, self.strand_map);
        Ok(vec![Gene {
            op: OpCode::Jump,
            args: vec![arg],
        }])
    }

    fn parse_question_branch(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();
        let arg = resolve_target(target_name, self.strand_map);
        Ok(vec![Gene {
            op: OpCode::Brz,
            args: vec![arg],
        }])
    }

    fn parse_call(&self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let name = parts.next().unwrap().as_str();
        let args_pair = parts.next().unwrap();

        let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
        if let OpCode::Unknown(_) = &op {
            if let Some(suggestion) = suggest_opcode(name) {
                eprintln!(
                    "Warning: Unknown enzyme '{}'. Did you mean '{}'?",
                    name, suggestion
                );
            }
        }
        let mut args = Vec::new();

        for arg_pair in args_pair.into_inner() {
            let val = parse_argument(arg_pair, self.strand_map, 0)?;
            args.push(val);
        }

        let is_intrinsic = match op {
            OpCode::Jump | OpCode::Brz => true,
            #[cfg(feature = "nova")]
            OpCode::Call | OpCode::Poly => true,
            _ => false,
        };

        if op == OpCode::Push {
            let mut genes = Vec::new();
            for arg in args {
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![arg],
                });
            }
            Ok(genes)
        } else if is_intrinsic {
            Ok(vec![Gene { op, args }])
        } else {
            let mut genes = Vec::new();
            for arg in args {
                genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![arg],
                });
            }
            genes.push(Gene { op, args: vec![] });
            Ok(genes)
        }
    }

    fn parse_polyglot_block(
        &mut self,
        inner: pest::iterators::Pair<'i, Rule>,
    ) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let grammar_name = parts.next().unwrap().as_str();
        let content_pair = parts.next().unwrap();
        let content = content_pair.as_str();

        let grammar = self
            .grammar_map
            .get(grammar_name)
            .ok_or(anyhow!("Unknown grammar: {}", grammar_name))?;

        let entry_rule = grammar.rules.get(&grammar.entry_point).ok_or(anyhow!(
            "Entry point '{}' not found in grammar",
            grammar.entry_point
        ))?;

        let (ast, consumed) = babel_parse(entry_rule, content, &grammar.rules)?;

        // Simple trim check for trailing whitespace?
        // babel_parse is greedy based on rules.
        // If content has trailing spaces and grammar doesn't consume them, consumed < len.
        if consumed != content.len() {
            // For robustness, ignore trailing whitespace?
            let remainder = &content[consumed..];
            if !remainder.trim().is_empty() {
                return Err(anyhow!(
                    "Incomplete parse. Consumed {}/{} chars. Remainder: '{}'",
                    consumed,
                    content.len(),
                    remainder
                ));
            }
        }

        flatten_ast(&ast)
    }

    #[cfg(feature = "nova")]
    fn parse_crispr_block(&mut self, inner: pest::iterators::Pair<'i, Rule>) -> Result<Vec<Gene>> {
        let mut parts = inner.into_inner();
        let target_name = parts.next().unwrap().as_str();

        let pattern_pair = parts.next().unwrap();
        let mut pattern_genes = Vec::new();
        for op_pair in pattern_pair.into_inner() {
            if op_pair.as_rule() == Rule::identifier {
                let op_str = op_pair.as_str();
                let op = OpCode::from_str(op_str)
                    .map_err(|_| anyhow!("Unknown opcode in pattern: {}", op_str))?;
                pattern_genes.push(Gene { op, args: vec![] });
            }
        }

        let guide_idx = self.strand_map.len() + self.anonymous_strands.len();
        self.anonymous_strands.push(Strand {
            genes: pattern_genes,
        });

        let mut genes = Vec::new();

        let target_arg = resolve_target(target_name, self.strand_map);
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![target_arg],
        });

        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(guide_idx as i64)],
        });

        genes.push(Gene {
            op: OpCode::CrisprScan,
            args: vec![],
        });

        self.depth += 1;
        for instr in parts {
            let sub = self.parse_instruction(instr)?;
            genes.extend(sub);
        }
        self.depth -= 1;

        Ok(genes)
    }
}

fn parse_instructions(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    macro_map: &HashMap<String, pest::iterators::Pairs<Rule>>,
    grammar_map: &HashMap<String, DefinedGrammar>,
    organelle_map: &HashMap<String, usize>,
    grid_maps: &HashMap<String, Vec<String>>,
    anonymous_strands: &mut Vec<Strand>,
    depth: usize,
) -> Result<Vec<Gene>> {
    let mut ctx = CompilerContext {
        strand_map,
        macro_map,
        grammar_map,
        organelle_map,
        grid_maps,
        anonymous_strands,
        depth,
    };
    ctx.parse_instruction(pair)
}

fn resolve_target(name: &str, strand_map: &HashMap<String, usize>) -> Nucleotide {
    if let Some(&idx) = strand_map.get(name) {
        Nucleotide::Number(idx as i64)
    } else {
        Nucleotide::Identifier(name.to_string())
    }
}

fn parse_literal(
    pair: pest::iterators::Pair<Rule>,
    _strand_map: &HashMap<String, usize>,
) -> Result<Nucleotide> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::number => Ok(Nucleotide::Number(inner.as_str().parse()?)),
        Rule::string => {
            let s = inner.as_str();
            Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
        }
        _ => unreachable!("Unexpected literal rule"),
    }
}

fn parse_argument(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    depth: usize,
) -> Result<Nucleotide> {
    if depth > MAX_PARSE_DEPTH {
        return Err(anyhow!("Recursion depth exceeded"));
    }
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => parse_literal(inner, strand_map),
        Rule::identifier | Rule::variable => {
            let id = inner.as_str();
            if inner.as_rule() == Rule::identifier {
                if let Some(&idx) = strand_map.get(id) {
                    return Ok(Nucleotide::Number(idx as i64));
                }
            }
            if inner.as_rule() == Rule::variable {
                Ok(Nucleotide::String(id.to_string()))
            } else {
                Ok(Nucleotide::Identifier(id.to_string()))
            }
        }
        Rule::junction => parse_junction(inner, strand_map, depth + 1),
        Rule::data_call => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str();
            let args_pair = parts.next().unwrap();
            let mut args = vec![Nucleotide::String(name.to_string())];
            for arg in args_pair.into_inner() {
                args.push(parse_argument(arg, strand_map, depth + 1)?);
            }
            Ok(Nucleotide::Junction(crate::ast::JunctionType::Any, args))
        }
        _ => unreachable!("Unexpected argument rule: {:?}", inner.as_rule()),
    }
}

fn parse_junction(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    depth: usize,
) -> Result<Nucleotide> {
    if depth > MAX_PARSE_DEPTH {
        return Err(anyhow!("Recursion depth exceeded"));
    }
    let mut parts = pair.into_inner();
    let type_str = parts.next().unwrap().as_str();
    let args_pair = parts.next().unwrap();

    let t = match type_str {
        "any" => crate::ast::JunctionType::Any,
        "all" => crate::ast::JunctionType::All,
        _ => return Err(anyhow!("Invalid junction type")),
    };

    let mut vals = Vec::new();
    for arg in args_pair.into_inner() {
        vals.push(parse_argument(arg, strand_map, depth + 1)?);
    }
    Ok(Nucleotide::Junction(t, vals))
}

fn babel_parse(
    grammar: &Nucleotide,
    input: &str,
    rule_set: &HashMap<String, Nucleotide>,
) -> Result<(Nucleotide, usize)> {
    use crate::ast::JunctionType;

    if let Nucleotide::Junction(JunctionType::Any, args) = grammar {
        if args.is_empty() {
            return Err(anyhow!("Empty grammar node"));
        }
        if let Nucleotide::String(type_str) = &args[0] {
            match type_str.as_str() {
                "Match" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Match requires pattern"));
                    }
                    if let Nucleotide::String(pattern) = &args[1] {
                        if input.starts_with(pattern) {
                            return Ok((Nucleotide::String(pattern.clone()), pattern.len()));
                        }
                    }
                    Err(anyhow!("Match failed"))
                }
                "Regex" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Regex requires pattern"));
                    }
                    if let Nucleotide::String(pattern) = &args[1] {
                        let anchored = format!("^{}", pattern);
                        let re = Regex::new(&anchored).map_err(|e| anyhow!("{}", e))?;
                        if let Some(mat) = re.find(input) {
                            let match_str = mat.as_str().to_string();
                            let len = match_str.len();
                            return Ok((Nucleotide::String(match_str), len));
                        }
                    }
                    Err(anyhow!("Regex failed"))
                }
                "Seq" => {
                    let mut total_consumed = 0;
                    let mut results = Vec::new();
                    for parser in args.iter().skip(1) {
                        let (res, consumed) =
                            babel_parse(parser, &input[total_consumed..], rule_set)?;
                        results.push(res);
                        total_consumed += consumed;
                    }
                    Ok((
                        Nucleotide::Junction(JunctionType::All, results),
                        total_consumed,
                    ))
                }
                "Alt" => {
                    for parser in args.iter().skip(1) {
                        if let Ok((res, consumed)) = babel_parse(parser, input, rule_set) {
                            return Ok((res, consumed));
                        }
                    }
                    Err(anyhow!("Alt failed"))
                }
                "Many" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Many requires parser"));
                    }
                    let p = &args[1];
                    let mut results = Vec::new();
                    let mut total_consumed = 0;
                    while let Ok((res, consumed)) =
                        babel_parse(p, &input[total_consumed..], rule_set)
                    {
                        if consumed == 0 {
                            break;
                        }
                        results.push(res);
                        total_consumed += consumed;
                    }
                    Ok((
                        Nucleotide::Junction(JunctionType::All, results),
                        total_consumed,
                    ))
                }
                "Opt" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Opt requires parser"));
                    }
                    let p = &args[1];
                    if let Ok((res, consumed)) = babel_parse(p, input, rule_set) {
                        Ok((res, consumed))
                    } else {
                        Ok((Nucleotide::Junction(JunctionType::All, Vec::new()), 0))
                    }
                }
                "Int" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Int requires parser"));
                    }
                    let p = &args[1];
                    let (res, consumed) = babel_parse(p, input, rule_set)?;
                    if let Nucleotide::String(s) = res {
                        if let Ok(n) = s.parse::<i64>() {
                            return Ok((Nucleotide::Number(n), consumed));
                        }
                    }
                    Err(anyhow!("Int conversion failed"))
                }
                "Map" => {
                    if args.len() < 3 {
                        return Err(anyhow!("Map requires [parser, template]"));
                    }
                    let parser = &args[1];
                    let template = &args[2];
                    let (res, consumed) = babel_parse(parser, input, rule_set)?;
                    let mapped = resolve_template(template, &res);
                    Ok((mapped, consumed))
                }
                "Ref" => {
                    if args.len() < 2 {
                        return Err(anyhow!("Ref requires name"));
                    }
                    if let Nucleotide::String(name) = &args[1] {
                        if let Some(rule) = rule_set.get(name) {
                            // Max recursion depth check? babel_parse is recursive.
                            // We rely on stack overflow protection? Or pass depth?
                            // For safety, maybe we should pass depth.
                            // But here we just recurse.
                            babel_parse(rule, input, rule_set)
                        } else {
                            Err(anyhow!("Unknown rule reference: {}", name))
                        }
                    } else {
                        Err(anyhow!("Invalid Ref name"))
                    }
                }
                _ => Err(anyhow!("Unknown grammar type: {}", type_str)),
            }
        } else {
            Err(anyhow!(
                "Invalid grammar node structure (expected Type String)"
            ))
        }
    } else {
        Err(anyhow!("Invalid grammar node (expected Junction)"))
    }
}

fn flatten_ast(ast: &Nucleotide) -> Result<Vec<Gene>> {
    match ast {
        Nucleotide::Junction(crate::ast::JunctionType::All, children) => {
            let mut genes = Vec::new();
            for child in children {
                genes.extend(flatten_ast(child)?);
            }
            Ok(genes)
        }
        Nucleotide::Junction(crate::ast::JunctionType::Any, children) => {
            if !children.is_empty() {
                if let Nucleotide::String(op_name) = &children[0] {
                    if let Ok(op) = OpCode::from_str(op_name) {
                        let mut args = Vec::new();
                        for child in children.iter().skip(1) {
                            args.push(child.clone());
                        }
                        return Ok(vec![Gene { op, args }]);
                    }
                }
            }
            let mut genes = Vec::new();
            for child in children {
                genes.extend(flatten_ast(child)?);
            }
            Ok(genes)
        }
        Nucleotide::String(s) => {
            if let Ok(op) = OpCode::from_str(s) {
                if let OpCode::Unknown(_) = op {
                    // Not a known opcode, push as string literal
                    // Check if it looks like a number
                    if let Ok(n) = s.parse::<i64>() {
                        Ok(vec![Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(n)],
                        }])
                    } else {
                        Ok(vec![Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::String(s.clone())],
                        }])
                    }
                } else {
                    Ok(vec![Gene { op, args: vec![] }])
                }
            } else {
                // Should not happen with strum default, but safe fallback
                Ok(vec![Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(s.clone())],
                }])
            }
        }
        Nucleotide::Number(n) => Ok(vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(*n)],
        }]),
        _ => Ok(vec![]),
    }
}

fn resolve_template(template: &Nucleotide, match_res: &Nucleotide) -> Nucleotide {
    match template {
        Nucleotide::String(s) => {
            // Split by whitespace to allow list construction e.g. "?1 ?3"
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() > 1 {
                let mut resolved_parts = Vec::new();
                for part in parts {
                    let sub_template = Nucleotide::String(part.to_string());
                    resolved_parts.push(resolve_template(&sub_template, match_res));
                }
                return Nucleotide::Junction(crate::ast::JunctionType::All, resolved_parts);
            }

            if s.starts_with('?') {
                if let Ok(idx) = s[1..].parse::<usize>() {
                    let i = idx.saturating_sub(1);
                    match match_res {
                        Nucleotide::Junction(crate::ast::JunctionType::All, children) => {
                            if i < children.len() {
                                return children[i].clone();
                            }
                        }
                        _ => {
                            if i == 0 {
                                return match_res.clone();
                            }
                        }
                    }
                    // If index out of bounds, ignore? or return empty?
                    // For now return original to be safe/debuggable
                    return template.clone();
                }
            }
            // Literal string
            template.clone()
        }
        Nucleotide::Junction(t, args) => {
            let resolved: Vec<Nucleotide> = args
                .iter()
                .map(|a| resolve_template(a, match_res))
                .collect();
            Nucleotide::Junction(*t, resolved)
        }
        _ => template.clone(),
    }
}

fn levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    if len_a < len_b {
        return levenshtein(b, a);
    }
    let mut cache: Vec<usize> = (0..=len_b).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev_dist = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            let dist = std::cmp::min(
                std::cmp::min(prev_dist + 1, cache[j + 1] + 1),
                cache[j] + cost,
            );
            cache[j] = prev_dist;
            prev_dist = dist;
        }
        cache[len_b] = prev_dist;
    }
    cache[len_b]
}

fn suggest_opcode(name: &str) -> Option<String> {
    let mut best_match = None;
    let mut min_dist = 3; // Max distance allowed

    for op in OpCode::iter() {
        if let OpCode::Unknown(_) = op {
            continue;
        }
        let op_str = op.as_ref();
        let dist = levenshtein(name, op_str);
        if dist < min_dist {
            min_dist = dist;
            best_match = Some(op_str.to_string());
        }
    }
    best_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_basic() {
        let src = r#"
        strand main {
            5
            3
            add
            print
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 1);
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 4);
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[2].op, OpCode::Add);
    }

    #[test]
    fn test_compile_jumps() {
        let src = r#"
        strand main {
            jump(loop)
        }
        strand loop {
            main
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 2);

        // Check jump target
        let jump_gene = &dna.helix.strands[0].genes[0];
        assert_eq!(jump_gene.op, OpCode::Jump);
        // "loop" is the second strand (index 1)
        assert_eq!(jump_gene.args[0], Nucleotide::Number(1));
    }

    #[test]
    fn test_macros() {
        let src = r#"
        macro ADD_PRINT {
            add print
        }
        strand main {
            5 3 ADD_PRINT
        }
        "#;
        let dna = compile(src, None).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 4); // push 5, push 3, add, print
        assert_eq!(genes[2].op, OpCode::Add);
        assert_eq!(genes[3].op, OpCode::Print);
    }

    #[test]
    fn test_sugar() {
        let src = r#"
        strand main {
            -> target
            ? target
        }
        strand target {
            drop
        }
        "#;
        let dna = compile(src, None).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes[0].op, OpCode::Jump);
        assert_eq!(genes[1].op, OpCode::Brz);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_blocks() {
        let src = r#"
        strand main {
            { 1 2 add } call
        }
        "#;
        let dna = compile(src, None).unwrap();
        assert_eq!(dna.helix.strands.len(), 2); // main + block

        let main_genes = &dna.helix.strands[0].genes;
        assert_eq!(main_genes[0].op, OpCode::Push); // Push block index
        assert_eq!(main_genes[0].args[0], Nucleotide::Number(1));
        assert_eq!(main_genes[1].op, OpCode::Call);

        let block_genes = &dna.helix.strands[1].genes;
        assert_eq!(block_genes.len(), 3);
        assert_eq!(block_genes[2].op, OpCode::Add);
    }
}
