use super::ChimeraVM;
use crate::ast::{Gene, Strand, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;

pub fn perform_ritual(
    vm: &mut ChimeraVM,
    parent_a_idx: usize,
    parent_b_idx: usize,
    sacrifice_cost: i64,
) -> Result<String, String> {
    if vm.energy < sacrifice_cost {
        return Err("Insufficient Energy for Ritual".to_string());
    }

    let helix_len = vm.dna.helix.strands.len();
    if parent_a_idx >= helix_len || parent_b_idx >= helix_len {
        return Err("Invalid parent strand index".to_string());
    }

    vm.energy -= sacrifice_cost;

    let parent_a = &vm.dna.helix.strands[parent_a_idx];
    let parent_b = &vm.dna.helix.strands[parent_b_idx];

    let mut new_genes = Vec::new();
    let mut rng = rand::thread_rng();

    // Ritual Logic: Chaotic Interleave
    // The more sacrifice, the higher the "Chaos Factor" (mutation chance)
    let chaos_factor = (sacrifice_cost as f64 / 100.0).clamp(0.01, 0.9);

    let len_a = parent_a.genes.len();
    let len_b = parent_b.genes.len();
    let max_len = std::cmp::max(len_a, len_b);

    for i in 0..max_len {
        let gene_source = if i < len_a && i < len_b {
            if rng.gen_bool(0.5) {
                &parent_a.genes[i]
            } else {
                &parent_b.genes[i]
            }
        } else if i < len_a {
            &parent_a.genes[i]
        } else {
            &parent_b.genes[i]
        };

        let mut gene = gene_source.clone();

        // Apply Ritual Mutation
        if rng.gen_bool(chaos_factor) {
            // 50/50 Chance: Mutate Op or Mutate Arg
            if rng.gen_bool(0.5) {
                // Mutate Arg (randomize number)
                if !gene.args.is_empty() {
                    if let Nucleotide::Number(_) = gene.args[0] {
                        gene.args[0] = Nucleotide::Number(rng.gen_range(0..100));
                    }
                }
            } else {
                // Mutate Op (Shuffle? Or just swap with neighbor?)
                // Since we can't easily generate random OpCode without strum iterator imported (which we can do),
                // let's just duplicate the gene for now (stutter)
                new_genes.push(gene.clone());
            }
        }

        new_genes.push(gene);
    }

    // High sacrifice bonus: Insert "Life" or "Chaos" genes
    if sacrifice_cost >= 50 {
        new_genes.insert(0, Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(sacrifice_cost)],
        });
        new_genes.insert(1, Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });
    }

    let new_strand = Strand { genes: new_genes };
    vm.dna.helix.strands.push(new_strand);
    let new_idx = vm.dna.helix.strands.len() - 1;

    Ok(format!("Ritual Complete. Birthed Strand {} (Chaos: {:.2})", new_idx, chaos_factor))
}
