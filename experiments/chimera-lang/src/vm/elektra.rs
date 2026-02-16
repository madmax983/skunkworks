use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::VisualEffect;

pub fn exec_elektra_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Electrogenesis => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    let cost = amount.abs().max(1);
                    if vm.energy >= cost {
                        vm.energy -= cost;
                        let (y, x) = vm.context_loc;
                        vm.voltage_grid[y][x] += amount as f32;
                        vm.output
                            .push(format!("ELECTROGENESIS: +{}V at {},{}", amount, x, y));
                    } else {
                        vm.output
                            .push("ELECTROGENESIS: Not enough energy".to_string());
                    }
                }
            }
        }
        OpCode::Induction => {
            let (y, x) = vm.context_loc;
            let v = vm.voltage_grid[y][x];
            let energy_gain = v.abs() as i64;
            if energy_gain > 0 {
                vm.energy = vm.energy.saturating_add(energy_gain);
                vm.voltage_grid[y][x] = 0.0;
                vm.stack.push(Value::Int(energy_gain));
                vm.output.push(format!(
                    "INDUCTION: Absorbed {}V -> {} Energy",
                    v, energy_gain
                ));
            } else {
                vm.stack.push(Value::Int(0));
            }
        }
        OpCode::WireGrowth => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(dir) = val {
                    if vm.energy >= 5 {
                        let (cy, cx) = vm.context_loc;
                        let (dy, dx) = match dir % 4 {
                            0 => (-1, 0),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        let ny = cy as i64 + dy;
                        let nx = cx as i64 + dx;

                        if ny >= 0 && ny < GRID_SIZE as i64 && nx >= 0 && nx < GRID_SIZE as i64 {
                            let ny = ny as usize;
                            let nx = nx as usize;
                            vm.grid[ny][nx] = Value::Int(1);
                            vm.energy -= 5;
                            vm.output.push(format!("WIREGROWTH: Wire at {},{}", nx, ny));
                        } else {
                            vm.output.push("WIREGROWTH: Out of bounds".to_string());
                        }
                    } else {
                        vm.output.push("WIREGROWTH: Not enough energy".to_string());
                    }
                }
            }
        }
        OpCode::CircuitBreaker => {
            if vm.stack.len() >= 2 {
                let s_val = vm.stack.pop().unwrap();
                let t_val = vm.stack.pop().unwrap();

                if let (Value::Int(strand_idx), Value::Int(threshold)) = (s_val, t_val) {
                    let (y, x) = vm.context_loc;
                    let v = vm.voltage_grid[y][x];
                    if v > threshold as f32 {
                        if strand_idx >= 0 {
                            vm.output
                                .push(format!("CIRCUITBREAKER: Tripped at {}V > {}", v, threshold));
                            return Some((strand_idx as usize, 0));
                        }
                    }
                }
            }
        }
        OpCode::Battery => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let v_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(v)) = (x_val, y_val, v_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let ux = x as usize;
                        let uy = y as usize;
                        vm.voltage_grid[uy][ux] = v as f32;
                        vm.resistance_grid[uy][ux] = -1.0;
                        vm.output.push(format!("BATTERY: {}V at {},{}", v, x, y));
                    }
                }
            }
        }
        OpCode::Ground => {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let ux = x as usize;
                        let uy = y as usize;
                        vm.voltage_grid[uy][ux] = 0.0;
                        vm.resistance_grid[uy][ux] = -2.0;
                        vm.output.push(format!("GROUND: 0V at {},{}", x, y));
                    }
                }
            }
        }
        OpCode::SenseVolt => {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let v = vm.voltage_grid[y as usize][x as usize];
                        vm.stack.push(Value::Int(v as i64));
                    } else {
                        vm.stack.push(Value::Int(0));
                    }
                }
            }
        }
        OpCode::Shock => {
            if vm.stack.len() >= 2 {
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
            }
        }
        OpCode::Lightning => {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    vm.output.push(format!("LIGHTNING: Strike at {},{}", x, y));
                }
            }
        }
        OpCode::TeslaCoil => {
            if vm.stack.len() >= 2 {
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
                                vm.output
                                    .push(format!("TESLA COIL: Fried {} organelles", hit_count));
                            }
                        }
                    } else {
                        vm.output
                            .push("TESLA COIL: Insufficient voltage".to_string());
                    }
                }
            }
        }
        #[cfg(all(feature = "elektra", feature = "nova"))]
        OpCode::Galvanize => {
            if let Some(val) = vm.stack.pop() {
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
            }
        }
        OpCode::Diode => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let dir_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(dir)) = (x_val, y_val, dir_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("D:{}", dir % 4));
                        vm.output.push(format!("DIODE: Created at {},{} dir {}", nx, ny, dir % 4));
                    }
                }
            }
        }
        OpCode::Transistor => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let dir_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(dir)) = (x_val, y_val, dir_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("T:{}", dir % 4));
                        vm.output.push(format!("TRANSISTOR: Created at {},{} base {}", nx, ny, dir % 4));
                    }
                }
            }
        }
        OpCode::Muscle => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let t_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(t)) = (x_val, y_val, t_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("M:{}", t));
                        vm.output.push(format!("MUSCLE: Created at {},{} thresh {}", nx, ny, t));
                    }
                }
            }
        }
        OpCode::Sensor => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let m_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(m)) = (x_val, y_val, m_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("S:{}", m));
                        vm.output.push(format!("SENSOR: Created at {},{} mode {}", nx, ny, m));
                    }
                }
            }
        }
        _ => {}
    }
    None
}

