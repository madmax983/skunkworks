#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Wavelength {
    Red,   // 0
    Green, // 1
    Blue,  // 2
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq)]
pub struct Photon {
    pub x: usize,
    pub y: usize,
    pub dx: i8,
    pub dy: i8,
    pub wavelength: Wavelength,
    pub intensity: i64,
}

#[cfg(feature = "nova")]
pub fn process_optics(vm: &mut ChimeraVM) {
    let active_photons = std::mem::take(&mut vm.photons);
    let mut next_photons = Vec::new();

    for mut p in active_photons {
        // Move
        if let Some((ny, nx)) = vm.normalize_coords(p.y as i64 + p.dy as i64, p.x as i64 + p.dx as i64) {
            p.y = ny;
            p.x = nx;

            // Decay
            p.intensity -= 1;
            if p.intensity <= 0 {
                continue;
            }

            // Interact with Grid
            let cell = &vm.grid[p.y][p.x];
            let mut absorbed = false;

            match cell {
                Value::Str(s) => {
                    if s.starts_with("REFLECTOR:") {
                        let ori: i64 = s.split(':').nth(1).unwrap_or("0").parse().unwrap_or(0);
                        // 0=|, 1=-, 2=/, 3=\
                        match ori {
                            0 => p.dx = -p.dx, // Vertical Mirror: Reflect X
                            1 => p.dy = -p.dy, // Horizontal Mirror: Reflect Y
                            2 => { // / Mirror: (1,0)->(0,-1), (-1,0)->(0,1), (0,1)->(-1,0), (0,-1)->(1,0)
                                let temp = p.dx;
                                p.dx = -p.dy;
                                p.dy = -temp;
                            }
                            3 => { // \ Mirror: (1,0)->(0,1), (-1,0)->(0,-1), (0,1)->(1,0), (0,-1)->(-1,0)
                                let temp = p.dx;
                                p.dx = p.dy;
                                p.dy = temp;
                            }
                            _ => {}
                        }
                    } else if s.starts_with("PRISM:") {
                        // Split? For now just random deflect
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        if rng.gen_bool(0.5) {
                            let temp = p.dx;
                            p.dx = p.dy;
                            p.dy = temp;
                        } else {
                            let temp = p.dx;
                            p.dx = -p.dy;
                            p.dy = -temp;
                        }
                    } else if s.starts_with("LENS:") {
                        let power: i64 = s.split(':').nth(1).unwrap_or("1").parse().unwrap_or(1);
                        p.intensity += power;
                    } else if s == "#" { // Wall
                        absorbed = true;
                    }
                }
                _ => {}
            }

            // Deposit Light
            vm.light_grid[p.y][p.x] = vm.light_grid[p.y][p.x].saturating_add(p.intensity);

            if !absorbed {
                next_photons.push(p);
            }
        }
    }
    vm.photons = next_photons;
}

#[cfg(feature = "nova")]
pub fn exec_laser(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., wavelength, intensity, dy, dx ]
    if vm.stack.len() >= 4 {
        let dx_val = vm.stack.pop().unwrap();
        let dy_val = vm.stack.pop().unwrap();
        let int_val = vm.stack.pop().unwrap();
        let wave_val = vm.stack.pop().unwrap();

        if let (Value::Int(dx), Value::Int(dy), Value::Int(i), Value::Int(w)) = (dx_val, dy_val, int_val, wave_val) {
            let wavelength = match w {
                0 => Wavelength::Red,
                1 => Wavelength::Green,
                _ => Wavelength::Blue,
            };

            let (cy, cx) = vm.context_loc;

            let photon = Photon {
                x: cx,
                y: cy,
                dx: dx.clamp(-1, 1) as i8,
                dy: dy.clamp(-1, 1) as i8,
                wavelength,
                intensity: i.clamp(1, 100),
            };

            vm.photons.push(photon);
            vm.energy = vm.energy.saturating_sub(5);
            vm.output.push(format!("LASER: Fired {:?} vec({},{})", wavelength, dx, dy));
        } else {
             vm.output.push("Error: Type mismatch for laser".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for laser".to_string());
    }
    None
}

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
                vm.output.push(format!(
                    "PRISM: Placed type {} at {},{}",
                    orientation, nx, ny
                ));
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
                vm.output
                    .push(format!("LENS: Placed power {} at {},{}", power, nx, ny));
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
        OpCode::Laser => exec_laser(vm),
        _ => None,
    }
}
