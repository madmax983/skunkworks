use anyhow::{anyhow, Result};
use pest::Parser;
use std::str::FromStr;

use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;

#[allow(missing_docs)]
pub mod prolougeparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[allow(missing_docs)]
    #[grammar = "prolouge_grammar.pest"]
    pub struct ProlougeParser;
}
pub use prolougeparser_mod::ProlougeParser;
pub use prolougeparser_mod::Rule;

pub fn compile(source: &str) -> Result<Dna> {
    let mut pairs = ProlougeParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut genes = Vec::new();

    for instruction in program.into_inner() {
        if instruction.as_rule() == Rule::EOI {
            break;
        }

        if instruction.as_rule() != Rule::instruction {
            continue;
        }

        let inner_instr = instruction
            .into_inner()
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected inner instruction pair"))?;

        match inner_instr.as_rule() {
            Rule::forth_instruction => {
                let forth_inner = inner_instr
                    .into_inner()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Expected inner rule"))?;
                genes.extend(compile_forth_instr(forth_inner)?);
            }
            #[cfg(feature = "nova")]
            Rule::raku_instruction => {
                let raku_inner = inner_instr
                    .into_inner()
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Expected inner rule"))?;
                genes.extend(compile_raku_instr(raku_inner)?);
            }
            Rule::orca_instruction => {
                genes.extend(compile_orca_instr(inner_instr)?);
            }
            Rule::elektra_instruction => {
                genes.extend(compile_elektra_instr(inner_instr)?);
            }
            Rule::prolog_instruction => {
                genes.extend(compile_prolog_instr(inner_instr)?);
            }
            Rule::genetics_instruction => {
                genes.extend(compile_genetics_instr(inner_instr)?);
            }
            Rule::tui_instruction => {
                genes.extend(compile_tui_instr(inner_instr)?);
            }
            _ => {}
        }
    }

    let main_strand = Strand { genes };
    let helix = Helix {
        strands: vec![main_strand],
    };
    Ok(Dna {
        evolution_config: None,
        helix,
    })
}

fn compile_forth_instr(inner: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
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

#[cfg(feature = "nova")]
fn compile_raku_instr(inner: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    if inner.as_rule() == Rule::operator {
        let op_str = inner.as_str();
        if op_str == "+" {
            genes.push(Gene::new(OpCode::HyperAdd, vec![]));
        } else if op_str == "-" {
            genes.push(Gene::new(OpCode::HyperSub, vec![]));
        } else if op_str == "*" {
            genes.push(Gene::new(OpCode::HyperMul, vec![]));
        } else if op_str == "/" {
            genes.push(Gene::new(OpCode::HyperDiv, vec![]));
        } else {
            genes.push(Gene::new(
                OpCode::Push,
                vec![Nucleotide::String(op_str.to_string())],
            ));
        }
    }
    Ok(genes)
}

fn compile_orca_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected op"))?
        .as_str();
    let y: i64 = if let Some(y_val) = inner.next() {
        y_val.as_str().parse().unwrap_or(0)
    } else {
        0
    };
    let x: i64 = if let Some(x_val) = inner.next() {
        x_val.as_str().parse().unwrap_or(0)
    } else {
        0
    };

    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(op.to_string())],
    ));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
    genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
    #[cfg(feature = "nova")]
    genes.push(Gene::new(OpCode::Orca, vec![]));
    Ok(genes)
}

fn compile_elektra_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let kind = inner
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected kind"))?
        .as_str();
    let y: i64 = if let Some(y_val) = inner.next() {
        y_val.as_str().parse().unwrap_or(0)
    } else {
        0
    };
    let x: i64 = if let Some(x_val) = inner.next() {
        x_val.as_str().parse().unwrap_or(0)
    } else {
        0
    };

    if kind == "battery" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(9)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        #[cfg(feature = "elektra")]
        genes.push(Gene::new(OpCode::Battery, vec![]));
    } else if kind == "ground" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(y)]));
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(x)]));
        #[cfg(feature = "elektra")]
        genes.push(Gene::new(OpCode::Ground, vec![]));
    }
    Ok(genes)
}

fn compile_prolog_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let name = inner
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected name"))?
        .as_str();
    let arg = if let Some(a) = inner.next() {
        a.as_str()
    } else {
        ""
    };

    let fact = Nucleotide::Junction(
        JunctionType::Any,
        vec![
            Nucleotide::String(name.to_string()),
            Nucleotide::String(arg.to_string()),
        ],
    );

    genes.push(Gene::new(OpCode::Push, vec![fact]));
    #[cfg(feature = "oracle")]
    genes.push(Gene::new(OpCode::Assert, vec![]));
    Ok(genes)
}

fn compile_genetics_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let op = inner
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected op"))?
        .as_str();
    let arg1 = if let Some(a) = inner.next() {
        a.as_str()
    } else {
        ""
    };
    let arg2 = if let Some(a) = inner.next() {
        a.as_str()
    } else {
        ""
    };

    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg1.to_string())],
    ));
    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(arg2.to_string())],
    ));

    if op == "splice" {
        genes.push(Gene::new(OpCode::Push, vec![Nucleotide::Number(0)]));
        #[cfg(feature = "nova")]
        genes.push(Gene::new(OpCode::Splice, vec![]));
    } else if op == "recombine" {
        #[cfg(feature = "nova")]
        genes.push(Gene::new(OpCode::Recombine, vec![]));
    }
    Ok(genes)
}

fn compile_tui_instr(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Gene>> {
    let mut genes = Vec::new();
    let mut inner = pair.into_inner();
    let action = inner
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected action"))?
        .as_str(); // "draw"
    let arg = if let Some(a) = inner.next() {
        a.as_str()
    } else {
        ""
    };

    genes.push(Gene::new(
        OpCode::Push,
        vec![Nucleotide::String(format!(
            "{} {}",
            action.to_uppercase(),
            arg
        ))],
    ));
    genes.push(Gene::new(OpCode::TuiDraw, vec![]));
    Ok(genes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prolouge_compiler() {
        let source = r#"
        10 20 add print
        battery 8 8
        bang 4 4
        parent(john).
        >>+<<
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert!(!genes.is_empty());
    }

    #[test]
    fn test_prolouge_compiler_genetics() {
        let source = r#"
        splice dna1 dna2
        recombine a b
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;
        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::String("dna1".to_string()));
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::String("dna2".to_string()));
        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[2].args[0], Nucleotide::Number(0));
        assert_eq!(genes[3].op, OpCode::Splice);
        assert_eq!(genes[4].op, OpCode::Push);
        assert_eq!(genes[4].args[0], Nucleotide::String("a".to_string()));
        assert_eq!(genes[5].op, OpCode::Push);
        assert_eq!(genes[5].args[0], Nucleotide::String("b".to_string()));
        assert_eq!(genes[6].op, OpCode::Recombine);
    }
}
