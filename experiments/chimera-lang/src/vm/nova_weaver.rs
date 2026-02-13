#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use rand::Rng;

/// Executes Weave-related OpCodes.
pub fn exec_weave_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Weave => {
            // Stack: [ ..., strand_a, strand_b, pattern ] -> [ ..., new_strand_idx ]
            if vm.stack.len() >= 3 {
                let pattern_val = vm.stack.pop().unwrap();
                let b_val = vm.stack.pop().unwrap();
                let a_val = vm.stack.pop().unwrap();

                if let (Value::Int(idx_a), Value::Int(idx_b)) = (a_val, b_val) {
                    let pattern_str = match pattern_val {
                        Value::Str(s) => s,
                        Value::Int(idx_p) => {
                            // Compile pattern from strand
                            let mut s = String::new();
                            if idx_p >= 0 && (idx_p as usize) < vm.dna.helix.strands.len() {
                                for gene in &vm.dna.helix.strands[idx_p as usize].genes {
                                    // Use first letter of OpCode as pattern char
                                    let op_str = gene.op.to_string();
                                    if let Some(c) = op_str.chars().next() {
                                        s.push(c.to_ascii_uppercase());
                                    }
                                }
                            }
                            s
                        }
                        _ => String::new(),
                    };

                    if idx_a >= 0
                        && idx_b >= 0
                        && (idx_a as usize) < vm.dna.helix.strands.len()
                        && (idx_b as usize) < vm.dna.helix.strands.len()
                    {
                        let strand_a = &vm.dna.helix.strands[idx_a as usize];
                        let strand_b = &vm.dna.helix.strands[idx_b as usize];

                        let mut new_genes = Vec::new();
                        let mut ptr_a = 0;
                        let mut ptr_b = 0;
                        let mut rng = rand::thread_rng();

                        for c in pattern_str.chars() {
                            match c {
                                'A' => {
                                    if ptr_a < strand_a.genes.len() {
                                        new_genes.push(strand_a.genes[ptr_a].clone());
                                        ptr_a += 1;
                                    }
                                }
                                'B' => {
                                    if ptr_b < strand_b.genes.len() {
                                        new_genes.push(strand_b.genes[ptr_b].clone());
                                        ptr_b += 1;
                                    }
                                }
                                'X' => {
                                    // Randomly pick A or B
                                    if rng.gen_bool(0.5) {
                                        if ptr_a < strand_a.genes.len() {
                                            new_genes.push(strand_a.genes[ptr_a].clone());
                                            ptr_a += 1;
                                        }
                                    } else {
                                        if ptr_b < strand_b.genes.len() {
                                            new_genes.push(strand_b.genes[ptr_b].clone());
                                            ptr_b += 1;
                                        }
                                    }
                                }
                                '0' => {
                                    // Skip / Gap (Insert Nop?)
                                    // Let's insert a Nop to preserve structure/timing
                                    new_genes.push(Gene {
                                        op: OpCode::Nop,
                                        args: vec![],
                                    });
                                }
                                _ => {}
                            }
                        }

                        // Append any new strand to Helix
                        let new_strand_idx = vm.dna.helix.strands.len();
                        vm.dna.helix.strands.push(Strand { genes: new_genes });
                        vm.telomeres.push(50); // Default telomere
                        #[cfg(feature = "cortex")]
                        {
                            vm.synapse_map.push(vec![]);
                            vm.activation_levels.push(0);
                        }

                        vm.stack.push(Value::Int(new_strand_idx as i64));
                        vm.output.push(format!(
                            "WEAVE: Created strand {} from {} and {} with pattern '{}'",
                            new_strand_idx, idx_a, idx_b, pattern_str
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid strand indices for Weave".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for Weave strands".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Weave".to_string());
            }
        }
        OpCode::Unravel => {
            // Stack: [ ..., strand_idx ] -> [ ... ]
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if idx >= 0 && (idx as usize) < vm.dna.helix.strands.len() {
                    let u_idx = idx as usize;
                    let genes_count = vm.dna.helix.strands[u_idx].genes.len();

                    // Clear genes
                    vm.dna.helix.strands[u_idx].genes.clear();

                    // Reclaim energy: 1 energy per gene
                    let reclaimed = genes_count as i64;
                    vm.energy = vm.energy.saturating_add(reclaimed);

                    vm.output.push(format!("UNRAVEL: Destroyed strand {}, reclaimed {} energy", idx, reclaimed));
                } else {
                    vm.output.push("Error: Invalid strand index for Unravel".to_string());
                }
            } else {
                vm.output.push("Error: Invalid arg for Unravel".to_string());
            }
        }
        _ => {}
    }
    None
}
