use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
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
fn test_necromancy_bury_exhume() {
    let mut vm = setup_vm();

    // 1. Add dummy strands to Helix
    // Strand 0 (Pad)
    vm.dna.helix.strands.push(Strand { genes: vec![] });

    // Strand 1 (Target)
    let gene = Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(100)],
    };
    let strand = Strand { genes: vec![gene] };
    vm.dna.helix.strands.push(strand); // Index 1

    assert_eq!(vm.dna.helix.strands.len(), 2);
    assert!(!vm.dna.helix.strands[1].genes.is_empty());

    // 2. Setup Bury Circuit: 1 -> ! -> †
    // Input West: 1 (Index of strand)
    vm.grid[6][3] = Value::Int(1);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("†".to_string());

    exec_prologue_tick(&mut vm);

    // 3. Verify Bury
    // Strand 1 should be empty (genes cleared)
    assert!(
        vm.dna.helix.strands[1].genes.is_empty(),
        "Strand 1 should be cleared"
    );
    // Graveyard should have 1 strand
    assert_eq!(vm.graveyard.len(), 1, "Graveyard should have 1 strand");
    // Verify content of buried strand
    assert_eq!(vm.graveyard[0].genes.len(), 1);
    if let Nucleotide::Number(n) = &vm.graveyard[0].genes[0].args[0] {
        assert_eq!(*n, 100);
    } else {
        panic!("Buried strand content mismatch");
    }

    // Clean up Bury circuit to prevent re-execution
    vm.grid[6][3] = Value::Int(0);
    vm.grid[6][4] = Value::Int(0);
    vm.grid[6][5] = Value::Int(0);

    // 4. Setup Exhume Circuit: 1 -> ! -> ‡
    // Input West: Any trigger (e.g., 1)
    vm.grid[9][3] = Value::Int(1);
    vm.grid[9][4] = Value::Str("!".to_string());
    vm.grid[9][5] = Value::Str("‡".to_string());

    exec_prologue_tick(&mut vm);

    // 5. Verify Exhume
    // Graveyard should be empty
    assert_eq!(
        vm.graveyard.len(),
        0,
        "Graveyard should be empty after exhume"
    );
    // Helix should have 3 strands (0: pad, 1: cleared, 2: exhumed)
    assert_eq!(vm.dna.helix.strands.len(), 3, "Helix should have 3 strands");
    // Verify content of exhumed strand
    assert_eq!(vm.dna.helix.strands[2].genes.len(), 1);
    if let Nucleotide::Number(n) = &vm.dna.helix.strands[2].genes[0].args[0] {
        assert_eq!(*n, 100);
    } else {
        panic!("Exhumed strand content mismatch");
    }

    // Check South output of Exhume (should be index 2)
    if let Value::Int(idx) = vm.grid[10][5] {
        assert_eq!(idx, 2, "Exhume should output new strand index");
    } else {
        panic!("Exhume did not output index to South");
    }
}

#[test]
fn test_necromancy_seance() {
    let mut vm = setup_vm();

    // 1. Add a dummy strand to Graveyard directly
    let gene = Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(666)],
    };
    let strand = Strand { genes: vec![gene] };
    vm.graveyard.push(strand);

    // 2. Setup Seance Circuit: 1 -> ! -> Ψ
    vm.grid[6][3] = Value::Int(1);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("Ψ".to_string());

    exec_prologue_tick(&mut vm);

    // 3. Verify Seance
    // Should have triggered interrupt to a new ghost strand
    // Helix should have 1 strand (the ghost)
    assert_eq!(
        vm.dna.helix.strands.len(),
        1,
        "Helix should have ghost strand"
    );

    // IP should be set to (0, 0) due to interrupt
    assert_eq!(vm.ip, (0, 0), "IP should be interrupted to ghost strand");

    // Graveyard should still have the strand (Seance doesn't consume)
    assert_eq!(vm.graveyard.len(), 1, "Graveyard should persist");
}
