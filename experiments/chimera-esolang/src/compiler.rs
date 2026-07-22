use crate::ast::ast::{Instruction, Program};
use anyhow::Result;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use std::str::FromStr;

pub fn compile(program: &Program) -> Result<Dna> {
    let mut helix = Helix {
        strands: Vec::new(),
    };

    // Very basic mapping for demo.
    // Genesis maps Instructions -> Chimera OpCodes

    #[allow(clippy::for_kv_map)]
    for (_name, strand) in &program.strands {
        let mut genes = Vec::new();

        for instr in &strand.instructions {
            match instr {
                Instruction::Number(n) => genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(*n)],
                }),
                Instruction::String(s) => genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(s.clone())],
                }),
                Instruction::Operator(op) => {
                    let opcode = match op.as_str() {
                        "+" => OpCode::Add,
                        "-" => OpCode::Sub,
                        "*" => OpCode::Mul,
                        "/" => OpCode::Div,
                        "dup" => OpCode::Dup,
                        "drop" => OpCode::Drop,
                        "swap" => OpCode::Swap,
                        _ => {
                            if let Ok(chimera_op) = OpCode::from_str(op) {
                                chimera_op
                            } else {
                                // Default back to push string if unknown operator
                                OpCode::Push
                            }
                        }
                    };

                    if opcode == OpCode::Push {
                        genes.push(Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::String(op.clone())],
                        });
                    } else {
                        genes.push(Gene {
                            op: opcode,
                            args: vec![],
                        });
                    }
                }
                Instruction::Identifier(id) => genes.push(Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String(id.clone())],
                }),
                Instruction::Call(id) => {
                    // Jump/call logic could go here
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Identifier(id.clone())],
                    });
                    genes.push(Gene {
                        op: OpCode::Call,
                        args: vec![],
                    });
                }
                Instruction::Spawn(id) => {
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Identifier(id.clone())],
                    });
                    genes.push(Gene {
                        op: OpCode::Spawn,
                        args: vec![],
                    });
                }
                Instruction::Query(q) => {
                    genes.push(Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::String(q.clone())],
                    });
                    genes.push(Gene {
                        op: OpCode::Rule,
                        args: vec![],
                    });
                }
            }
        }

        helix.strands.push(Strand { genes });
    }

    Ok(Dna {
        helix,
        evolution_config: None,
    })
}
