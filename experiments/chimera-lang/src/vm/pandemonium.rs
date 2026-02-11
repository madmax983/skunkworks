use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn apply_mutation(vm: &mut ChimeraVM, strand_idx: usize, gene_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &mut vm.dna.helix.strands[strand_idx];
        if gene_idx < strand.genes.len() {
            let mut rng = rand::thread_rng();
            // 50% change op, 50% change arg
            if rng.gen_bool(0.5) {
                 let enzymes = [
                    OpCode::Push, OpCode::Add, OpCode::Sub, OpCode::Mul, OpCode::Div,
                    OpCode::Dup, OpCode::Swap, OpCode::Drop, OpCode::Print,
                    OpCode::Jump, OpCode::Brz, OpCode::Photosynthesize, OpCode::Consume,
                    OpCode::GRead, OpCode::GWrite,
                 ];
                 let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                 strand.genes[gene_idx].op = new_op;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix, Strand, Gene};
    use crate::vm::ChimeraVM;

    fn make_vm() -> ChimeraVM {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(3)] },
        ];
        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
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
}
