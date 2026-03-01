use num_complex::Complex;

pub type Point = Complex<f32>;

/// Calculates the hyperbolic distance between two points in the Poincaré disk.
pub fn hyperbolic_distance(a: Point, b: Point) -> f32 {
    let num = (a - b).norm_sqr();
    let den = (1.0 - a.norm_sqr()) * (1.0 - b.norm_sqr());
    let delta = 1.0 + 2.0 * num / den;
    delta.acosh()
}

/// Moves point `u` towards `v` by a hyperbolic distance `dist`.
/// If `dist` is negative, moves away.
pub fn move_towards(u: Point, v: Point, dist: f32) -> Point {
    if dist.abs() < 1e-6 {
        return u;
    }

    // Mobius transform to bring u to origin
    // M_u(z) = (z - u) / (1 - conj(u) * z)
    // We want to find v' = M_u(v)
    let one = Complex::new(1.0, 0.0);
    let num = v - u;
    let den = one - u.conj() * v;
    let v_prime = num / den;

    // v' is now at some distance from origin.
    // The direction towards v is simply the phase of v'.
    let direction = if v_prime.norm() > 1e-6 {
        v_prime / v_prime.norm()
    } else {
        // Points are identical, pick random direction or zero
        return u;
    };

    // Calculate the new point u'_new at distance `dist` from origin along that direction.
    // distance d = 2 * atanh(|z|)
    // |z| = tanh(d / 2)
    let new_mag = (dist / 2.0).tanh();
    let u_prime_new = direction * new_mag;

    // Transform back: M_-u(z) = (z + u) / (1 + conj(u) * z)
    let num_back = u_prime_new + u;
    let den_back = one + u.conj() * u_prime_new;

    num_back / den_back
}

#[derive(Clone, Debug)]
pub enum Constraint {
    Distance {
        p1: usize,
        p2: usize,
        target_dist: f32,
        stiffness: f32,
    },
    Actuator {
        p1: usize,
        p2: usize,
        _base_dist: f32, // Renamed to suppress warning
        factor: f32,     // 0.0 (Contract) to 1.0 (Relax)
        min_dist: f32,
        max_dist: f32,
        stiffness: f32,
    },
    Pin {
        p: usize,
        pos: Point,
    },
}

pub struct PbdSystem {
    pub points: Vec<Point>,
    pub velocities: Vec<Point>, // Not really used in pure PBD, but maybe for damping
    pub constraints: Vec<Constraint>,
    pub weights: Vec<f32>, // 1/mass
}

impl PbdSystem {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            velocities: Vec::new(),
            constraints: Vec::new(),
            weights: Vec::new(),
        }
    }

    pub fn add_point(&mut self, pos: Point, mass: f32) -> usize {
        self.points.push(pos);
        self.velocities.push(Complex::new(0.0, 0.0));
        self.weights.push(if mass > 0.0 { 1.0 / mass } else { 0.0 });
        self.points.len() - 1
    }

    pub fn add_distance_constraint(
        &mut self,
        p1: usize,
        p2: usize,
        dist: f32,
        stiffness: f32,
    ) -> usize {
        let idx = self.constraints.len();
        self.constraints.push(Constraint::Distance {
            p1,
            p2,
            target_dist: dist,
            stiffness,
        });
        idx
    }

    pub fn add_actuator_constraint(
        &mut self,
        p1: usize,
        p2: usize,
        min: f32,
        max: f32,
        stiffness: f32,
    ) -> usize {
        let idx = self.constraints.len();
        self.constraints.push(Constraint::Actuator {
            p1,
            p2,
            _base_dist: (min + max) * 0.5,
            factor: 0.5,
            min_dist: min,
            max_dist: max,
            stiffness,
        });
        idx
    }

    pub fn add_pin_constraint(&mut self, p: usize, pos: Point) {
        self.constraints.push(Constraint::Pin { p, pos });
    }

    pub fn step(&mut self, _dt: f32, iterations: usize) {
        // Basic PBD loop
        // 1. Prediction (Skipped for now, assuming velocity is implicitly handled by projection)

        // 2. Solve Constraints
        for _ in 0..iterations {
            for i in 0..self.constraints.len() {
                let constraint = self.constraints[i].clone(); // Clone to avoid borrow issues
                match constraint {
                    Constraint::Distance {
                        p1,
                        p2,
                        target_dist,
                        stiffness,
                    } => {
                        self.solve_distance(p1, p2, target_dist, stiffness);
                    }
                    Constraint::Actuator {
                        p1,
                        p2,
                        factor,
                        min_dist,
                        max_dist,
                        stiffness,
                        ..
                    } => {
                        let target = min_dist + (max_dist - min_dist) * factor;
                        self.solve_distance(p1, p2, target, stiffness);
                    }
                    Constraint::Pin { p, pos } => {
                        self.points[p] = pos;
                    }
                }
            }
        }

        // 3. Integration (Update velocity) - Skipped
    }

    fn solve_distance(&mut self, p1: usize, p2: usize, target_dist: f32, stiffness: f32) {
        let u = self.points[p1];
        let v = self.points[p2];
        let w1 = self.weights[p1];
        let w2 = self.weights[p2];
        let w_sum = w1 + w2;

        if w_sum < 1e-6 {
            return;
        }

        let current_dist = hyperbolic_distance(u, v);
        let error = current_dist - target_dist;

        if error.abs() < 1e-5 {
            return;
        }

        // We need to move u towards v by (error * w1 / w_sum)
        // And v towards u by (error * w2 / w_sum)
        // If error > 0 (too far), move towards each other (+ distance)
        // Wait, 'move_towards' moves BY a distance.
        // If too far, we want to move closer. So move u towards v by some amount.

        let move_dist = error * stiffness;

        let u_correction = move_dist * (w1 / w_sum);
        let v_correction = move_dist * (w2 / w_sum);

        if w1 > 0.0 {
            self.points[p1] = move_towards(u, v, u_correction);
        }
        if w2 > 0.0 {
            // Move v towards u
            self.points[p2] = move_towards(v, u, v_correction);
        }
    }
}
