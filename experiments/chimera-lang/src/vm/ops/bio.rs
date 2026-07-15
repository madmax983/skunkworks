use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_bio_op(
        &mut self,
        op: OpCode,
        _args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Photosynthesize => self.apply_photosynthesize(),
            OpCode::Consume => self.apply_consume(),
            OpCode::Genome => self.apply_genome(),
            OpCode::Transcribe => self.apply_transcribe(),
            _ => {}
        }
        None
    }

    fn apply_photosynthesize(&mut self) {
        self.energy = self.energy.saturating_add(5);
    }

    fn apply_consume(&mut self) {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for consume".to_string());
            return;
        };

        match val {
            Value::Int(n) => self.energy = self.energy.saturating_add(n),
            Value::Str(s) => self.energy = self.energy.saturating_add(s.len() as i64),
            Value::Junction(_, _) => {
                self.output
                    .push("Error: Cannot consume junction".to_string());
            }
            Value::Superposition(states) => {
                let mut total = 0.0;
                for (v, p) in states {
                    match v {
                        Value::Int(n) => total += (n as f64) * p,
                        Value::Str(s) => total += (s.len() as f64) * p,
                        _ => {}
                    }
                }
                self.energy = self.energy.saturating_add(total as i64);
            }
            Value::Symbol(_) => {
                self.output.push("Error: Cannot consume symbol".to_string());
            }
            Value::Color(r, g, b) => {
                let e = (r as i64 + g as i64 + b as i64) / 3;
                self.energy = self.energy.saturating_add(e);
            }
        }
    }

    fn apply_genome(&mut self) {
        if self.ip.0 >= self.dna.helix.strands.len() {
            return;
        }
        let strand = &self.dna.helix.strands[self.ip.0];
        self.stack.push(Value::Int(strand.genes.len() as i64));
        for gene in &strand.genes {
            self.stack.push(Value::Str(gene.op.to_string()));
        }
    }

    fn apply_transcribe(&mut self) {
        if self.stack.len() < 4 {
            self.output
                .push("Error: Stack underflow for transcribe".to_string());
            return;
        }

        let val = self.stack.pop().unwrap();
        let arg_idx_val = self.stack.pop().unwrap();
        let gene_idx_val = self.stack.pop().unwrap();
        let strand_idx_val = self.stack.pop().unwrap();

        let (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) =
            (val, arg_idx_val, gene_idx_val, strand_idx_val)
        else {
            self.output
                .push("Error: Type mismatch for transcribe args".to_string());
            return;
        };

        let si_idx = si as usize;

        if si < 0 || si_idx >= self.dna.helix.strands.len() {
            self.output
                .push("Error: Strand index out of bounds".to_string());
            return;
        }

        let strand = &mut self.dna.helix.strands[si_idx];

        if gi < 0 || (gi as usize) >= strand.genes.len() {
            self.output
                .push("Error: Gene index out of bounds".to_string());
            return;
        }

        let gene = &mut strand.genes[gi as usize];

        if ai < 0 || (ai as usize) >= gene.args.len() {
            self.output
                .push("Error: Arg index out of bounds".to_string());
            return;
        }

        gene.args[ai as usize] = Nucleotide::Number(v);
        self.output.push(format!(
            "TRANSCRIBE: strand {} gene {} arg {} -> {}",
            si, gi, ai, v
        ));

        #[cfg(feature = "nova")]
        {
            if let Some(&partner_idx) = self.entangled_pairs.get(&si_idx) {
                if partner_idx < self.dna.helix.strands.len() {
                    let p_strand = &mut self.dna.helix.strands[partner_idx];
                    if (gi as usize) < p_strand.genes.len() {
                        let p_gene = &mut p_strand.genes[gi as usize];
                        if (ai as usize) < p_gene.args.len() {
                            p_gene.args[ai as usize] = Nucleotide::Number(v);
                            self.output.push(format!(
                                "ENTANGLEMENT: Transcribed partner {} gene {} arg {}",
                                partner_idx, gi, ai
                            ));
                        }
                    }
                }
            }
        }
    }
}
