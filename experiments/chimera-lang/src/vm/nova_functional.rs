#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, MAX_STRANDS};
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;

/// Executes functional programming OpCodes.
pub fn exec_functional_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Chain => exec_chain(vm),
        OpCode::Curry => exec_curry(vm),
        OpCode::Quote => exec_quote(vm),
        _ => None,
    }
}

/// Concatenates two strands into a new function.
/// **Stack:** `[ ..., strand_g, strand_f ] -> [ ..., new_strand_idx ]`
/// **Effect:** Creates `new_strand = f + g` (f executes then g).
fn exec_chain(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let f_val = vm.stack.pop().unwrap();
        let g_val = vm.stack.pop().unwrap();

        if let (Value::Int(f_idx), Value::Int(g_idx)) = (f_val, g_val) {
            let f = f_idx as usize;
            let g = g_idx as usize;
            let helix_len = vm.dna.helix.strands.len();

            if f < helix_len && g < helix_len {
                if helix_len >= MAX_STRANDS {
                    vm.output
                        .push("CHAIN ERROR: Strand limit exceeded".to_string());
                    return None;
                }

                let genes_f = vm.dna.helix.strands[f].genes.clone();
                let genes_g = vm.dna.helix.strands[g].genes.clone();

                let mut new_genes = genes_f;
                new_genes.extend(genes_g);

                vm.dna.helix.strands.push(Strand { genes: new_genes });
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }

                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.cladistics.register_strand(
                    new_idx,
                    Some(f), // Parent F
                    vm.tick_counter,
                    format!("Chain({}, {})", f, g),
                );

                vm.stack.push(Value::Int(new_idx as i64));
                vm.energy = vm.energy.saturating_sub(15);
                vm.output.push(format!("CHAIN: Created strand {}", new_idx));
            } else {
                vm.output
                    .push("CHAIN ERROR: Invalid strand index".to_string());
            }
        } else {
            vm.output.push("CHAIN ERROR: Type mismatch".to_string());
        }
    } else {
        vm.output.push("CHAIN ERROR: Stack underflow".to_string());
    }
    None
}

/// Partially applies a value to a strand.
/// **Stack:** `[ ..., value, strand_idx ] -> [ ..., new_strand_idx ]`
/// **Effect:** Creates `new_strand = [push(value)] + strand`.
fn exec_curry(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s_val = vm.stack.pop().unwrap();
        let arg_val = vm.stack.pop().unwrap();

        if let Value::Int(s_idx) = s_val {
            let s = s_idx as usize;
            if s < vm.dna.helix.strands.len() {
                if vm.dna.helix.strands.len() >= MAX_STRANDS {
                    vm.output
                        .push("CURRY ERROR: Strand limit exceeded".to_string());
                    return None;
                }

                // Convert Value back to Nucleotide for Push
                if let Some(nuc) = crate::vm::nova_genetics::value_to_nucleotide(&arg_val, 0) {
                    let push_gene = Gene {
                        op: OpCode::Push,
                        args: vec![nuc],
                    };

                    let mut new_genes = vec![push_gene];
                    new_genes.extend(vm.dna.helix.strands[s].genes.clone());

                    vm.dna.helix.strands.push(Strand { genes: new_genes });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    let new_idx = vm.dna.helix.strands.len() - 1;
                    vm.cladistics.register_strand(
                        new_idx,
                        Some(s),
                        vm.tick_counter,
                        format!("Curry({})", s),
                    );

                    vm.stack.push(Value::Int(new_idx as i64));
                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output.push(format!("CURRY: Created strand {}", new_idx));
                } else {
                    vm.output
                        .push("CURRY ERROR: Cannot curry complex value".to_string());
                }
            } else {
                vm.output
                    .push("CURRY ERROR: Invalid strand index".to_string());
            }
        } else {
            vm.output.push("CURRY ERROR: Type mismatch".to_string());
        }
    } else {
        vm.output.push("CURRY ERROR: Stack underflow".to_string());
    }
    None
}

/// Consumes the next instruction as a string literal.
/// **Stack:** `[ ... ] -> [ ..., op_string ]`
fn exec_quote(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (s_idx, g_idx) = vm.ip;
    if s_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[s_idx];
        if g_idx + 1 < strand.genes.len() {
            let next_gene = &strand.genes[g_idx + 1];
            vm.stack.push(Value::Str(next_gene.op.to_string()));

            // Advance IP to skip the quoted gene.
            // The main loop increments IP once after this returns None.
            // So we increment once here.
            vm.ip.1 += 1;

            vm.output.push(format!("QUOTE: {}", next_gene.op));
        } else {
            vm.output
                .push("QUOTE ERROR: Unexpected end of strand".to_string());
        }
    }
    None
}
