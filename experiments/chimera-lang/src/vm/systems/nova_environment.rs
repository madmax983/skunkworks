use crate::prelude::Value;
use crate::vm::ChimeraVM;
use crate::vm::*;
use rand::Rng;

impl ChimeraVM {
    pub(crate) fn process_nova_environment(&mut self, time_frozen: bool) {
        #[cfg(feature = "nova")]
        if !time_frozen {
            let manifestation = self.egregore.tick();
            match manifestation {
                nova_egregore::Manifestation::Smite => {
                    self.output.push("EGREGORE: SMITE!".to_string());
                    if !self.dna.helix.strands.is_empty() {
                        let mut rng = rand::thread_rng();
                        let idx = rng.gen_range(0..self.dna.helix.strands.len());
                        self.dna.helix.strands[idx].genes.clear();
                        self.output
                            .push(format!("EGREGORE: Struck down strand {}", idx));
                    }
                }
                nova_egregore::Manifestation::Bless => {
                    self.output.push("EGREGORE: BLESSING!".to_string());
                    self.energy = self.energy.saturating_add(100);
                }
                nova_egregore::Manifestation::Whisper(msg) => {
                    self.output.push(format!("EGREGORE: Whisper '{}'", msg));
                    self.stack.push(Value::Str(msg));
                }
                nova_egregore::Manifestation::Corrupt => {
                    self.output.push("EGREGORE: CORRUPTION!".to_string());
                    self.mutate();
                }
                nova_egregore::Manifestation::None => {}
            }

            nova_logistics::process_logistics(self);
            nova_ecology::tick_ecology(self);
            self.process_environment();
            nova_void::process_rifts(self);
            nova_flux::process_flux(self);
            nova_metamorphism::process_metamorphism(self);
            nova_metamorphosis::process_organelle_growth(self);
            if self.orca_mode {
                nova_signals::process_signals(self);
            }
            nova_sigil::process_passive_sigils(self);
            if self.relativity_mode {
                nova_relativity::update_relativity(self);
            }
            nova_ballistics::update_projectiles(self);
            nova_sovereignty::process_territory(self);
            nova_arcana::process_fate(self);
            #[cfg(feature = "resonance")]
            nova_resonance_war::process_resonance(self);
            nova_strings::update_strings(self);

            if self.logos_mode {
                nova_logos::process_logos(self);
            }
            if self.reactor_mode {
                nova_reactor::process_reactor(self);
            }
            if self.prologue_state.active {
                prologue::exec_prologue_tick(self);
            }
        }
    }
}
