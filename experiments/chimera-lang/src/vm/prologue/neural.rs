#[cfg(feature = "biophysics")]
use super::normalize_coords;
#[cfg(feature = "biophysics")]
use crate::vm::neuron::Neuron;
#[cfg(feature = "biophysics")]
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "biophysics")]
use std::collections::HashMap;

#[cfg(feature = "biophysics")]
pub fn scan_neural_grid(vm: &mut ChimeraVM) {
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();
    let mut active_neurons = std::collections::HashSet::new();

    for (y, x) in runes {
        if let Value::Str(s) = &vm.grid[y][x] {
            if s == "♦" {
                if !vm.neurons.contains_key(&(y, x)) {
                    vm.neurons.insert((y, x), Neuron::new());
                    vm.output
                        .push(format!("NEURAL: New Neuron formed at {},{}", x, y));
                }
                active_neurons.insert((y, x));
            } else if s == "•" {
                // Synapse Rune: Default weight 100 if not present
                if !vm.prologue_state.registers.contains_key(&(y, x)) {
                    vm.prologue_state.registers.insert((y, x), Value::Int(100));
                }
            }
        }
    }
    // Garbage Collection: Remove neurons that no longer have a corresponding rune
    let existing_neurons: Vec<(usize, usize)> = vm.neurons.keys().cloned().collect();
    for coord in existing_neurons {
        if !active_neurons.contains(&coord) {
            vm.neurons.remove(&coord);
            vm.output
                .push(format!("NEURAL: Neuron decayed at {},{}", coord.1, coord.0));
        }
    }
}

