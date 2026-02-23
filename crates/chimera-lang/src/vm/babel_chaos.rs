use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BabelState {
    pub integrity: f64, // 1.0 = stable, 0.0 = total chaos
    pub chaos_map: HashMap<OpCode, OpCode>,
    pub all_ops: Vec<OpCode>,
}

impl BabelState {
    pub fn new() -> Self {
        Self {
            integrity: 1.0,
            chaos_map: HashMap::new(),
            all_ops: OpCode::iter().collect(),
        }
    }

    pub fn get_random_op(&self) -> OpCode {
        if self.all_ops.is_empty() {
            return OpCode::Nop;
        }
        let mut rng = rand::thread_rng();
        self.all_ops[rng.gen_range(0..self.all_ops.len())].clone()
    }

    pub fn speak_in_tongues(&self) -> String {
        let mut rng = rand::thread_rng();
        let len = rng.gen_range(5..20);
        let mut parts = Vec::new();

        // Format for ChimeraParser (OpCode::Compile expects this)
        // [ gene() gene() ]
        parts.push("[".to_string());

        for _ in 0..len {
            let op = self.get_random_op();
            // Ensure parens are always present for grammar.pest
            let arg_str = match op {
                OpCode::Push => {
                    if rng.gen_bool(0.5) {
                        format!("({})", rng.gen_range(0..100))
                    } else {
                        format!("(\"chaos\")")
                    }
                }
                OpCode::Jump | OpCode::Brz | OpCode::JumpS | OpCode::BrzS => {
                    format!("({})", rng.gen_range(0..5))
                }
                // Default: empty args ()
                _ => format!("()"),
            };
            parts.push(format!("{}{}", op, arg_str));
        }

        parts.push("]".to_string());
        parts.join(" ")
    }

    pub fn confuse(&mut self) {
        if self.all_ops.is_empty() {
            return;
        }
        let mut rng = rand::thread_rng();

        // Generate some random mappings
        // Scramble 10 pairs
        for _ in 0..10 {
            let from = self.all_ops[rng.gen_range(0..self.all_ops.len())].clone();
            let to = self.all_ops[rng.gen_range(0..self.all_ops.len())].clone();
            self.chaos_map.insert(from, to);
        }
    }

    pub fn reset_map(&mut self) {
        self.chaos_map.clear();
    }
}

pub fn exec_babel_chaos_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Glossolalia => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(amount) => {
                        vm.babel_state.integrity =
                            (vm.babel_state.integrity - (amount as f64 / 100.0)).clamp(0.0, 1.0);
                        vm.output.push(format!(
                            "BABEL: Integrity degraded to {:.2}",
                            vm.babel_state.integrity
                        ));
                    }
                    Value::Str(s) if s == "Speak" => {
                        let speech = vm.babel_state.speak_in_tongues();
                        vm.stack.push(Value::Str(speech));
                        vm.output.push("BABEL: The machine speaks!".to_string());
                    }
                    Value::Str(s) if s == "Grammar" => {
                        // Generate random grammar structure
                        let mut rng = rand::thread_rng();
                        let grammar = Value::Junction(
                            JunctionType::Any,
                            vec![
                                Value::Str("Seq".to_string()),
                                Value::Junction(
                                    JunctionType::Any,
                                    vec![
                                        Value::Str("Match".to_string()),
                                        Value::Str(if rng.gen_bool(0.5) {
                                            "chaos".to_string()
                                        } else {
                                            "void".to_string()
                                        }),
                                    ],
                                ),
                                Value::Junction(
                                    JunctionType::Any,
                                    vec![
                                        Value::Str("Many".to_string()),
                                        Value::Junction(
                                            JunctionType::Any,
                                            vec![
                                                Value::Str("Regex".to_string()),
                                                Value::Str("[a-z]+".to_string()),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        );
                        vm.stack.push(grammar);
                        vm.output
                            .push("BABEL: A chaotic grammar manifests.".to_string());
                    }
                    _ => {}
                }
            }
        }
        OpCode::Clarify => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    vm.babel_state.integrity =
                        (vm.babel_state.integrity + (amount as f64 / 100.0)).clamp(0.0, 1.0);
                    vm.output.push(format!(
                        "BABEL: Integrity restored to {:.2}",
                        vm.babel_state.integrity
                    ));
                }
            }
        }
        OpCode::Confuse => {
            vm.babel_state.confuse();
            vm.output
                .push("BABEL: The language has been confounded!".to_string());
        }
        _ => {}
    }
}
