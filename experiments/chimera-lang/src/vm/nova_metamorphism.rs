#![cfg(feature = "nova")]

use super::ChimeraVM;
use crate::ast::{Gene, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;

pub fn process_metamorphism(vm: &mut ChimeraVM) {
    if !vm.metamorphism_enabled {
        return;
    }

    // Thresholds
    let pressure_threshold = 20;
    let heat_threshold = 100;

    let pressure = vm.stack.len();
    let heat = vm.energy;

    let mut rng = rand::thread_rng();

    // Select a random strand to metamorphose
    // We avoid the current strand to prevent instruction pointer invalidation during execution
    let strand_count = vm.dna.helix.strands.len();
    if strand_count == 0 {
        return;
    }

    // Try up to 3 times to find a valid target
    let mut target_idx = None;
    for _ in 0..3 {
        let idx = rng.gen_range(0..strand_count);
        if idx != vm.ip.0 {
            // Also check call stack to ensure we don't break return addresses
            let in_call_stack = vm.call_stack.iter().any(|(s, _)| *s == idx);
            if !in_call_stack {
                target_idx = Some(idx);
                break;
            }
        }
    }

    if let Some(idx) = target_idx {
        let strand = &mut vm.dna.helix.strands[idx];
        let mut changed = false;

        // Apply Pressure (Compression)
        if pressure > pressure_threshold {
            if compress_strand(strand) {
                vm.output
                    .push(format!("METAMORPHISM: Pressure compressed strand {}", idx));
                changed = true;
            }
        }

        // Apply Heat (Expansion)
        // If we just compressed, we skip expansion to avoid immediate undoing (hysteresis)
        if !changed && heat > heat_threshold {
            if expand_strand(strand) {
                vm.output
                    .push(format!("METAMORPHISM: Heat expanded strand {}", idx));
            }
        }
    }
}

/// Compresses sequence: Push(A), Push(B), Op -> Push(Result)
fn compress_strand(strand: &mut crate::ast::Strand) -> bool {
    let mut i = 0;

    // We iterate and build a new gene list if changes occur
    // Or simpler: modify in place using retain/splice?
    // Splicing is complex. Let's try to find ONE pattern and apply it per tick.
    // Iterative optimization is safer.

    while i + 2 < strand.genes.len() {
        let g1 = &strand.genes[i];
        let g2 = &strand.genes[i + 1];
        let g3 = &strand.genes[i + 2];

        if let (OpCode::Push, OpCode::Push) = (&g1.op, &g2.op) {
            if let (Some(n1), Some(n2)) = (get_int_arg(g1), get_int_arg(g2)) {
                let new_val = match g3.op {
                    OpCode::Add => Some(n1.wrapping_add(n2)),
                    OpCode::Sub => Some(n1.wrapping_sub(n2)),
                    OpCode::Mul => Some(n1.wrapping_mul(n2)),
                    _ => None,
                };

                if let Some(res) = new_val {
                    // Replace [i, i+1, i+2] with [Push(res)]
                    let new_gene = Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(res)],
                    };

                    // Remove 3, insert 1
                    strand.genes.remove(i);
                    strand.genes.remove(i);
                    strand.genes.remove(i); // i+2 becomes i after two removes
                    strand.genes.insert(i, new_gene);

                    // We only do one compression per tick to be gentle
                    return true;
                }
            }
        }
        i += 1;
    }

    false
}

/// Expands sequence: Push(C) -> Push(C/2), Push(C - C/2), Add
fn expand_strand(strand: &mut crate::ast::Strand) -> bool {
    let mut i = 0;

    while i < strand.genes.len() {
        let g = &strand.genes[i];

        if g.op == OpCode::Push {
            if let Some(val) = get_int_arg(g) {
                // Don't expand small numbers, it's boring
                if val.abs() > 5 {
                    let half = val / 2;
                    let remainder = val - half;

                    let g1 = Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(half)],
                    };
                    let g2 = Gene {
                        op: OpCode::Push,
                        args: vec![Nucleotide::Number(remainder)],
                    };
                    let g3 = Gene {
                        op: OpCode::Add,
                        args: vec![],
                    };

                    strand.genes.remove(i);
                    strand.genes.insert(i, g3);
                    strand.genes.insert(i, g2);
                    strand.genes.insert(i, g1);

                    return true;
                }
            }
        }
        i += 1;
    }

    false
}

fn get_int_arg(gene: &Gene) -> Option<i64> {
    if let Some(Nucleotide::Number(n)) = gene.args.first() {
        Some(*n)
    } else {
        None
    }
}
