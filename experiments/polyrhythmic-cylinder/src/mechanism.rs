use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct PluckEvent {
    pub frequency: f32,
    pub volume: f32,
    pub track_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Pin {
    pub angle: f32, // Radians relative to cylinder 0
    pub track_index: usize,
}

#[derive(Debug)]
pub struct Mainspring {
    pub tension: f32, // 0.0 to 1.0
    pub torque_constant: f32,
    pub unwind_rate: f32,
}

#[derive(Debug)]
pub struct Governor {
    pub drag_coefficient: f32,
}

#[derive(Debug)]
pub struct Cylinder {
    pub radius: f32,
    pub angular_velocity: f32, // Radians per second
    pub angle: f32, // Current rotation of cylinder
    pub pins: Vec<Pin>,
    pub mainspring: Mainspring,
    pub governor: Governor,
    pub moment_of_inertia: f32,
}

#[derive(Debug)]
pub struct Tooth {
    pub frequency: f32,
    pub track_index: usize,
    pub vibration_amplitude: f32,
}

#[derive(Debug)]
pub struct Comb {
    pub teeth: Vec<Tooth>,
}

impl Cylinder {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            angular_velocity: 0.0,
            angle: 0.0,
            pins: Vec::new(),
            mainspring: Mainspring {
                tension: 1.0,
                torque_constant: 50.0,
                unwind_rate: 0.001,
            },
            governor: Governor {
                drag_coefficient: 2.0,
            },
            moment_of_inertia: 10.0,
        }
    }

    pub fn add_pin(&mut self, angle: f32, track_index: usize) {
        self.pins.push(Pin { angle, track_index });
    }

    pub fn wind(&mut self, amount: f32) {
        self.mainspring.tension = (self.mainspring.tension + amount).min(1.0);
    }

    pub fn tick(&mut self, dt: f32, comb: &mut Comb) -> Vec<PluckEvent> {
        // Physics update
        let torque = self.mainspring.tension * self.mainspring.torque_constant;
        let drag = self.governor.drag_coefficient * self.angular_velocity.powi(2) * self.angular_velocity.signum();

        let net_torque = torque - drag;
        let alpha = net_torque / self.moment_of_inertia;

        self.angular_velocity += alpha * dt;
        // Friction/stopping
        if self.angular_velocity < 0.0 && torque > 0.0 {
             // Prevent reverse unless we want it?
             // Music boxes have ratchets.
             self.angular_velocity = 0.0;
        }

        // Unwind spring
        if self.angular_velocity > 0.0 {
            self.mainspring.tension -= self.angular_velocity.abs() * dt * self.mainspring.unwind_rate;
            if self.mainspring.tension < 0.0 { self.mainspring.tension = 0.0; }
        }

        let mut events = Vec::new();
        let angle_prev = self.angle;
        let angle_curr = self.angle + self.angular_velocity * dt;

        // Update state
        self.angle = angle_curr;

        // Check for collisions
        // The comb is assumed to be at World Angle 0.
        // A pin hits the comb when its world angle (pin.angle + cylinder.angle) crosses a multiple of 2PI.

        for pin in &self.pins {
            // Calculate "turns" before and after tick
            // We use (angle + pin_angle) / 2PI
            // We subtract a small epsilon to ensure we don't double trigger on exact boundaries if we were careful,
            // but floats are messy.
            // Let's use the floor method.

            let two_pi = 2.0 * PI;

            // Normalize inputs to be safe against huge floats over long runtime?
            // For a music box, angles might grow. But f32 precision drops.
            // We should probably normalize self.angle to 0..2PI, but then we lose the "crossing" info
            // unless we handle the wrap carefully.
            // Let's rely on the difference.

            let pos_prev = angle_prev + pin.angle;
            let pos_curr = angle_curr + pin.angle;

            // We shift by PI to move the wrap point away from 0?
            // No, we want to detect crossing 0 (which is k * 2PI).

            // We want to know if there is an integer k such that pos_prev < k * 2PI <= pos_curr.
            // equivalent to floor(pos_prev / 2PI) < floor(pos_curr / 2PI).

            // Assumption: angular_velocity is positive.
            if pos_curr > pos_prev {
                 let turn_prev = (pos_prev / two_pi).floor() as i32;
                 let turn_curr = (pos_curr / two_pi).floor() as i32;

                 if turn_curr > turn_prev {
                     // Hit!
                     // Find the corresponding tooth
                     if let Some(tooth) = comb.teeth.iter_mut().find(|t| t.track_index == pin.track_index) {
                         events.push(PluckEvent {
                             frequency: tooth.frequency,
                             volume: 1.0,
                             track_index: pin.track_index,
                         });
                         // Visual feedback
                         tooth.vibration_amplitude = 1.0;
                     }
                 }
            }
        }

        // Decay vibration
        for tooth in &mut comb.teeth {
            tooth.vibration_amplitude *= 0.95_f32.powf(dt * 60.0); // Approx frame independent decay
        }

        events
    }
}

impl Comb {
    pub fn new(teeth_count: usize) -> Self {
        let mut teeth = Vec::new();
        for i in 0..teeth_count {
            // Pentatonic scale or something pleasant
            let base_freq = 220.0;
            let freq = base_freq * (2.0_f32).powf(i as f32 / 12.0); // Chromatic for now

            teeth.push(Tooth {
                frequency: freq,
                track_index: i,
                vibration_amplitude: 0.0,
            });
        }
        Self { teeth }
    }
}
