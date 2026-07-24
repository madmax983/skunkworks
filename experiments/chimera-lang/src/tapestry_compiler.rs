use anyhow::{anyhow, Result};
use pest::Parser;

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;

#[allow(missing_docs)]
pub mod tapestryparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[allow(missing_docs)]
    #[grammar = "tapestry_grammar.pest"]
    pub struct TapestryParser;
}
pub use tapestryparser_mod::Rule;
pub use tapestryparser_mod::TapestryParser;

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = TapestryParser::parse(Rule::tapestry, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut strands_map: std::collections::HashMap<usize, Vec<Gene>> =
        std::collections::HashMap::new();

    for table in program.into_inner() {
        if table.as_rule() == Rule::EOI {
            break;
        }

        if table.as_rule() != Rule::table {
            continue;
        }

        for row in table.into_inner() {
            if row.as_rule() == Rule::data_row {
                for (col_idx, cell) in row.into_inner().enumerate() {
                    let text = cell.as_str().trim();
                    if text.is_empty() {
                        continue;
                    }

                    let op =
                        match text {
                            "+" => OpCode::Add,
                            "-" => OpCode::Sub,
                            "*" => OpCode::Mul,
                            "p" => OpCode::Print,
                            _ => {
                                if let Ok(num) = text.parse::<i64>() {
                                    strands_map.entry(col_idx).or_default().push(
                                        Gene {
                                            op: OpCode::Push,
                                            args: vec![Nucleotide::Number(num)],
                                        },
                                    );
                                    continue;
                                } else {
                                    strands_map.entry(col_idx).or_default().push(
                                        Gene {
                                            op: OpCode::Push,
                                            args: vec![Nucleotide::String(text.to_string())],
                                        },
                                    );
                                    continue;
                                }
                            }
                        };
                    strands_map
                        .entry(col_idx)
                        .or_default()
                        .push(Gene { op, args: vec![] });
                }
            }
        }
    }

    let mut strands = Vec::new();
    let mut max_col = 0;
    if !strands_map.is_empty() {
        max_col = *strands_map.keys().max().unwrap();
    }
    for i in 0..=max_col {
        if let Some(genes) = strands_map.remove(&i) {
            strands.push(Strand { genes });
        }
    }

    Ok(Dna {
        helix: Helix { strands },
        evolution_config: None,
    })
}
