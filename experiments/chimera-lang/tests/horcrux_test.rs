#[cfg(feature = "nova")]
#[test]
fn test_horcrux_ritual() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    // Strand 0: Main (Calls Victim)
    let main_genes = vec![Gene {
        op: OpCode::Call,
        args: vec![Nucleotide::Number(1)],
    }];

    // Strand 1: Victim (Writes Horcrux at 5,5)
    let victim_genes = vec![
        Gene {
            op: OpCode::SIndex,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::Horcrux,
            args: vec![],
        },
    ];

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![
                Strand { genes: main_genes },
                Strand {
                    genes: victim_genes,
                },
            ],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000; // Increase energy to survive the ritual

    // Step 1: Execute Main -> Call Victim
    // Main is Strand 0.
    // Gene 0: Call(1) -> Jumps to Strand 1
    vm.step();
    assert_eq!(vm.ip.0, 1);

    // Step 2: Execute Victim
    // Gene 0: SIndex -> Push 1
    vm.step();
    // Gene 1: Push(5)
    vm.step();
    // Gene 2: Push(5)
    vm.step();
    // Gene 3: Horcrux -> Writes grid, Kills strand 1
    vm.step();

    // Verify Grid
    let val = vm.grid[5][5].clone();
    if let Value::Str(s) = val {
        assert!(s.starts_with("Horcrux:"), "Grid should contain Horcrux");
        println!("Horcrux created: {}", s);
    } else {
        panic!("Grid does not contain string");
    }

    // Verify Victim Death
    assert!(
        vm.dna.helix.strands[1].genes.is_empty(),
        "Victim genes should be cleared"
    );

    // Step 3: Rebirth
    // Create a strand that executes Rebirth
    let necromancer_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // y
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // x
        Gene {
            op: OpCode::Rebirth,
            args: vec![],
        },
    ];

    // Inject this strand
    vm.dna.helix.strands.push(Strand {
        genes: necromancer_genes,
    });
    let necro_idx = vm.dna.helix.strands.len() - 1;

    // Manually set IP to execute the necromancer strand
    vm.ip = (necro_idx, 0);

    // Execute Necromancer
    vm.step(); // Push 5
    vm.step(); // Push 5
    vm.step(); // Rebirth

    // Verify New Strand
    if vm.dna.helix.strands.len() != 4 {
        println!("VM Output: {:#?}", vm.output);
    }
    assert_eq!(
        vm.dna.helix.strands.len(),
        4,
        "Should have 4 strands (0, 1[dead], 2[necro], 3[new])"
    );
    let new_strand = &vm.dna.helix.strands[3];
    assert!(
        !new_strand.genes.is_empty(),
        "Resurrected strand should have genes"
    );
    // Victim had 4 genes
    assert_eq!(
        new_strand.genes.len(),
        4,
        "Resurrected strand should have 4 genes"
    );

    // Verify Grid Cleared
    assert_eq!(vm.grid[5][5], Value::Int(0), "Grid should be cleared");
}
