use crate::ast::JunctionType;
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::value::Value;
#[cfg(feature = "nova")]
use crate::vm::Chirality;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_flow_op(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Jump => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            OpCode::Brz => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    if let Some(val) = self.stack.pop() {
                        fn check_zero(v: &Value) -> bool {
                            match v {
                                Value::Int(i) => *i == 0,
                                Value::Junction(t, vals) => match t {
                                    JunctionType::Any => vals.iter().any(check_zero),
                                    JunctionType::All | JunctionType::Dish => {
                                        vals.iter().all(check_zero)
                                    }
                                },
                                Value::Superposition(states) => {
                                    states.iter().any(|(v, _)| check_zero(v))
                                }
                                _ => false,
                            }
                        }

                        let is_zero = check_zero(&val);

                        #[cfg(feature = "nova")]
                        let condition = if self.chirality == Chirality::Right {
                            !is_zero
                        } else {
                            is_zero
                        };
                        #[cfg(not(feature = "nova"))]
                        let condition = is_zero;

                        if condition {
                            return Some((*n as usize, 0));
                        }
                    } else {
                        self.output
                            .push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            OpCode::JumpS => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            if target >= 0 {
                                return Some((target as usize, 0));
                            } else {
                                self.output.push("Error: Negative jump target".to_string());
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for jump_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for jump_s".to_string());
                }
                None
            }
            OpCode::BrzS => {
                if self.stack.len() >= 2 {
                    let target_val = self.stack.pop().unwrap();
                    let cond_val = self.stack.pop().unwrap();

                    match (target_val, cond_val) {
                        (Value::Int(target), Value::Int(cond)) => {
                            if cond == 0 {
                                if target >= 0 {
                                    return Some((target as usize, 0));
                                } else {
                                    self.output.push("Error: Negative jump target".to_string());
                                }
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for brz_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for brz_s".to_string());
                }
                None
            }
            _ => None,
        }
    }
}
