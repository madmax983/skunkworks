use super::{ChimeraVM, Value};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct VoidRift {
    pub location: (usize, usize),
    pub severity: usize,
    pub age: usize,
    pub consumed: usize,
}

impl VoidRift {
    pub fn new(location: (usize, usize)) -> Self {
        Self {
            location,
            severity: 1,
            age: 0,
            consumed: 0,
        }
    }
}

pub fn process_rifts(vm: &mut ChimeraVM) {
    let mut rifts = std::mem::take(&mut vm.void_rifts);
    let mut new_rifts = Vec::new();
    let _grid_size = crate::vm::GRID_SIZE;

    for rift in &mut rifts {
        rift.age += 1;

        // Growth rate depends on severity
        // Higher severity = faster consumption
        let tick_threshold = (100 / rift.severity).max(1);

        if rift.age % tick_threshold == 0 {
            let (ry, rx) = rift.location;
            let mut rng = rand::thread_rng();

            // Try to consume a neighbor
            let dy = rng.gen_range(-1..=1);
            let dx = rng.gen_range(-1..=1);

            if let Some((ny, nx)) = vm.normalize_coords(ry as i64 + dy, rx as i64 + dx) {
                // Check if cell has value
                if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                    vm.grid[ny][nx] = Value::Int(0);
                    rift.consumed += 1;
                    rift.severity = rift.severity.saturating_add(1);
                    vm.output
                        .push(format!("VOID RIFT: Consumed at {},{}", nx, ny));
                }
            }
        }

        // Critical Mass: If severity > 50, small chance to spawn a child rift
        if rift.severity > 50 {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.01) {
                let (ry, rx) = rift.location;
                // Teleport child nearby
                let dy = rng.gen_range(-5..=5);
                let dx = rng.gen_range(-5..=5);
                if let Some((ny, nx)) = vm.normalize_coords(ry as i64 + dy, rx as i64 + dx) {
                    new_rifts.push(VoidRift::new((ny, nx)));
                    vm.output
                        .push(format!("VOID RIFT: Sprouted child at {},{}", nx, ny));
                    // Reduce parent severity
                    rift.severity /= 2;
                }
            }
        }
    }

    vm.void_rifts = rifts;
    vm.void_rifts.extend(new_rifts);
}

pub fn exec_void_rift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., y, x ] -> [ ... ]
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.void_rifts.push(VoidRift::new((ny, nx)));
                vm.energy = vm.energy.saturating_sub(50);
                vm.output
                    .push(format!("VOID RIFT: Opened at {},{}", nx, ny));
            } else {
                vm.output
                    .push("Error: Invalid coordinates for void_rift".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for void_rift".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for void_rift".to_string());
    }
    None
}

pub fn exec_void_cast(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ... ] -> [ ..., power ]
    // Finds nearest rift and draws power

    let (cy, cx) = vm.context_loc;
    let mut nearest_dist = f64::MAX;
    let mut power = 0;

    for rift in &vm.void_rifts {
        let (ry, rx) = rift.location;
        let dx = (cx as i64 - rx as i64).pow(2);
        let dy = (cy as i64 - ry as i64).pow(2);
        let dist = ((dx + dy) as f64).sqrt();

        if dist < nearest_dist {
            nearest_dist = dist;
            // Power formula: Severity / (Distance + 1)
            power = (rift.severity as f64 / (dist + 1.0)) as i64;
        }
    }

    if power > 0 {
        vm.stack.push(Value::Int(power));
        vm.output
            .push(format!("VOID CAST: Drew {} power from rift", power));

        // Risk: Backlash
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1 * (power as f64 / 10.0)) {
            vm.energy = vm.energy.saturating_sub(power);
            vm.output.push("VOID BACKLASH: Energy drained!".to_string());
        }
    } else {
        vm.stack.push(Value::Int(0));
        vm.output.push("VOID CAST: No rifts nearby".to_string());
    }

    None
}
