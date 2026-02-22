use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;

pub fn apply_mutation(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        if gene_idx < strand.genes.len() {
            let mut rng = rand::thread_rng();
            // 50% change op, 50% change arg
            if rng.gen_bool(0.5) {
                let count = OpCode::iter().count();
                if count > 0 {
                    let idx = rng.gen_range(0..count);
                    if let Some(op) = OpCode::iter().nth(idx) {
                        strand.genes[gene_idx].op = op;
                    }
                }
            } else {
                if !strand.genes[gene_idx].args.is_empty() {
                    let val = rng.gen_range(0..100);
                    strand.genes[gene_idx].args[0] = Nucleotide::Number(val);
                }
            }
        }
    }
}

pub fn apply_scramble(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize, radius: f64) {
    let r = radius as usize;
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        let start = gene_idx.saturating_sub(r);
        let end = (gene_idx + r).min(strand.genes.len());

        if start < end {
            let slice = &mut strand.genes[start..end];
            let mut rng = rand::thread_rng();
            slice.shuffle(&mut rng);
        }
    }
}

pub fn apply_purge(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize, radius: f64) {
    let r = radius as usize;
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        let start = gene_idx.saturating_sub(r);
        let end = (gene_idx + r).min(strand.genes.len());

        if start < end {
            strand.genes.drain(start..end);
        }
    }
}

pub fn apply_duplicate(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        // Clone gene and insert
        let gene = if gene_idx < vm.dna.helix.strands[strand_idx].genes.len() {
            Some(vm.dna.helix.strands[strand_idx].genes[gene_idx].clone())
        } else {
            None
        };

        if let Some(g) = gene {
            vm.dna.helix.strands[strand_idx].genes.insert(gene_idx, g);
        }
    }
}

pub fn apply_storm(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize, radius: f64) {
    let r = radius as usize;
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        let start = gene_idx.saturating_sub(r);
        let end = (gene_idx + r).min(strand.genes.len());

        let mut rng = rand::thread_rng();

        for i in start..end {
            if rng.gen_bool(0.5) {
                // Apply Storm mutation
                #[cfg(feature = "elektra")]
                {
                    let storm_ops = [
                        OpCode::Lightning,
                        OpCode::Shock,
                        OpCode::TeslaCoil,
                        OpCode::Electrogenesis,
                        OpCode::Induction,
                        OpCode::Battery,
                        OpCode::Ground,
                        OpCode::CircuitBreaker,
                    ];
                    strand.genes[i].op = storm_ops[rng.gen_range(0..storm_ops.len())].clone();
                }

                #[cfg(all(feature = "nova", not(feature = "elektra")))]
                {
                    let chaos_ops = [
                        OpCode::Chaos,
                        OpCode::Glitch,
                        OpCode::Scramble,
                        OpCode::Disintegrate,
                        OpCode::EntropySurge,
                    ];
                    strand.genes[i].op = chaos_ops[rng.gen_range(0..chaos_ops.len())].clone();
                }

                #[cfg(not(any(feature = "elektra", feature = "nova")))]
                {
                    // Fallback for minimal features
                    let basic_chaos = [OpCode::Drop, OpCode::Swap, OpCode::Nop];
                    strand.genes[i].op = basic_chaos[rng.gen_range(0..basic_chaos.len())].clone();
                }
            }
        }
    }
}

/// Targeted mutation: Replaces occurrences of `from` OpCode with `to` OpCode in the target strand.
/// `probability` determines the chance of replacement for each occurrence (0.0 to 1.0).
pub fn apply_mutagen(
    vm: &mut ChimeraVM,
    strand_idx: usize,
    from: OpCode,
    to: OpCode,
    probability: f64,
) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        let mut rng = rand::thread_rng();
        let mut replacements = 0;

        for gene in &mut strand.genes {
            if gene.op == from {
                if rng.gen_bool(probability.clamp(0.0, 1.0)) {
                    gene.op = to.clone();
                    replacements += 1;
                }
            }
        }

        if replacements > 0 {
            vm.output.push(format!(
                "MUTAGEN: Replaced {} occurrences of {} with {} in strand {}",
                replacements, from, to, strand_idx
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Strand};
    use crate::vm::ChimeraVM;

    fn make_vm() -> ChimeraVM {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            },
        ];
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_duplicate() {
        let mut vm = make_vm();
        apply_duplicate(&mut vm, 0, 1); // Duplicate gene at index 1 (Push 2)
        assert_eq!(vm.dna.helix.strands[0].genes.len(), 4);
        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[1].args[0] {
            assert_eq!(*n, 2);
        }
        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[2].args[0] {
            assert_eq!(*n, 2);
        }
    }

    #[test]
    fn test_purge() {
        let mut vm = make_vm();
        apply_purge(&mut vm, 0, 1, 1.0); // Remove radius 1 around index 1. range [0, 2)
                                         // 1-1=0, 1+1=2. drain(0..2).
        assert_eq!(vm.dna.helix.strands[0].genes.len(), 1);
        // Only index 2 left (Push 3)
        if let Nucleotide::Number(n) = &vm.dna.helix.strands[0].genes[0].args[0] {
            assert_eq!(*n, 3);
        }
    }

    #[test]
    fn test_mutagen() {
        let mut vm = make_vm();
        // Strand 0 has [Push(1), Push(2), Push(3)]

        // Replace Push with Nop
        apply_mutagen(&mut vm, 0, OpCode::Push, OpCode::Nop, 1.0);

        for gene in &vm.dna.helix.strands[0].genes {
            assert_eq!(gene.op, OpCode::Nop);
        }
    }
}
