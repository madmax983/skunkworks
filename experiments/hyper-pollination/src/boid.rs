use crate::math::Vec4D;
use macroquad::color::Color;
use ::rand::Rng;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f32,
    pub max_force: f32,
    pub view_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub pollination_weight: f32, // Attraction to plants
    pub color: Color,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.02..0.05),
            max_force: rng.gen_range(0.001..0.005),
            view_radius: rng.gen_range(0.5..1.0),
            separation_weight: rng.gen_range(1.2..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            pollination_weight: rng.gen_range(0.5..1.5),
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid4D {
    pub position: Vec4D,
    pub velocity: Vec4D,
    pub acceleration: Vec4D,
    pub dna: Dna,
}

impl Boid4D {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let pos = Vec4D::new(
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
            rng.gen_range(-2.0..2.0),
        );
        let vel = Vec4D::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
        .scale(0.03);

        Self {
            position: pos,
            velocity: vel,
            acceleration: Vec4D::zero(),
            dna: Dna::random(),
        }
    }

    pub fn update(&mut self, bounds: Vec4D) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec4D::zero();

        // Bounce off walls (Hypercube Bounds)
        // X
        if self.position.x > bounds.x {
            self.position.x = bounds.x;
            self.velocity.x *= -1.0;
        } else if self.position.x < -bounds.x {
            self.position.x = -bounds.x;
            self.velocity.x *= -1.0;
        }
        // Y
        if self.position.y > bounds.y {
            self.position.y = bounds.y;
            self.velocity.y *= -1.0;
        } else if self.position.y < -bounds.y {
            self.position.y = -bounds.y;
            self.velocity.y *= -1.0;
        }
        // Z
        if self.position.z > bounds.z {
            self.position.z = bounds.z;
            self.velocity.z *= -1.0;
        } else if self.position.z < -bounds.z {
            self.position.z = -bounds.z;
            self.velocity.z *= -1.0;
        }
        // W
        if self.position.w > bounds.w {
            self.position.w = bounds.w;
            self.velocity.w *= -1.0;
        } else if self.position.w < -bounds.w {
            self.position.w = -bounds.w;
            self.velocity.w *= -1.0;
        }
    }

    pub fn flock(&mut self, boids: &[Boid4D], target: Option<Vec4D>) {
        let separation = self.separation(boids).scale(self.dna.separation_weight);
        let alignment = self.alignment(boids).scale(self.dna.alignment_weight);
        let cohesion = self.cohesion(boids).scale(self.dna.cohesion_weight);

        self.acceleration += separation;
        self.acceleration += alignment;
        self.acceleration += cohesion;

        if let Some(t) = target {
            let pollination = self.seek(t).scale(self.dna.pollination_weight);
            self.acceleration += pollination;
        }
    }

    fn separation(&self, boids: &[Boid4D]) -> Vec4D {
        let mut steer = Vec4D::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                let diff = (self.position - other.position).normalize();
                let diff = diff.scale(1.0 / d_sq.sqrt());
                steer += diff;
                count += 1;
            }
        }
        if count > 0 {
            steer = steer.scale(1.0 / count as f32);
            if steer.length_squared() > 0.0 {
                steer = steer.normalize().scale(self.dna.max_speed);
                steer = steer - self.velocity;
                steer = steer.limit(self.dna.max_force);
            }
        }
        steer
    }

    fn alignment(&self, boids: &[Boid4D]) -> Vec4D {
        let mut sum = Vec4D::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                sum += other.velocity;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f32);
            sum = sum.normalize().scale(self.dna.max_speed);
            let steer = sum - self.velocity;
            return steer.limit(self.dna.max_force);
        }
        Vec4D::zero()
    }

    fn cohesion(&self, boids: &[Boid4D]) -> Vec4D {
        let mut sum = Vec4D::zero();
        let mut count = 0;
        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < self.dna.view_radius * self.dna.view_radius {
                sum += other.position;
                count += 1;
            }
        }
        if count > 0 {
            sum = sum.scale(1.0 / count as f32);
            return self.seek(sum);
        }
        Vec4D::zero()
    }

    fn seek(&self, target: Vec4D) -> Vec4D {
        let desired = (target - self.position)
            .normalize()
            .scale(self.dna.max_speed);
        let steer = desired - self.velocity;
        steer.limit(self.dna.max_force)
    }

    // Color mixing logic
    pub fn pollinate(&mut self, plant_color: Color) {
        // Blend towards plant color (20%)
        self.dna.color.r = self.dna.color.r * 0.8 + plant_color.r * 0.2;
        self.dna.color.g = self.dna.color.g * 0.8 + plant_color.g * 0.2;
        self.dna.color.b = self.dna.color.b * 0.8 + plant_color.b * 0.2;
    }
}
