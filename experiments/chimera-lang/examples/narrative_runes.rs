use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;

fn main() {
    println!("🗣️ Echo's Narrative Rune Audit");
    println!("-------------------------------");

    let dna = Dna {
        helix: Helix { strands: vec![] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Incipit (α): Seed -> Theme
    println!("\n📜 1. Testing Incipit (α)...");
    // Layout:
    // (5,3): ! (Source) reading 5 from (5,2)
    // (5,4): ~ (Wire)
    // (5,5): α (Incipit) reading Signal at (5,4)
    vm.grid[5][2] = Value::Int(5);
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("~".to_string());
    vm.grid[5][5] = Value::Str("α".to_string());

    exec_prologue_tick(&mut vm);

    let theme = vm.prologue_state.signal_grid[5][6].clone();
    if let Some(val) = &theme {
        println!("✅ Incipit generated: {:?}", val);
    } else {
        println!("❌ Incipit failed.");
    }

    // 2. Terminus (ω): Story -> Outcome
    println!("\n📜 2. Testing Terminus (ω)...");
    // Layout:
    // (6,3): ! (Source) reading "Hero Shadow" from (6,2)
    // (6,4): ~
    // (6,5): ω (Terminus)
    vm.grid[6][2] = Value::Str("Hero Shadow".to_string());
    vm.grid[6][3] = Value::Str("!".to_string());
    vm.grid[6][4] = Value::Str("~".to_string());
    vm.grid[6][5] = Value::Str("ω".to_string());

    exec_prologue_tick(&mut vm);

    if let Some(val) = &vm.prologue_state.signal_grid[6][6] {
        println!("✅ Terminus generated: {:?}", val);
    } else {
        println!("❌ Terminus failed.");
    }

    // 3. Twist (?): Story -> Twist
    println!("\n📜 3. Testing Twist (?)....");
    // Layout:
    // (7,3): ! reading "Dream"
    // (7,4): ~
    // (7,5): ? (Twist)
    vm.grid[7][2] = Value::Str("Dream".to_string());
    vm.grid[7][3] = Value::Str("!".to_string());
    vm.grid[7][4] = Value::Str("~".to_string());
    vm.grid[7][5] = Value::Str("?".to_string());

    exec_prologue_tick(&mut vm);

    // Check East (7,6) for Twisted Story
    if let Some(val) = &vm.prologue_state.signal_grid[7][6] {
        println!("✅ Twist generated: {:?}", val);
    } else {
        println!("❌ Twist failed (or ? is acting only as Sink).");
    }

    // 4. Revision (✍): Story + Mode -> Edited
    println!("\n📜 4. Testing Revision (✍)...");
    // Layout:
    // West (8,4): "hello" (via ! at 8,4 reading 8,3)
    vm.grid[8][3] = Value::Str("hello".to_string());
    vm.grid[8][4] = Value::Str("!".to_string());

    // North (7,5): Mode "up" (via ! at 7,5 reading 7,4)
    // Note: (7,5) was occupied by ? in previous step, but we are overwriting it.
    // Wait, (7,5) was ?.
    // We want Revision at (8,5). It reads North (7,5).
    // If we overwrite (7,5) with !, it reads West (7,4).
    // (7,4) was ~ in Step 3.
    // We overwrite (7,4) with "up".
    vm.grid[7][4] = Value::Str("up".to_string());
    vm.grid[7][5] = Value::Str("!".to_string());

    // Rune at (8,5)
    vm.grid[8][5] = Value::Str("✍".to_string());

    exec_prologue_tick(&mut vm);

    if let Some(val) = &vm.prologue_state.signal_grid[8][6] {
        println!("✅ Revision generated: {:?}", val);
    } else {
        println!("❌ Revision failed.");
    }

    // 5. Library (📖): Write
    println!("\n📜 5. Testing Library (📖)...");
    // Layout:
    // Library at (10,5).

    // West (10,4): Key "Epic"
    vm.grid[10][3] = Value::Str("Epic".to_string());
    vm.grid[10][4] = Value::Str("!".to_string());

    // North (9,5): Mode 1
    // (9,5) was South of Revision in Step 4. Empty?
    // Use ! at (9,5) reading (9,4)
    vm.grid[9][4] = Value::Int(1);
    vm.grid[9][5] = Value::Str("!".to_string());

    // South (11,5): Value "War and Peace"
    // Use ! at (11,5) reading (11,4)
    vm.grid[11][4] = Value::Str("War and Peace".to_string());
    vm.grid[11][5] = Value::Str("!".to_string());

    // Rune
    vm.grid[10][5] = Value::Str("📖".to_string());

    exec_prologue_tick(&mut vm);

    if let Some(val) = vm.prologue_state.library.get("Epic") {
        println!("✅ Library Wrote: {:?}", val);
    } else {
        println!("❌ Library Write Failed.");
    }

    println!("\n✨ Audit Complete.");
}
