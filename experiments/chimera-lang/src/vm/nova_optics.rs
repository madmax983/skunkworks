#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn exec_reflector(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., orientation, y, x ]
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let ori_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(ori)) = (x_val, y_val, ori_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let orientation = ori.rem_euclid(4); // 0=|, 1=-, 2=/, 3=\
                vm.grid[ny][nx] = Value::Str(format!("REFLECTOR:{}", orientation));
                vm.energy = vm.energy.saturating_sub(10);
                vm.output.push(format!(
                    "REFLECTOR: Placed type {} at {},{}",
                    orientation, nx, ny
                ));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for reflector".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for reflector".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for reflector".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_prism(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., orientation, y, x ]
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let ori_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(ori)) = (x_val, y_val, ori_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let orientation = ori.rem_euclid(4);
                vm.grid[ny][nx] = Value::Str(format!("PRISM:{}", orientation));
                vm.energy = vm.energy.saturating_sub(15);
                vm.output
                    .push(format!("PRISM: Placed type {} at {},{}", orientation, nx, ny));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for prism".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for prism".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for prism".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_lens(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., power, y, x ]
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let pow_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(pow)) = (x_val, y_val, pow_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                let power = pow.clamp(1, 10);
                vm.grid[ny][nx] = Value::Str(format!("LENS:{}", power));
                vm.energy = vm.energy.saturating_sub(20);
                vm.output.push(format!(
                    "LENS: Placed power {} at {},{}",
                    power, nx, ny
                ));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for lens".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for lens".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for lens".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_optics_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Reflector => exec_reflector(vm),
        OpCode::Prism => exec_prism(vm),
        OpCode::Lens => exec_lens(vm),
        _ => None,
    }
}
