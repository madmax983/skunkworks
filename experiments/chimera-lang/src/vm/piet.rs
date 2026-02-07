use super::{ChimeraVM, Value};
use crate::vm::GRID_SIZE;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hue {
    Red = 0,
    Yellow = 1,
    Green = 2,
    Cyan = 3,
    Blue = 4,
    Magenta = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lightness {
    Light = 0,
    Normal = 1,
    Dark = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PietColor {
    Color(Hue, Lightness),
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodelChooser {
    Left = 0,
    Right = 1,
}

struct PietState {
    stack: Vec<i64>,
    dp: Direction,
    cc: CodelChooser,
    y: usize,
    x: usize,
    steps: usize,
}

fn rgb_dist(c1: (u8, u8, u8), c2: (u8, u8, u8)) -> i32 {
    let r = (c1.0 as i32) - (c2.0 as i32);
    let g = (c1.1 as i32) - (c2.1 as i32);
    let b = (c1.2 as i32) - (c2.2 as i32);
    r * r + g * g + b * b
}

fn map_color(r: u8, g: u8, b: u8) -> PietColor {
    // Standard Piet Palette
    let palette = [
        // Light
        (
            (255, 192, 192),
            PietColor::Color(Hue::Red, Lightness::Light),
        ),
        (
            (255, 255, 192),
            PietColor::Color(Hue::Yellow, Lightness::Light),
        ),
        (
            (192, 255, 192),
            PietColor::Color(Hue::Green, Lightness::Light),
        ),
        (
            (192, 255, 255),
            PietColor::Color(Hue::Cyan, Lightness::Light),
        ),
        (
            (192, 192, 255),
            PietColor::Color(Hue::Blue, Lightness::Light),
        ),
        (
            (255, 192, 255),
            PietColor::Color(Hue::Magenta, Lightness::Light),
        ),
        // Normal
        ((255, 0, 0), PietColor::Color(Hue::Red, Lightness::Normal)),
        (
            (255, 255, 0),
            PietColor::Color(Hue::Yellow, Lightness::Normal),
        ),
        ((0, 255, 0), PietColor::Color(Hue::Green, Lightness::Normal)),
        (
            (0, 255, 255),
            PietColor::Color(Hue::Cyan, Lightness::Normal),
        ),
        ((0, 0, 255), PietColor::Color(Hue::Blue, Lightness::Normal)),
        (
            (255, 0, 255),
            PietColor::Color(Hue::Magenta, Lightness::Normal),
        ),
        // Dark
        ((192, 0, 0), PietColor::Color(Hue::Red, Lightness::Dark)),
        (
            (192, 192, 0),
            PietColor::Color(Hue::Yellow, Lightness::Dark),
        ),
        ((0, 192, 0), PietColor::Color(Hue::Green, Lightness::Dark)),
        ((0, 192, 192), PietColor::Color(Hue::Cyan, Lightness::Dark)),
        ((0, 0, 192), PietColor::Color(Hue::Blue, Lightness::Dark)),
        (
            (192, 0, 192),
            PietColor::Color(Hue::Magenta, Lightness::Dark),
        ),
        // White / Black
        ((255, 255, 255), PietColor::White),
        ((0, 0, 0), PietColor::Black),
    ];

    let target = (r, g, b);
    let mut best_dist = i32::MAX;
    let mut best_color = PietColor::Black;

    for (c, pc) in palette.iter() {
        let d = rgb_dist(target, *c);
        if d < best_dist {
            best_dist = d;
            best_color = *pc;
        }
    }

    // Threshold? Nah, just nearest neighbor.
    best_color
}

#[allow(clippy::needless_range_loop)]
fn get_piet_grid(vm: &ChimeraVM) -> [[PietColor; GRID_SIZE]; GRID_SIZE] {
    let mut grid = [[PietColor::White; GRID_SIZE]; GRID_SIZE];
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if let Some((r, g, b)) = vm.chroma_grid[y][x].fg {
                grid[y][x] = map_color(r, g, b);
            } else {
                // No color = White (Canvas)
                grid[y][x] = PietColor::White;
            }
        }
    }
    grid
}

fn get_block(
    grid: &[[PietColor; GRID_SIZE]; GRID_SIZE],
    start_y: usize,
    start_x: usize,
) -> (HashSet<(usize, usize)>, PietColor) {
    let color = grid[start_y][start_x];
    let mut block = HashSet::new();
    let mut queue = VecDeque::new();

    block.insert((start_y, start_x));
    queue.push_back((start_y, start_x));

    while let Some((y, x)) = queue.pop_front() {
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in neighbors {
            let ny = y as i64 + dy;
            let nx = x as i64 + dx;
            if ny >= 0 && ny < GRID_SIZE as i64 && nx >= 0 && nx < GRID_SIZE as i64 {
                let ny = ny as usize;
                let nx = nx as usize;
                if grid[ny][nx] == color && !block.contains(&(ny, nx)) {
                    block.insert((ny, nx));
                    queue.push_back((ny, nx));
                }
            }
        }
    }
    (block, color)
}

fn find_exit_edge(
    block: &HashSet<(usize, usize)>,
    dp: Direction,
    cc: CodelChooser,
) -> (usize, usize) {
    // DP determines the edge (Rightmost, Downmost, etc.)
    // CC determines which end of that edge (Left/High or Right/Low relative to DP)

    // Sort logic depends on DP
    // DP Right (0): Look for Max X. Tie breaker: CC Left -> Min Y, CC Right -> Max Y.
    // DP Down (1): Look for Max Y. Tie breaker: CC Left -> Max X, CC Right -> Min X.
    // DP Left (2): Look for Min X. Tie breaker: CC Left -> Max Y, CC Right -> Min Y.
    // DP Up (3): Look for Min Y. Tie breaker: CC Left -> Min X, CC Right -> Max X.

    let mut candidates: Vec<&(usize, usize)> = block.iter().collect();

    match dp {
        Direction::Right => {
            // Max X
            let max_x = candidates.iter().map(|c| c.1).max().unwrap();
            candidates.retain(|c| c.1 == max_x);
            // Tie breaker
            candidates.sort_by_key(|c| c.0); // Sort by Y
            if cc == CodelChooser::Right {
                // Max Y
                **candidates.last().unwrap()
            } else {
                // Min Y
                **candidates.first().unwrap()
            }
        }
        Direction::Down => {
            // Max Y
            let max_y = candidates.iter().map(|c| c.0).max().unwrap();
            candidates.retain(|c| c.0 == max_y);
            candidates.sort_by_key(|c| c.1); // Sort by X
            if cc == CodelChooser::Right {
                // Min X
                **candidates.first().unwrap()
            } else {
                // Max X
                **candidates.last().unwrap()
            }
        }
        Direction::Left => {
            // Min X
            let min_x = candidates.iter().map(|c| c.1).min().unwrap();
            candidates.retain(|c| c.1 == min_x);
            candidates.sort_by_key(|c| c.0); // Sort by Y
            if cc == CodelChooser::Right {
                // Min Y
                **candidates.first().unwrap()
            } else {
                // Max Y
                **candidates.last().unwrap()
            }
        }
        Direction::Up => {
            // Min Y
            let min_y = candidates.iter().map(|c| c.0).min().unwrap();
            candidates.retain(|c| c.0 == min_y);
            candidates.sort_by_key(|c| c.1); // Sort by X
            if cc == CodelChooser::Right {
                // Max X
                **candidates.last().unwrap()
            } else {
                // Min X
                **candidates.first().unwrap()
            }
        }
    }
}

fn next_coord(y: usize, x: usize, dp: Direction) -> Option<(usize, usize)> {
    let (dy, dx) = match dp {
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Up => (-1, 0),
    };
    let ny = y as i64 + dy;
    let nx = x as i64 + dx;
    if ny >= 0 && ny < GRID_SIZE as i64 && nx >= 0 && nx < GRID_SIZE as i64 {
        Some((ny as usize, nx as usize))
    } else {
        None
    }
}

fn execute_op(vm: &mut ChimeraVM, state: &mut PietState, dh: i32, dl: i32, block_size: i64) {
    match (dh, dl) {
        (0, 1) => {
            // Push
            state.stack.push(block_size);
        }
        (0, 2) => {
            // Pop
            state.stack.pop();
        }
        (1, 0) => {
            // Add
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(b.wrapping_add(a));
            }
        }
        (1, 1) => {
            // Sub
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(b.wrapping_sub(a));
            }
        }
        (1, 2) => {
            // Mul
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(b.wrapping_mul(a));
            }
        }
        (2, 0) => {
            // Div
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                if a != 0 {
                    state.stack.push(b.wrapping_div(a));
                } else {
                    // Ignore or push error? Piet spec says ignore or undefined.
                    // We'll restore stack to match "ignore" (push back in reverse)
                    state.stack.push(b);
                    state.stack.push(a);
                }
            }
        }
        (2, 1) => {
            // Mod
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                if a != 0 {
                    state.stack.push(b.wrapping_rem(a));
                } else {
                    state.stack.push(b);
                    state.stack.push(a);
                }
            }
        }
        (2, 2) => {
            // Not
            if let Some(val) = state.stack.pop() {
                state.stack.push(if val == 0 { 1 } else { 0 });
            }
        }
        (3, 0) => {
            // Greater
            if state.stack.len() >= 2 {
                let a = state.stack.pop().unwrap();
                let b = state.stack.pop().unwrap();
                state.stack.push(if b > a { 1 } else { 0 });
            }
        }
        (3, 1) => {
            // Pointer
            if let Some(val) = state.stack.pop() {
                // Rotate DP val times clockwise
                let rotations = val.rem_euclid(4) as usize;
                for _ in 0..rotations {
                    state.dp = match state.dp {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Right,
                    };
                }
            }
        }
        (3, 2) => {
            // Switch
            if let Some(val) = state.stack.pop() {
                // Toggle CC val times
                if val.abs() % 2 == 1 {
                    state.cc = match state.cc {
                        CodelChooser::Left => CodelChooser::Right,
                        CodelChooser::Right => CodelChooser::Left,
                    };
                }
            }
        }
        (4, 0) => {
            // Dup
            if let Some(&val) = state.stack.last() {
                state.stack.push(val);
            }
        }
        (4, 1) => {
            // Roll
            if state.stack.len() >= 2 {
                let rolls = state.stack.pop().unwrap();
                let depth = state.stack.pop().unwrap();
                if depth > 0 && (depth as usize) <= state.stack.len() {
                    let idx = state.stack.len() - (depth as usize);
                    // Just do a naive rotate
                    // If rolls is negative, reverse direction.
                    // Extract the slice
                    let mut slice: Vec<i64> = state.stack.drain(idx..).collect();
                    let len = slice.len();
                    let r = rolls.rem_euclid(len as i64) as usize;
                    slice.rotate_right(r);
                    state.stack.extend(slice);
                } else {
                    // Put back
                    state.stack.push(depth);
                    state.stack.push(rolls);
                }
            }
        }
        (4, 2) => {
            // In (Number)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(n) = val {
                    state.stack.push(n);
                } else if let Value::Str(s) = val {
                    // Try parse or just len
                    if let Ok(n) = s.parse::<i64>() {
                        state.stack.push(n);
                    } else {
                        state.stack.push(s.len() as i64);
                    }
                }
            }
        }
        (5, 0) => {
            // In (Char)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(n) = val {
                    state.stack.push(n);
                } else if let Value::Str(s) = val {
                    if let Some(c) = s.chars().next() {
                        state.stack.push(c as i64);
                    }
                }
            }
        }
        (5, 1) => {
            // Out (Number)
            if let Some(val) = state.stack.pop() {
                vm.stack.push(Value::Int(val));
            }
        }
        (5, 2) => {
            // Out (Char)
            if let Some(val) = state.stack.pop() {
                // Convert to char string
                let c = (val as u8) as char;
                vm.stack.push(Value::Str(c.to_string()));
            }
        }
        _ => {}
    }
}

