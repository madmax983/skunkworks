use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod tui;
pub mod vm;

#[cfg(test)]
mod nova_crispr_test;
mod havoc_repro;
mod nova_test;
