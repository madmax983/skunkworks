#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_piet_push_out() {
        // Setup DNA: [ Push(5), Piet ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Piet,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Setup ChromaGrid
        for x in 0..10 {
            vm.chroma_grid[0][x].fg = Some((255, 192, 192)); // Red Light
        }
        vm.chroma_grid[0][10].fg = Some((255, 0, 0)); // Red Normal
        vm.chroma_grid[0][11].fg = Some((192, 0, 192)); // Magenta Dark
        vm.chroma_grid[0][12].fg = Some((0, 0, 0)); // Black

        // Execute Push(5)
        vm.step();
        // Execute Piet
        vm.step();

        println!("VM Output: {:?}", vm.output);

        // Check Result
        // Stack should have 10.
        // If it looped, it might have more stuff.
        // We look for the LAST pushed value being 10?
        // Or check if stack contains 10.
        assert!(vm.stack.contains(&Value::Int(10)));
    }

    #[test]
    fn test_piet_add() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Piet,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // 1. Red Light (Size 2)
        vm.chroma_grid[0][0].fg = Some((255, 192, 192));
        vm.chroma_grid[0][1].fg = Some((255, 192, 192));

        // 2. Red Normal (Size 3) -> Push 2
        vm.chroma_grid[0][2].fg = Some((255, 0, 0));
        vm.chroma_grid[0][3].fg = Some((255, 0, 0));
        vm.chroma_grid[0][4].fg = Some((255, 0, 0));

        // 3. Red Dark (Size 1) -> Push 3
        vm.chroma_grid[0][5].fg = Some((192, 0, 0));

        // 4. Yellow Dark (Size 1) -> Add
        vm.chroma_grid[0][6].fg = Some((192, 192, 0));

        // 5. Red Light (Size 1) -> Out(Number)
        vm.chroma_grid[0][7].fg = Some((255, 192, 192));

        // 6. Black
        vm.chroma_grid[0][8].fg = Some((0, 0, 0));

        vm.step(); // Push 10
        vm.step(); // Piet

        println!("VM Output: {:?}", vm.output);

        assert!(vm.stack.contains(&Value::Int(5)));
    }
}
