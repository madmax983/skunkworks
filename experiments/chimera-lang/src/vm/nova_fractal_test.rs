use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::nova_fractal::FractalMode;
use crate::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.metamorphism_enabled = false;
    vm
}

#[test]
fn test_mandelbrot_mode() {
    // [ push(50) mandelbrot() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Mandelbrot,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    // Default mode is already Mandelbrot, let's switch it first to verify it changes
    vm.fractal.mode = FractalMode::Julia;

    vm.step(); // push
    vm.step(); // mandelbrot

    assert_eq!(vm.fractal.mode, FractalMode::Mandelbrot);
    assert_eq!(vm.fractal.max_iter, 50);
}

#[test]
fn test_julia_mode() {
    // [ push(-500) push(100) julia() ] -> c = -0.5 + 0.1i (if divided by 1000)
    // Stack: [re, im]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-500)], // re
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)], // im
        },
        Gene {
            op: OpCode::Julia,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(vm.fractal.mode, FractalMode::Julia);
    // Since args popped from stack are (re: 100, im: -500) or vice versa depending on push order
    // push(-500) then push(100)
    // stack: [-500, 100]
    // julia pops: im = 100, re = -500
    assert_eq!(vm.fractal.c_re, -0.5);
    assert_eq!(vm.fractal.c_im, 0.1);
}

#[test]
fn test_zoom() {
    // [ push(200) zoom() ] -> zoom *= 2.0
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(200)],
        },
        Gene {
            op: OpCode::Zoom,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    let initial_zoom = vm.fractal.zoom;
    vm.step();
    vm.step();

    assert_eq!(vm.fractal.zoom, initial_zoom * 2.0);
}

#[test]
fn test_iterate() {
    // z = 0+0i, c = 1+1i
    // z^2 + c = 0 + 1+1i = 1+1i
    // Args: z_re, z_im, c_re, c_im (scaled by 1000)
    // [ push(0) push(0) push(1000) push(1000) iterate() ]
    // stack: [z_re, z_im, c_re, c_im] -> pop c_im, pop c_re, pop z_im, pop z_re

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // z_re
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // z_im
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1000)],
        }, // c_re
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1000)],
        }, // c_im
        Gene {
            op: OpCode::Iterate,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    for _ in 0..5 {
        vm.step();
    }

    // Expect: 1000, 1000 (1+1i)
    let z_im = vm.stack.pop().unwrap();
    let z_re = vm.stack.pop().unwrap();

    assert_eq!(z_im, Value::Int(1000));
    assert_eq!(z_re, Value::Int(1000));
}
