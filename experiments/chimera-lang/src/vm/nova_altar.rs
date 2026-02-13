use crate::vm::ChimeraVM;
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use rand::Rng;

pub fn perform_ritual(vm: &mut ChimeraVM, parent_a: usize, parent_b: usize, sacrifice: i64) -> Result<usize, String> {
    if parent_a >= vm.dna.helix.strands.len() || parent_b >= vm.dna.helix.strands.len() {
        return Err("Invalid parent strand index".to_string());
    }

    // Pay the cost
    let cost = sacrifice.abs();
    if vm.energy < cost {
        return Err("Insufficient energy for ritual".to_string());
    }
    vm.energy -= cost;

    let genes_a = vm.dna.helix.strands[parent_a].genes.clone();
    let genes_b = vm.dna.helix.strands[parent_b].genes.clone();

    let mut new_genes = Vec::new();
    let mut rng = rand::thread_rng();

    // Determine Ritual outcome based on sacrifice
    // High sacrifice = Lower mutation rate (Stability) OR Higher mutation rate (Chaos)?
    // Let's say:
    // < 10: Low energy, high chaos (unstable)
    // 10-50: Balanced
    // > 50: High energy, beneficial mutations?

    let mutation_chance = if sacrifice < 10 {
        0.5 // 50% mutation (Chaos)
    } else if sacrifice < 50 {
        0.1 // 10% mutation
    } else {
        0.01 // 1% mutation (Stable)
    };

    // Crossover Logic (Uniform Crossover for now)
    let len = genes_a.len().max(genes_b.len());
    for i in 0..len {
        let gene_a = genes_a.get(i);
        let gene_b = genes_b.get(i);

        let chosen_gene = match (gene_a, gene_b) {
            (Some(a), Some(b)) => {
                if rng.gen_bool(0.5) { a } else { b }
            }
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => break,
        };

        // Mutation
        if rng.gen_bool(mutation_chance) {
            // Mutate the gene
            // 1. Change OpCode?
            // 2. Change Args?
            if rng.gen_bool(0.3) {
                // Change OpCode (Drastic) -> Nop
                 new_genes.push(Gene {
                    op: OpCode::Nop,
                    args: vec![],
                });
            } else {
                 // Change Argument
                 let mut mutated_gene = chosen_gene.clone();
                 if !mutated_gene.args.is_empty() {
                     let arg_idx = rng.gen_range(0..mutated_gene.args.len());
                     // Assume first arg is usually a number or string
                     // Replace with random number
                     mutated_gene.args[arg_idx] = Nucleotide::Number(rng.gen_range(0..100));
                 }
                 new_genes.push(mutated_gene);
            }
        } else {
            new_genes.push(chosen_gene.clone());
        }
    }

    // High sacrifice bonus: Add a beneficial gene at the end?
    if sacrifice > 100 {
        new_genes.push(Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });
    }

    let new_strand = Strand { genes: new_genes };
    vm.dna.helix.strands.push(new_strand);
    let new_idx = vm.dna.helix.strands.len() - 1;

    vm.output.push(format!("RITUAL COMPLETE: Created Strand {} from {} & {} (Sacrifice: {})", new_idx, parent_a, parent_b, sacrifice));

    Ok(new_idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;

    #[test]
    fn test_ritual_basic() {
        let genes_a = vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }];
        let genes_b = vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }];

        let mut vm = ChimeraVM::new(Dna {
            helix: Helix {
                strands: vec![
                    Strand { genes: genes_a },
                    Strand { genes: genes_b },
                ]
            }
        });

        // Give energy
        vm.energy = 100;

        // Perform ritual
        let res = perform_ritual(&mut vm, 0, 1, 10);
        assert!(res.is_ok());

        let new_idx = res.unwrap();
        assert_eq!(new_idx, 2);
        assert_eq!(vm.dna.helix.strands.len(), 3);

        // Check new strand has genes
        let new_genes = &vm.dna.helix.strands[new_idx].genes;
        assert!(!new_genes.is_empty());
    }
}
