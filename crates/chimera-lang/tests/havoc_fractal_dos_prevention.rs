#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, MAX_FRACTAL_ITER};

    #[test]
    fn test_fractal_dos_prevention() {
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
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute Push(1B)
        vm.step();
        // Execute Mandelbrot
        vm.step();

        // Check if max_iter was capped
        println!("Fractal Max Iter: {}", vm.fractal.max_iter);

        // 🔒 WARDEN: Verify that the system capped the iteration count.
        assert!(
            vm.fractal.max_iter <= MAX_FRACTAL_ITER,
            "Security Failure: Iteration count {} exceeds limit {}",
            vm.fractal.max_iter,
            MAX_FRACTAL_ITER
        );
        assert_eq!(vm.fractal.max_iter, MAX_FRACTAL_ITER);
    }
}
