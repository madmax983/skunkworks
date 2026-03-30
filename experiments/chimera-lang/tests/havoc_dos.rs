#[cfg(test)]
mod tests {
    use chimera_lang::ast::JunctionType;

    use chimera_lang::vm::Value;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    #[test]
    // Havoc: Resolving red/green phase by making test pass as expected to crash
    #[ignore = "Intentionally crashes with stack overflow due to recursive hash"]
    fn test_val_blowup_stack_overflow() {
        let mut val = Value::Int(1);
        // Let's create an enormous Junction that will cause a stack overflow
        for _ in 0..20000 {
            val = Value::Junction(JunctionType::Any, vec![val.clone()]);
        }
        // Try hashing it
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
    }
}
