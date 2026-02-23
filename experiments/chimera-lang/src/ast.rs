use crate::opcode::OpCode;
use crate::Rule;
use pest::iterators::Pair;
use serde::{Deserialize, Serialize};

/// The complete genetic blueprint of a Chimera organism.
///
/// Contains the `Helix` which stores all executable code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dna {
    /// The double-helix structure containing strands of genes.
    pub helix: Helix,
    /// Optional configuration for evolutionary self-optimization.
    pub evolution_config: Option<EvolutionConfig>,
}

/// Configuration for the Evolution Engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub population_size: usize,
    /// Stored as string to preserve Eq/Hash (parsed as f64 at runtime)
    pub mutation_rate: String,
    /// Index of the strand used as the Fitness Function
    pub fitness_strand_idx: Option<usize>,
    /// Simple target value (if no fitness strand provided)
    pub target_value: Option<i64>,
}

/// A collection of DNA strands.
///
/// Represents the chromosome set of the organism.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Helix {
    /// List of named or anonymous functions (Strands).
    pub strands: Vec<Strand>,
}

/// A sequence of instructions (Genes).
///
/// Equivalent to a function or subroutine in traditional programming.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Strand {
    /// The sequence of operations to execute.
    pub genes: Vec<Gene>,
}

/// A single instruction.
///
/// Consists of an Enzyme (`OpCode`) and its Arguments (`Nucleotide`s).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Gene {
    /// The operation to perform (Enzyme).
    pub op: OpCode,
    /// The arguments for the operation (Nucleotides).
    pub args: Vec<Nucleotide>,
}

/// Logic gate type for Junctions.
///
/// Determines how a Junction aggregates its children.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub enum JunctionType {
    /// Returns true if ANY child is true (OR gate).
    Any,
    /// Returns true if ALL children are true (AND gate).
    All,
    /// Represents a spatial collection (e.g. cells in a Petri Dish).
    Dish,
}

/// The atomic unit of data in DNA.
///
/// Represents arguments to Genes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nucleotide {
    /// Integer literal (e.g. `42`).
    Number(i64),
    /// String literal (e.g. `"hello"`).
    String(String),
    /// Identifier (variable or label name).
    #[allow(dead_code)]
    Identifier(String),
    /// A structured collection of nucleotides (List/Tuple).
    Junction(JunctionType, Vec<Nucleotide>),
}

impl Dna {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::dna => {
                let mut inner = pair.into_inner();
                let helix_pair = inner.next().ok_or("Expected helix in DNA")?;
                let helix = Helix::try_from_pair(helix_pair)?;
                Ok(Dna {
                    helix,
                    evolution_config: None,
                })
            }
            _ => Err(format!("Expected DNA rule, got {:?}", pair.as_rule())),
        }
    }
}

// --- DX Improvements ---

impl Dna {
    /// Creates a simple organism with a single strand of genes.
    pub fn from_genes(genes: Vec<Gene>) -> Self {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
            evolution_config: None,
        }
    }
}

impl Gene {
    /// Helper to construct a Gene with arguments.
    pub fn new(op: OpCode, args: Vec<Nucleotide>) -> Self {
        Gene { op, args }
    }
}

impl From<OpCode> for Gene {
    fn from(op: OpCode) -> Self {
        Gene { op, args: vec![] }
    }
}

impl From<i64> for Nucleotide {
    fn from(n: i64) -> Self {
        Nucleotide::Number(n)
    }
}

impl From<String> for Nucleotide {
    fn from(s: String) -> Self {
        Nucleotide::String(s)
    }
}

impl From<&str> for Nucleotide {
    fn from(s: &str) -> Self {
        Nucleotide::String(s.to_string())
    }
}

impl Helix {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::helix => {
                let strands: Result<Vec<Strand>, String> =
                    pair.into_inner().map(Strand::try_from_pair).collect();
                Ok(Helix { strands: strands? })
            }
            _ => Err(format!("Expected Helix rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Strand {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::strand => {
                let genes: Result<Vec<Gene>, String> =
                    pair.into_inner().map(Gene::try_from_pair).collect();
                Ok(Strand { genes: genes? })
            }
            _ => Err(format!("Expected Strand rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Gene {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        match pair.as_rule() {
            Rule::gene => {
                let mut inner = pair.into_inner();
                let name = inner.next().ok_or("Expected gene name")?.as_str();
                let op = name
                    .parse()
                    .map_err(|_| format!("Failed to parse opcode: {}", name))?;
                let args_pair = inner.next().ok_or("Expected gene args")?;
                let args: Result<Vec<Nucleotide>, String> = args_pair
                    .into_inner()
                    .map(Nucleotide::try_from_pair)
                    .collect();
                Ok(Gene { op, args: args? })
            }
            _ => Err(format!("Expected Gene rule, got {:?}", pair.as_rule())),
        }
    }
}

impl Nucleotide {
    pub fn try_from_pair(pair: Pair<Rule>) -> Result<Self, String> {
        Self::try_from_pair_with_depth(pair, 0)
    }

    fn try_from_pair_with_depth(pair: Pair<Rule>, depth: usize) -> Result<Self, String> {
        if depth > 100 {
            return Err("Recursion limit exceeded".to_string());
        }
        match pair.as_rule() {
            Rule::number => {
                let s = pair.as_str();
                s.parse()
                    .map(Nucleotide::Number)
                    .map_err(|e| format!("Invalid number '{}': {}", s, e))
            }
            Rule::string => {
                let s = pair.as_str();
                // Remove quotes
                if s.len() < 2 {
                    return Err(format!("Invalid string literal: {}", s));
                }
                Ok(Nucleotide::String(s[1..s.len() - 1].to_string()))
            }
            Rule::identifier => Ok(Nucleotide::Identifier(pair.as_str().to_string())),
            Rule::junction => {
                let mut inner = pair.into_inner();
                let type_pair = inner.next().ok_or("Expected junction type")?;
                let j_type = match type_pair.as_str() {
                    "any" => JunctionType::Any,
                    "all" => JunctionType::All,
                    "dish" => JunctionType::Dish,
                    _ => return Err(format!("Unknown junction type: {}", type_pair.as_str())),
                };
                let args_pair = inner.next().ok_or("Expected junction args")?;
                let args: Result<Vec<Nucleotide>, String> = args_pair
                    .into_inner()
                    .map(|p| Self::try_from_pair_with_depth(p, depth + 1))
                    .collect();
                Ok(Nucleotide::Junction(j_type, args?))
            }
            _ => Err(format!(
                "Expected Nucleotide rule, got {:?}",
                pair.as_rule()
            )),
        }
    }
}
