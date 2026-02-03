#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

pub struct LorenzAttractor {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl Default for LorenzAttractor {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl LorenzAttractor {
    pub fn update(&self, p: Vector3, dt: f64) -> Vector3 {
        let dx = self.sigma * (p.y - p.x);
        let dy = p.x * (self.rho - p.z) - p.y;
        let dz = p.x * p.y - self.beta * p.z;

        Vector3::new(p.x + dx * dt, p.y + dy * dt, p.z + dz * dt)
    }
    pub fn name(&self) -> &'static str {
        "Lorenz"
    }
}

pub struct RosslerAttractor {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Default for RosslerAttractor {
    fn default() -> Self {
        Self {
            a: 0.2,
            b: 0.2,
            c: 5.7,
        }
    }
}

impl RosslerAttractor {
    pub fn update(&self, p: Vector3, dt: f64) -> Vector3 {
        let dx = -p.y - p.z;
        let dy = p.x + self.a * p.y;
        let dz = self.b + p.z * (p.x - self.c);

        Vector3::new(p.x + dx * dt, p.y + dy * dt, p.z + dz * dt)
    }
    pub fn name(&self) -> &'static str {
        "Rossler"
    }
}

pub struct AizawaAttractor {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Default for AizawaAttractor {
    fn default() -> Self {
        Self {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1,
        }
    }
}

impl AizawaAttractor {
    pub fn update(&self, p: Vector3, dt: f64) -> Vector3 {
        let dx = (p.z - self.b) * p.x - self.d * p.y;
        let dy = self.d * p.x + (p.z - self.b) * p.y;
        let dz = self.c + self.a * p.z
            - p.z.powi(3) / 3.0
            - (p.x.powi(2) + p.y.powi(2)) * (1.0 + self.e * p.z)
            + self.f * p.z * p.x.powi(3);

        Vector3::new(p.x + dx * dt, p.y + dy * dt, p.z + dz * dt)
    }
    pub fn name(&self) -> &'static str {
        "Aizawa"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_update() {
        let lorenz = LorenzAttractor::default();
        let p = Vector3::new(1.0, 1.0, 1.0);
        let next = lorenz.update(p, 0.01);

        // dx = 10(1-1) = 0 -> x stays 1.0
        // dy = 1(28-1) - 1 = 26 -> y becomes 1.0 + 0.26 = 1.26
        // dz = 1*1 - 2.66*1 = -1.66 -> z becomes 1.0 - 0.0166 = 0.9833

        assert!((next.x - 1.0).abs() < 0.001);
        assert!((next.y - 1.26).abs() < 0.001);
    }
}
