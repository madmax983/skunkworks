#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_claim(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(radius)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let owner = vm.ip.0;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, radius);

        let mut success = 0;
        for (x, y) in coords {
            // Can only claim if empty or already owned by self
            if vm.sovereignty_grid[y][x].is_none() || vm.sovereignty_grid[y][x] == Some(owner) {
                vm.sovereignty_grid[y][x] = Some(owner);
                success += 1;
            }
        }

        // Cost: 10 per cell claimed
        vm.energy = vm.energy.saturating_sub(success as i64 * 10);
        vm.output.push(format!(
            "CLAIM: Claimed {} cells for strand {}",
            success, owner
        ));
    } else {
        vm.output.push("Error: Type mismatch for claim".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_cede(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if vm.sovereignty_grid[ny][nx] == Some(vm.ip.0) {
                    vm.sovereignty_grid[ny][nx] = None;
                    vm.output.push(format!("CEDE: Released {},{}", nx, ny));
                } else {
                    vm.output.push("CEDE: Not your territory".to_string());
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for cede".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cede".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_sovereignty(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let owner = vm.sovereignty_grid[ny][nx]
                    .map(|id| id as i64)
                    .unwrap_or(-1);
                vm.stack.push(Value::Int(owner));
            } else {
                vm.stack.push(Value::Int(-1));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for sovereignty".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for sovereignty".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_tax(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(rate)) = vm.stack.pop() {
        if rate >= 0 {
            vm.tax_rates.insert(vm.ip.0, rate);
            vm.output
                .push(format!("TAX: Set rate to {} for strand {}", rate, vm.ip.0));
        } else {
            vm.output.push("Error: Negative tax rate".to_string());
        }
    } else {
        vm.output.push("Error: Type mismatch for tax".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn process_territory(vm: &mut ChimeraVM) {
    let (cy, cx) = vm.context_loc;
    let visitor = vm.ip.0;

    if let Some(owner) = vm.sovereignty_grid[cy][cx] {
        if owner != visitor {
            let tax_rate = *vm.tax_rates.get(&owner).unwrap_or(&0);
            if tax_rate > 0 {
                // Check if visitor has enough energy
                let payment = vm.energy.min(tax_rate);
                vm.energy -= payment;

                // Credit owner (via Market wallet)
                vm.market.credit(owner, payment);

                if payment > 0 {
                    vm.output.push(format!(
                        "TAX: Strand {} paid {} to {}",
                        visitor, payment, owner
                    ));
                }

                if vm.energy <= 0 {
                    vm.output
                        .push(format!("DEATH: Taxed to death by {}", owner));
                    vm.halted = true;
                }
            }
        }
    }
}
