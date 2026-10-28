//! # Listener Choir 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`,
//! where one bat hunted moths by true echolocation — a chirp out, a
//! return back, and nothing known except what the wave delivered.
//!
//! This is the succession vespertilio asked for: **more than one bat
//! in the same air.** A solo bat's cave is honest — every return in
//! its listening window is its own. A choir's cave is not. Sound is
//! sound: when three bats chirp in unison, each one's window fills
//! with the *others'* echoes, arriving at ticks that — read against
//! the wrong chirp — spell false ranges along false bearings. A
//! neighbour's outgoing chirp, passing overhead mid-window, masks
//! whatever returns land just behind it. The bats cannot label what
//! arrives. The medium does not sign its work.
//!
//! Three regimes, one cave, one seed:
//! - **solo** — one bat, fixed tempo. The honest baseline.
//! - **choir, naive** — three bats, identical fixed tempo, first
//!   chirp in unison. Maximum jamming.
//! - **choir, polite** — three bats, same start, but a bat whose
//!   window was jammed (it heard a foreign return, or was masked)
//!   delays its next chirp once by a few ticks — a jamming-
//!   avoidance response. Its tempo never changes; only its phase
//!   moves. The delays are earned only through the medium and
//!   differ per bat, so the unison breaks.
//!
//! The perception rule is sacred, as always: a bat knows only the
//! arrivals at its own ears — arrival tick, amplitude, and its own
//! memory of when it chirped and which way it faced. A believed
//! target is `my position when I chirped + (arrival − my chirp) ·
//! c/2` along *my* facing at *my* chirp. A bat steers on the FIRST
//! return of each window the moment it arrives — the nearest
//! reflector, and the freshest fix physics allows. For a foreign
//! echo that arithmetic is a lie, and the bat pursues the lie
//! exactly as faithfully as the truth. With a live target its
//! chirps come faster (period 30 against the scanning 44 — the
//! feeding buzz), and a place its own body proved is a stone is
//! remembered and not believed again. Catches are physical
//! contact, counted by the world, never by the bat.
//!
//! Arrival geometry is computed from the strike: an echo struck at
//! reflector R by emitter E reaches bat B at
//! `t + (|E→R| + |R→B|) / c`, amplitude attenuated over the whole
//! path. Positions are taken at the strike tick (a bat moves ≲1
//! unit while its echo is in flight; the ledger does not pretend
//! otherwise).
//!
//! `cargo run -p listener-choir -- --headless` prints the ledger
//! for all three regimes. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;

// ------------------------------------------------------------------
// The cave and the physics of its air.
// ------------------------------------------------------------------

const WORLD_W: f64 = 140.0;
const WORLD_H: f64 = 90.0;
/// Speed of sound, units per tick. With c = 2 an own echo's delay
/// in ticks numerically equals its range in units — the arithmetic
/// a bat does is `range = delay · c / 2`.
const C: f64 = 2.0;
/// A chirp reaches reflectors at most this far away.
const MAX_RANGE: f64 = 36.0;
/// Listening window after a chirp, in ticks. A bat only trusts
/// returns that land inside it — spelled range at most 28 units,
/// though its chirp can strike reflectors out to MAX_RANGE (their
/// echoes land after the window has closed, and are not heard).
const MAX_DELAY: u64 = 28;
/// Half-angle of the directional chirp cone, radians.
const CONE: f64 = 0.55;
/// Base chirp period, ticks. Longer than the window, so a bat's
/// own windows never overlap.
const BASE_PERIOD: u64 = 44;
/// Ticks simulated per regime.
const TICKS: u64 = 3600;
/// Quietest arrival a bat's ear registers.
const EAR: f64 = 0.22;
/// Ticks of receiver masking after a neighbour's chirp passes.
const MASK_TICKS: u64 = 5;
/// One-time delay of the next chirp per jammed window in the
/// polite regime, ticks — the jamming-avoidance response. The
/// bat's tempo itself never changes; only its phase moves, and
/// each bat's jam history differs, so the phases drift apart.
const DELAY_STEP: u64 = 7;

