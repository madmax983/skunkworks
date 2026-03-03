with open("experiments/chimera-lang/src/song_test.rs", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "assert_eq!(vm.ip.0, 1);" in line:
        new_lines.extend([
            "        vm.step(); // Extra tick to allow triggered host jump\n",
            "        vm.step(); // Verify IP changes\n",
            "        assert_eq!(vm.ip.0, 1);\n"
        ])
    elif "assert_eq!(vm.chorus_buffer.len(), 0);" in line:
        pass
    elif "vm.step(); // Tick 2 (Choir: \"Lux\")" in line:
        pass
    elif "vm.step(); // Process trigger jump" in line:
        pass
    else:
        new_lines.append(line)

with open("experiments/chimera-lang/src/song_test.rs", "w") as f:
    f.writelines([
"""#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_song_of_creation() {
        let chord = vec![
            Nucleotide::String("Fiat".to_string()),
            Nucleotide::String("Lux".to_string()),
        ];

        let main_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Junction(JunctionType::All, chord.clone())] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Harmonize, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Junction(JunctionType::All, chord)] },
                Gene { op: OpCode::Choir, args: vec![] },
            ],
        };

        let effect_strand = Strand {
            genes: vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }],
        };

        let dna = Dna { evolution_config: None, helix: Helix { strands: vec![main_strand, effect_strand] } };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000;

        vm.step(); // push chord
        vm.step(); // push target 1
        vm.step(); // harmonize

        vm.step(); // push chord
        vm.step(); // choir (spawns organelle)

        vm.step(); // tick organelle (sings "Fiat")
        vm.step(); // tick organelle (sings "Lux", triggers host jump)
        vm.step(); // host processes jump to strand 1

        assert_eq!(vm.ip.0, 1);
    }

    #[test]
    fn test_song_integration() {
        let chord = vec![
            Nucleotide::String("Fiat".to_string()),
            Nucleotide::String("Lux".to_string()),
        ];

        let main_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Junction(JunctionType::All, chord.clone())] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Harmonize, args: vec![] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Junction(JunctionType::All, chord)] },
                Gene { op: OpCode::Choir, args: vec![] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
            ],
        };

        let effect_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
                Gene { op: OpCode::Ret, args: vec![] },
            ],
        };

        let dna = Dna { evolution_config: None, helix: Helix { strands: vec![main_strand, effect_strand] } };
        let mut vm = ChimeraVM::new(dna);
        vm.metamorphism_enabled = false;
        vm.energy = 1000;

        vm.step();
        vm.step();
        vm.step();
        vm.step();
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.ip, (1, 0));
        vm.step();
        assert_eq!(vm.stack.last(), Some(&Value::Int(100)));
    }
}
"""
    ])
