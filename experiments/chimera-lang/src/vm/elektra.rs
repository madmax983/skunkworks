use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::VisualEffect;

const CONDUCTIVITY_WIRE: f32 = 10.0;
const CONDUCTIVITY_DEFAULT: f32 = 0.01;
const CONDUCTIVITY_NONE: f32 = 0.0;

fn get_conductivity(val: &Value) -> f32 {
    match val {
        Value::Int(0) => CONDUCTIVITY_NONE,
        Value::Int(1) | Value::Int(2) | Value::Int(3) => CONDUCTIVITY_WIRE,
        Value::Str(s) => {
            if s.starts_with("D:") || s.starts_with("T:") || s.starts_with("M:") || s.starts_with("S:") {
                CONDUCTIVITY_WIRE
            } else {
                CONDUCTIVITY_DEFAULT
            }
        }
        Value::Int(_) => CONDUCTIVITY_DEFAULT,
        _ => CONDUCTIVITY_NONE,
    }
}

pub fn exec_elektra_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Electrogenesis => exec_electrogenesis(vm),
        OpCode::Induction => exec_induction(vm),
        OpCode::WireGrowth => exec_wire_growth(vm),
        OpCode::CircuitBreaker => exec_circuit_breaker(vm),
        OpCode::Battery => exec_battery(vm),
        OpCode::Ground => exec_ground(vm),
        OpCode::SenseVolt => exec_sense_volt(vm),
        OpCode::Shock => exec_shock(vm),
        OpCode::Lightning => exec_lightning(vm),
        OpCode::TeslaCoil => exec_tesla_coil(vm),
        #[cfg(all(feature = "elektra", feature = "nova"))]
        OpCode::Galvanize => exec_galvanize(vm),
        OpCode::Diode => exec_component_placement(vm, "D"),
        OpCode::Transistor => exec_component_placement(vm, "T"),
        OpCode::Muscle => exec_component_placement(vm, "M"),
        OpCode::Sensor => exec_component_placement(vm, "S"),
        _ => None,
    }
}

fn exec_electrogenesis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.stack.pop()?;
    if let Value::Int(amount) = val {
        let cost = amount.abs().max(1);
        if vm.energy >= cost {
            vm.energy -= cost;
            let (y, x) = vm.context_loc;
            vm.voltage_grid[y][x] += amount as f32;
            vm.output.push(format!("ELECTROGENESIS: +{}V at {},{}", amount, x, y));
        } else {
            vm.output.push("ELECTROGENESIS: Not enough energy".to_string());
        }
    }
    None
}

fn exec_induction(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (y, x) = vm.context_loc;
    let v = vm.voltage_grid[y][x];
    let energy_gain = v.abs() as i64;
    if energy_gain > 0 {
        vm.energy = vm.energy.saturating_add(energy_gain);
        vm.voltage_grid[y][x] = 0.0;
        vm.stack.push(Value::Int(energy_gain));
        vm.output.push(format!("INDUCTION: Absorbed {}V -> {} Energy", v, energy_gain));
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

fn exec_wire_growth(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.stack.pop()?;
    if let Value::Int(dir) = val {
        if vm.energy < 5 {
            vm.output.push("WIREGROWTH: Not enough energy".to_string());
            return None;
        }
        let (cy, cx) = vm.context_loc;
        let (dy, dx) = match dir % 4 {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => (0, 0),
        };

        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            vm.grid[ny][nx] = Value::Int(1);
            vm.energy -= 5;
            vm.output.push(format!("WIREGROWTH: Wire at {},{}", nx, ny));
        } else {
            vm.output.push("WIREGROWTH: Out of bounds".to_string());
        }
    }
    None
}

fn exec_circuit_breaker(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let s_val = vm.stack.pop().unwrap();
    let t_val = vm.stack.pop().unwrap();

    if let (Value::Int(strand_idx), Value::Int(threshold)) = (s_val, t_val) {
        let (y, x) = vm.context_loc;
        let v = vm.voltage_grid[y][x];
        if v > threshold as f32 && strand_idx >= 0 {
            vm.output.push(format!("CIRCUITBREAKER: Tripped at {}V > {}", v, threshold));
            return Some((strand_idx as usize, 0));
        }
    }
    None
}

fn exec_battery(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 3 { return None; }
    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    let v_val = vm.stack.pop().unwrap();
    if let (Value::Int(x), Value::Int(y), Value::Int(v)) = (x_val, y_val, v_val) {
        if let Some((ny, nx)) = vm.normalize_coords(y, x) {
            vm.voltage_grid[ny][nx] = v as f32;
            vm.resistance_grid[ny][nx] = -1.0;
            vm.output.push(format!("BATTERY: {}V at {},{}", v, x, y));
        }
    }
    None
}

fn exec_ground(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
        if let Some((ny, nx)) = vm.normalize_coords(y, x) {
            vm.voltage_grid[ny][nx] = 0.0;
            vm.resistance_grid[ny][nx] = -2.0;
            vm.output.push(format!("GROUND: 0V at {},{}", x, y));
        }
    }
    None
}

