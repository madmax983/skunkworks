//! # Listener Flutter 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! (one bat, true echolocation) by way of `experiments/listener-choir`
//! (three bats, one shared air). The choir's README closed with an
//! unused sense, named plainly: *a bat reads only the first return
//! per window and never uses amplitude modulation — a moth's flutter
//! is in the cave, but this generation of bats cannot hear it yet.*
//! This is that sense, grown.
//!
//! The physics added here is one channel, honestly stated. A bat's
//! chirp is a short sweep, not an instant. A struck reflector answers
//! with a **glint train**: three glints, four ticks apart, whose
//! amplitudes sample the reflector's reflecting surface at the
//! strike and at +4 and +8 ticks after it.
//!
//! - A **stone** presents the same surface at every sample: its
//!   three glints carry the same amplitude (the path attenuation,
//!   and nothing else).
//! - A **moth** is beating its wings with period 12 ticks, so the
//!   three samples land a third of a wingbeat apart and its glint
//!   amplitudes swing: multiplier `0.62 + 0.38·cos(phase + 2π·t/12)`,
//!   a spread across the train that no steady reflector can imitate
//!   and no moth can hide (three samples a third of a cycle apart
//!   always span at least half the swing — proven in the tests over
//!   every phase and every strike tick).
//!
//! Two regimes, one bat, one cave, one seed:
//! - **single-return** — the principal's rule from vespertilio and
//!   the choir: believe the first glint the moment it arrives and
//!   pursue. Stones are indistinguishable from moths; a stone is
//!   learned only by flying to it and finding nothing (and then
//!   remembered, as the choir's bats remembered).
//! - **flutter** — the new sense: on the first return, the bat
//!   *holds still* and listens to the whole train (the third glint
//!   lands 8 ticks after the first — the moth moves ~1.2 units in
//!   that time, and that delay is the sense's price). It classifies
//!   by the train's amplitude spread, `(max − min) / mean`: above
//!   0.30, flutter — pursue; at or below, steady — refuse the
//!   spelled point without spending the flight, and remember it as
//!   empty. The bat never reads the reflector's kind; it reads
//!   three numbers the medium delivered. Whether a refusal was
//!   *right* is scored by the world, never by the bat.
//!
//! The perception rule is sacred, as always: a bat knows only the
//! arrivals at its own ears — arrival tick, amplitude, its own
//! chirp tick and facing, and its own memory of places it proved
//! empty. Catches and refusal-correctness are counted by the world.
//!
//! `cargo run -p listener-flutter -- --headless` prints the ledger
//! for both regimes. There is no TUI; --headless is handled before
//! anything else regardless.

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

/// Glints per train, and the ticks between them.
const GLINTS: usize = 3;
const GLINT_GAP: u64 = 4;
/// Moth wingbeat period, ticks. GLINT_GAP samples a third of it.
const WING_PERIOD: f64 = 12.0;
/// Classification line on (max − min) / mean across a train.
const FLUTTER_LINE: f64 = 0.30;

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
    0.62 + 0.38 * (phase + std::f64::consts::TAU * t as f64 / WING_PERIOD).cos()
}

