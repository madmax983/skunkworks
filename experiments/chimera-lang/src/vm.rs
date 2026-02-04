use crate::ast::{Dna, Nucleotide};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
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
    pub grid: Vec<Vec<Value>>,
}

impl ChimeraVM {
    pub fn new(dna: Dna) -> Self {
        let grid = vec![vec![Value::Int(0); 16]; 16];
        Self {
            dna,
            stack: Vec::new(),
            ip: (0, 0),
            output: Vec::new(),
            halted: false,
            grid,
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let helix_len = self.dna.helix.strands.len();
        if self.ip.0 >= helix_len {
            self.halted = true;
            return;
        }

        let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
        if self.ip.1 >= strand_len {
            // End of strand, move to next strand
            self.ip.0 += 1;
            self.ip.1 = 0;
            return;
        }

        // Clone gene info to release borrow on self.dna
        let (gene_name, gene_args) = {
            let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
            (gene.name.clone(), gene.args.clone())
        };

        let jump_target = self.execute_gene(&gene_name, &gene_args);

        if let Some(target) = jump_target {
            self.ip = target;
        } else {
            // Move to next gene
            self.ip.1 += 1;
        }
    }

    fn execute_gene(&mut self, name: &str, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match name {
            "push" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Nucleotide::Number(n) => self.stack.push(Value::Int(*n)),
                        Nucleotide::String(s) => self.stack.push(Value::Str(s.clone())),
                        _ => self
                            .output
                            .push(format!("Error: Invalid arg for push: {:?}", arg)),
                    }
                }
                None
            }
            "add" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a + b);
                None
            }
            "sub" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a - b);
                None
            }
            "mul" => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a * b);
                None
            }
            "div" => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a / b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
                None
            }
            "dup" => {
                if let Some(val) = self.stack.last() {
                    self.stack.push(val.clone());
                }
                None
            }
            "swap" => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
                None
            }
            "drop" => {
                self.stack.pop();
                None
            }
            "print" => {
                if let Some(val) = self.stack.pop() {
                    self.output.push(format!("{}", val));
                }
                None
            }
            "jump" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            "brz" => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    if let Some(val) = self.stack.pop() {
                        if let Value::Int(i) = val {
                            if i == 0 {
                                return Some((*n as usize, 0));
                            }
                        } else {
                            self.output.push("Error: Type mismatch for brz".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            // --- EVOLUTION ---
            "transcribe" => {
                // stack: value (top), arg_idx, gene_idx, strand_idx (bottom)
                if self.stack.len() < 4 {
                    self.output
                        .push("Error: Stack underflow for transcribe".to_string());
                    return None;
                }
                let val = self.stack.pop().unwrap();
                let arg_idx_val = self.stack.pop().unwrap();
                let gene_idx_val = self.stack.pop().unwrap();
                let strand_idx_val = self.stack.pop().unwrap();

                match (val, arg_idx_val, gene_idx_val, strand_idx_val) {
                    (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) => {
                        if si >= 0 && (si as usize) < self.dna.helix.strands.len() {
                            let strand = &mut self.dna.helix.strands[si as usize];
                            if gi >= 0 && (gi as usize) < strand.genes.len() {
                                let gene = &mut strand.genes[gi as usize];
                                if ai >= 0 && (ai as usize) < gene.args.len() {
                                    gene.args[ai as usize] = Nucleotide::Number(v);
                                    self.output.push(format!(
                                        "TRANSCRIBE: strand {} gene {} arg {} -> {}",
                                        si, gi, ai, v
                                    ));
                                } else {
                                    self.output
                                        .push("Error: Arg index out of bounds".to_string());
                                }
                            } else {
                                self.output
                                    .push("Error: Gene index out of bounds".to_string());
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds".to_string());
                        }
                    }
                    _ => self
                        .output
                        .push("Error: Type mismatch for transcribe args".to_string()),
                }
                None
            }
            "s_len" => {
                self.stack.push(Value::Int(self.stack.len() as i64));
                None
            }
            "helix_len" => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
                None
            }
            "gene_len" => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            if idx >= 0 && (idx as usize) < self.dna.helix.strands.len() {
                                let len = self.dna.helix.strands[idx as usize].genes.len();
                                self.stack.push(Value::Int(len as i64));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for gene_len".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for gene_len".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for gene_len".to_string());
                }
                None
            }
            // --- GRID ENZYMES ---
            "g_rows" => {
                self.stack.push(Value::Int(self.grid.len() as i64));
                None
            }
            "g_cols" => {
                if let Some(row) = self.grid.first() {
                    self.stack.push(Value::Int(row.len() as i64));
                } else {
                    self.stack.push(Value::Int(0));
                }
                None
            }
            "g_read" => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow for g_read".to_string());
                    return None;
                }
                let x_val = self.stack.pop().unwrap();
                let y_val = self.stack.pop().unwrap();

                match (y_val, x_val) {
                    (Value::Int(y), Value::Int(x)) => {
                        if y >= 0
                            && (y as usize) < self.grid.len()
                            && x >= 0
                            && (x as usize) < self.grid[y as usize].len()
                        {
                            self.stack.push(self.grid[y as usize][x as usize].clone());
                        } else {
                            self.output.push(format!("Error: Grid coords out of bounds ({}, {})", x, y));
                        }
                    }
                    _ => self.output.push("Error: Type mismatch for g_read coords".to_string()),
                }
                None
            }
            "g_write" => {
                if self.stack.len() < 3 {
                    self.output.push("Error: Stack underflow for g_write".to_string());
                    return None;
                }
                let x_val = self.stack.pop().unwrap();
                let y_val = self.stack.pop().unwrap();
                let val = self.stack.pop().unwrap();

                match (y_val, x_val) {
                    (Value::Int(y), Value::Int(x)) => {
                         if y >= 0
                            && (y as usize) < self.grid.len()
                            && x >= 0
                            && (x as usize) < self.grid[y as usize].len()
                        {
                            self.grid[y as usize][x as usize] = val;
                        } else {
                             self.output.push(format!("Error: Grid coords out of bounds ({}, {})", x, y));
                        }
                    }
                    _ => self.output.push("Error: Type mismatch for g_write coords".to_string()),
                }
                None
            }
            _ => {
                self.output.push(format!("Unknown enzyme: {}", name));
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
                "push",
                "add",
                "sub",
                "mul",
                "div",
                "dup",
                "print",
                "swap",
                "drop",
                "jump",
                "brz",
                "transcribe",
                "s_len",
                "helix_len",
                "gene_len",
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

    pub fn recombine(&mut self) {
        let mut rng = rand::thread_rng();
        let helix = &mut self.dna.helix;

        if helix.strands.len() < 2 {
            self.output.push("Recombination failed: Need >1 strand".to_string());
            return;
        }

        // Pick two distinct strands
        let idx_a = rng.gen_range(0..helix.strands.len());
        let mut idx_b = rng.gen_range(0..helix.strands.len());
        while idx_b == idx_a {
            idx_b = rng.gen_range(0..helix.strands.len());
        }

        let len_a = helix.strands[idx_a].genes.len();
        let len_b = helix.strands[idx_b].genes.len();

        if len_a == 0 || len_b == 0 {
             self.output.push("Recombination failed: Empty strand".to_string());
             return;
        }

        // Determine split point (min length to avoid out of bounds initially,
        // but we can just swap tails)
        let split_point = rng.gen_range(0..len_a.min(len_b));

        // We need to work around the borrow checker to swap elements between two vector elements
        // Splitting the mutable borrow of strands
        if idx_a < idx_b {
            let (left, right) = helix.strands.split_at_mut(idx_b);
            let strand_a = &mut left[idx_a];
            let strand_b = &mut right[0];

            let tail_a = strand_a.genes.split_off(split_point);
            let tail_b = strand_b.genes.split_off(split_point);

            strand_a.genes.extend(tail_b);
            strand_b.genes.extend(tail_a);
        } else {
             let (left, right) = helix.strands.split_at_mut(idx_a);
            let strand_b = &mut left[idx_b];
            let strand_a = &mut right[0];

            let tail_a = strand_a.genes.split_off(split_point);
            let tail_b = strand_b.genes.split_off(split_point);

            strand_a.genes.extend(tail_b);
            strand_b.genes.extend(tail_a);
        }

        self.output.push(format!("RECOMBINATION: Strands {} and {} swapped at index {}", idx_a, idx_b, split_point));
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

    #[test]
    fn test_transcribe() {
        // [ push(0) push(0) push(0) push(99) transcribe() push(0) ]
        // The last push(0) should be modified to push(99)
        // stack order: strand, gene, arg, value
        // strand 0, gene 5 (the last push), arg 0 -> 99
        let strand0 = Strand {
            genes: vec![
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 0: strand idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(5)],
                }, // 1: gene idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 2: arg idx
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(99)],
                }, // 3: value
                Gene {
                    name: "transcribe".to_string(),
                    args: vec![],
                }, // 4: transcribe
                Gene {
                    name: "push".to_string(),
                    args: vec![Nucleotide::Number(0)],
                }, // 5: target to be modified
            ],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..5 {
            vm.step();
        }

        // After transcribe, check if the last gene is modified
        if let Nucleotide::Number(n) = vm.dna.helix.strands[0].genes[5].args[0] {
            assert_eq!(n, 99);
        } else {
            panic!("Gene not modified");
        }

        // Execute the modified gene
        vm.step();
        assert_eq!(vm.stack.last().unwrap(), &Value::Int(99));
    }

    #[test]
    fn test_div_by_zero() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                name: "div".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // We expect an error message instead of a panic
        assert!(
            vm.output.iter().any(|s| s.contains("Division by zero")),
            "Expected 'Division by zero' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_div_overflow() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(i64::MIN)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(-1)],
            },
            Gene {
                name: "div".to_string(),
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Division overflow")),
            "Expected 'Division overflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_stack_underflow() {
        let genes = vec![Gene {
            name: "add".to_string(), // Requires 2 args, stack has 0
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Stack underflow")),
            "Expected 'Stack underflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_type_mismatch() {
        let genes = vec![
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                name: "push".to_string(),
                args: vec![Nucleotide::String("foo".to_string())],
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

        assert!(
            vm.output.iter().any(|s| s.contains("Type mismatch")),
            "Expected 'Type mismatch' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_recombination() {
        let strand1 = Strand {
            genes: vec![
                Gene { name: "A".to_string(), args: vec![] },
                Gene { name: "A".to_string(), args: vec![] },
                Gene { name: "A".to_string(), args: vec![] },
                Gene { name: "A".to_string(), args: vec![] },
            ]
        };
        let strand2 = Strand {
            genes: vec![
                Gene { name: "B".to_string(), args: vec![] },
                Gene { name: "B".to_string(), args: vec![] },
                Gene { name: "B".to_string(), args: vec![] },
                Gene { name: "B".to_string(), args: vec![] },
            ]
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand1, strand2],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Force recombination
        vm.recombine();

        // Check output log
        assert!(vm.output.iter().any(|s| s.contains("RECOMBINATION")));

        // Verify genes are mixed (this is probabilistic but with AAAA and BBBB and split point,
        // if split > 0 and < 4, we will have mixing).
        // Since we can't easily control RNG in this test without dependency injection,
        // we mainly check that the function runs and produces the log.
        // However, we can check that total number of genes is preserved (4+4=8)

        let count = vm.dna.helix.strands[0].genes.len() + vm.dna.helix.strands[1].genes.len();
        assert_eq!(count, 8);
    }
}
