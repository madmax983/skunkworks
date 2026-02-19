use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct RigidBody {
    pub angle: f32,
    pub velocity: f32,
    pub inertia: f32,
    pub friction: f32,
}

impl RigidBody {
    pub fn new(inertia: f32, friction: f32) -> Self {
        Self {
            angle: 0.0,
            velocity: 0.0,
            inertia,
            friction,
        }
    }

    pub fn integrate(&mut self, torque: f32, dt: f32) {
        let acceleration = torque / self.inertia;
        self.velocity += acceleration * dt;

        let drag_torque = -self.velocity * self.friction;
        self.velocity += (drag_torque / self.inertia) * dt;

        self.angle += self.velocity * dt;
    }

    pub fn apply_impulse(&mut self, impulse: f32) {
        self.velocity += impulse / self.inertia;
    }
}

pub struct Simulation {
    pub crown: RigidBody,
    pub verge: RigidBody,
    pub teeth: usize,
    pub voltage: f32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            crown: RigidBody::new(5.0, 0.5),
            verge: RigidBody::new(0.5, 0.8),
            teeth: 15,
            voltage: 800.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.crown.integrate(self.voltage, dt);

        // Weak centering spring to simulate gravity/hairspring (optional but helps stability)
        let spring_k = 50.0;
        let spring_torque = -self.verge.angle * spring_k;
        self.verge.integrate(spring_torque, dt);

        let step_angle = 2.0 * PI / self.teeth as f32;

        // Escapement Geometry:
        // Pallets block the crown wheel unless they swing out of the way.
        // Top Pallet: Blocks when verge is in "middle" range. Escapes when angle > escape_angle.
        // Bottom Pallet: Blocks when verge is in "middle" range. Escapes when angle < -escape_angle.

        let escape_angle = 0.6; // Radians
        let collision_depth = 0.2; // Radians
        let kick_force = 60000.0;

        // Top Pallet Interaction (Phase PI/2)
        // Active zone: [-0.1, escape_angle]
        // If verge is in this zone, it catches the tooth.
        // Tooth pushes verge POSITIVE (towards escape).
        if self.verge.angle > -0.2 && self.verge.angle < escape_angle {
             let phase = PI / 2.0;
             if Self::solve_collision(&mut self.crown, &mut self.verge, phase, step_angle, kick_force, dt, 1.0, collision_depth) {
                 // Collision occurred
             }
        }

        // Bottom Pallet Interaction (Phase 3PI/2)
        // Active zone: [-escape_angle, 0.1]
        // Tooth pushes verge NEGATIVE (towards -escape).
        if self.verge.angle < 0.2 && self.verge.angle > -escape_angle {
             let phase = 3.0 * PI / 2.0;
             if Self::solve_collision(&mut self.crown, &mut self.verge, phase, step_angle, kick_force, dt, -1.0, collision_depth) {
                 // Collision occurred
             }
        }
    }

    // Returns true if collision occurred
    fn solve_collision(
        crown: &mut RigidBody,
        verge: &mut RigidBody,
        phase: f32,
        step: f32,
        force_k: f32,
        dt: f32,
        verge_direction: f32,
        depth: f32
    ) -> bool {
        let rel = (crown.angle - phase).rem_euclid(step);

        if rel < depth {
            let penetration = rel;
            let force = penetration * force_k;

            // Crown pushed back
            crown.apply_impulse(-force * dt);

            // Verge pushed away
            verge.apply_impulse(force * verge_direction * dt);
            return true;
        }
        false
    }
}
