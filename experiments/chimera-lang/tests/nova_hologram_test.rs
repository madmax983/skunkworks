#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_hologram_interfere() {
        // [ Push(10) ] - Strand 0 (Dummy)
        // [ Interfere(0) ] - Strand 1 (Execution)
        let strand0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };

        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Interfere,
                args: vec![Nucleotide::Number(0)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute Strand 1
        vm.ip = (1, 0);
        vm.step();

        // Check grid is not empty
        let mut nonzero = false;
        for row in &vm.hologram_grid {
            for cell in row {
                if cell.0 != 0.0 || cell.1 != 0.0 {
                    nonzero = true;
                    break;
                }
            }
        }
        assert!(nonzero, "Hologram grid should contain interference pattern");
    }

    #[test]
    fn test_hologram_refract() {
        let strand0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };

        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Interfere,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Refract,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute Strand 1: Interfere
        vm.ip = (1, 0);
        vm.step();

        // Execute Refract
        vm.step();

        // Should have created a new strand (Index 2)
        // Strand 0: Original
        // Strand 1: Execution
        // Strand 2: Refracted
        assert_eq!(vm.dna.helix.strands.len(), 3);

        let new_strand = &vm.dna.helix.strands[2];
        assert!(!new_strand.genes.is_empty());
        assert_eq!(new_strand.genes[0].op, OpCode::Push);
    }

    #[test]
    fn test_hologram_project() {
        let mut vm = make_vm();

        // Add dummy strand to prevent immediate halt
        vm.dna.helix.strands.push(Strand {
            genes: vec![Gene {
                op: OpCode::Project,
                args: vec![],
            }],
        });

        // Manually set some hologram values
        vm.hologram_grid[8][8] = (10.0, 0.0); // High magnitude

        vm.ip = (0, 0);
        vm.step();

        // Grid should have value
        if let Value::Int(n) = vm.grid[8][8] {
            assert!(n > 0, "Project should write to grid");
        } else {
            panic!("Grid should contain Int");
        }
    }
}
