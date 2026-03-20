use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;
use crate::vm::MAX_RECURSION_DEPTH;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_stack_op(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => {
                if let Some(arg) = args.first() {
                    fn nuc_to_val(n: &Nucleotide, depth: usize) -> Option<Value> {
                        if depth > MAX_RECURSION_DEPTH {
                            return None;
                        }
                        match n {
                            Nucleotide::Number(v) => Some(Value::Int(*v)),
                            Nucleotide::String(s) => Some(Value::Str(s.clone())),
                            Nucleotide::Identifier(s) => Some(Value::Str(s.clone())),
                            Nucleotide::Junction(t, list) => {
                                let mut vals = Vec::new();
                                for item in list {
                                    if let Some(v) = nuc_to_val(item, depth + 1) {
                                        vals.push(v);
                                    } else {
                                        return None;
                                    }
                                }
                                Some(Value::Junction(*t, vals))
                            }
                        }
                    }

                    if let Some(val) = nuc_to_val(arg, 0) {
                        self.stack.push(val);
                    } else {
                        self.output
                            .push(format!("Error: Invalid arg for push: {:?}", arg));
                    }
                }
            }
            OpCode::Dup => {
                if let Some(val) = self.stack.last() {
                    let v: Value = Clone::clone(val);
                    self.stack.push(v);
                }
            }
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
            }
            OpCode::Drop => {
                self.stack.pop();
            }
            OpCode::SLen => {
                self.stack.push(Value::Int(self.stack.len() as i64));
            }
            OpCode::HelixLen => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
            }
            OpCode::GeneLen => {
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
            }
            _ => {}
        }
        None
    }
}
