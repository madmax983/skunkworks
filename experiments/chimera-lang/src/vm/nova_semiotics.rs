use super::{ChimeraVM, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Converts a value into an abstract Symbol.
///
/// **OpCode:** `Symbolize`
/// **Stack:** `[ ..., value ] -> [ ..., symbol ]`
pub fn exec_symbolize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        let id = hasher.finish();

        // Auto-register meaning in current context if not exists
        let key = (vm.semiotic_context, id);
        vm.meaning_map.entry(key).or_insert(val.clone());

        vm.stack.push(Value::Symbol(id));
        vm.energy = vm.energy.saturating_sub(5);
        vm.output
            .push(format!("SYMBOLIZE: Created Symbol §{:x}", id));
    } else {
        vm.output
            .push("Error: Stack underflow for symbolize".to_string());
    }
    None
}

/// Resolves a Symbol to a value based on the current context.
///
/// **OpCode:** `Interpret`
/// **Stack:** `[ ..., symbol ] -> [ ..., resolved_value ]`
pub fn exec_interpret(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Symbol(id) = val {
            let key = (vm.semiotic_context, id);
            if let Some(resolved) = vm.meaning_map.get(&key) {
                vm.stack.push(resolved.clone());
                vm.energy = vm.energy.saturating_sub(2);
                vm.output
                    .push(format!("INTERPRET: §{:x} -> {}", id, resolved));
            } else {
                // Meaning lost/unknown - drift?
                // For now, push the symbol back or push 0 (meaningless)
                vm.stack.push(Value::Int(0));
                vm.output
                    .push(format!("INTERPRET: Meaning lost for §{:x}", id));
            }
        } else {
            // Self-interpreting value
            vm.stack.push(val);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for interpret".to_string());
    }
    None
}

/// Shifts the semiotic context by XORing with a value.
///
/// **OpCode:** `ContextShift`
/// **Stack:** `[ ..., value ] -> [ ... ]`
pub fn exec_context_shift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        let shift = hasher.finish();

        vm.semiotic_context ^= shift;
        vm.energy = vm.energy.saturating_sub(10);
        vm.output.push(format!(
            "CONTEXT_SHIFT: New Context {:x}",
            vm.semiotic_context
        ));
    } else {
        vm.output
            .push("Error: Stack underflow for context_shift".to_string());
    }
    None
}

/// Deconstructs a string into a junction of constituent Symbols.
///
/// **OpCode:** `Deconstruct`
/// **Stack:** `[ ..., string ] -> [ ..., junction_of_symbols ]`
pub fn exec_deconstruct(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            let mut symbols = Vec::new();
            for c in s.chars() {
                let mut hasher = DefaultHasher::new();
                c.hash(&mut hasher);
                let id = hasher.finish();

                // Register default meaning for char symbols (char itself)
                let key = (0, id); // Base context
                vm.meaning_map
                    .entry(key)
                    .or_insert(Value::Str(c.to_string()));

                symbols.push(Value::Symbol(id));
            }
            vm.stack
                .push(Value::Junction(crate::ast::JunctionType::All, symbols));
            vm.energy = vm.energy.saturating_sub(s.len() as i64);
            vm.output
                .push(format!("DECONSTRUCT: Broken '{}' into symbols", s));
        } else {
            vm.output
                .push("Error: Type mismatch for deconstruct".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for deconstruct".to_string());
    }
    None
}
