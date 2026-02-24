use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, MAX_STRINGS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicString {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub tension: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub frequency: f64,
    pub damping: f64,
}

pub fn update_strings(vm: &mut ChimeraVM) {
    for s in &mut vm.strings {
        // Simple Harmonic Motion
        // F = -kx - cv
        // Here we simulate wave propagation abstractly via phase and amplitude decay
        s.phase += s.frequency;
        s.amplitude *= s.damping;
        if s.amplitude < 0.001 {
            s.amplitude = 0.0;
        }
    }
}

pub fn exec_string_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::StringNew => {
            if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let t_val = vm.stack.pop().unwrap();
                let l_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(t), Value::Int(l)) =
                    (x_val, y_val, t_val, l_val)
                {
                    if vm.strings.len() >= MAX_STRINGS {
                        vm.output
                            .push("Error: Max strings limit reached".to_string());
                        return None;
                    }

                    let start_x = x as f64;
                    let start_y = y as f64;
                    let length = l as f64;
                    let tension = t as f64;

                    // Default to Horizontal (East) for now
                    let dx = 1.0;
                    let dy = 0.0;

                    let end_x = start_x + dx * length;
                    let end_y = start_y + dy * length;

                    let s = CosmicString {
                        start: (start_x, start_y),
                        end: (end_x, end_y),
                        tension,
                        amplitude: 0.0,
                        phase: 0.0,
                        frequency: 0.1 * tension.max(1.0).sqrt(),
                        damping: 0.99,
                    };

                    vm.strings.push(s);
                    vm.output.push(format!(
                        "STRING: Created len={} tension={}",
                        length, tension
                    ));
                } else {
                    vm.output
                        .push("Error: Type mismatch for string_new".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for string_new".to_string());
            }
        }
        OpCode::StringPluck => {
            if let Some(Value::Int(force)) = vm.stack.pop() {
                let (cx, cy) = vm.context_loc;
                let cx = cx as f64;
                let cy = cy as f64;

                let mut nearest_idx = None;
                let mut min_dist = f64::MAX;

                for (i, s) in vm.strings.iter().enumerate() {
                    let (x1, y1): (f64, f64) = s.start;
                    let (x2, y2): (f64, f64) = s.end;

                    let l2: f64 = (x1 - x2).powi(2) + (y1 - y2).powi(2);
                    let dist = if l2 == 0.0 {
                        (cx - x1).powi(2) + (cy - y1).powi(2)
                    } else {
                        let t = ((cx - x1) * (x2 - x1) + (cy - y1) * (y2 - y1)) / l2;
                        let t = t.max(0.0).min(1.0);
                        let px = x1 + t * (x2 - x1);
                        let py = y1 + t * (y2 - y1);
                        (cx - px).powi(2) + (cy - py).powi(2)
                    };

                    if dist < min_dist {
                        min_dist = dist;
                        nearest_idx = Some(i);
                    }
                }

                if let Some(idx) = nearest_idx {
                    if min_dist < 4.0 {
                        vm.strings[idx].amplitude += force as f64;
                        vm.output
                            .push(format!("STRING: Plucked amp={}", vm.strings[idx].amplitude));
                    }
                }
            }
        }
        OpCode::StringTune => {
            if let Some(Value::Int(tension)) = vm.stack.pop() {
                let (cx, cy) = vm.context_loc;
                let cx = cx as f64;
                let cy = cy as f64;

                let mut nearest_idx = None;
                let mut min_dist = f64::MAX;

                for (i, s) in vm.strings.iter().enumerate() {
                    let (x1, y1): (f64, f64) = s.start;
                    let (x2, y2): (f64, f64) = s.end;
                    let l2: f64 = (x1 - x2).powi(2) + (y1 - y2).powi(2);
                    let dist = if l2 == 0.0 {
                        (cx - x1).powi(2) + (cy - y1).powi(2)
                    } else {
                        let t = ((cx - x1) * (x2 - x1) + (cy - y1) * (y2 - y1)) / l2;
                        let t = t.max(0.0).min(1.0);
                        let px = x1 + t * (x2 - x1);
                        let py = y1 + t * (y2 - y1);
                        (cx - px).powi(2) + (cy - py).powi(2)
                    };

                    if dist < min_dist {
                        min_dist = dist;
                        nearest_idx = Some(i);
                    }
                }

                if let Some(idx) = nearest_idx {
                    if min_dist < 4.0 {
                        vm.strings[idx].tension = tension as f64;
                        vm.strings[idx].frequency = 0.1 * (tension as f64).max(1.0).sqrt();
                        vm.output.push(format!("STRING: Tuned tension={}", tension));
                    }
                }
            }
        }
        OpCode::StringListen => {
            let (cx, cy) = vm.context_loc;
            let cx = cx as f64;
            let cy = cy as f64;

            let mut amp = 0.0;
            let mut min_dist = f64::MAX;

            for s in &vm.strings {
                let (x1, y1): (f64, f64) = s.start;
                let (x2, y2): (f64, f64) = s.end;
                let l2: f64 = (x1 - x2).powi(2) + (y1 - y2).powi(2);
                let dist = if l2 == 0.0 {
                    (cx - x1).powi(2) + (cy - y1).powi(2)
                } else {
                    let t = ((cx - x1) * (x2 - x1) + (cy - y1) * (y2 - y1)) / l2;
                    let t = t.max(0.0).min(1.0);
                    let px = x1 + t * (x2 - x1);
                    let py = y1 + t * (y2 - y1);
                    (cx - px).powi(2) + (cy - py).powi(2)
                };

                if dist < min_dist {
                    min_dist = dist;
                    amp = s.amplitude * (s.phase.sin().abs());
                }
            }

            if min_dist < 4.0 {
                vm.stack.push(Value::Int(amp as i64));
            } else {
                vm.stack.push(Value::Int(0));
            }
        }
        _ => {}
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};

    #[test]
    fn test_string_limit() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::StringNew,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100000; // Infinite energy for test
        vm.telomeres[0] = 10000; // Infinite life

        // Loop runs 6 ops. StringNew is 1 op.
        // To create 300 strings, we need 300 * 6 = 1800 steps.
        for _ in 0..2000 {
            vm.step();
        }

        assert!(vm.strings.len() <= MAX_STRINGS);
        assert!(vm.strings.len() > 0);

        // Check for error message
        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("Max strings limit reached")));
    }
}
