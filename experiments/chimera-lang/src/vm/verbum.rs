//! # Linguistics & Words of Power 🗣️
//!
//! This module defines the linguistic engine of the Chimera Virtual Machine.
//! In the Chimera universe, `Verbum` (Words) are constructed from `Gene` sequences
//! and possess intrinsic power, rarity, and etymological history.
//!
//! The `VerbumForge` acts as the lexicon and factory for these words, allowing
//! agents to construct complex magical incantations (macros) by forging smaller
//! genetic sequences into named `Verbum` entities.

use crate::ast::Gene;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The rarity of a forged [`Verbum`], determined dynamically by the length and
/// complexity of its underlying genetic sequence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    /// 0 to 5 genes. A simple, foundational word.
    Common,
    /// 6 to 10 genes. A slightly more complex utterance.
    Uncommon,
    /// 11 to 20 genes. A potent magical phrase.
    Rare,
    /// 21 to 50 genes. A specialized, highly concentrated incantation.
    Epic,
    /// 51 to 100 genes. A legendary word of power.
    Legendary,
    /// 100+ genes. A world-altering mythic construct.
    Mythic,
}

/// A "Word of Power" representing a macro of Chimera [`Gene`]s.
///
/// A [`Verbum`] encapsulates a reusable sequence of logic that can be invoked by name.
/// It tracks its own energy cost, rarity, and etymological roots (which other words
/// were combined to create it).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verbum {
    /// A unique identifier for the word within the forge.
    pub id: usize,
    /// The string representation or "name" of the word (e.g., "Fireball").
    pub name: String,
    /// The actual executable genetic sequence this word represents.
    pub genes: Vec<Gene>,
    /// The energy cost required to cast or invoke this word.
    pub cost: i64,
    /// The designated rarity based on the word's complexity.
    pub rarity: Rarity,
    /// The parent words that were combined or referenced to create this word.
    pub etymology: Vec<String>,
}

/// The lexicon and manufacturing plant for [`Verbum`] entities.
///
/// The [`VerbumForge`] is responsible for registering new words, calculating their
/// costs and rarities, and ensuring naming uniqueness.
///
/// # Examples
///
/// ```
/// use chimera_lang::ast::{Gene, Nucleotide};
/// use chimera_lang::opcode::OpCode;
/// use chimera_lang::vm::verbum::VerbumForge;
///
/// let mut forge = VerbumForge::new();
///
/// // Create a simple gene sequence
/// let genes = vec![
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
///     Gene { op: OpCode::Print, args: vec![] }
/// ];
///
/// // Forge a new word of power
/// let word_id = forge.forge("Answer".to_string(), genes, vec![]).unwrap();
///
/// // Retrieve the forged word
/// let word = forge.get("Answer").unwrap();
/// assert_eq!(word.name, "Answer");
/// assert_eq!(word.cost, 10); // 2 genes * 5 base cost
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerbumForge {
    /// A mapping from word names to their actual [`Verbum`] structure.
    pub words: HashMap<String, Verbum>,
    /// The counter used to assign unique IDs to newly forged words.
    pub next_id: usize,
}

impl VerbumForge {
    /// Creates a new, empty [`VerbumForge`].
    pub fn new() -> Self {
        Self {
            words: HashMap::new(),
            next_id: 0,
        }
    }

    /// Forges a new [`Verbum`] from a genetic sequence and registers it in the lexicon.
    ///
    /// The forge automatically calculates the word's energy `cost` (5 per gene) and
    /// its `rarity` based on the sequence length.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique string identifier for this new word.
    /// * `genes` - The executable Chimera sequence the word will contain.
    /// * `parents` - A list of parent word names that influenced this creation.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if a word with the given `name` already exists in the forge.
    ///
    /// # Examples
    ///
    /// ```
    /// use chimera_lang::vm::verbum::VerbumForge;
    /// let mut forge = VerbumForge::new();
    ///
    /// assert!(forge.forge("Test".to_string(), vec![], vec![]).is_ok());
    ///
    /// // Attempting to forge the same word twice results in an error
    /// assert!(forge.forge("Test".to_string(), vec![], vec![]).is_err());
    /// ```
    pub fn forge(
        &mut self,
        name: String,
        genes: Vec<Gene>,
        parents: Vec<String>,
    ) -> Result<usize, String> {
        if self.words.contains_key(&name) {
            return Err(format!("Word '{}' already exists", name));
        }

        let mut cost = genes.len() as i64 * 5; // Base cost
        if cost == 0 {
            cost = 1;
        }

        let rarity = match genes.len() {
            0..=5 => Rarity::Common,
            6..=10 => Rarity::Uncommon,
            11..=20 => Rarity::Rare,
            21..=50 => Rarity::Epic,
            51..=100 => Rarity::Legendary,
            _ => Rarity::Mythic,
        };

        let id = self.next_id;
        self.next_id += 1;

        let word = Verbum {
            id,
            name: name.clone(),
            genes: genes.clone(), // Clone here
            cost,
            rarity,
            etymology: parents,
        };

        self.words.insert(name, word);
        Ok(id)
    }

    /// Retrieves a reference to a forged [`Verbum`] by its name.
    ///
    /// Returns `None` if the word does not exist in the forge.
    pub fn get(&self, name: &str) -> Option<&Verbum> {
        self.words.get(name)
    }

    /// Retrieves the genetic sequence and energy cost of a forged word.
    ///
    /// This is a convenience method for quickly looking up the executable
    /// data of a word without needing the full [`Verbum`] struct.
    pub fn get_word_data(&self, name: &str) -> Option<(Vec<Gene>, i64)> {
        self.words.get(name).map(|w| (w.genes.clone(), w.cost))
    }
}
