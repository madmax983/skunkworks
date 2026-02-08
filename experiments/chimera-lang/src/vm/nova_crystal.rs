use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;

pub fn exec_crystal_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Nucleate => exec_nucleate(vm),
        OpCode::Accrete => exec_accrete(vm),
        OpCode::Shatter => exec_shatter(vm),
        OpCode::Anneal => exec_anneal(vm),
        _ => None,
    }
}

fn exec_nucleate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.grid[cy][cx] = Value::Int(100);
    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("NUCLEATE: Crystal seed at {},{}", cx, cy));
    None
}

fn exec_accrete(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(r)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
        let mut absorbed_sum = 0;

        for (x, y) in coords {
            // Don't absorb self
            if x == cx && y == cy {
                continue;
            }

            if let Value::Int(n) = &mut vm.grid[y][x] {
                if *n > 0 {
                    absorbed_sum += *n;
                    *n = 0;
                }
            }
        }

        if let Value::Int(current) = &mut vm.grid[cy][cx] {
            *current = current.saturating_add(absorbed_sum);
        } else {
            vm.grid[cy][cx] = Value::Int(absorbed_sum);
        }

        vm.energy = vm.energy.saturating_sub(r.abs() + 5);
        vm.output.push(format!(
            "ACCRETE: Absorbed {} from radius {}",
            absorbed_sum, r
        ));
    } else {
        vm.output
            .push("Error: Type mismatch for accrete".to_string());
    }
    None
}

fn exec_shatter(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(force)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let mut center_val = 0;

        if let Value::Int(n) = vm.grid[cy][cx] {
            center_val = n;
        }

        if center_val > 0 {
            vm.grid[cy][cx] = Value::Int(0);
            let f = force.clamp(1, 10);
            let fragments = center_val / f; // Simple distribution

            let mut rng = rand::thread_rng();
            for _ in 0..f {
                let dy = rng.gen_range(-1..=1);
                let dx = rng.gen_range(-1..=1);
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    if let Value::Int(n) = &mut vm.grid[ny][nx] {
                        *n = n.saturating_add(fragments);
                    } else {
                        vm.grid[ny][nx] = Value::Int(fragments);
                    }
                }
            }
            vm.output.push(format!(
                "SHATTER: Scattered {} with force {}",
                center_val, f
            ));
        }

        vm.energy = vm.energy.saturating_sub(force * 2);
    } else {
        vm.output
            .push("Error: Type mismatch for shatter".to_string());
    }
    None
}

fn exec_anneal(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(r)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

        // 1. Collect values
        let mut values = Vec::new();
        for &(x, y) in &coords {
            values.push(vm.grid[y][x].clone());
        }

        // 2. Sort values
        values.sort_by(|a, b| match (a, b) {
            (Value::Int(ia), Value::Int(ib)) => ia.cmp(ib),
            (Value::Str(sa), Value::Str(sb)) => sa.cmp(sb),
            (Value::Int(_), Value::Str(_)) => std::cmp::Ordering::Less,
            (Value::Str(_), Value::Int(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        });

        // 3. Write back
        for (i, &(x, y)) in coords.iter().enumerate() {
            if i < values.len() {
                vm.grid[y][x] = values[i].clone();
            }
        }

        vm.energy = vm.energy.saturating_sub(r.abs() * 2);
        vm.output.push(format!(
            "ANNEAL: Sorted {} cells radius {}",
            values.len(),
            r
        ));
    } else {
        vm.output
            .push("Error: Type mismatch for anneal".to_string());
    }
    None
}