fn exec_sense_volt(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
        if let Some((ny, nx)) = vm.normalize_coords(y, x) {
            let v = vm.voltage_grid[ny][nx];
            vm.stack.push(Value::Int(v as i64));
        } else {
            vm.stack.push(Value::Int(0));
        }
    }
    None
}

fn exec_shock(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let r_val = vm.stack.pop().unwrap();
    let p_val = vm.stack.pop().unwrap();
    if let (Value::Int(r), Value::Int(p)) = (r_val, p_val) {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
        for (x, y) in coords {
            if vm.resistance_grid[y][x] < 0.0 {
                vm.resistance_grid[y][x] = 1.0;
                vm.output.push(format!("SHOCK: Blown fuse at {},{}", x, y));
            }
            vm.visual_effects.push(VisualEffect::Spark {
                loc: (y, x),
                color: (255, 100, 100),
                ttl: 2,
            });
        }
        vm.output.push(format!("SHOCK: Discharged {} power", p));
    }
    None
}

fn exec_lightning(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
        vm.output.push(format!("LIGHTNING: Strike at {},{}", x, y));
    }
    None
}

fn exec_tesla_coil(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() < 2 { return None; }
    let r_val = vm.stack.pop().unwrap();
    let p_val = vm.stack.pop().unwrap();
    if let (Value::Int(r), Value::Int(p)) = (r_val, p_val) {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

        vm.output.push(format!(
            "TESLA COIL: Discharging {} power radius {} at {},{}",
            p, r, cx, cy
        ));

        for (x, y) in &coords {
            vm.visual_effects.push(VisualEffect::Lightning {
                from: (cy, cx),
                to: (*y, *x),
                color: (200, 200, 255),
                ttl: 3,
            });
        }

        if vm.voltage_grid[cy][cx] >= p as f32 {
            vm.voltage_grid[cy][cx] -= p as f32;

            #[cfg(feature = "nova")]
            {
                let mut hit_count = 0;
                for (x, y) in coords {
                    for org in vm.organelles.iter_mut() {
                        if org.context_loc == (y, x) {
                            org.halted = true;
                            hit_count += 1;
                        }
                    }
                }
                if hit_count > 0 {
                    vm.output.push(format!("TESLA COIL: Fried {} organelles", hit_count));
                }
            }
        } else {
            vm.output.push("TESLA COIL: Insufficient voltage".to_string());
        }
    }
    None
}

#[cfg(all(feature = "elektra", feature = "nova"))]
fn exec_galvanize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let val = vm.stack.pop()?;
    if let Value::Int(idx) = val {
        let idx = idx as usize;
        let (y, x) = vm.context_loc;
        let voltage = vm.voltage_grid[y][x];

        if voltage > 100.0 {
            if idx < vm.graveyard.len() {
                let mut strand = vm.graveyard[idx].clone();
                vm.graveyard.remove(idx);

                if !strand.genes.is_empty() {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let g_idx = rng.gen_range(0..strand.genes.len());
                    if !strand.genes[g_idx].args.is_empty() {
                        strand.genes[g_idx].args[0] =
                            Nucleotide::Number(rng.gen_range(0..100));
                    }
                }

                vm.dna.helix.strands.push(strand);
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }

                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.stack.push(Value::Int(new_idx as i64));

                vm.voltage_grid[y][x] = 0.0;

                vm.visual_effects.push(VisualEffect::Spark {
                    loc: (y, x),
                    color: (100, 255, 100),
                    ttl: 5,
                });

                vm.output.push(format!(
                    "GALVANIZE: IT'S ALIVE! Strand {} resurrected as {}",
                    idx, new_idx
                ));
            } else {
                vm.output
                    .push("GALVANIZE: Invalid graveyard index".to_string());
                vm.stack.push(Value::Int(-1));
            }
        } else {
            vm.output.push(format!(
                "GALVANIZE: Insufficient voltage ({:.1}V < 100.0V)",
                voltage
            ));
            vm.stack.push(Value::Int(-1));
        }
    }
    None
}

