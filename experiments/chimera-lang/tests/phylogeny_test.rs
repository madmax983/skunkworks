#![cfg(feature = "phylogeny")]

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
use rand::Rng;
use std::fs;
use std::path::PathBuf;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

fn setup_temp_dir() -> PathBuf {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen();
    let mut dir = std::env::temp_dir();
    dir.push(format!("chimera_test_{}", id));
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir(&dir).unwrap();
    dir
}

#[test]
fn test_synthesize_and_sequencing() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.join("test_dna.txt");
    let path_str = file_path.to_str().unwrap().to_string();
    let content = "ATGC".to_string();

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(path_str.clone())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(content.clone())],
        },
        Gene {
            op: OpCode::Synthesize,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(path_str.clone())],
        },
        Gene {
            op: OpCode::Sequencing,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    // Stack should have content read back
    // Stack: [Str("ATGC")] (Synthesize pops 2, Sequencing pushes 1)
    if let Some(result) = vm.stack.pop() {
        assert_eq!(result, Value::Str(content));
    } else {
        panic!("Stack empty, expected result. Output: {:?}", vm.output);
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_crawl() {
    let temp_dir = setup_temp_dir();
    let file1 = temp_dir.join("f1.txt");
    let file2 = temp_dir.join("f2.txt");
    fs::write(&file1, "A").unwrap();
    fs::write(&file2, "B").unwrap();
    let path_str = temp_dir.to_str().unwrap().to_string();

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(path_str)],
        },
        Gene {
            op: OpCode::Crawl,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    if let Some(result) = vm.stack.pop() {
        if let Value::Junction(_, files) = result {
            assert_eq!(files.len(), 2);
            let names: Vec<String> = files
                .iter()
                .map(|v| {
                    if let Value::Str(s) = v {
                        s.clone()
                    } else {
                        "".to_string()
                    }
                })
                .collect();
            assert!(names.contains(&"f1.txt".to_string()));
            assert!(names.contains(&"f2.txt".to_string()));
        } else {
            panic!("Expected Junction, got {:?}", result);
        }
    } else {
        panic!("Stack empty. Output: {:?}", vm.output);
    }

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_shell() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("echo hello".to_string())],
        },
        Gene {
            op: OpCode::Shell,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    if let Some(result) = vm.stack.pop() {
        if let Value::Str(s) = result {
            assert!(s.contains("hello"));
        } else {
            panic!("Expected String, got {:?}", result);
        }
    } else {
        panic!("Stack empty. Output: {:?}", vm.output);
    }
}

#[test]
fn test_infect() {
    let temp_dir = setup_temp_dir();
    let file_path = temp_dir.join("host.txt");
    fs::write(&file_path, "HostCode\n").unwrap();
    let path_str = file_path.to_str().unwrap().to_string();
    let viral_code = "ViralCode";

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(path_str.clone())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String(viral_code.to_string())],
        },
        Gene {
            op: OpCode::Infect,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    while !vm.halted {
        vm.step();
    }

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "HostCode\nViralCode");

    let _ = fs::remove_dir_all(&temp_dir);
}
