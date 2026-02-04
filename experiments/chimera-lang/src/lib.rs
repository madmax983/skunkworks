use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod opcode;
pub mod tui;
pub mod vm;

#[cfg(test)]
mod cortex_test;
mod havoc_repro;
#[cfg(test)]
mod nova_cerebellum_test;
#[cfg(test)]
mod nova_crispr_test;
#[cfg(test)]
mod nova_hormone_test;
#[cfg(test)]
mod nova_spore_test;
mod nova_test;
#[cfg(test)]
mod nova_waste_test;
