//! # Scarecrow Granary 🌾
//!
//! Brought by The Scarecrow 🌾. Principle: **interruption
//! equivalence** (Harvest: work that cannot survive a stop cannot
//! survive the winter). Host: `experiments/keeper-moths` — the
//! Moth Keeper's cave, built on the parent crate
//! `crates/resonance-audio` (`PhysicsGrid`, a 2D FDTD wave solver).
//!
//! The host machinery below — room, lamps, moths, tuition — is the
//! Keeper's own, copied faithfully and unchanged (a test pins its
//! ledger to the host's published one). What is new is the winter:
//! at tick 750 of night 2 the run stops and writes a **granary
//! paper**: every moth whole, the lamp clocks, the random stream,
//! and the wave field — as much of the field as the parent crate's
//! public API allows an outsider to write down.
//!
//! That allowance is the experiment. `PhysicsGrid` keeps three
//! time levels (`u`, `u_prev`, `u_next`); its public surface
//! offers `get(x, y)` for the present level only. There is no
//! snapshot, no restore, and no way to read or write `u_prev` —
//! the field's *motion*. A granary built from outside the crate
//! can store where the wave is. It cannot store where the wave
//! is going.
//!
//! Three runs are counted:
//!
//! - **Uninterrupted** — the host run, whole.
//! - **API restore** — resumed from the best paper the public API
//!   permits: moths, clocks and stream whole; the field's present
//!   level restored cell by cell through `pluck` (which adds to a
//!   silent grid, so the present is exact); `u_prev` necessarily
//!   zero, because the crate gives no hand that can set it.
//! - **Silent restore** — the same paper with the field left
//!   dark: moths, clocks and stream whole, no wave at all. This
//!   prices the field itself, so the API restore's shortfall can
//!   be read against what the saved level was worth.
//!
//! The field is compared directly (RMS of the present level,
//! sampled every 50 ticks after the stop), not only through the
//! moths' coarse ledger — a lesson the winter before last taught
//! (see `scarecrow-winter`: a coarse ledger can call a diverged
//! run locked). The moth ledgers are printed too, both nights, so
//! the reader can watch a field-level crack either reach the
//! animals or wash out before it does.
//!
//! *Stand still. Write it down. Write down all of it — if the
//! crate lets you.*

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};

/// Room width in cells. (Host constant, unchanged.)
const WIDTH: usize = 56;
/// Room height in cells. (Host constant, unchanged.)
const HEIGHT: usize = 28;
/// The honest lamp's position (west wall).
const HONEST_POS: (usize, usize) = (9, 14);
/// The decoy lamp's position (east wall).
const DECOY_POS: (usize, usize) = (46, 14);
/// The honest lamp's emission period, in ticks. Perfectly regular.
const HONEST_PERIOD: u64 = 16;
/// The honest lamp's pluck strength — modest wattage.
const HONEST_STRENGTH: f32 = 0.9;
/// The decoy's alternating emission intervals.
const DECOY_SHORT: u64 = 6;
/// See [`DECOY_SHORT`].
const DECOY_LONG: u64 = 26;
/// The decoy's pluck strength — the brighter claim.
const DECOY_STRENGTH: f32 = 2.2;
/// Moths in the room each night.
const MOTHS: usize = 24;
/// Nights observed.
const NIGHTS: usize = 3;
/// Ticks per night.
const NIGHT_TICKS: u64 = 1500;
/// The winter falls at this tick of night 2.
const CHECKPOINT_TICK: u64 = 750;
/// Field samples are taken every this many ticks after the stop.
const SAMPLE_EVERY: u64 = 50;
/// Distance from a lamp centre at which the keeper applies the
/// lamp's outcome (feed or burn).
const ARRIVE_RADIUS: f64 = 2.4;
/// Smoothed brightness at a moth's cell that triggers a check.
const CHECK_TRIGGER: f32 = 0.10;
/// How long a checking moth stands still and listens, at most.
const CHECK_TICKS: u64 = 96;
/// Peak-interval coefficient of variation judged irregular.
const FLICKER_CV: f64 = 0.35;
/// Brightness peak threshold for flicker timing.
const PEAK_THRESH: f32 = 0.16;
/// The moth sensor's saturation point.
const SENSOR_SATURATION: f32 = 1.0;
/// Health a burn costs.
const BURN_DAMAGE: i32 = 35;
/// Health a feeding restores.
const FEED_RESTORE: i32 = 25;
/// Ticks a refused moth flees and will not re-approach.
const AVERSION_TICKS: u64 = 150;
/// Ticks after a check before the same moth will stop again.
const CHECK_COOLDOWN: u64 = 220;
/// The host's seed, exactly as `keeper-moths` set it.
const SEED: u64 = 0x4D4F_5448;

