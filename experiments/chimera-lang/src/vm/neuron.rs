#![cfg(feature = "biophysics")]
use super::ChimeraVM;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::Value;

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
        }
    }

    pub fn step(&mut self, dt: f32) {
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
        self.i_inj *= 0.9;
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
                        if !vm.neurons.contains_key(&coord) {
                            vm.neurons.insert(coord, Neuron::new());
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
            // Placeholder
            vm.output.push("AXON: Not implemented yet".to_string());
        }
        _ => {}
    }
}
