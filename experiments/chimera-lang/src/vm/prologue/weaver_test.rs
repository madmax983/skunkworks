use super::super::*;
use crate::ast::{Dna, Helix};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_weaver_synthesis() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Circuit on Grid
    // Blueprint Rune B at (5,5)

    // Inputs for Blueprint using Sources (!)
    // Trigger: West of B is (5,4). Source at (5,4) needs 1 at (5,3).
    vm.grid[5][3] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    // Height: North of B is (4,5). Source at (4,5) needs 1 at (3,5). Wait, ! reads West.
    // So Source at (4,5) reads (4,4).
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string());

    // Width: South of B is (6,5). Source at (6,5) reads (6,4).
    vm.grid[6][4] = Value::Int(3);
    vm.grid[6][5] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("B".to_string());

    // Circuit Content (Grid Memory)
    // Area: y=5, x=6..9
    vm.grid[5][6] = Value::Int(10);
    vm.grid[5][7] = Value::Int(20);
    vm.grid[5][8] = Value::Str("A".to_string()); // Add

    // 2. Run Blueprint Tick
    // prepare_signals will fire !, activating B inputs.
    // propagate will run B. B will write to delayed_signals at (5,5).
    prologue::exec_prologue_tick(&mut vm);

    // 3. Place Weaver
    // Weaver at (5,6) reads West (5,5).
    // We overwrite 10 with Weaver. Note: Blueprint captured 10 in previous tick (hopefully).
    // B captures *grid* state. B runs in `propagate`.
    // Does B capture current grid state? `construct.rs`: `rows.push(grid[ny][nx].clone());`
    // It reads `grid` passed to `process_signal_propagation`.
    // This grid contains `10`.
    // So Blueprint should contain `10`.

    // Now we overwrite `10` with Weaver.
    vm.grid[5][6] = Value::Str("🕷".to_string());
    // Also clear previous B triggers to stop it from re-firing (optional)
    vm.grid[5][3] = Value::Int(0);

    // 4. Run Weaver Tick
    // prepare_signals moves delayed_signals (Blueprint) to signal_grid.
    // process_agents runs Weaver. Weaver reads signal_grid at (5,5).
    prologue::exec_prologue_tick(&mut vm);

    // 5. Verify DNA
    // Check output for debugging
    for line in &vm.output {
        println!("{}", line);
    }

    assert_eq!(
        vm.dna.helix.strands.len(),
        1,
        "Weaver should have synthesized a strand"
    );

    let strand = &vm.dna.helix.strands[0];
    assert_eq!(strand.genes.len(), 3);

    assert_eq!(strand.genes[0].op, OpCode::Push);
    assert_eq!(strand.genes[0].args[0], crate::ast::Nucleotide::Number(10));

    assert_eq!(strand.genes[1].op, OpCode::Push);
    assert_eq!(strand.genes[1].args[0], crate::ast::Nucleotide::Number(20));

    assert_eq!(strand.genes[2].op, OpCode::Add);
}
