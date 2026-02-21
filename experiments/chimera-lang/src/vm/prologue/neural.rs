#[cfg(feature = "biophysics")]
use super::normalize_coords;
#[cfg(feature = "biophysics")]
use crate::vm::neuron::Neuron;
#[cfg(feature = "biophysics")]
use crate::vm::{ChimeraVM, Value};

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
pub fn fire_neurons(vm: &mut ChimeraVM) {
    // Phase 1: Check for recent spikes and emit signals
    // If a neuron spiked in the last tick, we emit a signal NOW for this tick's propagation.
    let tick = vm.tick_counter;

    // We iterate over neurons. Since we need mutable access to prologue_state,
    // we can't iterate vm.neurons mutably at the same time?
    // Actually, we just need read access to neurons and write access to signal_grid.
    // But vm.neurons is part of vm.
    // We can collect spikes first.

    let mut spikes = Vec::new();
    let mut voltages = Vec::new();

    for ((y, x), neuron) in &vm.neurons {
        // Check if spiked recently (e.g. last tick or this tick if step ran first)
        // Assuming step ran AFTER prologue last tick, or BEFORE prologue this tick.
        // Let's assume step runs AFTER. So we check last_spike == tick - 1.
        if neuron.last_spike == tick.saturating_sub(1) {
            spikes.push((*y, *x));
        }

        // Optional: Emit analog voltage?
        if neuron.v > -55.0 {
            voltages.push((*y, *x, neuron.v));
        }
    }

    for (y, x) in spikes {
        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));

        #[cfg(feature = "nova")]
        {
            // Flash light
            vm.light_grid[y][x] = 255;
            vm.light_color_grid[y][x] = (100, 200, 255); // Cyan flash
        }
    }

    // Emit faint glow for active neurons
    #[cfg(feature = "nova")]
    for (y, x, v) in voltages {
        let brightness = ((v + 65.0) * 2.0).clamp(0.0, 50.0) as i64;
        if brightness > 0 {
            vm.light_grid[y][x] = vm.light_grid[y][x].max(brightness);
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
        }
    }
}
