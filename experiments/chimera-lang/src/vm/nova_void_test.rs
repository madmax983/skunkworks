#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Strand};

    use crate::vm::nova_void::*;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_void_rift_lifecycle() {
        let mut vm = make_vm(vec![]);

        // 1. Open Rift manually via opcode helper
        vm.stack.push(Value::Int(8)); // y
        vm.stack.push(Value::Int(8)); // x
        exec_void_rift(&mut vm);

        assert_eq!(vm.void_rifts.len(), 1);
        assert_eq!(vm.void_rifts[0].location, (8, 8));
        assert_eq!(vm.void_rifts[0].severity, 1);

        // 2. Process Rifts (Advance Age)
        // Severity 1 -> Threshold 100.
        vm.void_rifts[0].age = 99;

        // Populate neighbors
        vm.grid[8][9] = Value::Int(100);
        vm.grid[7][8] = Value::Int(100);
        vm.grid[9][8] = Value::Int(100);
        vm.grid[8][7] = Value::Int(100);

        process_rifts(&mut vm); // Age becomes 100. Should try to consume.

        assert_eq!(vm.void_rifts[0].age, 100);
    }

    #[test]
    fn test_void_cast() {
        let mut vm = make_vm(vec![]);

        // Open Rift at (8,8)
        vm.stack.push(Value::Int(8)); // y
        vm.stack.push(Value::Int(8)); // x
        exec_void_rift(&mut vm);

        // Increase severity manually
        if let Some(rift) = vm.void_rifts.get_mut(0) {
            rift.severity = 50;
        }

        // Move Context to (8,9) (Distance 1)
        vm.context_loc = (8, 9);

        exec_void_cast(&mut vm);

        // Power = Severity / (Dist + 1) = 50 / (1 + 1) = 25
        if let Some(Value::Int(power)) = vm.stack.pop() {
            assert_eq!(power, 25);
        } else {
            panic!("Expected power on stack, got {:?}", vm.stack.last());
        }
    }
}
