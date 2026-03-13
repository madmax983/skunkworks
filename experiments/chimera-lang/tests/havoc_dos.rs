#[cfg(test)]
mod tests {
    use chimera_lang::ast::JunctionType;

    use chimera_lang::vm::Value;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    #[test]
    fn test_val_blowup_stack_overflow() {
        let mut val = Value::Int(1);
        // Let's create an enormous Junction that will cause a stack overflow
        for _ in 0..100000 {
            val = Value::Junction(JunctionType::Any, vec![val.clone()]);
        }
        // Try hashing it
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
    }
}