/// The glint train a struck reflector sends back: GLINTS amplitudes,
/// sampled GLINT_GAP ticks apart starting at the strike tick.
/// A stone's train is flat by construction; a moth's swings.
fn glint_train(is_moth: bool, phase: f64, base_amp: f64, t_strike: u64) -> [f64; GLINTS] {
    let mut out = [base_amp; GLINTS];
    if is_moth {
        for (k, slot) in out.iter_mut().enumerate() {
            *slot = base_amp * moth_multiplier(phase, t_strike + GLINT_GAP * k as u64);
        }
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
fn classify(samples: &[f64]) -> Verdict {
    if samples.len() < 2 {
        return Verdict::Unknown;
    }
    let max = samples.iter().cloned().fold(f64::MIN, f64::max);
    let min = samples.iter().cloned().fold(f64::MAX, f64::min);
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    if mean <= 0.0 {
        return Verdict::Unknown;
    }
    if (max - min) / mean > FLUTTER_LINE {
        Verdict::Flutter
    } else {
        Verdict::Steady
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

#[derive(Clone, Copy, PartialEq)]
enum Regime {
    Single,
    Flutter,
}

/// A train the flutter bat is still listening to: it holds still
/// until the last glint lands, then judges the whole train.
struct Pending {
    spelled: Pos,
    kind: RefKind,
    samples: Vec<f64>,
    resolve_tick: u64,
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

fn run_regime(regime: Regime) -> RegimeStats {
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
                reflectors.push((s, RefKind::Stone(si), false, 0.0));
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
                let train = glint_train(is_moth, phase, amp, tick);
                for (k, &a) in train.iter().enumerate() {
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
            match regime {
                Regime::Single => {
                    // The principal's rule: believe it now.
                    bat.pursue(spelled, s.kind, tick);
                }
                Regime::Flutter => {
                    // Hold still and hear the whole train out.
                    bat.pending = Some(Pending {
                        spelled,
                        kind: s.kind,
                        samples: vec![s.amp],
                        resolve_tick: first_tick + GLINT_GAP * (GLINTS as u64 - 1),
                    });
                }
            }
        }

        // --- the flutter bat judges a completed train ----------------
        if let Some(p) = bat.pending.take_if(|p| tick >= p.resolve_tick) {
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
    }
}

fn print_regime(name: &str, st: &RegimeStats) {
    println!("  regime '{name}': 1 bat, {} ticks", TICKS);
    println!("    chirps emitted:      {}", st.chirps);
    println!("    returns heard (trains): {}", st.trains_heard);
    println!("    glints heard:        {}", st.glints_heard);
    println!("    pursuits begun:      {}", st.pursuits);
    println!("    stone pursuits (world-scored): {}", st.stone_pursuits);
    println!("    stone visits (flew there, no food): {}", st.stone_visits);
    println!("    refusals (steady trains, not flown to): {}", st.refusals);
    println!("    refusals correct (it was a stone): {}", st.refusals_correct);
    println!("    refusals wrong (it was a moth): {}", st.refusals_wrong);
    println!("    fooled flutter (stone judged fluttering): {}", st.fooled_flutter);
    println!("    moths caught:        {}", st.catches);
}

fn run_headless() {
    println!("listener-flutter headless: one bat, one new sense — the glint train");
    println!(
        "  cave {:.0}x{:.0}, {} moths, {} stones, c = {:.1} units/tick, window {} ticks, wingbeat {} ticks, glints {} x {} ticks apart, flutter line {:.2}",
        WORLD_W,
        WORLD_H,
        MOTH_STARTS.len(),
        STONES.len(),
        C,
        MAX_DELAY,
        WING_PERIOD,
        GLINTS,
        GLINT_GAP,
        FLUTTER_LINE
    );
    let single = run_regime(Regime::Single);
    print_regime("single-return (the principal's rule)", &single);
    let flutter = run_regime(Regime::Flutter);
    print_regime("flutter (listen to the whole train)", &flutter);
    println!("  the comparison the experiment exists to make:");
    println!(
        "    stone visits — single {}, flutter {}",
        single.stone_visits, flutter.stone_visits
    );
    println!(
        "    catches — single {}, flutter {}",
        single.catches, flutter.catches
    );
    println!(
        "    verdict: the flutter sense {} stone visits and {} catches",
        if flutter.stone_visits < single.stone_visits {
            "CUT"
        } else {
            "did NOT cut"
        },
        if flutter.catches >= single.catches {
            "HELD"
        } else {
            "cost"
        }
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
        println!("listener-flutter: a moth's wings write on its echo. A stone's do not.");
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
    fn a_stone_train_is_flat_and_a_moth_train_never_is() {
        // The channel's guarantee, checked over every phase and
        // every strike tick in the wingbeat cycle: three samples a
        // third of a cycle apart always span a wide spread, while
        // a stone's train has spread exactly zero.
        for phase_steps in 0..64 {
            let phase = phase_steps as f64 * std::f64::consts::TAU / 64.0;
            for t in 0..12u64 {
                let stone = glint_train(false, phase, 1.0, t);
                assert_eq!(classify(&stone), Verdict::Steady);
                let moth = glint_train(true, phase, 1.0, t);
                assert_eq!(
                    classify(&moth),
                    Verdict::Flutter,
                    "moth train must read as flutter at phase {phase}, tick {t}: {moth:?}"
                );
            }
        }
    }

    #[test]
    fn classify_needs_two_glints() {
        assert_eq!(classify(&[]), Verdict::Unknown);
        assert_eq!(classify(&[0.9]), Verdict::Unknown);
        assert_eq!(classify(&[0.9, 0.9, 0.9]), Verdict::Steady);
        assert_eq!(classify(&[0.9, 0.3, 0.9]), Verdict::Flutter);
    }

    #[test]
    fn the_single_return_bat_actually_hunts() {
        let st = run_regime(Regime::Single);
        assert!(st.catches > 0, "a bat in an honest cave must eat");
        assert!(st.trains_heard > 0);
        assert_eq!(st.refusals, 0, "the principal's rule never refuses");
    }

    #[test]
    fn flutter_cuts_stone_visits_without_starving() {
        let single = run_regime(Regime::Single);
        let flutter = run_regime(Regime::Flutter);
        assert!(
            single.stone_visits > 0,
            "the baseline must actually waste flights on stones, got {}",
            single.stone_visits
        );
        assert!(
            flutter.stone_visits < single.stone_visits,
            "flutter must cut stone visits: flutter {} vs single {}",
            flutter.stone_visits,
            single.stone_visits
        );
        assert!(
            flutter.refusals_correct > 0,
            "some refusals must be stones correctly refused"
        );
        assert_eq!(
            flutter.fooled_flutter, 0,
            "a flat train can never cross the flutter line"
        );
        assert!(
            flutter.catches > 0,
            "the flutter bat must still eat, got {}",
            flutter.catches
        );
    }
}
