use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

pub fn apply_symbiosis_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };

    // Helper to get neighbor coords
    let east_coords = normalize_coords(y as i64, x as i64 + 1);
    let west_coords = normalize_coords(y as i64, x as i64 - 1);

    match rune {
        "u" => {
            // Upload: West (Value) -> Push to Stack
            if let Some(val) = w_sig {
                vm.stack.push(val.clone());
                vm.output.push(format!("SYMBIOSIS: Uploaded {:?}", val));
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
            }
        }
        "y" => {
            // Yank: West (Trigger) -> Pop from Stack -> Write South
            if w_sig.is_some() {
                if let Some(val) = vm.stack.pop() {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = val.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(val.clone()); // Light up with value
                        vm.output.push(format!("SYMBIOSIS: Yanked {:?}", val));
                    }
                } else {
                    vm.output
                        .push("SYMBIOSIS: Stack Underflow on Yank".to_string());
                }
            }
        }
        "p" => {
            // Parasite: West (Payload) -> East (Host)
            if let (Some(payload), Some((ey, ex))) = (w_sig, east_coords) {
                let target_val = vm.grid[ey][ex].clone();
                let mut mutated = false;

                if let Value::Str(s) = target_val {
                    if s == "C" {
                        // Critter Injection
                        if let Some(state_val) = vm.prologue_state.registers.get(&(ey, ex)) {
                            if let Value::Str(state_str) = state_val {
                                match state_str.parse::<super::critter::CritterState>() {
                                    Ok(mut critter) => {
                                        let injection = match &payload {
                                            Value::Str(g) => g.clone(),
                                            Value::Int(i) => format!("{}", i),
                                            _ => "M".to_string(),
                                        };

                                        critter.genes.push_str(&injection);
                                        let new_state = critter.to_value();

                                        vm.prologue_state
                                            .registers
                                            .insert((ey, ex), new_state.clone());

                                        // Sync agent list
                                        for agent in vm.prologue_state.agents.iter_mut() {
                                            if agent.x == ex && agent.y == ey {
                                                agent.state = new_state.clone();
                                                break;
                                            }
                                        }

                                        mutated = true;
                                        vm.output.push(format!("SYMBIOSIS: Parasite injected '{}' into Critter at {},{}", injection, ex, ey));
                                    }
                                    Err(_) => {
                                        vm.output.push(format!(
                                            "SYMBIOSIS ERROR: Failed to parse critter state: {}",
                                            state_str
                                        ));
                                    }
                                }
                            } else {
                                vm.output
                                    .push("SYMBIOSIS ERROR: Register not a string".to_string());
                            }
                        } else {
                            vm.output
                                .push("SYMBIOSIS ERROR: Register not found".to_string());
                        }
                    } else if s == "@" || s == "K" || s == "H" {
                        // Simple Agent Injection
                        vm.prologue_state
                            .registers
                            .insert((ey, ex), payload.clone());
                        mutated = true;
                        vm.output.push(format!(
                            "SYMBIOSIS: Parasite overwrote Agent state at {},{}",
                            ex, ey
                        ));
                    }
                }

                #[cfg(feature = "nova")]
                if !mutated {
                    for org in vm.organelles.iter_mut() {
                        if org.context_loc == (ey, ex) {
                            match &payload {
                                Value::Int(id) => {
                                    org.genome_id = *id as u64;
                                    mutated = true;
                                    vm.output.push(format!(
                                        "SYMBIOSIS: Parasite switched Organelle genome to {}",
                                        id
                                    ));
                                }
                                Value::Str(code) => {
                                    org.traits.push(code.clone());
                                    mutated = true;
                                    vm.output.push(format!(
                                        "SYMBIOSIS: Parasite added trait '{}'",
                                        code
                                    ));
                                }
                                _ => {}
                            }
                            break;
                        }
                    }
                }

                if mutated {
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        "o" => {
            // Osmosis: West <-> East
            if let (Some((wy, wx)), Some((ey, ex))) = (west_coords, east_coords) {
                let w_val = &vm.grid[wy][wx];
                let e_val = &vm.grid[ey][ex];

                let is_agent = |v: &Value| matches!(v, Value::Str(s) if s == "C" || s == "@" || s == "K" || s == "H");

                if is_agent(w_val) && is_agent(e_val) {
                    let mut w_state = vm.prologue_state.registers.get(&(wy, wx)).cloned();
                    let mut e_state = vm.prologue_state.registers.get(&(ey, ex)).cloned();

                    if let (Some(Value::Str(ws)), Some(Value::Str(es))) = (&w_state, &e_state) {
                        if let (Ok(mut wc), Ok(mut ec)) = (
                            ws.parse::<super::critter::CritterState>(),
                            es.parse::<super::critter::CritterState>(),
                        ) {
                            // Osmosis: Equalize Energy
                            let total_energy = wc.energy + ec.energy;
                            wc.energy = total_energy / 2;
                            ec.energy = total_energy - wc.energy;

                            // Swap a gene
                            if !wc.genes.is_empty() && !ec.genes.is_empty() {
                                let mut rng = rand::thread_rng();
                                let idx_w = rng.gen_range(0..wc.genes.len());
                                let idx_e = rng.gen_range(0..ec.genes.len());

                                let mut w_chars: Vec<char> = wc.genes.chars().collect();
                                let mut e_chars: Vec<char> = ec.genes.chars().collect();

                                std::mem::swap(&mut w_chars[idx_w], &mut e_chars[idx_e]);

                                wc.genes = w_chars.into_iter().collect();
                                ec.genes = e_chars.into_iter().collect();
                            }

                            w_state = Some(wc.to_value());
                            e_state = Some(ec.to_value());

                            vm.output.push(format!(
                                "SYMBIOSIS: Osmosis between Critters at {},{} and {},{}",
                                wx, wy, ex, ey
                            ));
                        }
                    }

                    if let Some(s) = w_state {
                        vm.prologue_state.registers.insert((wy, wx), s.clone());
                        for agent in vm.prologue_state.agents.iter_mut() {
                            if agent.x == wx && agent.y == wy {
                                agent.state = s.clone();
                                break;
                            }
                        }
                    }
                    if let Some(s) = e_state {
                        vm.prologue_state.registers.insert((ey, ex), s.clone());
                        for agent in vm.prologue_state.agents.iter_mut() {
                            if agent.x == ex && agent.y == ey {
                                agent.state = s.clone();
                                break;
                            }
                        }
                    }

                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        "x" => {
            // Xenograft: Swap positions West <-> East
            if let (Some((wy, wx)), Some((ey, ex))) = (west_coords, east_coords) {
                let w_val = vm.grid[wy][wx].clone();
                let e_val = vm.grid[ey][ex].clone();

                if !matches!(w_val, Value::Int(0)) && !matches!(e_val, Value::Int(0)) {
                    // Swap Grid
                    vm.grid[wy][wx] = e_val;
                    vm.grid[ey][ex] = w_val;

                    // Swap Registers
                    let w_reg = vm.prologue_state.registers.remove(&(wy, wx));
                    let e_reg = vm.prologue_state.registers.remove(&(ey, ex));

                    if let Some(r) = w_reg {
                        vm.prologue_state.registers.insert((ey, ex), r);
                    }
                    if let Some(r) = e_reg {
                        vm.prologue_state.registers.insert((wy, wx), r);
                    }

                    // Swap Agents in List
                    for agent in vm.prologue_state.agents.iter_mut() {
                        if agent.x == wx && agent.y == wy {
                            agent.x = ex;
                            agent.y = ey;
                        } else if agent.x == ex && agent.y == ey {
                            agent.x = wx;
                            agent.y = wy;
                        }
                    }

                    #[cfg(feature = "nova")]
                    {
                        for org in vm.organelles.iter_mut() {
                            if org.context_loc == (wy, wx) {
                                org.context_loc = (ey, ex);
                            } else if org.context_loc == (ey, ex) {
                                org.context_loc = (wy, wx);
                            }
                        }
                    }

                    vm.output.push(format!(
                        "SYMBIOSIS: Xenograft swap {},{} <-> {},{}",
                        wx, wy, ex, ey
                    ));
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        #[cfg(feature = "nova")]
        "w" => {
            // Write Akashic: West (Value), North (Key) -> Akashic
            if let (Some(val), Some(Value::Str(key))) = (w_sig, n_sig) {
                vm.akashic.storage.insert(key.clone(), val.clone());
                if let Err(e) = vm.akashic.save() {
                    vm.output.push(format!("SYMBIOSIS ERROR: {}", e));
                } else {
                    vm.output
                        .push(format!("SYMBIOSIS: Wrote Akashic '{}'", key));
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        #[cfg(feature = "nova")]
        "j" => {
            // Join (Read) Akashic: North (Key) -> Grid (South)
            if let Some(Value::Str(key)) = n_sig {
                if let Some(val) = vm.akashic.storage.get(&key) {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = val.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(val.clone());
                        vm.output.push(format!("SYMBIOSIS: Read Akashic '{}'", key));
                    }
                } else {
                    vm.output
                        .push(format!("SYMBIOSIS: Akashic Key '{}' not found", key));
                }
            }
        }
        _ => {}
    }
}
