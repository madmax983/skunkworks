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
        // Attempt to render as a table for complex Junctions
        if let Some(table_str) = self.as_table_string() {
            return write!(f, "\n{}", table_str);
        }
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

    /// Tries to format the Value as a pretty table if it's a Junction of Junctions.
    fn as_table_string(&self) -> Option<String> {
        if let Value::Junction(_, rows) = self {
            // Heuristic: Must be a list of Junctions to be a table
            if rows.is_empty() {
                return None;
            }

            // Check first row to establish column count
            let cols_len = if let Value::Junction(_, cols) = &rows[0] {
                cols.len()
            } else {
                return None; // Not a table (list of primitives?)
            };

            if cols_len == 0 {
                return None;
            }

            // Validate that most rows look like rows?
            // Actually let's just try to build it.
            // Only convert if depth is exactly 2 (Junction -> Junctions -> Primitives)
            // or we want to allow nested values in cells (which display fine).

            let mut table = comfy_table::Table::new();
            table.load_preset(comfy_table::presets::UTF8_FULL);
            // Compact mode for CLI dashboard feel
            table.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS);

            for row_val in rows {
                if let Value::Junction(_, cells) = row_val {
                    let row_cells: Vec<comfy_table::Cell> = cells
                        .iter()
                        .map(|v| {
                            // Special formatting for boolean-like values
                            match v {
                                Value::Str(s) if s.eq_ignore_ascii_case("true") => {
                                    comfy_table::Cell::new("True").fg(comfy_table::Color::Green)
                                }
                                Value::Str(s) if s.eq_ignore_ascii_case("false") => {
                                    comfy_table::Cell::new("False").fg(comfy_table::Color::Red)
                                }
                                // Maybe Int(1)/Int(0)?
                                // Value::Int(1) => comfy_table::Cell::new("1").fg(comfy_table::Color::Green),
                                // Value::Int(0) => comfy_table::Cell::new("0").fg(comfy_table::Color::Red),
                                _ => comfy_table::Cell::new(v.to_string()),
                            }
                        })
                        .collect();
                    table.add_row(row_cells);
                } else {
                    return None; // Mixed structure
                }
            }

            return Some(table.to_string());
        }
        None
    }

    /// Applies a binary operation recursively to Values.
    ///
    /// Handles:
    /// - (Int, Int) -> Int
    /// - (Junction, Scalar) -> Junction (map)
    /// - (Scalar, Junction) -> Junction (map)
    /// - (Junction, Junction) -> Junction (cross product)
    /// - (Superposition, Scalar) -> Superposition (map)
    /// - (Scalar, Superposition) -> Superposition (map)
    /// - (Superposition, Superposition) -> Superposition (cross product)
    ///
    /// Returns `None` if recursion depth exceeds `max_depth` or result size exceeds `max_size`.
    pub fn apply_binary_op<F>(
        self,
        other: Value,
        op: F,
        max_depth: usize,
        max_size: usize,
    ) -> Option<Value>
    where
        F: Fn(i64, i64) -> i64 + Copy,
    {
        self.apply_binary_op_recursive(other, op, 0, max_depth, max_size)
    }

    fn apply_binary_op_recursive<F>(
        self,
        other: Value,
        op: F,
        depth: usize,
        max_depth: usize,
        max_size: usize,
    ) -> Option<Value>
    where
        F: Fn(i64, i64) -> i64 + Copy,
    {
        if depth > max_depth {
            return None;
        }
        match (self, other) {
            (Value::Int(ia), Value::Int(ib)) => Some(Value::Int(op(ia, ib))),
            (Value::Junction(t, vals), scalar @ Value::Int(_)) => {
                let mut res = Vec::new();
                for v in vals {
                    if res.len() >= max_size {
                        return None;
                    }
                    if let Some(r) = v.apply_binary_op_recursive(
                        scalar.clone(),
                        op,
                        depth + 1,
                        max_depth,
                        max_size,
                    ) {
                        res.push(r);
                    } else {
                        return None;
                    }
                }
                Some(Value::Junction(t, res))
            }
            (scalar @ Value::Int(_), Value::Junction(t, vals)) => {
                let mut res = Vec::new();
                for v in vals {
                    if res.len() >= max_size {
                        return None;
                    }
                    if let Some(r) = scalar.clone().apply_binary_op_recursive(
                        v,
                        op,
                        depth + 1,
                        max_depth,
                        max_size,
                    ) {
                        res.push(r);
                    } else {
                        return None;
                    }
                }
                Some(Value::Junction(t, res))
            }
            (Value::Junction(ta, va), Value::Junction(_tb, vb)) => {
                // Cross product, defaulting to type of A
                let mut res = Vec::new();
                for xa in va {
                    for xb in &vb {
                        if res.len() >= max_size {
                            return None;
                        }
                        if let Some(r) = xa.clone().apply_binary_op_recursive(
                            xb.clone(),
                            op,
                            depth + 1,
                            max_depth,
                            max_size,
                        ) {
                            res.push(r);
                        }
                    }
                }
                Some(Value::Junction(ta, res))
            }
            (Value::Superposition(states), scalar @ Value::Int(_)) => {
                let mut res = Vec::new();
                for (v, p) in states {
                    if let Some(r) = v.apply_binary_op_recursive(
                        scalar.clone(),
                        op,
                        depth + 1,
                        max_depth,
                        max_size,
                    ) {
                        res.push((r, p));
                    } else {
                        return None;
                    }
                }
                Some(Value::Superposition(res))
            }
            (scalar @ Value::Int(_), Value::Superposition(states)) => {
                let mut res = Vec::new();
                for (v, p) in states {
                    if let Some(r) = scalar.clone().apply_binary_op_recursive(
                        v,
                        op,
                        depth + 1,
                        max_depth,
                        max_size,
                    ) {
                        res.push((r, p));
                    } else {
                        return None;
                    }
                }
                Some(Value::Superposition(res))
            }
            (Value::Superposition(states_a), Value::Superposition(states_b)) => {
                let mut res = Vec::new();
                for (va, pa) in states_a {
                    for (vb, pb) in &states_b {
                        if res.len() >= max_size {
                            return None;
                        }
                        if let Some(r) = va.clone().apply_binary_op_recursive(
                            vb.clone(),
                            op,
                            depth + 1,
                            max_depth,
                            max_size,
                        ) {
                            res.push((r, pa * pb));
                        }
                    }
                }
                Some(Value::Superposition(res))
            }
            _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_binary_op_int() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        let res = a.apply_binary_op(b, |x, y| x + y, 100, 1024).unwrap();
        assert_eq!(res, Value::Int(30));
    }

    #[test]
    fn test_apply_binary_op_junction_scalar() {
        // Junction(Any, [1, 2]) + 3 = Junction(Any, [4, 5])
        let a = Value::Junction(JunctionType::Any, vec![Value::Int(1), Value::Int(2)]);
        let b = Value::Int(3);
        let res = a.apply_binary_op(b, |x, y| x + y, 100, 1024).unwrap();
        assert_eq!(
            res,
            Value::Junction(JunctionType::Any, vec![Value::Int(4), Value::Int(5)])
        );
    }

    #[test]
    fn test_apply_binary_op_scalar_junction() {
        // 3 + Junction(Any, [1, 2]) = Junction(Any, [4, 5])
        let a = Value::Int(3);
        let b = Value::Junction(JunctionType::Any, vec![Value::Int(1), Value::Int(2)]);
        let res = a.apply_binary_op(b, |x, y| x + y, 100, 1024).unwrap();
        assert_eq!(
            res,
            Value::Junction(JunctionType::Any, vec![Value::Int(4), Value::Int(5)])
        );
    }

    #[test]
    fn test_apply_binary_op_junction_junction() {
        // Junction(Any, [1, 2]) + Junction(All, [10, 20])
        // = Junction(Any, [11, 21, 12, 22])
        let a = Value::Junction(JunctionType::Any, vec![Value::Int(1), Value::Int(2)]);
        let b = Value::Junction(JunctionType::All, vec![Value::Int(10), Value::Int(20)]);
        let res = a.apply_binary_op(b, |x, y| x + y, 100, 1024).unwrap();

        match res {
            Value::Junction(JunctionType::Any, vals) => {
                assert_eq!(vals.len(), 4);
                assert!(vals.contains(&Value::Int(11)));
                assert!(vals.contains(&Value::Int(21)));
                assert!(vals.contains(&Value::Int(12)));
                assert!(vals.contains(&Value::Int(22)));
            }
            _ => panic!("Expected Junction(Any, ...)"),
        }
    }

    #[test]
    fn test_limit_exceeded() {
        let a = Value::Junction(JunctionType::Any, vec![Value::Int(1); 10]);
        let b = Value::Junction(JunctionType::Any, vec![Value::Int(2); 10]);
        // 10 * 10 = 100 elements, limit 50
        let res = a.apply_binary_op(b, |x, y| x + y, 100, 50);
        assert!(res.is_none());
    }
}

#[test]
fn test_apply_binary_op_junction_junction_partial_failure() {
    // Junction(Any, [1, "a"]) + Junction(Any, [2])
    // 1 + 2 = 3
    // "a" + 2 = Error (None)
    // Result should be Junction(Any, [3]) (skipping failure)

    let a = Value::Junction(
        JunctionType::Any,
        vec![Value::Int(1), Value::Str("a".to_string())],
    );
    let b = Value::Junction(JunctionType::Any, vec![Value::Int(2)]);

    // We need an op that fails for Str
    let res = a.apply_binary_op(b, |x, y| x + y, 100, 1024).unwrap();

    match res {
        Value::Junction(JunctionType::Any, vals) => {
            assert_eq!(vals.len(), 1);
            assert_eq!(vals[0], Value::Int(3));
        }
        _ => panic!("Expected Junction(Any, [3])"),
    }
}
