use anyhow::{anyhow, Result};
use pest::Parser;

use crate::ast::{Dna, Gene, JunctionType, Nucleotide};
use crate::opcode::OpCode;
use std::str::FromStr;

pub(crate) mod helixparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "helix_grammar.pest"]
    pub struct HelixParser;
}
pub use helixparser_mod::HelixParser;
pub use helixparser_mod::Rule;

fn compile_instruction(
    instruction_pair: pest::iterators::Pair<Rule>,
    genes: &mut Vec<Gene>,
) -> Result<()> {
    match instruction_pair.as_rule() {
        Rule::prolog_instruction => {
            let mut p_inner = instruction_pair.into_inner();
            let name = p_inner.next().unwrap().as_str();
            let arg = p_inner.next().unwrap().as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String(name.to_string()),
                        Nucleotide::String(arg.to_string()),
                    ],
                )],
            ));
            genes.push(Gene::new(OpCode::PrologCall, vec![]));
        }
        Rule::hyper_instruction => {
            let mut h_inner = instruction_pair.into_inner();
            let op_str = h_inner.next().unwrap().as_str();
            match op_str {
                "+" | "add" => genes.push(Gene::new(OpCode::HyperAdd, vec![])),
                "-" | "sub" => genes.push(Gene::new(OpCode::HyperSub, vec![])),
                "*" | "mul" => genes.push(Gene::new(OpCode::HyperMul, vec![])),
                "/" | "div" => genes.push(Gene::new(OpCode::HyperDiv, vec![])),
                _ => {
                    genes.push(Gene::new(
                        OpCode::Push,
                        vec![Nucleotide::String(op_str.to_string())],
                    ));
                    genes.push(Gene::new(OpCode::Unknown(op_str.to_string()), vec![]));
                }
            }
        }
        Rule::orca_instruction => {
            let mut o_inner = instruction_pair.into_inner();
            let op = o_inner.next().unwrap().as_str();
            let y: i64 = o_inner.next().unwrap().as_str().parse()?;
            let x: i64 = o_inner.next().unwrap().as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
            if op == "bang" {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String("bang".to_string())],
                ));
                genes.push(Gene::new(OpCode::Orca, vec![]));
            } else if op == "jumper" {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String("jumper".to_string())],
                ));
                genes.push(Gene::new(OpCode::Orca, vec![]));
            } else if op == "warp" {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String("warp".to_string())],
                ));
                genes.push(Gene::new(OpCode::Orca, vec![]));
            } else {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(op.to_string())],
                ));
                genes.push(Gene::new(OpCode::Orca, vec![]));
            }
        }
        Rule::elektra_instruction => {
            let mut e_inner = instruction_pair.into_inner();
            let op = e_inner.next().unwrap().as_str();
            let y: i64 = e_inner.next().unwrap().as_str().parse()?;
            let x: i64 = e_inner.next().unwrap().as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(op.to_string())],
            ));
            genes.push(Gene::new(OpCode::Battery, vec![]));
        }
        Rule::genetics_instruction => {
            let mut g_inner = instruction_pair.into_inner();
            let op = g_inner.next().unwrap().as_str();
            let ident1 = g_inner.next().unwrap().as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(ident1.to_string())],
            ));
            if let Some(ident2) = g_inner.next() {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(ident2.as_str().to_string())],
                ));
            }

            if op == "splice" {
                genes.push(Gene::new(OpCode::Splice, vec![]));
            } else if op == "recombine" {
                genes.push(Gene::new(OpCode::Recombine, vec![]));
            } else if op == "mutate" {
                genes.push(Gene::new(OpCode::Mutagen, vec![]));
            } else if op == "cross" {
                genes.push(Gene::new(OpCode::Cross, vec![]));
            } else {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(op.to_string())],
                ));
            }
        }
        Rule::tui_instruction => {
            let mut t_inner = instruction_pair.into_inner();
            let op = t_inner.next().unwrap().as_str();
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(op.to_string())],
            ));
            genes.push(Gene::new(OpCode::TuiDraw, vec![]));
        }
        Rule::forth_instruction => {
            let f_inner = instruction_pair.into_inner().next().unwrap();
            match f_inner.as_rule() {
                Rule::identifier => {
                    let id = f_inner.as_str().to_ascii_lowercase();
                    if let Ok(opcode) = OpCode::from_str(&id) {
                        genes.push(Gene::new(opcode, vec![]));
                    } else {
                        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(id)]));
                    }
                }
                Rule::number => {
                    let n: i64 = f_inner.as_str().parse()?;
                    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
                }
                Rule::string => {
                    let s = f_inner.as_str();
                    genes.push(Gene::new(
                        OpCode::Push,
                        vec![Nucleotide::String(s[1..s.len() - 1].to_string())],
                    ));
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = HelixParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut strand_a_genes = Vec::new();
    let mut strand_b_genes = Vec::new();

    for line_pair in program.into_inner() {
        if line_pair.as_rule() == Rule::EOI {
            break;
        }

        if line_pair.as_rule() == Rule::line {
            let mut line_inner = line_pair.into_inner();

            let strand_a_pair = line_inner.next().unwrap();
            for instr in strand_a_pair.into_inner() {
                let inner_instr = instr.into_inner().next().unwrap();
                compile_instruction(inner_instr, &mut strand_a_genes)?;
            }

            let strand_b_pair = line_inner.next().unwrap();
            for instr in strand_b_pair.into_inner() {
                let inner_instr = instr.into_inner().next().unwrap();
                compile_instruction(inner_instr, &mut strand_b_genes)?;
            }
        }
    }

    Ok(Dna {
        evolution_config: None,
        helix: crate::ast::Helix {
            strands: vec![
                crate::ast::Strand {
                    genes: strand_a_genes,
                },
                crate::ast::Strand {
                    genes: strand_b_genes,
                },
            ],
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helix_compiler() {
        let source = r#"
            10 20 add | "hello" print
            >>+<< | bang 8 8
            battery 2 2 | fact(x).
            splice DNA1 DNA2 | draw
        "#;
        let dna = compile(source).unwrap();

        assert_eq!(dna.helix.strands.len(), 2);

        let a_genes = &dna.helix.strands[0].genes;
        println!("a_genes: {:#?}", a_genes);
        let b_genes = &dna.helix.strands[1].genes;

        // 10 20 add
        assert_eq!(a_genes[0].op, OpCode::Push);
        assert_eq!(a_genes[1].op, OpCode::Push);
        assert_eq!(a_genes[2].op, OpCode::Add);

        // "hello" print
        assert_eq!(b_genes[0].op, OpCode::Push);
        assert_eq!(b_genes[1].op, OpCode::Print);

        // >>+<<
        assert_eq!(a_genes[3].op, OpCode::HyperAdd);

        // bang 8 8
        assert_eq!(b_genes[2].op, OpCode::Push);
        assert_eq!(b_genes[3].op, OpCode::Push);
        assert_eq!(b_genes[4].op, OpCode::Push);
        assert_eq!(b_genes[5].op, OpCode::Orca);

        // battery 2 2
        assert_eq!(a_genes[4].op, OpCode::Push);
        assert_eq!(a_genes[5].op, OpCode::Push);
        assert_eq!(a_genes[6].op, OpCode::Push); // "battery"
        assert_eq!(a_genes[7].op, OpCode::Battery);

        // fact(x).
        assert_eq!(b_genes[6].op, OpCode::Push);
        assert_eq!(b_genes[7].op, OpCode::PrologCall);

        // splice DNA1 DNA2
        assert_eq!(a_genes[8].op, OpCode::Push);
        assert_eq!(a_genes[9].op, OpCode::Push);
        assert_eq!(a_genes[10].op, OpCode::Splice);

        // draw
        assert_eq!(b_genes[8].op, OpCode::Push); // "draw"
        assert_eq!(b_genes[9].op, OpCode::TuiDraw);
    }
}
