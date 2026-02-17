use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct VerletPoint {
    pub pos: Vec2,
    pub old_pos: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub locked: bool,
}

impl VerletPoint {
    pub fn new(x: f32, y: f32) -> Self {
        let pos = vec2(x, y);
        Self {
            pos,
            old_pos: pos,
            acc: vec2(0.0, 0.0),
            mass: 1.0,
            radius: 5.0,
            locked: false,
        }
    }

    pub fn update(&mut self, dt: f32, drag: f32) {
        if self.locked {
            return;
        }

        let vel = (self.pos - self.old_pos) * (1.0 - drag);
        self.old_pos = self.pos;
        self.pos += vel + self.acc * dt * dt;
        self.acc = vec2(0.0, 0.0);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force / self.mass;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DistanceConstraint {
    pub p1: usize,
    pub p2: usize,
    pub length: f32,
    pub stiffness: f32,
}

impl DistanceConstraint {
    pub fn new(p1: usize, p2: usize, length: f32) -> Self {
        Self {
            p1,
            p2,
            length,
            stiffness: 1.0,
        }
    }
}

pub struct PhysicsWorld {
    pub points: Vec<VerletPoint>,
    pub constraints: Vec<DistanceConstraint>,
    pub drag: f32,
    pub gravity: Vec2,
    pub bounds: Rect,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            constraints: Vec::new(),
            drag: 0.01,
            gravity: vec2(0.0, 0.0),
            bounds: Rect::new(0.0, 0.0, 800.0, 600.0), // Default bounds
        }
    }

    pub fn add_point(&mut self, point: VerletPoint) -> usize {
        self.points.push(point);
        self.points.len() - 1
    }

    pub fn add_constraint(&mut self, constraint: DistanceConstraint) {
        self.constraints.push(constraint);
    }

    pub fn update(&mut self, dt: f32) {
        let sub_steps = 8;
        let sub_dt = dt / sub_steps as f32;

        for _ in 0..sub_steps {
            self.step(sub_dt);
        }
    }

    fn step(&mut self, dt: f32) {
        // Apply forces
        for p in &mut self.points {
            if !p.locked {
                p.apply_force(self.gravity);
            }
        }

        // Update positions
        for p in &mut self.points {
            p.update(dt, self.drag);

            // Wall collisions (simple clamp)
            if p.pos.x < self.bounds.x {
                p.pos.x = self.bounds.x;
                p.old_pos.x = self.bounds.x;
            }
            if p.pos.x > self.bounds.w {
                p.pos.x = self.bounds.w;
                p.old_pos.x = self.bounds.w;
            }
            if p.pos.y < self.bounds.y {
                p.pos.y = self.bounds.y;
                p.old_pos.y = self.bounds.y;
            }
            if p.pos.y > self.bounds.h {
                p.pos.y = self.bounds.h;
                p.old_pos.y = self.bounds.h;
            }
        }

        // Solve Constraints
        self.solve_constraints();
    }

    fn solve_constraints(&mut self) {
        let constraints = &self.constraints;
        let points = &mut self.points;

        for c in constraints {
            if c.p1 == c.p2 {
                continue;
            }

            let (p1, p2) = get_two_mut(points, c.p1, c.p2);

            let delta = p2.pos - p1.pos;
            let dist = delta.length();

            if dist == 0.0 {
                continue;
            }

            let diff = (dist - c.length) / dist;
            let correction = delta * 0.5 * diff * c.stiffness;

            if !p1.locked {
                p1.pos += correction;
            }
            if !p2.locked {
                p2.pos -= correction;
            }
        }
    }
}

// Helper to get two mutable references from a slice
fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    assert!(i != j);
    if i < j {
        let (left, right) = slice.split_at_mut(j);
        (&mut left[i], &mut right[0])
    } else {
        let (left, right) = slice.split_at_mut(i);
        (&mut right[0], &mut left[j])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_update() {
        let mut world = PhysicsWorld::new();
        // Pendulum
        // Point 0: Locked at (0,0)
        // Point 1: at (10, 0).
        // Gravity (0, 10).
        // Constraint len 10.

        let mut p1 = VerletPoint::new(0.0, 0.0);
        p1.locked = true;
        let idx1 = world.add_point(p1);

        let p2 = VerletPoint::new(10.0, 0.0);
        let idx2 = world.add_point(p2);

        world.add_constraint(DistanceConstraint::new(idx1, idx2, 10.0));
        world.gravity = vec2(0.0, 10.0);

        // Update
        for _ in 0..100 {
            world.update(0.016);
        }

        // Check if p2 has swung down.
        // It starts at (10, 0). Gravity pulls it down.
        // It should be roughly at (0, 10) or swinging.
        // With drag, it should settle near (0, 10).

        let final_pos = world.points[idx2].pos;
        println!("Final Pos: {:?}", final_pos);

        // It should have moved from y=0 towards y>0.
        assert!(final_pos.y > 0.0);

        // Constraint should be satisfied (dist ~ 10)
        let dist = (world.points[idx2].pos - world.points[idx1].pos).length();
        assert!((dist - 10.0).abs() < 0.1);
    }
}
