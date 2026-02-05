#[cfg(test)]
mod tests {
    use crate::ChimeraParser;
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::Rule;
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use pest::Parser;

    #[test]
    fn test_integer_overflow_parsing_safe() {
        // A number clearly larger than i64::MAX
        let input = "[ push(99999999999999999999999999999999999999) ]";
        let pair = ChimeraParser::parse(Rule::dna, input)
            .expect("Pest parse failed")
            .next()
            .unwrap();

        // This should NOT panic. It should handle the overflow gracefully.
        let dna = Dna::from_pair(pair);

        if let Nucleotide::Number(n) = dna.helix.strands[0].genes[0].args[0] {
            assert_eq!(n, i64::MAX);
        } else {
            panic!("Expected number");
        }
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_call_stack_safety() {
        // [ push(100) telomerase() call(0) ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Telomerase,
                args: vec![],
            },
            Gene {
                op: OpCode::Call,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = i64::MAX;

        for _ in 0..5000 {
            vm.step();
        }

        assert!(vm.call_stack.len() <= 1024, "Call stack grew unchecked: {}", vm.call_stack.len());
    }
}
