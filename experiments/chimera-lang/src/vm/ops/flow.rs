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
            OpCode::Jump => self.apply_jump(args),
            OpCode::Brz => self.apply_brz(args),
            OpCode::JumpS => self.apply_jump_s(),
            OpCode::BrzS => self.apply_brz_s(),
            _ => None,
        }
    }

    fn apply_jump(&mut self, args: &[Nucleotide]) -> Option<(usize, usize)> {
        let Some(Nucleotide::Number(n)) = args.first() else {
            self.output.push("Error: Invalid arg for jump".to_string());
            return None;
        };
        Some((*n as usize, 0))
    }

    fn apply_brz(&mut self, args: &[Nucleotide]) -> Option<(usize, usize)> {
        let Some(Nucleotide::Number(n)) = args.first() else {
            self.output.push("Error: Invalid arg for brz".to_string());
            return None;
        };

        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for brz".to_string());
            return None;
        };

        fn check_zero(v: &Value) -> bool {
            match v {
                Value::Int(i) => *i == 0,
                Value::Junction(t, vals) => match t {
                    JunctionType::Any => vals.iter().any(check_zero),
                    JunctionType::All | JunctionType::Dish => vals.iter().all(check_zero),
                },
                Value::Superposition(states) => states.iter().any(|(v, _)| check_zero(v)),
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

        None
    }

    fn apply_jump_s(&mut self) -> Option<(usize, usize)> {
        let Some(val) = self.stack.pop() else {
            self.output
                .push("Error: Stack underflow for jump_s".to_string());
            return None;
        };

        let Value::Int(target) = val else {
            self.output
                .push("Error: Type mismatch for jump_s".to_string());
            return None;
        };

        if target >= 0 {
            Some((target as usize, 0))
        } else {
            self.output.push("Error: Negative jump target".to_string());
            None
        }
    }

    fn apply_brz_s(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() < 2 {
            self.output
                .push("Error: Stack underflow for brz_s".to_string());
            return None;
        }

        let target_val = self.stack.pop().unwrap();
        let cond_val = self.stack.pop().unwrap();

        let (Value::Int(target), Value::Int(cond)) = (target_val, cond_val) else {
            self.output
                .push("Error: Type mismatch for brz_s".to_string());
            return None;
        };

        if cond != 0 {
            return None;
        }

        if target >= 0 {
            Some((target as usize, 0))
        } else {
            self.output.push("Error: Negative jump target".to_string());
            None
        }
    }
}
