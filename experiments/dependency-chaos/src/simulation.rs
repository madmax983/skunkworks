use crate::physics::{State, DoublePendulumParams, rk4_step};
use rayon::prelude::*;
use rand::Rng;

pub struct Cloud {
    pub states: Vec<State>,
    pub params: DoublePendulumParams,
}

impl Cloud {
    pub fn new(count: usize, center: State, spread: f32, params: DoublePendulumParams) -> Self {
        let mut states = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            states.push(State {
                theta1: center.theta1 + rng.gen_range(-spread..spread),
                theta2: center.theta2 + rng.gen_range(-spread..spread),
                omega1: center.omega1 + rng.gen_range(-spread..spread),
                omega2: center.omega2 + rng.gen_range(-spread..spread),
            });
        }

        Self {
            states,
            params,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.states.par_iter_mut().for_each(|state| {
            *state = rk4_step(state, &self.params, dt);
        });
    }

    pub fn kick(&mut self, force: f32) {
         self.states.par_iter_mut().for_each(|state| {
            let mut rng = rand::thread_rng();
            state.omega1 += rng.gen_range(-force..force);
            state.omega2 += rng.gen_range(-force..force);
        });
    }

    pub fn reset(&mut self, center: State, spread: f32) {
        self.states.par_iter_mut().for_each(|state| {
             let mut rng = rand::thread_rng();
             state.theta1 = center.theta1 + rng.gen_range(-spread..spread);
             state.theta2 = center.theta2 + rng.gen_range(-spread..spread);
             state.omega1 = center.omega1 + rng.gen_range(-spread..spread);
             state.omega2 = center.omega2 + rng.gen_range(-spread..spread);
        });
    }
}
