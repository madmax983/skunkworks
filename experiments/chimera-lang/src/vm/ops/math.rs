use crate::opcode::OpCode;
use crate::value::Value;
#[cfg(feature = "nova")]
use crate::vm::Chirality;

impl crate::vm::ChimeraVM {
    pub(crate) fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64 + Copy,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        if let Some(res) = a.apply_binary_op(
            b,
            op,
            crate::vm::MAX_RECURSION_DEPTH,
            crate::vm::MAX_JUNCTION_SIZE,
        ) {
            stack.push(res);
        } else {
            output.push("Error: Type mismatch or complexity limit".to_string());
        }
    }

    pub(crate) fn apply_bit_not(&mut self) {
        if self.stack.is_empty() {
            self.output.push("Error: Stack underflow".to_string());
            return;
        }

        let val = self.stack.pop().unwrap();
        let Value::Int(n) = val else {
            self.output.push("Error: Type mismatch".to_string());
            return;
        };

        self.stack.push(Value::Int(!n));
    }

    pub(crate) fn apply_div(&mut self) {
        if self.stack.len() < 2 {
            self.output.push("Error: Stack underflow".to_string());
            return;
        }

        let b_val = self.stack.pop().unwrap();
        let a_val = self.stack.pop().unwrap();

        let (Value::Int(a), Value::Int(b)) = (a_val, b_val) else {
            self.output.push("Error: Type mismatch".to_string());
            return;
        };

        if b == 0 {
            self.output.push("Error: Division by zero".to_string());
        } else if a == i64::MIN && b == -1 {
            self.output.push("Error: Division overflow".to_string());
        } else {
            self.stack.push(Value::Int(a / b));
        }
    }

    pub(crate) fn apply_mod(&mut self) {
        if self.stack.len() < 2 {
            self.output.push("Error: Stack underflow".to_string());
            return;
        }

        let b_val = self.stack.pop().unwrap();
        let a_val = self.stack.pop().unwrap();

        let (Value::Int(a), Value::Int(b)) = (a_val, b_val) else {
            self.output.push("Error: Type mismatch".to_string());
            return;
        };

        if b == 0 {
            self.output.push("Error: Division by zero".to_string());
        } else if a == i64::MIN && b == -1 {
            self.output.push("Error: Division overflow".to_string());
        } else {
            self.stack.push(Value::Int(a % b));
        }
    }

    pub(crate) fn apply_add_str_concat(&mut self) -> bool {
        if self.stack.len() < 2 {
            return false;
        }

        let b_is_str = matches!(self.stack.last(), Some(Value::Str(_)));
        let a_is_str = matches!(
            self.stack.get(self.stack.len().saturating_sub(2)),
            Some(Value::Str(_))
        );

        if !a_is_str || !b_is_str {
            return false;
        }

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        let (Value::Str(s1), Value::Str(s2)) = (a, b) else {
            return false;
        };

        if s1.len().saturating_add(s2.len()) > crate::vm::MAX_STRING_LEN {
            self.output
                .push("Error: String length exceeds maximum allowed length".to_string());
        } else {
            self.stack.push(Value::Str(s1 + &s2));
        }

        true
    }

    pub(crate) fn apply_eq(&mut self) {
        if self.stack.len() < 2 {
            self.output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(Value::Int(if a == b { 1 } else { 0 }));
    }

    pub(crate) fn apply_cmp(&mut self, effective_op: OpCode) {
        if self.stack.len() < 2 {
            self.output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        let (Value::Int(ia), Value::Int(ib)) = (a, b) else {
            self.output.push("Error: Type mismatch".to_string());
            return;
        };

        let res = match effective_op {
            OpCode::Gt => ia > ib,
            OpCode::Lt => ia < ib,
            _ => false,
        };
        self.stack.push(Value::Int(if res { 1 } else { 0 }));
    }

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
            OpCode::Eq => self.apply_eq(),
            OpCode::Gt | OpCode::Lt => self.apply_cmp(effective_op),
            OpCode::Add => {
                if !self.apply_add_str_concat() {
                    Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_add(b));
                }
            }
            OpCode::Sub => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_sub(b));
            }
            OpCode::Mul => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_mul(b));
            }
            OpCode::Div => self.apply_div(),
            OpCode::Mod => self.apply_mod(),
            OpCode::BitAnd => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a & b);
            }
            OpCode::BitOr => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a | b);
            }
            OpCode::BitXor => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a ^ b);
            }
            OpCode::BitNot => self.apply_bit_not(),
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
