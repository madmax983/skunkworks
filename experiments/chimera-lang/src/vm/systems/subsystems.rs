use crate::vm::ChimeraVM;
use crate::vm::*;

impl ChimeraVM {
    pub(crate) fn process_subsystems(&mut self, time_frozen: bool) {
        #[cfg(feature = "cortex")]
        if !time_frozen {
            self.update_cortex_state();
        }

        #[cfg(feature = "biophysics")]
        if !time_frozen {
            let mut spikes = Vec::new();
            for (coord, neuron) in self.neurons.iter_mut() {
                if neuron.step(0.1, self.tick_counter) {
                    spikes.push(*coord);
                }
            }

            for source in spikes {
                if let Some(targets) = self.biophysics_synapses.get(&source) {
                    for (target, weight) in targets {
                        if let Some(target_neuron) = self.neurons.get_mut(target) {
                            target_neuron.i_inj += weight * 10.0;
                        }
                    }
                }

                // Synaptic Bridge: Neuron -> Cortex
                #[cfg(feature = "cortex")]
                if let Some(strands) = self.neuron_to_cortex_map.get(&source) {
                    for &s_idx in strands {
                        if s_idx < self.activation_levels.len() {
                            self.activation_levels[s_idx] =
                                self.activation_levels[s_idx].saturating_add(20);
                        }
                    }
                }
            }
        }

        #[cfg(feature = "resonance")]
        if !time_frozen {
            self.update_audio_state();
        }

        #[cfg(feature = "silicon")]
        if !time_frozen && self.silicon_mode {
            silicon::step_circuit(self);
        }

        #[cfg(feature = "elektra")]
        if !time_frozen {
            elektra::update_circuit(self);

            // Process Patch Bay
            let mut energy_gain = 0.0;
            let mut chaos_mod = 0.0;

            for ((y, x), target) in &self.patch_bay {
                if *y < GRID_SIZE && *x < GRID_SIZE {
                    let v = self.voltage_grid[*y][*x];
                    match target {
                        PatchTarget::EnergyRegen => {
                            if v > 0.0 {
                                energy_gain += v * 0.1;
                            }
                        }
                        PatchTarget::MutationRate => {
                            chaos_mod += v * 0.01;
                        }
                    }
                }
            }

            if energy_gain > 0.0 {
                self.energy = self.energy.saturating_add(energy_gain as i64);
            }
            if chaos_mod > 0.0 {
                self.glitch_level = (self.glitch_level + chaos_mod).clamp(0.0, 1.0);
            }
        }
    }
}
