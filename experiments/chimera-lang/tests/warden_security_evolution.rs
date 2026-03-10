#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::compiler::compile;
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{
        evolution::{Challenge, EvolutionEngine},
        ChimeraVM,
    };

    #[test]
    fn test_evolution_zero_population_config_parse() {
        // Threat: population: 0 causes empty pool in tournament_select and index out of bounds in step()
        // This test ensures parsing works correctly (0 is parsed) AND execution is safe (clamped).
        let src = r#"
        evolution ZeroPop {
            population: 0
            mutation_rate: 0.1
        }
        strand main {
            push(1)
        }
        "#;
        let dna = compile(src, None).unwrap();

        if let Some(ref config) = dna.evolution_config {
            // Verify parser is working (fixed the silent literal bug)
            // If parser is fixed, this should be 0.
            // If parser is buggy, it defaults to 50.
            // We expect it to be 0 now.
            assert_eq!(
                config.population_size, 0,
                "Parser failed to read population: 0"
            );

            let seed = dna.helix.strands[0].clone();
            let vm = ChimeraVM::new(dna.clone());

            // This should safely handle 0 population (clamped to 1 internally)
            let mut engine = EvolutionEngine::from_config(seed, config.clone());
            engine.step(&vm);

            // Verify clamping happened
            assert_eq!(
                engine.population.len(),
                1,
                "Engine did not clamp population size to 1"
            );
        } else {
            panic!("Failed to parse evolution config");
        }
    }

    #[test]
    fn test_evolution_engine_direct_zero_pop() {
        // Threat: Direct instantiation with 0 population causes panic

        let seed = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![seed.clone()],
            },
        };
        let vm = ChimeraVM::new(dna);

        // Directly create engine with 0 population
        let mut engine = EvolutionEngine::new(seed, 0, Challenge::Target(42));

        // This should NOT panic
        engine.step(&vm);

        // Verify clamping
        assert_eq!(
            engine.population.len(),
            1,
            "Engine did not clamp population size to 1"
        );
    }
}