pub fn exec_piet(vm: &mut ChimeraVM, max_steps: i64) {
    let grid = get_piet_grid(vm);

    // Start at top left (or current context loc?)
    // Piet spec starts at 0,0.
    // If 0,0 is Black, it terminates immediately?
    // Let's assume standard Piet.

    let mut state = PietState {
        stack: Vec::new(),
        dp: Direction::Right,
        cc: CodelChooser::Left,
        y: 0,
        x: 0,
        steps: 0,
    };

    if grid[0][0] == PietColor::Black {
        vm.output
            .push("PIET: Terminated (Start is Black)".to_string());
        return;
    }

    // Slide through white loop?
    // Piet Logic:
    // 1. Determine current block.
    // 2. Determine exit edge/codel.
    // 3. Attempt to move.

    while state.steps < max_steps as usize {
        state.steps += 1;

        let curr_color = grid[state.y][state.x];

        // Handle White Space handling (sliding)
        // If current is white, we just move straight until we hit non-white or black/edge.
        // But transition FROM color TO white is Op(No-op).
        // Transition FROM white TO color is no-op.

        // Simpler model: treat White as a special 1-cell block or implement slide logic.
        // Spec: "Slide through white codels in straight line".

        if curr_color == PietColor::White {
            // Slide
            let mut next = next_coord(state.y, state.x, state.dp);
            while let Some((ny, nx)) = next {
                if grid[ny][nx] == PietColor::White {
                    state.y = ny;
                    state.x = nx;
                    next = next_coord(state.y, state.x, state.dp);
                } else if grid[ny][nx] == PietColor::Black {
                    // Hit black/edge while sliding -> Rotate DP/CC toggles...
                    // "If it hits a restriction, the CC is toggled. If still restricted, DP rotated..."
                    // This is complex for white sliding.
                    // Simplified: White acts like a no-op path. If blocked, it stops?
                    // Actually, if we hit Black from White, we retrace?
                    // Let's implement full White logic if possible, else simplified.
                    // Simplified: Treat White cells as normal blocks but with no ops on transition?
                    // No, that breaks sliding.

                    // Let's stick to "Current cell is Color".
                    break;
                } else {
                    // Hit Color. Move there. No Op executed (White -> Color).
                    state.y = ny;
                    state.x = nx;
                    break; // Loop continues with this new color
                }
            }
            if next.is_none() {
                // Hit edge while sliding
                // "Behaves as if it hit a black block"
                state.cc = match state.cc {
                    CodelChooser::Left => CodelChooser::Right,
                    CodelChooser::Right => CodelChooser::Left,
                };
                state.dp = match state.dp {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
                // We are stuck on white?
                // We need to re-evaluate sliding from current white cell with new DP.
                continue;
            }
            // Check if we are still on White (stuck?)
            if grid[state.y][state.x] == PietColor::White {
                // We hit black/edge and didn't move off white.
                continue;
            }
        }

        // Current cell is Colored (not White).
        let (block, color) = get_block(&grid, state.y, state.x);
        let block_size = block.len() as i64;

        let mut attempts = 0;
        let mut moved = false;

        while attempts < 8 {
            let (ey, ex) = find_exit_edge(&block, state.dp, state.cc);
            let next = next_coord(ey, ex, state.dp);

            if let Some((ny, nx)) = next {
                let next_color = grid[ny][nx];
                if next_color == PietColor::Black {
                    // Blocked
                } else {
                    // Move
                    // Calculate Delta
                    if next_color != PietColor::White {
                        // Color -> Color: Execute Op
                        if let (PietColor::Color(h1, l1), PietColor::Color(h2, l2)) =
                            (color, next_color)
                        {
                            let dh = ((h2 as i32) - (h1 as i32)).rem_euclid(6);
                            let dl = ((l2 as i32) - (l1 as i32)).rem_euclid(3);
                            execute_op(vm, &mut state, dh, dl, block_size);
                        }
                    }
                    // If next is White, we just move (no Op).

                    state.y = ny;
                    state.x = nx;
                    moved = true;
                    break;
                }
            }

            // Toggle CC / Rotate DP
            if attempts % 2 == 0 {
                state.cc = match state.cc {
                    CodelChooser::Left => CodelChooser::Right,
                    CodelChooser::Right => CodelChooser::Left,
                };
            } else {
                state.dp = match state.dp {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
            attempts += 1;
        }

        if !moved {
            // Trapped
            vm.output.push("PIET: Terminated (Trapped)".to_string());
            break;
        }
    }

    vm.energy = vm.energy.saturating_sub((state.steps / 10) as i64);
    vm.output
        .push(format!("PIET: Executed {} steps", state.steps));
}
