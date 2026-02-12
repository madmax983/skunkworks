use crate::physics::PhysicsWorld;
use nalgebra::Vector2;
use quipu::{Cord, Knot};
use rapier2d::prelude::*;

pub struct QuipuMachine {
    pub input_cord: Cord,
    pub output_cord: Cord,
    pub knot_bodies: Vec<(Knot, RigidBodyHandle)>,
    pub integrator_value: f64,
    pub wheel_angle: f32,

    next_cluster_idx: Option<usize>,
    time_since_last_spawn: f32,
}

impl QuipuMachine {
    pub fn new(input_val: u64) -> Self {
        let input_cord = Cord::from(input_val);
        let next_cluster_idx = if input_cord.clusters.is_empty() {
            None
        } else {
            Some(input_cord.clusters.len() - 1)
        };

        Self {
            input_cord,
            output_cord: Cord::new(),
            knot_bodies: Vec::new(),
            integrator_value: 0.0,
            wheel_angle: 0.0,
            next_cluster_idx,
            time_since_last_spawn: 0.0,
        }
    }

    pub fn tick(&mut self, physics: &mut PhysicsWorld, dt: f32) {
        // 1. Spawn Knots
        self.time_since_last_spawn += dt;
        if self.time_since_last_spawn > 1.0 {
            if let Some(cluster_idx) = self.next_cluster_idx {
                let cluster = &self.input_cord.clusters[cluster_idx];

                for (i, knot) in cluster.iter().enumerate() {
                    let y_start = 20.0 + (i as f32 * 3.0); // Start at Top
                    let radius = match knot {
                        Knot::Simple => 1.0,
                        Knot::Long(v) => 1.0 + (*v as f32 * 0.2),
                        Knot::FigureEight => 1.5,
                    };

                    // Spawn offset by 1.0 so they hit the sensor off-center, generating lateral force
                    let handle = physics.create_knot_body(1.0, y_start, radius);
                    // No need to force velocity here as create_knot_body sets it correctly now
                    self.knot_bodies.push((*knot, handle));
                }

                if cluster_idx == 0 {
                    self.next_cluster_idx = None;
                } else {
                    self.next_cluster_idx = Some(cluster_idx - 1);
                }
                self.time_since_last_spawn = 0.0;
            }
        }

        // 2. Read Sensor
        if let Some(sensor) = physics.rigid_body_set.get_mut(physics.sensor_handle) {
            let x = sensor.translation().x;
            let v = sensor.linvel().x;

            // Constrain Y to 0.0 (prevent being pushed down by knots)
            sensor.set_translation(Vector2::new(x, 0.0), true);

            // Integrate
            self.integrator_value += x.abs() as f64 * dt as f64 * 0.5;
            self.wheel_angle += x * dt;

            // Spring force
            let k = 20.0;
            let c = 2.0;
            let force = -k * x - c * v;
            sensor.apply_impulse(Vector2::new(force * dt, 0.0), true);
        }

        // 3. Update Output
        // Accumulate until we reach a "Knot Value"
        // 10.0 integrator units = 1 Simple Knot
        while self.integrator_value >= 10.0 {
            self.integrator_value -= 10.0;
            if self.output_cord.clusters.is_empty() {
                self.output_cord.clusters.push(Vec::new());
            }
            self.output_cord.clusters[0].push(Knot::Simple);
        }

        // 4. Cleanup
        let mut to_remove = Vec::new();
        self.knot_bodies.retain(|(_, handle)| {
            if let Some(body) = physics.rigid_body_set.get(*handle) {
                if body.translation().y < -20.0 { // Below bottom
                    to_remove.push(*handle);
                    return false;
                }
            }
            true
        });

        for handle in to_remove {
            physics.remove_body(handle);
        }
    }
}
