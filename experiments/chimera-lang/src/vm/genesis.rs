use crate::ast::Strand;
use crate::vm::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenesisTrace {
    pub generation: usize,
    pub score: i64,
    pub mutation_desc: String,
    pub accepted: bool,
}

/// Runs a multi-generational evolutionary algorithm to optimize a subject strand based on a fitness strand.
///
/// # Arguments
/// * `vm` - The current VM state (will be cloned for simulation).
/// * `subject_idx` - The index of the strand to evolve.
/// * `fitness_idx` - The index of the strand that evaluates fitness.
/// * `generations` - Number of generations to run.
///
/// # Returns
/// A tuple containing:
/// * `bool` - Whether evolution found a better (or equal) solution.
/// * `i64` - The final score.
/// * `Vec<GenesisTrace>` - History of the evolution.
/// * `Option<Strand>` - The best strand found (if any).
pub fn evolve(
    vm: &ChimeraVM,
    subject_idx: usize,
    fitness_idx: usize,
    generations: usize,
) -> (bool, i64, Vec<GenesisTrace>, Option<Strand>) {
    let mut traces = Vec::new();
    let best_strand = if subject_idx < vm.dna.helix.strands.len() {
        vm.dna.helix.strands[subject_idx].clone()
    } else {
        return (false, 0, traces, None);
    };

    // Calculate baseline score
    let mut baseline_vm = vm.clone();
    // Silence output for simulation
    baseline_vm.output.clear();

    // 1. Run Subject
    baseline_vm.ip = (subject_idx, 0);
    baseline_vm.halted = false;
    run_strand(&mut baseline_vm, 1000, Some(fitness_idx));

    // 2. Run Fitness
    // Fitness strand should inspect stack/grid/energy and push a score.
    baseline_vm.ip = (fitness_idx, 0);
    baseline_vm.halted = false;
    run_strand(&mut baseline_vm, 1000, None);

    // 3. Get Score
    let mut best_score = if let Some(Value::Int(s)) = baseline_vm.stack.last() {
        *s
    } else {
        i64::MIN // Punishment for no output
    };

    traces.push(GenesisTrace {
        generation: 0,
        score: best_score,
        mutation_desc: "Baseline".to_string(),
        accepted: true,
    });

    let mut current_best_strand = best_strand.clone();

    for gen in 1..=generations {
        let mut sim_vm = vm.clone();

        // Inject current best strand
        if subject_idx < sim_vm.dna.helix.strands.len() {
            sim_vm.dna.helix.strands[subject_idx] = current_best_strand.clone();
        }

        // Mutate
        sim_vm.mutate_strand(subject_idx);
        // Capture what happened
        let mutation_desc = sim_vm
            .output
            .last()
            .cloned()
            .unwrap_or_else(|| "No mutation".to_string());

        let mutated_strand = if subject_idx < sim_vm.dna.helix.strands.len() {
            sim_vm.dna.helix.strands[subject_idx].clone()
        } else {
            current_best_strand.clone()
        };

        // Run Simulation
        sim_vm.output.clear();
        sim_vm.ip = (subject_idx, 0);
        sim_vm.halted = false;
        run_strand(&mut sim_vm, 1000, Some(fitness_idx));

        // Run Fitness
        sim_vm.ip = (fitness_idx, 0);
        sim_vm.halted = false;
        run_strand(&mut sim_vm, 1000, None);

        // Score
        let score = if let Some(Value::Int(s)) = sim_vm.stack.last() {
            *s
        } else {
            i64::MIN
        };

        let accepted = score > best_score;
        if accepted {
            best_score = score;
            current_best_strand = mutated_strand;
        }

        traces.push(GenesisTrace {
            generation: gen,
            score,
            mutation_desc,
            accepted,
        });
    }

    (true, best_score, traces, Some(current_best_strand))
}

fn run_strand(vm: &mut ChimeraVM, max_ticks: usize, stop_at_strand: Option<usize>) {
    for _ in 0..max_ticks {
        vm.step();
        if vm.halted {
            break;
        }
        if vm.ip.0 >= vm.dna.helix.strands.len() {
            break;
        }

        // Prevent fallthrough to forbidden strand (e.g. fitness strand)
        if let Some(stop) = stop_at_strand {
            if vm.ip.0 == stop {
                break;
            }
        }

        // Optional: Stop if we leave the initial strand?
        // No, we might call helper functions.
    }
}
