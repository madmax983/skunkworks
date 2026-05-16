#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a `QuipuState`.
pub struct QuipuState {
    /// The `cords` field.
    pub cords: Vec<i64>,
    /// The `active_cord` field.
    pub active_cord: usize,
}

impl QuipuState {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        // 16 Cords by default
        Self {
            cords: vec![0; 16],
            active_cord: 0,
        }
    }

    /// Performs the `tie` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tie
    /// ```
    pub fn tie(&mut self, val: i64) {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord] = val;
        }
    }

    /// Performs the `read` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of read
    /// ```
    pub fn read(&self) -> i64 {
        if self.active_cord < self.cords.len() {
            self.cords[self.active_cord]
        } else {
            0
        }
    }

    /// Performs the `untie` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of untie
    /// ```
    pub fn untie(&mut self) -> i64 {
        if self.active_cord < self.cords.len() {
            let val = self.cords[self.active_cord];
            self.cords[self.active_cord] = 0;
            val
        } else {
            0
        }
    }

    /// Performs the `select_cord` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of select_cord
    /// ```
    pub fn select_cord(&mut self, idx: usize) {
        if idx < self.cords.len() {
            self.active_cord = idx;
        }
    }

    /// Performs the `tangle` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of tangle
    /// ```
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
/// Performs the `exec_knot` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_knot
/// ```
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
/// Performs the `exec_unknot` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_unknot
/// ```
pub fn exec_unknot(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.quipu.untie();
    vm.stack.push(Value::Int(val));
    vm.output.push(format!("UNKNOT: Untied {}", val));
    None
}

#[cfg(feature = "nova")]
/// Performs the `exec_cord` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_cord
/// ```
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
/// Performs the `exec_read_cord` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_read_cord
/// ```
pub fn exec_read_cord(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.quipu.read();
    vm.stack.push(Value::Int(val));
    None
}

#[cfg(feature = "nova")]
/// Performs the `exec_tangle` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_tangle
/// ```
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
