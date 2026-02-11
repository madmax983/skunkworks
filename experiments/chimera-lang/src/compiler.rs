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

const MAX_INCLUDE_DEPTH: usize = 32;

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
                    let sub_expanded = preprocess(&content, Some(bp), visited, depth + 1)?;
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
    let expanded_source = preprocess(source, base_path, &mut visited, 0)?;

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
    let mut anonymous_strands = Vec::new();

    // Pass 2: Generate Genes and Collect Rules
    let mut rules_genes = Vec::new();

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
                        &mut anonymous_strands,
                        0,
                    )?;
                    genes.extend(generated);
                }
                strands_ast.push(Strand { genes });
            }
            Rule::rule_def => {
                let generated = parse_rule_def(pair, &strand_map)?;
                rules_genes.extend(generated);
            }
            _ => {}
        }
    }

    // Prepend logic rules to the first strand (or create a main strand if none exist)
    if !rules_genes.is_empty() {
        if strands_ast.is_empty() {
            strands_ast.push(Strand { genes: rules_genes });
        } else {
            // Find the first defined strand and prepend
            // Note: pass 1 collected strand names. The first one in source order is at index 0.
            let first_strand = &mut strands_ast[0];
            // Must insert at beginning
            for gene in rules_genes.into_iter().rev() {
                first_strand.genes.insert(0, gene);
            }
        }
    }

    // Append anonymous strands
    strands_ast.extend(anonymous_strands);

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
    anonymous_strands: &mut Vec<Strand>,
    depth: usize,
) -> Result<Vec<Gene>> {
    if depth > 50 {
        return Err(anyhow!("Macro recursion depth exceeded"));
    }

    // pair is `instruction`
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::block => {
            let mut genes = Vec::new();
            for instr in inner.into_inner() {
                let sub =
                    parse_instructions(instr, strand_map, macro_map, anonymous_strands, depth + 1)?;
                genes.extend(sub);
            }
            // Create new strand
            let index = strand_map.len() + anonymous_strands.len();
            anonymous_strands.push(Strand { genes });

            // Return push(index)
            Ok(vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(index as i64)],
            }])
        }
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
                    let sub = parse_instructions(
                        instr,
                        strand_map,
                        macro_map,
                        anonymous_strands,
                        depth + 1,
                    )?;
                    macro_genes.extend(sub);
                }
                return Ok(macro_genes);
            }

            // Check if it's a strand name (Push index)
            if let Some(&idx) = strand_map.get(name) {
                return Ok(vec![Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(idx as i64)],
                }]);
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

            // Desugar arguments for most opcodes into stack pushes
            // Only intrinsics keep args in the Gene
            let is_intrinsic = match op {
                OpCode::Jump | OpCode::Brz => true,
                #[cfg(feature = "nova")]
                OpCode::Call | OpCode::Poly => true,
                _ => false,
            };

            if op == OpCode::Push {
                // Generate a Push gene for EACH argument (supports multi-push)
                let mut genes = Vec::new();
                for arg in args {
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![arg],
                    });
                }
                Ok(genes)
            } else if is_intrinsic {
                // Intrinsic: Keep as single gene with args
                Ok(vec![Gene { op, args }])
            } else {
                // Non-Intrinsic: Desugar to pushes + op
                // Example: recombine(A, B, C) -> push(A) push(B) push(C) recombine()
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
        Rule::crispr_block => {
            let mut parts = inner.into_inner();
            let target_name = parts.next().unwrap().as_str();

            // pattern_def is next
            let pattern_pair = parts.next().unwrap();
            // pattern_pair rule is pattern_def. inner is identifier list.
            let mut pattern_genes = Vec::new();
            for op_pair in pattern_pair.into_inner() {
                // pattern_def inner contains "pattern", ":" literals which pest might skip if atomic?
                // Wait, pattern_def = { "pattern" ~ ":" ~ identifier ~ ("," ~ identifier)* }
                // The identifiers are the only thing produced if others are silent?
                // But literals are produced if not atomic/silent.
                // Looking at grammar: pattern_def is not atomic (@).
                // "pattern" and ":" are literals.
                // We must filter for identifiers or loop carefully.
                // Actually, pest pairs iterator skips string literals defined in rule usually?
                // No, it depends.
                // Let's assume we iterate and check rule.
                if op_pair.as_rule() == Rule::identifier {
                    let op_str = op_pair.as_str();
                    let op = OpCode::from_str(op_str)
                        .map_err(|_| anyhow!("Unknown opcode in pattern: {}", op_str))?;
                    pattern_genes.push(Gene { op, args: vec![] });
                }
            }

            // Register Guide Strand
            let guide_idx = strand_map.len() + anonymous_strands.len();
            anonymous_strands.push(Strand {
                genes: pattern_genes,
            });

            let mut genes = Vec::new();

            // Push Target Index
            let target_arg = resolve_target(target_name, strand_map);
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![target_arg],
            });

            // Push Guide Index
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(guide_idx as i64)],
            });

            // CrisprScan
            genes.push(Gene {
                op: OpCode::CrisprScan,
                args: vec![],
            });

            // Process body instructions
            for instr in parts {
                let sub = parse_instructions(
                    instr,
                    strand_map,
                    macro_map,
                    anonymous_strands,
                    depth + 1,
                )?;
                genes.extend(sub);
            }

            Ok(genes)
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
        Rule::term => parse_term(inner, strand_map),
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
        Rule::variable => {
            // Variables like ?X are treated as strings
            let var_name = inner.as_str();
            Ok(Nucleotide::String(var_name.to_string()))
        }
        Rule::junction => {
            let mut parts = inner.into_inner();
            let type_str = parts.next().unwrap().as_str();
            let args_pair = parts.next().unwrap();

            let t = match type_str {
                "any" => crate::ast::JunctionType::Any,
                "all" => crate::ast::JunctionType::All,
                _ => return Err(anyhow!("Invalid junction type")),
            };

            let mut vals = Vec::new();
            for arg in args_pair.into_inner() {
                vals.push(parse_argument(arg, strand_map)?);
            }
            Ok(Nucleotide::Junction(t, vals))
        }
        _ => unreachable!("Unexpected argument rule: {:?}", inner.as_rule()),
    }
}

fn parse_term(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
) -> Result<Nucleotide> {
    let mut parts = pair.into_inner();
    let name = parts.next().unwrap().as_str();
    let mut args = vec![Nucleotide::String(name.to_string())];

    if let Some(arg_list) = parts.next() {
        for arg_pair in arg_list.into_inner() {
            args.push(parse_argument(arg_pair, strand_map)?);
        }
    }

    Ok(Nucleotide::Junction(crate::ast::JunctionType::Any, args))
}

fn parse_rule_def(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
) -> Result<Vec<Gene>> {
    let mut parts = pair.into_inner();
    let head_pair = parts.next().unwrap();
    let body_list_pair = parts.next().unwrap();

    let head_nuc = parse_term(head_pair, strand_map)?;

    let mut body_terms = Vec::new();
    for term_pair in body_list_pair.into_inner() {
        body_terms.push(parse_term(term_pair, strand_map)?);
    }
    let body_nuc = Nucleotide::Junction(crate::ast::JunctionType::All, body_terms);

    Ok(vec![
        Gene {
            op: OpCode::Push,
            args: vec![head_nuc],
        },
        Gene {
            op: OpCode::Push,
            args: vec![body_nuc],
        },
        Gene {
            op: OpCode::Rule,
            args: vec![],
        },
    ])
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
