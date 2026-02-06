use crate::harvester::MusicalCommit;
use ratatui::style::Color;

pub fn calculate_color(commit: &MusicalCommit) -> Color {
    // 1. Hue from Hash (first byte)
    // Hash is a hex string. Take first 2 chars.
    let hash_slice = if commit.hash.len() >= 2 {
        &commit.hash[0..2]
    } else {
        "00"
    };
    let hash_val = u8::from_str_radix(hash_slice, 16).unwrap_or(0);
    let hue = (hash_val as f32 / 255.0) * 360.0;

    // 2. Lightness from Churn
    // Normalize churn. Cap at 2000 lines.
    // Base lightness 0.5. Max 0.8 (to avoid white-out).
    let churn_norm = (commit.churn as f32 / 2000.0).min(1.0);
    let lightness = 0.5 + (churn_norm * 0.3);

    // 3. Saturation
    // Let's keep it vibrant.
    let saturation = 1.0;

    hsl_to_rgb(hue, saturation, lightness)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r_prime, g_prime, b_prime) = if (0.0..60.0).contains(&h) {
        (c, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, c, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, c, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, c)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r_prime + m) * 255.0) as u8;
    let g = ((g_prime + m) * 255.0) as u8;
    let b = ((b_prime + m) * 255.0) as u8;

    Color::Rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_color_determinism() {
        let commit = MusicalCommit {
            hash: "a1b2c3d4e5f6".to_string(),
            author: "Nova".to_string(),
            timestamp: 1234567890,
            churn: 100,
        };

        let color1 = calculate_color(&commit);
        let color2 = calculate_color(&commit);

        assert_eq!(color1, color2, "Color should be deterministic");
    }

    #[test]
    fn test_hsl_to_rgb() {
        // Red
        let red = hsl_to_rgb(0.0, 1.0, 0.5);
        if let Color::Rgb(r, g, b) = red {
            assert!(r > 200);
            assert!(g < 50);
            assert!(b < 50);
        } else {
            panic!("Should return RGB");
        }
    }
}
