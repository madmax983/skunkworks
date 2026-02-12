use crate::fungus::CostMap;
use crate::physics::Body;
use crate::physics::G;
use macroquad::prelude::*;

pub struct GravityMap<'a> {
    pub bodies: &'a [Body],
    pub width: usize,
    pub height: usize,
    pub world_size: Vec2,
}

impl<'a> CostMap for GravityMap<'a> {
    fn get_cost(&self, x: i32, y: i32) -> f32 {
        let center_x = self.width as f32 / 2.0;
        let center_y = self.height as f32 / 2.0;

        let scale_x = self.world_size.x / self.width as f32;
        let scale_y = self.world_size.y / self.height as f32;

        // Grid (0,0) is Top-Left. World (0,0) is Center.
        // x=0 -> -width/2 * scale

        let wx = (x as f32 - center_x) * scale_x;
        // let wy = (y as f32 - center_y) * scale_y; // y grows down in grid, and usually down in screen.
        // In physics::check_crossings, we used standard Y.
        // harmony-of-spheres main uses camera zoom (1, -1) to flip Y?
        // "zoom: Vec2::new(1.0 / ..., -1.0 / ...)" -> Flip Y.
        // So World Y is UP. Screen Y is DOWN.
        // Grid Y is DOWN (0 at top).
        // So wy should be inverted?
        // (y - center) is positive for bottom half.
        // We want world Y negative for bottom half?
        // Yes, if camera flips it.
        // Let's flip wy.
        let wy = -(y as f32 - center_y) * scale_y;

        let pos = Vec2::new(wx, wy);

        let mut potential = 0.0;
        for body in self.bodies {
            let r = pos.distance(body.pos).max(20.0); // Avoid singularity, slightly larger radius
            potential += (G * body.mass) / r;
        }

        // Potential is magnitude of gravitational potential sum.
        // Large Potential = Deep Well = Low Cost.

        // Heuristic tuning:
        // Star Mass ~ 50000, G=1000. GM = 5e7.
        // at r=1000 (edge), V ~ 5e4.
        // at r=100 (close), V ~ 5e5.

        // We want Cost ~ 1 at r=100, Cost >> 1 at r=1000?
        // If Cost = 1e6 / V.
        // r=100 -> 1e6 / 5e5 = 2.0.
        // r=1000 -> 1e6 / 5e4 = 20.0.
        // This gives a 10x penalty for void. Good.

        let cost = 1000000.0 / (potential + 1.0);

        cost.max(1.0)
    }
}
