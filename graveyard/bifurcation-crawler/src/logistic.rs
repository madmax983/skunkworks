use macroquad::prelude::*;

/// Represents the visual and mathematical state of the Logistic Map bifurcation diagram.
///
/// This struct computes the chaotic bifurcations of the equation $x_{n+1} = r \cdot x_n \cdot (1 - x_n)$
/// and generates a visual representation of it as a `macroquad::Texture2D`.
pub struct LogisticMap {
    /// The generated visual texture of the bifurcation diagram.
    pub texture: Texture2D,
    /// The minimum $r$ parameter (horizontal axis start, typically ~2.8 for the stable phase).
    pub min_r: f32,
    /// The maximum $r$ parameter (horizontal axis end, typically ~4.0 for pure chaos).
    pub max_r: f32,
}

impl LogisticMap {
    /// Constructs a new `LogisticMap` and pre-computes its texture representation.
    ///
    /// # Arguments
    ///
    /// * `width` - The pixel width of the generated texture map.
    /// * `height` - The pixel height of the generated texture map.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bifurcation_crawler::logistic::LogisticMap;
    /// // Generate a 512x512 logistic map texture
    /// let map = LogisticMap::new(512, 512);
    /// assert_eq!(map.min_r, 2.8);
    /// assert_eq!(map.max_r, 4.0);
    /// ```
    pub fn new(width: u16, height: u16) -> Self {
        let mut image = Image::gen_image_color(width, height, BLACK);

        let min_r = 2.8;
        let max_r = 4.0;

        for px in 0..width {
            let r = min_r + (px as f32 / width as f32) * (max_r - min_r);
            let mut x = 0.5;

            // Settle
            for _ in 0..100 {
                x = r * x * (1.0 - x);
            }

            // Draw
            for _ in 0..200 {
                x = r * x * (1.0 - x);
                let py = ((1.0 - x) * (height as f32 - 1.0)) as u32;
                if py < height as u32 {
                    // Accumulate brightness
                    let existing = image.get_pixel(px as u32, py);
                    let brightness = (existing.g + 0.1).min(1.0);
                    image.set_pixel(
                        px as u32,
                        py,
                        Color::new(0.0, brightness, existing.b + 0.05, 1.0),
                    );
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        Self {
            texture,
            min_r,
            max_r,
        }
    }

    /// Calculates the Euclidean distance from a given `x` coordinate to the nearest mathematical
    /// attractor point for a given growth rate `r`.
    ///
    /// The algorithm quickly iterates the logistic map equation to let it "settle" into its orbit,
    /// then checks the nearest distance among the active orbital points.
    ///
    /// # Arguments
    ///
    /// * `r` - The growth rate parameter on the horizontal axis (expected between `min_r` and `max_r`).
    /// * `x` - The population ratio on the vertical axis (expected between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bifurcation_crawler::logistic::LogisticMap;
    /// let map = LogisticMap::new(100, 100);
    ///
    /// // At r=3.0, the attractor bifurcates. Let's check distance to the stable line.
    /// let distance = map.distance_to_attractor(2.9, 0.655);
    /// assert!(distance < 0.05); // Very close to the stable attractor
    /// ```
    pub fn distance_to_attractor(&self, r: f32, x: f32) -> f32 {
        if r < self.min_r || r > self.max_r {
            return 1.0; // Out of bounds is chaos/void
        }

        let mut curr = 0.5;
        // Settle
        for _ in 0..100 {
            curr = r * curr * (1.0 - curr);
        }

        let mut min_dist = 1.0f32;
        // Check orbit
        for _ in 0..100 {
            curr = r * curr * (1.0 - curr);
            let dist = (curr - x).abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
        min_dist
    }
}
