use num_complex::Complex;
use poincare_disk::{mobius_add, mobius_sub, hyperbolic_dist, Point};

#[derive(Clone, Debug)]
pub struct Node {
    pub pos: Point,
    pub vel: Complex<f64>, // Velocity vector in the local tangent space (at the origin if transformed)
    pub fixed: bool,
}

pub struct HyperbolicString {
    pub nodes: Vec<Node>,
    pub tension: f64,
    pub damping: f64,
    pub rest_length: f64,
}

impl HyperbolicString {
    pub fn new(start: Point, end: Point, segments: usize, tension: f64, damping: f64) -> Self {
        let mut nodes = Vec::with_capacity(segments + 1);

        // Initialize nodes along the geodesic from start to end
        // We can find intermediate points by interpolating in the "Klein model" (where geodesics are straight)
        // or by using Möbius transforms.
        // Easiest: Transform start to origin. Then end is at some point P_end'.
        // Interpolate linearly from 0 to P_end' *in radial distance*? No.
        // In the disk model, a geodesic through the origin is a straight line.
        // So we transform start to origin. The target becomes P_end_prime.
        // We place nodes along the segment [0, P_end_prime].
        // The spacing should be equidistant in hyperbolic metric.
        // Dist D = hyperbolic_dist(0, P_end_prime).
        // Segment length d = D / segments.
        // Node i distance from origin = i * d.
        // Euclidean distance r_i = tanh( (i*d) / 2 ).
        // Direction is P_end_prime / |P_end_prime|.

        let transform_to_origin = poincare_disk::Mobius::inverse_translation(start);
        let end_prime = transform_to_origin.apply(end);
        let total_dist = hyperbolic_dist(Point::new(0.0, 0.0), end_prime);
        let segment_dist = total_dist / segments as f64;
        let direction = if end_prime.norm() > 1e-9 {
            end_prime / end_prime.norm()
        } else {
            Complex::new(1.0, 0.0)
        };

        // Transform back to original position
        let transform_back = transform_to_origin.inverse();

        for i in 0..=segments {
            let dist_from_origin = i as f64 * segment_dist;
            let r_euclid = (dist_from_origin / 2.0).tanh();
            let pos_prime = direction * r_euclid;
            let pos = transform_back.apply(pos_prime);

            nodes.push(Node {
                pos,
                vel: Complex::new(0.0, 0.0),
                fixed: i == 0 || i == segments,
            });
        }

        Self {
            nodes,
            tension,
            damping,
            rest_length: segment_dist,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let n = self.nodes.len();
        if n < 2 { return; }

        let mut forces = vec![Complex::new(0.0, 0.0); n];

        // Calculate forces
        for i in 0..n {
            if self.nodes[i].fixed { continue; }

            // Neighbors: i-1 and i+1
            let neighbors = [i.checked_sub(1), Some(i + 1)];

            for &neighbor_idx in &neighbors {
                if let Some(j) = neighbor_idx {
                    if j < n {
                        // Calculate vector to neighbor in local tangent space of i
                        // Move i to origin -> j moves to P_j'
                        // P_j' = mobius_sub(P_j, P_i)
                        let p_j_prime = mobius_sub(self.nodes[j].pos, self.nodes[i].pos);

                        let dist = 2.0 * p_j_prime.norm().atanh();

                        if dist > 1e-9 {
                            let direction = p_j_prime / p_j_prime.norm();
                            let displacement = dist - self.rest_length;
                            // Force = k * displacement * direction
                            let force_mag = self.tension * displacement;
                            forces[i] += direction * force_mag;
                        }
                    }
                }
            }
        }

        // Apply integration
        for i in 0..n {
            if self.nodes[i].fixed { continue; }

            // Symplectic Euler-ish
            // v += a * dt
            self.nodes[i].vel += forces[i] * dt;

            // Damping
            self.nodes[i].vel *= 1.0 - self.damping * dt;

            // Update position
            // Move by velocity vector (treated as displacement from origin in local frame)
            let step = self.nodes[i].vel * dt;

            // Limit step size to avoid jumping out of disk or too far
            if step.norm() > 0.1 {
                self.nodes[i].vel *= 0.1 / step.norm(); // Cap velocity for stability
            }
            let step_clamped = self.nodes[i].vel * dt;

            // New Pos = P_i (+) Step
            // Note: mobius_add(z, a) = (z+a)/(1+a'z). Maps 0 to a.
            // We want to map P_i (which is 0 in local frame) to Step.
            // So we apply the translation defined by Step to P_i?
            // Actually, `mobius_add(step, p_i)` means "Translation by P_i applied to step"?
            // If P_i is the "base point" (translation from origin), and Step is the "local movement",
            // then NewPos = mobius_add(Step, P_i).
            // Let's verify:
            // mobius_add(0, P_i) = P_i. (Correct, 0 step means stay at P_i).
            // mobius_add(Step, 0) = Step. (Correct, if P_i was origin, we move to Step).
            self.nodes[i].pos = mobius_add(step_clamped, self.nodes[i].pos);

            // Clamp to stay inside disk (numerical stability)
            let norm = self.nodes[i].pos.norm();
            if norm > 0.999 {
                self.nodes[i].pos = self.nodes[i].pos / norm * 0.999;
                self.nodes[i].vel *= 0.0; // Kill velocity on collision with "infinity"
            }
        }
    }

    pub fn pluck(&mut self, index: usize, force: Complex<f64>) {
        if index > 0 && index < self.nodes.len() - 1 {
             self.nodes[index].vel += force;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_initialization() {
        let start = Point::new(-0.5, 0.0);
        let end = Point::new(0.5, 0.0);
        let string = HyperbolicString::new(start, end, 2, 50.0, 0.5);

        assert_eq!(string.nodes.len(), 3);
        assert!((string.nodes[0].pos - start).norm() < 1e-9);
        assert!((string.nodes[2].pos - end).norm() < 1e-9);

        // Midpoint should be at origin
        assert!(string.nodes[1].pos.norm() < 1e-9);
    }
}
