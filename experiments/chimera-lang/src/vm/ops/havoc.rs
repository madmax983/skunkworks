use crate::opcode::OpCode;
use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_havoc_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::HavocRate => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.havoc.rate = (n as f64) / 100.0,
                        _ => self
                            .output
                            .push("Error: HavocRate requires Int (0-100)".to_string()),
                    }
                }
            }
            OpCode::HavocScope => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.havoc.scope = n as u8,
                        _ => self
                            .output
                            .push("Error: HavocScope requires Int".to_string()),
                    }
                }
            }
            _ => {}
        }
        None
    }
}
