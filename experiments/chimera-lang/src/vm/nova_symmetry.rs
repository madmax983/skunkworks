#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

#[cfg(feature = "nova")]
pub fn exec_reflect(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(axis) = val {
            let mut new_grid = vm.grid.clone();
            let size = GRID_SIZE;

            match axis {
                0 => {
                    // Vertical (Left -> Right)
                    for y in 0..size {
                        for x in 0..size / 2 {
                            new_grid[y][size - 1 - x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("REFLECT: Vertical axis".to_string());
                }
                1 => {
                    // Horizontal (Top -> Bottom)
                    for y in 0..size / 2 {
                        for x in 0..size {
                            new_grid[size - 1 - y][x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("REFLECT: Horizontal axis".to_string());
                }
                2 => {
                    // Diagonal (Top-Left -> Bottom-Right) (\)
                    // Copy Upper-Right to Lower-Left
                    for y in 0..size {
                        for x in y + 1..size {
                            new_grid[x][y] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("REFLECT: Diagonal (\\) axis".to_string());
                }
                3 => {
                    // Diagonal (Top-Right -> Bottom-Left) (/)
                    // Copy Top-Left to Bottom-Right
                    for y in 0..size {
                        for x in 0..size - 1 - y {
                            new_grid[size - 1 - x][size - 1 - y] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("REFLECT: Diagonal (/) axis".to_string());
                }
                _ => {
                    vm.output.push(format!("Error: Invalid axis {}", axis));
                    return None;
                }
            }
            vm.grid = new_grid;
            vm.energy = vm.energy.saturating_sub(20);
        } else {
            vm.output
                .push("Error: Type mismatch for reflect".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for reflect".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_rotate(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(turns) = val {
            let t = turns.rem_euclid(4); // 0, 1, 2, 3
            if t == 0 {
                return None;
            }

            let mut new_grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];
            let size = GRID_SIZE;

            for y in 0..size {
                for x in 0..size {
                    let (ny, nx) = match t {
                        1 => (x, size - 1 - y),           // 90 CW
                        2 => (size - 1 - y, size - 1 - x), // 180
                        3 => (size - 1 - x, y),           // 270 CW (90 CCW)
                        _ => (y, x),
                    };
                    new_grid[ny][nx] = vm.grid[y][x].clone();
                }
            }

            vm.grid = new_grid;
            vm.energy = vm.energy.saturating_sub(15 * t);
            vm.output.push(format!("ROTATE: {} degrees", t * 90));
        } else {
            vm.output
                .push("Error: Type mismatch for rotate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rotate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_symmetrize(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(mode) = val {
            let mut new_grid = vm.grid.clone();
            let size = GRID_SIZE;

            match mode {
                0 => {
                    // Mirror X (Left Half -> Right Half)
                    for y in 0..size {
                        for x in 0..size / 2 {
                            new_grid[y][size - 1 - x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("SYMMETRIZE: Mirror X".to_string());
                }
                1 => {
                    // Mirror Y (Top Half -> Bottom Half)
                    for y in 0..size / 2 {
                        for x in 0..size {
                            new_grid[size - 1 - y][x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("SYMMETRIZE: Mirror Y".to_string());
                }
                2 => {
                    // Quad (Top-Left Quadrant -> All 4)
                    let half = size / 2;
                    // 1. Copy TL to TR (Mirror X of top half)
                    for y in 0..half {
                        for x in 0..half {
                            new_grid[y][size - 1 - x] = new_grid[y][x].clone();
                        }
                    }
                    // 2. Copy Top Half to Bottom Half (Mirror Y)
                    for y in 0..half {
                        for x in 0..size {
                            new_grid[size - 1 - y][x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("SYMMETRIZE: Quad".to_string());
                }
                3 => {
                    // Octal (Top-Left Triangle of TL Quadrant -> All 8)
                    // 1. Enforce diagonal symmetry in TL Quadrant
                    // Copy Upper-Right (x > y) to Lower-Left (y > x)
                    // This means the "source" is the upper-right triangle of the TL quadrant.
                    for y in 0..size / 2 {
                        for x in y + 1..size / 2 {
                            new_grid[x][y] = new_grid[y][x].clone();
                        }
                    }
                    // Now TL is diagonally symmetric.
                    // 2. Apply Quad symmetry
                    let half = size / 2;
                    for y in 0..half {
                        for x in 0..half {
                            new_grid[y][size - 1 - x] = new_grid[y][x].clone();
                        }
                    }
                    for y in 0..half {
                        for x in 0..size {
                            new_grid[size - 1 - y][x] = new_grid[y][x].clone();
                        }
                    }
                    vm.output.push("SYMMETRIZE: Octal".to_string());
                }
                _ => {
                    vm.output.push(format!("Error: Invalid mode {}", mode));
                    return None;
                }
            }
            vm.grid = new_grid;
            vm.energy = vm.energy.saturating_sub(50);
        } else {
            vm.output
                .push("Error: Type mismatch for symmetrize".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for symmetrize".to_string());
    }
    None
}
