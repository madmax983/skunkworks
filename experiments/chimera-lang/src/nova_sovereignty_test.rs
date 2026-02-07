#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_claim_territory() {
        // [ push(1) claim() ] -> Claims radius 1 at start location (8,8)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Claim,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // push
        vm.step(); // claim

        // Check output
        assert!(vm.output.iter().any(|s| s.contains("CLAIM: Claimed")));

        // Check ownership at center (8,8)
        assert_eq!(vm.sovereignty.get_owner(8, 8), Some(0));
        // Check ownership at neighbor (8,9)
        assert_eq!(vm.sovereignty.get_owner(8, 9), Some(0));
        // Check outside (8,10) (radius 1: dist^2 <= 1. (8,10) dist=2, dist^2=4 > 1)
        assert_eq!(vm.sovereignty.get_owner(8, 10), None);
    }

    #[test]
    fn test_taxation_allows_owner() {
        // Owner claims and sets tax. Moves freely.
        // [ push(1) claim() push(100) tax() push(0) push(1) migrate() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Claim, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Tax, args: vec![] },
            // Move East (0, 1) -> push(0) push(1)
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Migrate, args: vec![] },
        ];
        let mut vm = make_vm(genes);

        // Give lots of energy so migrate cost doesn't kill it
        vm.energy = 1000;

        vm.step(); // push 1
        vm.step(); // claim
        vm.step(); // push 100
        vm.step(); // tax
        vm.step(); // push 0
        vm.step(); // push 1
        vm.step(); // migrate

        assert_eq!(vm.context_loc, (8, 9)); // Moved successfully
    }

    #[test]
    fn test_taxation_blocks_poor_stranger() {
        // Strand 0 Claims (8,8) r=5. Tax=50.
        // Strand 1 Tries to move from (8,7) to (8,8).
        // Strand 1 has 0 credits.

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
                Gene { op: OpCode::Claim, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
                Gene { op: OpCode::Tax, args: vec![] },
            ]
        };

        let strand1 = Strand {
            genes: vec![
                // Move East (0, 1) into (8,8) -> push(0) push(1)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Migrate, args: vec![] },
            ]
        };

        let dna = Dna { helix: Helix { strands: vec![strand0, strand1] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        // Run Strand 0 setup
        vm.ip = (0, 0);
        vm.step(); vm.step(); // Claim
        vm.step(); vm.step(); // Tax

        // Setup Strand 1
        vm.ip = (1, 0);
        vm.context_loc = (8, 7); // Adjacent

        // Move
        vm.step(); // push
        vm.step(); // push
        vm.step(); // migrate

        // Should be blocked
        assert_eq!(vm.context_loc, (8, 7)); // Did not move
        assert!(vm.output.iter().any(|s| s.contains("Insufficient funds")));
    }

    #[test]
    fn test_taxation_allows_rich_stranger() {
        // Strand 0 Claims. Tax=50.
        // Strand 1 (Rich) Moves.

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
                Gene { op: OpCode::Claim, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
                Gene { op: OpCode::Tax, args: vec![] },
            ]
        };

        let strand1 = Strand {
            genes: vec![
                // Invest 100 energy into credits
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
                Gene { op: OpCode::Invest, args: vec![] },
                // Move East -> push(0) push(1)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Migrate, args: vec![] },
            ]
        };

        let dna = Dna { helix: Helix { strands: vec![strand0, strand1] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 2000;

        // Run Strand 0 setup
        vm.ip = (0, 0);
        vm.step(); vm.step(); // Claim
        vm.step(); vm.step(); // Tax

        // Setup Strand 1
        vm.ip = (1, 0);
        vm.context_loc = (8, 7);

        vm.step(); vm.step(); // Invest 100

        // Verify wallet
        assert_eq!(vm.market.get_balance(1), 100);

        // Move
        vm.step(); vm.step(); // push args
        vm.step(); // migrate

        // Should move
        assert_eq!(vm.context_loc, (8, 8));

        // Verify transaction
        // Strand 1 paid 50. Balance 50.
        assert_eq!(vm.market.get_balance(1), 50);
        // Strand 0 received 50. Balance 50.
        assert_eq!(vm.market.get_balance(0), 50);
    }

    #[test]
    fn test_grant_bypass() {
        // Strand 0 Claims. Tax=1000 (Unaffordable). Grants Strand 1.
        // Strand 1 Moves (Poor).

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
                Gene { op: OpCode::Claim, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1000)] },
                Gene { op: OpCode::Tax, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Grant to Strand 1
                Gene { op: OpCode::Grant, args: vec![] },
            ]
        };

        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Migrate, args: vec![] },
            ]
        };

        let dna = Dna { helix: Helix { strands: vec![strand0, strand1] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 2000;

        // Run Strand 0
        vm.ip = (0, 0);
        for _ in 0..6 { vm.step(); } // Claim, Tax, Grant

        // Run Strand 1
        vm.ip = (1, 0);
        vm.context_loc = (8, 7);
        vm.step(); vm.step(); vm.step(); // Move

        // Should move freely
        assert_eq!(vm.context_loc, (8, 8));
    }
}
