#![cfg(feature = "cortex")]
//! # The Cortex System 🧠
//!
//! The `Cortex` module transforms the Chimera VM into a **Spiking Neural Network**.
//!
//! In this model:
//! - **Neurons** are DNA Strands.
//! - **Synapses** are directed links between strands.
//! - **Action Potentials (Spikes)** are signals sent via `OpCode::Spark`.
//! - **Membrane Potential** is the `activation_level` of a strand.
//!
//! ## Mechanics
//!
//! 1.  **Connectivity**: Strands can form one-way connections to other strands using `Link`.
//! 2.  **Firing**: When a strand executes `Spark(n)`, it adds `n` to the activation level of all its postsynaptic targets.
//! 3.  **Decay**: At the end of every tick, all activation levels decay by 1 (down to 0).
//! 4.  **Gating**: Strands can conditionally execute code based on their own activation level using `Gate(threshold)`.
//!
//! ## Example: A Simple Oscillator
//!
//! ```ignore
//! // Neuron A (Strand 0): Fires if stimulated, stimulates B
//! gate(5)     // Wait for threshold 5
//! push(10)    // Prepare spike strength
//! spark()     // Fire 10 to Neuron B
//!
//! // Neuron B (Strand 1): Fires if stimulated, stimulates A (Loop)
//! gate(5)
//! push(10)
//! spark()
//! ```

use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;

/// Executes Cortex-related OpCodes (Neural Network Logic).
///
/// This function handles the synaptic plasticity and signaling between strands.
///
/// # Supported Enzymes
///
/// - `Link`: Create a synapse from the current strand to a target.
/// - `Sever`: Remove a synapse.
/// - `Spark`: Fire a signal to all connected strands.
/// - `Sense`: Read the current strand's activation level.
/// - `Gate`: Conditional execution based on activation level.
///
/// # Examples
///
/// ```rust
/// use chimera_lang::prelude::*;
///
/// // Create a 2-neuron network: A -> B
/// // A: [ Link(B), Push(10), Spark ]
/// // B: [ Sense ]
///
/// let genes_a = vec![
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Target B (Index 1)
///     Gene { op: OpCode::Link, args: vec![] },                      // Link A->B
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
///     Gene { op: OpCode::Spark, args: vec![] },                     // Fire!
/// ];
///
/// let genes_b = vec![
///     Gene { op: OpCode::Sense, args: vec![] }, // Read activation
/// ];
///
/// let dna = Dna { evolution_config: None, helix: Helix { strands: vec![Strand { genes: genes_a }, Strand { genes: genes_b }] } };
/// let mut vm = ChimeraVM::new(dna);
///
/// // Step A: Link and Spark
/// vm.step(); // push(1)
/// vm.step(); // link()
/// vm.step(); // push(10)
/// vm.step(); // spark() -> B.activation += 10
///
/// // By default, `vm.step()` executes instructions sequentially.
/// // After Strand A finishes (4 instructions), execution context would naturally move to Strand B.
///
/// // However, `Spark` increases activation immediately.
/// // We verify that Strand B's activation level (Index 1) has increased.
/// // Note: Activation decays by 1 at the end of every full tick cycle.
///
/// assert!(vm.activation_levels[1] >= 9);
/// ```
#[cfg(feature = "cortex")]
pub fn exec_cortex_op(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) {
    match op {
        OpCode::Link => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(target) => {
                        let target_idx = target as usize;
                        let s_idx = vm.ip.0;
                        // Check bounds using activation_levels as proxy for strand count
                        if target_idx < vm.activation_levels.len() && s_idx < vm.synapse_map.len() {
                            if !vm.synapse_map[s_idx].contains(&target_idx) {
                                vm.synapse_map[s_idx].push(target_idx);
                                vm.output.push(format!("LINK: {} -> {}", s_idx, target_idx));
                            }
                        } else {
                            vm.output
                                .push("Error: Invalid strand index for link".to_string());
                        }
                    }
                    _ => vm.output.push("Error: Type mismatch for link".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for link".to_string());
            }
        }
        OpCode::Sever => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(target) => {
                        let target_idx = target as usize;
                        let s_idx = vm.ip.0;
                        if s_idx < vm.synapse_map.len() {
                            if let Some(pos) =
                                vm.synapse_map[s_idx].iter().position(|&x| x == target_idx)
                            {
                                vm.synapse_map[s_idx].remove(pos);
                                vm.output
                                    .push(format!("SEVER: {} -x {}", s_idx, target_idx));
                            }
                        }
                    }
                    _ => vm.output.push("Error: Type mismatch for sever".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sever".to_string());
            }
        }
        OpCode::Spark => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(amount) => {
                        let s_idx = vm.ip.0;
                        if s_idx < vm.synapse_map.len() {
                            let targets = vm.synapse_map[s_idx].clone();
                            let count = targets.len();
                            for target_idx in targets {
                                if target_idx < vm.activation_levels.len() {
                                    vm.activation_levels[target_idx] =
                                        vm.activation_levels[target_idx].saturating_add(amount);
                                }
                            }
                            vm.energy = vm.energy.saturating_sub((count as i64) + 1);
                            vm.output
                                .push(format!("SPARK: Fired {} to {} targets", amount, count));
                        }
                    }
                    _ => vm.output.push("Error: Type mismatch for spark".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for spark".to_string());
            }
        }
        OpCode::Sense => {
            let s_idx = vm.ip.0;
            if s_idx < vm.activation_levels.len() {
                let level = vm.activation_levels[s_idx];
                vm.stack.push(Value::Int(level));
            } else {
                vm.stack.push(Value::Int(0));
            }
        }
        OpCode::Gate => {
            if let Some(Nucleotide::Number(threshold)) = args.first() {
                let s_idx = vm.ip.0;
                if s_idx < vm.activation_levels.len() && vm.activation_levels[s_idx] < *threshold {
                    // Skip next instruction
                    vm.ip.1 += 1;
                }
            } else {
                vm.output.push("Error: Invalid arg for gate".to_string());
            }
        }
        _ => {}
    }
}
