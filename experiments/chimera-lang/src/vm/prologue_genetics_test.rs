use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::exec_prologue_tick;

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

fn add_strand(vm: &mut ChimeraVM, op: OpCode, val: i64) -> usize {
    let genes = vec![Gene {
        op,
        args: vec![Nucleotide::Number(val)],
    }];
    let strand = Strand { genes };
    vm.dna.helix.strands.push(strand);
    vm.dna.helix.strands.len() - 1
}

#[test]
fn test_breed_rune() {
    let mut vm = make_vm();

    // Strand A: Push(10)
    let idx_a = add_strand(&mut vm, OpCode::Push, 10);
    // Strand B: Push(20)
    let idx_b = add_strand(&mut vm, OpCode::Push, 20);

    // Setup Grid:
    //   50 (Prob)
    //    !
    // A  b  B
    //    S

    // Using direct signal injection for simplicity in test setup
    // b is at (5, 5)
    vm.grid[5][5] = Value::Str("b".to_string());

    // Inputs
    vm.prologue_state.signal_grid[5][4] = Some(Value::Int(idx_a as i64)); // West
    vm.prologue_state.signal_grid[5][6] = Some(Value::Int(idx_b as i64)); // East
    vm.prologue_state.signal_grid[4][5] = Some(Value::Int(0)); // North (Prob 0 -> Pick B)

    // We need to trigger scan_grid_rules to populate runes set
    vm.prologue_state.scan_grid_rules(&vm.grid);

    // Call genetics logic directly or via tick?
    // scan_grid_rules is called in tick.
    // signal_grid is reset in tick.
    // If we set signal_grid manually, we should call apply_genetic_sinks directly or skip prepare_signals.
    // But apply_genetic_sinks is private? No, I made it pub in genetics.rs?
    // Let's check visibility.
    // "pub fn apply_genetic_sinks" -> Yes.

    // However, calling it directly is easier.
    use crate::vm::prologue::genetics::apply_genetic_sinks;
    apply_genetic_sinks(&mut vm, "b", 5, 5);

    // Check Result
    // New strand should be created.
    assert_eq!(vm.dna.helix.strands.len(), 3);

    // Since Prob is 0, should pick B genes.
    let new_strand = &vm.dna.helix.strands[2];
    assert_eq!(new_strand.genes.len(), 1);
    assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(20));

    // Check Output Signal
    if let Some(Value::Int(idx)) = vm.prologue_state.signal_grid[5][5] {
         assert_eq!(idx, 1); // Emits 1 (Signal Activity)
    }

    // Check South Output (New Index)
    if let Value::Int(idx) = vm.grid[6][5] {
        assert_eq!(idx, 2);
    } else {
        panic!("South output not set");
    }
}

#[test]
fn test_clone_rune() {
    let mut vm = make_vm();
    let idx = add_strand(&mut vm, OpCode::Push, 42);

    // c at (5,5)
    vm.grid[5][5] = Value::Str("c".to_string());
    // Input West
    vm.prologue_state.signal_grid[5][4] = Some(Value::Int(idx as i64));

    use crate::vm::prologue::genetics::apply_genetic_sinks;
    apply_genetic_sinks(&mut vm, "c", 5, 5);

    assert_eq!(vm.dna.helix.strands.len(), 2);
    let clone = &vm.dna.helix.strands[1];
    assert_eq!(clone.genes[0].args[0], Nucleotide::Number(42));

    // Check South Output
    if let Value::Int(idx) = vm.grid[6][5] {
        assert_eq!(idx, 1);
    } else {
        panic!("South output not set");
    }
}

#[test]
fn test_inject_rune() {
    let mut vm = make_vm();

    // Target: [ Push(10), Push(20) ]
    let genes_t = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
    ];
    vm.dna.helix.strands.push(Strand { genes: genes_t });
    let idx_target = 0;

    // Payload: [ Push(99) ]
    let idx_payload = add_strand(&mut vm, OpCode::Push, 99);

    // i at (5,5)
    vm.grid[5][5] = Value::Str("i".to_string());

    // Inputs
    vm.prologue_state.signal_grid[5][4] = Some(Value::Int(idx_target as i64)); // West (Target)
    vm.prologue_state.signal_grid[4][5] = Some(Value::Int(idx_payload as i64)); // North (Payload)
    vm.prologue_state.signal_grid[5][6] = Some(Value::Int(1)); // East (Insert Index 1)

    use crate::vm::prologue::genetics::apply_genetic_sinks;
    apply_genetic_sinks(&mut vm, "i", 5, 5);

    // Should have 3 strands now (Original, Payload, New)
    assert_eq!(vm.dna.helix.strands.len(), 3);

    let new_strand = &vm.dna.helix.strands[2];
    assert_eq!(new_strand.genes.len(), 3);
    // [ Push(10), Push(99), Push(20) ]
    assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(10));
    assert_eq!(new_strand.genes[1].args[0], Nucleotide::Number(99));
    assert_eq!(new_strand.genes[2].args[0], Nucleotide::Number(20));
}

#[test]
fn test_hybridize_rune() {
    let mut vm = make_vm();

    // A: [ Push(1), Push(1) ]
    let genes_a = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
    ];
    vm.dna.helix.strands.push(Strand { genes: genes_a });
    let idx_a = 0;

    // B: [ Push(2), Push(2) ]
    let genes_b = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
    ];
    vm.dna.helix.strands.push(Strand { genes: genes_b });
    let idx_b = 1;

    // Mask: [ Nop(), Push(0) ]
    // Nop has no args -> Use A
    // Push(0) has args -> Use B
    let genes_m = vec![
        Gene { op: OpCode::Nop, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
    ];
    vm.dna.helix.strands.push(Strand { genes: genes_m });
    let idx_m = 2;

    // h at (5,5)
    vm.grid[5][5] = Value::Str("h".to_string());

    vm.prologue_state.signal_grid[5][4] = Some(Value::Int(idx_a as i64));
    vm.prologue_state.signal_grid[5][6] = Some(Value::Int(idx_b as i64));
    vm.prologue_state.signal_grid[4][5] = Some(Value::Int(idx_m as i64));

    use crate::vm::prologue::genetics::apply_genetic_sinks;
    apply_genetic_sinks(&mut vm, "h", 5, 5);

    // Result: [ A[0], B[1] ] -> [ Push(1), Push(2) ]
    let new_strand = &vm.dna.helix.strands[3];
    assert_eq!(new_strand.genes.len(), 2);
    assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(1));
    assert_eq!(new_strand.genes[1].args[0], Nucleotide::Number(2));
}
