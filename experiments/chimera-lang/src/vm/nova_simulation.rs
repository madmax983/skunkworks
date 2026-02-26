use super::{ChimeraVM, Value};

pub fn execute_ephemeral_strand(vm: &mut ChimeraVM, strand: &crate::ast::Strand) {
    if vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
        vm.output
            .push("Error: Recursion limit exceeded in ephemeral execution".to_string());
        return;
    }
    vm.recursion_depth += 1;

    for gene in &strand.genes {
        let result = vm.execute_gene_inner(gene.op.clone(), &gene.args);
        if let Some(target) = result {
            vm.ip = target;
            // Jump occurred! Stop ephemeral execution and let the main loop continue from new IP.
            break;
        }
    }

    vm.recursion_depth -= 1;
}

pub fn execute_strand_sync(vm: &mut ChimeraVM, strand_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = vm.dna.helix.strands[strand_idx].clone();
        execute_ephemeral_strand(vm, &strand);
    } else {
        vm.output
            .push("Error: Invalid strand index for sync execution".to_string());
    }
}

/// Runs a predictive simulation to see if the current path leads to death.
///
/// This creates a clone of the VM and runs it forward in time for `ticks` cycles.
/// If the clone halts (runs out of energy), the prophecy returns 1 (Death).
///
/// **OpCode:** `Prophecy`
/// **Stack:** `[ ..., ticks ] -> [ ..., result (1=Death, 0=Life) ]`
#[allow(clippy::needless_range_loop)]
pub fn exec_prophecy(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks (top)
    let ticks = vm.pop_int("prophecy")?;

    if ticks <= 0 {
        vm.output
            .push("Error: Invalid ticks for prophecy".to_string());
        return None;
    }

    let safe_ticks = ticks.min(1000);

    // Clone VM
    let mut sim_vm = vm.clone();

    // Inherit and increment recursion depth to prevent infinite prophecy loops
    sim_vm.recursion_depth += 1;
    if sim_vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
        vm.output
            .push("Error: Simulation depth limit exceeded in prophecy".to_string());
        return None;
    }
    if sim_vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
        vm.output
            .push("Error: Recursion limit exceeded in prophecy".to_string());
        return None;
    }

    sim_vm.output.clear(); // Silence output
    sim_vm.halted = false; // Ensure it can run (unless already dead?)

    // Advance IP to avoid infinite recursion (executing prophecy again)
    // We assume standard sequential flow (IP.1 + 1)
    sim_vm.ip.1 += 1;

    if vm.energy <= 0 {
        // If already dead, prophecy is 1
        vm.stack.push(Value::Int(1));
    } else {
        // Run simulation loop
        for _ in 0..safe_ticks {
            sim_vm.step();
            if sim_vm.halted {
                break;
            }
        }

        // Result: 1 if Dead (halted), 0 if Alive
        let result = if sim_vm.halted { 1 } else { 0 };
        vm.stack.push(Value::Int(result));

        // Cost
        let cost = 50 + (safe_ticks / 2);
        vm.energy = vm.energy.saturating_sub(cost);
        vm.output.push(format!(
            "PROPHECY: Predicted {} (1=Death, 0=Life) in {} ticks",
            result, safe_ticks
        ));
    }

    None
}

pub fn exec_lisp_eval(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s = vm.pop_str("lisp_eval")?;

    match crate::lisp::compile_fragment(&s) {
        Ok(genes) => {
            let strand = crate::ast::Strand { genes };
            execute_ephemeral_strand(vm, &strand);
            vm.output.push("LISP_EVAL: Success".to_string());
        }
        Err(e) => {
            vm.output.push(format!("LISP_EVAL ERROR: {}", e));
        }
    }
    None
}

/// Runs a sandboxed simulation of a specific strand.
///
/// Useful for testing code safely before integrating it into the main genome.
/// The simulation runs in a cloned environment; changes do not affect the real world.
///
/// **OpCode:** `Simulate`
/// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., top_val, final_energy, status ]`
pub fn exec_simulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, strand_idx (bottom)
    let ticks = vm.pop_int("simulate")?;
    let s_idx = vm.pop_int("simulate")?;

    let idx = s_idx as usize;

    if idx >= vm.dna.helix.strands.len() || ticks <= 0 {
        vm.output
            .push("Error: Invalid args for simulate".to_string());
        return None;
    }

    if vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
        vm.output
            .push("Error: Simulation depth limit exceeded".to_string());
        return None;
    }
    if vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
        vm.output
            .push("Error: Recursion limit exceeded".to_string());
        return None;
    }

    // Cap ticks to prevent DoS
    let safe_ticks = ticks.min(1000);

    // Fork VM
    // Cloning `vm` clones everything, which provides an accurate snapshot.
    let mut sim_vm = vm.clone();

    // Setup simulation context
    sim_vm.ip = (idx, 0);
    sim_vm.output.clear(); // Silence output
    sim_vm.halted = false;

    // Run simulation loop
    for _ in 0..safe_ticks {
        sim_vm.step();
        if sim_vm.halted {
            break;
        }
    }

    // Collect Results
    // 1. Top of stack (or 0 if empty)
    let top_val = sim_vm.stack.last().cloned().unwrap_or(Value::Int(0));
    // 2. Final Energy
    let energy = sim_vm.energy;
    // 3. Status (1 = Alive, 0 = Halted/Dead)
    let status = if sim_vm.halted { 0 } else { 1 };

    // Push results to original VM stack
    vm.stack.push(top_val);
    vm.stack.push(Value::Int(energy));
    vm.stack.push(Value::Int(status));

    // Deduct Energy Cost: Base cost + duration cost
    let cost = safe_ticks.saturating_add(50);
    vm.energy = vm.energy.saturating_sub(cost);

    vm.output.push(format!(
        "SIMULATE: Ran strand {} for {} ticks. Status: {}",
        idx, safe_ticks, status
    ));

    None
}

