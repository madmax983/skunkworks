use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use std::str::FromStr;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;

#[derive(Parser)]
#[grammar = "prolouge_grammar.pest"]
pub struct ProlougeParser;

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = ProlougeParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut genes = Vec::new();

    for section in program.into_inner() {
        if section.as_rule() == Rule::EOI {
            break;
        }

        if section.as_rule() != Rule::section {
            continue;
        }

        let inner_block = section.into_inner().next().unwrap();
        match inner_block.as_rule() {
            Rule::forth_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_forth_instr(instr)?);
                }
            }
            Rule::raku_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_raku_instr(instr)?);
                }
            }
            Rule::orca_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_orca_instr(instr)?);
                }
            }
            Rule::elektra_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_elektra_instr(instr)?);
                }
            }
            Rule::prolog_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_prolog_instr(instr)?);
                }
            }
            Rule::genetics_block => {
                for instr in inner_block.into_inner() {
                    genes.extend(compile_genetics_instr(instr)?);
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

fn compile_forth_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => {
            let n: i64 = inner.as_str().parse()?;
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
        }
        Rule::string => {
            let s = inner.as_str();
            let content = s[1..s.len() - 1].to_string();
            genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
        }
        Rule::identifier => {
            let id = inner.as_str();
            if id.eq_ignore_ascii_case("print") {
                genes.push(Gene::new(OpCode::Print, vec![]));
            } else if id.eq_ignore_ascii_case("add") {
                genes.push(Gene::new(OpCode::Add, vec![]));
            } else if id.eq_ignore_ascii_case("sub") {
                genes.push(Gene::new(OpCode::Sub, vec![]));
            } else if let Ok(op) = OpCode::from_str(id) {
                genes.push(Gene::new(op, vec![]));
            } else {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(id.to_string())],
                ));
            }
        }
        _ => {}
    }
    Ok(genes)
}



fn compile_raku_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    // hyper_instruction = { (">>" ~ operator ~ "<<") | string | number | identifier }
    let s = pair.as_str();
    if s.starts_with(">>") && s.ends_with("<<") {
        // Find inner
        let mut inner = pair.into_inner();
        let id = inner.next().unwrap().as_str();
        match id {
            "+" | "add" => genes.push(Gene::new(OpCode::HyperAdd, vec![])),
            "-" | "sub" => genes.push(Gene::new(OpCode::HyperSub, vec![])),
            "*" | "mul" => genes.push(Gene::new(OpCode::HyperMul, vec![])),
            "/" | "div" => genes.push(Gene::new(OpCode::HyperDiv, vec![])),
            _ => {
                // If it's a general operator
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(id.to_string())],
                ));
                genes.push(Gene::new(OpCode::ZipWith, vec![]));
            }
        }
    } else {
        // Just normal instruction parse
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::number => {
                let n: i64 = inner.as_str().parse()?;
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(n)]));
            }
            Rule::string => {
                let string_val = inner.as_str();
                let content = string_val[1..string_val.len() - 1].to_string();
                genes.push(Gene::new(OpCode::Push, vec![Nucleotide::String(content)]));
            }
            Rule::identifier => {
                genes.push(Gene::new(
                    OpCode::Push,
                    vec![Nucleotide::String(inner.as_str().to_string())],
                ));
            }
            _ => {}
        }
    }
    Ok(genes)
}



fn compile_orca_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner.next().unwrap().as_str();
    let y: i64 = inner.next().unwrap().as_str().parse()?;
    let x: i64 = inner.next().unwrap().as_str().parse()?;

    // Push args and call Orca or related
    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(op.to_string())],
    ));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
    genes.push(Gene::new(OpCode::Orca, vec![]));
    Ok(genes)
}

fn compile_elektra_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let kind = inner.next().unwrap().as_str();
    let y: i64 = inner.next().unwrap().as_str().parse()?;
    let x: i64 = inner.next().unwrap().as_str().parse()?;

    if kind == "battery" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(9)])); // 9V default
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        genes.push(Gene::new(OpCode::Battery, vec![]));
    } else if kind == "ground" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        genes.push(Gene::new(OpCode::Ground, vec![]));
    } else {
        // generic
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        if let Ok(op) = OpCode::from_str(kind) {
            genes.push(Gene::new(op, vec![]));
        } else {
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(kind.to_string())],
            ));
        }
    }
    Ok(genes)
}

fn compile_prolog_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let arg = inner.next().unwrap().as_str();

    let fact = Nucleotide::Junction(
        JunctionType::Any,
        vec![
            Nucleotide::String(name.to_string()),
            Nucleotide::String(arg.to_string()),
        ],
    );

    genes.push(Gene::new(OpCode::Push, vec![fact]));
    genes.push(Gene::new(OpCode::Assert, vec![]));
    Ok(genes)
}

fn compile_genetics_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner.next().unwrap().as_str();
    let arg1 = inner.next().unwrap().as_str();
    let arg2 = inner.next().unwrap().as_str();

    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg1.to_string())],
    ));
    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg2.to_string())],
    ));

    if op == "splice" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(0)])); // Splice method
        genes.push(Gene::new(OpCode::Splice, vec![]));
    } else if let Ok(opcode) = OpCode::from_str(&op.to_ascii_lowercase()) {
        genes.push(Gene::new(opcode, vec![]));
    } else {
        // Fallback for custom or unknown genetic opcodes
        genes.push(Gene::new(OpCode::Unknown(op.to_string()), vec![]));
    }

    Ok(genes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetics_block() {
        let code = r#"
genetics {
    splice dna1 dna2
    recombine a b
}
"#;
        let dna = compile(code).unwrap();
        let genes = &dna.helix.strands[0].genes;

        // "splice dna1 dna2" -> Push("dna1"), Push("dna2"), Push(0), Splice
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::String("dna1".to_string()));
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::String("dna2".to_string()));
        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[2].args[0], Nucleotide::Number(0));
        assert_eq!(genes[3].op, OpCode::Splice);

        // "recombine a b" -> Push("a"), Push("b"), Recombine
        assert_eq!(genes[4].op, OpCode::Push);
        assert_eq!(genes[4].args[0], Nucleotide::String("a".to_string()));
        assert_eq!(genes[5].op, OpCode::Push);
        assert_eq!(genes[5].args[0], Nucleotide::String("b".to_string()));
        assert_eq!(genes[6].op, OpCode::Recombine);
    }
}