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
            OpCode::Push => self.apply_push(args),
            OpCode::Dup => self.apply_dup(),
            OpCode::Swap => self.apply_swap(),
            OpCode::Drop => self.apply_drop(),
            OpCode::SLen => self.apply_s_len(),
            OpCode::HelixLen => self.apply_helix_len(),
            OpCode::GeneLen => self.apply_gene_len(),
            _ => {}
        }
        None
    }

    fn apply_push(&mut self, args: &[Nucleotide]) {
        let Some(arg) = args.first() else {
            return;
        };

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

    fn apply_dup(&mut self) {
        let Some(val) = self.stack.last() else {
            return;
        };
        let v: Value = Clone::clone(val);
        self.stack.push(v);
    }

    fn apply_swap(&mut self) {
        let len = self.stack.len();
        if len >= 2 {
            self.stack.swap(len - 1, len - 2);
        } else {
            self.output
                .push("Error: Stack underflow for swap".to_string());
        }
    }

    fn apply_drop(&mut self) {
        self.stack.pop();
    }

    fn apply_s_len(&mut self) {
        self.stack.push(Value::Int(self.stack.len() as i64));
    }

    fn apply_helix_len(&mut self) {
        self.stack
            .push(Value::Int(self.dna.helix.strands.len() as i64));
    }

    fn apply_gene_len(&mut self) {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for gene_len".to_string());
            return;
        };

        let Value::Int(idx) = val else {
            self.output
                .push("Error: Type mismatch for gene_len".to_string());
            return;
        };

        if idx >= 0 && (idx as usize) < self.dna.helix.strands.len() {
            let len = self.dna.helix.strands[idx as usize].genes.len();
            self.stack.push(Value::Int(len as i64));
        } else {
            self.output
                .push("Error: Strand index out of bounds for gene_len".to_string());
        }
    }
}
