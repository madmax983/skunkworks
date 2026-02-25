#[cfg(feature = "cistron")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "cistron")]
use std::collections::HashMap;
#[cfg(feature = "cistron")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "cistron")]
use crate::ast::Nucleotide;

#[cfg(feature = "cistron")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protein {
    pub concentration: f64,
    pub decay_rate: f64,
}

#[cfg(feature = "cistron")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
    pub target_gene: String,
    pub tf_name: String,
    pub mode: RegulationMode, // 0=Promote, 1=Repress
    pub strength: f64,
}

#[cfg(feature = "cistron")]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegulationMode {
    Promote,
    Repress,
}

#[cfg(feature = "cistron")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CistronState {
    pub proteins: HashMap<String, Protein>,
    pub regulations: Vec<Regulation>,
    pub gene_expression: HashMap<String, f64>,
}

#[cfg(feature = "cistron")]
impl CistronState {
    pub fn new() -> Self {
        Self {
            proteins: HashMap::new(),
            regulations: Vec::new(),
            gene_expression: HashMap::new(),
        }
    }

    pub fn step(&mut self) {
        // Decay proteins
        for protein in self.proteins.values_mut() {
            protein.concentration *= 1.0 - protein.decay_rate;
            if protein.concentration < 0.001 {
                protein.concentration = 0.0;
            }
        }

        // Calculate Expression Levels
        self.gene_expression.clear();

        // Initialize with default? Or compute on fly?
        let mut updates: HashMap<String, f64> = HashMap::new();

        for rule in &self.regulations {
            let tf_conc = self.proteins.get(&rule.tf_name).map(|p| p.concentration).unwrap_or(0.0);
            let effect = tf_conc * rule.strength;

            let entry = updates.entry(rule.target_gene.clone()).or_insert(0.0);

            match rule.mode {
                RegulationMode::Promote => *entry += effect,
                RegulationMode::Repress => *entry -= effect,
            }
        }

        // Apply to state (Base 0.5 + modifiers)
        for (gene, modifier) in updates {
            let val = (0.5 + modifier).clamp(0.0, 1.0);
            self.gene_expression.insert(gene, val);
        }
    }
}

#[cfg(feature = "cistron")]
pub fn exec_cistron_op(vm: &mut ChimeraVM, op: crate::opcode::OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    use crate::opcode::OpCode;

    match op {
        OpCode::Regulate => {
            // Immediate: [ gene, tf, mode, strength ]
            if args.len() >= 4 {
                if let (Nucleotide::String(g), Nucleotide::String(tf), Nucleotide::Number(m), Nucleotide::Number(s)) = (&args[0], &args[1], &args[2], &args[3]) {
                    let mode = if *m == 0 { RegulationMode::Promote } else { RegulationMode::Repress };
                    let strength = (*s as f64) / 100.0;
                    vm.cistron_state.regulations.push(Regulation {
                        target_gene: g.clone(),
                        tf_name: tf.clone(),
                        mode,
                        strength
                    });
                    vm.output.push("CISTRON: Regulation registered (Immediate).".to_string());
                    return None;
                }
            }

            // Stack: [ gene, tf, mode, strength ]
            if vm.stack.len() >= 4 {
                let strength_val = vm.stack.pop().unwrap();
                let mode_val = vm.stack.pop().unwrap();
                let tf_val = vm.stack.pop().unwrap();
                let gene_val = vm.stack.pop().unwrap();

                if let (Value::Str(g), Value::Str(tf), Value::Int(m), Value::Int(s)) = (gene_val, tf_val, mode_val, strength_val) {
                    let mode = if m == 0 { RegulationMode::Promote } else { RegulationMode::Repress };
                    let strength = (s as f64) / 100.0;
                    vm.cistron_state.regulations.push(Regulation {
                        target_gene: g,
                        tf_name: tf,
                        mode,
                        strength
                    });
                    vm.output.push("CISTRON: Regulation registered.".to_string());
                } else {
                    vm.output.push("Error: Type mismatch for Regulate".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for Regulate".to_string());
            }
        }
        OpCode::SynthesizeProtein => {
            // Immediate: [ name, amount, decay ]
            if args.len() >= 3 {
                if let (Nucleotide::String(name), Nucleotide::Number(amt), Nucleotide::Number(dec)) = (&args[0], &args[1], &args[2]) {
                     let protein = vm.cistron_state.proteins.entry(name.clone()).or_insert(Protein {
                        concentration: 0.0,
                        decay_rate: (*dec as f64) / 100.0
                    });
                    protein.concentration += (*amt as f64) / 100.0;
                    protein.concentration = protein.concentration.min(10.0);
                    vm.output.push(format!("CISTRON: Synthesized {} (Conc: {:.2})", name, protein.concentration));
                    return None;
                }
            }

            // Stack: [ name, amount, decay ]
            if vm.stack.len() >= 3 {
                let decay_val = vm.stack.pop().unwrap();
                let amount_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();
                if let (Value::Str(name), Value::Int(amt), Value::Int(dec)) = (name_val, amount_val, decay_val) {
                    let protein = vm.cistron_state.proteins.entry(name.clone()).or_insert(Protein {
                        concentration: 0.0,
                        decay_rate: (dec as f64) / 100.0
                    });
                    protein.concentration += (amt as f64) / 100.0;
                    protein.concentration = protein.concentration.min(10.0);
                    vm.output.push(format!("CISTRON: Synthesized {} (Conc: {:.2})", name, protein.concentration));
                } else {
                    vm.output.push("Error: Type mismatch for SynthesizeProtein".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for SynthesizeProtein".to_string());
            }
        }
        OpCode::SenseProtein => {
            // [ name ] -> [ amount ]
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(name) = val {
                    let conc = vm.cistron_state.proteins.get(&name).map(|p| p.concentration).unwrap_or(0.0);
                    vm.stack.push(Value::Int((conc * 100.0) as i64));
                } else {
                    vm.output.push("Error: Type mismatch for SenseProtein".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for SenseProtein".to_string());
            }
        }
        _ => {}
    }
    None
}
