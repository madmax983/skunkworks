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
            OpCode::Photosynthesize => {
                self.energy = self.energy.saturating_add(5);
            }
            OpCode::Consume => {
                if let Some(val) = self.stack.pop() {
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
                } else {
                    self.output
                        .push("Error: Stack underflow for consume".to_string());
                }
            }
            OpCode::Genome => {
                if self.ip.0 < self.dna.helix.strands.len() {
                    let strand = &self.dna.helix.strands[self.ip.0];
                    self.stack.push(Value::Int(strand.genes.len() as i64));
                    for gene in &strand.genes {
                        self.stack.push(Value::Str(gene.op.to_string()));
                    }
                }
            }
            OpCode::Transcribe => {
                if self.stack.len() < 4 {
                    self.output
                        .push("Error: Stack underflow for transcribe".to_string());
                    return None;
                }
                let val = self.stack.pop()?;
                let arg_idx_val = self.stack.pop()?;
                let gene_idx_val = self.stack.pop()?;
                let strand_idx_val = self.stack.pop()?;

                match (val, arg_idx_val, gene_idx_val, strand_idx_val) {
                    (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) => {
                        let si_idx = si as usize;
                        #[cfg(feature = "nova")]
                        let mut success = false;
                        if si >= 0 && si_idx < self.dna.helix.strands.len() {
                            let strand = &mut self.dna.helix.strands[si_idx];
                            if gi >= 0 && (gi as usize) < strand.genes.len() {
                                let gene = &mut strand.genes[gi as usize];
                                if ai >= 0 && (ai as usize) < gene.args.len() {
                                    gene.args[ai as usize] = Nucleotide::Number(v);
                                    self.output.push(format!(
                                        "TRANSCRIBE: strand {} gene {} arg {} -> {}",
                                        si, gi, ai, v
                                    ));
                                    #[cfg(feature = "nova")]
                                    {
                                        success = true;
                                    }
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

                        #[cfg(feature = "nova")]
                        if success {
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
                    _ => self
                        .output
                        .push("Error: Type mismatch for transcribe args".to_string()),
                }
            }
            _ => {}
        }
        None
    }
}
