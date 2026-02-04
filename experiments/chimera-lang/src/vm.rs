use crate::ast::{Dna, Nucleotide};
use rand::Rng;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
        }
    }
}

pub struct ChimeraVM {
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize), // (strand_idx, gene_idx)
    pub output: Vec<String>,
    pub halted: bool,
}

impl ChimeraVM {
    pub fn new(dna: Dna) -> Self {
        Self {
            dna,
            stack: Vec::new(),
            ip: (0, 0),
            output: Vec::new(),
            halted: false,
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let helix = &self.dna.helix;
        if self.ip.0 >= helix.strands.len() {
            self.halted = true;
            return;
        }

        let strand = &helix.strands[self.ip.0];
        if self.ip.1 >= strand.genes.len() {
            // End of strand, move to next strand
            self.ip.0 += 1;
            self.ip.1 = 0;
            // Recursively call step to execute first gene of next strand immediately?
            // Or wait for next cycle? Let's wait.
            return;
        }

        let gene = &strand.genes[self.ip.1];
        let jump_target =
            Self::execute_gene(&mut self.stack, &mut self.output, &gene.name, &gene.args);

        if let Some(target) = jump_target {
            self.ip = target;
        } else {
            // Move to next gene
            self.ip.1 += 1;
        }
    }

    fn execute_gene(
        stack: &mut Vec<Value>,
        output: &mut Vec<String>,
        name: &str,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match name {
            "push" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Nucleotide::Number(n) => stack.push(Value::Int(*n)),
                        Nucleotide::String(s) => stack.push(Value::Str(s.clone())),
                        _ => output.push(format!("Error: Invalid arg for push: {:?}", arg)),
                    }
                }
                None
            }
            "add" => {
                Self::binary_op(stack, output, |a, b| a + b);
                None
            }
            "sub" => {
                Self::binary_op(stack, output, |a, b| a - b);
                None
            }
            "mul" => {
                Self::binary_op(stack, output, |a, b| a * b);
                None
            }
            "div" => {
                Self::binary_op(stack, output, |a, b| a / b);
                None
            }
            "dup" => {
                if let Some(val) = stack.last() {
                    stack.push(val.clone());
                }
                None
            }
            "swap" => {
                let len = stack.len();
                if len >= 2 {
                    stack.swap(len - 1, len - 2);
                } else {
                    output.push("Error: Stack underflow for swap".to_string());
                }
                None
            }
            "drop" => {
                stack.pop();
                None
            }
            "print" => {
                if let Some(val) = stack.pop() {
                    output.push(format!("{}", val));
                }
                None
            }
            "jump" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            "brz" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    // Peek or pop? Branch if top is zero usually pops.
                    if let Some(val) = stack.pop() {
                        if let Value::Int(i) = val {
                            if i == 0 {
                                return Some((*n as usize, 0));
                            }
                        } else {
                            output.push("Error: Type mismatch for brz".to_string());
                        }
                    } else {
                        output.push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            _ => {
                output.push(format!("Unknown enzyme: {}", name));
                None
            }
        }
    }

    fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match (a, b) {
            (Value::Int(ia), Value::Int(ib)) => stack.push(Value::Int(op(ia, ib))),
            _ => output.push("Error: Type mismatch".to_string()),
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let helix = &mut self.dna.helix;
        if helix.strands.is_empty() {
            return;
        }

        let strand_idx = rng.gen_range(0..helix.strands.len());
        let strand = &mut helix.strands[strand_idx];
        if strand.genes.is_empty() {
            return;
        }

        let gene_idx = rng.gen_range(0..strand.genes.len());
        let gene = &mut strand.genes[gene_idx];

        // 50% chance to change name, 50% to change arg
        if rng.gen_bool(0.5) {
            let enzymes = [
                "push", "add", "sub", "mul", "div", "dup", "print", "swap", "drop", "jump", "brz",
            ];
            let new_name = enzymes[rng.gen_range(0..enzymes.len())];
            // Add "Mutation" log
            self.output
                .push(format!("MUTATION: {} -> {}", gene.name, new_name));
            gene.name = new_name.to_string();
        } else if !gene.args.is_empty() {
            if let Some(Nucleotide::Number(n)) = gene.args.first_mut() {
                let old_n = *n;
                *n = rng.gen_range(0..100); // Random number
                self.output
                    .push(format!("MUTATION: arg {} -> {}", old_n, *n));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_add() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                name: "add".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        match vm.stack[0] {
            Value::Int(30) => (),
            _ => panic!("Expected 30"),
        }
    }

    #[test]
    fn test_jump() {
        // [ jump(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "jump".to_string(),
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: jump(1)
        vm.step();
        assert_eq!(vm.ip, (1, 0));

        // Step 2: push(200)
        vm.step();
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }

    #[test]
    fn test_brz() {
        // [ push(0) brz(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    name: "brz".to_string(),
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(0)
        vm.step();

        // Step 2: brz(1) -> pops 0, jumps to (1, 0)
        vm.step();
        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.stack.len(), 0);

        // Step 3: push(200)
        vm.step();
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }
}
