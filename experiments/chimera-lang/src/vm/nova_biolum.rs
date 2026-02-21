use super::{ChimeraVM, Value};

// Replicate neighbor directions from nova.rs since they are private there
const NEIGHBOR_DIRECTIONS: [(i64, i64, u8); 4] = [
    (-1, 0, 1), // N
    (1, 0, 2),  // S
    (0, -1, 8), // W
    (0, 1, 4),  // E
];

pub fn exec_luciferin(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [r, g, b, intensity]
    if vm.stack.len() >= 4 {
        let intensity_val = vm.stack.pop().unwrap();
        let b_val = vm.stack.pop().unwrap();
        let g_val = vm.stack.pop().unwrap();
        let r_val = vm.stack.pop().unwrap();

        if let (Value::Int(r), Value::Int(g), Value::Int(b), Value::Int(i)) =
            (r_val, g_val, b_val, intensity_val)
        {
            let (cy, cx) = vm.context_loc;
            if i > 0 {
                let r = r.clamp(0, 255) as u8;
                let g = g.clamp(0, 255) as u8;
                let b = b.clamp(0, 255) as u8;

                // Add intensity
                vm.light_grid[cy][cx] = vm.light_grid[cy][cx].saturating_add(i);

                // Set color (Logic: If adding light, we set the source color.
                // In reality, mixing lights is additive.
                // We'll simplisticly set it for the source.)
                vm.light_color_grid[cy][cx] = (r, g, b);

                vm.energy = vm.energy.saturating_sub(i / 10 + 1);
                vm.output.push(format!(
                    "LUCIFERIN: Emitted ({},{},{}) intensity {} at {},{}",
                    r, g, b, i, cx, cy
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for luciferin".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for luciferin".to_string());
    }
    None
}

pub fn exec_photophore(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [radius]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let intensity = vm.light_grid[cy][cx];
                let color = vm.light_color_grid[cy][cx];

                if intensity > 0 {
                    let mut count = 0;
                    crate::vm::iterate_circle(
                        #[cfg(feature = "nova")]
                        vm.topology,
                        cx as i64,
                        cy as i64,
                        r,
                        |tx, ty| {
                            vm.light_grid[ty][tx] = vm.light_grid[ty][tx].saturating_add(intensity);
                            // Propagate color
                            // If target has no light, take source color.
                            // If target has light, maybe blend? For now, source overwrites to simulate "projection".
                            vm.light_color_grid[ty][tx] = color;
                            count += 1;
                        },
                    );
                    vm.energy = vm.energy.saturating_sub((count / 2) as i64);
                    vm.output.push(format!(
                        "PHOTOPHORE: Projected light radius {} from {},{}",
                        r, cx, cy
                    ));
                } else {
                    vm.output
                        .push("PHOTOPHORE: No light to project".to_string());
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for photophore".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for photophore".to_string());
    }
    None
}

#[allow(clippy::needless_range_loop)]
pub fn diffuse_light_color(vm: &mut ChimeraVM) {
    let size = 16;
    let mut buffer = [[(0u8, 0u8, 0u8); 16]; 16];

    for y in 0..size {
        for x in 0..size {
            // Self
            let my_light = vm.light_grid[y][x];
            let (mr, mg, mb) = vm.light_color_grid[y][x];

            // If no light, no color to diffuse/maintain (fade to black logic handled implicitly if light -> 0)
            // But light_grid is updated separately. We need to follow where light went.
            // Actually, simply blurring the color grid weighted by light intensity is a decent approximation.

            let inertia = vm.biome_grid[y][x].diffusion_inertia();

            let mut r_acc: i64 = mr as i64 * my_light * inertia;
            let mut g_acc: i64 = mg as i64 * my_light * inertia;
            let mut b_acc: i64 = mb as i64 * my_light * inertia;
            let mut weight_acc: i64 = my_light * inertia;

            for (dy, dx, mask) in NEIGHBOR_DIRECTIONS {
                if (vm.membranes[y][x] & mask) != 0 {
                    continue;
                }

                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let n_light = vm.light_grid[ny][nx];
                    if n_light > 0 {
                        let (nr, ng, nb) = vm.light_color_grid[ny][nx];
                        r_acc += nr as i64 * n_light;
                        g_acc += ng as i64 * n_light;
                        b_acc += nb as i64 * n_light;
                        weight_acc += n_light;
                    }
                }
            }

            if weight_acc > 0 {
                buffer[y][x] = (
                    (r_acc / weight_acc).clamp(0, 255) as u8,
                    (g_acc / weight_acc).clamp(0, 255) as u8,
                    (b_acc / weight_acc).clamp(0, 255) as u8,
                );
            } else {
                // Keep old color if no light activity, or fade to black?
                // If light is 0, color doesn't matter much, but let's keep it for persistence if light returns?
                // No, bioluminescence usually fades.
                buffer[y][x] = (0, 0, 0);
            }
        }
    }

    for y in 0..size {
        vm.light_color_grid[y].copy_from_slice(&buffer[y]);
    }
}
