#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::JunctionType;

/// Compresses a square grid area into a value on the stack.
///
/// **OpCode:** `Pocket`
/// **Stack:** `[ ..., radius ] -> [ ..., pocket_val ]`
#[cfg(feature = "nova")]
pub fn exec_pocket(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r >= 0 {
                let radius = r;
                let (cy, cx) = vm.context_loc;
                let mut data = Vec::new();

                // Header: Marker, Radius
                let mut content = vec![Value::Str("POCKET".to_string()), Value::Int(radius)];

                // Capture Square Area (2r+1 x 2r+1)
                // Order: row-major from top-left (-r, -r) to bottom-right (r, r)
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                        {
                            data.push(vm.grid[ny][nx].clone());
                            // Clear the cell
                            vm.grid[ny][nx] = Value::Int(0);
                        } else {
                            // Out of bounds (e.g. Plane topology edge)
                            // We push Int(0) to preserve the square shape in the data
                            data.push(Value::Int(0));
                        }
                    }
                }

                content.extend(data);

                // Cost: Area size
                let area = (2 * radius + 1).pow(2);
                vm.energy = vm.energy.saturating_sub(area);

                vm.stack.push(Value::Junction(JunctionType::All, content));
                vm.output.push(format!(
                    "POCKET: Captured radius {} at {},{}",
                    radius, cx, cy
                ));
            } else {
                vm.output
                    .push("POCKET: Radius must be non-negative".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for pocket".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for pocket".to_string());
    }
    None
}

/// Decompresses a pocket value onto the grid.
///
/// **OpCode:** `Unpocket`
/// **Stack:** `[ ..., pocket_val ] -> [ ... ]`
#[cfg(feature = "nova")]
pub fn exec_unpocket(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Junction(JunctionType::All, items) = val {
            // Validate Header
            if items.len() >= 2 {
                if let (Value::Str(marker), Value::Int(r)) = (&items[0], &items[1]) {
                    if marker == "POCKET" {
                        let radius = *r;
                        let expected_size = (2 * radius + 1).pow(2) as usize;

                        if items.len() == 2 + expected_size {
                            let (cy, cx) = vm.context_loc;
                            let data = &items[2..];
                            let mut idx = 0;

                            for dy in -radius..=radius {
                                for dx in -radius..=radius {
                                    let cell_val = &data[idx];
                                    idx += 1;

                                    if let Some((ny, nx)) =
                                        vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                                    {
                                        // Only overwrite if we have data (0 is data in this context)
                                        // But wait, if we captured "void" (0), we should write 0.
                                        // Yes, overwrite everything.
                                        vm.grid[ny][nx] = cell_val.clone();
                                    }
                                }
                            }

                            // Cost: Area size / 2 (cheaper to release?)
                            let area = (2 * radius + 1).pow(2);
                            vm.energy = vm.energy.saturating_sub(area / 2);
                            vm.output.push(format!(
                                "UNPOCKET: Released radius {} at {},{}",
                                radius, cx, cy
                            ));
                        } else {
                            vm.output
                                .push("UNPOCKET: Corrupted pocket data size".to_string());
                        }
                    } else {
                        vm.output
                            .push("UNPOCKET: Invalid junction marker".to_string());
                    }
                } else {
                    vm.output
                        .push("UNPOCKET: Invalid pocket header format".to_string());
                }
            } else {
                vm.output
                    .push("UNPOCKET: Invalid pocket structure".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for unpocket (expected Junction)".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for unpocket".to_string());
    }
    None
}
