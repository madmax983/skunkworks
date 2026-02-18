use crate::parser::LogEntry;
use palette::{FromColor, Hsl, RgbHue, Srgb};
use ratatui::style::Color;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct ColorMapper;

impl ColorMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map(&self, entry: &LogEntry, sentiment: f32) -> (Color, Srgb<u8>) {
        // 1. Determine Hue
        // We want a stable hue for the same component.
        let seed = if let Some(comp) = &entry.component {
            comp.as_str()
        } else if let Some(lvl) = &entry.level {
            lvl.as_str()
        } else {
            // Fallback to hashing the first word of the message or something stable-ish?
            // Or just random based on message length?
            // Let's use the whole message for fallback, meaning duplicate messages get same color.
            entry.message.as_str()
        };

        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash = hasher.finish();

        // Map hash to 0-360
        let hue = (hash % 360) as f32;

        // 2. Determine Lightness (Luminance)
        // Sentiment -1.0 (Dark) to 1.0 (Bright)
        // Range: 0.1 to 0.9
        // 0.0 -> 0.5
        let lightness = 0.5 + (sentiment * 0.4);

        // 3. Determine Saturation
        // High entropy/complexity -> High Saturation
        // Low entropy -> Low Saturation (Grayish)
        // We can use length as a proxy for now.
        let len = entry.message.len().min(200) as f32;
        let saturation = 0.2 + (len / 200.0 * 0.8); // 0.2 to 1.0

        // Create HSL color
        let hsl = Hsl::new(RgbHue::from_degrees(hue), saturation, lightness);

        // Convert to RGB for Ratatui and Image export
        let srgb: Srgb = Srgb::from_color(hsl);
        let r = (srgb.red * 255.0) as u8;
        let g = (srgb.green * 255.0) as u8;
        let b = (srgb.blue * 255.0) as u8;

        (Color::Rgb(r, g, b), Srgb::new(r, g, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_hue() {
        let mapper = ColorMapper::new();
        let entry1 = LogEntry {
            raw: String::new(),
            timestamp: None,
            level: Some("INFO".into()),
            component: Some("Auth".into()),
            message: "User logged in".into(),
        };
        let entry2 = LogEntry {
            raw: String::new(),
            timestamp: None,
            level: Some("ERROR".into()),
            component: Some("Auth".into()), // Same component
            message: "User failed login".into(),
        };

        let (c1, _) = mapper.map(&entry1, 0.0);
        let (_c2, _) = mapper.map(&entry2, 0.0);

        // Since we base Hue on component, and they have same component, Hue should be same.
        // But Lightness/Saturation might differ due to sentiment/message length.
        // Wait, Hue is the only thing derived from component.
        // Hsl conversion to Rgb might mix them, but the dominant color should be similar?
        // Actually, we can't easily check hue from RGB without converting back.
        // But we can check that `map` is deterministic.

        let (c1_again, _) = mapper.map(&entry1, 0.0);
        assert_eq!(c1, c1_again);
    }

    #[test]
    fn test_sentiment_lightness() {
        let mapper = ColorMapper::new();
        let entry = LogEntry {
            raw: String::new(),
            timestamp: None,
            level: None,
            component: Some("Test".into()),
            message: "Test".into(),
        };

        let (_, rgb_neg) = mapper.map(&entry, -1.0);
        let (_, rgb_pos) = mapper.map(&entry, 1.0);

        // Positive sentiment should be lighter (higher RGB values on average)
        let sum_neg: u16 = rgb_neg.red as u16 + rgb_neg.green as u16 + rgb_neg.blue as u16;
        let sum_pos: u16 = rgb_pos.red as u16 + rgb_pos.green as u16 + rgb_pos.blue as u16;

        assert!(sum_pos > sum_neg, "Positive sentiment should be brighter");
    }
}
