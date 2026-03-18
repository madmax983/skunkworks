#![cfg(feature = "biophysics")]
use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Neuron {
    // State variables
    pub v: f32, // Membrane potential (mV)
    pub m: f32, // Na activation
    pub h: f32, // Na inactivation
    pub n: f32, // K activation

    // Parameters (tunable via mutations?) - For now fixed
    pub c_m: f32,
    pub g_na: f32,
    pub g_k: f32,
    pub g_l: f32,
    pub e_na: f32,
    pub e_k: f32,
    pub e_l: f32,

    pub i_inj: f32,
    pub last_spike: u64,
    pub receptors: [(f32, f32); 3], // (sensitivity, threshold) for 3 hormone channels
    pub plasticity: f32,
    pub stdp_window: u64,
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new()
    }
}

impl Neuron {
    pub fn new() -> Self {
        Self {
            v: -65.0,
            m: 0.05,
            h: 0.6,
            n: 0.32,
            c_m: 1.0,
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            e_na: 50.0,
            e_k: -77.0,
            e_l: -54.387,
            i_inj: 0.0,
            last_spike: 0,
            receptors: [(0.0, 0.0); 3],
            plasticity: 0.1,
            stdp_window: 10,
        }
    }

    pub fn step(&mut self, dt: f32, tick: u64) -> bool {
        let v = self.v;
        let alpha_n = if (v + 55.0).abs() < 1e-5 {
            0.1
        } else {
            0.01 * (v + 55.0) / (1.0 - (-(v + 55.0) / 10.0).exp())
        };
        let beta_n = 0.125 * (-(v + 65.0) / 80.0).exp();

        let alpha_m = if (v + 40.0).abs() < 1e-5 {
            1.0
        } else {
            0.1 * (v + 40.0) / (1.0 - (-(v + 40.0) / 10.0).exp())
        };
        let beta_m = 4.0 * (-(v + 65.0) / 18.0).exp();

        let alpha_h = 0.07 * (-(v + 65.0) / 20.0).exp();
        let beta_h = 1.0 / (1.0 + (-(v + 35.0) / 10.0).exp());

        let dn = alpha_n * (1.0 - self.n) - beta_n * self.n;
        let dm = alpha_m * (1.0 - self.m) - beta_m * self.m;
        let dh = alpha_h * (1.0 - self.h) - beta_h * self.h;

        let i_na = self.g_na * self.m.powi(3) * self.h * (v - self.e_na);
        let i_k = self.g_k * self.n.powi(4) * (v - self.e_k);
        let i_l = self.g_l * (v - self.e_l);

        let dv = (self.i_inj - i_na - i_k - i_l) / self.c_m;

        self.v += dv * dt;
        self.n += dn * dt;
        self.m += dm * dt;
        self.h += dh * dt;

        // Decay injected current to prevent accumulation without input
        self.i_inj *= 0.99;

        if self.v > 0.0 && tick > self.last_spike + 20 {
            self.last_spike = tick;
            return true;
        }
        false
    }
}

