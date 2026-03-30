use crate::opcode::OpCode;
use crate::value::Value;
#[cfg(feature = "nova")]
use crate::vm::Chirality;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_math_op(&mut self, op: OpCode) {
        #[cfg(feature = "nova")]
        let effective_op = if self.chirality == Chirality::Right {
            match op {
                OpCode::Add => OpCode::Sub,
                OpCode::Sub => OpCode::Add,
                OpCode::Mul => OpCode::Div,
                OpCode::Div => OpCode::Mul,
                OpCode::Gt => OpCode::Lt,
                OpCode::Lt => OpCode::Gt,
                _ => op,
            }
        } else {
            op
        };
        #[cfg(not(feature = "nova"))]
        let effective_op = op;

        match effective_op {
            OpCode::Eq => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Int(if a == b { 1 } else { 0 }));
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Gt | OpCode::Lt => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Int(ia), Value::Int(ib)) => {
                            let res = match effective_op {
                                OpCode::Gt => ia > ib,
                                OpCode::Lt => ia < ib,
                                _ => false,
                            };
                            self.stack.push(Value::Int(if res { 1 } else { 0 }));
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Add => {
                // Check for string concatenation
                if self.stack.len() >= 2 {
                    let b_is_str = matches!(self.stack.last(), Some(Value::Str(_)));
                    let a_is_str = matches!(
                        self.stack.get(self.stack.len().saturating_sub(2)),
                        Some(Value::Str(_))
                    );

                    if a_is_str && b_is_str {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if let (Value::Str(mut s1), Value::Str(s2)) = (a, b) {
                            // 🔒 WARDEN: Enforce MAX_STRING_LEN during concatenation
                            if s1.len() + s2.len() > crate::vm::MAX_STRING_LEN {
                                s1.push_str(&s2);
                                s1.truncate(crate::vm::MAX_STRING_LEN);
                                self.output.push(format!(
                                    "ADD: String truncated to {} chars",
                                    crate::vm::MAX_STRING_LEN
                                ));
                            } else {
                                s1.push_str(&s2);
                            }
                            self.stack.push(Value::Str(s1));
                            return;
                        }
                    }
                }
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_add(b));
            }
            OpCode::Sub => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_sub(b));
            }
            OpCode::Mul => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_mul(b));
            }
            OpCode::Div => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a / b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
            }
            OpCode::Mod => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a % b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
            }
            OpCode::BitAnd => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a & b);
            }
            OpCode::BitOr => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a | b);
            }
            OpCode::BitXor => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a ^ b);
            }
            OpCode::BitNot => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.stack.push(Value::Int(!n)),
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                } else {
                    self.output.push("Error: Stack underflow".to_string());
                }
            }
            OpCode::Shl => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| {
                    if b >= 0 {
                        a.wrapping_shl(b as u32)
                    } else {
                        a
                    }
                });
            }
            OpCode::Shr => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| {
                    if b >= 0 {
                        a.wrapping_shr(b as u32)
                    } else {
                        a
                    }
                });
            }
            _ => {}
        }
    }
}
