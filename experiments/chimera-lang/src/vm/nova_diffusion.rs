use super::{nova_biome::Biome, ChimeraVM};

/// Pre-calculated neighbor offsets (dy, dx) and their corresponding bitmasks.
///
/// Optimization: Used to avoid repeated calls to `get_direction_mask` inside hot loops.
/// Mappings: N=1, S=2, E=4, W=8.
pub const NEIGHBOR_DIRECTIONS: [(i64, i64, u8); 4] = [
    (-1, 0, 1), // N
    (1, 0, 2),  // S
    (0, -1, 8), // W
    (0, 1, 4),  // E
];

pub fn get_open_neighbors(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    NEIGHBOR_DIRECTIONS
        .into_iter()
        .filter_map(move |(dy, dx, mask)| {
            if (vm.membranes[y][x] & mask) != 0 {
                None
            } else {
                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
            }
        })
}

/// Generic diffusion logic for scalar grids (i64).
///
/// Applies inertia, wind flow, and decay.
///
/// **Optimization:** Uses a stack-allocated buffer `[[i64; GRID_SIZE]; GRID_SIZE]` to avoid
/// repeated heap allocations (`Vec<Vec<i64>>`) every tick.
fn diffuse_scalar_grid<F>(
    source: &mut [Vec<i64>],
    biomes: &[Vec<Biome>],
    wind: &[Vec<(i8, i8)>],
    membranes: &[Vec<u8>],
    topology: &crate::vm::Topology,
    decay_fn: F,
) where
    F: Fn(usize, usize, &Biome) -> i64,
{
    let size = crate::vm::GRID_SIZE;
    debug_assert!(
        size.is_power_of_two(),
        "Grid size must be power of 2 for bitwise wrapping"
    );
    let size_mask = (size - 1) as i64;
    let is_torus = *topology == crate::vm::Topology::Torus;
    let mut buffer = [[0i64; crate::vm::GRID_SIZE]; crate::vm::GRID_SIZE];

    for y in 0..size {
        for x in 0..size {
            let inertia = biomes[y][x].diffusion_inertia();
            let weight_center = 10;
            // Use i64 for accumulation (safe as values are bounded)
            let mut sum = source[y][x]
                .saturating_mul(inertia)
                .saturating_mul(weight_center);
            let mut total_weight = inertia.saturating_mul(weight_center);

            for (dy, dx, mask) in NEIGHBOR_DIRECTIONS {
                if (membranes[y][x] & mask) != 0 {
                    continue;
                }

                let neighbor = if is_torus {
                    Some((
                        ((y as i64 + dy) & size_mask) as usize,
                        ((x as i64 + dx) & size_mask) as usize,
                    ))
                } else {
                    topology.normalize(y as i64 + dy, x as i64 + dx, size, size)
                };

                if let Some((ny, nx)) = neighbor {
                    let (w_dy, w_dx) = wind[ny][nx];
                    // Wind flow from neighbor (ny, nx) to here (y, x).
                    let flow = -((w_dy as i64) * dy + (w_dx as i64) * dx);
                    let weight = (10 + flow).max(0);

                    sum = sum.saturating_add(source[ny][nx].saturating_mul(weight));
                    total_weight = total_weight.saturating_add(weight);
                }
            }

            let decay = decay_fn(y, x, &biomes[y][x]);
            if total_weight > 0 {
                buffer[y][x] = (sum / total_weight) * decay / 100;
            }
        }
    }

    for y in 0..size {
        // Optimization: Use copy_from_slice (memcpy) instead of element-wise loop.
        // We slice by `size` to ensure lengths match and avoid panics if vectors are oversized.
        source[y][..size].copy_from_slice(&buffer[y][..size]);
    }
}

