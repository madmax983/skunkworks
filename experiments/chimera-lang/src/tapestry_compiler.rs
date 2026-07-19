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
pub use tapestryparser_mod::TapestryParser;
pub use tapestryparser_mod::Rule;

pub fn compile_tapestry(source: &str) -> Result<Dna> {
    let mut parsed = TapestryParser::parse(Rule::program, source)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    let program = parsed.next().ok_or_else(|| anyhow!("Empty program"))?;

    let mut columns: Vec<Strand> = Vec::new();
    let mut num_cols = 0;

    // First pass to determine number of columns from the header
    for record in program.clone().into_inner() {
        if record.as_rule() == Rule::header_row {
             for _cell in record.into_inner() {
                 columns.push(Strand { genes: Vec::new() });
             }
             num_cols = columns.len();
             break;
        }
    }

    // Second pass to parse data
    for record in program.into_inner() {
         if record.as_rule() == Rule::data_row {
             let mut i = 0;
             for cell in record.into_inner() {
                  if cell.as_rule() == Rule::instruction_cell {
                       if let Some(instruction) = cell.into_inner().next() {
                            if let Ok(gene) = compile_instruction(instruction) {
                                 if i < num_cols {
                                      columns[i].genes.push(gene);
                                 }
                            }
                       }
                       i += 1;
                  }
             }
         }
    }

    let helix = Helix {
        strands: columns,
    };

    let dna = Dna {
        helix,
        evolution_config: None,
    };

    Ok(dna)
}

fn compile_instruction(instruction: pest::iterators::Pair<Rule>) -> Result<Gene> {
    let inner = instruction.into_inner().next().ok_or_else(|| anyhow!("Empty instruction"))?;
    match inner.as_rule() {
        Rule::forth_instruction => {
            let val = inner.into_inner().next().unwrap();
            match val.as_rule() {
                Rule::number => {
                    let num = val.as_str().parse::<i64>().unwrap();
                    Ok(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(num)],
                    })
                }
                Rule::string => {
                    let s = val.as_str().trim_matches('"').to_string();
                    Ok(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::String(s)],
                    })
                }
                Rule::identifier => {
                    let s = val.as_str().to_string();
                    match s.as_str() {
                        "print" => Ok(Gene { op: OpCode::Print, args: vec![] }),
                        "add" => Ok(Gene { op: OpCode::Add, args: vec![] }),
                        _ => Ok(Gene { op: OpCode::Push, args: vec![Nucleotide::String(s)] }),
                    }
                }
                _ => Err(anyhow!("Unknown forth value")),
            }
        },
        Rule::orca_instruction => {
            let mut inner_orca = inner.into_inner();
            let op_str = inner_orca.next().unwrap().as_str();
            let n1 = inner_orca.next().unwrap().as_str().parse::<i64>().unwrap();
            let n2 = inner_orca.next().unwrap().as_str().parse::<i64>().unwrap();

            let opcode = match op_str {
                "bang" => OpCode::Emit,
                "jumper" => OpCode::Jump,
                "warp" => OpCode::TimeWarp,
                _ => return Err(anyhow!("Unknown orca opcode")),
            };

            Ok(Gene {
                op: opcode,
                args: vec![Nucleotide::Number(n1), Nucleotide::Number(n2)]
            })
        },
        Rule::elektra_instruction => {
            let mut inner_el = inner.into_inner();
            let op_str = inner_el.next().unwrap().as_str();
            let n1 = inner_el.next().unwrap().as_str().parse::<i64>().unwrap();
            let n2 = inner_el.next().unwrap().as_str().parse::<i64>().unwrap();

            let opcode = match op_str {
                "battery" => OpCode::Battery,
                "capacitor" => OpCode::Capacitor,
                "resistor" => OpCode::Wire,
                _ => return Err(anyhow!("Unknown elektra opcode")),
            };

            Ok(Gene {
                op: opcode,
                args: vec![Nucleotide::Number(n1), Nucleotide::Number(n2)]
            })
        },
        Rule::genetics_instruction => {
            let mut inner_gen = inner.into_inner();
            let op_str = inner_gen.next().unwrap().as_str();
            let arg1 = inner_gen.next().unwrap().as_str().to_string();

            let opcode = match op_str {
                "splice" => OpCode::Splice,
                "recombine" => OpCode::Recombine,
                "mutate" => OpCode::EvoMutate,
                "cross" => OpCode::Cross,
                _ => return Err(anyhow!("Unknown genetics opcode")),
            };

            Ok(Gene {
                op: opcode,
                args: vec![Nucleotide::String(arg1)]
            })
        },
        Rule::prolog_instruction => {
            let mut inner_prolog = inner.into_inner();
            let id1 = inner_prolog.next().unwrap().as_str().to_string();
            let id2 = inner_prolog.next().unwrap().as_str().to_string();
            Ok(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(format!("{}({})", id1, id2))]
            })
        },
        Rule::tui_instruction => {
            let inner_tui = inner.into_inner().next().unwrap().as_str();
            let op = match inner_tui {
                "draw" => OpCode::TuiDraw,
                "tui_draw" => OpCode::TuiDraw,
                "render" => OpCode::TuiDraw,
                _ => return Err(anyhow!("Unknown tui opcode")),
            };
            Ok(Gene { op, args: vec![] })
        },
        Rule::hyper_instruction => {
            let inner_hyper = inner.into_inner().next().unwrap().as_str();
            Ok(Gene {
                op: OpCode::Map,
                args: vec![Nucleotide::String(inner_hyper.to_string())]
            })
        },
        _ => Err(anyhow!("Unsupported instruction type in Tapestry compiler stub"))
    }
}
