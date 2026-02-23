#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;

pub fn exec_cartography_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Scan => exec_scan(vm),
        OpCode::Locate => exec_locate(vm),
        OpCode::Chart => exec_chart(vm),
        OpCode::Atlas => exec_atlas(vm),
        _ => None,
    }
}

fn exec_scan(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(radius)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let mut values = Vec::new();

        let r = radius.clamp(0, 10); // Limit radius to avoid excessive cost
        crate::vm::iterate_circle(vm.topology, cx as i64, cy as i64, r, |x, y| {
            if !matches!(vm.grid[y][x], Value::Int(0)) {
                values.push(vm.grid[y][x].clone());
            }
        });

        // Cost based on area
        vm.energy = vm.energy.saturating_sub(r * r);
        vm.stack.push(Value::Junction(JunctionType::All, values));
        vm.output
            .push(format!("SCAN: Radius {} at {},{}", r, cx, cy));
    } else {
        vm.output.push("Error: Type mismatch for scan".to_string());
    }
    None
}

fn exec_locate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.stack.push(Value::Int(cy as i64));
    vm.stack.push(Value::Int(cx as i64));
    None
}

fn exec_chart(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: value, y, x (top)
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            let size = GRID_SIZE as i64;
            if x >= 0 && x < size && y >= 0 && y < size {
                vm.cartography_grid[y as usize][x as usize] = val;
                vm.energy = vm.energy.saturating_sub(2);
                vm.output.push(format!("CHART: Marked {},{}", x, y));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for chart".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for chart".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for chart".to_string());
    }
    None
}

fn exec_atlas(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: y, x (top)
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            let size = GRID_SIZE as i64;
            if x >= 0 && x < size && y >= 0 && y < size {
                let val = vm.cartography_grid[y as usize][x as usize].clone();
                vm.stack.push(val);
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for atlas".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for atlas".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for atlas".to_string());
    }
    None
}
