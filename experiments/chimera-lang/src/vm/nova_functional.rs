#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, MAX_GENES_PER_STRAND, MAX_STRANDS};
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

fn register_new_strand(
    vm: &mut ChimeraVM,
    genes: Vec<Gene>,
    parent_idx: Option<usize>,
    label: String,
) -> usize {
    vm.dna.helix.strands.push(Strand { genes });
    vm.telomeres.push(50);
    #[cfg(feature = "cortex")]
    {
        vm.activation_levels.push(0);
        vm.synapse_map.push(Vec::new());
    }

    let new_idx = vm.dna.helix.strands.len() - 1;
    vm.cladistics
        .register_strand(new_idx, parent_idx, vm.tick_counter, label);
    new_idx
}

/// Concatenates two strands into a new function.
/// **Stack:** `[ ..., strand_g, strand_f ] -> [ ..., new_strand_idx ]`
/// **Effect:** Creates `new_strand = f + g` (f executes then g).
fn exec_chain(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // We need 2 args. Check stack length first.
    if vm.stack.len() < 2 {
        vm.output.push("CHAIN ERROR: Stack underflow".to_string());
        return None;
    }

    // Pop both items first to preserve stack behavior on error
    let f_val = vm.stack.pop().unwrap();
    let g_val = vm.stack.pop().unwrap();

    let (Value::Int(f_idx_i64), Value::Int(g_idx_i64)) = (f_val, g_val) else {
        vm.output.push("CHAIN ERROR: Type mismatch".to_string());
        return None;
    };

    let f_idx = f_idx_i64 as usize;
    let g_idx = g_idx_i64 as usize;

    let helix_len = vm.dna.helix.strands.len();
    if f_idx >= helix_len || g_idx >= helix_len {
        vm.output
            .push("CHAIN ERROR: Invalid strand index".to_string());
        return None;
    }

    if helix_len >= MAX_STRANDS {
        vm.output
            .push("CHAIN ERROR: Strand limit exceeded".to_string());
        return None;
    }

    let genes_f = vm.dna.helix.strands[f_idx].genes.clone();
    let genes_g = vm.dna.helix.strands[g_idx].genes.clone();

    // 🔒 WARDEN: Check length limit
    let new_len = genes_f.len() + genes_g.len();
    if new_len > MAX_GENES_PER_STRAND {
        vm.output.push(format!(
            "CHAIN ERROR: Result length {} exceeds limit {}",
            new_len, MAX_GENES_PER_STRAND
        ));
        return None;
    }

    let mut new_genes = genes_f;
    new_genes.extend(genes_g);

    let new_idx = register_new_strand(
        vm,
        new_genes,
        Some(f_idx),
        format!("Chain({}, {})", f_idx, g_idx),
    );

    vm.stack.push(Value::Int(new_idx as i64));
    vm.energy = vm.energy.saturating_sub(15);
    vm.output.push(format!("CHAIN: Created strand {}", new_idx));

    None
}

/// Partially applies a value to a strand.
/// **Stack:** `[ ..., value, strand_idx ] -> [ ..., new_strand_idx ]`
/// **Effect:** Creates `new_strand = [push(value)] + strand`.
fn exec_curry(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 {
        vm.output.push("CURRY ERROR: Stack underflow".to_string());
        return None;
    }

    // Pop both items first
    let s_val = vm.stack.pop().unwrap();
    let arg_val = vm.stack.pop().unwrap();

    let Value::Int(s_idx_i64) = s_val else {
        vm.output.push("CURRY ERROR: Type mismatch".to_string());
        return None;
    };
    let s_idx = s_idx_i64 as usize;

    let helix_len = vm.dna.helix.strands.len();
    if s_idx >= helix_len {
        vm.output
            .push("CURRY ERROR: Invalid strand index".to_string());
        return None;
    }

    if helix_len >= MAX_STRANDS {
        vm.output
            .push("CURRY ERROR: Strand limit exceeded".to_string());
        return None;
    }

    // 🔒 WARDEN: Check length limit (+1 for push)
    let current_len = vm.dna.helix.strands[s_idx].genes.len();
    if current_len + 1 > MAX_GENES_PER_STRAND {
        vm.output.push(format!(
            "CURRY ERROR: Result length {} exceeds limit {}",
            current_len + 1,
            MAX_GENES_PER_STRAND
        ));
        return None;
    }

    // Convert Value back to Nucleotide for Push
    let Some(nuc) = crate::vm::nova_genetics::value_to_nucleotide(&arg_val, 0) else {
        vm.output
            .push("CURRY ERROR: Cannot curry complex value".to_string());
        return None;
    };

    let push_gene = Gene {
        op: OpCode::Push,
        args: vec![nuc],
    };

    let mut new_genes = vec![push_gene];
    new_genes.extend(vm.dna.helix.strands[s_idx].genes.clone());

    let new_idx = register_new_strand(vm, new_genes, Some(s_idx), format!("Curry({})", s_idx));

    vm.stack.push(Value::Int(new_idx as i64));
    vm.energy = vm.energy.saturating_sub(10);
    vm.output.push(format!("CURRY: Created strand {}", new_idx));

    None
}

/// Consumes the next instruction as a string literal.
/// **Stack:** `[ ... ] -> [ ..., op_string ]`
fn exec_quote(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (s_idx, g_idx) = vm.ip;

    // Check strand valid (should be if we are executing it)
    if s_idx >= vm.dna.helix.strands.len() {
        // Should effectively never happen during execution but safe to guard
        return None;
    }

    let strand = &vm.dna.helix.strands[s_idx];

    if g_idx + 1 >= strand.genes.len() {
        vm.output
            .push("QUOTE ERROR: Unexpected end of strand".to_string());
        return None;
    }

    let next_gene = &strand.genes[g_idx + 1];
    vm.stack.push(Value::Str(next_gene.op.to_string()));

    // Advance IP to skip the quoted gene.
    // The main loop increments IP once after this returns None.
    // So we increment once here.
    vm.ip.1 += 1;

    vm.output.push(format!("QUOTE: {}", next_gene.op));
    None
}
