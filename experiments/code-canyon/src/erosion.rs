use crate::map::Terrain;
use rand::Rng;

pub struct ErosionParams {
    pub inertia: f32,
    pub gravity: f32,
    pub capacity: f32,
    pub evaporation: f32,
    pub erosion_radius: i32,
    pub min_slope: f32,
    pub deposition_rate: f32,
    pub erosion_rate: f32,
    pub max_steps: usize,
}

impl Default for ErosionParams {
    fn default() -> Self {
        Self {
            inertia: 0.05,
            gravity: 4.0,
            capacity: 4.0,
            evaporation: 0.02,
            erosion_radius: 3,
            min_slope: 0.0001,
            deposition_rate: 0.3,
            erosion_rate: 0.3,
            max_steps: 64,
        }
    }
}

pub fn erode(terrain: &mut Terrain, iterations: usize, params: &ErosionParams) {
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let x = rng.gen_range(0..terrain.width - 1) as f32;
        let y = rng.gen_range(0..terrain.height - 1) as f32;
        trace_droplet(terrain, x, y, params);
    }
}

fn trace_droplet(terrain: &mut Terrain, mut x: f32, mut y: f32, params: &ErosionParams) {
    let mut dir_x = 0.0;
    let mut dir_y = 0.0;
    let mut speed = 1.0;
    let mut water = 1.0;
    let mut sediment = 0.0;

    for _ in 0..params.max_steps {
        let node_x = x as usize;
        let node_y = y as usize;
        let cell_offset_x = x - node_x as f32;
        let cell_offset_y = y - node_y as f32;

        let (g_x, g_y) = calculate_gradient(terrain, x, y);

        // Normalize gradient
        let len = (g_x * g_x + g_y * g_y).sqrt();
        let (g_x, g_y) = if len > 0.0 {
            (g_x / len, g_y / len)
        } else {
            (0.0, 0.0) // Flat terrain, stop
        };

        // Update direction with inertia
        dir_x = dir_x * params.inertia - g_x * (1.0 - params.inertia);
        dir_y = dir_y * params.inertia - g_y * (1.0 - params.inertia);

        // Normalize direction
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        let (dir_x, dir_y) = if len > 0.0 {
            (dir_x / len, dir_y / len)
        } else {
            (0.0, 0.0)
        };

        // New position
        let new_x = x + dir_x;
        let new_y = y + dir_y;

        if new_x < 0.0
            || new_x >= (terrain.width - 1) as f32
            || new_y < 0.0
            || new_y >= (terrain.height - 1) as f32
        {
            break;
        }

        // Height difference
        let height_old = get_height_interpolated(terrain, x, y);
        let height_new = get_height_interpolated(terrain, new_x, new_y);
        let height_diff = height_new - height_old;

        // Sediment capacity
        let capacity = (-height_diff).max(params.min_slope) * speed * water * params.capacity;

        // Erosion or Deposition
        if height_diff > 0.0 || sediment > capacity {
            // Deposit
            let amount_to_deposit = if height_diff > 0.0 {
                height_diff.min(sediment) // Fill the hole
            } else {
                (sediment - capacity) * params.deposition_rate
            };

            sediment -= amount_to_deposit;
            deposit(
                terrain,
                node_x,
                node_y,
                cell_offset_x,
                cell_offset_y,
                amount_to_deposit,
            );
        } else {
            // Erode
            let amount_to_erode = ((capacity - sediment) * params.erosion_rate).min(-height_diff);

            sediment += amount_to_erode;
            erode_radius(
                terrain,
                node_x,
                node_y,
                amount_to_erode,
                params.erosion_radius,
            );
        }

        // Update speed and water
        speed = (speed * speed - height_diff * params.gravity).abs().sqrt();
        water *= 1.0 - params.evaporation;

        x = new_x;
        y = new_y;
    }
}

fn calculate_gradient(terrain: &Terrain, x: f32, y: f32) -> (f32, f32) {
    let x0 = x as usize;
    let y0 = y as usize;
    let x1 = (x0 + 1).min(terrain.width - 1);
    let y1 = (y0 + 1).min(terrain.height - 1);

    let u = x - x0 as f32;
    let v = y - y0 as f32;

    let h00 = terrain.heightmap[y0 * terrain.width + x0];
    let h10 = terrain.heightmap[y0 * terrain.width + x1];
    let h01 = terrain.heightmap[y1 * terrain.width + x0];
    let h11 = terrain.heightmap[y1 * terrain.width + x1];

    let gx = (h10 - h00) * (1.0 - v) + (h11 - h01) * v;
    let gy = (h01 - h00) * (1.0 - u) + (h11 - h10) * u;

    (gx, gy)
}

fn get_height_interpolated(terrain: &Terrain, x: f32, y: f32) -> f32 {
    let x0 = x as usize;
    let y0 = y as usize;
    let x1 = (x0 + 1).min(terrain.width - 1);
    let y1 = (y0 + 1).min(terrain.height - 1);

    let u = x - x0 as f32;
    let v = y - y0 as f32;

    let h00 = terrain.heightmap[y0 * terrain.width + x0];
    let h10 = terrain.heightmap[y0 * terrain.width + x1];
    let h01 = terrain.heightmap[y1 * terrain.width + x0];
    let h11 = terrain.heightmap[y1 * terrain.width + x1];

    h00 * (1.0 - u) * (1.0 - v) + h10 * u * (1.0 - v) + h01 * (1.0 - u) * v + h11 * u * v
}

fn deposit(terrain: &mut Terrain, x: usize, y: usize, u: f32, v: f32, amount: f32) {
    let x1 = (x + 1).min(terrain.width - 1);
    let y1 = (y + 1).min(terrain.height - 1);

    terrain.heightmap[y * terrain.width + x] += amount * (1.0 - u) * (1.0 - v);
    terrain.heightmap[y * terrain.width + x1] += amount * u * (1.0 - v);
    terrain.heightmap[y1 * terrain.width + x] += amount * (1.0 - u) * v;
    terrain.heightmap[y1 * terrain.width + x1] += amount * u * v;
}

fn erode_radius(terrain: &mut Terrain, x: usize, y: usize, amount: f32, radius: i32) {
    // Simplified: Erode weighted by distance from center
    let mut weight_sum = 0.0;
    let mut weights = Vec::new();

    // Collect weights
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < terrain.width as i32 && ny >= 0 && ny < terrain.height as i32 {
                let dist = (dx * dx + dy * dy) as f32;
                if dist <= (radius * radius) as f32 {
                    let w = (radius as f32 - dist.sqrt()).max(0.0);
                    weight_sum += w;
                    weights.push((nx as usize, ny as usize, w));
                }
            }
        }
    }

    if weight_sum > 0.0 {
        for (nx, ny, w) in weights {
            terrain.heightmap[ny * terrain.width + nx] -= amount * (w / weight_sum);
        }
    }
}
