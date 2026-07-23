use anyhow::{anyhow, Result};
use pest::Parser;

pub mod ast;
pub mod compiler;
pub mod parser;

use ast::ast::*;
use parser::{EsolangParser, Rule};

pub fn parse(source: &str) -> Result<Program> {
    let mut pairs = EsolangParser::parse(Rule::program, source)?;
    let program = pairs.next().ok_or(anyhow!("No program found"))?;

    let mut parsed_program = Program {
        strands: std::collections::HashMap::new(),
        organelles: std::collections::HashMap::new(),
        grids: std::collections::HashMap::new(),
        rules: std::collections::HashMap::new(),
    };

    for pair in program.into_inner() {
        #[allow(clippy::single_match)]
        match pair.as_rule() {
            Rule::declaration => {
                let inner = pair.into_inner().next().unwrap();
                #[allow(clippy::single_match)]
                match inner.as_rule() {
                    Rule::strand_def => {
                        let mut strand_parts = inner.into_inner();
                        let name = strand_parts.next().unwrap().as_str().to_string();
                        let mut instructions = Vec::new();
                        for instr_pair in strand_parts {
                            instructions.push(parse_instruction(instr_pair)?);
                        }
                        parsed_program
                            .strands
                            .insert(name.clone(), ast::ast::Strand { name, instructions });
                    }
                    // ... parsing rules ...
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(parsed_program)
}

fn parse_instruction(pair: pest::iterators::Pair<'_, Rule>) -> Result<Instruction> {
    let inner = pair.into_inner().next().unwrap();
    #[allow(clippy::single_match)]
    match inner.as_rule() {
        Rule::number => Ok(Instruction::Number(inner.as_str().parse()?)),
        Rule::string => {
            let s = inner.as_str();
            Ok(Instruction::String(s[1..s.len() - 1].to_string()))
        }
        Rule::identifier => Ok(Instruction::Identifier(inner.as_str().to_string())),
        Rule::operator => Ok(Instruction::Operator(inner.as_str().to_string())),
        Rule::spawn_instr => Ok(Instruction::Spawn(
            inner.into_inner().next().unwrap().as_str().to_string(),
        )),
        Rule::exec_instr => Ok(Instruction::Call(
            inner.into_inner().next().unwrap().as_str().to_string(),
        )),
        Rule::query_instr => Ok(Instruction::Query(
            inner.into_inner().next().unwrap().as_str().to_string(),
        )),
        _ => Err(anyhow!("Unknown instruction rule: {:?}", inner.as_rule())),
    }
}

// Facade API
pub use ast::*;
pub use compiler::*;
pub use parser::*;
