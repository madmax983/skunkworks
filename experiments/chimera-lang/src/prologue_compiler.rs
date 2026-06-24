//! Compilation pipeline for the Prologue circuit language.
//!
//! Provides the parser and compiler to map 2D textual layout grids
//! into executable `PrologueProgram` logic circuits.

use anyhow::{anyhow, Result};
use pest::Parser;
use std::collections::HashMap;
use std::path::Path;

use crate::ast::Dna;
#[cfg(feature = "nova")]
use crate::vm::prologue::AlchemyRule;
use crate::vm::{Value, GRID_SIZE};

#[cfg(not(feature = "nova"))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlchemyRule {
    pub pattern: Vec<Vec<Option<crate::vm::Value>>>,
    pub result: crate::vm::Value,
    pub wildcard: bool,
    pub ingredients: Vec<crate::vm::Value>,
    pub is_radial: bool,
}

/// The parser for the Prologue circuit language.
///
/// Responsible for reading the hybrid text/grid representations used by
/// the Prologue system via the rules defined in `prologue_grammar.pest`.
#[allow(missing_docs)]
pub mod prologueparser_mod {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[allow(missing_docs)]
    #[grammar = "prologue_grammar.pest"]
    pub struct PrologueParser;
}
pub use prologueparser_mod::PrologueParser;
pub use prologueparser_mod::Rule;

/// Represents the parsed structure of a Prologue circuit script.
///
/// Contains both the genetic code logic (`dna`) and the structural layout
/// and configuration needed to execute it.
///
/// # Examples
///
/// ```
/// use chimera_lang::ast::{Dna, Helix};
/// use chimera_lang::prologue_compiler::PrologueProgram;
/// use std::collections::HashMap;
///
/// let program = PrologueProgram {
///     dna: Dna { helix: Helix { strands: vec![] }, evolution_config: None },
///     grid: None,
///     orca_mode: Some(false),
///     custom_runes: HashMap::new(),
///     alchemy_book: vec![],
/// };
///
/// assert_eq!(program.orca_mode, Some(false));
/// ```
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
/// Represents a `PrologueProgram`.
pub struct PrologueProgram {
    /// The compiled biological instruction set to execute the logic circuit.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    /// The `dna` field.
    pub dna: Dna,
    /// An optional pre-configured initial 2D memory space state.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    /// The `grid` field.
    pub grid: Option<Vec<Vec<Value>>>,
    /// Flags if this circuit uses Orca-specific execution timing rules.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    /// The `orca_mode` field.
    pub orca_mode: Option<bool>,
    /// Custom operational runes mapped to specific DNA `OpCode` sequences.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    /// The `custom_runes` field.
    pub custom_runes: HashMap<String, usize>,
    /// Transformation rules for transmuting values via `OpCode::Alchemy`.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    /// The `alchemy_book` field.
    pub alchemy_book: Vec<AlchemyRule>,
}

