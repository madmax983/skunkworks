//! # Listener Adaptive 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! and `experiments/listener-ripple` — the ripple bat's fixed sense,
//! given one new freedom its predecessor's README named: adapt.
//!
//! listener-ripple swept one fixed flutter line (0.30) across five
//! noise levels and counted the cost: errors 0 at sigma 0.00, first
//! wrong refusals at sigma 0.10, 14 errors at sigma 0.30. Its closing
//! seed asked for *the adaptive line: a bat that estimates sigma from
//! its own trains and moves its line (or demands a fourth glint),
//! judged against this fixed line's ledger at sigma 0.20 and 0.30.*
//! This is that bat. It does not move the line — it buys more
//! evidence, but only when the evidence it has is ambiguous:
//!
//! - The medium is ripple's medium, unchanged: shallow wingbeat
//!   (0.80 + 0.20*cos, period 12), rippled stones (0.92 + 0.08*cos,
//!   period 17), per-glint noise in [-sigma, +sigma] drawn
//!   deterministically from the train's identity, glints under the
//!   ear threshold (0.22) dropped. One change only: the medium now
//!   sends a FOURTH glint, 4 ticks after the third, on every train.
//! - The fixed bat (control) hears the first three glints and judges
//!   on the 0.30 line, exactly as ripple did; the fourth glint
//!   arrives after its verdict and is ignored. Its ledger must
//!   reproduce ripple's, sigma for sigma.
//! - The adaptive bat hears the same first three glints. If their
//!   spread ratio (max-min)/mean falls in the uncertain band
//!   [0.20, 0.36] — a band no clean train can enter (clean shallow
//!   moths read >= ~0.375, clean rippled stones <= ~0.17, both
//!   proven in tests) — it holds still for the fourth glint and
//!   judges all four amplitudes on the same 0.30 line. Outside the
//!   band it judges at once, paying nothing.
//!
//! The price is counted honestly: every extension costs 4 ticks of
//! held-still listening, during which the bat does not chirp, scan,
//! or pursue. The ledger prints extensions, four-glint judgments,
//! and extra listen ticks beside the errors they bought.
//!
//! The perception rule is sacred, as always: both bats read only
//! the amplitudes the medium actually delivered — never a clean
//! value, never a reflector's kind, never the true sigma. Whether a
//! verdict was right is scored by the world, never by the bat.
//!
//! `cargo run -p listener-adaptive -- --headless` prints both bats'
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30. There is no TUI;
//! --headless is handled before anything else regardless.

use anyhow::Result;

// ------------------------------------------------------------------
// The cave and the physics of its air (the choir's cave, re-used).
// ------------------------------------------------------------------

const WORLD_W: f64 = 140.0;
const WORLD_H: f64 = 90.0;
const C: f64 = 2.0;
const MAX_RANGE: f64 = 36.0;
const MAX_DELAY: u64 = 28;
const CONE: f64 = 0.55;
const BASE_PERIOD: u64 = 44;
const TICKS: u64 = 3600;
const EAR: f64 = 0.22;

const BAT_SPEED: f64 = 1.25;
const SCAN_SPEED: f64 = 0.45;
const CATCH_R: f64 = 2.8;

/// Glints the fixed bat judges, and the ticks between them.
const GLINTS: usize = 3;
/// Glints the medium actually sends per train (the adaptive bat
/// may hear the fourth; the fixed bat's verdict precedes it).
const GLINTS_SENT: usize = 4;
const GLINT_GAP: u64 = 4;
/// Moth wingbeat period, ticks. GLINT_GAP samples a third of it.
const WING_PERIOD: f64 = 12.0;
/// Stone ripple period, ticks.
const RIPPLE_PERIOD: f64 = 17.0;
/// Noise levels swept by the headless ledger.
const SIGMAS: [f64; 4] = [0.0, 0.10, 0.20, 0.30];
/// Classification line on (max − min) / mean across a train.
const FLUTTER_LINE: f64 = 0.30;
/// The uncertain band on the three-glint spread ratio: inside it,
/// the adaptive bat demands the fourth glint before judging.
/// No clean train enters it (proven in tests): clean shallow moth
/// trains read >= ~0.375, clean rippled stone trains <= ~0.17.
const BAND_LO: f64 = 0.20;
const BAND_HI: f64 = 0.36;