const BAT_SPEED: f64 = 1.25;
const SCAN_SPEED: f64 = 0.45;
const CATCH_R: f64 = 2.8;

// ------------------------------------------------------------------
// Deterministic randomness (SplitMix64). One seed for every regime,
// so the moths start from the same cave; what diverges is earned.
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
// Geometry helpers.
// ------------------------------------------------------------------

type Pos = (f64, f64);

fn dist(a: Pos, b: Pos) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn angle_to(from: Pos, to: Pos) -> f64 {
    (to.1 - from.1).atan2(to.0 - from.0)
}

/// Smallest signed difference a − b, wrapped to (−π, π].
fn ang_diff(a: f64, b: f64) -> f64 {
    let mut d = (a - b) % std::f64::consts::TAU;
    if d > std::f64::consts::PI {
        d -= std::f64::consts::TAU;
    } else if d < -std::f64::consts::PI {
        d += std::f64::consts::TAU;
    }
    d
}

/// Tick at which an echo struck at `reflector` by `emitter` reaches
/// `listener`: the wave travels emitter → reflector → listener at c.
/// This is the whole medium in one line — pluck here, arrive there,
/// at a time set by the path and nothing else.
fn echo_arrival_tick(t_emit: u64, emitter: Pos, reflector: Pos, listener: Pos) -> u64 {
    let path = dist(emitter, reflector) + dist(reflector, listener);
    t_emit + (path / C).round() as u64
}

/// Amplitude of that echo at the listener: a reflector sends back a
/// fixed share, attenuated over the entire path it travelled.
fn echo_amplitude(emitter: Pos, reflector: Pos, listener: Pos) -> f64 {
    let path = dist(emitter, reflector) + dist(reflector, listener);
    1.6 / (1.0 + path * 0.06)
}

// ------------------------------------------------------------------
// Sounds in flight.
// ------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
enum RefKind {
    Moth(usize),
    Stone(usize),
}

/// `kind`'s reflector identity is carried for the ledger and for
/// future senses; this generation of bats cannot read it — that is
/// rather the point — so the compiler is told to stop worrying.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum SoundKind {
    /// A neighbour's outgoing chirp passing this bat's ears.
    Chirp { emitter: usize },
    /// A return wavefront arriving at this bat's ears.
    Echo { emitter: usize, kind: RefKind },
}

#[derive(Clone, Copy, Debug)]
struct Sound {
    tick: u64,
    to: usize,
    kind: SoundKind,
}

// ------------------------------------------------------------------
// The singers.
// ------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Choir {
    Solo,
    Naive,
    Polite,
}

struct Bat {
    pos: Pos,
    heading: f64,
    period: u64,
    next_chirp: u64,
    last_chirp: Option<u64>,
    chirp_pos: Pos,
    chirp_heading: f64,
    /// Has the first return of the current window already been
    /// believed? A bat steers on the first return the moment it
    /// arrives (nearest reflector, freshest possible fix) and
    /// ignores the rest of that window's arrivals.
    first_taken: bool,
    window_jammed: bool,
    masked_until: u64,
    target: Option<Pos>,
    target_deadline: u64,
    /// Places this bat's own echoes sent it where no food was —
    /// its own experience, remembered. First returns that spell a
    /// point near a remembered empty place are distrusted.
    empty: Vec<Pos>,
    // Ledger counters, per bat.
    chirps: u64,
    chirps_heard: u64,
    own_returns: u64,
    foreign_returns: u64,
    masked_returns: u64,
    pursuits: u64,
    false_pursuits: u64,
    stone_visits: u64,
    delays: u64,
    catches: u64,
}

