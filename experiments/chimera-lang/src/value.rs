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
    /// A 24-bit RGB Color. Used for Chromatics and Visuals.
    Color(u8, u8, u8),
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
            Value::Color(r, g, b) => {
                r.hash(state);
                g.hash(state);
                b.hash(state);
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_depth(f, 0)
    }
}

impl Value {
    fn fmt_depth(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        if depth > 50 {
            return write!(f, "...");
        }
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
                    v.fmt_depth(f, depth + 1)?;
                }
                write!(f, ")")
            }
            Value::Superposition(states) => {
                write!(f, "Ψ(")?;
                for (i, (v, p)) in states.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    v.fmt_depth(f, depth + 1)?;
                    write!(f, ":{:.2}", p)?;
                }
                write!(f, ")")
            }
            Value::Symbol(id) => write!(f, "§{:x}", id),
            Value::Color(r, g, b) => write!(f, "#[{:02X},{:02X},{:02X}]", r, g, b),
        }
    }

    /// Recursively calculates the depth of nested structures.
    ///
    /// - Int/Str: Depth 0
    /// - Junction/Superposition: 1 + max(children.depth())
    pub fn depth(&self) -> usize {
        self.depth_safe(0)
    }

    fn depth_safe(&self, depth: usize) -> usize {
        // Safe limit to prevent stack overflow during check
        if depth > 1000 {
            return 1000;
        }
        match self {
            Value::Int(_) | Value::Str(_) | Value::Symbol(_) | Value::Color(_, _, _) => 0,
            Value::Junction(_, vals) => {
                1 + vals
                    .iter()
                    .map(|v| v.depth_safe(depth + 1))
                    .max()
                    .unwrap_or(0)
            }
            Value::Superposition(states) => {
                1 + states
                    .iter()
                    .map(|(v, _)| v.depth_safe(depth + 1))
                    .max()
                    .unwrap_or(0)
            }
        }
    }

    /// Calculates the total number of nodes in the value tree.
    pub fn complexity(&self) -> usize {
        self.complexity_safe(0)
    }

    fn complexity_safe(&self, depth: usize) -> usize {
        if depth > 1000 {
            return 1000;
        }
        match self {
            Value::Int(_) | Value::Str(_) | Value::Symbol(_) | Value::Color(_, _, _) => 1,
            Value::Junction(_, vals) => {
                1 + vals
                    .iter()
                    .map(|v| v.complexity_safe(depth + 1))
                    .sum::<usize>()
            }
            Value::Superposition(states) => {
                1 + states
                    .iter()
                    .map(|(v, _)| v.complexity_safe(depth + 1))
                    .sum::<usize>()
            }
        }
    }
}
