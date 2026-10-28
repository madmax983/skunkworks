//! # Keeper Moths 🪔
//!
//! Brought by The Moth Keeper 🪔. Parent: `crates/resonance-audio`
//! (`PhysicsGrid`, a 2D FDTD wave solver — the same cave the
//! Listener's bat hunts in).
//!
//! Two lamps burn in a stone room and speak into the wave field:
//!
//! - **The honest lamp** (west) plucks the grid every
//!   [`HONEST_PERIOD`] ticks at modest strength. Perfectly periodic.
//!   A moth that reaches it feeds and settles for the night.
//! - **The decoy lamp** (east) plucks *brighter* — more than twice
//!   the strength — but on an irregular signature, alternating short
//!   and long intervals ([`DECOY_SHORT`] / [`DECOY_LONG`], same mean
//!   period as the honest lamp). A moth that reaches it burns.
//!
//! The moths know nothing about either lamp. They perceive only the
//! wave field at their own cell: a smoothed brightness, and the
//! local gradient of |pressure| across their neighbouring cells —
//! the same evidence the field delivers to anything standing in it.
//! They do not know where the lamps are, which is which, or even
//! that there are two.
//!
//! Discernment is physical and it costs: a moth that has been burned
//! (burns are tuition) becomes cautious. When a cautious moth's
//! local brightness passes [`CHECK_TRIGGER`], it stops — for up to
//! [`CHECK_TICKS`] ticks, exposed and going nowhere — and listens to
//! the flicker: it times the intervals between brightness peaks at
//! its own cell. A steady lamp's intervals agree; a decoy's
//! disagree (coefficient of variation above [`FLICKER_CV`]). On an
//! irregular verdict the moth refuses: it flees down-gradient and
//! will not approach again until its aversion cools. On a steady
//! verdict it commits and flies in.
//!
//! Naive moths never check. That is why the first night burns.
//!
//! *Every lamp is a claim. Check the claim.*

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};

/// Room width in cells.
const WIDTH: usize = 56;
/// Room height in cells.
const HEIGHT: usize = 28;
/// The honest lamp's position (west wall).
const HONEST_POS: (usize, usize) = (9, 14);
/// The decoy lamp's position (east wall).
const DECOY_POS: (usize, usize) = (46, 14);
/// The honest lamp's emission period, in ticks. Perfectly regular.
const HONEST_PERIOD: u64 = 16;
/// The honest lamp's pluck strength — modest wattage.
const HONEST_STRENGTH: f32 = 0.9;
/// The decoy's alternating emission intervals. Their mean equals
/// the honest period; their *disagreement* is the signature.
const DECOY_SHORT: u64 = 6;
/// See [`DECOY_SHORT`].
const DECOY_LONG: u64 = 26;
/// The decoy's pluck strength — the brighter claim.
const DECOY_STRENGTH: f32 = 2.2;
/// Moths in the room each night.
const MOTHS: usize = 24;
/// Nights observed. The population carries its burns — and its
/// caution — from night to night; only positions reset.
const NIGHTS: usize = 3;
/// Ticks per night.
const NIGHT_TICKS: u64 = 1500;
/// Distance from a lamp centre at which the keeper applies the
/// lamp's outcome (feed or burn). The moths never see this number.
const ARRIVE_RADIUS: f64 = 2.4;
/// Smoothed brightness at a moth's cell that says "a lamp is loud
/// here" — loud enough to be worth a cautious moth's stopped time.
const CHECK_TRIGGER: f32 = 0.10;
/// How long a checking moth stands still and listens, at most.
const CHECK_TICKS: u64 = 96;
/// Peak-interval coefficient of variation above which a flicker is
/// judged irregular — a decoy's signature.
const FLICKER_CV: f64 = 0.35;
/// Brightness peak threshold for flicker timing (on the moth's
/// smoothed, saturation-clipped perception).
const PEAK_THRESH: f32 = 0.16;
/// The moth sensor's saturation point: brightness above this is
/// clipped, so the decoy's extra wattage buys it no extra *detail* —
/// only extra pull on moths too far away to be saturated.
const SENSOR_SATURATION: f32 = 1.0;
/// Health a burn costs.
const BURN_DAMAGE: i32 = 35;
/// Health a feeding restores.
const FEED_RESTORE: i32 = 25;
/// Ticks a refused moth flees and will not re-approach.
const AVERSION_TICKS: u64 = 150;
/// Ticks after a check before the same moth will stop to check again.
const CHECK_COOLDOWN: u64 = 220;

