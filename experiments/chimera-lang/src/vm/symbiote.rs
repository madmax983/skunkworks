use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioteSegment {
    pub x: usize,
    pub y: usize,
    pub energy: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbiote {
    pub active: bool,
    pub energy: i64,
    pub segments: Vec<SymbioteSegment>,
}

impl Symbiote {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            active: true,
            energy: 100,
            segments: vec![SymbioteSegment { x, y, energy: 10 }],
        }
    }

    pub fn tick(&mut self, _vm: &mut ChimeraVM) {
        if !self.active {
            return;
        }
        self.energy = self.energy.saturating_sub(1);
        if self.energy <= 0 {
            self.active = false;
        }
    }
}

pub fn exec_spawn_symbiote(vm: &mut ChimeraVM) {
    if vm.symbiote_entity.is_none() {
        let (cy, cx) = vm.context_loc;
        vm.symbiote_entity = Some(Symbiote::new(cx, cy));
        vm.output.push(format!("SYMBIOTE: Spawned at {},{}", cx, cy));
    } else {
        vm.output.push("SYMBIOTE: Already exists".to_string());
    }
}

pub fn exec_feed_symbiote(vm: &mut ChimeraVM) {
    if let Some(mut symbiote) = vm.symbiote_entity.take() {
        if let Some(Value::Int(amount)) = vm.stack.pop() {
            symbiote.energy = symbiote.energy.saturating_add(amount);
            vm.output.push(format!("SYMBIOTE: Fed {} energy", amount));
        } else {
            vm.output.push("Error: FeedSymbiote requires Int".to_string());
        }
        vm.symbiote_entity = Some(symbiote);
    } else {
        vm.output.push("Error: No Symbiote to feed".to_string());
    }
}

pub fn exec_symbiote_op(vm: &mut ChimeraVM) {
    if let Some(mut symbiote) = vm.symbiote_entity.take() {
        if let Some(Value::Int(op)) = vm.stack.pop() {
            match op {
                0 => {
                    // Move
                    if vm.stack.len() >= 2 {
                        let dx = match vm.stack.pop() {
                            Some(Value::Int(x)) => x,
                            _ => 0,
                        };
                        let dy = match vm.stack.pop() {
                            Some(Value::Int(y)) => y,
                            _ => 0,
                        };
                        if let Some(head) = symbiote.segments.first_mut() {
                            head.x = (head.x as i64 + dx).rem_euclid(crate::vm::GRID_SIZE as i64) as usize;
                            head.y = (head.y as i64 + dy).rem_euclid(crate::vm::GRID_SIZE as i64) as usize;
                            vm.output.push(format!("SYMBIOTE: Moved to {},{}", head.x, head.y));
                        }
                    }
                }
                _ => {
                    vm.output.push(format!("Error: Unknown SymbioteOp {}", op));
                }
            }
        }
        vm.symbiote_entity = Some(symbiote);
    } else {
        vm.output.push("Error: No Symbiote active".to_string());
    }
}