/// Simulates the diffusion of chemical signals (hormones) across the grid.
///
/// Uses a simple cellular automaton model: each cell becomes the average of itself
/// and its open neighbors (neighbors not blocked by membranes).
///
/// # Examples
///
/// ```ignore
/// // Inside VM step loop
/// nova::diffuse_hormones(&mut vm);
/// ```
#[allow(clippy::needless_range_loop)]
/// Simulates the diffusion of chemical signals (hormones) across the grid.
///
/// Uses a simple cellular automaton model: each cell becomes the average of itself
/// and its open neighbors (neighbors not blocked by membranes).
///
/// Optimized to avoid intermediate Vec allocations.
pub fn diffuse_hormones(vm: &mut ChimeraVM) {
    let mut buffer = [[[0i64; 3]; 16]; 16];
    let size = crate::vm::GRID_SIZE as i64;
    debug_assert!(
        (crate::vm::GRID_SIZE).is_power_of_two(),
        "Grid size must be power of 2 for bitwise wrapping"
    );
    let size_mask = size - 1;
    let is_torus = vm.topology == crate::vm::Topology::Torus;

    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let weight_center = 10;
            // Use i64 for accumulation
            let mut sums = [
                vm.hormone_grid[y][x][0]
                    .saturating_mul(inertia)
                    .saturating_mul(weight_center),
                vm.hormone_grid[y][x][1]
                    .saturating_mul(inertia)
                    .saturating_mul(weight_center),
                vm.hormone_grid[y][x][2]
                    .saturating_mul(inertia)
                    .saturating_mul(weight_center),
            ];
            let mut total_weight = inertia.saturating_mul(weight_center);

            // Manual neighbor iteration to calculate wind bias
            for (dy, dx, mask) in NEIGHBOR_DIRECTIONS {
                if (vm.membranes[y][x] & mask) != 0 {
                    continue;
                }

                let neighbor = if is_torus {
                    Some((
                        ((y as i64 + dy) & size_mask) as usize,
                        ((x as i64 + dx) & size_mask) as usize,
                    ))
                } else {
                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                };

                if let Some((ny, nx)) = neighbor {
                    let (w_dy, w_dx) = vm.wind_grid[ny][nx];
                    // Wind flow from neighbor (ny, nx) to here (y, x).
                    // Vector from neighbor to here is (-dy, -dx).
                    // Dot product: w_dy * (-dy) + w_dx * (-dx)
                    let flow = -((w_dy as i64) * dy + (w_dx as i64) * dx);
                    let weight = (10 + flow).max(0); // Base 10

                    for c in 0..3 {
                        sums[c] = sums[c]
                            .saturating_add(vm.hormone_grid[ny][nx][c].saturating_mul(weight));
                    }
                    total_weight = total_weight.saturating_add(weight);
                }
            }

            if total_weight > 0 {
                for c in 0..3 {
                    buffer[y][x][c] = sums[c] / total_weight;
                }
            }
        }
    }
    for y in 0..16 {
        // Optimization: Use copy_from_slice (memcpy).
        vm.hormone_grid[y][..16].copy_from_slice(&buffer[y][..16]);
    }
}

/// Simulates the diffusion of metabolic waste products.
///
/// Waste accumulates and spreads. High concentrations trigger damage/mutation.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_waste(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.waste_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, b| b.decay_rate(),
    );
}

/// Simulates the diffusion and decay of light.
///
/// Light spreads but decays rapidly (50% per tick), simulating absorption and scattering.
/// Chloroplasts harvest energy from this grid.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_light(vm: &mut ChimeraVM) {
    super::nova_biolum::diffuse_light_color(vm);
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let mut sum = (vm.light_grid[y][x] as i128) * (inertia as i128);
            let mut count = inertia;

            for (ny, nx) in get_open_neighbors(vm, y, x) {
                sum += vm.light_grid[ny][nx] as i128;
                count += 1;
            }

            // Light interacts with Clouds (Moisture)
            let moisture = vm.moisture_grid[y][x];
            let cloud_opacity = (moisture as i64).clamp(0, 50); // Up to 50% block

            // Blur and decay (95% base + cloud)
            let transmission = 95 - cloud_opacity; // 95% -> 45% transmission relative to input
                                                   // Wait, previous was / 2 (50%).
                                                   // New logic: (sum / count) * transmission / 100?
                                                   // If transmission is 50 (clear sky), it matches previous.
                                                   // If transmission is 0 (thick cloud), light dies.

            buffer[y][x] = ((sum / count as i128) * transmission as i128 / 100) as i64;
        }
    }
    for y in 0..16 {
        // Optimization: Use copy_from_slice (memcpy).
        vm.light_grid[y][..16].copy_from_slice(&buffer[y][..16]);
    }
}

/// Simulates the diffusion of mutagenic radiation.
///
/// Mutagen spreads and decays slowly (90% retained per tick).
/// High levels cause random DNA mutations.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_mutagen(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.mutagen_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, b| b.decay_rate() * 9 / 10,
    );
}

/// Simulates the diffusion of entropy.
///
/// Entropy spreads and decays slowly.
/// High levels cause Reality Decay (glitches).
#[allow(clippy::needless_range_loop)]
pub fn diffuse_entropy(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.entropy_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, _| 95,
    );
}
