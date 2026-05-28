use origami::{OrigamiMesh, MiuraParams, Orientation, generate_miura_mesh};
use flocking::{FlockingParams, compute_force};
use locus::Vec2;

pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct OrigamiFlockHybrid {
    pub mesh: OrigamiMesh,
    pub params: MiuraParams,
    pub boids: Vec<Boid>,
    pub grid_width: usize,
    pub grid_height: usize,
}

impl OrigamiFlockHybrid {
    pub fn new(width: usize, height: usize, swarm_size: usize) -> Self {
        let params = MiuraParams {
            a: 10.0,
            b: 10.0,
            gamma: std::f32::consts::PI / 4.0,
            orientation: Orientation::Horizontal,
        };
        let mesh = generate_miura_mesh(params, (width, height), 0.5);

        let mut boids = Vec::with_capacity(swarm_size);
        for _ in 0..swarm_size {
            boids.push(Boid {
                position: Vec2::new(
                    (rand::random::<f64>() - 0.5) * (width as f64 * 10.0),
                    (rand::random::<f64>() - 0.5) * (height as f64 * 10.0),
                ),
                velocity: Vec2::new(
                    rand::random::<f64>() - 0.5,
                    rand::random::<f64>() - 0.5,
                ).normalize() * 10.0,
            });
        }

        Self { mesh, params, boids, grid_width: width + 1, grid_height: height + 1 }
    }

    pub fn update(&mut self, dt: f64) {
        let flocking_params = FlockingParams {
            view_radius: 50.0,
            separation_radius: 20.0,
            max_speed: 30.0,
            max_force: 5.0,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        // Update swarm
        for i in 0..self.boids.len() {
            let force = compute_force(&positions, &velocities, i, &flocking_params);
            self.boids[i].velocity += force * dt;
            if self.boids[i].velocity.magnitude() > flocking_params.max_speed {
                self.boids[i].velocity = self.boids[i].velocity.normalize() * flocking_params.max_speed;
            }
            let vel = self.boids[i].velocity; self.boids[i].position += vel * dt;

            // simple bounds
            let half_w = (self.grid_width as f64 * 10.0) / 2.0;
            let half_h = (self.grid_height as f64 * 10.0) / 2.0;

            if self.boids[i].position.x > half_w { self.boids[i].position.x = -half_w; }
            if self.boids[i].position.x < -half_w { self.boids[i].position.x = half_w; }
            if self.boids[i].position.y > half_h { self.boids[i].position.y = -half_h; }
            if self.boids[i].position.y < -half_h { self.boids[i].position.y = half_h; }
        }

        // Swarm actively deforms the soft-body paper mesh
        for boid in &self.boids {
            let half_w = (self.grid_width as f64 * 10.0) / 2.0;
            let half_h = (self.grid_height as f64 * 10.0) / 2.0;

            let x_norm = (boid.position.x + half_w) / (half_w * 2.0);
            let y_norm = (boid.position.y + half_h) / (half_h * 2.0);

            let x_grid = (x_norm * (self.grid_width - 1) as f64).clamp(0.0, (self.grid_width - 1) as f64) as usize;
            let y_grid = (y_norm * (self.grid_height - 1) as f64).clamp(0.0, (self.grid_height - 1) as f64) as usize;

            let idx = y_grid * self.grid_width + x_grid;
            if idx < self.mesh.vertices.len() {
                // The presence of a boid pulls the paper "up" (z axis)
                // creating a physical dent/pull in the mesh
                self.mesh.vertices[idx].pos.z += 10.0 * dt as f32;
            }
        }

        // decay mesh back to 0.0 Z
        for v in &mut self.mesh.vertices {
             v.pos.z *= 0.99; // decay
        }
    }
}
