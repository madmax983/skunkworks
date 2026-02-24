use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_alchemist_transmutation() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5,5) Alchemist
    // (5,6) Int(10) (East)
    // (5,7) Int(20) (East of 10)

    vm.grid[5][5] = Value::Str("⚗".to_string());
    vm.grid[5][6] = Value::Int(10);
    vm.grid[5][7] = Value::Int(20);

    // Tick 1: Alchemist scans grid, initializes.
    exec_prologue_tick(&mut vm);

    // Alchemist should be at 5,5.
    if let Value::Str(s) = &vm.grid[5][5] {
        assert_eq!(s, "⚗");
    }

    // Tick 2: Alchemist moves East (Default dir 1), Gathers 10.
    exec_prologue_tick(&mut vm);

    // Alchemist should be at 5,6. Grid[5,5] empty.
    assert_eq!(vm.grid[5][5], Value::Int(0));
    if let Value::Str(s) = &vm.grid[5][6] {
        assert_eq!(s, "⚗");
    }

    // Tick 3: Alchemist moves East, Gathers 20.
    // Crucible has [10, 20]. Recipe [Int, Int] -> Sum (30).
    // Transmutation happens. Result (30) ejected behind (West -> 5,5).
    // Alchemist is at 5,7.
    exec_prologue_tick(&mut vm);

    // Check Position
    if let Value::Str(s) = &vm.grid[5][7] {
        assert_eq!(s, "⚗");
    }

    // Check Result at 5,6? No, ejected behind.
    // Alchemist moved 5,6 -> 5,7. Behind is West of 5,7 which is 5,6.
    // Wait, Alchemist moved from 5,6 to 5,7.
    // Before move, it was at 5,6. It gathered 20 from 5,7?
    // Let's trace `process_alchemist_logic`.
    // It checks recipes FIRST.
    // Then scans/moves.

    // Tick 2 (End): Alchemist at 5,6. Crucible: [10].
    // Tick 3 (Start):
    // 1. Check Recipe: [10]. No.
    // 2. Scan: Look East (5,7). See 20.
    // 3. Gather 20. Crucible: [10, 20]. Grid[5,7] becomes 0.
    // 4. Move to 5,7.

    // Tick 3 (End): Alchemist at 5,7. Crucible: [10, 20].

    // Tick 4 (Start):
    // 1. Check Recipe: [10, 20]. YES -> Sum 30.
    // 2. Eject Result. Direction is East (1). Behind is West (0, -1).
    //    Target: (5, 7-1) = (5,6).
    //    Is (5,6) empty? Yes (Alchemist left it).
    //    Write 30 to (5,6). Clear Crucible.
    // 3. Scan: Look East (5,8). Empty/Wall. Move or Turn.

    exec_prologue_tick(&mut vm);

    // Check Result at 5,6
    if let Value::Int(n) = &vm.grid[5][6] {
        assert_eq!(*n, 30);
    } else {
        panic!("Expected result 30 at 5,6, got {:?}", vm.grid[5][6]);
    }
}