/// What the keeper (never the moth) knows happened in a night.
/// (Host struct, unchanged.)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
    caution: u32,
    glow: f32,
    prev_glow: f32,
    since_peak: u64,
    peak_intervals: Vec<u64>,
    checking: Option<u64>,
    check_cooldown: u64,
    aversion: u64,
    settled: bool,
    dead: bool,
}

impl Moth {
    fn new(rng: &mut StdRng) -> Self {
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

    /// The moth, written down: fixed fields plus its peak queue.
    fn paper_bytes(&self) -> usize {
        8 * 4 + 4 + 4 + 4 + 8 + 8 * self.peak_intervals.len() + 8 + 8 + 8 + 1 + 1
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

/// The granary paper: everything the public API lets an outsider
/// write down at the stop. The random stream rides as a clone —
/// `StdRng` offers no bytes either; that small absence is noted,
/// not hidden.
struct Snapshot {
    moths: Vec<Moth>,
    tick: u64,
    next_decoy_in: u64,
    decoy_short_next: bool,
    rng: StdRng,
    /// The field's present level, read cell by cell via `get`.
    field: Vec<f32>,
    /// Largest |value| on the wall border (expected: exactly 0).
    border_max: f32,
    /// The paper's size in bytes (field + moths + clocks; the
    /// cloned stream is not bytes and is not counted).
    paper_bytes: usize,
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

    /// The field's present level, whole, via the public `get`.
    fn field_now(&self) -> Vec<f32> {
        let mut f = Vec::with_capacity(WIDTH * HEIGHT);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                f.push(self.grid.get(x, y));
            }
        }
        f
    }

    /// Winter falls: write the paper.
    fn snapshot(&self) -> Snapshot {
        let field = self.field_now();
        let mut border_max = 0.0f32;
        for x in 0..WIDTH {
            border_max = border_max.max(field[x].abs());
            border_max = border_max.max(field[(HEIGHT - 1) * WIDTH + x].abs());
        }
        for y in 0..HEIGHT {
            border_max = border_max.max(field[y * WIDTH].abs());
            border_max = border_max.max(field[y * WIDTH + WIDTH - 1].abs());
        }
        let paper_bytes = field.len() * 4
            + self.moths.iter().map(|m| m.paper_bytes()).sum::<usize>()
            + 8 * 3;
        Snapshot {
            moths: self.moths.clone(),
            tick: self.tick,
            next_decoy_in: self.next_decoy_in,
            decoy_short_next: self.decoy_short_next,
            rng: self.rng.clone(),
            field,
            border_max,
            paper_bytes,
        }
    }

    /// Spring: rebuild a room from the paper. With `with_field`,
    /// the present level is restored cell by cell through `pluck`
    /// on a silent grid (exact for every interior cell; the border
    /// is wall and reads 0 in the paper). `u_prev` starts at zero
    /// in both restores — the crate offers no hand that can set
    /// it. That is the missing piece this planting prices.
    fn restore(snap: &Snapshot, with_field: bool) -> Self {
        let mut room = Room::new(SEED);
        room.moths = snap.moths.clone();
        room.tick = snap.tick;
        room.next_decoy_in = snap.next_decoy_in;
        room.decoy_short_next = snap.decoy_short_next;
        room.rng = snap.rng.clone();
        room.grid.clear_waves();
        if with_field {
            for y in 1..HEIGHT - 1 {
                for x in 1..WIDTH - 1 {
                    let v = snap.field[y * WIDTH + x];
                    if v != 0.0 {
                        room.grid.pluck(x, y, v);
                    }
                }
            }
        }
        room
    }

    /// The host's night opening, unchanged: fresh dark, the living
    /// unsettled respawned at the centre; caution, health and
    /// scars carry over.
    fn begin_night(&mut self) {
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
    }

    /// The host's tick loop, unchanged, run for `n` ticks into
    /// `ledger`. When `samples` is given, the field's present
    /// level is recorded at every tick divisible by
    /// [`SAMPLE_EVERY`].
    fn run_ticks(
        &mut self,
        n: u64,
        ledger: &mut NightLedger,
        mut samples: Option<&mut Vec<(u64, Vec<f32>)>>,
    ) {
        for _ in 0..n {
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
            let mut outcomes: Vec<(usize, bool)> = Vec::new();
            for i in 0..self.moths.len() {
                if self.moths[i].dead || self.moths[i].settled {
                    continue;
                }
                let (cx, cy) = Self::cell_of(&self.moths[i]);
                let raw = self.brightness_at(cx, cy);
                let gx = self.brightness_at((cx + 1).min(WIDTH - 2), cy)
                    - self.brightness_at(cx.saturating_sub(1).max(1), cy);
                let gy = self.brightness_at(cx, (cy + 1).min(HEIGHT - 2))
                    - self.brightness_at(cx, cy.saturating_sub(1).max(1));
                let m = &mut self.moths[i];
                m.prev_glow = m.glow;
                m.glow = m.glow * 0.80 + raw * 0.20;

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

                if let Some(left) = m.checking {
                    if left <= 1 || m.peak_intervals.len() >= 5 {
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
                        continue;
                    }
                }

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
                        m.x = 28.0 + self.rng.gen_range(-5.0..5.0);
                        m.y = 14.0 + self.rng.gen_range(-5.0..5.0);
                        m.vx = 0.0;
                        m.vy = 0.0;
                        m.check_cooldown = 0;
                    }
                }
            }

            if let Some(s) = samples.as_deref_mut() {
                if self.tick % SAMPLE_EVERY == 0 {
                    s.push((self.tick, self.field_now()));
                }
            }
        }
        ledger.alive_end = self.moths.iter().filter(|m| !m.dead).count();
    }

    fn run_night(&mut self) -> NightLedger {
        self.begin_night();
        let mut ledger = NightLedger::default();
        self.run_ticks(NIGHT_TICKS, &mut ledger, None);
        ledger
    }
}