// ------------------------------------------------------------------
// Deterministic randomness (SplitMix64), the choir's seed.
// ------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
}

// ------------------------------------------------------------------
// Geometry helpers (the choir's, unchanged).
// ------------------------------------------------------------------

type Pos = (f64, f64);

fn dist(a: Pos, b: Pos) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn angle_to(from: Pos, to: Pos) -> f64 {
    (to.1 - from.1).atan2(to.0 - from.0)
}

fn ang_diff(a: f64, b: f64) -> f64 {
    let mut d = (a - b) % std::f64::consts::TAU;
    if d > std::f64::consts::PI {
        d -= std::f64::consts::TAU;
    } else if d < -std::f64::consts::PI {
        d += std::f64::consts::TAU;
    }
    d
}

fn echo_arrival_tick(t_emit: u64, emitter: Pos, reflector: Pos, listener: Pos) -> u64 {
    let path = dist(emitter, reflector) + dist(reflector, listener);
    t_emit + (path / C).round() as u64
}

fn echo_amplitude(emitter: Pos, reflector: Pos, listener: Pos) -> f64 {
    let path = dist(emitter, reflector) + dist(reflector, listener);
    1.6 / (1.0 + path * 0.06)
}

// ------------------------------------------------------------------
// The flutter channel itself.
// ------------------------------------------------------------------

/// A moth's reflecting multiplier at tick `t`: its wings beat with
/// period WING_PERIOD, each moth on its own phase.
fn moth_multiplier(phase: f64, t: u64) -> f64 {
    0.80 + 0.20 * (phase + std::f64::consts::TAU * t as f64 / WING_PERIOD).cos()
}

/// A stone's reflecting multiplier at tick `t`: no longer flat.
/// A slow ripple, period RIPPLE_PERIOD ticks, each stone on its
/// own phase (`1.37 * stone_index` at strike time).
fn stone_multiplier(phase: f64, t: u64) -> f64 {
    0.92 + 0.08 * (phase + std::f64::consts::TAU * t as f64 / RIPPLE_PERIOD).cos()
}

/// Deterministic per-glint noise: SplitMix64 on the train's
/// identity and the glint's index, mapped to [-sigma, +sigma].
/// The medium draws it; the same train always gets the same noise.
fn glint_noise(train: u64, glint_idx: usize, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let mut rng = Rng(train
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (glint_idx as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9)
        ^ 0x5250_4C45_2026_1030);
    (rng.unit() * 2.0 - 1.0) * sigma
}

