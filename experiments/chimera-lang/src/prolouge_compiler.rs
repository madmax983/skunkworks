use anyhow::{anyhow, Result};
use pest::Parser;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use std::str::FromStr;

pub mod prolougeparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "prolouge_grammar.pest"]
    pub struct ProlougeParser;
}
pub use prolougeparser_mod::ProlougeParser;
pub use prolougeparser_mod::Rule;

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = ProlougeParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut genes = Vec::new();

    for instruction_pair in program.into_inner() {
        if instruction_pair.as_rule() == Rule::EOI {
            break;
        }

        let inner = instruction_pair
            .into_inner()
            .next()
            .ok_or_else(|| anyhow!("Instruction missing inner rule"))?;
        match inner.as_rule() {
            Rule::prolog_instruction => {
                let mut p_inner = inner.into_inner();
                let name = p_inner
                    .next()
                    .ok_or_else(|| anyhow!("Prolog instruction missing name"))?
                    .as_str();
                let arg = p_inner
                    .next()
                    .ok_or_else(|| anyhow!("Prolog instruction missing argument"))?
                    .as_str();
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
                let mut h_inner = inner.into_inner();
                let op_str = h_inner
                    .next()
                    .ok_or_else(|| anyhow!("Hyper instruction missing operator"))?
                    .as_str();
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
                let mut o_inner = inner.into_inner();
                let op = o_inner
                    .next()
                    .ok_or_else(|| anyhow!("Orca instruction missing operator"))?
                    .as_str();
                let y: i64 = o_inner
                    .next()
                    .ok_or_else(|| anyhow!("Orca instruction missing Y argument"))?
                    .as_str()
                    .parse()?;
                let x: i64 = o_inner
                    .next()
                    .ok_or_else(|| anyhow!("Orca instruction missing X argument"))?
                    .as_str()
                    .parse()?;
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
                let mut e_inner = inner.into_inner();
                let op = e_inner
                    .next()
                    .ok_or_else(|| anyhow!("Elektra instruction missing operator"))?
                    .as_str();
                let y: i64 = e_inner
                    .next()
                    .ok_or_else(|| anyhow!("Elektra instruction missing Y argument"))?
                    .as_str()
                    .parse()?;
                let x: i64 = e_inner
                    .next()
                    .ok_or_else(|| anyhow!("Elektra instruction missing X argument"))?
                    .as_str()
                    .parse()?;
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(op.to_string())],
                ));
                genes.push(Gene::new(OpCode::Battery, vec![]));
            }
            Rule::genetics_instruction => {
                let mut g_inner = inner.into_inner();
                let op = g_inner
                    .next()
                    .ok_or_else(|| anyhow!("Genetics instruction missing operator"))?
                    .as_str();
                let ident1 = g_inner
                    .next()
                    .ok_or_else(|| anyhow!("Genetics instruction missing identifier"))?
                    .as_str();
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
                let mut t_inner = inner.into_inner();
                let op = t_inner
                    .next()
                    .ok_or_else(|| anyhow!("TUI instruction missing operator"))?
                    .as_str();
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(op.to_string())],
                ));
                genes.push(Gene::new(OpCode::TuiDraw, vec![]));
            }
            Rule::forth_instruction => {
                let f_inner = inner
                    .into_inner()
                    .next()
                    .ok_or_else(|| anyhow!("Forth instruction missing inner rule"))?;
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
    }

    Ok(Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prolouge_compiler() {
        let source = r#"
            10 20 add
            "hello" print
            >>+<<
            bang 8 8
            battery 2 2
            fact(x).
            splice DNA1 DNA2
            draw
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        // 10 20 add
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[2].op, OpCode::Add);

        // "hello" print
        assert_eq!(genes[3].op, OpCode::Push);
        assert_eq!(genes[4].op, OpCode::Print);

        // >>+<<
        assert_eq!(genes[5].op, OpCode::HyperAdd);

        // bang 8 8
        assert_eq!(genes[6].op, OpCode::Push);
        assert_eq!(genes[7].op, OpCode::Push);
        assert_eq!(genes[8].op, OpCode::Push);
        assert_eq!(genes[9].op, OpCode::Orca);

        // battery 2 2
        assert_eq!(genes[11].op, OpCode::Push);
        assert_eq!(genes[11].op, OpCode::Push);
        assert_eq!(genes[12].op, OpCode::Push); // "battery"
        assert_eq!(genes[13].op, OpCode::Battery);

        // fact(x).
        assert_eq!(genes[14].op, OpCode::Push);
        assert_eq!(genes[15].op, OpCode::PrologCall);

        // splice DNA1 DNA2
        assert_eq!(genes[17].op, OpCode::Push);
        assert_eq!(genes[17].op, OpCode::Push);
        assert_eq!(genes[18].op, OpCode::Splice);

        // draw
        assert_eq!(genes[19].op, OpCode::Push); // "draw"
        assert_eq!(genes[20].op, OpCode::TuiDraw);
    }
}
