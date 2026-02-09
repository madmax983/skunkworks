#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_babel_tower_defense() {
        let mut vm = make_vm();

        // 1. Spawn a Virus
        // "INFECT: Released 'Flu' (ID 0) at 8,8"
        // Stack: mutation_rate, pattern, name (Top)
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] }, // Rate
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("ACGT".to_string())] }, // Pattern
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("Flu".to_string())] }, // Name
            Gene { op: OpCode::Infect, args: vec![] },
        ];
        vm.inject_genes(genes);

        // Run Infect
        for _ in 0..4 { vm.step(); }

        // Verify Virus is at 8,8
        assert!(vm.viral_grid[8][8].is_some(), "Virus should be present at 8,8");

        // 2. Construct Parser (Antibody) matching "ACGT"
        // Stack: Name, Parser (Top)
        let parser_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("AntiFlu".to_string())] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("ACGT".to_string())] },
            Gene { op: OpCode::ParserMatch, args: vec![] },
            Gene { op: OpCode::Learn, args: vec![] },
        ];
        vm.inject_genes(parser_genes);

        // Run Construction and Learn
        for _ in 0..4 { vm.step(); }

        // Verify Antibody Learned
        assert!(vm.antibodies.contains_key("AntiFlu"), "Antibody should be learned");

        // 3. Sanitize using the Antibody (Targeted Immune Response)
        // We need to retrieve the parser first?
        // No, Sanitize takes the parser on stack directly if we want to use specific one.
        // Or we can construct it again.
        // Let's construct it again on stack for Sanitize call.
        // Stack: Parser, Radius
        let cure_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("ACGT".to_string())] },
            Gene { op: OpCode::ParserMatch, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // Radius
            Gene { op: OpCode::Sanitize, args: vec![] },
        ];
        vm.inject_genes(cure_genes);

        // Run Cure
        for _ in 0..4 { vm.step(); }

        // Verify Virus is gone
        assert!(vm.viral_grid[8][8].is_none(), "Virus should be cleared");

        // Verify Energy Gain (Started 50, spent some, gained from cure)
        // Initial 50.
        // Infect cost? No cost to infect usually (it's an op).
        // Sanitize gain.
        // We can check if output contains success message.
        let success = vm.output.iter().any(|s| s.contains("SANITIZE: Cured 1 infections"));
        assert!(success, "Should report cure success");
    }
}
