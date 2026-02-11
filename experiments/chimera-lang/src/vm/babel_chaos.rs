use crate::opcode::OpCode;
use crate::ast::Nucleotide;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use serde::{Deserialize, Serialize};

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

    pub fn confuse(&mut self) {
        if self.all_ops.is_empty() { return; }
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
                if let Value::Int(amount) = val {
                    vm.babel_state.integrity = (vm.babel_state.integrity - (amount as f64 / 100.0)).clamp(0.0, 1.0);
                    vm.output.push(format!("BABEL: Integrity degraded to {:.2}", vm.babel_state.integrity));
                }
            }
        }
        OpCode::Clarify => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    vm.babel_state.integrity = (vm.babel_state.integrity + (amount as f64 / 100.0)).clamp(0.0, 1.0);
                    vm.output.push(format!("BABEL: Integrity restored to {:.2}", vm.babel_state.integrity));
                }
            }
        }
        OpCode::Confuse => {
            vm.babel_state.confuse();
            vm.output.push("BABEL: The language has been confounded!".to_string());
        }
        _ => {}
    }
}
