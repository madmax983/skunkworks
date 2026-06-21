#![allow(missing_docs)]
use pest_derive::Parser;

/// The base parser for the Chimera language.
///
/// This parser defines the core grammatical rules for the language
/// via the `grammar.pest` file, mapping the physical layout of instructions
/// into their structured counterparts.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;
