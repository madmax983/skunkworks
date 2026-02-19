#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_symbiosis_upload() {
        let mut vm = setup_vm();
        // Upload (u): Reads West (Value) -> Pushes to Stack
        // 42 -> ! -> u
        // ! at 6,4 (reading 6,3).
        // u at 6,5 (reading West 6,4).

        vm.grid[6][3] = Value::Int(42);
        vm.grid[6][4] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("u".to_string());

        exec_prologue_tick(&mut vm);

        // Check Stack
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(42));
    }

    #[test]
    fn test_symbiosis_yank() {
        let mut vm = setup_vm();
        // Yank (y): Reads West (Trigger) -> Pops Stack -> Writes South
        // 1 -> ! -> y
        // ! at 6,4 (reads 6,3).
        // y at 6,5.
        // Grid South at 7,5.

        // Pre-fill stack
        vm.stack.push(Value::Int(99));

        vm.grid[6][3] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("y".to_string());

        exec_prologue_tick(&mut vm);

        // Check Stack Empty
        assert_eq!(vm.stack.len(), 0);
        // Check Grid Output
        assert_eq!(vm.grid[7][5], Value::Int(99));
    }

    #[cfg(feature = "nova")]
    #[test]
    fn test_symbiosis_akashic() {
        let mut vm = setup_vm();
        // Write (w): West (Val), North (Key) -> Akashic
        // Val: 100
        // Key: "score"
        // w at 6,5.
        // West Source: ! at 6,4. Reads 6,3.
        // North Source: ! at 5,5. Reads 5,4.

        vm.grid[6][3] = Value::Int(100);
        vm.grid[6][4] = Value::Str("!".to_string());

        vm.grid[5][4] = Value::Str("score".to_string());
        vm.grid[5][5] = Value::Str("!".to_string());

        vm.grid[6][5] = Value::Str("w".to_string());

        exec_prologue_tick(&mut vm);

        // Check Akashic
        if let Some(val) = vm.akashic.storage.get("score") {
            assert_eq!(*val, Value::Int(100));
        } else {
            panic!("Akashic write failed");
        }

        // Read (j): North (Key) -> Grid South
        // Key: "score"
        // j at 10,5.
        // North Source: ! at 9,5. Reads 9,4.

        vm.grid[9][4] = Value::Str("score".to_string());
        vm.grid[9][5] = Value::Str("!".to_string());
        vm.grid[10][5] = Value::Str("j".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(vm.grid[11][5], Value::Int(100));
    }
}
