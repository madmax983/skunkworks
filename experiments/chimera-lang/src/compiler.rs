use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "script_grammar.pest"]
pub struct ScriptParser;

fn preprocess(
    source: &str,
    base_path: Option<&Path>,
    visited: &mut HashSet<PathBuf>,
) -> Result<String> {
    let mut expanded = String::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("include") {
            // Extract filename
            // Format: include "filename"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let raw_filename = parts[1];
                // Strip quotes if present
                let filename = raw_filename.trim_matches('"');

                if let Some(bp) = base_path {
                    let path = bp.join(filename);

                    // Canonicalize to detect cycles accurately (resolves symlinks, .., etc.)
                    // Note: This requires the file to exist.
                    let abs_path = if path.exists() {
                        path.canonicalize()?
                    } else {
                        // If file doesn't exist, read_to_string will fail later with a good message.
                        // But for cycle detection, we use the best path we have.
                        path.clone()
                    };

                    if !visited.insert(abs_path.clone()) {
                        return Err(anyhow!("Recursive include detected: {:?}", abs_path));
                    }

                    let content = fs::read_to_string(&path)
                        .map_err(|e| anyhow!("Failed to include file {:?}: {}", path, e))?;

                    // Recursive preprocess
                    let sub_expanded = preprocess(&content, Some(bp), visited)?;
                    expanded.push_str(&sub_expanded);
                    expanded.push('\n');

                    // Remove from visited set to allow inclusion in sibling branches (diamond problem)
                    // but prevent cycles in the current recursion stack.
                    visited.remove(&abs_path);
                } else {
                    return Err(anyhow!("Cannot include files without a base path"));
                }
            } else {
                return Err(anyhow!("Invalid include statement: {}", trimmed));
            }
        } else {
            expanded.push_str(line);
            expanded.push('\n');
        }
    }
    Ok(expanded)
}

pub fn compile(source: &str, base_path: Option<&Path>) -> Result<Dna> {
    let mut visited = HashSet::new();
    let expanded_source = preprocess(source, base_path, &mut visited)?;

    let mut pairs = ScriptParser::parse(Rule::program, &expanded_source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    // Pass 1: Collect strand names and macros
    let mut strand_map: HashMap<String, usize> = HashMap::new();
    let mut macro_map: HashMap<String, pest::iterators::Pairs<Rule>> = HashMap::new();

    for pair in program.clone().into_inner() {
        match pair.as_rule() {
            Rule::strand_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let idx = strand_map.len();
                if strand_map.insert(name.to_string(), idx).is_some() {
                    return Err(anyhow!("Duplicate strand name: {}", name));
                }
            }
            Rule::macro_def => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                // Store the instructions (rest of inner)
                macro_map.insert(name.to_string(), inner);
            }
            _ => {}
        }
    }

    let mut strands_ast = Vec::new();

    // Pass 2: Generate Genes
    for pair in program.into_inner() {
        if pair.as_rule() == Rule::strand_def {
            let mut inner = pair.into_inner();
            let _name = inner.next().unwrap(); // skip name
            let mut genes = Vec::new();

            for instr in inner {
                let generated = parse_instructions(instr, &strand_map, &macro_map, 0)?;
                genes.extend(generated);
            }
            strands_ast.push(Strand { genes });
        }
    }

    Ok(Dna {
        helix: Helix {
            strands: strands_ast,
        },
    })
}

fn parse_instructions(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
    macro_map: &HashMap<String, pest::iterators::Pairs<Rule>>,
    depth: usize,
) -> Result<Vec<Gene>> {
    if depth > 50 {
        return Err(anyhow!("Macro recursion depth exceeded"));
    }

    // pair is `instruction`
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => {
            let val = parse_literal(inner, strand_map)?;
            Ok(vec![Gene {
                op: OpCode::Push,
                args: vec![val],
            }])
        }
        Rule::simple_op => {
            let name = inner.clone().into_inner().next().unwrap().as_str();
            // Check macro
            if let Some(body) = macro_map.get(name) {
                let mut macro_genes = Vec::new();
                for instr in body.clone() {
                    let sub = parse_instructions(instr, strand_map, macro_map, depth + 1)?;
                    macro_genes.extend(sub);
                }
                return Ok(macro_genes);
            }

            let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
            Ok(vec![Gene { op, args: vec![] }])
        }
        Rule::arrow_jump => {
            // "->" ~ identifier
            let mut parts = inner.into_inner();
            let target_name = parts.next().unwrap().as_str();
            let arg = resolve_target(target_name, strand_map);
            Ok(vec![Gene {
                op: OpCode::Jump,
                args: vec![arg],
            }])
        }
        Rule::question_branch => {
            // "?" ~ identifier
            let mut parts = inner.into_inner();
            let target_name = parts.next().unwrap().as_str();
            let arg = resolve_target(target_name, strand_map);
            Ok(vec![Gene {
                op: OpCode::Brz,
                args: vec![arg],
            }])
        }
        Rule::call => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str();
            let args_pair = parts.next().unwrap();

            let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
            let mut args = Vec::new();

            for arg_pair in args_pair.into_inner() {
                let val = parse_argument(arg_pair, strand_map)?;
                args.push(val);
            }

            Ok(vec![Gene { op, args }])
        }
        _ => unreachable!("Unexpected instruction rule: {:?}", inner.as_rule()),
    }
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
            // Remove quotes
            Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
        }
        _ => unreachable!("Unexpected literal rule"),
    }
}

fn parse_argument(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
) -> Result<Nucleotide> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => parse_literal(inner, strand_map),
        Rule::identifier => {
            let id = inner.as_str();
            // Try to resolve as strand index
            if let Some(&idx) = strand_map.get(id) {
                Ok(Nucleotide::Number(idx as i64))
            } else {
                // Keep as identifier
                Ok(Nucleotide::Identifier(id.to_string()))
            }
        }
        _ => unreachable!("Unexpected argument rule"),
    }
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
}
