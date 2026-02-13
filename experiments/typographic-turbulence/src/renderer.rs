use crate::lbm::{FluidSim, WIDTH, HEIGHT};

const DENSITY_CHARS: &[char] = &[' ', ' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

pub fn render_ascii(sim: &FluidSim) -> String {
    // Pre-allocate string buffer
    // WIDTH chars + 1 newline per row
    let mut output = String::with_capacity((WIDTH + 1) * HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            if sim.obstacles[idx] {
                // Obstacles are rendered by the main loop overlay,
                // but for the base fluid string, we can put a placeholder or transparent char.
                // Let's put a space, so the text overlay can be drawn on top (or we combine here).
                // Actually, if we return a String, we can't easily "overlay" without rebuilding.
                // It's better if this function takes the text_overlay map.
                output.push(' ');
            } else {
                let rho = sim.density[idx];
                // Map density to char
                // range 0.0 to ~2.0
                // 1.0 is equilibrium.
                // Let's visualize deviation from 1.0?
                // Or just absolute density.
                // Fluids often have density ~1.0. Waves are 1.0 +/- 0.01.
                // This is "Compressible" LBM, so density varies.
                // But for "Smoke", we usually simulate a scalar "Ink" field advected by the fluid.
                // Here we are visualizing the *fluid density* itself (acoustic waves).
                // To see "ripples", we need high contrast around 1.0.

                let val = (rho - 1.0).abs() * 20.0; // Amplify variations
                // Or just use the " Ink" approach: Add a passive scalar field?
                // The plan didn't specify.
                // Visualizing pressure waves (density) is cool.
                // Let's try amplifying (rho - 1.0).

                let char_idx = (val.clamp(0.0, 1.0) * (DENSITY_CHARS.len() - 1) as f32) as usize;
                output.push(DENSITY_CHARS[char_idx]);
            }
        }
        output.push('\n');
    }
    output
}
