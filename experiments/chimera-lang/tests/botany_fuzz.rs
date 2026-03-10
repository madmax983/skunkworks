#[cfg(test)]
mod tests {
    use chimera_lang::vm::{ChimeraVM, Value, nova::{Organelle, OrganelleType}};
    use chimera_lang::ast::{Dna, Helix};

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn havoc_botany_chars_panic() {
        // 👺 Havoc: The `tick_seed` function in `nova_botany.rs` compares
        // `i >= s.len()` where `s.len()` is the length of the string in BYTES.
        // It then fetches `s.chars().nth(i).unwrap()`.
        // If `s` contains a multi-byte Emoji like "🍄" (4 bytes), then `s.len()` is 4.
        // By setting index `i` to 2, `2 >= 4` is false, so it tries to fetch `nth(2)`.
        // But there is only 1 character in the string, so `nth(2)` is None!
        // This causes an immediate unwrap() panic!
        let dna = Dna {
            helix: Helix { strands: vec![] },
            evolution_config: None,
        };
        let mut vm = ChimeraVM::new(dna);

        vm.stack.push(Value::Str("rules".to_string()));
        vm.stack.push(Value::Str("mapping".to_string()));

        let s = "🍄"; // 1 char, 4 bytes. len() == 4
        vm.stack.push(Value::Str(s.to_string()));
        vm.stack.push(Value::Int(2)); // index 2 bypasses `i >= 4` but panics on `nth(2)`
        vm.stack.push(Value::Int(0)); // turtle stack

        let mut organelle = Organelle {
            stack: vec![],
            ip: (0, 0),
            context_loc: (0, 0),
            call_stack: vec![],
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Seed,
            direction: (0, 0),
            ttl: None,
            name: "Havoc Spore".to_string(),
            traits: vec![],
            id: 1,
            tissue_id: None,
            genome_id: 1,
            energy: 100,
            experience: 0,
            stage: 0,
        };

        chimera_lang::vm::nova_botany::tick_seed(&mut vm, &mut organelle);
    }
}
