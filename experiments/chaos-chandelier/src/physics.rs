use macroquad::prelude::Vec2;

#[derive(Clone, Debug)]
pub struct DoublePendulum {
    // Parameters
    pub m1: f32,
    pub l1: f32,
    pub m2: f32,
    pub l2: f32,
    pub g: f32,

    // State
    pub theta1: f32,
    pub theta2: f32,
    pub omega1: f32,
    pub omega2: f32,

    // History for trail
    pub trail: Vec<Vec2>,
}

impl DoublePendulum {
    pub fn new(m1: f32, l1: f32, m2: f32, l2: f32, theta1: f32, theta2: f32) -> Self {
        Self {
            m1,
            l1,
            m2,
            l2,
            g: 9.81,
            theta1,
            theta2,
            omega1: 0.0,
            omega2: 0.0,
            trail: Vec::with_capacity(100),
        }
    }

    pub fn step(&mut self, dt: f32) {
        // RK4 Integration
        let state = [self.theta1, self.theta2, self.omega1, self.omega2];

        let k1 = self.derivative(state);
        let k2 = self.derivative(self.add_state(state, k1, dt * 0.5));
        let k3 = self.derivative(self.add_state(state, k2, dt * 0.5));
        let k4 = self.derivative(self.add_state(state, k3, dt));

        // Update state
        self.theta1 += (dt / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]);
        self.theta2 += (dt / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]);
        self.omega1 += (dt / 6.0) * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]);
        self.omega2 += (dt / 6.0) * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]);

        // Wrap angles? No, chaos needs winding numbers sometimes, but for display we use sin/cos.
        // But for float precision over long runs, maybe wrap.
        // self.theta1 = self.theta1 % (2.0 * std::f32::consts::PI);
        // self.theta2 = self.theta2 % (2.0 * std::f32::consts::PI);
    }

    fn add_state(&self, state: [f32; 4], k: [f32; 4], scale: f32) -> [f32; 4] {
        [
            state[0] + k[0] * scale,
            state[1] + k[1] * scale,
            state[2] + k[2] * scale,
            state[3] + k[3] * scale,
        ]
    }

    fn derivative(&self, state: [f32; 4]) -> [f32; 4] {
        let t1 = state[0];
        let t2 = state[1];
        let w1 = state[2];
        let w2 = state[3];

        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;
        let g = self.g;

        // Equations of motion for Double Pendulum (Lagrangian)
        // Source: https://www.myphysicslab.com/pendulum/double-pendulum-en.html

        // Denominator term
        // den = 2*m1 + m2 - m2 * cos(2*t1 - 2*t2)
        let delta = t1 - t2;
        let den = 2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos();

        // a1 (omega1_dot)
        // num1 = -g * (2*m1 + m2) * sin(t1)
        // num2 = -m2 * g * sin(t1 - 2*t2)
        // num3 = -2 * sin(t1 - t2) * m2 * (w2^2 * l2 + w1^2 * l1 * cos(t1 - t2))
        // a1 = (num1 + num2 + num3) / (l1 * den)

        let num1 = -g * (2.0 * m1 + m2) * t1.sin();
        let num2 = -m2 * g * (t1 - 2.0 * t2).sin();
        let num3 = -2.0 * delta.sin() * m2 * (w2 * w2 * l2 + w1 * w1 * l1 * delta.cos());

        let dw1 = (num1 + num2 + num3) / (l1 * den);

        // a2 (omega2_dot)
        // num1 = 2 * sin(t1 - t2)
        // num2 = w1^2 * l1 * (m1 + m2)
        // num3 = g * (m1 + m2) * cos(t1)
        // num4 = w2^2 * l2 * m2 * cos(t1 - t2)
        // a2 = (num1 * (num2 + num3 + num4)) / (l2 * den)

        let num1_2 = 2.0 * delta.sin();
        let num2_2 = w1 * w1 * l1 * (m1 + m2);
        let num3_2 = g * (m1 + m2) * t1.cos();
        let num4_2 = w2 * w2 * l2 * m2 * delta.cos();

        let dw2 = (num1_2 * (num2_2 + num3_2 + num4_2)) / (l2 * den);

        [w1, w2, dw1, dw2]
    }

    pub fn get_pos1(&self) -> Vec2 {
        Vec2::new(self.l1 * self.theta1.sin(), self.l1 * self.theta1.cos())
    }

    pub fn get_pos2(&self) -> Vec2 {
        let p1 = self.get_pos1();
        p1 + Vec2::new(self.l2 * self.theta2.sin(), self.l2 * self.theta2.cos())
    }

    #[allow(dead_code)]
    pub fn energy(&self) -> f32 {
        let w1 = self.omega1;
        let w2 = self.omega2;
        let t1 = self.theta1;
        let t2 = self.theta2;
        let m1 = self.m1;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;
        let g = self.g;

        let ke = 0.5 * (m1 + m2) * l1 * l1 * w1 * w1
            + 0.5 * m2 * l2 * l2 * w2 * w2
            + m2 * l1 * l2 * w1 * w2 * (t1 - t2).cos();

        // Potential Energy (zero at pivot, y goes down)
        // y1 = l1 * cos(t1)
        // y2 = y1 + l2 * cos(t2)
        // PE = - (m1 g y1 + m2 g y2)
        // Wait, standard potential is mgh.
        // If y is positive down: PE = -mgh (if h is height above reference).
        // Let's use reference at pivot (y=0).
        // PE = - m1 * g * (l1 * cos(t1)) - m2 * g * (l1 * cos(t1) + l2 * cos(t2))
        //    = - (m1 + m2) * g * l1 * cos(t1) - m2 * g * l2 * cos(t2)

        let pe = -(m1 + m2) * g * l1 * t1.cos() - m2 * g * l2 * t2.cos();

        ke + pe
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_conservation() {
        let mut p = DoublePendulum::new(1.0, 1.0, 1.0, 1.0, 1.5, 2.0);
        let initial_energy = p.energy();
        let dt = 0.01;

        for _ in 0..100 {
            p.step(dt);
        }

        let final_energy = p.energy();
        // RK4 is not symplectic, so energy will drift.
        // But for short runs it should be close.
        let drift = (final_energy - initial_energy).abs();

        // 100 steps of 0.01 is 1 second.
        // With RK4, drift should be small but not zero.
        assert!(
            drift < 0.5,
            "Energy drifted too much: {} -> {}",
            initial_energy,
            final_energy
        );
    }
}
