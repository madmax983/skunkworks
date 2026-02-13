use crate::physics::PhysicsWorld;

#[derive(Debug, Clone)]
pub struct PluckEvent {
    pub integrator_idx: usize,
    pub note: u8,
    pub velocity: f32,
}

pub struct MusicBox {
    last_angles: Vec<f32>,
    tooth_spacing: f32, // radians per tooth
}

impl MusicBox {
    pub fn new(num_integrators: usize) -> Self {
        Self {
            last_angles: vec![0.0; num_integrators],
            tooth_spacing: std::f32::consts::PI / 6.0, // 30 degrees
        }
    }

    pub fn update(&mut self, world: &PhysicsWorld) -> Vec<PluckEvent> {
        let mut events = Vec::new();

        // Resize if needed (in case integrators added dynamically, though unlikely)
        if self.last_angles.len() < world.integrators.len() {
            self.last_angles.resize(world.integrators.len(), 0.0);
        }

        for (i, integrator) in world.integrators.iter().enumerate() {
            if let Some(body) = world.rigid_body_set.get(integrator.output_handle) {
                let angle = body.rotation().angle();
                let last_angle = self.last_angles[i];

                // Check if crossed a multiple of tooth_spacing
                let steps_prev = (last_angle / self.tooth_spacing).floor() as i32;
                let steps_curr = (angle / self.tooth_spacing).floor() as i32;

                if steps_curr != steps_prev {
                    // Trigger pluck
                    let velocity = body.angvel().abs();
                    // Map index to note
                    let note = 60 + (i as u8) * 5;
                    events.push(PluckEvent {
                        integrator_idx: i,
                        note,
                        velocity,
                    });
                }
                self.last_angles[i] = angle;
            }
        }
        events
    }
}
