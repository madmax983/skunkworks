use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod vm;
pub mod tui;

#[cfg(test)]
mod nova_test;
#[cfg(test)]
mod nova_crispr_test;