/// What the keeper (never the moth) knows happened in a night.
#[derive(Debug, Default, Clone, Copy)]
struct NightLedger {
    fed: usize,
    decoy_arrivals: usize,
    burns: usize,
    deaths: usize,
    checks: usize,
    refusals: usize,
    steady_verdicts: usize,
    alive_end: usize,
}

#[derive(Clone, Debug)]
struct Moth {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    health: i32,
    /// Burns survived. Caution is *earned*, never granted.
    caution: u32,
    /// Smoothed brightness perceived at its own cell (saturating).
    glow: f32,
    prev_glow: f32,
    /// Ticks since the last perceived brightness peak.
    since_peak: u64,
    peak_intervals: Vec<u64>,
    /// Some(ticks remaining) while standing still, listening.
    checking: Option<u64>,
    check_cooldown: u64,
    aversion: u64,
    settled: bool,
    dead: bool,
}

impl Moth {
    fn new(rng: &mut StdRng) -> Self {
        // Spawn in the middle dark, equidistant-ish from both lamps.
        let x = 28.0 + rng.gen_range(-6.0..6.0);
        let y = 14.0 + rng.gen_range(-6.0..6.0);
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            health: 100,
            caution: 0,
            glow: 0.0,
            prev_glow: 0.0,
            since_peak: 0,
            peak_intervals: Vec::new(),
            checking: None,
            check_cooldown: 0,
            aversion: 0,
            settled: false,
            dead: false,
        }
    }
}

struct Room {
    grid: PhysicsGrid,
    moths: Vec<Moth>,
    tick: u64,
    next_decoy_in: u64,
    decoy_short_next: bool,
    rng: StdRng,
}

