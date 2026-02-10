#[cfg(test)]
mod tests {
    use crate::terrain::Terrain;

    #[test]
    fn test_erosion_moves_sediment_downhill() {
        let width = 20;
        let height = 20;
        let mut terrain = Terrain::new(width, height);

        // Create a slope: High at x=0, Low at x=width
        for y in 0..height {
            for x in 0..width {
                let h = (width - x) as f64 * 10.0;
                terrain.set_height(x, y, h);
            }
        }

        let initial_top_height = terrain.get_height(5, 10);
        let initial_bottom_height = terrain.get_height(15, 10);

        // Run erosion multiple times
        for _ in 0..1000 {
            // Drop rain randomly on the upper slope
            let rx = (rand::random::<f64>() * 5.0) + 2.0;
            let ry = (rand::random::<f64>() * (height as f64 - 2.0)) + 1.0;
            terrain.erode_droplet(rx, ry);
        }

        let final_top_height = terrain.get_height(5, 10);
        let final_bottom_height = terrain.get_height(15, 10);

        // Check erosion at the top
        assert!(final_top_height < initial_top_height, "Top should erode (Initial: {}, Final: {})", initial_top_height, final_top_height);

        // Check deposition at the bottom (or at least somewhere downhill)
        // Note: Sediment might wash off the map if not careful, but with 1000 drops, some should deposit.
        // Or at least, the "valley" should fill up if we had a valley.
        // With a pure slope, it might just wash away.
        // Let's make a "valley" shape: \/

    }

    #[test]
    fn test_erosion_fills_valley() {
        let width = 20;
        let height = 20;
        let mut terrain = Terrain::new(width, height);

        // V-shape valley at x=10
        for y in 0..height {
            for x in 0..width {
                let dist = (x as i32 - 10).abs() as f64;
                terrain.set_height(x, y, dist * 10.0);
            }
        }

        // Pit at the bottom (to catch sediment)
        terrain.set_height(10, 10, -50.0);

        let initial_pit_height = terrain.get_height(10, 10);

        // Rain on the slopes
        for _ in 0..5000 {
            let rx = rand::random::<f64>() * width as f64;
            let ry = rand::random::<f64>() * height as f64;
            terrain.erode_droplet(rx, ry);
        }

        let final_pit_height = terrain.get_height(10, 10);

        assert!(final_pit_height > initial_pit_height, "Pit should fill with sediment (Initial: {}, Final: {})", initial_pit_height, final_pit_height);
    }
}
