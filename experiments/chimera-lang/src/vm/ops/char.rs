use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_char_op(&mut self) -> Option<(usize, usize)> {
        if let Some(val) = self.stack.pop() {
            match val {
                Value::Int(n) => {
                    // Try to convert to char
                    if let Some(c) = char::from_u32(n as u32) {
                        self.stack.push(Value::Str(c.to_string()));
                    } else {
                        self.output.push("Error: Invalid char code".to_string());
                        self.stack.push(Value::Str("".to_string()));
                    }
                }
                _ => {
                    self.output.push("Error: Type mismatch for Chr".to_string());
                }
            }
        } else {
            self.output
                .push("Error: Stack underflow for Chr".to_string());
        }
        None
    }
}
