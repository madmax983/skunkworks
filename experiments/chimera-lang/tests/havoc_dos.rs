#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

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