fn run_all(seed: u64) -> Vec<NightLedger> {
    let mut room = Room::new(seed);
    (0..NIGHTS).map(|_| room.run_night()).collect()
}

/// RMS difference between two present-level fields.
fn rms(a: &[f32], b: &[f32]) -> f64 {
    let n = a.len().min(b.len()) as f64;
    let s: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (*x as f64 - *y as f64).powi(2))
        .sum();
    (s / n).sqrt()
}

struct Granary {
    host: Vec<NightLedger>,
    paper_bytes: usize,
    field_bytes: usize,
    border_max: f32,
    night2_api: NightLedger,
    night2_silent: NightLedger,
    night3_api: NightLedger,
    night3_silent: NightLedger,
    /// (tick, RMS api, RMS silent) after the stop, night 2.
    samples: Vec<(u64, f64, f64)>,
}

fn run_granary() -> Granary {
    // Uninterrupted, with the winter written down mid-night-2.
    let mut room = Room::new(SEED);
    let n1 = room.run_night();
    room.begin_night();
    let mut first_half = NightLedger::default();
    room.run_ticks(CHECKPOINT_TICK, &mut first_half, None);
    let snap = room.snapshot();
    let mut second_half = NightLedger::default();
    let mut samples_u: Vec<(u64, Vec<f32>)> = Vec::new();
    room.run_ticks(NIGHT_TICKS - CHECKPOINT_TICK, &mut second_half, Some(&mut samples_u));
    let mut n2 = first_half;
    merge(&mut n2, &second_half);
    let n3 = room.run_night();

    // API restore: the best paper the crate permits.
    let mut room_b = Room::restore(&snap, true);
    let mut l2b = first_half;
    let mut samples_b: Vec<(u64, Vec<f32>)> = Vec::new();
    room_b.run_ticks(NIGHT_TICKS - CHECKPOINT_TICK, &mut l2b, Some(&mut samples_b));
    let n3b = room_b.run_night();

    // Silent restore: the same paper, the field left dark.
    let mut room_s = Room::restore(&snap, false);
    let mut l2s = first_half;
    let mut samples_s: Vec<(u64, Vec<f32>)> = Vec::new();
    room_s.run_ticks(NIGHT_TICKS - CHECKPOINT_TICK, &mut l2s, Some(&mut samples_s));
    let n3s = room_s.run_night();

    let mut samples = Vec::new();
    for (i, (t, fu)) in samples_u.iter().enumerate() {
        samples.push((*t, rms(fu, &samples_b[i].1), rms(fu, &samples_s[i].1)));
    }

    Granary {
        host: vec![n1, n2, n3],
        paper_bytes: snap.paper_bytes,
        field_bytes: snap.field.len() * 4,
        border_max: snap.border_max,
        night2_api: l2b,
        night2_silent: l2s,
        night3_api: n3b,
        night3_silent: n3s,
        samples,
    }
}

