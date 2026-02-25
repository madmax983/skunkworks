#[cfg(feature = "cistron")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "cistron")]
use crate::opcode::OpCode;
#[cfg(feature = "cistron")]
use anyhow::Result;
#[cfg(feature = "cistron")]
use pest::Parser;
#[cfg(feature = "cistron")]
use pest_derive::Parser;

#[cfg(feature = "cistron")]
#[derive(Parser)]
#[grammar = "cistron.pest"]
pub struct CistronParser;

#[cfg(feature = "cistron")]
pub fn compile(source: &str) -> Result<Dna> {
    let pairs = CistronParser::parse(Rule::program, source)?;
    let mut genes = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::stmt => {
                    let actual = inner.into_inner().next().unwrap();
                    match actual.as_rule() {
                        Rule::protein_def => {
                            let mut inner_rules = actual.into_inner();
                            let name = inner_rules.next().unwrap().as_str();
                            let body = inner_rules.next().unwrap();
                            let mut decay = 5; // Default 0.05

                            for stmt in body.into_inner() {
                                match stmt.as_rule() {
                                    Rule::decay_def => {
                                        let val_str = stmt.into_inner().as_str();
                                        if let Ok(f) = val_str.parse::<f64>() {
                                            decay = (f * 100.0) as i64;
                                        }
                                    }
                                    _ => {}
                                }
                            }

                            genes.push(Gene {
                                op: OpCode::SynthesizeProtein,
                                args: vec![
                                    Nucleotide::String(name.to_string()),
                                    Nucleotide::Number(50),
                                    Nucleotide::Number(decay)
                                ]
                            });
                        }
                        Rule::gene_def => {
                            let mut inner_rules = actual.into_inner();
                            let gene_name = inner_rules.next().unwrap().as_str();
                            let body = inner_rules.next().unwrap();

                            for stmt in body.into_inner() {
                                match stmt.as_rule() {
                                    Rule::promote_def => {
                                        let mut promote_args = stmt.into_inner();
                                        let tf_name = promote_args.next().unwrap().as_str();
                                        let mut weight = 100;
                                        if let Some(w) = promote_args.next() {
                                            let f: f64 = w.as_str().parse()?;
                                            weight = (f * 100.0) as i64;
                                        }

                                        genes.push(Gene {
                                            op: OpCode::Regulate,
                                            args: vec![
                                                Nucleotide::String(gene_name.to_string()),
                                                Nucleotide::String(tf_name.to_string()),
                                                Nucleotide::Number(0), // Promote
                                                Nucleotide::Number(weight)
                                            ]
                                        });
                                    }
                                    Rule::repress_def => {
                                        let mut repress_args = stmt.into_inner();
                                        let tf_name = repress_args.next().unwrap().as_str();
                                        let mut weight = 100;
                                        if let Some(w) = repress_args.next() {
                                            let f: f64 = w.as_str().parse()?;
                                            weight = (f * 100.0) as i64;
                                        }

                                        genes.push(Gene {
                                            op: OpCode::Regulate,
                                            args: vec![
                                                Nucleotide::String(gene_name.to_string()),
                                                Nucleotide::String(tf_name.to_string()),
                                                Nucleotide::Number(1), // Repress
                                                Nucleotide::Number(weight)
                                            ]
                                        });
                                    }
                                    _ => {}
                                }
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
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    })
}
