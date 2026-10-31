//! # Damped Moths 🪔
//!
//! Brought by The Moth Keeper 🪔. The controlled follow-up that
//! `tempo-moths`' falsification named: the SAME three lamps, the
//! SAME moths, the SAME two discernment channels (flicker, then
//! tempo memory), in two rooms that differ only in their walls'
//! appetite.
//!
//! - **The bell room** is `tempo-moths`' room verbatim: stone
//!   walls, `Material::Air` inside (damping 0.999), every claim
//!   ringing until the next one lands on top of it. Same seed —
//!   its ledger must reproduce the principal's exactly, and it
//!   does; that is the control.
//! - **The damped room** lines the inside of the same stone with
//!   a one-cell frame of `Material::Void` (damping 0.5): a wave
//!   that reaches the frame dies there instead of reflecting.
//!   The ringing dies between claims. What reaches a listening
//!   post is, far more nearly, the lamp's own clock.
//!
//! The lamps (see the constants): honest west, period 16,
//! strength 0.9, feeds; irregular decoy east, 6/26 alternating,
//! strength 2.2, burns; slow decoy south, period 48, strength
//! 1.4, perfectly steady, burns — the flicker check waves it
//! through and only tempo could betray it, if the room lets
//! tempo through.
//!
//! Two ledgers. First the medium itself: a *ring-down* — one
//! pluck at the room's centre, counted in ticks until the
//! loudest cell falls silent — how long a single claim keeps
//! talking in each room. (A sterile one-lamp tempo probe was
//! tried first and struck on evidence: a lone lamp's smoothed
//! rise at any post peaks near 0.03–0.06 a tick, at or under
//! the moths' own [`RISE_THRESH`] — the principal's tempo
//! samples were always samples of the composite field, three
//! lamps and their ringing together. So transmission is
//! measured where it actually happens, in the moths' check
//! samples, bucketed keeper-side by nearest lamp.) Then the
//! moths themselves, three nights in each room, and the
//! question the principal's README left open: in the quiet
//! room, do the memorised tempos finally pay — do slow-lamp
//! burns fall while feeding holds?
//!
//! The moths know nothing about any lamp or either room. They
//! perceive only the wave field at their own cell.
//!
//! *Every lamp is a claim. Check the claim — and check whether
//! the room is a bell or a witness.*

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};

/// Room width in cells.
const WIDTH: usize = 56;
/// Room height in cells.
const HEIGHT: usize = 40;
/// The honest lamp's position (west wall).
const HONEST_POS: (usize, usize) = (9, 20);
/// The decoy lamp's position (east wall).
const DECOY_POS: (usize, usize) = (46, 20);
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
/// The slow lamp's position (south wall, nearest the spawning dark).
const SLOW_POS: (usize, usize) = (28, 36);
/// The slow lamp's emission period: perfectly steady, but 3x the
/// honest tempo. Its flicker is regular, so the flicker check alone
/// waves it through; only its tempo could betray it.
const SLOW_PERIOD: u64 = 48;
/// The slow lamp's pluck strength — brighter than honest, dimmer
/// than the irregular decoy.
const SLOW_STRENGTH: f32 = 1.4;
/// Per-tick glow rise that counts as a wavefront onset for the
/// tempo channel. A near lamp's direct wavefront arrives as a
/// sharp rise; reflections and far lamps swell gradually, so
/// intervals between onsets track the near lamp's own emission
/// period instead of the room's resonance.
const RISE_THRESH: f32 = 0.06;
/// Relative deviation from a remembered home tempo beyond which
/// a steady flicker is refused as the wrong tempo.
const TEMPO_TOL: f64 = 0.35;
/// Relative closeness to a remembered burnt tempo within which
/// a steady flicker is recognised and refused.
const BURNT_TEMPO_MATCH: f64 = 0.20;
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
/// Longer than the Keeper's 96: tempo is a slower fact than
/// regularity, and the second channel costs more stopped time.
const CHECK_TICKS: u64 = 160;
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
    tempo_refusals: usize,
    steady_verdicts: usize,
    slow_arrivals: usize,
    slow_burns: usize,
    moths_with_home_tempo: usize,
    moths_with_burnt_tempo: usize,
    alive_end: usize,
    /// Keeper-side transmission ledger: at each completed check
    /// that gathered >= 2 tempo intervals, the sample mean,
    /// bucketed by which lamp stands nearest the listening moth
    /// (0 honest, 1 irregular, 2 slow). The moth never sees the
    /// bucket; the keeper reads what the room delivered.
    tempo_heard_sum: [f64; 3],
    tempo_heard_n: [usize; 3],
    /// Checks whose tempo sample reached the full >= 3 intervals
    /// the judgment itself demands.
    tempo_full_samples: usize,
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
    /// Mean interval of a steady flicker that fed this moth.
    home_tempo: Option<f64>,
    /// Mean interval of a steady flicker that burned this moth.
    burnt_tempo: Option<f64>,
    /// (mean, cv) of this moth's most recent completed check
    /// (the low-threshold flicker sample).
    last_check: Option<(f64, f64)>,
    /// Rise-onset intervals heard during the current check —
    /// the tempo channel's raw evidence.
    tempo_intervals: Vec<u64>,
    /// Rise-onset intervals heard while flying, rolling — on
    /// final approach the nearest lamp's wavefronts dominate
    /// these, so an arrival can memorise the tempo that met it.
    recent_tempo: Vec<u64>,
    recent_since: u64,
    tempo_since_peak: u64,
    /// (mean, cv) of the tempo sample at the most recent check.
    last_tempo: Option<(f64, f64)>,
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
        let y = 20.0 + rng.gen_range(-6.0..6.0);
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            health: 100,
            caution: 0,
            home_tempo: None,
            burnt_tempo: None,
            last_check: None,
            tempo_intervals: Vec::new(),
            recent_tempo: Vec::new(),
            recent_since: 0,
            tempo_since_peak: 0,
            last_tempo: None,
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


