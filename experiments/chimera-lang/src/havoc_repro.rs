#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix, Strand, Gene};

    fn make_bomb_dna() -> Dna {
        // Construct a strand that pushes coordinates and then calls virus
        // We want to fill the stack with (0,0) first.
        // But doing it via DNA genes is slow (one step per push).
        // We can manually fill the stack in the test setup.

        let genes = vec![
            Gene {
                name: "virus".to_string(),
                args: vec![],
            }
        ];

        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_stack_overflow() {
        // 🧨 Havoc: Trigger recursion bomb
        let mut vm = ChimeraVM::new(make_bomb_dna());

        // Setup grid with "virus" at (0,0)
        vm.grid[0][0] = Value::Str("virus".to_string());

        // Push args for virus recursion: (0, 0)
        // Each "virus" call pops 2 args and recurses.
        // We want to overflow the stack.
        // 50,000 pairs = 100,000 items.
        for _ in 0..50_000 {
            vm.stack.push(Value::Int(0)); // y
            vm.stack.push(Value::Int(0)); // x
        }

        println!("Starting recursion bomb...");
        // This calls the first gene "virus".
        // "virus" pops (0,0), reads grid[0][0] -> "virus".
        // It calls execute_gene("virus").
        // Which pops (0,0), reads grid[0][0] -> "virus".
        // Recursion!
        vm.step();

        println!("Survived recursion bomb!");
    }
}
