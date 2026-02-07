#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value, GRID_SIZE};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_claim() {
        // [ claim() ]
        let genes = vec![
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Initial state
        let (cy, cx) = vm.context_loc;
        assert_eq!(vm.sovereignty_grid[cy][cx], None);

        // Execute Claim
        vm.step();

        // Check ownership
        assert_eq!(vm.sovereignty_grid[cy][cx], Some(0));

        // Check energy cost (initial 50 - 1 tick - 10 claim = 39)
        assert_eq!(vm.energy, 39);
    }

    #[test]
    fn test_tax() {
        // [ claim() tax() ]
        let genes = vec![
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
            Gene {
                op: OpCode::Tax,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Claim (Cost 10 + 1 tick)
        vm.step();
        assert_eq!(vm.energy, 39);

        // Tax (Cost 5 + 1 tick, Gain 1 for 1 cell) -> Net -5
        vm.step();

        // Expected: 39 - 1 (tick) - 5 (tax base) + 1 (tax revenue) = 34
        assert_eq!(vm.energy, 34);

        // Stack should have gain (1)
        assert_eq!(vm.stack.last(), Some(&Value::Int(1)));
    }

    #[test]
    fn test_sovereignty_check() {
        // [ claim() sovereignty() ]
        let genes = vec![
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
            Gene {
                op: OpCode::Sovereignty,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Claim
        vm.step(); // Sovereignty

        assert_eq!(vm.stack.last(), Some(&Value::Int(0)));
    }

    #[test]
    fn test_conquest() {
        // Strand 0 claims. Strand 1 overwrites.
        // DNA: [ claim() ] [ claim() ]
        let s0 = Strand {
            genes: vec![Gene { op: OpCode::Claim, args: vec![] }]
        };
        let s1 = Strand {
            genes: vec![Gene { op: OpCode::Claim, args: vec![] }]
        };
        let dna = Dna { helix: Helix { strands: vec![s0, s1] } };
        let mut vm = ChimeraVM::new(dna);

        // Exec Strand 0
        vm.step();
        let (cy, cx) = vm.context_loc;
        assert_eq!(vm.sovereignty_grid[cy][cx], Some(0));

        // Manually switch to Strand 1 (normally happens sequentially if 0 finishes, but let's force ip)
        vm.ip = (1, 0);
        vm.step();

        assert_eq!(vm.sovereignty_grid[cy][cx], Some(1));
    }
}
