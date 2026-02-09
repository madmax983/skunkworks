#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::opcode::OpCode;
    use crate::ast::{Dna, Helix, Strand};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![Strand { genes: vec![] }] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_glitch_mechanics() {
        let mut vm = make_vm();

        // Initial state
        assert_eq!(vm.glitch_level, 0.0);

        // Execute Glitch(10)
        // stack: severity
        vm.stack.push(Value::Int(10));
        crate::vm::nova::exec_nova_op(&mut vm, OpCode::Glitch, &[]);

        // Glitch(10) -> sev=10. glitch_level += 1.0 (10/10). Clamped to 1.0.
        assert!(vm.glitch_level > 0.0);
        assert_eq!(vm.glitch_level, 1.0);

        // Stabilize(5) -> reduces by 0.5
        vm.stack.push(Value::Int(5));
        crate::vm::nova::exec_nova_op(&mut vm, OpCode::Stabilize, &[]);
        assert_eq!(vm.glitch_level, 0.5);

        // Decay
        vm.step(); // Step decays by * 0.95
        // 0.5 * 0.95 = 0.475
        assert!(vm.glitch_level < 0.5);
    }
}