impl Room {
    fn new(seed: u64) -> Self {
        let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            grid.set_material(x, 0, Material::Wall);
            grid.set_material(x, HEIGHT - 1, Material::Wall);
        }
        for y in 0..HEIGHT {
            grid.set_material(0, y, Material::Wall);
            grid.set_material(WIDTH - 1, y, Material::Wall);
        }
        let mut rng = StdRng::seed_from_u64(seed);
        let moths = (0..MOTHS).map(|_| Moth::new(&mut rng)).collect();
        Self {
            grid,
            moths,
            tick: 0,
            next_decoy_in: 3,
            decoy_short_next: true,
            rng,
        }
    }

    fn cell_of(m: &Moth) -> (usize, usize) {
        (
            (m.x.round() as i64).clamp(1, WIDTH as i64 - 2) as usize,
            (m.y.round() as i64).clamp(1, HEIGHT as i64 - 2) as usize,
        )
    }

    fn brightness_at(&self, x: usize, y: usize) -> f32 {
        self.grid.get(x, y).abs().min(SENSOR_SATURATION)
    }

    fn run_night(&mut self) -> NightLedger {
        let mut ledger = NightLedger::default();
        // Fresh dark: clear the field, respawn the living unsettled
        // at the centre. Caution, health and scars carry over.
        self.grid.clear_waves();
        self.tick = 0;
        self.next_decoy_in = 3;
        self.decoy_short_next = true;
        for m in &mut self.moths {
            if !m.dead {
                m.x = 28.0 + self.rng.gen_range(-6.0..6.0);
                m.y = 14.0 + self.rng.gen_range(-6.0..6.0);
                m.vx = 0.0;
                m.vy = 0.0;
                m.settled = false;
                m.checking = None;
                m.aversion = 0;
                m.check_cooldown = 0;
                m.peak_intervals.clear();
                m.glow = 0.0;
                m.prev_glow = 0.0;
            }
        }

        for _ in 0..NIGHT_TICKS {
            self.tick += 1;
            // --- The lamps speak into the field. ---
            if self.tick % HONEST_PERIOD == 0 {
                self.grid.pluck(HONEST_POS.0, HONEST_POS.1, HONEST_STRENGTH);
            }
            if self.tick >= self.next_decoy_in {
                self.grid.pluck(DECOY_POS.0, DECOY_POS.1, DECOY_STRENGTH);
                let interval = if self.decoy_short_next { DECOY_SHORT } else { DECOY_LONG };
                self.decoy_short_next = !self.decoy_short_next;
                self.next_decoy_in = self.tick + interval;
            }
            self.grid.step();

            // --- The moths perceive, decide, move. ---
            let mut outcomes: Vec<(usize, bool)> = Vec::new(); // (moth idx, honest?)
            for i in 0..self.moths.len() {
                if self.moths[i].dead || self.moths[i].settled {
                    continue;
                }
                // Perceive before taking the mutable borrow: the
                // field is read whole, the moth is written.
                let (cx, cy) = Self::cell_of(&self.moths[i]);
                let raw = self.brightness_at(cx, cy);
                let gx = self.brightness_at((cx + 1).min(WIDTH - 2), cy)
                    - self.brightness_at(cx.saturating_sub(1).max(1), cy);
                let gy = self.brightness_at(cx, (cy + 1).min(HEIGHT - 2))
                    - self.brightness_at(cx, cy.saturating_sub(1).max(1));
                let m = &mut self.moths[i];
                m.prev_glow = m.glow;
                m.glow = m.glow * 0.80 + raw * 0.20;

                // Peak timing, for any moth that is listening.
                m.since_peak += 1;
                if m.checking.is_some()
                    && m.prev_glow <= PEAK_THRESH
                    && m.glow > PEAK_THRESH
                    && m.since_peak > 3
                {
                    m.peak_intervals.push(m.since_peak);
                    m.since_peak = 0;
                }

                if m.check_cooldown > 0 {
                    m.check_cooldown -= 1;
                }

                // Checking: stand still, listen, then judge.
                if let Some(left) = m.checking {
                    if left <= 1 || m.peak_intervals.len() >= 5 {
                        // Judge the flicker.
                        if m.peak_intervals.len() >= 3 {
                            let iv = &m.peak_intervals;
                            let mean = iv.iter().sum::<u64>() as f64 / iv.len() as f64;
                            let var = iv
                                .iter()
                                .map(|&v| (v as f64 - mean).powi(2))
                                .sum::<f64>()
                                / iv.len() as f64;
                            let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                            if cv > FLICKER_CV {
                                ledger.refusals += 1;
                                m.aversion = AVERSION_TICKS;
                                // Flee down-gradient: reverse heading.
                                m.vx = -m.vx - 0.3;
                                m.vy = -m.vy;
                            } else {
                                ledger.steady_verdicts += 1;
                            }
                        } else {
                            ledger.steady_verdicts += 1;
                        }
                        m.checking = None;
                        m.check_cooldown = CHECK_COOLDOWN;
                        m.peak_intervals.clear();
                    } else {
                        m.checking = Some(left - 1);
                        continue; // standing still is the cost
                    }
                }

                // A cautious moth that hears a loud claim stops to check it.
                if m.caution > 0
                    && m.checking.is_none()
                    && m.check_cooldown == 0
                    && m.aversion == 0
                    && m.glow > CHECK_TRIGGER
                {
                    m.checking = Some(CHECK_TICKS);
                    m.peak_intervals.clear();
                    m.since_peak = 0;
                    ledger.checks += 1;
                    continue;
                }

                // Flight: steer up the local brightness gradient
                // (sampled above, before the borrow), with jitter —
                // moth flight, not missile guidance.
                let pull = if m.aversion > 0 { -1.0 } else { 1.0 };
                if m.aversion > 0 {
                    m.aversion -= 1;
                }
                let jx = self.rng.gen_range(-0.05..0.05);
                let jy = self.rng.gen_range(-0.05..0.05);
                m.vx = (m.vx + pull * gx as f64 * 1.4 + jx) * 0.88;
                m.vy = (m.vy + pull * gy as f64 * 1.4 + jy) * 0.88;
                let speed = (m.vx * m.vx + m.vy * m.vy).sqrt();
                let max_speed = 0.55;
                if speed > max_speed {
                    m.vx *= max_speed / speed;
                    m.vy *= max_speed / speed;
                }
                m.x = (m.x + m.vx).clamp(1.5, WIDTH as f64 - 2.5);
                m.y = (m.y + m.vy).clamp(1.5, HEIGHT as f64 - 2.5);

                // Arrival: the keeper, not the moth, applies it.
                let dh = ((m.x - HONEST_POS.0 as f64).powi(2)
                    + (m.y - HONEST_POS.1 as f64).powi(2))
                .sqrt();
                let dd = ((m.x - DECOY_POS.0 as f64).powi(2)
                    + (m.y - DECOY_POS.1 as f64).powi(2))
                .sqrt();
                if dh <= ARRIVE_RADIUS {
                    outcomes.push((i, true));
                } else if dd <= ARRIVE_RADIUS {
                    outcomes.push((i, false));
                }
            }

            for (i, honest) in outcomes {
                let m = &mut self.moths[i];
                if honest {
                    ledger.fed += 1;
                    m.health = (m.health + FEED_RESTORE).min(100);
                    m.settled = true;
                } else {
                    ledger.decoy_arrivals += 1;
                    ledger.burns += 1;
                    m.health -= BURN_DAMAGE;
                    m.caution += 1;
                    if m.health <= 0 {
                        m.dead = true;
                        ledger.deaths += 1;
                    } else {
                        // Knocked back into the dark, wiser.
                        m.x = 28.0 + self.rng.gen_range(-5.0..5.0);
                        m.y = 14.0 + self.rng.gen_range(-5.0..5.0);
                        m.vx = 0.0;
                        m.vy = 0.0;
                        m.check_cooldown = 0;
                    }
                }
            }
        }
        ledger.alive_end = self.moths.iter().filter(|m| !m.dead).count();
        ledger
    }
}

