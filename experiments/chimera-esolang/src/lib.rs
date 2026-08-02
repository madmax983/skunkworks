//! # Chimera Esolang (Genesis) ⚛️
//!
//! Genesis is a highly experimental esoteric language designed to cross paradigms: Orca grid layouts, Forth stacks, Prolog logic, Raku operators, and Genetic mutations, compiled down into `chimera-lang` DNA.
//!
//! ## 🧬 Grammar and Syntax
//!
//! A Genesis program consists of declarations.
//!
//! ### Strands (Forth/Genetics)
//! Defines a sequence of instructions.
//! ```genesis
//! strand main {
//!     "Hello Genesis"
//!     print
//! }
//! ```
//!
//! ### Organelles (Concurrent Agents)
//! Defines autonomous sub-units.
//! ```genesis
//! organelle worker {
//!     dup
//!     mutate
//! }
//! ```
//!
//! ### Grids (Orca inspired)
//! Defines a spatial layout.
//! ```genesis
//! grid layout {
//!     | A B C |
//!     | D E F |
//! }
//! ```
//!
//! ### Rules (Prolog inspired)
//! Defines logic rules.
//! ```genesis
//! rule ancestor(?X) :- parent(?X).
//! ```
//!
//! ### Operators
//! Genesis supports Forth-like stack manipulation (`dup`, `drop`, `swap`, `over`, `rot`), standard math operators (`+`, `-`, `*`, `/`, `%`), meta operators (`<<`, `>>`), and genetic operators (`mutate`, `crossover`, `transcribe`).
//!
//! ## 🚀 Usage
//! ```rust
//! # fn main() {
//! use chimera_esolang::parse;
//! use chimera_esolang::compile;
//! use chimera_lang::vm::ChimeraVM;
//!
//! let source = "strand main { \"Hello Genesis\" print }";
//! let program = parse(source).unwrap();
//! let dna = compile(&program).unwrap();
//! let vm = ChimeraVM::new(dna);
//! # }
//! ```
//!
//! ## ⚠️ Warning
//! Running Genesis triggers Mad Scientist mode (`PrologueEsolang`). Extreme genetic chaos and random Orca signal bursts will flood the grid.
//!
use anyhow::{anyhow, Result};
use pest::Parser;

pub(crate) mod ast;
pub(crate) mod compiler;
pub(crate) mod parser;

#[allow(unused_imports)]
use crate::ast::ast::*;
#[allow(hidden_glob_reexports)]
use parser::{EsolangParser, Rule};

/// Parses a source string into a Program AST.
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
pub use ast::ast::*;
pub use compiler::*;
#[allow(unused_imports)]
pub use parser::*;
