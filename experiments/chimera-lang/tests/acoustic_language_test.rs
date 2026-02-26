#![cfg(feature = "resonance")]

use chimera_lang::acoustic_compiler;
use chimera_lang::prelude::*;

#[test]
fn test_compile_acoustic_basic() {
    let source = r#"
        score TestScore {
            tempo 120

            instrument Guitar {
                def strum {
                    play(60)
                    rest(2)
                }
            }

            track Main {
                op(Push, 100)
                call Guitar.strum()
                repeat(2) {
                    play(72)
                }
            }
        }
    "#;

    let dna = acoustic_compiler::compile(source).expect("Failed to compile Acoustic source");

    // Check Helix Structure
    // Expected Strands:
    // Guitar.strum (Parsed first)
    // Main (Parsed second)

    assert_eq!(dna.helix.strands.len(), 2, "Should have 2 strands");

    let guitar_strum = &dna.helix.strands[0];
    let main = &dna.helix.strands[1];

    // Check Guitar.strum content
    // play(60) -> Push(60), Note
    // rest(2) -> Push(2), Rest
    assert_eq!(guitar_strum.genes.len(), 4);
    assert_eq!(guitar_strum.genes[0].op, OpCode::Push);
    assert_eq!(guitar_strum.genes[0].args[0], Nucleotide::Number(60));
    assert_eq!(guitar_strum.genes[1].op, OpCode::Note);

    assert_eq!(guitar_strum.genes[2].op, OpCode::Push);
    assert_eq!(guitar_strum.genes[2].args[0], Nucleotide::Number(2));
    assert_eq!(guitar_strum.genes[3].op, OpCode::Rest);

    // Check Main content
    // op(Push, 100) -> Push(100) (Immediate)
    // call Guitar.strum() -> Call(0)
    // repeat(2) { play(72) } -> Push(72), Note, Push(72), Note

    assert_eq!(main.genes[0].op, OpCode::Push);
    assert_eq!(main.genes[0].args[0], Nucleotide::Number(100));

    assert_eq!(main.genes[1].op, OpCode::Call);
    assert_eq!(main.genes[1].args[0], Nucleotide::Number(0)); // Guitar.strum index

    // Repeat unrolled
    assert_eq!(main.genes[2].op, OpCode::Push);
    assert_eq!(main.genes[2].args[0], Nucleotide::Number(72));
    assert_eq!(main.genes[3].op, OpCode::Note);

    assert_eq!(main.genes[4].op, OpCode::Push);
    assert_eq!(main.genes[4].args[0], Nucleotide::Number(72));
    assert_eq!(main.genes[5].op, OpCode::Note);
}

#[test]
fn test_compile_acoustic_loop() {
    let source = r#"
        score LoopScore {
            track Looper {
                play(10)
                loop {
                    play(20)
                }
            }
        }
    "#;

    let dna = acoustic_compiler::compile(source).expect("Failed to compile");
    assert_eq!(dna.helix.strands.len(), 1);

    let looper = &dna.helix.strands[0];
    // play(10) -> Push(10), Note
    // loop { play(20) } -> Push(20), Note, Jump(0)

    let len = looper.genes.len();
    assert_eq!(looper.genes[len - 1].op, OpCode::Jump);
    assert_eq!(looper.genes[len - 1].args[0], Nucleotide::Number(0)); // Jumps to self
}