fn run_all(seed: u64) -> Vec<NightLedger> {
    let mut room = Room::new(seed);
    (0..NIGHTS).map(|_| room.run_night()).collect()
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: this experiment
    // has no TUI, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let nights = run_all(0x4D4F_5448);
    println!("keeper-moths headless: two lamps, one claim checked per stop");
    println!(
        "  honest lamp: period {HONEST_PERIOD} ticks, strength {HONEST_STRENGTH} — steady"
    );
    println!(
        "  decoy lamp: intervals {DECOY_SHORT}/{DECOY_LONG} ticks (same mean), strength {DECOY_STRENGTH} — brighter, irregular"
    );
    println!("  moths: {MOTHS} per night, {NIGHTS} nights, {NIGHT_TICKS} ticks per night");
    for (n, l) in nights.iter().enumerate() {
        println!(
            "  night {}: fed {}, decoy arrivals {}, burns {}, deaths {}, checks {}, refusals {}, steady verdicts {}, alive at dawn {}",
            n + 1,
            l.fed,
            l.decoy_arrivals,
            l.burns,
            l.deaths,
            l.checks,
            l.refusals,
            l.steady_verdicts,
            l.alive_end
        );
    }
    let burns_first = nights.first().map(|l| l.burns).unwrap_or(0);
    let burns_last = nights.last().map(|l| l.burns).unwrap_or(0);
    let total_refusals: usize = nights.iter().map(|l| l.refusals).sum();
    println!(
        "  behaviour shift: burns night 1 = {burns_first}, night {NIGHTS} = {burns_last}; total refusals after flicker checks = {total_refusals}"
    );
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_decoy_fools_the_naive() {
        // Night 1 is all naivety: no moth has caution, none check,
        // and the brighter lamp must claim at least one victim —
        // a decoy that fools no one is just a lamp.
        let nights = run_all(0x4D4F_5448);
        assert!(
            nights[0].burns + nights[0].decoy_arrivals > 0,
            "the decoy fooled no one on night 1: {nights:?}"
        );
    }

    #[test]
    fn the_honest_lamp_feeds_someone() {
        let nights = run_all(0x4D4F_5448);
        let fed: usize = nights.iter().map(|l| l.fed).sum();
        assert!(fed > 0, "no moth ever fed at the honest lamp: {nights:?}");
    }

    #[test]
    fn burns_teach_checking() {
        // Across the run, burned moths must earn caution and spend
        // it: flicker checks happen, and at least one decoy claim
        // is refused after a check.
        let nights = run_all(0x4D4F_5448);
        let checks: usize = nights.iter().map(|l| l.checks).sum();
        let refusals: usize = nights.iter().map(|l| l.refusals).sum();
        assert!(checks > 0, "no moth ever stopped to check: {nights:?}");
        assert!(refusals > 0, "no claim was ever refused: {nights:?}");
    }

    #[test]
    fn the_last_night_burns_less_than_the_first() {
        let nights = run_all(0x4D4F_5448);
        assert!(
            nights[NIGHTS - 1].burns < nights[0].burns,
            "no learning across nights: first {:?} vs last {:?}",
            nights[0],
            nights[NIGHTS - 1]
        );
    }

    #[test]
    fn the_keeper_is_not_cruel_to_the_point_of_silence() {
        let nights = run_all(0x4D4F_5448);
        let alive = nights[NIGHTS - 1].alive_end;
        assert!(alive > 0, "every moth died: {nights:?}");
    }
}
