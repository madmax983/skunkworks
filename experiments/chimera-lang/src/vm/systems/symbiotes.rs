use crate::vm::ChimeraVM;

impl ChimeraVM {
    #[cfg(feature = "nova")]
    pub(crate) fn process_symbiotes(&mut self) {
        let count = self.symbiotes.len();
        for i in 0..count {
            if self.energy <= 0 {
                break;
            }

            // Swap execution context
            let mut sym_ip = self.symbiotes[i];
            std::mem::swap(&mut self.ip, &mut sym_ip);

            // Execute logic (simplified version of step checks)
            let helix_len = self.dna.helix.strands.len();
            if self.ip.0 < helix_len {
                let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
                if self.ip.1 < strand_len {
                    // Check telomeres/epigenetics? For now, skip for symbiotes to avoid complexity
                    let (gene_op, gene_args) = {
                        let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
                        (gene.op.clone(), gene.args.clone())
                    };

                    let jump_target = self.execute_gene(gene_op, &gene_args);
                    if let Some(target) = jump_target {
                        self.ip = target;
                    } else {
                        self.ip.1 += 1;
                    }
                }
            }

            // Swap back
            std::mem::swap(&mut self.ip, &mut sym_ip);
            self.symbiotes[i] = sym_ip;
        }
    }
}
