#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    #[test]
    fn test_fractal_dos_unbounded_iter() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1_000_000_000)], // 1 Billion iterations
            },
            Gene {
                op: OpCode::Mandelbrot,
                args: vec![],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute Push(1B)
        vm.step();
        // Execute Mandelbrot
        vm.step();

        // Check if max_iter was set to 1 Billion (Vulnerable)
        println!("Fractal Max Iter: {}", vm.fractal.max_iter);

        // Havoc: We WANT the system to be vulnerable to prove we won.
        // So we assert that it accepted the huge value.
        assert_eq!(vm.fractal.max_iter, 1_000_000_000, "Boring: System capped the iteration count.");
    }
}