/// Enters a "Dream State" to safely test mutations.
///
/// The VM clones itself and forces a mutation on the target strand.
/// It then runs the simulation for `ticks`.
///
/// - If **Energy increases**: The dream is "realized" (mutation accepted).
/// - If **Energy decreases**: The dream is forgotten (mutation discarded).
/// - If **Entropy is high**: A Nightmare occurs (bad mutation forced).
///
/// **OpCode:** `Dream`
/// **Stack:** `[ ..., ticks, strand_idx ] -> [ ..., result ]`
pub fn exec_dream(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, strand_idx (bottom)
    let ticks = vm.pop_int("dream")?;
    let s_idx = vm.pop_int("dream")?;

    let idx = s_idx as usize;

    if idx >= vm.dna.helix.strands.len() || ticks <= 0 {
        vm.output.push("Error: Invalid args for dream".to_string());
        return None;
    }

    if vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
        vm.output
            .push("Error: Simulation depth limit exceeded".to_string());
        return None;
    }

    // Cap ticks
    let safe_ticks = ticks.min(1000);

    // Clone VM
    let mut dream_vm = vm.clone();

    // Force a mutation
    dream_vm.mutate();
    let mutation_desc = dream_vm
        .output
        .last()
        .cloned()
        .unwrap_or_else(|| "Unknown Mutation".to_string());

    // Capture mutated strand
    let mutated_strand = if idx < dream_vm.dna.helix.strands.len() {
        Some(dream_vm.dna.helix.strands[idx].clone())
    } else {
        None
    };

    // Run simulation
    dream_vm.ip = (idx, 0);
    dream_vm.output.clear();
    dream_vm.halted = false;

    for _ in 0..safe_ticks {
        dream_vm.step();
        if dream_vm.halted {
            break;
        }
    }

    // Evaluate
    let mut success = dream_vm.energy > vm.energy;

    // Nightmare Check
    let (cy, cx) = vm.context_loc;
    let entropy = vm.entropy_grid[cy][cx];
    let is_nightmare = entropy > 50;

    if is_nightmare {
        success = true; // Nightmares are forced
        vm.output
            .push("NIGHTMARE: The Void invades the dream...".to_string());
    }

    // Pay Cost (Base 50 + ticks/2)
    let cost = 50 + (safe_ticks / 2);

    let trace = crate::vm::dream::DreamTrace::new(
        0,
        idx,
        safe_ticks as usize,
        cost,
        dream_vm.energy,
        if dream_vm.halted { 0 } else { 1 },
        mutation_desc,
        mutated_strand,
        success,
        is_nightmare,
        dream_vm.output.clone(),
        None,
    );
    vm.dream_traces.push(trace);

    if success {
        // Adopt DNA
        vm.dna = dream_vm.dna;
        vm.stack.push(Value::Int(1)); // Success
        if is_nightmare {
            vm.output.push("DREAM: Nightmare realized!".to_string());
        } else {
            vm.output.push("DREAM: Mutation accepted".to_string());
        }
    } else {
        vm.stack.push(Value::Int(0)); // Failure
        vm.output.push("DREAM: Mutation discarded".to_string());
    }

    vm.energy = vm.energy.saturating_sub(cost);

    None
}

pub fn exec_lucid(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let amount = vm.pop_int("lucid")?;

    if amount > 0 {
        let cost = amount;
        if vm.energy >= cost {
            vm.energy -= cost;
            let (cy, cx) = vm.context_loc;
            vm.entropy_grid[cy][cx] = vm.entropy_grid[cy][cx].saturating_sub(amount).max(0);
            vm.output.push("LUCIDITY: Clarity restored.".to_string());
        } else {
            vm.output.push("LUCID: Insufficient energy".to_string());
        }
    }
    None
}
