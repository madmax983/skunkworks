use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::Value;
use rand::Rng;

/// Applies Architect Runes for Blueprint Manipulation.
///
/// Runes:
/// *   `⟳` (Rotate): Rotates a Blueprint 90 degrees clockwise.
/// *   `↔` (Flip H): Mirrors the blueprint horizontally.
/// *   `↕` (Flip V): Mirrors the blueprint vertically.
/// *   `❏` (Merge): Overlays the West blueprint onto the North blueprint.
/// *   `▓` (Life): Runs 1 step of Conway's Game of Life on the Blueprint.
/// *   `░` (Noise): Fills Blueprint with random values.
pub fn apply_architect_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    // Helper to get signal from neighbor
    let get_sig = |dy: i64, dx: i64| -> Option<Value> {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            current_signals[ny][nx].clone()
        } else {
            None
        }
    };

    match rune {
        "⟳" => {
            // Rotate: West (Blueprint) -> Self (Rotated)
            if let Some(val) = get_sig(0, -1) {
                if let Some(grid) = value_to_grid(&val) {
                    let rotated = rotate_grid(&grid);
                    let new_val = grid_to_value(&rotated);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
        "↔" => {
            // Flip H: West (Blueprint) -> Self (Flipped)
            if let Some(val) = get_sig(0, -1) {
                if let Some(grid) = value_to_grid(&val) {
                    let flipped: Vec<Vec<Value>> = grid
                        .into_iter()
                        .map(|mut row| {
                            row.reverse();
                            row
                        })
                        .collect();
                    let new_val = grid_to_value(&flipped);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
        "↕" => {
            // Flip V: West (Blueprint) -> Self (Flipped)
            if let Some(val) = get_sig(0, -1) {
                if let Some(mut grid) = value_to_grid(&val) {
                    grid.reverse();
                    let new_val = grid_to_value(&grid);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
        "❏" => {
            // Merge: West (Overlay), North (Base) -> Self (Merged)
            let w_sig = get_sig(0, -1);
            let n_sig = get_sig(-1, 0);

            if let (Some(w_val), Some(n_val)) = (w_sig, n_sig) {
                if let (Some(overlay), Some(base)) = (value_to_grid(&w_val), value_to_grid(&n_val))
                {
                    let merged = merge_grids(&base, &overlay);
                    let new_val = grid_to_value(&merged);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
        "▓" => {
            // Life: West (Blueprint) -> Self (Next Gen)
            if let Some(val) = get_sig(0, -1) {
                if let Some(grid) = value_to_grid(&val) {
                    let next_gen = apply_life_step(&grid);
                    let new_val = grid_to_value(&next_gen);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
        "░" => {
            // Noise: West (Size Int) -> Self (Blueprint)
            if let Some(Value::Int(size)) = get_sig(0, -1) {
                let s = size.max(1).min(32) as usize; // Cap size
                let noise_grid = generate_noise(s, s);
                let new_val = grid_to_value(&noise_grid);
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(new_val);
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}

fn value_to_grid(val: &Value) -> Option<Vec<Vec<Value>>> {
    if let Value::Junction(JunctionType::Dish, rows) = val {
        let mut grid = Vec::new();
        for row in rows {
            if let Value::Junction(JunctionType::Dish, cells) = row {
                grid.push(cells.clone());
            } else {
                return None;
            }
        }
        Some(grid)
    } else {
        None
    }
}

fn grid_to_value(grid: &[Vec<Value>]) -> Value {
    let rows: Vec<Value> = grid
        .iter()
        .map(|row| Value::Junction(JunctionType::Dish, row.clone()))
        .collect();
    Value::Junction(JunctionType::Dish, rows)
}

fn rotate_grid(grid: &[Vec<Value>]) -> Vec<Vec<Value>> {
    if grid.is_empty() {
        return Vec::new();
    }
    let h = grid.len();
    let w = grid[0].len();
    let mut new_grid = vec![vec![Value::Int(0); h]; w]; // Swap dimensions

    for r in 0..h {
        for c in 0..w {
            // new_c = h - 1 - r
            // new_r = c
            if c < new_grid.len() && (h - 1 - r) < new_grid[0].len() {
                new_grid[c][h - 1 - r] = grid[r][c].clone();
            }
        }
    }
    new_grid
}

fn merge_grids(base: &[Vec<Value>], overlay: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let h = base.len().max(overlay.len());
    let w = if h > 0 {
        base.first()
            .map(|r| r.len())
            .unwrap_or(0)
            .max(overlay.first().map(|r| r.len()).unwrap_or(0))
    } else {
        0
    };

    let mut new_grid = vec![vec![Value::Int(0); w]; h];

    // Fill Base
    for r in 0..base.len() {
        for c in 0..base[r].len() {
            if c < base[r].len() {
                new_grid[r][c] = base[r][c].clone();
            }
        }
    }

    // Overlay (Non-zero/Non-empty overwrites)
    for r in 0..overlay.len() {
        for c in 0..overlay[r].len() {
            let val = &overlay[r][c];
            if is_alive(val) {
                if r < h && c < w {
                    new_grid[r][c] = val.clone();
                }
            }
        }
    }
    new_grid
}

fn is_alive(val: &Value) -> bool {
    match val {
        Value::Int(n) => *n != 0,
        Value::Str(s) => !s.is_empty(),
        _ => false, // Treat other types as dead/inert
    }
}

fn apply_life_step(grid: &[Vec<Value>]) -> Vec<Vec<Value>> {
    let h = grid.len();
    if h == 0 {
        return Vec::new();
    }
    let w = grid[0].len();
    let mut new_grid = vec![vec![Value::Int(0); w]; h];

    for r in 0..h {
        for c in 0..w {
            let mut live_neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i64 + dr;
                    let nc = c as i64 + dc;
                    if nr >= 0 && nr < h as i64 && nc >= 0 && nc < w as i64 {
                        if is_alive(&grid[nr as usize][nc as usize]) {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            let was_alive = is_alive(&grid[r][c]);
            if was_alive {
                if live_neighbors == 2 || live_neighbors == 3 {
                    new_grid[r][c] = grid[r][c].clone(); // Survive
                } else {
                    new_grid[r][c] = Value::Int(0); // Die (Under/Overpopulation)
                }
            } else {
                if live_neighbors == 3 {
                    new_grid[r][c] = Value::Int(1); // Reproduce
                } else {
                    new_grid[r][c] = Value::Int(0); // Stay Dead
                }
            }
        }
    }
    new_grid
}

fn generate_noise(h: usize, w: usize) -> Vec<Vec<Value>> {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![Value::Int(0); w]; h];
    for r in 0..h {
        for c in 0..w {
            if rng.gen_bool(0.5) {
                grid[r][c] = Value::Int(1);
            }
        }
    }
    grid
}
