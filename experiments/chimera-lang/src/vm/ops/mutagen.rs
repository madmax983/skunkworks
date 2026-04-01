use crate::vm::pandemonium;
use crate::opcode::OpCode;
use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_mutagen_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 4 {
            let to_val = self.stack.pop().unwrap();
            let from_val = self.stack.pop().unwrap();
            let prob_val = self.stack.pop().unwrap();
            let target_val = self.stack.pop().unwrap();

            if let (
                Value::Str(to_s),
                Value::Str(from_s),
                Value::Int(prob_int),
                Value::Int(target_idx),
            ) = (to_val, from_val, prob_val, target_val)
            {
                if let (Ok(to_op), Ok(from_op)) = (to_s.parse::<OpCode>(), from_s.parse::<OpCode>())
                {
                    let prob = (prob_int as f64) / 100.0;
                    pandemonium::apply_mutagen(self, target_idx as usize, from_op, to_op, prob);
                } else {
                    self.output
                        .push("Error: Invalid OpCode string for Mutagen".to_string());
                }
            } else {
                self.output
                    .push("Error: Type mismatch for Mutagen".to_string());
            }
        } else {
            self.output
                .push("Error: Stack underflow for Mutagen".to_string());
        }
        None
    }
}
