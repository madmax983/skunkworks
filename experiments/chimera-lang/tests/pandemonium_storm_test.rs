#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM};
    use chimera_lang::vm::pandemonium::apply_storm;

    fn make_vm(len: usize) -> ChimeraVM {
        let mut genes = Vec::new();
        for i in 0..len {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i as i64)],
            });
        }
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_storm_mutation() {
        let mut vm = make_vm(50); // Larger sample size to ensure statistical probability

        // Before: All Push
        for gene in &vm.dna.helix.strands[0].genes {
            assert_eq!(gene.op, OpCode::Push);
        }

        // Apply Storm at index 25 with radius 25
        apply_storm(&mut vm, 0, 25, 25.0);

        let mut changed = false;
        let mut found_elektra = false;
        let mut found_chaos = false;

        for gene in &vm.dna.helix.strands[0].genes {
            if gene.op != OpCode::Push {
                changed = true;

                #[cfg(feature = "elektra")]
                {
                    if matches!(gene.op, OpCode::Lightning | OpCode::Shock | OpCode::TeslaCoil | OpCode::Electrogenesis | OpCode::Induction | OpCode::Battery | OpCode::Ground | OpCode::CircuitBreaker) {
                        found_elektra = true;
                    }
                }

                #[cfg(not(feature = "elektra"))]
                {
                     // If no elektra, check for chaos
                     if matches!(gene.op, OpCode::Chaos | OpCode::Glitch | OpCode::Scramble | OpCode::Disintegrate | OpCode::EntropySurge) {
                        found_chaos = true;
                     }
                }
            }
        }

        assert!(changed, "Storm should mutate at least one gene");

        #[cfg(feature = "elektra")]
        assert!(found_elektra, "Storm with Elektra feature should produce Elektra opcodes");

        #[cfg(all(feature = "nova", not(feature = "elektra")))]
        assert!(found_chaos, "Storm without Elektra (but with Nova) should produce Chaos opcodes");
    }
}
