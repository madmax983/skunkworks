#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value, MAX_STRANDS};

    fn setup_vm_with_strands(count: usize) -> ChimeraVM {
        let mut strands = Vec::new();
        for _ in 0..count {
            strands.push(Strand {
                genes: vec![Gene {
                    op: OpCode::Nop,
                    args: vec![],
                }],
            });
        }
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_unbounded_strand_growth_via_x_rune() {
        let mut vm = setup_vm_with_strands(1);

        // X at (5,5)
        vm.grid[5][5] = Value::Str("X".to_string());

        let mut exceeded = false;

        // Loop to generate strands
        // We inject signals manually to ensure X fires every tick
        for _ in 0..(MAX_STRANDS + 100) {
            // Inject signals for X inputs
            // West (5,4) and East (5,6)
            vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(0));
            vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(0));

            exec_prologue_tick(&mut vm);

            if vm.dna.helix.strands.len() > MAX_STRANDS {
                exceeded = true;
                break;
            }
        }

        // Warden Check: If exceeded is true, the vulnerability exists.
        // We assert !exceeded to verify the fix.
        assert!(
            !exceeded,
            "Strands count {} exceeded limit {}",
            vm.dna.helix.strands.len(),
            MAX_STRANDS
        );

        // Also check if we hit the limit
        // We expect it to be MAX_STRANDS
        assert_eq!(
            vm.dna.helix.strands.len(),
            MAX_STRANDS,
            "Should have reached MAX_STRANDS but stopped there"
        );
    }
}
