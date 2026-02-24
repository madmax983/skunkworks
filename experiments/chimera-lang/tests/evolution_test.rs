use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
#[ignore]
fn test_prophecy_rune() {
    // Strand 0: Supernova (Immediate Halt/Death)
    let genes_dead = vec![Gene {
        op: OpCode::Supernova,
        args: vec![],
    }];

    // Strand 1: Sustainable Loop (Life)
    // [ push(1) jump(1) ] -> Jump to self (Strand 1)
    let genes_alive = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        },
    ];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: genes_dead }, Strand { genes: genes_alive }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.energy = 1000;

    // Setup Prophecy Circuit for Strand 0 (Death)
    // "0" -> ! -> c -> ?
    vm.grid[5][4] = Value::Str("0".to_string());
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("c".to_string());

    exec_prologue_tick(&mut vm);

    match &vm.grid[6][6] {
        Value::Int(res) => assert_eq!(*res, 1, "Prophecy should predict death (1) for Strand 0"),
        _ => panic!("Prophecy output invalid: {:?}", vm.grid[6][6]),
    }

    // Reset Grid for Strand 1 (Life)
    vm.grid[5][4] = Value::Int(1);
    vm.grid[6][6] = Value::Int(-1);

    exec_prologue_tick(&mut vm);

    match &vm.grid[6][6] {
        Value::Int(res) => assert_eq!(*res, 0, "Prophecy should predict life (0) for Strand 1"),
        _ => panic!("Prophecy output invalid: {:?}", vm.grid[6][6]),
    }
}

#[test]
fn test_entangle_rune() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![
                Strand { genes: vec![] }, // 0
                Strand { genes: vec![] }, // 1
            ],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Entangle Circuit
    // West Input: "0" -> ! -> 8
    vm.grid[5][4] = Value::Str("0".to_string());
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("8".to_string());

    // East Input: 1 -> ! -> ~ -> ~ -> 8
    vm.grid[7][6] = Value::Int(1);
    vm.grid[7][7] = Value::Str("!".to_string());
    vm.grid[6][7] = Value::Str("~".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());

    assert!(!vm.entangled_pairs.contains_key(&0));

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.entangled_pairs.get(&0), Some(&1));
    assert_eq!(vm.entangled_pairs.get(&1), Some(&0));
}