#[cfg(feature = "biophysics")]
pub fn apply_neural_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    registers: &mut HashMap<(usize, usize), Value>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    // Guard against multiple executions in the same tick (propagation loop)
    if next_signals[y][x].is_some() {
        return false;
    }

    match rune {
        "•" => {
            // Synapse: Reads West signal, applies weight, outputs Self (to be read by East neighbor).
            // Weight is stored in registers (default 100).
            if let Some(val) = w_sig {
                let weight = if let Some(Value::Int(w)) = registers.get(&(y, x)) {
                    *w
                } else {
                    100
                };

                let weighted_val = match val {
                    Value::Int(n) => Value::Int(n * weight / 100),
                    // For strings, we can't really multiply, so we pass through if weight > 50
                    _ => {
                        if weight > 50 {
                            val
                        } else {
                            Value::Int(0)
                        }
                    }
                };

                next_signals[y][x] = Some(weighted_val);
                changes = true;
            }
        }
        "°" => {
            // Learning: Reads West signal. If active, increases weight of adjacent Synapses.
            if let Some(val) = w_sig {
                let active = match val {
                    Value::Int(n) => n > 0,
                    Value::Str(s) => !s.is_empty(),
                    _ => false,
                };

                if active {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if let Some(Value::Int(w)) = registers.get_mut(&(ny, nx)) {
                                *w = (*w + 10).clamp(0, 500); // Increase weight, cap at 500%
                            }
                        }
                    }
                    // Light up self
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(1));
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

#[cfg(feature = "biophysics")]
pub fn fire_neurons(vm: &mut ChimeraVM) {
    // Phase 1: Check for recent spikes and emit signals
    // If a neuron spiked in the last tick, we emit a signal NOW for this tick's propagation.
    let tick = vm.tick_counter;

    let mut spikes = Vec::new();
    let mut voltages = Vec::new();

    for ((y, x), neuron) in &vm.neurons {
        if neuron.last_spike == tick.saturating_sub(1) {
            spikes.push((*y, *x));
        }
        if neuron.v > -55.0 {
            voltages.push((*y, *x, neuron.v));
        }
    }

    for (y, x) in &spikes {
        vm.prologue_state.signal_grid[*y][*x] = Some(Value::Int(1));

        // Elektra Output
        #[cfg(feature = "elektra")]
        {
            if *y < GRID_SIZE && *x < GRID_SIZE {
                vm.voltage_grid[*y][*x] = 100.0;
                vm.resistance_grid[*y][*x] = -1.0; // Source
            }
        }

        #[cfg(feature = "nova")]
        {
            // Flash light
            vm.light_grid[*y][*x] = 255;
            vm.light_color_grid[*y][*x] = (100, 200, 255); // Cyan flash
        }
    }

    // Emit faint glow for active neurons
    #[cfg(feature = "nova")]
    for (y, x, v) in &voltages {
        let brightness = ((v + 65.0) * 2.0).clamp(0.0, 50.0) as i64;
        if brightness > 0 {
            vm.light_grid[*y][*x] = vm.light_grid[*y][*x].max(brightness);
        }
    }

    // Phase 1.5: Synaptic Plasticity & Growth
    let mut new_synapses = Vec::new();
    let mut weight_updates = Vec::new();

    // Growth: Spiking neurons look for active neighbors
    for (sy, sx) in &spikes {
        for dy in -2..=2 {
            for dx in -2..=2 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if let Some((ny, nx)) = normalize_coords(*sy as i64 + dy, *sx as i64 + dx) {
                    if let Some(target_neuron) = vm.neurons.get(&(ny, nx)) {
                        // Check if already connected
                        let connected = vm
                            .biophysics_synapses
                            .get(&(*sy, *sx))
                            .map(|v| v.iter().any(|(t, _)| *t == (ny, nx)))
                            .unwrap_or(false);

                        if !connected {
                            // Hebbian Growth: If target is also active (recently spiked or depolarized)
                            if target_neuron.last_spike >= tick.saturating_sub(5)
                                || target_neuron.v > -50.0
                            {
                                new_synapses.push(((*sy, *sx), (ny, nx)));
                            }
                        }
                    }
                }
            }
        }
    }

    // STDP Learning
    // We iterate all synapses to find Hebbian events
    for (src, targets) in &vm.biophysics_synapses {
        for (tgt, _) in targets {
            let src_spiked = spikes.contains(src);
            let tgt_spiked = spikes.contains(tgt);

            if src_spiked && !tgt_spiked {
                // Pre spiked now. Post might have spiked recently (Pre before Post -> LTP)
                // Wait. STDP:
                // Pre then Post (Positive delay) -> LTP
                // Post then Pre (Negative delay) -> LTD
                //
                // Case 1: Pre spiked NOW (t). Post spiked recently (t-delta).
                // This implies Post BEFORE Pre -> LTD.
                if let Some(tgt_neuron) = vm.neurons.get(tgt) {
                    let window = vm.neurons[src].stdp_window;
                    if tgt_neuron.last_spike >= tick.saturating_sub(window)
                        && tgt_neuron.last_spike < tick.saturating_sub(1)
                    {
                        weight_updates.push((*src, *tgt, -vm.neurons[src].plasticity));
                    }
                }
            } else if tgt_spiked && !src_spiked {
                // Post spiked NOW (t). Pre might have spiked recently (t-delta).
                // This implies Pre BEFORE Post -> LTP.
                if let Some(src_neuron) = vm.neurons.get(src) {
                    let window = src_neuron.stdp_window;
                    if src_neuron.last_spike >= tick.saturating_sub(window)
                        && src_neuron.last_spike < tick.saturating_sub(1)
                    {
                        weight_updates.push((*src, *tgt, vm.neurons[src].plasticity));
                    }
                }
            } else if src_spiked && tgt_spiked {
                // Simultaneous -> Associative -> LTP
                if let Some(src_neuron) = vm.neurons.get(src) {
                    weight_updates.push((*src, *tgt, src_neuron.plasticity));
                }
            }
        }
    }

    // Apply Growth
    for (src, tgt) in new_synapses {
        vm.biophysics_synapses
            .entry(src)
            .or_default()
            .push((tgt, 0.1));
    }

    // Apply Weights
    for (src, tgt, delta) in weight_updates {
        if let Some(targets) = vm.biophysics_synapses.get_mut(&src) {
            for (t, w) in targets.iter_mut() {
                if *t == tgt {
                    *w = (*w + delta).clamp(0.0, 5.0);
                }
            }
        }
    }
}

