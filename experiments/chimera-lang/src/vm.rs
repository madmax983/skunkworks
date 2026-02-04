use crate::ast::{Dna, Nucleotide};

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
        Self::execute_gene(&mut self.stack, &mut self.output, &gene.name, &gene.args);

        // Move to next gene
        self.ip.1 += 1;
    }

    fn execute_gene(stack: &mut Vec<Value>, output: &mut Vec<String>, name: &str, args: &[Nucleotide]) {
        match name {
            "push" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Nucleotide::Number(n) => stack.push(Value::Int(*n)),
                        Nucleotide::String(s) => stack.push(Value::Str(s.clone())),
                        _ => output.push(format!("Error: Invalid arg for push: {:?}", arg)),
                    }
                }
            }
            "add" => Self::binary_op(stack, output, |a, b| a + b),
            "sub" => Self::binary_op(stack, output, |a, b| a - b),
            "mul" => Self::binary_op(stack, output, |a, b| a * b),
            "div" => Self::binary_op(stack, output, |a, b| a / b),
            "dup" => {
                if let Some(val) = stack.last() {
                    stack.push(val.clone());
                }
            }
            "print" => {
                if let Some(val) = stack.pop() {
                    output.push(format!("{}", val));
                }
            }
            _ => output.push(format!("Unknown enzyme: {}", name)),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};

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
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(10)] },
            Gene { name: "push".to_string(), args: vec![Nucleotide::Number(20)] },
            Gene { name: "add".to_string(), args: vec![] },
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
}
