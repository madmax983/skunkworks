use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptimizerType {
    SGD,
    Momentum,
    Adam,
}

impl OptimizerType {
    pub fn next(&self) -> Self {
        match self {
            OptimizerType::SGD => OptimizerType::Momentum,
            OptimizerType::Momentum => OptimizerType::Adam,
            OptimizerType::Adam => OptimizerType::SGD,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            OptimizerType::SGD => "SGD",
            OptimizerType::Momentum => "Momentum",
            OptimizerType::Adam => "Adam",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            OptimizerType::SGD => GREEN,
            OptimizerType::Momentum => BLUE,
            OptimizerType::Adam => MAGENTA,
        }
    }
}

#[derive(Clone, Debug)]
pub enum OptimizerState {
    SGD,
    Momentum { velocity: Vec2 },
    Adam { m: Vec2, v: Vec2, t: i32 },
}

impl OptimizerState {
    pub fn new(opt_type: OptimizerType) -> Self {
        match opt_type {
            OptimizerType::SGD => OptimizerState::SGD,
            OptimizerType::Momentum => OptimizerState::Momentum {
                velocity: Vec2::ZERO,
            },
            OptimizerType::Adam => OptimizerState::Adam {
                m: Vec2::ZERO,
                v: Vec2::ZERO,
                t: 1,
            },
        }
    }

    /// Takes the gradient and learning rate, returns the step vector.
    /// The step vector points in the direction of the gradient (Ascent).
    /// To minimize, subtract this step. To maximize, add it.
    pub fn compute_step(&mut self, gradient: Vec2, learning_rate: f32) -> Vec2 {
        match self {
            OptimizerState::SGD => gradient * learning_rate,
            OptimizerState::Momentum { velocity } => {
                let momentum_factor = 0.9;
                *velocity = *velocity * momentum_factor + gradient * learning_rate;
                *velocity
            }
            OptimizerState::Adam { m, v, t } => {
                let beta1 = 0.9;
                let beta2 = 0.999;
                let epsilon = 1e-8;

                // Update biased first moment estimate
                *m = *m * beta1 + gradient * (1.0 - beta1);

                // Update biased second raw moment estimate (element-wise square)
                let g_sq = Vec2::new(gradient.x * gradient.x, gradient.y * gradient.y);
                *v = *v * beta2 + g_sq * (1.0 - beta2);

                // Compute bias-corrected first moment estimate
                let m_hat = *m / (1.0 - beta1.powi(*t));

                // Compute bias-corrected second raw moment estimate
                let v_hat = *v / (1.0 - beta2.powi(*t));

                *t += 1;

                // Compute the update
                let dx = m_hat.x / (v_hat.x.sqrt() + epsilon);
                let dy = m_hat.y / (v_hat.y.sqrt() + epsilon);

                Vec2::new(dx, dy) * learning_rate
            }
        }
    }
}
