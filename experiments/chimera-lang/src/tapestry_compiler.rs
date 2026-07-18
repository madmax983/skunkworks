use anyhow::{anyhow, Result};
use pest::Parser;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use std::str::FromStr;

pub mod tapestryparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "tapestry_grammar.pest"]
    pub struct TapestryParser;
}
pub use tapestryparser_mod::TapestryParser;
pub use tapestryparser_mod::Rule;

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
    let mut pairs = TapestryParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut strands_genes: Vec<Vec<Gene>> = Vec::new();

    for table_row_pair in program.into_inner() {
        if table_row_pair.as_rule() == Rule::EOI {
            break;
        }

        if table_row_pair.as_rule() == Rule::table_row {
            let mut col_index = 0;
            for cell_pair in table_row_pair.into_inner() {
                if cell_pair.as_rule() == Rule::cell {
                    if col_index >= strands_genes.len() {
                        strands_genes.push(Vec::new());
                    }
                    for instr in cell_pair.into_inner() {
                        let inner_instr = instr.into_inner().next().unwrap();
                        compile_instruction(inner_instr, &mut strands_genes[col_index])?;
                    }
                    col_index += 1;
                }
            }
        }
    }

    let strands = strands_genes
        .into_iter()
        .map(|genes| Strand { genes })
        .collect();

    Ok(Dna {
        evolution_config: None,
        helix: Helix { strands },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tapestry_compiler() {
        let source = r#"
| 10 20 add | "hello" print | >>+<< |
| bang 8 8 | battery 2 2 | fact(x). |
| splice DNA1 DNA2 | draw | |
        "#;
        let dna = compile(source).unwrap();

        assert_eq!(dna.helix.strands.len(), 3);

        let col0_genes = &dna.helix.strands[0].genes;
        let col1_genes = &dna.helix.strands[1].genes;
        let col2_genes = &dna.helix.strands[2].genes;

        // Col 0
        // 10 20 add
        assert_eq!(col0_genes[0].op, OpCode::Push);
        assert_eq!(col0_genes[1].op, OpCode::Push);
        assert_eq!(col0_genes[2].op, OpCode::Add);

        // bang 8 8
        assert_eq!(col0_genes[3].op, OpCode::Push); // 8
        assert_eq!(col0_genes[4].op, OpCode::Push); // 8
        assert_eq!(col0_genes[5].op, OpCode::Push); // "bang"
        assert_eq!(col0_genes[6].op, OpCode::Orca);

        // splice DNA1 DNA2
        assert_eq!(col0_genes[7].op, OpCode::Push); // "DNA1"
        assert_eq!(col0_genes[8].op, OpCode::Push); // "DNA2"
        assert_eq!(col0_genes[9].op, OpCode::Splice);


        // Col 1
        // "hello" print
        assert_eq!(col1_genes[0].op, OpCode::Push); // "hello"
        assert_eq!(col1_genes[1].op, OpCode::Print);

        // battery 2 2
        assert_eq!(col1_genes[2].op, OpCode::Push); // 2
        assert_eq!(col1_genes[3].op, OpCode::Push); // 2
        assert_eq!(col1_genes[4].op, OpCode::Push); // "battery"
        assert_eq!(col1_genes[5].op, OpCode::Battery);

        // draw
        assert_eq!(col1_genes[6].op, OpCode::Push); // "draw"
        assert_eq!(col1_genes[7].op, OpCode::TuiDraw);

        // Col 2
        // >>+<<
        assert_eq!(col2_genes[0].op, OpCode::HyperAdd);

        // fact(x).
        assert_eq!(col2_genes[1].op, OpCode::Push); // x
        assert_eq!(col2_genes[2].op, OpCode::PrologCall);
    }
}