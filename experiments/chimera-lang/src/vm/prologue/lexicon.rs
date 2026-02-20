use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

/// Scans the grid for "Spells" (words) and casts them.
///
/// This function looks for known keywords in the grid (horizontally and vertically).
/// If a spell is found, it consumes the letters (turns them to 0) and executes the corresponding OpCode.
pub fn process_lexicon(vm: &mut ChimeraVM) {
    let mut spells_to_cast = Vec::new();

    // Dictionary of Spells
    // Word -> OpCode
    let dictionary = vec![
        ("FIRE", OpCode::Fire),
        ("HEAL", OpCode::Photosynthesize),
        ("VOID", OpCode::Void),
        ("LIFE", OpCode::Spawn),
        ("DIE", OpCode::Apoptosis),
        ("WARP", OpCode::QuantumJump),
        ("TIME", OpCode::TimeWarp),
        ("SOUL", OpCode::Spirit),
        ("CHAOS", OpCode::Chaos),
        ("ORDER", OpCode::Stabilize),
        ("GROW", OpCode::Grow),
        ("HUNT", OpCode::Track),
        ("SEEK", OpCode::Seek),
        ("SCAN", OpCode::Scan),
        ("ECHO", OpCode::Sonar),
        ("BOMB", OpCode::Supernova),
        ("NULL", OpCode::Nop),
        ("LOVE", OpCode::Bond),
        ("HATE", OpCode::Unbond),
        ("READ", OpCode::GRead),
        ("SAVE", OpCode::AkashicSave),
        ("LOAD", OpCode::AkashicLoad),
        ("DATA", OpCode::Genome),
        ("WISH", OpCode::Miracle),
    ];

    // 1. Horizontal Scan
    for y in 0..GRID_SIZE {
        let row_str: String = (0..GRID_SIZE)
            .map(|x| match &vm.grid[y][x] {
                Value::Str(s) => s.chars().next().unwrap_or('.'),
                _ => '.',
            })
            .collect();

        for (word, op) in &dictionary {
            // Find ALL occurrences, not just first
            let mut start = 0;
            while let Some(idx) = row_str[start..].find(word) {
                let real_idx = start + idx;
                spells_to_cast.push((op.clone(), y, real_idx, word.len(), true));
                start = real_idx + word.len();
            }
        }
    }

    // 2. Vertical Scan
    for x in 0..GRID_SIZE {
        let col_str: String = (0..GRID_SIZE)
            .map(|y| match &vm.grid[y][x] {
                Value::Str(s) => s.chars().next().unwrap_or('.'),
                _ => '.',
            })
            .collect();

        for (word, op) in &dictionary {
            let mut start = 0;
            while let Some(idx) = col_str[start..].find(word) {
                let real_idx = start + idx;
                spells_to_cast.push((op.clone(), real_idx, x, word.len(), false));
                start = real_idx + word.len();
            }
        }
    }

    // 3. Cast Spells
    for (op, y, x, len, horizontal) in spells_to_cast {
        // Verify spell is still there (could be consumed by overlapping spell)
        // For simplicity, we allow overlap consumption or double-consumption if intersection.
        // But strict "Consumption" means we should check.
        // Let's just burn it all.

        // Visual Feedback
        vm.output.push(format!("LEXICON: Casting {} at {},{}", op, x, y));

        // Consume Reagents (Turn to 0)
        if horizontal {
            for i in 0..len {
                vm.grid[y][x + i] = Value::Int(0);
            }
        } else {
            for i in 0..len {
                vm.grid[y + i][x] = Value::Int(0);
            }
        }

        // Execute Effect
        // For now, no args. Future: Parse args from grid?
        vm.execute_gene_inner(op, &[]);
    }
}