fn exec_component_placement(vm: &mut ChimeraVM, type_prefix: &str) -> Option<(usize, usize)> {
    if vm.stack.len() < 3 { return None; }
    let x_val = vm.stack.pop().unwrap();
    let y_val = vm.stack.pop().unwrap();
    let param_val = vm.stack.pop().unwrap();

    if let (Value::Int(x), Value::Int(y), Value::Int(p)) = (x_val, y_val, param_val) {
        if let Some((ny, nx)) = vm.normalize_coords(y, x) {
            let p_mod = if type_prefix == "D" || type_prefix == "T" { p % 4 } else { p };
            vm.grid[ny][nx] = Value::Str(format!("{}:{}", type_prefix, p_mod));

            let name = match type_prefix {
                "D" => "DIODE",
                "T" => "TRANSISTOR",
                "M" => "MUSCLE",
                "S" => "SENSOR",
                _ => "COMPONENT",
            };

            vm.output.push(format!("{}: Created at {},{} param {}", name, nx, ny, p_mod));
        }
    }
    None
}

// Helper to calculate effective conductivity multiplier based on component logic
fn get_component_multiplier(vm: &ChimeraVM, y: usize, x: usize, neighbor_y: usize, neighbor_x: usize) -> f32 {
    let mut mult = 1.0;

    // Check SELF component logic
    if let Value::Str(s) = &vm.grid[y][x] {
        if let Some(stripped) = s.strip_prefix("D:") {
            let dir = stripped.parse::<i64>().unwrap_or(0);
            mult *= check_diode(dir, y, x, neighbor_y, neighbor_x, vm);
        } else if let Some(stripped) = s.strip_prefix("T:") {
            let dir = stripped.parse::<i64>().unwrap_or(0);
            mult *= check_transistor(dir, y, x, neighbor_y, neighbor_x, vm);
        }
    }

    // Check NEIGHBOR component logic (Reciprocal)
    if let Value::Str(s) = &vm.grid[neighbor_y][neighbor_x] {
        if let Some(stripped) = s.strip_prefix("D:") {
            let dir = stripped.parse::<i64>().unwrap_or(0);
            // Invert perspective: neighbor is self, self is neighbor
            mult *= check_diode(dir, neighbor_y, neighbor_x, y, x, vm);
        } else if let Some(stripped) = s.strip_prefix("T:") {
            let dir = stripped.parse::<i64>().unwrap_or(0);
            mult *= check_transistor(dir, neighbor_y, neighbor_x, y, x, vm);
        }
    }

    mult
}

fn check_diode(dir: i64, y: usize, x: usize, ny: usize, nx: usize, vm: &ChimeraVM) -> f32 {
    let (dy, dx) = match dir {
        0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
    };
    let ry = ny as i64 - y as i64;
    let rx = nx as i64 - x as i64;

    if ry == dy && rx == dx {
        // Forward (Cathode) - Block reverse current
        if vm.voltage_grid[y][x] < vm.voltage_grid[ny][nx] {
            return 0.001;
        }
    } else if ry == -dy && rx == -dx {
        // Backward (Anode) - Block reverse current
        if vm.voltage_grid[ny][nx] < vm.voltage_grid[y][x] {
            return 0.001;
        }
    } else {
        return 0.0; // Block sides
    }
    1.0
}

fn check_transistor(dir: i64, y: usize, x: usize, ny: usize, nx: usize, vm: &ChimeraVM) -> f32 {
    let (dy, dx) = match dir {
        0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
    };
    let ry = ny as i64 - y as i64;
    let rx = nx as i64 - x as i64;

    if ry == dy && rx == dx {
        return 0.0; // Base is high impedance
    }

    // Check Base Voltage
    if let Some((by, bx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        if vm.voltage_grid[by][bx] < 50.0 {
            return 0.001;
        }
    }
    1.0
}

#[allow(clippy::needless_range_loop)]
pub fn update_circuit(vm: &mut ChimeraVM) {
    let iterations = 10;
    let grid_size = GRID_SIZE;

    #[cfg(feature = "biophysics")]
    apply_biophysics_coupling(vm, grid_size);

    // Update Sensors
    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Value::Str(s) = &vm.grid[y][x] {
                if let Some(stripped) = s.strip_prefix("S:") {
                    let mode = stripped.parse::<i64>().unwrap_or(0);
                    update_sensor(vm, y, x, mode);
                }
            }
        }
    }

    let mut next_voltage = vm.voltage_grid.clone();

    for _ in 0..iterations {
        for y in 0..grid_size {
            for x in 0..grid_size {
                if vm.resistance_grid[y][x] < 0.0 {
                    next_voltage[y][x] = vm.voltage_grid[y][x];
                    continue;
                }

                let base_cond = get_conductivity(&vm.grid[y][x]);

                if base_cond <= 0.001 {
                    next_voltage[y][x] = vm.voltage_grid[y][x] * 0.9;
                    continue;
                }

                let mut v_sum = 0.0;
                let mut weight_sum = 0.0;

                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        let neighbor_r = vm.resistance_grid[ny][nx];
                        let neighbor_cond = if neighbor_r < 0.0 {
                            10.0
                        } else {
                            get_conductivity(&vm.grid[ny][nx])
                        };

                        if neighbor_cond <= 0.001 {
                            continue;
                        }

                        let mult = get_component_multiplier(vm, y, x, ny, nx);
                        let effective_cond = (base_cond * neighbor_cond) / (base_cond + neighbor_cond) * mult;

                        v_sum += vm.voltage_grid[ny][nx] * effective_cond;
                        weight_sum += effective_cond;
                    }
                }

                if weight_sum > 0.0 {
                    next_voltage[y][x] = v_sum / weight_sum;
                } else {
                    next_voltage[y][x] = vm.voltage_grid[y][x] * 0.95;
                }
            }
        }
        vm.voltage_grid = next_voltage.clone();
    }

    update_muscles(vm, grid_size);
    update_current_grid(vm, grid_size);
}

