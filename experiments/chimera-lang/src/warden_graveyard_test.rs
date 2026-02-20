#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, MAX_GRAVEYARD_SIZE};

    #[test]
    fn test_graveyard_limit() {
        let genes = vec![
            Gene {
                op: OpCode::Bury,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let victim_strand = Strand { genes: vec![] };
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }, victim_strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100000;
        vm.telomeres[0] = 10000;

        for i in 0..300 {
            vm.stack.push(crate::vm::Value::Int(1));
            vm.step(); // Bury
            vm.step(); // Jump
            if vm.halted {
                println!("VM halted at step {} loop {}", vm.tick_counter, i);
                break;
            }
        }

        let size = vm.graveyard.len();
        println!("Graveyard size: {}", size);
        println!("VM Halted: {}", vm.halted);
        println!("Output last 5 lines:");
        for line in vm.output.iter().rev().take(5) {
            println!("{}", line);
        }

        assert!(
            size <= MAX_GRAVEYARD_SIZE,
            "Graveyard size {} exceeded limit {}",
            size,
            MAX_GRAVEYARD_SIZE
        );
        assert_eq!(size, MAX_GRAVEYARD_SIZE, "Graveyard should be full");
    }
}
