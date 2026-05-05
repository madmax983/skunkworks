use crate::prelude::Value;
use crate::vm::ChimeraVM;
use crate::vm::*;
use rand::Rng;

impl ChimeraVM {
    pub(crate) fn process_chaos_and_events(&mut self, time_frozen: bool) -> bool {
        if !time_frozen {
            let mut havoc: havoc::HavocEngine = std::mem::take(&mut self.havoc);
            havoc.tick(self);
            self.havoc = havoc;
        }

        // Decay glitch effect
        self.glitch_level *= 0.95;
        if self.glitch_level < 0.01 {
            self.glitch_level = 0.0;
        }

        // Link Glitch Level to Babel Integrity
        #[cfg(feature = "nova")]
        {
            let integrity: f64 = self.babel_state.integrity;
            let chaos = (1.0f64 - integrity).max(0.0f64) as f32;
            if chaos > self.glitch_level {
                self.glitch_level = chaos;
            }
        }

        if !time_frozen {
            self.chaos_struct.tick();
        }

        if !time_frozen && self.chaos_mode {
            let mut rng = rand::thread_rng();
            #[cfg(feature = "nova")]
            let chance: f64 =
                0.1 * self.biome_grid[self.context_loc.0][self.context_loc.1].mutation_rate();
            #[cfg(not(feature = "nova"))]
            let chance: f64 = 0.1;

            if rng.gen_bool(chance.clamp(0.0, 1.0)) {
                self.mutate();
            }
        }

        #[cfg(feature = "nova")]
        if !time_frozen && self.glitch_level > 0.8 {
            let mut rng = rand::thread_rng();

            // Throttle message
            if self.tick_counter.is_multiple_of(10) {
                self.output.push("ENTROPY STORM ACTIVE".to_string());
            }

            // Spontaneous Tunneling (5%)
            if rng.gen_bool(0.05) {
                if let Some(target) = nova_flux::exec_quantum_tunnel(self) {
                    self.ip = target;
                }
            }

            // Reality Flux (5%)
            if rng.gen_bool(0.05) {
                let rows = self.grid.len();
                if rows > 0 {
                    let cols = self.grid[0].len();
                    let ry = rng.gen_range(0..rows);
                    let rx = rng.gen_range(0..cols);
                    self.grid[ry][rx] = Value::Int(rng.gen_range(0..100));
                    self.output
                        .push(format!("STORM: Reality warp at {},{}", rx, ry));
                }
            }

            // Amnesia (1%)
            if rng.gen_bool(0.01) {
                self.stack.pop();
                self.output
                    .push("STORM: Memory lost (Stack Pop)".to_string());
            }
        }

        self.check_starvation()
    }
}
