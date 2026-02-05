use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_pigment() {
    // [ push(5) push(5) push(100) push(150) push(200) pigment() ]
    // Sets (5,5) to RGB(100, 150, 200)
    // Note: stack order for pigment is: r, g, b, y, x (top).
    // So we push r, g, b, y, x in order.
    // Wait, my impl says:
    // let x_val = vm.stack.pop().unwrap(); // x is top
    // let y_val = vm.stack.pop().unwrap();
    // let b_val = vm.stack.pop().unwrap();
    // let g_val = vm.stack.pop().unwrap();
    // let r_val = vm.stack.pop().unwrap();
    //
    // So Stack bottom -> top: r, g, b, y, x.

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // r
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(150)],
        }, // g
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(200)],
        }, // b
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::Pigment,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..6 {
        vm.step();
    }

    assert_eq!(vm.chroma_grid[5][5].fg, Some((100, 150, 200)));
}

#[test]
fn test_glyph() {
    // [ push(65) push(6) push(6) glyph() ]
    // Stack: char, y, x (top)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(65)],
        }, // char code 'A'
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // x
        Gene {
            op: OpCode::Glyph,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..4 {
        vm.step();
    }

    assert_eq!(vm.chroma_grid[6][6].char, Some('A'));
}

#[test]
fn test_clear_pigment() {
    // Set first
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // r
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // g
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        }, // b
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::Pigment,
            args: vec![],
        },
        // Clear (r = -1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-1)],
        }, // r
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // g
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // b
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::Pigment,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..12 {
        vm.step();
    }
    assert_eq!(vm.chroma_grid[5][5].fg, None);
}

#[test]
fn test_clear_glyph() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(65)],
        }, // char
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // x
        Gene {
            op: OpCode::Glyph,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-1)],
        }, // char -1
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(6)],
        }, // x
        Gene {
            op: OpCode::Glyph,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..8 {
        vm.step();
    }
    assert_eq!(vm.chroma_grid[6][6].char, None);
}