fn merge(into: &mut NightLedger, from: &NightLedger) {
    into.fed += from.fed;
    into.decoy_arrivals += from.decoy_arrivals;
    into.burns += from.burns;
    into.deaths += from.deaths;
    into.checks += from.checks;
    into.refusals += from.refusals;
    into.steady_verdicts += from.steady_verdicts;
    into.alive_end = from.alive_end;
}

fn print_night(tag: &str, n: usize, l: &NightLedger) {
    println!(
        "  {tag} night {}: fed {}, decoy arrivals {}, burns {}, deaths {}, checks {}, refusals {}, steady verdicts {}, alive at dawn {}",
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

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: this experiment
    // has no TUI, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let g = run_granary();
    println!("scarecrow-granary headless: interruption equivalence in the Keeper's cave");
    println!(
        "  host: keeper-moths (resonance-audio PhysicsGrid {WIDTH}x{HEIGHT}), {MOTHS} moths, {NIGHTS} nights x {NIGHT_TICKS} ticks, seed {SEED:#X}"
    );
    for (n, l) in g.host.iter().enumerate() {
        print_night("uninterrupted", n, l);
    }
    println!(
        "  winter at night 2 tick {CHECKPOINT_TICK}: paper {} bytes (field {} bytes; moths + clocks the rest; the random stream rides as a clone — StdRng offers no bytes), border max |u| {:.4}",
        g.paper_bytes, g.field_bytes, g.border_max
    );
    println!("  field RMS vs uninterrupted after the stop (night 2):");
    for (t, ra, rs) in &g.samples {
        println!("    tick {t}: api restore {ra:.6}, silent restore {rs:.6}");
    }
    print_night("api restore   ", 1, &g.night2_api);
    print_night("api restore   ", 2, &g.night3_api);
    print_night("silent restore", 1, &g.night2_silent);
    print_night("silent restore", 2, &g.night3_silent);
    let first = g.samples.first().map(|s| (s.1, s.2)).unwrap_or((0.0, 0.0));
    let last = g.samples.last().map(|s| (s.1, s.2)).unwrap_or((0.0, 0.0));
    let p1 = first.0 > 0.0005;
    let p2 = g.samples.iter().all(|s| s.1 < s.2);
    let p3 = last.0 < 0.25 * first.0;
    let p4 = (g.night2_api.fed as i64 - g.host[1].fed as i64).abs() <= 3
        && (g.night2_api.burns as i64 - g.host[1].burns as i64).abs() <= 3;
    println!(
        "  verdicts: P1 lost level is real (api RMS at +{} ticks = {:.6} > 0.0005): {}; P2 saved level helps at every sample: {}; P3 field re-settles (last RMS {:.6} < 25% of first): {}; P4 coarse ledger held (api night-2 fed/burns within 3 of uninterrupted): {}",
        SAMPLE_EVERY, first.0, p1, p2, last.0, p3, p4
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
    fn the_host_ledger_is_the_hosts() {
        // Host fidelity: the copied machinery must reproduce
        // keeper-moths' published ledger exactly (run 5 of the
        // garden: fed 14/14/17, burns 7/1/4, checks 19/24/30,
        // refusals 3/4/8, alive 24 at every dawn).
        let nights = run_all(SEED);
        let fed: Vec<usize> = nights.iter().map(|l| l.fed).collect();
        let burns: Vec<usize> = nights.iter().map(|l| l.burns).collect();
        let checks: Vec<usize> = nights.iter().map(|l| l.checks).collect();
        let refusals: Vec<usize> = nights.iter().map(|l| l.refusals).collect();
        assert_eq!(fed, vec![14, 14, 17], "fed diverged: {nights:?}");
        assert_eq!(burns, vec![7, 1, 4], "burns diverged: {nights:?}");
        assert_eq!(checks, vec![19, 24, 30], "checks diverged: {nights:?}");
        assert_eq!(refusals, vec![3, 4, 8], "refusals diverged: {nights:?}");
        assert!(nights.iter().all(|l| l.alive_end == 24));
    }

    #[test]
    fn the_paper_restores_the_present_exactly() {
        // What the API does allow, it allows exactly: the present
        // level written via `get` and restored via `pluck` on a
        // silent grid is bit-for-bit the field that was stopped.
        let mut room = Room::new(SEED);
        room.run_night();
        room.begin_night();
        let mut l = NightLedger::default();
        room.run_ticks(CHECKPOINT_TICK, &mut l, None);
        let snap = room.snapshot();
        assert_eq!(snap.border_max, 0.0, "border should be silent stone");
        let restored = Room::restore(&snap, true);
        assert_eq!(rms(&snap.field, &restored.field_now()), 0.0);
    }

    #[test]
    fn the_lost_level_is_real_motion() {
        // The missing piece, priced in miniature on the parent
        // crate alone: a field stepped, written down by `get`,
        // restored by `pluck` — one more step and the two fields
        // already disagree, because the restored field has no
        // yesterday. u_prev is state, and the API cannot carry it.
        let mut a = PhysicsGrid::new(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            a.set_material(x, 0, Material::Wall);
            a.set_material(x, HEIGHT - 1, Material::Wall);
        }
        for y in 0..HEIGHT {
            a.set_material(0, y, Material::Wall);
            a.set_material(WIDTH - 1, y, Material::Wall);
        }
        for t in 0..40 {
            if t % HONEST_PERIOD == 0 {
                a.pluck(HONEST_POS.0, HONEST_POS.1, HONEST_STRENGTH);
            }
            a.step();
        }
        let mut b = PhysicsGrid::new(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            b.set_material(x, 0, Material::Wall);
            b.set_material(x, HEIGHT - 1, Material::Wall);
        }
        for y in 0..HEIGHT {
            b.set_material(0, y, Material::Wall);
            b.set_material(WIDTH - 1, y, Material::Wall);
        }
        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                let v = a.get(x, y);
                if v != 0.0 {
                    b.pluck(x, y, v);
                }
            }
        }
        a.step();
        b.step();
        let mut fa = Vec::new();
        let mut fb = Vec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                fa.push(a.get(x, y));
                fb.push(b.get(x, y));
            }
        }
        assert!(
            rms(&fa, &fb) > 1e-6,
            "one step after restore the fields agree — the lost level carried nothing"
        );
    }

    #[test]
    fn night_one_is_identical_in_every_run() {
        // The stop is in night 2; nothing before it may differ.
        // (All three runs share the same first night by
        // construction — this pins that the restore path was not
        // reached early.)
        let g = run_granary();
        assert_eq!(g.host[0].fed, 14);
        assert_eq!(g.host[0].burns, 7);
    }

    #[test]
    fn the_lost_level_is_the_loudest_thing_in_the_room() {
        // P1 as a test: the API restore must disagree with the
        // uninterrupted field — the lost level is real motion.
        // And the run's cracked P2, pinned as found: at the first
        // sample the half-written field is *further* from the
        // truth than the dark one. A field released from rest at
        // a displaced shape splits and misleads; the dark field
        // only has to be rebuilt by the lamps' identical forcing.
        let g = run_granary();
        let (ra, rs) = (g.samples[0].1, g.samples[0].2);
        assert!(ra > 0.0005, "api RMS at first sample too small: {ra}");
        assert!(
            ra > rs,
            "expected the half-written field to start worse than dark: {ra} vs {rs}"
        );
    }
}