/// The room itself. Stone border always; the damped room adds a
/// one-cell `Material::Void` frame just inside the stone, so a
/// wavefront that reaches the wall is absorbed (damping 0.5 per
/// step) instead of reflected back into the room's conversation.
fn build_grid(damped: bool) -> PhysicsGrid {
    let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        grid.set_material(x, 0, Material::Wall);
        grid.set_material(x, HEIGHT - 1, Material::Wall);
    }
    for y in 0..HEIGHT {
        grid.set_material(0, y, Material::Wall);
        grid.set_material(WIDTH - 1, y, Material::Wall);
    }
    if damped {
        for x in 1..WIDTH - 1 {
            grid.set_material(x, 1, Material::Void);
            grid.set_material(x, HEIGHT - 2, Material::Void);
        }
        for y in 1..HEIGHT - 1 {
            grid.set_material(1, y, Material::Void);
            grid.set_material(WIDTH - 2, y, Material::Void);
        }
    }
    grid
}

/// The room's memory for a single claim: pluck the centre once
/// at the decoy's wattage and count the ticks until the loudest
/// cell in the room falls below 0.01 — how long one statement
/// keeps talking after it was made. (A moth's tempo onsets, it
/// turns out, are almost never a lone lamp's wavefront — a single
/// lamp's smoothed rise at any post peaks near 0.03–0.06 a tick,
/// under the [`RISE_THRESH`] — so the principal's tempo samples
/// were always samples of the *composite* field. The ring-down
/// prices that composite's persistence directly.)
fn ring_down(damped: bool) -> u64 {
    let mut grid = build_grid(damped);
    grid.pluck(WIDTH / 2, HEIGHT / 2, DECOY_STRENGTH);
    for tick in 1..=20000u64 {
        grid.step();
        let mut loudest: f32 = 0.0;
        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                let v = grid.get(x, y).abs();
                if v > loudest {
                    loudest = v;
                }
            }
        }
        if loudest < 0.01 {
            return tick;
        }
    }
    20000
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
    fn new(seed: u64, damped: bool) -> Self {
        let grid = build_grid(damped);
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
                m.y = 20.0 + self.rng.gen_range(-6.0..6.0);
                m.vx = 0.0;
                m.vy = 0.0;
                m.settled = false;
                m.checking = None;
                m.aversion = 0;
                m.check_cooldown = 0;
                m.peak_intervals.clear();
                m.tempo_intervals.clear();
                m.recent_tempo.clear();
                m.recent_since = 0;
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
            if self.tick % SLOW_PERIOD == 0 {
                self.grid.pluck(SLOW_POS.0, SLOW_POS.1, SLOW_STRENGTH);
            }
            if self.tick >= self.next_decoy_in {
                self.grid.pluck(DECOY_POS.0, DECOY_POS.1, DECOY_STRENGTH);
                let interval = if self.decoy_short_next { DECOY_SHORT } else { DECOY_LONG };
                self.decoy_short_next = !self.decoy_short_next;
                self.next_decoy_in = self.tick + interval;
            }
            self.grid.step();

            // --- The moths perceive, decide, move. ---
            let mut outcomes: Vec<(usize, u8)> = Vec::new(); // (moth idx, lamp: 0 honest, 1 irregular, 2 slow)
            for i in 0..self.moths.len() {
                if self.moths[i].dead {
                    continue;
                }
                if self.moths[i].settled {
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
                if m.prev_glow <= PEAK_THRESH
                    && m.glow > PEAK_THRESH
                    && m.since_peak > 3
                {
                    if m.checking.is_some() {
                        m.peak_intervals.push(m.since_peak);
                    }
                    m.since_peak = 0;
                }
                m.recent_since += 1;
                if m.glow - m.prev_glow > RISE_THRESH && m.recent_since > 3 {
                    m.recent_tempo.push(m.recent_since);
                    if m.recent_tempo.len() > 6 {
                        m.recent_tempo.remove(0);
                    }
                    m.recent_since = 0;
                }

                if m.check_cooldown > 0 {
                    m.check_cooldown -= 1;
                }

                // Checking: stand still, listen, then judge.
                if let Some(left) = m.checking {
                    m.tempo_since_peak += 1;
                    if m.glow - m.prev_glow > RISE_THRESH
                        && m.tempo_since_peak > 3
                    {
                        m.tempo_intervals.push(m.tempo_since_peak);
                        m.tempo_since_peak = 0;
                    }
                    if left <= 1 || (m.peak_intervals.len() >= 5 && m.tempo_intervals.len() >= 3) {
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
                            let (tmean, tcv) = interval_stats(&m.tempo_intervals);
                            if m.tempo_intervals.len() >= 3 {
                                m.last_tempo = Some((tmean, tcv));
                                ledger.tempo_full_samples += 1;
                            }
                            if m.tempo_intervals.len() >= 2 {
                                let (bmean, _) = interval_stats_min(&m.tempo_intervals, 2);
                                let dh2 = (m.x - HONEST_POS.0 as f64).powi(2)
                                    + (m.y - HONEST_POS.1 as f64).powi(2);
                                let dd2 = (m.x - DECOY_POS.0 as f64).powi(2)
                                    + (m.y - DECOY_POS.1 as f64).powi(2);
                                let ds2 = (m.x - SLOW_POS.0 as f64).powi(2)
                                    + (m.y - SLOW_POS.1 as f64).powi(2);
                                let bucket = if dh2 <= dd2 && dh2 <= ds2 {
                                    0
                                } else if dd2 <= ds2 {
                                    1
                                } else {
                                    2
                                };
                                ledger.tempo_heard_sum[bucket] += bmean;
                                ledger.tempo_heard_n[bucket] += 1;
                            }
                            m.last_check = Some((mean, cv));
                            if cv > FLICKER_CV {
                                ledger.refusals += 1;
                                m.aversion = AVERSION_TICKS;
                                // Flee down-gradient: reverse heading.
                                m.vx = -m.vx - 0.3;
                                m.vy = -m.vy;
                            } else if m.tempo_intervals.len() >= 3 && tempo_refused(m, tmean) {
                                ledger.refusals += 1;
                                ledger.tempo_refusals += 1;
                                m.aversion = AVERSION_TICKS;
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
                    m.tempo_intervals.clear();
                    m.tempo_since_peak = 0;
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
                let ds = ((m.x - SLOW_POS.0 as f64).powi(2)
                    + (m.y - SLOW_POS.1 as f64).powi(2))
                .sqrt();
                if dh <= ARRIVE_RADIUS {
                    outcomes.push((i, 0));
                } else if dd <= ARRIVE_RADIUS {
                    outcomes.push((i, 1));
                } else if ds <= ARRIVE_RADIUS {
                    outcomes.push((i, 2));
                }
            }

            for (i, lamp) in outcomes {
                // What the moth itself heard on the way in: the mean
                // and regularity of its rolling recent intervals.
                let (tmean_a, _tcv_a) = if self.moths[i].recent_tempo.len() >= 3 {
                    interval_stats(&self.moths[i].recent_tempo)
                } else {
                    self.moths[i].last_tempo.unwrap_or((0.0, 1.0))
                };
                let m = &mut self.moths[i];
                if lamp == 0 {
                    ledger.fed += 1;
                    m.health = (m.health + FEED_RESTORE).min(100);
                    m.settled = true;
                    if tmean_a > 0.0 {
                        m.home_tempo = Some(tmean_a);
                    }
                } else {
                    if lamp == 2 {
                        ledger.slow_arrivals += 1;
                        ledger.slow_burns += 1;
                        if tmean_a > 0.0 {
                            // A steady flicker burned it: remember
                            // the tempo that did it. (An irregular
                            // burn teaches flicker caution only; its
                            // mean tempo is the honest tempo, and
                            // memorising it would poison the channel.)
                            m.burnt_tempo = Some(tmean_a);
                        }
                    } else {
                        ledger.decoy_arrivals += 1;
                    }
                    ledger.burns += 1;
                    m.health -= BURN_DAMAGE;
                    m.caution += 1;
                    if m.health <= 0 {
                        m.dead = true;
                        ledger.deaths += 1;
                    } else {
                        // Knocked back into the dark, wiser.
                        m.x = 28.0 + self.rng.gen_range(-5.0..5.0);
                        m.y = 20.0 + self.rng.gen_range(-5.0..5.0);
                        m.vx = 0.0;
                        m.vy = 0.0;
                        m.check_cooldown = 0;
                    }
                }
            }
        }
        ledger.alive_end = self.moths.iter().filter(|m| !m.dead).count();
        ledger.moths_with_home_tempo = self.moths.iter().filter(|m| m.home_tempo.is_some()).count();
        ledger.moths_with_burnt_tempo = self.moths.iter().filter(|m| m.burnt_tempo.is_some()).count();
        ledger
    }
}

fn interval_stats(iv: &[u64]) -> (f64, f64) {
    interval_stats_min(iv, 3)
}

fn interval_stats_min(iv: &[u64], min: usize) -> (f64, f64) {
    if iv.len() < min {
        return (0.0, 0.0);
    }
    let mean = iv.iter().sum::<u64>() as f64 / iv.len() as f64;
    let var = iv.iter().map(|&v| (v as f64 - mean).powi(2)).sum::<f64>() / iv.len() as f64;
    let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
    (mean, cv)
}

/// The second discernment channel. A steady flicker is refused on
/// tempo if it disagrees with the tempo that once fed this moth, or
/// — for a moth that has never fed — if it matches the tempo of a
/// steady flicker that once burned it.
fn tempo_refused(m: &Moth, mean: f64) -> bool {
    if mean <= 0.0 {
        return false;
    }
    if let Some(home) = m.home_tempo {
        return (mean - home).abs() / home > TEMPO_TOL;
    }
    if let Some(burnt) = m.burnt_tempo {
        return (mean - burnt).abs() / burnt <= BURNT_TEMPO_MATCH;
    }
    false
}

fn run_all(seed: u64, damped: bool) -> Vec<NightLedger> {
    let mut room = Room::new(seed, damped);
    (0..NIGHTS).map(|_| room.run_night()).collect()
}

fn print_ledger(label: &str, nights: &[NightLedger]) {
    println!("  [{label}]");
    for (n, l) in nights.iter().enumerate() {
        println!(
            "  night {}: fed {}, irregular arrivals {}, slow arrivals {}, burns {} (slow {}), deaths {}, checks {}, refusals {} (tempo {}), steady verdicts {}, moths holding a home tempo {}, moths holding a burnt tempo {}, alive at dawn {}",
            n + 1,
            l.fed,
            l.decoy_arrivals,
            l.slow_arrivals,
            l.burns,
            l.slow_burns,
            l.deaths,
            l.checks,
            l.refusals,
            l.tempo_refusals,
            l.steady_verdicts,
            l.moths_with_home_tempo,
            l.moths_with_burnt_tempo,
            l.alive_end
        );
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: this experiment
    // has no TUI, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    println!("damped-moths headless: tempo-moths' three lamps and two channels, in a bell and in a damped room");
    println!(
        "  honest lamp: period {HONEST_PERIOD} ticks, strength {HONEST_STRENGTH} — steady, feeds"
    );
    println!(
        "  decoy lamp: intervals {DECOY_SHORT}/{DECOY_LONG} ticks (same mean), strength {DECOY_STRENGTH} — brighter, irregular, burns"
    );
    println!(
        "  slow lamp: period {SLOW_PERIOD} ticks, strength {SLOW_STRENGTH} — steady, wrong tempo, burns"
    );
    println!(
        "  ring-down (one centre pluck at decoy wattage, ticks until the loudest cell < 0.01): bell {} ticks, damped {} ticks",
        ring_down(false),
        ring_down(true)
    );
    println!("  moths: {MOTHS} per night, {NIGHTS} nights, {NIGHT_TICKS} ticks per night, same seed both rooms");
    let bell = run_all(0x4D4F_5448, false);
    print_ledger("bell room (tempo-moths' room, the control)", &bell);
    let damp = run_all(0x4D4F_5448, true);
    print_ledger("damped room (Void absorbing frame inside the stone)", &damp);
    let bf: usize = bell.iter().map(|l| l.fed).sum();
    let df: usize = damp.iter().map(|l| l.fed).sum();
    let bs: usize = bell.iter().map(|l| l.slow_burns).sum();
    let ds: usize = damp.iter().map(|l| l.slow_burns).sum();
    let bt: usize = bell.iter().map(|l| l.tempo_refusals).sum();
    let dt: usize = damp.iter().map(|l| l.tempo_refusals).sum();
    println!(
        "  comparison: fed bell {bf} vs damped {df}; slow burns bell {bs} vs damped {ds}; tempo refusals bell {bt} vs damped {dt}"
    );
    for (label, nights) in [("bell", &bell), ("damped", &damp)] {
        let mut sum = [0.0f64; 3];
        let mut n = [0usize; 3];
        let mut full = 0usize;
        for l in nights.iter() {
            for b in 0..3 {
                sum[b] += l.tempo_heard_sum[b];
                n[b] += l.tempo_heard_n[b];
            }
            full += l.tempo_full_samples;
        }
        let mean = |b: usize| if n[b] > 0 { sum[b] / n[b] as f64 } else { 0.0 };
        println!(
            "  heard tempo at checks [{label}] (mean onset interval of the check sample, bucketed by nearest lamp; emitted honest 16, slow 48): nearest honest {:.2} ({} samples), nearest decoy {:.2} ({}), nearest slow {:.2} ({}); checks with a full >= 3-interval tempo sample: {}",
            mean(0), n[0], mean(1), n[1], mean(2), n[2], full
        );
    }
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bell_room_is_the_principal() {
        // The control must hold: undamped, same seed, the ledger
        // is tempo-moths' own (fed 12/9/5, slow burns 7/8/6).
        let bell = run_all(0x4D4F_5448, false);
        let fed: Vec<usize> = bell.iter().map(|l| l.fed).collect();
        let slow: Vec<usize> = bell.iter().map(|l| l.slow_burns).collect();
        assert_eq!(fed, vec![12, 9, 5], "bell room diverged from the principal: {bell:?}");
        assert_eq!(slow, vec![7, 8, 6], "bell room diverged from the principal: {bell:?}");
    }

    #[test]
    fn both_decoys_fool_the_naive_in_the_damped_room() {
        let damp = run_all(0x4D4F_5448, true);
        assert!(damp[0].decoy_arrivals > 0, "irregular decoy fooled no one night 1: {damp:?}");
        assert!(damp[0].slow_arrivals > 0, "slow lamp fooled no one night 1: {damp:?}");
    }

    #[test]
    fn the_damped_room_falls_silent_sooner() {
        // The medium's own ledger: one claim keeps talking for
        // strictly fewer ticks in the damped room than in the bell.
        let bell = ring_down(false);
        let damp = ring_down(true);
        assert!(damp < bell, "damped ring-down {damp} not shorter than bell {bell}");
        assert!(damp > 0, "the damped room swallowed the pluck whole");
    }

    #[test]
    fn slow_burns_fall_in_the_damped_room() {
        // The succession's question, answered by the ledger as
        // found: fewer moths burn at the steady slow lamp across
        // the three nights than in the bell.
        let bell = run_all(0x4D4F_5448, false);
        let damp = run_all(0x4D4F_5448, true);
        let bs: usize = bell.iter().map(|l| l.slow_burns).sum();
        let ds: usize = damp.iter().map(|l| l.slow_burns).sum();
        assert!(ds < bs, "damped slow burns {ds} did not fall below bell {bs}");
    }

    #[test]
    fn the_keeper_is_not_cruel_to_the_point_of_silence() {
        let damp = run_all(0x4D4F_5448, true);
        assert!(damp[NIGHTS - 1].alive_end > 0, "every damped moth died: {damp:?}");
        let bell = run_all(0x4D4F_5448, false);
        assert!(bell[NIGHTS - 1].alive_end > 0, "every bell moth died: {bell:?}");
    }
}

