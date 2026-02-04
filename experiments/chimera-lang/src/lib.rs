use pest_derive::Parser;

pub mod ast;
pub mod tui;
pub mod vm;

#[cfg(test)]
mod nova_test;
#[cfg(test)]
mod havoc_repro;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;
