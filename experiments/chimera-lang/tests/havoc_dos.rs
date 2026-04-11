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
            // Use std::mem::replace instead of val.clone() to prevent
            // the previous 'val' from being dropped recursively every iteration.
            val = Value::Junction(JunctionType::Any, vec![std::mem::replace(&mut val, Value::Int(0))]);
        }
        // Try hashing it
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);

        // Prevent drop stack overflow
        // We actually need to clear out the huge Junction so it doesn't trigger the compiler's default Drop.
        // `safe_drop` takes a mut ref, but the outer value is still there (though replaced with Int(0) inside).
        // Since we `std::mem::replace` it with Int(0), the outer value is now `Int(0)`, which drops trivially!
        val.safe_drop();
    }
}
