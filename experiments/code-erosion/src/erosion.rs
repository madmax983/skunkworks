use crate::terrain::TerrainMap;

const INERTIA: f64 = 0.05;
const CAPACITY_FACTOR: f64 = 4.0;
const DEPOSITION_RATE: f64 = 0.3;
const EROSION_RATE: f64 = 0.3;
const EVAPORATION_RATE: f64 = 0.02;
const GRAVITY: f64 = 4.0;
const MIN_SLOPE: f64 = 0.0001;

#[derive(Debug, Clone)]
pub struct Droplet {
    pub x: f64,
    pub y: f64,
    pub dir_x: f64,
    pub dir_y: f64,
    pub velocity: f64,
    pub water: f64,
    pub sediment: f64,
}

impl Droplet {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            dir_x: 0.0,
            dir_y: 0.0,
            velocity: 1.0,
            water: 1.0,
            sediment: 0.0,
        }
    }

    pub fn erode(&mut self, map: &mut TerrainMap) -> bool {
        // Stop if out of bounds or out of water
        if self.water < 0.01 {
            return false;
        }

        let x_int = self.x as usize;
        let y_int = self.y as usize;

        if x_int >= map.width - 1 || y_int >= map.height - 1 {
            return false;
        }

        let (gx, gy) = map.get_gradient(self.x, self.y);

        // Update direction
        self.dir_x = self.dir_x * INERTIA - gx * (1.0 - INERTIA);
        self.dir_y = self.dir_y * INERTIA - gy * (1.0 - INERTIA);

        // Normalize direction
        let len = (self.dir_x * self.dir_x + self.dir_y * self.dir_y).sqrt();
        if len > 0.0 {
            self.dir_x /= len;
            self.dir_y /= len;
        }

        let old_x = self.x;
        let old_y = self.y;
        let h_old = map.get_f64(old_x, old_y);

        // Move
        self.x += self.dir_x;
        self.y += self.dir_y;

        // Check bounds after move
        if self.x < 0.0
            || self.y < 0.0
            || self.x >= (map.width - 1) as f64
            || self.y >= (map.height - 1) as f64
        {
            return false;
        }

        let h_new = map.get_f64(self.x, self.y);
        let diff = h_new - h_old;

        // Calculate sediment capacity
        let c = (-diff).max(MIN_SLOPE) * self.velocity * self.water * CAPACITY_FACTOR;

        if self.sediment > c || diff > 0.0 {
            // Deposit
            let amount = (self.sediment - c) * DEPOSITION_RATE;
            // Or if going uphill, fill the hole? simplified:
            let amount = if diff > 0.0 {
                diff.min(self.sediment) // Fill the pit up to water level? Simplified: just deposit everything or proportional.
            } else {
                amount
            };

            let amount = amount.max(0.0); // Safety

            self.sediment -= amount;
            // Modify map at OLD position (depositing sediment where we were)
            // Or new position? Usually we deposit at the current location if we can't carry it.
            // Let's deposit at old position to fill the valley we are leaving?
            // Standard implementation: Deposit at OLD pos is safer for stability.
            let cur_h = map.get_f64(old_x, old_y);
            map.set_f64(old_x, old_y, cur_h + amount);
        } else {
            // Erode
            let amount = ((c - self.sediment) * EROSION_RATE).min(-diff);
            let amount = amount.max(0.0);

            self.sediment += amount;
            let cur_h = map.get_f64(old_x, old_y);
            map.set_f64(old_x, old_y, cur_h - amount);
        }

        // Update velocity
        self.velocity = (self.velocity * self.velocity + diff * GRAVITY).sqrt();
        if self.velocity.is_nan() {
            self.velocity = 0.0;
        } // Sqrt of negative if going uphill fast

        // Evaporate
        self.water *= 1.0 - EVAPORATION_RATE;

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_downhill() {
        let mut map = TerrainMap::new(10, 10);
        // Create a slope: h = x
        for x in 0..10 {
            for y in 0..10 {
                map.set(x, y, x as f64);
            }
        }

        // Start at x=5, y=5. Gradient is (1, 0) (h increases with x).
        // So water should flow towards LOWER h.
        // Wait, gradient is pointing uphill?
        // h(x+1) > h(x). Gradient > 0.
        // direction -= gradient. So direction should be NEGATIVE x.

        let mut drop = Droplet::new(5.0, 5.0);
        drop.erode(&mut map);

        assert!(
            drop.x < 5.0,
            "Water should flow downhill (negative x direction), got x={}",
            drop.x
        );
    }

    #[test]
    fn test_deposition() {
        let mut map = TerrainMap::new(10, 10);
        map.set(5, 5, 10.0);
        map.set(4, 5, 5.0); // Valley at 4,5

        let mut drop = Droplet::new(5.0, 5.0);
        drop.sediment = 100.0; // Lots of sediment
        drop.velocity = 0.1; // Slow moving

        // Should flow to 4.0
        // And deposit because sediment > capacity

        drop.erode(&mut map);

        // Height at 5,5 (old pos) should increase due to deposition?
        // Wait, my logic deposits at OLD pos.
        // "Deposit at OLD pos to fill the valley we are leaving?"
        // If I am at 5,5 (Peak) and move to 4,5 (Valley).
        // Diff = 5 - 10 = -5.
        // Capacity = high (steep slope).
        // If sediment > capacity (which it is, 100), we deposit.
        // We deposit at OLD pos (5,5). So the peak gets HIGHER?
        // That seems counter-intuitive for "filling a valley".
        // Usually you deposit at the NEW pos to fill the hole you just entered.
        // Let's re-read standard algo or think about it.

        // If I can't carry sediment, I drop it.
        // If I am on a flat surface, I drop it HERE.
        // If I am entering a pit, I should drop it IN THE PIT.
        // So deposition should probably be at `old_x` if we haven't moved, or `x` if we have?
        // But my code: `map.set_f64(old_x, old_y, cur_h + amount);`
        // So I am raising the point I just left.

        // If I am going downhill fast, I erode. I dig out the point I just left.
        // That makes sense. I take dirt FROM here and move it there.
        // So Erosion at old_pos is correct.
        // Deposition at old_pos means I drop dirt before I leave.
        // That also makes sense if I'm slowing down.

        let h_after = map.get(5, 5);
        assert!(
            h_after > 10.0,
            "Should have deposited sediment at old position, h was 10.0, now {}",
            h_after
        );
    }
}
