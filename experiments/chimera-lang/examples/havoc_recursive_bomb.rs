use chimera_lang::prelude::*;
use chimera_lang::vm::Value;
use chimera_lang::ast::JunctionType;

fn main() {
    println!("👺 Havoc: Initializing Recursive Depth Charge...");

    // 1. Setup VM
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 2. Construct the "Ouroboros" Circuit
    // This circuit creates a feedback loop where a value is wrapped in a list
    // and written back to itself every tick, growing exponentially in depth.

    // Components:
    // (4,4): 1 (Accumulator Seed)
    // (5,4): ! (Source) - Reads (4,4), emits signal.
    // (6,4): [ (Collect) - Reads (5,4), wraps signal in Junction([sig]).
    // Wire Chain: Transports signal from (6,4) to (3,3).
    // (3,3): ~ (Wire End)
    // (3,4): $ (Scribe) - Reads West (3,3), writes South (4,4).

    // Coordinates:
    let acc_pos = (4, 4);

    vm.grid[4][4] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[6][4] = Value::Str("[".to_string());

    // Wire path
    vm.grid[6][3] = Value::Str("~".to_string());
    vm.grid[6][2] = Value::Str("~".to_string());
    vm.grid[5][2] = Value::Str("~".to_string());
    vm.grid[4][2] = Value::Str("~".to_string());
    vm.grid[3][2] = Value::Str("~".to_string());
    vm.grid[3][3] = Value::Str("~".to_string());

    vm.grid[3][4] = Value::Str("$".to_string());

    println!("👺 Havoc: Circuit Constructed. Detonating...");

    // 3. Run circuit logic
    // Even if the circuit is resilient (due to execution order), the vulnerability
    // exists in the Value type itself.
    let mut ticks = 0;
    loop {
        vm.step();
        ticks += 1;

        // Break early if it's not exploding automatically
        if ticks > 1000 {
            break;
        }
    }

    // 4. Manual Detonation
    // If the Prologue won't build the bomb, we'll build it by hand to prove the vulnerability.
    println!("👺 Havoc: Prologue circuit resilience exceeded. Engaging manual override...");

    let mut bomb = Value::Int(1);
    for _ in 0..20000 {
        bomb = Value::Junction(JunctionType::Any, vec![bomb]);
    }

    println!("👺 Havoc: Bomb constructed (Depth 20000).");
    println!("👺 Havoc: Detonating (Printing)...");

    // This should Stack Overflow because Value::fmt is recursive
    println!("{}", bomb);
}