#[cfg(feature = "biophysics")]
pub fn integrate_neurons(vm: &mut ChimeraVM) {
    // Phase 2: Integrate signals from grid into neurons
    // This happens after propagation, so we read signal_grid.

    let mut inputs = Vec::new();

    for ((y, x), _) in &vm.neurons {
        // Check neighbors
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut sum_input = 0.0;

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(*y as i64 + dy, *x as i64 + dx) {
                if let Some(val) = &vm.prologue_state.signal_grid[ny][nx] {
                    match val {
                        Value::Int(n) => sum_input += *n as f32,
                        Value::Str(s) => sum_input += s.len() as f32,
                        _ => {}
                    }
                }
            }
        }

        if sum_input > 0.0 {
            inputs.push((*y, *x, sum_input));
        }
    }

    // Apply inputs
    for (y, x, input) in inputs {
        if let Some(neuron) = vm.neurons.get_mut(&(y, x)) {
            neuron.i_inj += input * 10.0; // Scale input current

            // Elektra Input
            #[cfg(feature = "elektra")]
            {
                if y < GRID_SIZE && x < GRID_SIZE {
                    let v = vm.voltage_grid[y][x];
                    if v.abs() > 0.1 {
                        neuron.i_inj += v * 0.1;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "biophysics")]
pub fn apply_neural_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // West signal for input (Rule Name)
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "🌱" => {
            // Neural Growth: Reads West (Grammar Rule Name).
            // Generates blueprint and executes L-System.
            if let Some(Value::Str(name)) = w_sig {
                if let Ok(blueprint) = vm.prologue_state.logos_engine.generate(&name) {
                    execute_neural_l_system(vm, &blueprint, y, x);
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Activate
                }
            }
        }
        _ => {}
    }
}

#[cfg(feature = "biophysics")]
fn execute_neural_l_system(vm: &mut ChimeraVM, blueprint: &str, start_y: usize, start_x: usize) {
    let mut cy = start_y;
    let mut cx = start_x;
    let mut dir = 1; // 0=N, 1=E, 2=S, 3=W (Start East)
    let mut stack = Vec::new();

    // Limit execution to prevent infinite loops or massive writes
    let max_steps = 100;
    let mut steps = 0;

    for char in blueprint.chars() {
        if steps >= max_steps {
            break;
        }
        steps += 1;

        match char {
            'F' => {
                let (dy, dx) = match dir {
                    0 => (-1, 0),
                    1 => (0, 1),
                    2 => (1, 0),
                    3 => (0, -1),
                    _ => (0, 0),
                };
                if let Some((ny, nx)) = normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                    cy = ny;
                    cx = nx;
                    // Draw Wire if empty
                    if matches!(vm.grid[cy][cx], Value::Int(0)) {
                         vm.grid[cy][cx] = Value::Str("~".to_string());
                    }
                }
            }
            '+' => dir = (dir + 1) % 4,
            '-' => dir = (dir + 3) % 4,
            '[' => stack.push((cy, cx, dir)),
            ']' => {
                if let Some((py, px, pdir)) = stack.pop() {
                    cy = py;
                    cx = px;
                    dir = pdir;
                }
            }
            'N' => {
                if matches!(vm.grid[cy][cx], Value::Int(0)) || matches!(vm.grid[cy][cx], Value::Str(ref s) if s == "~") {
                    vm.grid[cy][cx] = Value::Str("♦".to_string());
                    // Register new neuron immediately so it works next tick
                    if !vm.neurons.contains_key(&(cy, cx)) {
                        vm.neurons.insert((cy, cx), Neuron::new());
                    }
                }
            }
            'S' => {
                if matches!(vm.grid[cy][cx], Value::Int(0)) || matches!(vm.grid[cy][cx], Value::Str(ref s) if s == "~") {
                    vm.grid[cy][cx] = Value::Str("•".to_string());
                    vm.prologue_state.registers.insert((cy, cx), Value::Int(100)); // Default weight
                }
            }
            'L' => {
                if matches!(vm.grid[cy][cx], Value::Int(0)) || matches!(vm.grid[cy][cx], Value::Str(ref s) if s == "~") {
                    vm.grid[cy][cx] = Value::Str("°".to_string());
                }
            }
            _ => {} // Ignore other chars
        }
    }
}
