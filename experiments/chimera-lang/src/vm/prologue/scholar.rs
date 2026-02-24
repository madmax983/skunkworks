use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

/// The Scholar Agent (🎓) seeks knowledge (Books) and scribes new ones.
///
/// State Structure:
/// [XP, Mode, TargetY, TargetX]
/// *   XP: Int (Accumulated knowledge)
/// *   Mode: Int (0=Wander, 1=Read, 2=Write)
/// *   Target: Coordinates of interest
pub fn process_scholar_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // Unpack State
    let (xp, mode, _ty, _tx) = match &agent.state {
        Value::Junction(_, list) if list.len() >= 4 => {
            let xp = if let Value::Int(n) = list[0] { n } else { 0 };
            let mode = if let Value::Int(n) = list[1] { n } else { 0 };
            let ty = if let Value::Int(n) = list[2] { n } else { 0 };
            let tx = if let Value::Int(n) = list[3] { n } else { 0 };
            (xp, mode, ty, tx)
        }
        _ => (0, 0, 0, 0),
    };

    match mode {
        0 => {
            // WANDER / SEEK
            // Look for Books
            if let Some((by, bx)) = find_nearest_rune(grid_snapshot, y, x, "📖") {
                // Move towards it
                if let Some((ny, nx)) = move_towards(y, x, by, bx, grid_snapshot) {
                    updated_agent.state = pack_state(xp, 0, by as i64, bx as i64);
                    return Some((updated_agent, Some((ny, nx))));
                } else {
                    if distance(y, x, by, bx) <= 1 {
                        updated_agent.state = pack_state(xp, 1, by as i64, bx as i64); // Switch to Read
                        return Some((updated_agent, None));
                    }
                }
            }

            // Random wander
            let (ny, nx) = random_move(y, x, grid_snapshot);
            return Some((updated_agent, Some((ny, nx))));
        }
        1 => {
            // READ
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut found_book = false;
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Str(s) = &grid_snapshot[ny][nx] {
                        if s == "📖" {
                            // If we received knowledge signal
                            if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
                                let gained = match val {
                                    Value::Int(_) => 1,
                                    Value::Str(s) => s.len() as i64,
                                    _ => 5,
                                };
                                let new_xp = xp + gained;
                                vm.output.push(format!(
                                    "🎓 Scholar: Gained {} XP (Total: {})",
                                    gained, new_xp
                                ));

                                if new_xp > 50 {
                                    updated_agent.state = pack_state(0, 2, 0, 0);
                                // Graduate
                                } else {
                                    updated_agent.state = pack_state(new_xp, 0, 0, 0);
                                    // Wander
                                }
                            } else {
                                // Inject delayed signal to Book (Request)
                                // Only if we are West of Book (Inputs are directional)
                                if nx == x + 1 && ny == y {
                                    // We are West
                                    if let Some(key) = vm.prologue_state.library.keys().next() {
                                        vm.prologue_state.delayed_signals[y][x] =
                                            Some(Value::Str(key.clone()));
                                    }
                                }
                            }
                            found_book = true;
                            break;
                        }
                    }
                }
            }
            if !found_book {
                updated_agent.state = pack_state(xp, 0, 0, 0);
            }
            return Some((updated_agent, None));
        }
        2 => {
            // GRADUATE
            let (ny, nx) = random_move(y, x, grid_snapshot);
            if matches!(grid_snapshot[ny][nx], Value::Int(0)) {
                vm.grid[y][x] = Value::Str("Φ".to_string()); // Leave a Philosopher Stone behind
                updated_agent.state = pack_state(0, 0, 0, 0);
                return Some((updated_agent, Some((ny, nx))));
            }
            return Some((updated_agent, Some((ny, nx))));
        }
        _ => {}
    }

    Some((updated_agent, None))
}

fn pack_state(xp: i64, mode: i64, ty: i64, tx: i64) -> Value {
    Value::Junction(
        crate::ast::JunctionType::All,
        vec![
            Value::Int(xp),
            Value::Int(mode),
            Value::Int(ty),
            Value::Int(tx),
        ],
    )
}

fn find_nearest_rune(
    grid: &[Vec<Value>],
    y: usize,
    x: usize,
    rune: &str,
) -> Option<(usize, usize)> {
    for r in 1..6 {
        for dy in -(r as i64)..=r as i64 {
            for dx in -(r as i64)..=r as i64 {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Str(s) = &grid[ny][nx] {
                        if s == rune {
                            return Some((ny, nx));
                        }
                    }
                }
            }
        }
    }
    None
}

fn move_towards(
    y: usize,
    x: usize,
    ty: usize,
    tx: usize,
    grid: &[Vec<Value>],
) -> Option<(usize, usize)> {
    let dy = (ty as i64 - y as i64).signum();
    let dx = (tx as i64 - x as i64).signum();

    if dy != 0 {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64) {
            if is_walkable(grid, ny, nx) {
                return Some((ny, nx));
            }
        }
    }
    if dx != 0 {
        if let Some((ny, nx)) = normalize_coords(y as i64, x as i64 + dx) {
            if is_walkable(grid, ny, nx) {
                return Some((ny, nx));
            }
        }
    }
    None
}

fn random_move(y: usize, x: usize, grid: &[Vec<Value>]) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let mut dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    // Simple shuffle logic or pick random index
    let idx = rng.gen_range(0..4);
    dirs.swap(0, idx);

    for (dy, dx) in dirs {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if is_walkable(grid, ny, nx) {
                return (ny, nx);
            }
        }
    }
    (y, x)
}

fn is_walkable(grid: &[Vec<Value>], y: usize, x: usize) -> bool {
    matches!(grid[y][x], Value::Int(0))
}

fn distance(y1: usize, x1: usize, y2: usize, x2: usize) -> i64 {
    (y1 as i64 - y2 as i64).abs() + (x1 as i64 - x2 as i64).abs()
}
