#![cfg(feature = "resonance")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "acoustic.pest"]
pub struct AcousticParser;

pub fn compile(source: &str) -> Result<Dna> {
    // Parse the source
    // Note: Rule comes from the derived Parser which uses the grammar file
    let pairs = AcousticParser::parse(Rule::score, source)?;
    let mut genes = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::stmt => {
                    let cmd = inner.into_inner().next().unwrap();
                    match cmd.as_rule() {
                        Rule::pluck => {
                            let mut args = cmd.into_inner();
                            let strength_str = args.next().unwrap().as_str();
                            let strength: f64 = strength_str.parse()?;
                            let s_int = (strength * 100.0) as i64;
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(s_int)],
                            });
                            genes.push(Gene {
                                op: OpCode::Pluck,
                                args: vec![],
                            });
                        }
                        Rule::oscillate => {
                            let mut args = cmd.into_inner();
                            let freq: f64 = args.next().unwrap().as_str().parse()?;
                            let strength: f64 = args.next().unwrap().as_str().parse()?;
                            let f_int = freq as i64;
                            let s_int = (strength * 100.0) as i64;
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(f_int)],
                            });
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(s_int)],
                            });
                            genes.push(Gene {
                                op: OpCode::Oscillate,
                                args: vec![],
                            });
                        }
                        Rule::move_cmd => {
                            let mut args = cmd.into_inner();
                            let dy: i64 = args.next().unwrap().as_str().parse()?;
                            let dx: i64 = args.next().unwrap().as_str().parse()?;
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(dy)],
                            });
                            genes.push(Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(dx)],
                            });
                            genes.push(Gene {
                                op: OpCode::Migrate,
                                args: vec![],
                            });
                        }
                        Rule::wait => {
                            let mut args = cmd.into_inner();
                            let ticks: i64 = args.next().unwrap().as_str().parse()?;
                            for _ in 0..ticks {
                                genes.push(Gene {
                                    op: OpCode::Nop,
                                    args: vec![],
                                });
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    })
}
