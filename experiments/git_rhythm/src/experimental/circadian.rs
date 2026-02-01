use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum CircadianPhase {
    Night,
    Dawn,
    Day,
    Dusk,
}

pub struct CircadianContext {
    pub timestamp: i64,
}

impl CircadianContext {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }

    pub fn phase(&self) -> CircadianPhase {
        // Timestamp is seconds since epoch.
        // We assume UTC for simplicity in this artistic context.
        // 86400 seconds in a day.
        let seconds_in_day = self.timestamp.rem_euclid(86400);
        let hour = seconds_in_day / 3600;

        match hour {
            5..=7 => CircadianPhase::Dawn,
            8..=17 => CircadianPhase::Day,
            18..=21 => CircadianPhase::Dusk,
            _ => CircadianPhase::Night,
        }
    }

    pub fn theme(&self) -> (Color, &'static str) {
        match self.phase() {
            CircadianPhase::Night => (Color::Rgb(20, 25, 60), "Midnight Oil"),
            CircadianPhase::Dawn => (Color::Rgb(255, 140, 100), "Morning Coffee"),
            CircadianPhase::Day => (Color::Rgb(135, 206, 235), "Flow State"),
            CircadianPhase::Dusk => (Color::Rgb(147, 112, 219), "Evening Crunch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circadian_phases() {
        // Night: 02:00
        let ctx = CircadianContext::new(2 * 3600);
        assert_eq!(ctx.phase(), CircadianPhase::Night);

        // Dawn: 06:00
        let ctx = CircadianContext::new(6 * 3600);
        assert_eq!(ctx.phase(), CircadianPhase::Dawn);

        // Day: 12:00
        let ctx = CircadianContext::new(12 * 3600);
        assert_eq!(ctx.phase(), CircadianPhase::Day);

        // Dusk: 20:00
        let ctx = CircadianContext::new(20 * 3600);
        assert_eq!(ctx.phase(), CircadianPhase::Dusk);
    }

    #[test]
    fn test_theme_colors() {
        let ctx = CircadianContext::new(12 * 3600);
        let (color, mood) = ctx.theme();
        assert_eq!(mood, "Flow State");
        assert_eq!(color, Color::Rgb(135, 206, 235));
    }
}
