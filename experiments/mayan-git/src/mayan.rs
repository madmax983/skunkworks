use chrono::{DateTime, Utc};
use std::fmt;

/// The Mayan Long Count Calendar.
///
/// The Long Count is a system of counting days (Kin) since the creation date of the current era.
/// The standard correlation (GMT) places the creation date (0.0.0.0.0) at August 11, 3114 BCE.
///
/// Hierarchy:
/// - Kin: 1 day
/// - Uinal: 20 Kin (20 days)
/// - Tun: 18 Uinal (360 days)
/// - Katun: 20 Tun (7,200 days)
/// - Baktun: 20 Katun (144,000 days)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LongCount {
    pub baktun: u32,
    pub katun: u8,
    pub tun: u8,
    pub uinal: u8,
    pub kin: u8,
}

impl LongCount {
    /// The Julian Day Number for the Mayan Epoch (0.0.0.0.0).
    /// Using the GMT correlation (584283).
    pub const EPOCH_JD: i64 = 584283;

    pub fn new(baktun: u32, katun: u8, tun: u8, uinal: u8, kin: u8) -> Self {
        Self { baktun, katun, tun, uinal, kin }
    }

    /// Convert a chrono DateTime to LongCount.
    pub fn from_timestamp(dt: DateTime<Utc>) -> Self {
        // Unix Epoch (1970-01-01) is roughly JD 2440588
        let unix_days = dt.timestamp() / 86400;
        let jd = unix_days + 2440588;

        let mut days_since_epoch = jd - Self::EPOCH_JD;
        if days_since_epoch < 0 {
            days_since_epoch = 0;
        }

        let total_days = days_since_epoch as u64;

        let baktun = (total_days / 144000) as u32;
        let rem1 = total_days % 144000;

        let katun = (rem1 / 7200) as u8;
        let rem2 = rem1 % 7200;

        let tun = (rem2 / 360) as u8;
        let rem3 = rem2 % 360;

        let uinal = (rem3 / 20) as u8;
        let kin = (rem3 % 20) as u8;

        Self { baktun, katun, tun, uinal, kin }
    }

    /// Returns a string representation of the glyphs as a vertical stack of ASCII art blocks.
    /// This is a simplified "Stela".
    pub fn to_stela(&self) -> String {
        let mut s = String::new();
        // Baktun
        s.push_str(&format!("{:^9} (Baktun {})\n", "", self.baktun));
        s.push_str(&Self::draw_glyph(self.baktun as u8));
        s.push('\n');

        // Katun
        s.push_str(&format!("{:^9} (Katun {})\n", "", self.katun));
        s.push_str(&Self::draw_glyph(self.katun));
        s.push('\n');

        // Tun
        s.push_str(&format!("{:^9} (Tun {})\n", "", self.tun));
        s.push_str(&Self::draw_glyph(self.tun));
        s.push('\n');

        // Uinal
        s.push_str(&format!("{:^9} (Uinal {})\n", "", self.uinal));
        s.push_str(&Self::draw_glyph(self.uinal));
        s.push('\n');

        // Kin
        s.push_str(&format!("{:^9} (Kin {})\n", "", self.kin));
        s.push_str(&Self::draw_glyph(self.kin));

        s
    }

    fn draw_glyph(n: u8) -> String {
        // Simple Maya numerals:
        // 0 = Shell (Θ)
        // 1 = Dot (●)
        // 5 = Bar (▬)
        if n == 0 {
            return "    Θ    \n".to_string();
        }
        let mut s = String::new();
        let bars = n / 5;
        let dots = n % 5;

        if dots > 0 {
            let mut dot_line = String::new();
            for _ in 0..dots {
                dot_line.push_str("● ");
            }
            s.push_str(&format!("{:^9}\n", dot_line.trim()));
        }

        for _ in 0..bars {
            s.push_str("▬▬▬▬▬▬▬▬▬\n");
        }
        s
    }
}

impl fmt::Display for LongCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn to_mayan_inline(n: u8) -> String {
            if n == 0 { return "(@)".to_string(); }
            let bars = n / 5;
            let dots = n % 5;
            let mut s = String::new();
            // Bars first or dots first? Standard is dots on top. Inline: dots then bars?
            // Actually, usually dots are on top of bars.
            // Inline: `...||`
            for _ in 0..dots { s.push('.'); }
            for _ in 0..bars { s.push('|'); }
            s
        }
        // Baktun usually displayed as simple number in scholarly text, but we want NO ARABIC.
        // We'll use the inline notation.
        // Note: Baktun can go higher than 19 in deep time, but for Git history (near 2012), it's around 13.
        write!(f, "{}.{}.{}.{}.{}",
            to_mayan_inline(self.baktun as u8),
            to_mayan_inline(self.katun),
            to_mayan_inline(self.tun),
            to_mayan_inline(self.uinal),
            to_mayan_inline(self.kin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_baktun_13_end_of_world() {
        // Dec 21, 2012
        let date = Utc.with_ymd_and_hms(2012, 12, 21, 12, 0, 0).unwrap();
        let lc = LongCount::from_timestamp(date);
        // 13.0.0.0.0
        // 13 = ...||
        // 0 = (@)
        assert_eq!(lc.to_string(), "...||.(@).(@).(@).(@)");
    }

    #[test]
    fn test_day_after_end_of_world() {
        // Dec 22, 2012
        let date = Utc.with_ymd_and_hms(2012, 12, 22, 12, 0, 0).unwrap();
        let lc = LongCount::from_timestamp(date);
        // 13.0.0.0.1
        // 1 = .
        assert_eq!(lc.to_string(), "...||.(@).(@).(@)..");
    }

    #[test]
    fn test_epoch() {
        // Aug 11, 3114 BCE is hard to represent with simple integer timestamp if it overflows i64 or is negative
        // But let's test a known intermediate date.
        // Jan 1, 1970 (Unix Epoch)
        // JD 2440588.
        // Days since 584283 = 1,856,305
        // 1856305 / 144000 = 12 baktuns (1728000)
        // Remainder = 128305
        // 128305 / 7200 = 17 katuns (122400)
        // Remainder = 5905
        // 5905 / 360 = 16 tuns (5760)
        // Remainder = 145
        // 145 / 20 = 7 uinals (140)
        // Remainder = 5 kins
        // So 12.17.16.7.5
        // 12 = ..||
        // 17 = ..|||
        // 16 = .|||
        // 7 = ..|
        // 5 = |
        let date = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();
        let lc = LongCount::from_timestamp(date);
        assert_eq!(lc.to_string(), "..||...|||..|||...|.|");
    }
}
