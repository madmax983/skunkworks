use crate::map::Map;

const GRAVITY: f32 = 9.81;
const PIPE_AREA: f32 = 1.0;
const PIPE_LEN: f32 = 1.0;
const DAMPING: f32 = 0.995;

pub fn step(map: &mut Map, dt: f32) {
    let w = map.width;
    let h = map.height;
    let size = w * h;

    // 1. Update Flux
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let h1 = map.terrain[idx] + map.water[idx];

            // Neighbors: Left(0), Right(1), Top(2), Bottom(3)

            // Left
            if x > 0 {
                let n_idx = idx - 1;
                let h2 = map.terrain[n_idx] + map.water[n_idx];
                let dh = h1 - h2;
                map.flux[idx][0] += dt * GRAVITY * PIPE_AREA * dh / PIPE_LEN;
                map.flux[idx][0] *= DAMPING;
                if map.flux[idx][0] < 0.0 { map.flux[idx][0] = 0.0; }
            } else {
                map.flux[idx][0] = 0.0;
            }

            // Right
            if x < w - 1 {
                let n_idx = idx + 1;
                let h2 = map.terrain[n_idx] + map.water[n_idx];
                let dh = h1 - h2;
                map.flux[idx][1] += dt * GRAVITY * PIPE_AREA * dh / PIPE_LEN;
                map.flux[idx][1] *= DAMPING;
                if map.flux[idx][1] < 0.0 { map.flux[idx][1] = 0.0; }
            } else {
                map.flux[idx][1] = 0.0;
            }

            // Top
            if y > 0 {
                let n_idx = idx - w;
                let h2 = map.terrain[n_idx] + map.water[n_idx];
                let dh = h1 - h2;
                map.flux[idx][2] += dt * GRAVITY * PIPE_AREA * dh / PIPE_LEN;
                map.flux[idx][2] *= DAMPING;
                if map.flux[idx][2] < 0.0 { map.flux[idx][2] = 0.0; }
            } else {
                map.flux[idx][2] = 0.0;
            }

            // Bottom
            if y < h - 1 {
                let n_idx = idx + w;
                let h2 = map.terrain[n_idx] + map.water[n_idx];
                let dh = h1 - h2;
                map.flux[idx][3] += dt * GRAVITY * PIPE_AREA * dh / PIPE_LEN;
                map.flux[idx][3] *= DAMPING;
                if map.flux[idx][3] < 0.0 { map.flux[idx][3] = 0.0; }
            } else {
                map.flux[idx][3] = 0.0;
            }
        }
    }

    // 2. Validate Flux and Calculate Changes
    let mut changes = vec![0.0; size];

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let out_total = map.flux[idx][0] + map.flux[idx][1] + map.flux[idx][2] + map.flux[idx][3];

            // Scaling to prevent negative volume
            let scale = if out_total * dt > map.water[idx] {
                map.water[idx] / (out_total * dt).max(0.0001)
            } else {
                1.0
            };

            map.flux[idx][0] *= scale;
            map.flux[idx][1] *= scale;
            map.flux[idx][2] *= scale;
            map.flux[idx][3] *= scale;

            let outflow = out_total * scale;

            let mut inflow = 0.0;
            if x > 0 { inflow += map.flux[idx - 1][1]; }
            if x < w - 1 { inflow += map.flux[idx + 1][0]; }
            if y > 0 { inflow += map.flux[idx - w][3]; }
            if y < h - 1 { inflow += map.flux[idx + w][2]; }

            changes[idx] = (inflow - outflow) * dt;

            // Calculate velocity for rendering (simplified)
            let in_left = if x > 0 { map.flux[idx-1][1] } else { 0.0 };
            let in_right = if x < w - 1 { map.flux[idx+1][0] } else { 0.0 };
            let in_top = if y > 0 { map.flux[idx-w][3] } else { 0.0 };
            let in_bottom = if y < h - 1 { map.flux[idx+w][2] } else { 0.0 };

            let u = (map.flux[idx][1] - map.flux[idx][0] + in_left - in_right) / 2.0;
            let v = (map.flux[idx][3] - map.flux[idx][2] + in_top - in_bottom) / 2.0;

            map.velocity[idx] = [u, v];
        }
    }

    // 3. Apply changes
    for i in 0..size {
        map.water[i] += changes[i];
        if map.water[i] < 0.0 { map.water[i] = 0.0; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_spread() {
        let mut map = Map::new(3, 3);
        // Clear terrain for simple test
        map.terrain = vec![0.0; 9];
        // Add water in center
        let center = 4;
        map.water[center] = 10.0;

        step(&mut map, 0.1);

        // Center should have less water
        assert!(map.water[center] < 10.0);
        // Neighbors should have some water
        assert!(map.water[center-1] > 0.0); // Left
        assert!(map.water[center+1] > 0.0); // Right
        assert!(map.water[center-3] > 0.0); // Top
        assert!(map.water[center+3] > 0.0); // Bottom
    }
}