impl Bat {
    fn new(pos: Pos) -> Self {
        Bat {
            pos,
            heading: 0.0,
            period: BASE_PERIOD,
            next_chirp: 0,
            last_chirp: None,
            chirp_pos: pos,
            chirp_heading: 0.0,
            first_taken: false,
            window_jammed: false,
            masked_until: 0,
            target: None,
            target_deadline: 0,
            empty: Vec::new(),
            chirps: 0,
            chirps_heard: 0,
            own_returns: 0,
            foreign_returns: 0,
            masked_returns: 0,
            pursuits: 0,
            false_pursuits: 0,
            stone_visits: 0,
            delays: 0,
            catches: 0,
        }
    }

    /// The polite regime's one move, called when a listening window
    /// closes: if the window was jammed, delay the next chirp once.
    /// The tempo never changes — only the phase moves.
    /// Returns true if a delay happened.
    fn maybe_delay_next(&mut self, polite: bool) -> bool {
        if polite && self.window_jammed {
            self.next_chirp += DELAY_STEP;
            self.delays += 1;
            true
        } else {
            false
        }
    }
}

struct Moth {
    pos: Pos,
    heading: f64,
    alive: bool,
}

#[derive(Default, Clone, Debug)]
struct RegimeStats {
    chirps: u64,
    chirps_heard: u64,
    own_returns: u64,
    foreign_returns: u64,
    masked_returns: u64,
    pursuits: u64,
    false_pursuits: u64,
    stone_visits: u64,
    delays: u64,
    catches: u64,
    per_bat_catches: Vec<u64>,
    per_bat_delays: Vec<u64>,
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
const STONES: [Pos; 5] = [
    (48.0, 58.0),
    (90.0, 15.0),
    (95.0, 80.0),
    (35.0, 75.0),
    (115.0, 45.0),
];

fn run_regime(choir: Choir) -> RegimeStats {
    let polite = choir == Choir::Polite;
    let roosts: &[Pos] = match choir {
        Choir::Solo => &[(25.0, 45.0)],
        _ => &[(25.0, 22.0), (25.0, 45.0), (25.0, 68.0)],
    };
    let mut bats: Vec<Bat> = roosts.iter().map(|&p| Bat::new(p)).collect();
    let mut moths: Vec<Moth> = MOTH_STARTS
        .iter()
        .zip(MOTH_HEADINGS.iter())
        .map(|(&pos, &heading)| Moth {
            pos,
            heading,
            alive: true,
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

        // --- windows close: only the polite lesson is taken here --
        for bat in bats.iter_mut() {
            if let Some(lc) = bat.last_chirp {
                if tick == lc + MAX_DELAY + 1 {
                    bat.maybe_delay_next(polite);
                    bat.window_jammed = false;
                }
            }
        }

        // --- chirps -------------------------------------------------
        for bi in 0..bats.len() {
            if tick < bats[bi].next_chirp {
                continue;
            }
            let (epos, eheading) = (bats[bi].pos, bats[bi].heading);
            {
                let bat = &mut bats[bi];
                bat.chirps += 1;
                bat.last_chirp = Some(tick);
                bat.chirp_pos = epos;
                bat.chirp_heading = eheading;
                bat.first_taken = false;
                bat.window_jammed = false;
                // The feeding buzz is real bat behaviour and pure
                // mechanics here: with a live target the next chirp
                // comes sooner (period 30, still longer than the
                // 28-tick window + its closing tick), so fixes
                // arrive while the moth is still near the last one.
                bat.next_chirp = tick + if bat.target.is_some() { 30 } else { bat.period };
            }
            // The outgoing pulse passes every other bat's ears.
            for (oi, other) in bats.iter().enumerate() {
                if oi == bi {
                    continue;
                }
                let d = dist(epos, other.pos);
                sounds.push(Sound {
                    tick: tick + (d / C).round() as u64,
                    to: oi,
                    kind: SoundKind::Chirp { emitter: bi },
                });
            }
            // Reflectors inside the cone send wavefronts to ALL ears.
            let mut reflectors: Vec<(Pos, RefKind)> = Vec::new();
            for (mi, m) in moths.iter().enumerate() {
                if m.alive {
                    reflectors.push((m.pos, RefKind::Moth(mi)));
                }
            }
            for (si, &s) in STONES.iter().enumerate() {
                reflectors.push((s, RefKind::Stone(si)));
            }
            for (rpos, kind) in reflectors {
                let d1 = dist(epos, rpos);
                if d1 > MAX_RANGE {
                    continue;
                }
                if ang_diff(angle_to(epos, rpos), eheading).abs() > CONE {
                    continue;
                }
                for (oi, other) in bats.iter().enumerate() {
                    let amp = echo_amplitude(epos, rpos, other.pos);
                    if amp < EAR {
                        continue;
                    }
                    sounds.push(Sound {
                        tick: echo_arrival_tick(tick, epos, rpos, other.pos),
                        to: oi,
                        kind: SoundKind::Echo {
                            emitter: bi,
                            kind,
                        },
                    });
                }
            }
        }

        // --- arrivals at ears ---------------------------------------
        let mut i = 0;
        while i < sounds.len() {
            if sounds[i].tick != tick {
                i += 1;
                continue;
            }
            let s = sounds.swap_remove(i);
            let bat = &mut bats[s.to];
            match s.kind {
                SoundKind::Chirp { .. } => {
                    bat.chirps_heard += 1;
                    if tick >= bat.masked_until {
                        bat.masked_until = tick + MASK_TICKS;
                    }
                    // A loud pass during my window is interference,
                    // even before it masks anything concrete.
                    if let Some(lc) = bat.last_chirp {
                        if tick > lc && tick <= lc + MAX_DELAY {
                            bat.window_jammed = true;
                        }
                    }
                }
                SoundKind::Echo { emitter, .. } => {
                    if tick <= bat.masked_until && bat.masked_until > 0 && tick >= bat.masked_until - MASK_TICKS {
                        // Masked: the wavefront arrived, the ear was
                        // shut behind a neighbour's passing chirp.
                        // Only counts if it fell in a listening
                        // window — silence outside a window is not
                        // a loss, it is just the cave.
                        if let Some(lc) = bat.last_chirp {
                            if tick > lc && tick <= lc + MAX_DELAY {
                                bat.masked_returns += 1;
                                bat.window_jammed = true;
                                continue;
                            }
                        }
                    }
                    if let Some(lc) = bat.last_chirp {
                        if tick > lc && tick <= lc + MAX_DELAY {
                            if emitter == s.to {
                                bat.own_returns += 1;
                            } else {
                                bat.foreign_returns += 1;
                                bat.window_jammed = true;
                            }
                            // Steer on the first return NOW, while
                            // the fix is only as old as the echo's
                            // flight. Later arrivals in this window
                            // are farther reflectors — or other
                            // bats' business — and are not believed.
                            if !bat.first_taken {
                                let r = (tick - lc) as f64 * C / 2.0;
                                let p = (
                                    bat.chirp_pos.0 + r * bat.chirp_heading.cos(),
                                    bat.chirp_pos.1 + r * bat.chirp_heading.sin(),
                                );
                                // A point this bat already proved
                                // is a stone is not believed again;
                                // the window stays open for the
                                // next return.
                                if !bat.empty.iter().any(|&e| dist(e, p) < 2.0) {
                                    bat.target = Some(p);
                                    bat.target_deadline = tick + 70;
                                    bat.pursuits += 1;
                                    if emitter != s.to {
                                        bat.false_pursuits += 1;
                                    }
                                    bat.first_taken = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        // --- bats move on what they believe --------------------------
        for bat in bats.iter_mut() {
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
                        // Arrived where the echo said food was.
                        if STONES.iter().any(|&s| dist(s, target) < 2.5)
                            && !moths.iter().any(|m| m.alive && dist(m.pos, target) < 3.0)
                        {
                            bat.stone_visits += 1;
                        }
                        // Only a verified stone is remembered as
                        // empty: a moth that fled may wander back,
                        // but a stone is a stone forever.
                        if STONES.iter().any(|&s| dist(s, target) < 2.5)
                            && !moths.iter().any(|m| m.alive && dist(m.pos, target) < 3.0)
                        {
                            bat.empty.push(target);
                            if bat.empty.len() > 12 {
                                bat.empty.remove(0);
                            }
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

        // --- the world, not the bats, counts a catch -----------------
        for mi in 0..moths.len() {
            if !moths[mi].alive {
                continue;
            }
            for bi in 0..bats.len() {
                if dist(bats[bi].pos, moths[mi].pos) < CATCH_R {
                    bats[bi].catches += 1;
                    // Respawn far from every bat, by the world's rule.
                    let mut p = (rng.range(10.0, WORLD_W - 10.0), rng.range(10.0, WORLD_H - 10.0));
                    for _ in 0..8 {
                        if bats.iter().all(|b| dist(b.pos, p) > 18.0) {
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
    }

    let mut st = RegimeStats::default();
    for bat in &bats {
        st.chirps += bat.chirps;
        st.chirps_heard += bat.chirps_heard;
        st.own_returns += bat.own_returns;
        st.foreign_returns += bat.foreign_returns;
        st.masked_returns += bat.masked_returns;
        st.pursuits += bat.pursuits;
        st.false_pursuits += bat.false_pursuits;
        st.stone_visits += bat.stone_visits;
        st.delays += bat.delays;
        st.catches += bat.catches;
        st.per_bat_catches.push(bat.catches);
        st.per_bat_delays.push(bat.delays);
    }
    st
}

fn print_regime(name: &str, bats_n: usize, st: &RegimeStats) {
    let per_bat = st.catches as f64 / bats_n as f64;
    println!(
        "  regime '{name}': {bats_n} bat(s), {} ticks",
        TICKS
    );
    println!("    chirps emitted:      {}", st.chirps);
    println!("    chirps heard (foreign): {}", st.chirps_heard);
    println!("    own returns heard:   {}", st.own_returns);
    println!("    foreign returns heard: {}", st.foreign_returns);
    println!("    returns masked:      {}", st.masked_returns);
    println!("    pursuits begun:      {}", st.pursuits);
    println!("    false pursuits (foreign echo believed): {}", st.false_pursuits);
    println!("    stone visits (echo lied about food): {}", st.stone_visits);
    println!("    chirp delays (polite JAR): {}", st.delays);
    println!("    delays per bat:      {:?}", st.per_bat_delays);
    println!(
        "    moths caught:        {} (per bat {:.2}; per-bat {:?})",
        st.catches, per_bat, st.per_bat_catches
    );
}

fn run_headless() {
    println!("listener-choir headless: one cave, three ways of singing in it");
    println!(
        "  cave {:.0}x{:.0}, {} moths, {} stones, c = {:.1} units/tick, window {} ticks",
        WORLD_W,
        WORLD_H,
        MOTH_STARTS.len(),
        STONES.len(),
        C,
        MAX_DELAY
    );
    let solo = run_regime(Choir::Solo);
    print_regime("solo", 1, &solo);
    let naive = run_regime(Choir::Naive);
    print_regime("choir, naive", 3, &naive);
    let polite = run_regime(Choir::Polite);
    print_regime("choir, polite", 3, &polite);
    let solo_pb = solo.catches as f64;
    let naive_pb = naive.catches as f64 / 3.0;
    let polite_pb = polite.catches as f64 / 3.0;
    println!("  the comparison the experiment exists to make:");
    println!(
        "    catches per bat — solo {:.2}, naive {:.2}, polite {:.2}",
        solo_pb, naive_pb, polite_pb
    );
    println!(
        "    foreign returns — naive {}, polite {}",
        naive.foreign_returns, polite.foreign_returns
    );
    println!(
        "    verdict: jamming {} the naive choir; chirp delays {} the polite choir",
        if naive_pb < solo_pb { "COST" } else { "did NOT cost" },
        if polite_pb > naive_pb { "RECOVERED" } else { "did NOT recover" }
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
        println!("listener-choir: three bats walk into one cave. The cave is the medium; the medium does not sign its work.");
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
        // Pluck here, arrive there: emitter at the origin strikes a
        // reflector 10 units out; its own echo travels 20 units at
        // c = 2 and must land exactly 10 ticks after the chirp.
        let t = echo_arrival_tick(100, (0.0, 0.0), (10.0, 0.0), (0.0, 0.0));
        assert_eq!(t, 110);
        // A listener standing beside the reflector hears the same
        // wavefront after only the outgoing leg: 5 ticks.
        let t2 = echo_arrival_tick(100, (0.0, 0.0), (10.0, 0.0), (10.0, 0.0));
        assert_eq!(t2, 105);
        // And amplitude dies with path length, always.
        assert!(
            echo_amplitude((0.0, 0.0), (10.0, 0.0), (0.0, 0.0))
                > echo_amplitude((0.0, 0.0), (30.0, 0.0), (0.0, 0.0))
        );
    }

    #[test]
    fn a_foreign_echo_spells_a_false_range() {
        // Bat A chirps east at tick 0 from (25,45); a reflector sits
        // 20 units ahead of A. Bat B, at (25,22), chirped at tick 0
        // too (unison). A's echo reaches B after (20 + dist(R,B))/c
        // ticks; read against B's own chirp, B computes a range that
        // is NOT the distance to anything in front of B. That gap
        // between spelled range and true geometry is the jamming.
        let a = (25.0, 45.0);
        let b = (25.0, 22.0);
        let r = (45.0, 45.0);
        let arrival = echo_arrival_tick(0, a, r, b);
        let spelled_range = (arrival - 0) as f64 * C / 2.0;
        let true_range_b_to_r = dist(b, r);
        assert!(
            (spelled_range - true_range_b_to_r).abs() > 3.0,
            "foreign echo should mislead: spelled {spelled_range}, true {true_range_b_to_r}"
        );
    }

    #[test]
    fn chirp_delays_only_when_jammed_and_only_when_polite() {
        let mut bat = Bat::new((25.0, 45.0));
        bat.next_chirp = 100;
        bat.window_jammed = false;
        assert!(!bat.maybe_delay_next(true));
        assert_eq!(bat.next_chirp, 100);
        bat.window_jammed = true;
        assert!(!bat.maybe_delay_next(false), "naive bats never learn");
        assert_eq!(bat.next_chirp, 100);
        assert!(bat.maybe_delay_next(true));
        assert_eq!(bat.next_chirp, 100 + DELAY_STEP);
        assert_eq!(bat.period, BASE_PERIOD, "the tempo itself never changes");
    }

    #[test]
    fn the_solo_bat_actually_hunts() {
        let st = run_regime(Choir::Solo);
        assert!(st.catches > 0, "a solo bat in an honest cave must eat");
        assert_eq!(st.foreign_returns, 0, "nobody else is singing");
        assert!(st.own_returns > 0);
    }

    #[test]
    fn unison_jams_and_politeness_recovers() {
        let solo = run_regime(Choir::Solo);
        let naive = run_regime(Choir::Naive);
        let polite = run_regime(Choir::Polite);
        assert!(
            naive.foreign_returns > 0,
            "a synchronized choir must hear each other"
        );
        assert!(
            naive.false_pursuits > 0,
            "some of those foreign returns must be believed"
        );
        let solo_pb = solo.catches as f64;
        let naive_pb = naive.catches as f64 / 3.0;
        assert!(
            naive_pb < solo_pb,
            "jamming must cost: naive per bat {naive_pb}, solo {solo_pb}"
        );
        assert!(
            polite.foreign_returns < naive.foreign_returns,
            "chirp delays must thin the jamming: polite {} vs naive {}",
            polite.foreign_returns,
            naive.foreign_returns
        );
        let polite_pb = polite.catches as f64 / 3.0;
        assert!(
            polite_pb > naive_pb,
            "politeness must recover catches: polite {polite_pb}, naive {naive_pb}"
        );
    }
}
