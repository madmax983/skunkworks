use super::PrologueAgent;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstralState {
    pub vx: f64,
    pub vy: f64,
    pub px: f64, // Sub-pixel precision X
    pub py: f64, // Sub-pixel precision Y
    pub mass: f64,
}

impl Default for AstralState {
    fn default() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            px: -1.0, // Sentinel value for uninitialized
            py: -1.0,
            mass: 1.0,
        }
    }
}

impl AstralState {
    pub fn to_value(&self) -> Value {
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![
                Value::Int((self.vx * 1000.0) as i64),
                Value::Int((self.vy * 1000.0) as i64),
                Value::Int((self.px * 1000.0) as i64),
                Value::Int((self.py * 1000.0) as i64),
                Value::Int((self.mass * 1000.0) as i64),
            ],
        )
    }

    pub fn from_value(val: &Value) -> Self {
        if let Value::Junction(_, list) = val {
            if list.len() >= 5 {
                let vx = if let Value::Int(i) = list[0] { i as f64 / 1000.0 } else { 0.0 };
                let vy = if let Value::Int(i) = list[1] { i as f64 / 1000.0 } else { 0.0 };
                let px = if let Value::Int(i) = list[2] { i as f64 / 1000.0 } else { -1.0 };
                let py = if let Value::Int(i) = list[3] { i as f64 / 1000.0 } else { -1.0 };
                let mass = if let Value::Int(i) = list[4] { i as f64 / 1000.0 } else { 1.0 };
                return Self { vx, vy, px, py, mass };
            }
        }
        Self::default()
    }
}

pub fn process_astral_agent(
    _vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut state = AstralState::from_value(&agent.state);

    // Initialize position if fresh spawn
    if state.px < 0.0 || state.py < 0.0 {
        state.px = agent.x as f64;
        state.py = agent.y as f64;
    }

    // Calculate Gravity
    let (fx, fy) = calculate_gravity(state.px, state.py, grid_snapshot);

    // Update Velocity (F = ma => a = F/m)
    // Here we assume F is applied directly to velocity for simplicity (dt=1)
    state.vx += fx / state.mass;
    state.vy += fy / state.mass;

    // Apply Friction / Drag (simulates imperfect vacuum or "ether")
    state.vx *= 0.95;
    state.vy *= 0.95;

    // Update Position
    state.px += state.vx;
    state.py += state.vy;

    // Handle Bounds (Wrap)
    if state.px < 0.0 { state.px += GRID_SIZE as f64; }
    if state.px >= GRID_SIZE as f64 { state.px -= GRID_SIZE as f64; }
    if state.py < 0.0 { state.py += GRID_SIZE as f64; }
    if state.py >= GRID_SIZE as f64 { state.py -= GRID_SIZE as f64; }

    // Snap to Grid
    let nx = state.px.round() as usize % GRID_SIZE;
    let ny = state.py.round() as usize % GRID_SIZE;

    // Interaction: If we land on a non-empty cell (that isn't us), maybe interact?
    // For now, we just update position.
    // Future: Executing runes we pass over.

    // Log position for debugging
    // vm.output.push(format!("ASTRAL: Pos({:.2}, {:.2}) Vel({:.2}, {:.2})", state.px, state.py, state.vx, state.vy));

    let mut updated_agent = agent.clone();
    updated_agent.state = state.to_value();

    if nx != agent.x || ny != agent.y {
        Some((updated_agent, Some((ny, nx))))
    } else {
        Some((updated_agent, None)) // Stay put but update state (velocity changed)
    }
}

fn calculate_gravity(px: f64, py: f64, grid: &[Vec<Value>]) -> (f64, f64) {
    let mut total_fx = 0.0;
    let mut total_fy = 0.0;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mass = get_rune_mass(&grid[y][x]);
            if mass > 0.0 {
                let dx = x as f64 - px;
                let dy = y as f64 - py;

                // Handle Toroidal wrapping for shortest distance?
                // Let's keep it simple Euclidean for now, or maybe limit range.
                // Distance squared
                let dist_sq = dx*dx + dy*dy;

                if dist_sq > 0.1 { // Avoid singularity
                    let force = 0.01 * mass / dist_sq; // G * M / r^2
                    let dist = dist_sq.sqrt();

                    // F_vec = F * (r_vec / r)
                    total_fx += force * (dx / dist);
                    total_fy += force * (dy / dist);
                }
            }
        }
    }

    (total_fx, total_fy)
}

fn get_rune_mass(val: &Value) -> f64 {
    match val {
        Value::Str(s) => match s.as_str() {
            "*" => 50.0,  // Black Hole / Heavy Star
            "!" => 10.0,  // Source
            "?" => 10.0,  // Sink
            "&" | "|" | "+" => 5.0, // Gates
            "@" | "K" | "H" => 2.0, // Other Agents
            _ => 0.0,
        },
        Value::Int(n) => (*n as f64).abs().min(10.0), // Integers have mass
        _ => 0.0,
    }
}