// Helper to calculate effective conductivity multiplier based on component logic
fn get_component_multiplier(vm: &ChimeraVM, y: usize, x: usize, neighbor_y: usize, neighbor_x: usize) -> f32 {
    let mut mult = 1.0;

    // Check SELF component logic
    if let Value::Str(s) = &vm.grid[y][x] {
        if s.starts_with("D:") {
            let dir = s.trim_start_matches("D:").parse::<i64>().unwrap_or(0);
            let (dy, dx) = match dir {
                0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
            };
            // Relative position of neighbor
            let ry = neighbor_y as i64 - y as i64;
            let rx = neighbor_x as i64 - x as i64;

            // Check wrapping (simple check, might be wrong for torus boundary)
            // Assuming neighbors are adjacent.

            // Forward (Cathode)
            if ry == dy && rx == dx {
                if vm.voltage_grid[y][x] < vm.voltage_grid[neighbor_y][neighbor_x] {
                    mult *= 0.001;
                }
            }
            // Backward (Anode)
            else if ry == -dy && rx == -dx {
                if vm.voltage_grid[neighbor_y][neighbor_x] < vm.voltage_grid[y][x] {
                    mult *= 0.001;
                }
            }
            else {
                mult = 0.0; // Block sides
            }
        } else if s.starts_with("T:") {
            let dir = s.trim_start_matches("T:").parse::<i64>().unwrap_or(0);
            let (dy, dx) = match dir {
                0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
            };
            let ry = neighbor_y as i64 - y as i64;
            let rx = neighbor_x as i64 - x as i64;

            if ry == dy && rx == dx {
                mult = 0.0; // Base is high impedance
            } else {
                // Check Base Voltage
                if let Some((by, bx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if vm.voltage_grid[by][bx] < 50.0 {
                        mult *= 0.001;
                    }
                }
            }
        }
    }

    // Check NEIGHBOR component logic (Reciprocal)
    if let Value::Str(s) = &vm.grid[neighbor_y][neighbor_x] {
        if s.starts_with("D:") {
            let dir = s.trim_start_matches("D:").parse::<i64>().unwrap_or(0);
            let (dy, dx) = match dir {
                0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
            };
            // Relative position of SELF from neighbor
            let ry = y as i64 - neighbor_y as i64;
            let rx = x as i64 - neighbor_x as i64;

            if ry == dy && rx == dx {
                // We are at Neighbor's Cathode
                if vm.voltage_grid[neighbor_y][neighbor_x] < vm.voltage_grid[y][x] {
                    mult *= 0.001;
                }
            } else if ry == -dy && rx == -dx {
                // We are at Neighbor's Anode
                if vm.voltage_grid[y][x] < vm.voltage_grid[neighbor_y][neighbor_x] {
                    mult *= 0.001;
                }
            } else {
                mult = 0.0;
            }
        } else if s.starts_with("T:") {
            let dir = s.trim_start_matches("T:").parse::<i64>().unwrap_or(0);
            let (dy, dx) = match dir {
                0 => (-1, 0), 1 => (0, 1), 2 => (1, 0), 3 => (0, -1), _ => (0, 0)
            };
            let ry = y as i64 - neighbor_y as i64;
            let rx = x as i64 - neighbor_x as i64;

            if ry == dy && rx == dx {
                mult = 0.0;
            } else {
                if let Some((by, bx)) = vm.normalize_coords(neighbor_y as i64 + dy, neighbor_x as i64 + dx) {
                    if vm.voltage_grid[by][bx] < 50.0 {
                        mult *= 0.001;
                    }
                }
            }
        }
    }

    mult
}

#[allow(clippy::needless_range_loop)]
pub fn update_circuit(vm: &mut ChimeraVM) {
    let iterations = 10;
    let grid_size = GRID_SIZE;

    #[cfg(feature = "biophysics")]
    {
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

    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s.starts_with("S:") {
                    let mode = s.trim_start_matches("S:").parse::<i64>().unwrap_or(0);
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
            }
        }
    }

    let mut next_voltage = vm.voltage_grid.clone();

    for _ in 0..iterations {
        for y in 0..grid_size {
            for x in 0..grid_size {
                let r_self = vm.resistance_grid[y][x];

                if r_self < 0.0 {
                    next_voltage[y][x] = vm.voltage_grid[y][x];
                    continue;
                }

                let cell_val = &vm.grid[y][x];
                let base_cond = match cell_val {
                    Value::Int(0) => 0.0,
                    Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                    Value::Str(s) => {
                        if s.starts_with("D:") || s.starts_with("T:") || s.starts_with("M:") || s.starts_with("S:") {
                            10.0
                        } else {
                            0.01
                        }
                    }
                    Value::Int(_) => 0.01,
                    _ => 0.0,
                };

                if base_cond <= 0.001 {
                    next_voltage[y][x] = vm.voltage_grid[y][x] * 0.9;
                    continue;
                }

                let mut v_sum = 0.0;
                let mut weight_sum = 0.0;

                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        let neighbor_val = &vm.grid[ny][nx];
                        let neighbor_r = vm.resistance_grid[ny][nx];
                        let neighbor_cond = if neighbor_r < 0.0 {
                            10.0
                        } else {
                            match neighbor_val {
                                Value::Int(0) => 0.0,
                                Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                                Value::Str(s) => {
                                    if s.starts_with("D:") || s.starts_with("T:") || s.starts_with("M:") || s.starts_with("S:") {
                                        10.0
                                    } else {
                                        0.01
                                    }
                                },
                                Value::Int(_) => 0.01,
                                _ => 0.0,
                            }
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

    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s.starts_with("M:") {
                    let threshold = s.trim_start_matches("M:").parse::<f32>().unwrap_or(50.0);
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

    for y in 0..grid_size {
        for x in 0..grid_size {
            let cell_val = &vm.grid[y][x];
            let conductivity = match cell_val {
                Value::Int(0) => 0.0,
                Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                Value::Str(s) if s.starts_with("D:") || s.starts_with("T:") || s.starts_with("M:") => 10.0,
                Value::Int(_) => 0.01,
                _ => 0.0,
            };

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