pub fn exec_biophysics_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::NeuroGenesis => {
            // Stack: [ ..., y, x ]
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if vm.is_valid_coord(y, x) {
                        let coord = (y as usize, x as usize);
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            vm.neurons.entry(coord)
                        {
                            e.insert(Neuron::new());
                            vm.energy = vm.energy.saturating_sub(20);
                            vm.output
                                .push(format!("NEUROGENESIS: Created neuron at {},{}", x, y));
                        } else {
                            vm.output.push(format!(
                                "NEUROGENESIS: Neuron already exists at {},{}",
                                x, y
                            ));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid coordinate for neurogenesis".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for neurogenesis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for neurogenesis".to_string());
            }
        }
        OpCode::Stimulate => {
            // Stack: [ ..., amount, y, x ]
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let amt_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(amt)) = (y_val, x_val, amt_val) {
                    let coord = (y as usize, x as usize);
                    if let Some(neuron) = vm.neurons.get_mut(&coord) {
                        neuron.i_inj += amt as f32;
                        vm.output.push(format!(
                            "STIMULATE: Injected {} into neuron at {},{}",
                            amt, x, y
                        ));
                    } else {
                        vm.output
                            .push(format!("STIMULATE: No neuron at {},{}", x, y));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for stimulate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for stimulate".to_string());
            }
        }
        OpCode::Dendrite => {
            // Stack: [ ..., y, x ] -> [ ..., voltage ]
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    let coord = (y as usize, x as usize);
                    if let Some(neuron) = vm.neurons.get(&coord) {
                        vm.stack.push(Value::Int(neuron.v as i64));
                    } else {
                        vm.stack.push(Value::Int(0));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for dendrite".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for dendrite".to_string());
            }
        }
        OpCode::Axon => {
            // Stack: [ ..., x_source, y_source, x_target, y_target ] (Top)
            // Popping order: y_target, x_target, y_source, x_source
            if vm.stack.len() >= 4 {
                let y_tgt_val = vm.stack.pop().unwrap();
                let x_tgt_val = vm.stack.pop().unwrap();
                let y_src_val = vm.stack.pop().unwrap();
                let x_src_val = vm.stack.pop().unwrap();

                if let (Value::Int(ys), Value::Int(xs), Value::Int(yt), Value::Int(xt)) =
                    (y_src_val, x_src_val, y_tgt_val, x_tgt_val)
                {
                    if vm.is_valid_coord(ys, xs) && vm.is_valid_coord(yt, xt) {
                        let source = (ys as usize, xs as usize);
                        let target = (yt as usize, xt as usize);

                        if vm.neurons.contains_key(&source) && vm.neurons.contains_key(&target) {
                            vm.biophysics_synapses
                                .entry(source)
                                .or_default()
                                .push((target, 1.0)); // Default weight 1.0
                            vm.output.push(format!(
                                "AXON: Connected ({},{}) -> ({},{})",
                                xs, ys, xt, yt
                            ));
                        } else {
                            vm.output
                                .push("AXON: Neurons must exist at both ends".to_string());
                        }
                    } else {
                        vm.output.push("AXON: Invalid coordinates".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for axon".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for axon".to_string());
            }
        }
        OpCode::Receptor => {
            // Stack: [ ..., channel, sensitivity, threshold, y, x ]
            if vm.stack.len() >= 5 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let t_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                let c_val = vm.stack.pop().unwrap();

                if let (
                    Value::Int(x),
                    Value::Int(y),
                    Value::Int(thresh),
                    Value::Int(sens),
                    Value::Int(chan),
                ) = (x_val, y_val, t_val, s_val, c_val)
                {
                    if vm.is_valid_coord(y, x) {
                        let coord = (y as usize, x as usize);
                        if let Some(neuron) = vm.neurons.get_mut(&coord) {
                            let channel_idx = (chan.unsigned_abs() as usize) % 3;
                            // Sensitivity is scaled by 10.0 (e.g., 10 = 1.0)
                            let sensitivity = sens as f32 / 10.0;
                            let threshold = thresh as f32;
                            neuron.receptors[channel_idx] = (sensitivity, threshold);
                            vm.output.push(format!(
                                "RECEPTOR: Added channel {} (sens={:.1}, thresh={:.1}) at {},{}",
                                channel_idx, sensitivity, threshold, x, y
                            ));
                        } else {
                            vm.output
                                .push(format!("RECEPTOR: No neuron at {},{}", x, y));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid coordinate for receptor".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for receptor".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for receptor".to_string());
            }
        }
        OpCode::NeuroCoupling => {
            // Stack: [ ..., weight, y, x ]
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let w_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(w)) = (y_val, x_val, w_val) {
                    if vm.is_valid_coord(y, x) {
                        let coord = (y as usize, x as usize);
                        if vm.neurons.contains_key(&coord) {
                            // Weight 100 = 1.0
                            let weight = w as f32 / 100.0;
                            vm.biophysics_couplings.insert(coord, weight);
                            vm.output
                                .push(format!("NEURO_COUPLING: Set {:.2} at {},{}", weight, x, y));
                        } else {
                            vm.output
                                .push(format!("NEURO_COUPLING: No neuron at {},{}", x, y));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid coordinate for NeuroCoupling".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for NeuroCoupling".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for NeuroCoupling".to_string());
            }
        }
        OpCode::NeuroSynapse => {
            // Stack: [ ..., strand_idx, y, x ]
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(s_idx)) = (y_val, x_val, s_val) {
                    if vm.is_valid_coord(y, x) {
                        let coord = (y as usize, x as usize);
                        let s_idx = s_idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            if vm.neurons.contains_key(&coord) {
                                vm.neuron_to_cortex_map
                                    .entry(coord)
                                    .or_default()
                                    .push(s_idx);
                                vm.output.push(format!(
                                    "NEURO_SYNAPSE: Connected {},{} -> Strand {}",
                                    x, y, s_idx
                                ));
                            } else {
                                vm.output
                                    .push(format!("NEURO_SYNAPSE: No neuron at {},{}", x, y));
                            }
                        } else {
                            vm.output
                                .push("Error: Invalid strand index for NeuroSynapse".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid coordinate for NeuroSynapse".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for NeuroSynapse".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for NeuroSynapse".to_string());
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::value::Value;
    use crate::vm::ChimeraVM;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_neuron_initialization() {
        let n = Neuron::new();
        assert_eq!(n.v, -65.0);
        assert_eq!(n.i_inj, 0.0);
    }

    #[test]
    fn test_neuron_step_decay() {
        let mut n = Neuron::new();
        n.v = -50.0; // Perturb
                     // Step without input
        for _ in 0..100 {
            n.step(0.1, 0);
        }
        // Should decay towards rest (approx -65)
        assert!(
            n.v < -60.0,
            "Voltage should decay to resting potential, got {}",
            n.v
        );
    }

    #[test]
    fn test_neuron_spike() {
        let mut n = Neuron::new();
        n.i_inj = 50.0; // Strong input
        let mut spiked = false;
        // Step enough times to integrate
        for i in 0..100 {
            if n.step(0.1, i) {
                spiked = true;
                break;
            }
        }
        assert!(spiked, "Neuron should spike with high input");
        assert!(n.last_spike > 0 || spiked); // If spiked, last_spike set
    }

    #[test]
    fn test_neuro_genesis_op() {
        let mut vm = make_vm();
        vm.stack.push(Value::Int(5)); // y
        vm.stack.push(Value::Int(5)); // x

        exec_biophysics_op(&mut vm, OpCode::NeuroGenesis, &[]);

        assert!(vm.neurons.contains_key(&(5, 5)));
    }

    #[test]
    fn test_stimulate_op() {
        let mut vm = make_vm();
        // Create neuron first
        vm.neurons.insert((5, 5), Neuron::new());

        vm.stack.push(Value::Int(100)); // amount
        vm.stack.push(Value::Int(5)); // y
        vm.stack.push(Value::Int(5)); // x

        exec_biophysics_op(&mut vm, OpCode::Stimulate, &[]);

        let n = vm.neurons.get(&(5, 5)).unwrap();
        assert_eq!(n.i_inj, 100.0);
    }

    #[test]
    fn test_dendrite_op() {
        let mut vm = make_vm();
        let mut n = Neuron::new();
        n.v = -40.0;
        vm.neurons.insert((5, 5), n);

        vm.stack.push(Value::Int(5)); // y
        vm.stack.push(Value::Int(5)); // x

        exec_biophysics_op(&mut vm, OpCode::Dendrite, &[]);

        assert_eq!(vm.stack.pop(), Some(Value::Int(-40)));
    }

    #[test]
    fn test_axon_op() {
        let mut vm = make_vm();
        vm.neurons.insert((0, 0), Neuron::new());
        vm.neurons.insert((1, 1), Neuron::new());

        // Stack: y_src, x_src, y_tgt, x_tgt
        // Pushed order: src_x, src_y, tgt_x, tgt_y

        vm.stack.push(Value::Int(0)); // x_src
        vm.stack.push(Value::Int(0)); // y_src
        vm.stack.push(Value::Int(1)); // x_tgt
        vm.stack.push(Value::Int(1)); // y_tgt

        exec_biophysics_op(&mut vm, OpCode::Axon, &[]);

        let synapse = vm.biophysics_synapses.get(&(0, 0)).unwrap();
        assert_eq!(synapse[0], ((1, 1), 1.0));
    }

    #[test]
    fn test_neuro_coupling_op() {
        let mut vm = make_vm();
        vm.neurons.insert((2, 2), Neuron::new());

        vm.stack.push(Value::Int(100)); // weight (1.0)
        vm.stack.push(Value::Int(2)); // y
        vm.stack.push(Value::Int(2)); // x

        exec_biophysics_op(&mut vm, OpCode::NeuroCoupling, &[]);

        let w = vm.biophysics_couplings.get(&(2, 2)).unwrap();
        assert!((w - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_neuro_synapse_op() {
        let mut vm = make_vm();
        vm.neurons.insert((3, 3), Neuron::new());

        // We need a valid strand index. make_vm creates 1 empty strand (idx 0).
        vm.stack.push(Value::Int(0)); // strand_idx
        vm.stack.push(Value::Int(3)); // y
        vm.stack.push(Value::Int(3)); // x

        exec_biophysics_op(&mut vm, OpCode::NeuroSynapse, &[]);

        let strands = vm.neuron_to_cortex_map.get(&(3, 3)).unwrap();
        assert_eq!(strands[0], 0);
    }

    #[test]
    fn test_op_error_handling() {
        let mut vm = make_vm();

        // NeuroGenesis Underflow
        exec_biophysics_op(&mut vm, OpCode::NeuroGenesis, &[]);
        assert!(vm.output.last().unwrap().contains("Stack underflow"));

        // Type Mismatch
        vm.stack.push(Value::Str("foo".to_string()));
        vm.stack.push(Value::Int(0));
        exec_biophysics_op(&mut vm, OpCode::NeuroGenesis, &[]);
        assert!(vm.output.last().unwrap().contains("Type mismatch"));

        // Invalid Coord
        vm.stack.push(Value::Int(1000)); // y
        vm.stack.push(Value::Int(1000)); // x
        exec_biophysics_op(&mut vm, OpCode::NeuroGenesis, &[]);
        assert!(vm.output.last().unwrap().contains("Invalid coordinate"));
    }
}
