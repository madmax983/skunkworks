use crate::ast::JunctionType;
use serde::{Deserialize, Serialize};

/// The fundamental data types in the Chimera VM.
///
/// Can be stored on the Stack, in the Grid, or in a Junction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// A 64-bit integer. The basic unit of arithmetic and coordinates.
    Int(i64),
    /// A UTF-8 string. Used for gene names, messages, and genetic code.
    Str(String),
    /// A collection of values with logic gate semantics (Any/All).
    ///
    /// Used for complex data structures and pattern matching.
    Junction(JunctionType, Vec<Value>),
    /// A quantum superposition of values with associated probabilities.
    ///
    /// Used by Nova features for probabilistic computing.
    Superposition(Vec<(Value, f64)>),
    /// An abstract symbol ID. Used by Semiotics features.
    Symbol(u64),
}

impl Eq for Value {}

#[allow(clippy::derive_hash_xor_eq)]
impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Value::Int(i) => i.hash(state),
            Value::Str(s) => s.hash(state),
            Value::Junction(t, vals) => {
                t.hash(state);
                vals.hash(state);
            }
            Value::Superposition(states) => {
                for (v, p) in states {
                    v.hash(state);
                    p.to_bits().hash(state);
                }
            }
            Value::Symbol(id) => id.hash(state),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Junction(t, vals) => {
                let t_str = match t {
                    JunctionType::Any => "any",
                    JunctionType::All => "all",
                    JunctionType::Dish => "dish",
                };
                write!(f, "{}(", t_str)?;
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
            Value::Superposition(states) => {
                write!(f, "Ψ(")?;
                for (i, (v, p)) in states.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}:{:.2}", v, p)?;
                }
                write!(f, ")")
            }
            Value::Symbol(id) => write!(f, "§{:x}", id),
        }
    }
}

impl Value {
    /// Recursively calculates the depth of nested structures.
    ///
    /// - Int/Str: Depth 0
    /// - Junction/Superposition: 1 + max(children.depth())
    pub fn depth(&self) -> usize {
        match self {
            Value::Int(_) | Value::Str(_) | Value::Symbol(_) => 0,
            Value::Junction(_, vals) => 1 + vals.iter().map(|v| v.depth()).max().unwrap_or(0),
            Value::Superposition(states) => {
                1 + states.iter().map(|(v, _)| v.depth()).max().unwrap_or(0)
            }
        }
    }
}
