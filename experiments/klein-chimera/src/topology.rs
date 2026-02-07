use glam::Vec2;

pub struct Topology {
    pub width: f32,
    pub height: f32,
}

impl Topology {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Wraps a position according to Klein Bottle topology.
    /// Returns the wrapped position and a boolean indicating if a chiral flip occurred.
    ///
    /// Centered coordinates: [-width/2, width/2] x [-height/2, height/2]
    /// X wraps normally (Torus).
    /// Y wraps with a twist (Reflection of X).
    pub fn wrap(&self, pos: Vec2) -> (Vec2, bool) {
        let mut x = pos.x;
        let mut y = pos.y;
        let mut flipped = false;

        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        // Wrap X (Cylinder-like)
        if x < -half_w {
            x += self.width;
        } else if x > half_w {
            x -= self.width;
        }

        // Wrap Y (Twist)
        if y < -half_h {
            y += self.height;
            x = -x; // Flip X coordinate
            flipped = !flipped; // Toggle flip state
        } else if y > half_h {
            y -= self.height;
            x = -x; // Flip X coordinate
            flipped = !flipped; // Toggle flip state
        }

        (Vec2::new(x, y), flipped)
    }

    /// Calculates distance on the manifold.
    /// Does NOT account for all geodesics, just the direct one and the wrapped ones.
    /// For flocking, we usually check neighbors.
    pub fn distance_sq(&self, a: Vec2, b: Vec2) -> f32 {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();

        // Normal wrap distance
        let dx_wrap = if dx > self.width / 2.0 { self.width - dx } else { dx };
        let dy_wrap = if dy > self.height / 2.0 { self.height - dy } else { dy };

        // Twist wrap distance?
        // If we cross the twist boundary, Y distance is dy_wrap, but X becomes relative to -X?
        // This is complex for flocking. Simplified: Use normal torus distance for local interactions,
        // but maybe ignore the twist for distance calculation to keep it simple,
        // OR implement proper metric.
        // For now, standard torus metric is "good enough" for local flocking if the twist is far away.
        // But the twist connects top to bottom.

        dx_wrap * dx_wrap + dy_wrap * dy_wrap
    }
}
