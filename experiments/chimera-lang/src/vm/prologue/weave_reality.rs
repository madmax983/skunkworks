use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use serde::{Deserialize, Serialize};

/// Represents the fundamental physical or computational laws governing a cell in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RealityMode {
    /// The standard default logic rules of the Prologue VM.
    #[default]
    Prologue,
    /// Orca emulation mode, enforcing strict directional causality and timing.
    Orca,
    /// Silicon logic mode, enforcing circuit-like electrical properties.
    Silicon,
    /// Conway's Game of Life cellular automata rules.
    Life,
}

/// The state tracking the current laws of physics applying to each cell of the simulation grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityState {
    /// A 2D map mapping every (Y, X) coordinate to its current governing `RealityMode`.
    pub reality_map: Vec<Vec<RealityMode>>,
}

impl Default for RealityState {
    fn default() -> Self {
        Self {
            reality_map: vec![vec![RealityMode::Prologue; GRID_SIZE]; GRID_SIZE],
        }
    }
}

impl RealityState {
    /// Bootstraps the grid into standard existence.
    ///
    /// By defaulting to `Prologue` mode everywhere, agents can move and execute standard
    /// runes seamlessly until they enter a localized anomaly (like an `Orca` or `Life` bubble).
    ///
    /// ## Examples
    ///
    /// ```
    /// use chimera_lang::vm::prologue::weave_reality::{RealityState, RealityMode};
    ///
    /// let reality = RealityState::new();
    /// // The origin point is standard prologue logic.
    /// assert_eq!(reality.reality_map[0][0], RealityMode::Prologue);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Probes the local laws of physics at a given coordinate.
    ///
    /// Rather than agents tracking their own reality, they query the environment directly.
    /// Out-of-bounds coordinates fall back to the safe `Prologue` mode to prevent
    /// existential panics when checking grid boundaries.
    ///
    /// ## Examples
    ///
    /// ```
    /// use chimera_lang::vm::prologue::weave_reality::{RealityState, RealityMode};
    ///
    /// let reality = RealityState::new();
    /// // Safe bounds checking protects agents at the edge of the known universe.
    /// assert_eq!(reality.get_mode(9999, 9999), RealityMode::Prologue);
    /// ```
    pub fn get_mode(&self, y: usize, x: usize) -> RealityMode {
        if y < GRID_SIZE && x < GRID_SIZE {
            self.reality_map[y][x]
        } else {
            RealityMode::Prologue
        }
    }
}

/// Scans the grid for 'Reality Bubbles' (🌐) and applies their overlapping local rulesets.
///
/// Rather than updating the entire grid sequentially, this functions locates isolated pockets
/// of altered physics (where a 🌐 rune dictates the rules) and paints their area of effect
/// onto the `RealityState` map. This enables multiple paradigms to coexist simultaneously.
///
/// ## Examples
///
/// ```
/// // Not directly executable without a full ChimeraVM context,
/// // but acts as the engine's physics setup phase.
/// ```
pub fn scan_reality_bubbles(vm: &mut ChimeraVM) {
    // Reset map to Prologue
    for row in vm.prologue_state.reality_state.reality_map.iter_mut() {
        for cell in row.iter_mut() {
            *cell = RealityMode::Prologue;
        }
    }

    // We scan for 🌐 runes
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in runes {
        if let Value::Str(s) = &vm.grid[y][x] {
            if s == "🌐" {
                apply_reality_bubble(vm, y, x);
            }
        }
    }
}

fn apply_reality_bubble(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Radius
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx]
            .clone()
            .or_else(|| Some(vm.grid[wy][wx].clone()))
    } else {
        None
    };

    // North: Mode
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx]
            .clone()
            .or_else(|| Some(vm.grid[ny][nx].clone()))
    } else {
        None
    };

    // Resolve Radius (Int or Str->Int)
    let radius = match w_sig {
        Some(Value::Int(n)) => Some(n),
        Some(Value::Str(s)) => s.parse::<i64>().ok(),
        _ => None,
    };

    if let (Some(r), Some(mode_val)) = (radius, n_sig) {
        let mode = match mode_val {
            Value::Int(1) => RealityMode::Orca,
            Value::Str(s) if s == "O" => RealityMode::Orca,
            Value::Int(2) => RealityMode::Silicon,
            Value::Str(s) if s == "S" => RealityMode::Silicon,
            Value::Int(3) => RealityMode::Life,
            Value::Str(s) if s == "L" => RealityMode::Life,
            _ => RealityMode::Prologue,
        };

        if mode == RealityMode::Prologue {
            return;
        }

        // Apply Bubble
        let r = r.max(0);

        crate::vm::iterate_circle(
            #[cfg(feature = "nova")]
            crate::vm::Topology::Torus,
            #[cfg(not(feature = "nova"))]
            crate::vm::Topology::Plane,
            x as i64,
            y as i64,
            r,
            |cx, cy| {
                vm.prologue_state.reality_state.reality_map[cy][cx] = mode;
            },
        );
    }
}
