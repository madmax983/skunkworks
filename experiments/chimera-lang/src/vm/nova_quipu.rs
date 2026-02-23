#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuipuState {
    pub cords: Vec<i64>,
    pub active_cord: usize,
}

impl QuipuState {
    pub fn new() -> Self {
        // 16 Cords by default
        Self {
            cords: vec![0; 16],
            active_cord: 0,
        }
    }

    pub fn tie(&mut self, val: i64) {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord] = val;
        }
    }

    pub fn read(&self) -> i64 {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord]
        } else {
            0
        }
    }

    pub fn untie(&mut self) -> i64 {
        if self.active_cord < self.cords.len() {
            let val = self.cords[self.active_cord];
            self.cords[self.active_cord] = 0;
            val
        } else {
            0
        }
    }

    pub fn select_cord(&mut self, idx: usize) {
        if idx < self.cords.len() {
            self.active_cord = idx;
        }
    }

    pub fn tangle(&mut self, other_idx: usize) {
        if self.active_cord < self.cords.len() && other_idx < self.cords.len() {
            let val_b = self.cords[other_idx];
            self.cords[self.active_cord] = self.cords[self.active_cord].wrapping_add(val_b);
        }
    }
}

impl Default for QuipuState {
    fn default() -> Self {
        Self::new()
    }
}

// --- VM Execution Logic ---

#[cfg(feature = "nova")]
pub fn exec_knot(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(n) => {
                vm.quipu.tie(n);
                vm.output.push(format!("KNOT: Tied {}", n));
            }
            _ => vm.output.push("Error: Type mismatch for knot".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for knot".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_unknot(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.quipu.untie();
    vm.stack.push(Value::Int(val));
    vm.output.push(format!("UNKNOT: Untied {}", val));
    None
}

#[cfg(feature = "nova")]
pub fn exec_cord(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(idx) = val {
            if idx >= 0 {
                vm.quipu.select_cord(idx as usize);
                vm.output.push(format!("CORD: Selected {}", idx));
            } else {
                vm.output.push("Error: Negative cord index".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for cord".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cord".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_read_cord(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.quipu.read();
    vm.stack.push(Value::Int(val));
    None
}

#[cfg(feature = "nova")]
pub fn exec_tangle(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(idx) = val {
            if idx >= 0 {
                vm.quipu.tangle(idx as usize);
                vm.output
                    .push(format!("TANGLE: Entangled with cord {}", idx));
            } else {
                vm.output.push("Error: Negative cord index".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for tangle".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for tangle".to_string());
    }
    None
}