#[cfg(feature = "biophysics")]
fn apply_biophysics_coupling(vm: &mut ChimeraVM, grid_size: usize) {
    let mut coupling_data = Vec::new();
    for (coord, weight) in &vm.biophysics_couplings {
        if let Some(neuron) = vm.neurons.get(coord) {
            if coord.0 < grid_size && coord.1 < grid_size {
                let grid_v = vm.voltage_grid[coord.0][coord.1];
                coupling_data.push((*coord, *weight, neuron.v, grid_v));
            }
        }
    }
    for (coord, weight, neuron_v, grid_v) in coupling_data {
        vm.voltage_grid[coord.0][coord.1] += neuron_v * weight * 0.1;
        if let Some(neuron) = vm.neurons.get_mut(&coord) {
            neuron.i_inj += grid_v * weight;
        }
    }
}

fn update_sensor(vm: &mut ChimeraVM, y: usize, x: usize, mode: i64) {
    let active = match mode {
        0 => {
            #[cfg(feature = "nova")]
            {
                vm.organelles.iter().any(|o| o.context_loc == (y, x))
            }
            #[cfg(not(feature = "nova"))]
            false
        },
        1 => {
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut found = false;
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                        found = true;
                        break;
                    }
                }
            }
            found
        },
        _ => false
    };

    if active {
        vm.voltage_grid[y][x] = 100.0;
        vm.resistance_grid[y][x] = -1.0;
    } else if vm.resistance_grid[y][x] == -1.0 {
            vm.resistance_grid[y][x] = 1.0;
    }
}

fn update_muscles(vm: &mut ChimeraVM, grid_size: usize) {
    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Value::Str(s) = &vm.grid[y][x] {
                if let Some(stripped) = s.strip_prefix("M:") {
                    let threshold = stripped.parse::<f32>().unwrap_or(50.0);
                    if vm.voltage_grid[y][x] > threshold {
                        #[cfg(feature = "nova")]
                        {
                            let topology = vm.topology;
                            for org in vm.organelles.iter_mut() {
                                if org.context_loc == (y, x) {
                                    use ::rand::Rng;
                                    let mut rng = ::rand::thread_rng();
                                    let dy = rng.gen_range(-1..=1);
                                    let dx = rng.gen_range(-1..=1);
                                    if let Some((ny, nx)) = topology.normalize(y as i64 + dy, x as i64 + dx, GRID_SIZE, GRID_SIZE) {
                                        org.context_loc = (ny, nx);
                                    }
                                }
                            }
                        }

                        vm.visual_effects.push(VisualEffect::Spark {
                            loc: (y, x),
                            color: (200, 100, 100),
                            ttl: 1,
                        });
                    }
                }
            }
        }
    }
}

fn update_current_grid(vm: &mut ChimeraVM, grid_size: usize) {
    for y in 0..grid_size {
        for x in 0..grid_size {
            let conductivity = get_conductivity(&vm.grid[y][x]);
            let mut max_diff = 0.0;
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let diff = (vm.voltage_grid[y][x] - vm.voltage_grid[ny][nx]).abs();
                    if diff > max_diff {
                        max_diff = diff;
                    }
                }
            }
            vm.current_grid[y][x] = max_diff * conductivity;
        }
    }
}
