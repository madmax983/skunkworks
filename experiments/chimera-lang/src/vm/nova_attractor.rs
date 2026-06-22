#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
/// Represents a `AttractorState`.
pub struct AttractorState {
    /// The `x` field.
    pub x: f64,
    /// The `y` field.
    pub y: f64,
    /// The `z` field.
    pub z: f64,
    /// The `sigma` field.
    pub sigma: f64,
    /// The `rho` field.
    pub rho: f64,
    /// The `beta` field.
    pub beta: f64,
    /// The `dt` field.
    pub dt: f64,
    /// The `mode` field.
    pub mode: u8, // 0=Lorenz, 1=Rossler, 2=Thomas
    /// The `history` field.
    pub history: VecDeque<(f64, f64, f64)>,
}

impl Default for AttractorState {
    fn default() -> Self {
        Self::new()
    }
}

impl AttractorState {
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
    pub fn new() -> Self {
        Self {
            x: 0.1,
            y: 0.0,
            z: 0.0,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            dt: 0.01,
            mode: 0,
            history: VecDeque::new(),
        }
    }

    /// Performs the `step` operation.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of step
    /// ```
    pub fn step(&mut self) {
        let (dx, dy, dz) = match self.mode {
            0 => {
                // Lorenz
                let dx = self.sigma * (self.y - self.x);
                let dy = self.x * (self.rho - self.z) - self.y;
                let dz = self.x * self.y - self.beta * self.z;
                (dx, dy, dz)
            }
            1 => {
                // Rossler
                // Standard params: a=0.2, b=0.2, c=5.7
                // Mapping sigma->a, rho->b, beta->c
                // Default: sigma=0.2, rho=0.2, beta=5.7
                let dx = -self.y - self.z;
                let dy = self.x + self.sigma * self.y;
                let dz = self.rho + self.z * (self.x - self.beta);
                (dx, dy, dz)
            }
            2 => {
                // Thomas
                // b=0.208186
                // dx = sin(y) - b*x
                // dy = sin(z) - b*y
                // dz = sin(x) - b*z
                let b = self.sigma; // Use sigma as b
                let dx = self.y.sin() - b * self.x;
                let dy = self.z.sin() - b * self.y;
                let dz = self.x.sin() - b * self.z;
                (dx, dy, dz)
            }
            _ => (0.0, 0.0, 0.0),
        };

        self.x += dx * self.dt;
        self.y += dy * self.dt;
        self.z += dz * self.dt;

        if self.history.len() >= 1000 {
            self.history.pop_front();
        }
        self.history.push_back((self.x, self.y, self.z));
    }
}

/// Performs the `exec_attractor_op` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of exec_attractor_op
/// ```
#[allow(dead_code)]
pub(crate) fn exec_attractor_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::AttractorInit => {
            if let Some(Value::Int(mode)) = vm.stack.pop() {
                vm.attractor.mode = mode as u8;
                vm.attractor.x = 0.1;
                vm.attractor.y = 0.0;
                vm.attractor.z = 0.0;
                vm.attractor.history.clear();

                // Set default params based on mode
                match mode {
                    0 => {
                        // Lorenz
                        vm.attractor.sigma = 10.0;
                        vm.attractor.rho = 28.0;
                        vm.attractor.beta = 8.0 / 3.0;
                    }
                    1 => {
                        // Rossler
                        vm.attractor.sigma = 0.2;
                        vm.attractor.rho = 0.2;
                        vm.attractor.beta = 5.7;
                    }
                    2 => {
                        // Thomas
                        vm.attractor.sigma = 0.19; // b
                    }
                    _ => {}
                }

                vm.output
                    .push(format!("ATTRACTOR: Initialized Mode {}", mode));
            }
            None
        }
        OpCode::AttractorStep => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(dt_int) = val {
                    // dt in 1/1000s
                    vm.attractor.dt = (dt_int as f64) / 1000.0;
                    vm.attractor.step();
                }
            } else {
                // Use existing dt
                vm.attractor.step();
            }
            None
        }
        OpCode::AttractorSurf => {
            if let Some(Value::Int(scale)) = vm.stack.pop() {
                // Map Z to strand index
                // Lorenz Z is approx 0-50
                let z = vm.attractor.z;
                let idx = (z * (scale as f64) / 50.0).abs() as usize;

                if idx < vm.dna.helix.strands.len() {
                    vm.output
                        .push(format!("SURF: Riding chaos to Strand {}", idx));
                    return Some((idx, 0));
                }
            }
            None
        }
        OpCode::AttractorMap => {
            if let Some(Value::Int(target)) = vm.stack.pop() {
                match target {
                    0 => {
                        // Map X to Global Entropy
                        // Lorenz X: -20 to 20
                        let chaos = (vm.attractor.x.abs() / 20.0).clamp(0.0, 1.0);
                        vm.glitch_level = chaos as f32;
                        vm.output
                            .push(format!("MAP: Attractor X -> Glitch {:.2}", chaos));
                    }
                    1 => {
                        // Map Y to Havoc Rate
                        // Lorenz Y: -30 to 30
                        let rate = (vm.attractor.y.abs() / 30.0).clamp(0.0, 1.0);
                        vm.havoc.rate = rate;
                        vm.output
                            .push(format!("MAP: Attractor Y -> Havoc {:.2}", rate));
                    }
                    _ => {}
                }
            }
            None
        }
        _ => None,
    }
}
