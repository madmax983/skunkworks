use crate::ast::JunctionType;
use crate::vm::{oracle, MAX_RECURSION_DEPTH};
use std::collections::HashMap;
use crate::value::Value;

impl crate::vm::ChimeraVM {
    pub(crate) fn exec_findall_op(&mut self) -> Option<(usize, usize)> {
        if self.stack.len() >= 2 {
            let goal = self.stack.pop().unwrap();
            let template = self.stack.pop().unwrap();

            let mut solutions = Vec::new();
            oracle::solve(
                &[goal],
                HashMap::new(),
                &self.knowledge_base,
                self,
                &mut solutions,
                0,
            );

            let mut results = Vec::new();
            for subst in solutions {
                results.push(oracle::resolve(&template, &subst));
            }

            let new_val = Value::Junction(JunctionType::All, results);
            if new_val.depth() > MAX_RECURSION_DEPTH {
                self.output
                    .push("Error: FindAll depth limit exceeded".to_string());
            } else {
                self.stack.push(new_val);
            }
        } else {
            self.output
                .push("Error: Stack underflow for findall".to_string());
        }
        None
    }
}
