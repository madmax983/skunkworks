use crate::phonology::TerrainPoint;
use rand::Rng;

pub fn hydraulic_erosion(terrain: &mut Vec<TerrainPoint>) {
    let len = terrain.len();
    if len < 2 {
        return;
    }

    let mut rng = rand::thread_rng();

    // Simulate multiple drops per call for noticeable effect
    for _ in 0..100 {
        let mut pos = rng.gen_range(0..len);
        let mut sediment = 0.0;
        let erosion_rate = 0.1;
        let deposition_rate = 0.1;

        // Drop life
        for _ in 0..10 {
            let current_h = terrain[pos].height;
            let current_hard = terrain[pos].hardness;

            // Find lowest neighbor
            let mut best_neighbor = pos;
            let mut best_diff = 0.0;

            // Check Left
            if pos > 0 {
                let diff = current_h - terrain[pos - 1].height;
                if diff > best_diff {
                    best_diff = diff;
                    best_neighbor = pos - 1;
                }
            }

            // Check Right
            if pos < len - 1 {
                let diff = current_h - terrain[pos + 1].height;
                if diff > best_diff {
                    best_diff = diff;
                    best_neighbor = pos + 1;
                }
            }

            if best_neighbor != pos {
                // Move downhill
                // Erode current
                let erode_amount = best_diff * erosion_rate * (1.0 - current_hard * 0.5); // Hard rocks erode less
                terrain[pos].height -= erode_amount;
                sediment += erode_amount;

                // Move
                pos = best_neighbor;

                // Deposit some
                let deposit = sediment * deposition_rate;
                terrain[pos].height += deposit;
                sediment -= deposit;
            } else {
                // Local minima - deposit all sediment
                terrain[pos].height += sediment;
                break;
            }
        }
    }
}

pub fn thermal_weathering(terrain: &mut Vec<TerrainPoint>) {
    let len = terrain.len();
    if len < 3 {
        return;
    }

    // Simple diffusion (blur)
    // We need a temp buffer to avoid feedback loop bias
    let old_terrain = terrain.clone();

    for i in 1..len - 1 {
        let h_left = old_terrain[i - 1].height;
        let h_right = old_terrain[i + 1].height;
        let h_curr = old_terrain[i].height;

        // If current is higher than average of neighbors, it crumbles
        let avg = (h_left + h_right) / 2.0;
        if h_curr > avg {
            let diff = h_curr - avg;
            let crumble = diff * 0.1 * (1.0 - old_terrain[i].hardness * 0.8);

            terrain[i].height -= crumble;
            terrain[i - 1].height += crumble * 0.5;
            terrain[i + 1].height += crumble * 0.5;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phonology::TerrainPoint;

    #[test]
    fn test_peak_erosion() {
        let mut terrain = vec![
            TerrainPoint {
                height: 0.1,
                hardness: 0.1,
            },
            TerrainPoint {
                height: 1.0,
                hardness: 0.1,
            }, // Peak
            TerrainPoint {
                height: 0.1,
                hardness: 0.1,
            },
        ];

        let initial_height = terrain[1].height;

        // Apply significant erosion
        // We need enough iterations/drops to ensure hit
        for _ in 0..10 {
            hydraulic_erosion(&mut terrain);
        }

        assert!(
            terrain[1].height < initial_height,
            "Peak should erode. Was {}, now {}",
            initial_height,
            terrain[1].height
        );
    }

    #[test]
    fn test_sediment_transport() {
        let mut terrain = vec![
            TerrainPoint {
                height: 1.0,
                hardness: 0.1,
            }, // Source
            TerrainPoint {
                height: 0.5,
                hardness: 0.1,
            }, // Slope
            TerrainPoint {
                height: 0.0,
                hardness: 0.1,
            }, // Sink
        ];
        // Note: Needs a slope or direct connection. 2 points might be tricky if drop spawns on 0.
        // I added a middle point to ensure flow.

        let initial_sink = terrain[2].height;

        for _ in 0..20 {
            hydraulic_erosion(&mut terrain);
        }

        assert!(
            terrain[2].height > initial_sink,
            "Sediment should fill valley. Was {}, now {}",
            initial_sink,
            terrain[2].height
        );
    }
}