/// The glint train a struck reflector sends back: GLINTS amplitudes,
/// sampled GLINT_GAP ticks apart starting at the strike tick.
/// A stone's train is flat by construction; a moth's swings.
fn glint_train(
    is_moth: bool,
    phase: f64,
    base_amp: f64,
    t_strike: u64,
    train: u64,
    sigma: f64,
) -> [f64; GLINTS_SENT] {
    let mut out = [base_amp; GLINTS_SENT];
    for (k, slot) in out.iter_mut().enumerate() {
        let t = t_strike + GLINT_GAP * k as u64;
        let mult = if is_moth {
            moth_multiplier(phase, t)
        } else {
            stone_multiplier(phase, t)
        };
        *slot = base_amp * mult * (1.0 + glint_noise(train, k, sigma));
    }
    out
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Verdict {
    Flutter,
    Steady,
    Unknown,
}

/// The bat's whole new sense, in one function: judge a train by
/// the spread of the amplitudes the medium actually delivered.
/// Fewer than two glints is no evidence at all.
/// (max - min) / mean over the delivered samples, or None when
/// there is no evidence to spread (fewer than two, or no signal).
fn spread_ratio(samples: &[f64]) -> Option<f64> {
    if samples.len() < 2 {
        return None;
    }
    let max = samples.iter().cloned().fold(f64::MIN, f64::max);
    let min = samples.iter().cloned().fold(f64::MAX, f64::min);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    if mean <= 0.0 {
        return None;
    }
    Some((max - min) / mean)
}

/// Is a three-glint spread ambiguous enough to buy a fourth glint?
fn in_uncertain_band(samples: &[f64]) -> bool {
    if samples.len() != GLINTS {
        return false;
    }
    match spread_ratio(samples) {
        Some(r) => (BAND_LO..=BAND_HI).contains(&r),
        None => false,
    }
}

fn classify(samples: &[f64]) -> Verdict {
    match spread_ratio(samples) {
        None => Verdict::Unknown,
        Some(r) if r > FLUTTER_LINE => Verdict::Flutter,
        Some(_) => Verdict::Steady,
    }
}

// ------------------------------------------------------------------
// Sounds in flight: one glint of one train.
// ------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
enum RefKind {
    Moth(usize),
    Stone(usize),
}

/// `kind` is carried for the world's ledger scoring (was a refusal
/// right? was a pursuit aimed at stone?). The bat's decisions use
/// only arrival tick, amplitude, and train grouping — never kind.
#[derive(Clone, Copy, Debug)]
struct Sound {
    tick: u64,
    /// Strike tick + reflector tag: glints of one train share it.
    train: u64,
    kind: RefKind,
    glint_idx: usize,
    amp: f64,
}

// ------------------------------------------------------------------
// The singer (one bat; the choir comes later, if it earns it).
// ------------------------------------------------------------------

/// Only one regime now: listener-flutter's flutter bat, unchanged.
/// What varies across runs is the medium's noise, not the bat.
#[derive(Clone, Copy, PartialEq)]
enum Regime {
    Fixed,
    Adaptive,
}

/// A train the flutter bat is still listening to: it holds still
/// until the last glint lands, then judges the whole train.
struct Pending {
    spelled: Pos,
    kind: RefKind,
    samples: Vec<f64>,
    resolve_tick: u64,
    extended: bool,
}

struct Bat {
    pos: Pos,
    heading: f64,
    next_chirp: u64,
    last_chirp: Option<u64>,
    chirp_pos: Pos,
    chirp_heading: f64,
    first_taken: bool,
    attended_train: Option<u64>,
    pending: Option<Pending>,
    target: Option<Pos>,
    target_deadline: u64,
    empty: Vec<Pos>,
    // Ledger counters.
    chirps: u64,
    trains_heard: u64,
    glints_heard: u64,
    pursuits: u64,
    stone_pursuits: u64,
    stone_visits: u64,
    refusals: u64,
    refusals_correct: u64,
    refusals_wrong: u64,
    fooled_flutter: u64,
    catches: u64,
    glints_dropped: u64,
    partial_trains: u64,
    extensions: u64,
    four_glint_judgments: u64,
}

impl Bat {
    fn new(pos: Pos) -> Self {
        Bat {
            pos,
            heading: 0.0,
            next_chirp: 0,
            last_chirp: None,
            chirp_pos: pos,
            chirp_heading: 0.0,
            first_taken: false,
            attended_train: None,
            pending: None,
            target: None,
            target_deadline: 0,
            empty: Vec::new(),
            chirps: 0,
            trains_heard: 0,
            glints_heard: 0,
            pursuits: 0,
            stone_pursuits: 0,
            stone_visits: 0,
            refusals: 0,
            refusals_correct: 0,
            refusals_wrong: 0,
            fooled_flutter: 0,
            catches: 0,
            glints_dropped: 0,
            partial_trains: 0,
            extensions: 0,
            four_glint_judgments: 0,
        }
    }

    fn remember_empty(&mut self, p: Pos) {
        self.empty.push(p);
        if self.empty.len() > 24 {
            self.empty.remove(0);
        }
    }

    /// Begin a pursuit of `spelled`, scored by the world via `kind`.
    fn pursue(&mut self, spelled: Pos, kind: RefKind, tick: u64) {
        self.target = Some(spelled);
        self.target_deadline = tick + 70;
        self.pursuits += 1;
        if matches!(kind, RefKind::Stone(_)) {
            self.stone_pursuits += 1;
        }
    }
}

struct Moth {
    pos: Pos,
    heading: f64,
    alive: bool,
    phase: f64,
}

#[derive(Default, Clone, Debug)]
struct RegimeStats {
    chirps: u64,
    trains_heard: u64,
    glints_heard: u64,
    pursuits: u64,
    stone_pursuits: u64,
    stone_visits: u64,
    refusals: u64,
    refusals_correct: u64,
    refusals_wrong: u64,
    fooled_flutter: u64,
    catches: u64,
    glints_dropped: u64,
    partial_trains: u64,
    extensions: u64,
    four_glint_judgments: u64,
}

const MOTH_STARTS: [Pos; 8] = [
    (60.0, 20.0),
    (80.0, 45.0),
    (55.0, 70.0),
    (100.0, 30.0),
    (110.0, 65.0),
    (70.0, 55.0),
    (40.0, 30.0),
    (120.0, 55.0),
];
const MOTH_HEADINGS: [f64; 8] = [0.3, 2.1, -1.2, 0.9, -2.6, 1.7, 1.1, -0.4];
/// Twelve stones — the choir's five, plus seven more, so the
/// stone channel is struck often enough for the ledger to judge
/// a sense whose whole claim is *not flying to rock*.
const STONES: [Pos; 12] = [
    (48.0, 58.0),
    (90.0, 15.0),
    (95.0, 80.0),
    (35.0, 75.0),
    (115.0, 45.0),
    (65.0, 35.0),
    (75.0, 75.0),
    (105.0, 20.0),
    (45.0, 15.0),
    (85.0, 60.0),
    (55.0, 45.0),
    (125.0, 75.0),
];

fn run_regime(regime: Regime, sigma: f64) -> RegimeStats {
    let mut bat = Bat::new((25.0, 45.0));
    let mut moths: Vec<Moth> = MOTH_STARTS
        .iter()
        .zip(MOTH_HEADINGS.iter())
        .enumerate()
        .map(|(i, (&pos, &heading))| Moth {
            pos,
            heading,
            alive: true,
            phase: 1.93 * i as f64,
        })
        .collect();
    let mut rng = Rng(0x1157_E11E_2026_1028);
    let mut sounds: Vec<Sound> = Vec::new();

    for tick in 0..TICKS {
        // --- moths wander, knowing nothing -------------------------
        for m in moths.iter_mut() {
            if !m.alive {
                continue;
            }
            m.heading += (rng.unit() - 0.5) * 0.3;
            let mut nx = m.pos.0 + m.heading.cos() * 0.15;
            let mut ny = m.pos.1 + m.heading.sin() * 0.15;
            if !(2.0..=WORLD_W - 2.0).contains(&nx) {
                m.heading = std::f64::consts::PI - m.heading;
                nx = m.pos.0;
            }
            if !(2.0..=WORLD_H - 2.0).contains(&ny) {
                m.heading = -m.heading;
                ny = m.pos.1;
            }
            m.pos = (nx, ny);
        }

        // --- chirp (never while listening to a train) --------------
        if bat.pending.is_none() && tick >= bat.next_chirp {
            let (epos, eheading) = (bat.pos, bat.heading);
            bat.chirps += 1;
            bat.last_chirp = Some(tick);
            bat.chirp_pos = epos;
            bat.chirp_heading = eheading;
            bat.first_taken = false;
            bat.attended_train = None;
            bat.next_chirp = tick + if bat.target.is_some() { 30 } else { BASE_PERIOD };

            let mut reflectors: Vec<(Pos, RefKind, bool, f64)> = Vec::new();
            for (mi, m) in moths.iter().enumerate() {
                if m.alive {
                    reflectors.push((m.pos, RefKind::Moth(mi), true, m.phase));
                }
            }
            for (si, &s) in STONES.iter().enumerate() {
                reflectors.push((s, RefKind::Stone(si), false, 1.37 * si as f64));
            }
            for (ridx, (rpos, kind, is_moth, phase)) in reflectors.into_iter().enumerate() {
                let d1 = dist(epos, rpos);
                if d1 > MAX_RANGE {
                    continue;
                }
                if ang_diff(angle_to(epos, rpos), eheading).abs() > CONE {
                    continue;
                }
                let amp = echo_amplitude(epos, rpos, epos);
                if amp < EAR {
                    continue;
                }
                let arrival = echo_arrival_tick(tick, epos, rpos, epos);
                let train_id = tick * 64 + ridx as u64;
                let train = glint_train(is_moth, phase, amp, tick, train_id, sigma);
                let send = match regime {
                    Regime::Fixed => GLINTS,
                    Regime::Adaptive => GLINTS_SENT,
                };
                for (k, &a) in train.iter().take(send).enumerate() {
                    // Audibility is judged on the noisy amplitude
                    // the medium actually delivers: a glint that
                    // dips under the ear never arrives, and the
                    // train the bat judges arrives partial.
                    if a < EAR {
                        bat.glints_dropped += 1;
                        continue;
                    }
                    sounds.push(Sound {
                        tick: arrival + GLINT_GAP * k as u64,
                        train: train_id,
                        kind,
                        glint_idx: k,
                        amp: a,
                    });
                }
            }
        }

        // --- arrivals at the ear ------------------------------------
        let mut i = 0;
        while i < sounds.len() {
            if sounds[i].tick != tick {
                i += 1;
                continue;
            }
            let s = sounds.swap_remove(i);
            let Some(lc) = bat.last_chirp else { continue };
            // A glint counts only inside its chirp's listening
            // window (the train's last glint may land up to 8 ticks
            // after the first, still inside the 28-tick window for
            // any return the first glint of which was inside it).
            let first_tick = s.tick - GLINT_GAP * s.glint_idx as u64;
            if first_tick <= lc || first_tick > lc + MAX_DELAY {
                continue;
            }
            if s.glint_idx == 0 {
                bat.trains_heard += 1;
            }
            bat.glints_heard += 1;

            if bat.attended_train == Some(s.train) {
                // A later glint of the train being judged: the
                // flutter bat collects it; the single-return bat
                // has already acted and ignores it.
                if let Some(p) = bat.pending.as_mut() {
                    p.samples.push(s.amp);
                }
                continue;
            }
            if bat.first_taken {
                continue;
            }
            // First glint of a new train in this window.
            let r = (first_tick - lc) as f64 * C / 2.0;
            let spelled = (
                bat.chirp_pos.0 + r * bat.chirp_heading.cos(),
                bat.chirp_pos.1 + r * bat.chirp_heading.sin(),
            );
            if bat.empty.iter().any(|&e| dist(e, spelled) < 2.0) {
                // A place already proven empty is not believed
                // again; the window stays open for the next train.
                continue;
            }
            bat.first_taken = true;
            bat.attended_train = Some(s.train);
            // Both bats hold still and hear the first train out;
            // they differ only in what a completed train may ask
            // for (the adaptive bat may ask for one more glint).
            bat.pending = Some(Pending {
                spelled,
                kind: s.kind,
                samples: vec![s.amp],
                resolve_tick: first_tick + GLINT_GAP * (GLINTS as u64 - 1),
                extended: false,
            });
        }

        // --- a completed train is judged (or extended) --------------
        // The adaptive bat, faced with a three-glint spread inside
        // the uncertain band, holds still for the fourth glint and
        // judges all four. The fixed bat never extends.
        if matches!(regime, Regime::Adaptive) {
            let extend = match bat.pending.as_ref() {
                Some(p) => tick >= p.resolve_tick && !p.extended && in_uncertain_band(&p.samples),
                None => false,
            };
            if extend {
                let p = bat.pending.as_mut().expect("pending checked above");
                p.extended = true;
                p.resolve_tick = tick + GLINT_GAP;
                bat.extensions += 1;
            }
        }
        if let Some(p) = bat.pending.take_if(|p| tick >= p.resolve_tick) {
            if p.samples.len() < GLINTS {
                bat.partial_trains += 1;
            }
            if p.samples.len() >= GLINTS_SENT {
                bat.four_glint_judgments += 1;
            }
            match classify(&p.samples) {
                Verdict::Flutter => {
                    if matches!(p.kind, RefKind::Stone(_)) {
                        bat.fooled_flutter += 1;
                    }
                    bat.pursue(p.spelled, p.kind, tick);
                }
                Verdict::Steady => {
                    bat.refusals += 1;
                    match p.kind {
                        RefKind::Stone(_) => bat.refusals_correct += 1,
                        RefKind::Moth(_) => bat.refusals_wrong += 1,
                    }
                    bat.remember_empty(p.spelled);
                }
                Verdict::Unknown => {
                    // No evidence either way: pursue on faith,
                    // as the principal would.
                    bat.pursue(p.spelled, p.kind, tick);
                }
            }
            bat.next_chirp = tick + if bat.target.is_some() { 30 } else { BASE_PERIOD };
        }

        // --- the bat moves on what it believes -----------------------
        if bat.pending.is_none() {
            let mut speed = SCAN_SPEED;
            if let Some(target) = bat.target {
                if tick >= bat.target_deadline {
                    bat.target = None;
                } else {
                    let desired = angle_to(bat.pos, target);
                    let turn = ang_diff(desired, bat.heading).clamp(-0.30, 0.30);
                    bat.heading += turn;
                    speed = BAT_SPEED;
                    if dist(bat.pos, target) < 1.8 {
                        let stone_there = STONES.iter().any(|&s| dist(s, target) < 2.5);
                        let moth_there =
                            moths.iter().any(|m| m.alive && dist(m.pos, target) < 3.0);
                        if stone_there && !moth_there {
                            bat.stone_visits += 1;
                            bat.remember_empty(target);
                        }
                        bat.target = None;
                    }
                }
            }
            if bat.target.is_none() {
                bat.heading += 0.04;
                speed = SCAN_SPEED;
            }
            let mut nx = bat.pos.0 + bat.heading.cos() * speed;
            let mut ny = bat.pos.1 + bat.heading.sin() * speed;
            if !(1.0..=WORLD_W - 1.0).contains(&nx) {
                bat.heading = std::f64::consts::PI - bat.heading;
                nx = bat.pos.0;
            }
            if !(1.0..=WORLD_H - 1.0).contains(&ny) {
                bat.heading = -bat.heading;
                ny = bat.pos.1;
            }
            bat.pos = (nx, ny);
        }

        // --- the world, not the bat, counts a catch ------------------
        for mi in 0..moths.len() {
            if !moths[mi].alive {
                continue;
            }
            if dist(bat.pos, moths[mi].pos) < CATCH_R {
                bat.catches += 1;
                let mut p = (rng.range(10.0, WORLD_W - 10.0), rng.range(10.0, WORLD_H - 10.0));
                for _ in 0..8 {
                    if dist(bat.pos, p) > 18.0 {
                        break;
                    }
                    p = (rng.range(10.0, WORLD_W - 10.0), rng.range(10.0, WORLD_H - 10.0));
                }
                moths[mi].pos = p;
                moths[mi].heading = rng.range(-std::f64::consts::PI, std::f64::consts::PI);
                break;
            }
        }
    }

    RegimeStats {
        chirps: bat.chirps,
        trains_heard: bat.trains_heard,
        glints_heard: bat.glints_heard,
        pursuits: bat.pursuits,
        stone_pursuits: bat.stone_pursuits,
        stone_visits: bat.stone_visits,
        refusals: bat.refusals,
        refusals_correct: bat.refusals_correct,
        refusals_wrong: bat.refusals_wrong,
        fooled_flutter: bat.fooled_flutter,
        catches: bat.catches,
        glints_dropped: bat.glints_dropped,
        partial_trains: bat.partial_trains,
        extensions: bat.extensions,
        four_glint_judgments: bat.four_glint_judgments,
    }
}

fn print_bat(label: &str, sigma: f64, st: &RegimeStats) {
    let errors = st.refusals_wrong + st.fooled_flutter;
    println!("  regime '{} sigma = {:.2}': 1 bat, {} ticks", label, sigma, TICKS);
    println!("    chirps emitted:      {}", st.chirps);
    println!("    returns heard (trains): {}", st.trains_heard);
    println!("    glints heard:        {}", st.glints_heard);
    println!("    glints dropped (modulated amp under the ear): {}", st.glints_dropped);
    println!("    trains attended partial (< {} glints): {}", GLINTS, st.partial_trains);
    println!("    extensions (uncertain band, fourth glint demanded): {}", st.extensions);
    println!("    four-glint judgments: {}", st.four_glint_judgments);
    println!("    extra listen ticks (4 per extension): {}", st.extensions * GLINT_GAP);
    println!("    pursuits begun:      {}", st.pursuits);
    println!("    stone pursuits (world-scored): {}", st.stone_pursuits);
    println!("    stone visits (flew there, no food): {}", st.stone_visits);
    println!("    refusals (steady trains, not flown to): {}", st.refusals);
    println!("    refusals correct (it was a stone): {}", st.refusals_correct);
    println!("    refusals wrong (it was a moth): {}", st.refusals_wrong);
    println!("    fooled flutter (stone judged fluttering): {}", st.fooled_flutter);
    println!("    classification errors (wrong + fooled): {}", errors);
    println!("    moths caught:        {}", st.catches);
}

fn run_headless() {
    println!("listener-adaptive headless: the ripple bat, fixed vs adaptive — a fourth glint bought only inside the uncertain band");
    println!(
        "  cave {:.0}x{:.0}, {} moths, {} stones, c = {:.1} units/tick, window {} ticks, wingbeat {} ticks (depth 0.20), stone ripple {} ticks (depth 0.08), glints {} sent x {} ticks apart, flutter line {:.2} (unchanged), uncertain band [{:.2}, {:.2}]",
        WORLD_W, WORLD_H, MOTH_STARTS.len(), STONES.len(), C, MAX_DELAY,
        WING_PERIOD, RIPPLE_PERIOD, GLINTS_SENT, GLINT_GAP, FLUTTER_LINE, BAND_LO, BAND_HI
    );
    for sigma in SIGMAS {
        let fixed = run_regime(Regime::Fixed, sigma);
        print_bat("fixed", sigma, &fixed);
        let adaptive = run_regime(Regime::Adaptive, sigma);
        print_bat("adaptive", sigma, &adaptive);
        println!(
            "    comparison sigma {:.2}: errors fixed {} -> adaptive {}, catches fixed {} -> adaptive {}, extensions {}",
            sigma,
            fixed.refusals_wrong + fixed.fooled_flutter,
            adaptive.refusals_wrong + adaptive.fooled_flutter,
            fixed.catches, adaptive.catches, adaptive.extensions
        );
    }
    let f30 = run_regime(Regime::Fixed, 0.30);
    let a30 = run_regime(Regime::Adaptive, 0.30);
    println!("  the comparison the experiment exists to make:");
    println!(
        "    errors at sigma 0.30 — fixed: {}, adaptive: {}",
        f30.refusals_wrong + f30.fooled_flutter,
        a30.refusals_wrong + a30.fooled_flutter
    );
    println!(
        "    catches at sigma 0.30 — fixed: {}, adaptive: {}",
        f30.catches, a30.catches
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, in any mode, by design.
    let headless = args.iter().any(|a| a == "--headless");
    if headless {
        run_headless();
    } else {
        println!("listener-adaptive: the same sense, allowed to ask for one more glint.");
        println!("  (hear it headless any time: --headless)");
        run_headless();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_arrives_when_the_path_says() {
        let t = echo_arrival_tick(100, (0.0, 0.0), (10.0, 0.0), (0.0, 0.0));
        assert_eq!(t, 110);
        let t2 = echo_arrival_tick(100, (0.0, 0.0), (10.0, 0.0), (10.0, 0.0));
        assert_eq!(t2, 105);
    }

    #[test]
    fn clean_trains_never_enter_the_uncertain_band() {
        // The band is honest only if the clean cave never pays for
        // it: over every phase and strike tick, a clean three-glint
        // shallow moth train reads above BAND_HI and a clean
        // rippled stone train reads below BAND_LO.
        for phase_steps in 0..64 {
            let phase = phase_steps as f64 * std::f64::consts::TAU / 64.0;
            for t in 0..12u64 {
                let moth = glint_train(true, phase, 1.0, t, 999, 0.0);
                let r = spread_ratio(&moth[..GLINTS]).expect("three glints spread");
                assert!(r > BAND_HI, "clean moth in band at phase {phase}, tick {t}: ratio {r}");
                assert_eq!(classify(&moth[..GLINTS]), Verdict::Flutter);
            }
            for t in 0..17u64 {
                let stone = glint_train(false, phase, 1.0, t, 998, 0.0);
                let r = spread_ratio(&stone[..GLINTS]).expect("three glints spread");
                assert!(r < BAND_LO, "clean stone in band at phase {phase}, tick {t}: ratio {r}");
                assert_eq!(classify(&stone[..GLINTS]), Verdict::Steady);
            }
        }
    }

    #[test]
    fn noise_is_deterministic() {
        let a = glint_train(false, 0.7, 1.0, 40, 12345, 0.30);
        let b = glint_train(false, 0.7, 1.0, 40, 12345, 0.30);
        assert_eq!(a, b);
    }

    #[test]
    fn classify_needs_two_glints() {
        assert_eq!(classify(&[]), Verdict::Unknown);
        assert_eq!(classify(&[0.9]), Verdict::Unknown);
        assert_eq!(classify(&[0.9, 0.9, 0.9]), Verdict::Steady);
        assert_eq!(classify(&[0.9, 0.3, 0.9]), Verdict::Flutter);
        // The band judges three glints only, never fewer or more.
        assert!(!in_uncertain_band(&[0.9, 0.7]));
        assert!(!in_uncertain_band(&[0.9, 0.7, 0.8, 0.75]));
        assert!(in_uncertain_band(&[1.0, 0.8, 0.9]));
    }

    #[test]
    fn fixed_bat_reproduces_the_ripple_cave() {
        // The control is the predecessor's own bat: at sigma 0 its
        // ledger is ripple's clean ledger, line for line.
        let st = run_regime(Regime::Fixed, 0.0);
        assert_eq!(st.chirps, 71);
        assert_eq!(st.trains_heard, 35);
        assert_eq!(st.refusals_correct, 12);
        assert_eq!(st.refusals_wrong + st.fooled_flutter, 0);
        assert_eq!(st.stone_visits, 0);
        assert_eq!(st.catches, 7);
        assert_eq!(st.extensions, 0);
    }

    #[test]
    fn adaptive_bat_pays_nothing_in_the_clean_cave() {
        // No clean train enters the band, so the adaptive bat in a
        // quiet cave is the fixed bat: no extensions, same errors,
        // same catches.
        let fixed = run_regime(Regime::Fixed, 0.0);
        let adaptive = run_regime(Regime::Adaptive, 0.0);
        assert_eq!(adaptive.extensions, 0);
        assert_eq!(adaptive.four_glint_judgments, 0);
        assert_eq!(
            adaptive.refusals_wrong + adaptive.fooled_flutter,
            fixed.refusals_wrong + fixed.fooled_flutter
        );
        assert_eq!(adaptive.catches, fixed.catches);
    }

    #[test]
    fn noise_makes_the_adaptive_bat_ask() {
        let loud = run_regime(Regime::Adaptive, 0.30);
        assert!(loud.extensions > 0, "at sigma 0.30 some trains must land in the band");
        assert!(
            loud.four_glint_judgments > 0,
            "some demanded fourth glints must actually arrive and be judged"
        );
        let fixed = run_regime(Regime::Fixed, 0.30);
        assert_eq!(fixed.extensions, 0, "the fixed bat never asks");
        assert!(
            fixed.refusals_wrong + fixed.fooled_flutter > 0,
            "the loud cave must still cost the fixed bat errors"
        );
    }
}