/// Transforms the visual representation of a `.pro` file into a structured [`PrologueProgram`].
///
/// The Prologue language uses a 2D text grid combined with genetic sequences to define a digital circuit.
/// This compiler exists to parse that hybrid visual/textual format into a runtime-ready data structure containing both the [`Dna`] and the initial memory grid.
///
/// # Examples
///
/// ```
/// use chimera_lang::prologue_compiler::compile;
///
/// // A simple configuration containing only genetic instructions.
/// let source = r#"
/// dna {
///     strand main {
///         5
///     }
/// }
/// "#;
/// let program = compile(source, None).unwrap();
/// assert_eq!(program.dna.helix.strands.len(), 1);
/// ```
///
/// # Details
/// - **The Grid**: Ensure the `grid` block represents a valid 16x16 toroidal space. Elements beyond 16x16 are truncated.
/// - **Integration**: The compiled `dna` section is merged with custom `definitions` mapped to specific rune characters.
/// - **Panics**: Returns `Result::Err` rather than panicking if the syntax is invalid or if the internal ChimeraScript fails to compile.
///
/// Use this in conjunction with [`crate::compiler::compile`] if loading raw scripts.
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
/// Performs the `compile` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of compile
/// ```
pub fn compile(source: &str, base_path: Option<&Path>) -> Result<PrologueProgram> {
    let mut pairs = PrologueParser::parse(Rule::program, source)?;

    let mut grid = None;
    let mut dna: Option<Dna> = None;
    let mut orca_mode = None;
    let mut custom_runes = HashMap::new();
    let mut alchemy_book = Vec::new();

    let program = pairs.next().ok_or_else(|| anyhow!("Empty program"))?;

    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::section => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::config_section => {
                        // "config" ~ "{" ~ config_entry* ~ "}"
                        for entry in inner.into_inner() {
                            let mut entry_inner = entry.into_inner();
                            let key = entry_inner.next().unwrap().as_str();
                            let val = entry_inner.next().unwrap().as_str();
                            if key == "mode" {
                                if val == "Orca" {
                                    orca_mode = Some(true);
                                } else if val == "Normal" {
                                    orca_mode = Some(false);
                                }
                            }
                        }
                    }
                    Rule::grid_section => {
                        let content = inner.into_inner().next().unwrap().as_str();
                        let mut new_grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];

                        // Trim leading/trailing newlines to handle brace placement
                        let trimmed = content.trim_matches(|c| c == '\n' || c == '\r');
                        let lines: Vec<&str> = trimmed.lines().collect();

                        for (y, line) in lines.iter().enumerate() {
                            if y >= GRID_SIZE {
                                break;
                            }
                            let row = parse_grid_line(line);
                            for (x, val) in row.into_iter().enumerate() {
                                if x >= GRID_SIZE {
                                    break;
                                }
                                new_grid[y][x] = val;
                            }
                        }
                        grid = Some(new_grid);
                    }
                    Rule::dna_section => {
                        let content = inner.into_inner().next().unwrap().as_str();
                        let compiled_dna = crate::compiler::compile(content, base_path)?;
                        if let Some(existing_dna) = dna.as_mut() {
                            existing_dna
                                .helix
                                .strands
                                .extend(compiled_dna.helix.strands);
                            if existing_dna.evolution_config.is_none() {
                                existing_dna.evolution_config = compiled_dna.evolution_config;
                            }
                        } else {
                            dna = Some(compiled_dna);
                        }
                    }
                    Rule::definitions_section => {
                        for entry in inner.into_inner() {
                            let mut entry_inner = entry.into_inner();
                            let rune_char = entry_inner.next().unwrap().as_str();
                            let definition_body = entry_inner.next().unwrap();
                            // definition_body -> nested_text -> str
                            // We need the string content inside the braces
                            let content = definition_body.into_inner().next().unwrap().as_str();

                            // The content already defines a strand (e.g., `strand alpha { ... }`)
                            let compiled_def = crate::compiler::compile(content, base_path)?;

                            if dna.is_none() {
                                dna = Some(Dna {
                                    evolution_config: None,
                                    helix: crate::ast::Helix { strands: vec![] },
                                });
                            }

                            if let Some(main_dna) = dna.as_mut() {
                                let start_idx = main_dna.helix.strands.len();
                                // Merge strands
                                main_dna.helix.strands.extend(compiled_def.helix.strands);
                                // Map rune to start index of its definition
                                custom_runes.insert(rune_char.to_string(), start_idx);
                            }
                        }
                    }
                    Rule::alchemy_section => {
                        for rule in inner.into_inner() {
                            if let Rule::alchemy_rule = rule.as_rule() {
                                let mut rule_inner = rule.into_inner();
                                let ingredients_pair = rule_inner.next().unwrap();
                                let result_pair = rule_inner.next().unwrap();

                                let mut ingredients = Vec::new();
                                for term in ingredients_pair.into_inner() {
                                    let inner = term.into_inner().next().unwrap();
                                    match inner.as_rule() {
                                        Rule::string_literal => {
                                            let s = inner.as_str();
                                            ingredients
                                                .push(Value::Str(s[1..s.len() - 1].to_string()));
                                        }
                                        Rule::number_literal => {
                                            if let Ok(n) = inner.as_str().parse::<i64>() {
                                                ingredients.push(Value::Int(n));
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                let result = {
                                    let inner = result_pair.into_inner().next().unwrap();
                                    match inner.as_rule() {
                                        Rule::string_literal => {
                                            let s = inner.as_str();
                                            Value::Str(s[1..s.len() - 1].to_string())
                                        }
                                        Rule::number_literal => {
                                            if let Ok(n) = inner.as_str().parse::<i64>() {
                                                Value::Int(n)
                                            } else {
                                                Value::Int(0)
                                            }
                                        }
                                        _ => Value::Int(0),
                                    }
                                };

                                alchemy_book.push(AlchemyRule {
                                    ingredients,
                                    result,
                                    #[cfg(not(feature = "nova"))]
                                    pattern: vec![],
                                    #[cfg(not(feature = "nova"))]
                                    wildcard: false,
                                    #[cfg(not(feature = "nova"))]
                                    is_radial: false,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    let final_dna = dna.ok_or_else(|| anyhow!("No DNA section found"))?;

    Ok(PrologueProgram {
        dna: final_dna,
        grid,
        orca_mode,
        custom_runes,
        alchemy_book,
    })
}

fn parse_grid_line(line: &str) -> Vec<Value> {
    let mut row = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '"' {
            // Parse String
            chars.next(); // consume opening quote
            let mut s = String::new();
            let mut escaped = false;
            while let Some(&next_c) = chars.peek() {
                if escaped {
                    s.push(chars.next().unwrap());
                    escaped = false;
                } else if next_c == '\\' {
                    chars.next(); // consume backslash
                    escaped = true;
                } else if next_c == '"' {
                    chars.next(); // consume closing quote
                    break;
                } else {
                    s.push(chars.next().unwrap());
                }
            }
            row.push(Value::Str(s));
        } else if c.is_ascii_digit() || c == '-' {
            // Parse Number or potential single char '-' rune
            // To distinguish '-' (math) from -5 (number), we peek ahead.
            // If '-' is followed by digit, it's a number.

            let mut s = String::new();
            s.push(chars.next().unwrap()); // consume the first digit or '-'

            // Check if it's just a '-' and the next character isn't a digit.
            let is_negative_sign_only =
                c == '-' && chars.peek().is_none_or(|&nc| !nc.is_ascii_digit());

            if is_negative_sign_only {
                row.push(Value::Str(s));
            } else {
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_ascii_digit() {
                        s.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(n) = s.parse::<i64>() {
                    row.push(Value::Int(n));
                } else {
                    row.push(Value::Str(s));
                }
            }
        } else if c.is_alphabetic() {
            // Identifier (might be multi-char like "func")
            // BUT: Single char runes are common (A, S, M, O).
            // Logic: If it looks like a word (>1 char), treat as String.
            // If single char, treat as String (Rune).
            let mut s = String::new();
            s.push(chars.next().unwrap());
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    s.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            row.push(Value::Str(s));
        } else {
            // Single Char Rune (Symbols like !, ?, ~, &)
            let c = chars.next().unwrap();
            if c == '.' {
                row.push(Value::Int(0));
            } else {
                row.push(Value::Str(c.to_string()));
            }
        }
    }
    row
}
