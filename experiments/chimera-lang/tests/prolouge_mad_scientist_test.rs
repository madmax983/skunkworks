#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::prolouge_compiler::compile;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_prolouge_compiler_brainfuck_and_tui() {
        let source = r#"
        brainfuck {
            +++[>+++<-]>
        }
        tui {
            "DRAW RECT"
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(
            genes[0].args[0],
            Nucleotide::String("            +++[>+++<-]>".to_string())
        );
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::String("".to_string()));
        assert_eq!(genes[2].op, OpCode::Brainfuck);

        assert_eq!(genes[3].op, OpCode::Push);
        assert_eq!(
            genes[3].args[0],
            Nucleotide::String("            \"DRAW RECT\"".to_string())
        );
        assert_eq!(genes[4].op, OpCode::TuiDraw);
    }

    #[test]
    fn test_prolouge_compiler_forth() {
        let source = r#"
        forth {
            5 3 add
            print
        }
        "#;
        let dna = compile(source).unwrap();
        assert_eq!(dna.helix.strands.len(), 1);
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(5));

        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(3));

        assert_eq!(genes[2].op, OpCode::Add);
        assert_eq!(genes[3].op, OpCode::Print);
    }

    #[test]
    fn test_prolouge_compiler_orca() {
        let source = r#"
        orca {
            bang 8 8
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::String("bang".to_string()));

        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(8));

        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[2].args[0], Nucleotide::Number(8));

        assert_eq!(genes[3].op, OpCode::Orca);
    }

    #[test]
    fn test_prolouge_compiler_lisp() {
        let source = r#"
        lisp {
            (+ 5 3)
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(5));

        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(3));

        assert_eq!(genes[2].op, OpCode::Add);
    }

    #[test]
    fn test_prolouge_compiler_piet() {
        let source = r#"
        piet {
            rgb(255, 0, 0)
            rgb(0, 255, 0)
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(
            genes[0].args[0],
            Nucleotide::String(
                "            rgb(255, 0, 0)
            rgb(0, 255, 0)"
                    .to_string()
            )
        );
        assert_eq!(genes[1].op, OpCode::Piet);
    }

    #[test]
    fn test_prolouge_compiler_elektra() {
        let source = r#"
        elektra {
            battery 8 8
            ground 4 4
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(genes[0].args[0], Nucleotide::Number(9)); // 9V default
        assert_eq!(genes[1].op, OpCode::Push);
        assert_eq!(genes[1].args[0], Nucleotide::Number(8));
        assert_eq!(genes[2].op, OpCode::Push);
        assert_eq!(genes[2].args[0], Nucleotide::Number(8));
        assert_eq!(genes[3].op, OpCode::Battery);

        assert_eq!(genes[4].op, OpCode::Push);
        assert_eq!(genes[4].args[0], Nucleotide::Number(4));
        assert_eq!(genes[5].op, OpCode::Push);
        assert_eq!(genes[5].args[0], Nucleotide::Number(4));
        assert_eq!(genes[6].op, OpCode::Ground);
    }

    #[test]
    fn test_prolouge_compiler_befunge() {
        let source = r#"
        befunge {
            >987v>.v
            v456<  :
            >321 ^ _@
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(
            genes[0].args[0],
            Nucleotide::String(
                "            >987v>.v
            v456<  :
            >321 ^ _@"
                    .to_string()
            )
        );
        assert_eq!(genes[1].op, OpCode::Befunge);
    }

    #[test]
    fn test_befunge_execution() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("23+.".to_string())],
            },
            Gene {
                op: OpCode::Befunge,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![chimera_lang::ast::Strand { genes }],
            },
        });

        vm.step(); // Push string
        vm.step(); // Befunge

        assert_eq!(vm.output, vec!["5".to_string()]);
    }

    #[test]
    fn test_prolouge_execution_chaos() {
        let mut vm = make_vm();

        // Ensure not active initially
        assert!(!vm.prologue_state.active);

        let initial_energy = vm.energy;

        // Execute OpCode::Prolouge via dispatcher
        let res = chimera_lang::vm::nova::exec_nova_op(&mut vm, OpCode::Prolouge, &[]);

        assert!(res.is_none());
        assert!(vm.prologue_state.active);
        assert!(vm.prologue_state.orca_mode);
        assert_eq!(vm.glitch_level, 100.0);
        assert_eq!(vm.energy, initial_energy + 1000);

        assert!(vm
            .output
            .contains(&"PROLOUGE: Mad Scientist Mode ACTIVATED ⚛️".to_string()));

        // Test that runes or signals were added to the grid occasionally
        let mut has_signals = false;
        let mut has_runes = false;

        let target_runes = [
            "M", "ζ", "₣", "⚡", "O", "?", "!", "*", "~", "♻", "P", "c", "[a-z]+",
        ];
        for y in 0..chimera_lang::vm::GRID_SIZE {
            for x in 0..chimera_lang::vm::GRID_SIZE {
                if vm.signal_grid[y][x] == 1 {
                    has_signals = true;
                }
                if let Value::Str(r) = &vm.grid[y][x] {
                    if target_runes.contains(&r.as_str()) {
                        has_runes = true;
                    }
                }
            }
        }

        // It's technically random but with grid size 16x16 (256 cells)
        // 15% chance for runes = ~38 runes
        // 5% chance for signals = ~12 signals
        // So probability of having 0 is practically 0.
        assert!(has_runes, "Should have injected runes");
        assert!(has_signals, "Should have injected signals");
    }

    #[test]
    fn test_prolouge_compiler_regex() {
        let source = r#"
        regex {
            [a-z]+
        }
        "#;
        let dna = compile(source).unwrap();
        let genes = &dna.helix.strands[0].genes;

        assert_eq!(genes[0].op, OpCode::Push);
        assert_eq!(
            genes[0].args[0],
            Nucleotide::String("            [a-z]+".to_string())
        );
        assert_eq!(genes[1].op, OpCode::ParserRegex);
    }
}
