use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    ChimeraVM::new(dna)
}

fn string_genes(s: &str) -> Vec<Gene> {
    let mut genes = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return genes;
    }

    // First char
    genes.push(Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(chars[0] as i64)],
    });
    genes.push(Gene {
        op: OpCode::Chr,
        args: vec![],
    });

    for c in chars.iter().skip(1) {
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(*c as i64)],
        });
        genes.push(Gene {
            op: OpCode::Chr,
            args: vec![],
        });
        genes.push(Gene {
            op: OpCode::Add,
            args: vec![],
        });
    }
    genes
}

#[test]
fn test_holo_grammar_invocation() {
    let mut vm = make_vm();

    // 1. Construct a strand that builds a grammar: [ Push("hello") Push("Match") Grammar() ]
    // Using procedural string construction because Hologram doesn't store strings.

    let mut genes = Vec::new();
    // Build "hello"
    genes.extend(string_genes("hello"));
    // Build "Match"
    genes.extend(string_genes("Match"));
    // Grammar
    genes.push(Gene {
        op: OpCode::Grammar,
        args: vec![],
    });

    let strand = Strand { genes };
    vm.dna.helix.strands.push(strand);

    // 2. Encode into Hologram
    crate::vm::nova_hologram::exec_interfere(&mut vm, OpCode::Interfere, &[Nucleotide::Number(0)]);

    // 3. Clear state
    vm.stack.clear();

    // 4. Test HoloInvoke
    // Stack: [ "hello" ] -> HoloInvoke -> [ "hello" ] (AST)
    vm.stack.push(Value::Str("hello".to_string()));

    crate::vm::nova_hologram::exec_holo_invoke(&mut vm, OpCode::HoloInvoke, &[]);

    // Check result
    // Should be AST: "hello" (since Match returns the matched string as AST)

    assert_eq!(vm.stack.len(), 1, "Stack should contain result");
    if let Some(val) = vm.stack.pop() {
        assert_eq!(val, Value::Str("hello".to_string()));
    } else {
        panic!("Stack empty");
    }
}

#[test]
fn test_holo_speak_generation() {
    let mut vm = make_vm();

    // Grammar: [ Push("world") Push("Match") Grammar() ]
    let mut genes = Vec::new();
    genes.extend(string_genes("world"));
    genes.extend(string_genes("Match"));
    genes.push(Gene {
        op: OpCode::Grammar,
        args: vec![],
    });

    let strand = Strand { genes };
    vm.dna.helix.strands.push(strand);

    // Encode
    crate::vm::nova_hologram::exec_interfere(&mut vm, OpCode::Interfere, &[Nucleotide::Number(0)]);

    // Clear
    vm.stack.clear();

    // Speak
    crate::vm::nova_hologram::exec_holo_speak(&mut vm, OpCode::HoloSpeak, &[]);

    // Expect "world"
    assert_eq!(vm.stack.len(), 1);
    if let Some(val) = vm.stack.pop() {
        assert_eq!(val, Value::Str("world".to_string()));
    } else {
        panic!("Stack empty");
    }
}
