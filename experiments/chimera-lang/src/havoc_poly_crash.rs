#[cfg(test)]
mod tests {
    use crate::ast::JunctionType;
    use crate::vm::Value;
    use rustc_hash::FxHasher;
    use std::hash::Hash;

    #[test]
    fn test_value_hash_recursion_blowup() {
        // Build a highly recursive value to blow up Hash
        let mut val = Value::Int(1);
        for _ in 0..1000 {
            val = Value::Junction(JunctionType::Any, vec![val.clone()]);
        }
        let mut hasher = FxHasher::default();
        val.hash(&mut hasher);
    }
}
