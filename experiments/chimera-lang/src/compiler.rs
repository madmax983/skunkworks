use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "script_grammar.pest"]
pub struct ScriptParser;

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = ScriptParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    // Pass 1: Collect strand names
    let mut strand_map: HashMap<String, usize> = HashMap::new();
    let mut strands_ast = Vec::new();

    // Iterate inner rules (strands)
    for pair in program.clone().into_inner() {
        if pair.as_rule() == Rule::strand_def {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str(); // identifier
            let idx = strand_map.len();
            if strand_map.insert(name.to_string(), idx).is_some() {
                return Err(anyhow!("Duplicate strand name: {}", name));
            }
        }
    }

    // Pass 2: Generate Genes
    for pair in program.into_inner() {
        if pair.as_rule() == Rule::strand_def {
            let mut inner = pair.into_inner();
            let _name = inner.next().unwrap(); // skip name (already processed)
            let mut genes = Vec::new();

            for instr in inner {
                // instruction*
                let gene = parse_instruction(instr, &strand_map)?;
                genes.push(gene);
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

fn parse_instruction(
    pair: pest::iterators::Pair<Rule>,
    strand_map: &HashMap<String, usize>,
) -> Result<Gene> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::literal => {
            // Implicit push: "5" -> push(5)
            let val = parse_literal(inner, strand_map)?;
            Ok(Gene {
                op: OpCode::Push,
                args: vec![val],
            })
        }
        Rule::simple_op => {
            let name = inner.into_inner().next().unwrap().as_str();
            let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
            Ok(Gene { op, args: vec![] })
        }
        Rule::call => {
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str();
            let args_pair = parts.next().unwrap(); // argument_list

            let op = OpCode::from_str(name).map_err(|_| anyhow!("Unknown opcode: {}", name))?;
            let mut args = Vec::new();

            for arg_pair in args_pair.into_inner() {
                let val = parse_argument(arg_pair, strand_map)?;
                args.push(val);
            }

            Ok(Gene { op, args })
        }
        _ => unreachable!("Unexpected instruction rule: {:?}", inner.as_rule()),
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
        let dna = compile(src).unwrap();
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
        let dna = compile(src).unwrap();
        assert_eq!(dna.helix.strands.len(), 2);

        // Check jump target
        let jump_gene = &dna.helix.strands[0].genes[0];
        assert_eq!(jump_gene.op, OpCode::Jump);
        // "loop" is the second strand (index 1)
        assert_eq!(jump_gene.args[0], Nucleotide::Number(1));
    }
}
