use locus::Vec2;

// Simplified chaotic double pendulum for the TUI (using standard RK4 or simple Euler integration)
#[derive(Clone, Debug)]
pub struct DoublePendulum {
    pub origin: Vec2,
    pub r1: f64,
    pub r2: f64,
    pub m1: f64,
    pub m2: f64,
    pub a1: f64,
    pub a2: f64,
    pub a1_v: f64,
    pub a2_v: f64,
    pub g: f64,
}

impl DoublePendulum {
    pub fn new(origin: Vec2) -> Self {
        Self {
            origin,
            r1: 25.0,
            r2: 25.0,
            m1: 10.0,
            m2: 10.0,
            a1: std::f64::consts::PI / 2.0,
            a2: std::f64::consts::PI / 2.0,
            a1_v: 0.0,
            a2_v: 0.0,
            g: 1.0,
        }
    }

    pub fn p1(&self) -> Vec2 {
        Vec2::new(
            self.origin.x + self.r1 * self.a1.sin(),
            self.origin.y - self.r1 * self.a1.cos(), // Note: coordinate system y-axis is inverted
        )
    }

    pub fn p2(&self) -> Vec2 {
        let p1 = self.p1();
        Vec2::new(
            p1.x + self.r2 * self.a2.sin(),
            p1.y - self.r2 * self.a2.cos(),
        )
    }

    pub fn update(&mut self, dt: f64) {
        // Double pendulum equations of motion
        let num1 = -self.g * (2.0 * self.m1 + self.m2) * self.a1.sin();
        let num2 = -self.m2 * self.g * (self.a1 - 2.0 * self.a2).sin();
        let num3 = -2.0 * self.a2.sin() * self.m2;
        let num4 = self.a2_v * self.a2_v * self.r2 + self.a1_v * self.a1_v * self.r1 * (self.a1 - self.a2).cos();
        let den = self.r1 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 * self.a2).cos());
        let a1_a = (num1 + num2 + num3 * num4) / den;

        let num1_2 = 2.0 * (self.a1 - self.a2).sin();
        let num2_2 = self.a1_v * self.a1_v * self.r1 * (self.m1 + self.m2);
        let num3_2 = self.g * (self.m1 + self.m2) * self.a1.cos();
        let num4_2 = self.a2_v * self.a2_v * self.r2 * self.m2 * (self.a1 - self.a2).cos();
        let den_2 = self.r2 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 * self.a2).cos());
        let a2_a = (num1_2 * (num2_2 + num3_2 + num4_2)) / den_2;

        self.a1_v += a1_a * dt;
        self.a2_v += a2_a * dt;
        self.a1 += self.a1_v * dt;
        self.a2 += self.a2_v * dt;

        // Damping
        self.a1_v *= 0.999;
        self.a2_v *= 0.999;
    }
}
