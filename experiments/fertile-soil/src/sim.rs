use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct SimulationParams {
    pub feed: f32,
    pub kill: f32,
    pub dt: f32,
    pub diff_a: f32,
    pub diff_b: f32,
}

impl Default for SimulationParams {
    fn default() -> Self {
        // Standard "Spots" or "Cells" parameters
        Self {
            feed: 0.055,
            kill: 0.062,
            dt: 1.0,
            diff_a: 1.0,
            diff_b: 0.5,
        }
    }
}

pub fn get_initial_state(width: usize, height: usize) -> Vec<u8> {
    let mut data = vec![0u8; width * height * 4];
    let mut rng = rand::thread_rng();

    // Fill with A=1 (R=255), B=0
    for i in (0..data.len()).step_by(4) {
        data[i] = 255;     // R (A)
        data[i + 1] = 0;   // G (B)
        data[i + 2] = 0;   // B (aux)
        data[i + 3] = 255; // A (Alpha)
    }

    // Add a seed of B in the center
    let center_x = width / 2;
    let center_y = height / 2;
    let radius = (width.min(height) / 10).max(5); // 10% or at least 5px

    for y in (center_y.saturating_sub(radius))..(center_y + radius).min(height) {
        for x in (center_x.saturating_sub(radius))..(center_x + radius).min(width) {
            let idx = (y * width + x) * 4;
            // Set B=1 (G=255)
            // A decreases slightly as B increases? Usually we just inject B.
            data[idx + 1] = 255;

            // Add some noise to break symmetry
            if rng.gen_bool(0.5) {
                data[idx + 1] = 200;
            }
        }
    }

    // Add random specks
    for _ in 0..(width * height / 100) {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let idx = (y * width + x) * 4;
        data[idx + 1] = 255;
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_default() {
        let params = SimulationParams::default();
        assert_eq!(params.feed, 0.055);
    }

    #[test]
    fn test_initial_state_size() {
        let width = 100;
        let height = 100;
        let state = get_initial_state(width, height);
        assert_eq!(state.len(), width * height * 4);
    }

    #[test]
    fn test_initial_state_content() {
        // Should have some A (Red) and some B (Green)
        let width = 10;
        let height = 10;
        let state = get_initial_state(width, height);

        // Check if there is at least one non-zero byte
        let has_data = state.iter().any(|&x| x > 0);
        assert!(has_data, "State should not be empty/black");

        // Check for Green channel activity
        let has_green = state.iter().enumerate().any(|(i, &x)| i % 4 == 1 && x > 0);
        assert!(has_green, "State should have some B (Green) component");
    }
}
