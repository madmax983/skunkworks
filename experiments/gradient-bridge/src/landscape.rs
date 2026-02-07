use macroquad::prelude::*;
use std::f32::consts::PI;

pub trait ObjectiveFunction {
    fn name(&self) -> &str;
    /// The value of the function at (x, y).
    /// Higher values represent "hills" (light), lower values represent "valleys" (water).
    fn value(&self, x: f32, y: f32) -> f32;

    /// The gradient vector pointing in the direction of steepest ascent.
    fn gradient(&self, x: f32, y: f32) -> Vec2 {
        let h = 0.001;
        let dx = (self.value(x + h, y) - self.value(x - h, y)) / (2.0 * h);
        let dy = (self.value(x, y + h) - self.value(x, y - h)) / (2.0 * h);
        Vec2::new(dx, dy)
    }
}

pub struct GaussianHills;
impl ObjectiveFunction for GaussianHills {
    fn name(&self) -> &str { "Gaussian Hills" }
    fn value(&self, x: f32, y: f32) -> f32 {
        // A nice mix of peaks and valleys
        let v1 = (-(x*x + y*y) * 0.1).exp() * 5.0; // Central peak
        let v2 = (x * 0.8).sin() * (y * 0.8).cos() * 2.0; // Waves
        v1 + v2
    }
}

pub struct Rastrigin;
impl ObjectiveFunction for Rastrigin {
    fn name(&self) -> &str { "Rastrigin" }
    fn value(&self, x: f32, y: f32) -> f32 {
        let a = 10.0;
        let val = a * 2.0 + (x.powi(2) - a * (2.0 * PI * x).cos())
                          + (y.powi(2) - a * (2.0 * PI * y).cos());
        // Invert it so "deep valleys" are low values
        -val * 0.2
    }
}

pub struct Rosenbrock;
impl ObjectiveFunction for Rosenbrock {
    fn name(&self) -> &str { "Rosenbrock" }
    fn value(&self, x: f32, y: f32) -> f32 {
        let a = 1.0;
        let b = 100.0;
        let val = (a - x).powi(2) + b * (y - x.powi(2)).powi(2);
        -val * 0.001 // Scale down significantly
    }
}

pub struct Ackley;
impl ObjectiveFunction for Ackley {
    fn name(&self) -> &str { "Ackley" }
    fn value(&self, x: f32, y: f32) -> f32 {
        let term1 = -20.0 * (-0.2 * (0.5 * (x.powi(2) + y.powi(2))).sqrt()).exp();
        let term2 = -(0.5 * ((2.0 * PI * x).cos() + (2.0 * PI * y).cos())).exp();
        let val = term1 + term2 + 20.0 + std::f32::consts::E;
        -val // Invert for consistency
    }
}

pub struct EggHolder;
impl ObjectiveFunction for EggHolder {
    fn name(&self) -> &str { "Egg Holder" }
    fn value(&self, x: f32, y: f32) -> f32 {
        // Typically defined on larger range [-512, 512], scale input
        let sx = x * 5.0; // Scaled for our -10..10 view
        let sy = y * 5.0;
        let val = -(sy + 47.0) * ((sx/2.0 + sx + 47.0).sin()) - sx * ((sx - (sy + 47.0)).sin());
        val * 0.01
    }
}
