use chrono::{DateTime, Utc};
use std::fmt;

/// Mayan Long Count Date
/// Baktun.Katun.Tun.Uinal.Kin
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MayanDate {
    pub baktun: u32,
    pub katun: u32,
    pub tun: u32,
    pub uinal: u32,
    pub kin: u32,
}

impl MayanDate {
    pub const GMT_CORRELATION: i64 = 584283;

    pub fn new(baktun: u32, katun: u32, tun: u32, uinal: u32, kin: u32) -> Self {
        Self {
            baktun,
            katun,
            tun,
            uinal,
            kin,
        }
    }

    pub fn total_days(&self) -> u64 {
        self.baktun as u64 * 144000
            + self.katun as u64 * 7200
            + self.tun as u64 * 360
            + self.uinal as u64 * 20
            + self.kin as u64
    }

    /// Create a MayanDate from the total number of days since the creation epoch (0.0.0.0.0).
    pub fn from_days(days: u64) -> Self {
        let mut d = days;

        let baktun = d / 144000;
        d %= 144000;

        let katun = d / 7200;
        d %= 7200;

        let tun = d / 360;
        d %= 360;

        let uinal = d / 20;
        d %= 20;

        let kin = d;

        Self {
            baktun: baktun as u32,
            katun: katun as u32,
            tun: tun as u32,
            uinal: uinal as u32,
            kin: kin as u32,
        }
    }
}

impl From<DateTime<Utc>> for MayanDate {
    fn from(dt: DateTime<Utc>) -> Self {
        // Calculate Julian Day Number
        // Chrono's `ordinal` is day of year.
        // Easier to use num_days_from_ce or simliar, but let's stick to Unix timestamp.

        let unix_secs = dt.timestamp();
        let days_since_unix_epoch = unix_secs / 86400;

        // Unix Epoch (1970-01-01) is Julian Day 2440587.5
        // Mayan Creation is JD 584283.
        // Days since creation = (JD_now - JD_creation)
        // Days since creation = (JD_unix + days_since_unix - JD_creation)
        // Days since creation = (2440588 + days_since_unix - 584283) (using integer JD for noon)
        // Let's use the known correlation: 1970-01-01 is 1856305 days since creation.

        // Note: days_since_unix_epoch is i64, could be negative.
        // But Mayan cycle is long enough that we assume post-creation.
        // 1856305 is the number of days from Mayan 0.0.0.0.0 to Unix Epoch.
        let days_since_creation = 1856305 + days_since_unix_epoch;

        // If it's negative, we are before creation, which this struct doesn't handle well (unsigned fields).
        // Let's assume > 0.
        let days = if days_since_creation < 0 { 0 } else { days_since_creation as u64 };

        Self::from_days(days)
    }
}

impl fmt::Display for MayanDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.baktun, self.katun, self.tun, self.uinal, self.kin
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_unix_epoch() {
        let dt = Utc.timestamp_opt(0, 0).unwrap();
        let mayan = MayanDate::from(dt);
        // 1970-01-01 is 12.17.16.7.5
        assert_eq!(mayan.baktun, 12);
        assert_eq!(mayan.katun, 17);
        assert_eq!(mayan.tun, 16);
        assert_eq!(mayan.uinal, 7);
        assert_eq!(mayan.kin, 5);
    }

    #[test]
    fn test_2012_end_of_cycle() {
        // Dec 21, 2012 is 13.0.0.0.0
        // Unix timestamp for 2012-12-21 00:00:00 UTC
        let dt = Utc.with_ymd_and_hms(2012, 12, 21, 0, 0, 0).unwrap();
        let mayan = MayanDate::from(dt);
        assert_eq!(mayan.baktun, 13);
        assert_eq!(mayan.katun, 0);
        assert_eq!(mayan.tun, 0);
        assert_eq!(mayan.uinal, 0);
        assert_eq!(mayan.kin, 0);
    }

    #[test]
    fn test_display() {
        let d = MayanDate::new(13, 0, 0, 0, 0);
        assert_eq!(format!("{}", d), "13.0.0.0.0");
    }

    #[test]
    fn test_round_trip_days() {
        let d = MayanDate::new(13, 0, 0, 0, 0);
        let days = d.total_days();
        let d2 = MayanDate::from_days(days);
        assert_eq!(d, d2);
    }
}
