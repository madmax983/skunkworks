use crate::opcode::OpCode;
use crate::prelude::Value;
use crate::vm::ChimeraVM;
use crate::vm::*;
use rand::Rng;

impl ChimeraVM {
    #[cfg(feature = "nova")]
    pub(crate) fn process_environment(&mut self) {
        let (cy, cx) = self.context_loc;
        self.waste_grid[cy][cx] += 10;

        nova_diffusion::diffuse_hormones(self);
        nova_diffusion::diffuse_waste(self);
        nova_diffusion::diffuse_light(self);
        nova_diffusion::diffuse_mutagen(self);
        nova_diffusion::diffuse_entropy(self);
        nova_scent::process_scents(self);

        nova_fluid::process_hydra_components(self);
        nova_fluid::process_fluid(self);
        nova_fluid::process_sensors(self);

        for row in self.hormone_grid.iter_mut() {
            let row: &mut Vec<[i64; 3]> = row;
            for cell in row.iter_mut() {
                let cell_arr: &mut [i64; 3] = cell;
                for val in cell_arr.iter_mut() {
                    let v: &mut i64 = val;
                    if *v > 0 {
                        *v -= 1;
                    }
                }
            }
        }

        if self.waste_grid[cy][cx] > 100 {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.05) {
                self.output
                    .push(format!("MUTATION: TOXICITY at {},{}", cx, cy));
                self.mutate();
            }
        }

        if self.mutagen_grid[cy][cx] > 50 {
            let mut rng = rand::thread_rng();
            // Higher probability for mutagen (10%)
            if rng.gen_bool(0.10) {
                self.output
                    .push(format!("MUTATION: RADIATION at {},{}", cx, cy));
                self.mutate();
            }
        }

        // Reality Decay (Entropy)
        if self.entropy_grid[cy][cx] > 50 {
            let mut rng = rand::thread_rng();
            // 20% chance of Glitch per tick if high entropy
            if rng.gen_bool(0.20) {
                self.output.push(format!("REALITY DECAY at {},{}", cx, cy));
                // Simulate Glitch(1)
                self.stack.push(Value::Int(1)); // Severity 1
                if nova::exec_nova_op(self, OpCode::Glitch, &[]).is_some() {
                    // Jump occurred (unlikely for Glitch but possible if we extended it)
                }
            }
        }
    }
}
