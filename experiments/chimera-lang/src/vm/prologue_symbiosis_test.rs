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

    #[test]
    fn test_parasite_rune() {
        use crate::vm::prologue::critter::CritterState;
        let mut vm = setup_vm();

        // Setup: "G" -> ! -> ~ -> p -> C
        // 5,2: "G" (Payload)
        // 5,3: "!" (Source)
        // 5,4: "~" (Wire)
        // 5,5: "p" (Parasite)
        // 5,6: "C" (Target)

        vm.grid[5][2] = Value::Str("G".to_string());
        vm.grid[5][3] = Value::Str("!".to_string());
        vm.grid[5][4] = Value::Str("~".to_string());
        vm.grid[5][5] = Value::Str("p".to_string());
        vm.grid[5][6] = Value::Str("C".to_string());

        // Register Critter
        let critter = CritterState::default(); // genes="R"
        vm.prologue_state.registers.insert((5, 6), critter.to_value());

        exec_prologue_tick(&mut vm);

        // Check Critter state in Registers
        if let Some(val) = vm.prologue_state.registers.get(&(5, 6)) {
            if let Value::Str(s) = val {
                let c: CritterState = s.parse().unwrap();
                // Default "R", injected "G" -> "RG"
                assert!(c.genes.contains('G'), "Critter genes '{}' should contain 'G'", c.genes);
            } else {
                panic!("Critter state invalid");
            }
        } else {
            panic!("Critter register missing");
        }
    }

    #[test]
    fn test_xenograft_rune() {
        use crate::vm::prologue::critter::CritterState;
        let mut vm = setup_vm();

        // Setup: C1 x C2
        // 5,4: "C" (Critter 1)
        // 5,5: "x" (Xenograft)
        // 5,6: "C" (Critter 2)

        vm.grid[5][4] = Value::Str("C".to_string());
        vm.grid[5][5] = Value::Str("x".to_string());
        vm.grid[5][6] = Value::Str("C".to_string());

        let mut c1 = CritterState::default();
        c1.energy = 100;
        c1.genes = "Z".to_string(); // Nop to prevent moving
        let mut c2 = CritterState::default();
        c2.energy = 200;
        c2.genes = "Z".to_string(); // Nop

        vm.prologue_state.registers.insert((5, 4), c1.to_value());
        vm.prologue_state.registers.insert((5, 6), c2.to_value());

        exec_prologue_tick(&mut vm);

        // Check Swap
        // C1 should be at 5,6. C2 at 5,4.

        let reg_at_4 = vm.prologue_state.registers.get(&(5, 4)).unwrap();
        let reg_at_6 = vm.prologue_state.registers.get(&(5, 6)).unwrap();

        // Extract string directly to avoid quotes from Value::Display
        let s4_str = if let Value::Str(s) = reg_at_4 { s } else { panic!("Not a string") };
        let s6_str = if let Value::Str(s) = reg_at_6 { s } else { panic!("Not a string") };

        let s4: CritterState = s4_str.parse().expect("Failed to parse s4");
        let s6: CritterState = s6_str.parse().expect("Failed to parse s6");

        // Check energy (expect 1 tick cost: 200 -> 199, 100 -> 99)
        assert!(s4.energy >= 199, "Critter at West should now be C2 (200 -> 199)");
        assert!(s6.energy >= 99, "Critter at East should now be C1 (100 -> 99)");
    }
}
