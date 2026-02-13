#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    fn swap_organelle_context(vm: &mut ChimeraVM, org: &mut crate::vm::nova::Organelle) {
        std::mem::swap(&mut vm.stack, &mut org.stack);
        std::mem::swap(&mut vm.ip, &mut org.ip);
        std::mem::swap(&mut vm.context_loc, &mut org.context_loc);
        std::mem::swap(&mut vm.call_stack, &mut org.call_stack);
        std::mem::swap(&mut vm.recursion_depth, &mut org.recursion_depth);
    }

    #[test]
    fn test_plant_sowing() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("F=F[+F]F".to_string()));
        vm.stack.push(Value::Str("F".to_string()));
        crate::vm::nova_botany::exec_plant(&mut vm);

        assert_eq!(vm.organelles.len(), 1);
        let org = &vm.organelles[0];
        assert_eq!(org.kind, crate::vm::nova::OrganelleType::Seed);
        assert_eq!(org.stack.len(), 4);
    }

    #[test]
    fn test_plant_drawing() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("X=F".to_string()));
        vm.stack.push(Value::Str("X".to_string()));
        crate::vm::nova_botany::exec_plant(&mut vm);

        let mut org = vm.organelles.pop().unwrap();

        // Tick 1: X -> F
        swap_organelle_context(&mut vm, &mut org);
        crate::vm::nova_botany::tick_seed(&mut vm, &mut org);
        swap_organelle_context(&mut vm, &mut org);

        if let Value::Str(s) = &org.stack[1] {
             assert_eq!(s, "F");
        }

        // Tick 2: F -> Draw
        swap_organelle_context(&mut vm, &mut org);
        let (y, x) = vm.context_loc;
        crate::vm::nova_botany::tick_seed(&mut vm, &mut org);

        match &vm.grid[y][x] {
            Value::Str(s) => assert_eq!(s, "#"),
            _ => panic!("Expected # on grid, got {:?}", vm.grid[y][x]),
        }
        assert_ne!(vm.context_loc, (y, x));
        swap_organelle_context(&mut vm, &mut org);
    }

    #[test]
    fn test_plant_singing() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("X=♪".to_string()));
        vm.stack.push(Value::Str("X".to_string()));
        crate::vm::nova_botany::exec_plant(&mut vm);

        let mut org = vm.organelles.pop().unwrap();

        // Tick 1: X -> ♪
        swap_organelle_context(&mut vm, &mut org);
        crate::vm::nova_botany::tick_seed(&mut vm, &mut org);
        swap_organelle_context(&mut vm, &mut org);

        // Tick 2: Interpret ♪
        swap_organelle_context(&mut vm, &mut org);
        crate::vm::nova_botany::tick_seed(&mut vm, &mut org);
        swap_organelle_context(&mut vm, &mut org);

        assert!(!vm.chorus_buffer.is_empty());
        let note = vm.chorus_buffer.pop_front().unwrap();
        // Verify it is a valid note
        assert!(["Do", "Re", "Mi", "Fa", "Sol", "La", "Si"].contains(&note.as_str()));
    }

    #[test]
    fn test_plant_resonance_boost() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("X=F".to_string()));
        vm.stack.push(Value::Str("X".to_string()));
        crate::vm::nova_botany::exec_plant(&mut vm);

        let mut org = vm.organelles.pop().unwrap();

        // Set Resonance at (8,8)
        vm.resonance_grid[8][8] = (440.0, 1.0); // Amp 1.0 > 0.5

        swap_organelle_context(&mut vm, &mut org);
        // Should execute 2 ticks worth: X->F then F->Draw
        crate::vm::nova_botany::tick_seed(&mut vm, &mut org);

        let (y, x) = vm.context_loc;

        // Check grid - should be drawn immediately
        match &vm.grid[8][8] { // Note: interpret_char writes to CURRENT location (8,8) then MOVES.
            Value::Str(s) => assert_eq!(s, "#"),
            _ => panic!("Expected # on grid at 8,8 due to boost, got {:?}", vm.grid[8][8]),
        }

        swap_organelle_context(&mut vm, &mut org);
    }
}
