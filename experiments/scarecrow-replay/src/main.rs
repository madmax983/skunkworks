//! scarecrow-replay — The Scarecrow 🌾 plants Harvest's journal
//! and replay in `sono-flock` (The Cartographer of Lost Things 🗺️).
//!
//! The host couples a flock to a wave field in both directions:
//! boids pluck the field, loud cells jitter the boids. The coupling
//! is a circle — the field moves the flock, the flock strikes the
//! field — and a circle is exactly what a journal is for. Harvest's
//! rule: work that is written down can be replayed, and the replay
//! is the same work.
//!
//! The journal here holds ONLY the plucks: (tick, boid, x, y), in
//! the order struck. It does not hold the jitter (that is derived —
//! the field, replayed, will be loud in the same cells at the same
//! ticks, and the jitter stream, shared and seeded, will be drawn
//! in the same order). It does not hold any boid (positions are
//! derived too). If the journal is complete, the replay must land
//! boid for boid where the live run landed, bitwise.
//!
//! The boid index is in the journal on purpose, and the reason is
//! in the parent crate: `PhysicsGrid::pluck` adds to the field's
//! present level at once, and a boid later in the flock order reads
//! the field in the SAME tick it was struck. A journal that named
//! only the tick would have to guess where in the tick each strike
//! fell. The naive replay below makes exactly that guess — it
//! applies each tick's strikes at the end of the tick — and the
//! ledger prices the guess.
//!
//! Five plantings of the same row:
//! - Live:    the host's coupled run, unchanged, writing the journal.
//! - Replay:  no pluck coin at all — the journal strikes the field,
//!            each strike at its own boid's turn.
//! - Naive:   the same journal, strikes applied at end of tick.
//! - Torn:    the journal's second half burned (entries from tick
//!            600 on are missing). The replay must agree with the
//!            live run until the tear, then part from it.
//! - Shifted: one entry moved one cell east (the journal's first
//!            pluck). A small lie, written down early.
//!
//! The host's machinery, constants, parameters, and streams are
//! copied faithfully from `sono-flock` and unchanged; tests pin
//! this crate's mute/deaf/coupled ledgers to the host's published
//! ones. A per-tick checksum over every boid's position bits names
//! the first tick two runs disagree — divergence has an address.

use anyhow::Result;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use resonance_audio::PhysicsGrid;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;
const BOIDS: usize = 50;
const TICKS: usize = 1200;
const PLUCK_PROB: f64 = 0.01;
const PLUCK_STRENGTH: f32 = 0.5;
const WAVE_THRESHOLD: f32 = 0.2;
const TEAR_TICK: u16 = 600;
/// Packed journal record: tick (2 bytes) + boid (1) + x (1) + y (1).
const JOURNAL_BYTES_PER_EVENT: usize = 5;

/// Deterministic SplitMix64 — one stream per purpose, the host's
/// own design, copied unchanged.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Regime {
    Mute,
    Deaf,
    Coupled,
}

impl Regime {
    fn plucks(self) -> bool {
        self != Regime::Mute
    }
    fn listens(self) -> bool {
        self == Regime::Coupled
    }
}

struct Boid {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,
}

/// The host's flocking parameters, verbatim.
fn corpse_params() -> FlockingParams {
    FlockingParams {
        view_radius: 10.0,
        separation_radius: 5.0,
        max_speed: 1.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    }
}

fn torus_delta(a: f64, b: f64, extent: f64) -> f64 {
    let d = (a - b).abs();
    d.min(extent - d)
}

fn mean_pairwise_toroidal(boids: &[Boid]) -> f64 {
    let mut sum = 0.0;
    let mut n = 0usize;
    for i in 0..boids.len() {
        for j in (i + 1)..boids.len() {
            let dx = torus_delta(boids[i].pos.x, boids[j].pos.x, WIDTH as f64);
            let dy = torus_delta(boids[i].pos.y, boids[j].pos.y, HEIGHT as f64);
            sum += (dx * dx + dy * dy).sqrt();
            n += 1;
        }
    }
    sum / n as f64
}

fn polarization(boids: &[Boid]) -> f64 {
    let mut sx = 0.0;
    let mut sy = 0.0;
    for b in boids {
        let m = b.vel.magnitude();
        if m > 0.0 {
            sx += b.vel.x / m;
            sy += b.vel.y / m;
        }
    }
    ((sx * sx + sy * sy).sqrt()) / boids.len() as f64
}

/// FNV-1a over every boid's position bits, in flock order. Two runs
/// with the same checksum at a tick hold the same field of birds,
/// bit for bit, at that tick.
fn flock_checksum(boids: &[Boid]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in boids {
        for v in [b.pos.x.to_bits(), b.pos.y.to_bits()] {
            for byte in v.to_le_bytes() {
                h ^= byte as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
    }
    h
}

/// One journal entry: a pluck, written down as it was struck —
/// when, by whom, and where.
type Pluck = (u16, u8, u8, u8);

/// How the coupled run's plucks are decided.
enum Drive<'a> {
    /// The live run: the host's pluck coin decides, and every
    /// strike is written into the journal being built.
    Live,
    /// A replay: no coin exists in this run at all. The journal
    /// strikes the field at each entry's own boid-turn.
    Replay(&'a [Pluck]),
    /// A naive replay: the same journal, but each tick's strikes
    /// are applied at the end of the tick — the journal read as if
    /// a tick were an instant, which it is not.
    Naive(&'a [Pluck]),
}

#[derive(Debug, Clone)]
struct Ledger {
    plucks: u64,
    jitter_events: u64,
    jitter_ticks: u64,
    field_max_abs: f32,
    field_final_abs_sum: f64,
    pol_initial: f64,
    pol_final: f64,
    pair_initial: f64,
    pair_final: f64,
    final_positions: Vec<(f64, f64)>,
    /// Per-tick flock checksums, for naming the first divergence.
    checksums: Vec<u64>,
}

fn spawn() -> Vec<Boid> {
    let mut spawn_rng = SplitMix64(0x5050_4F4E);
    let mut boids: Vec<Boid> = Vec::with_capacity(BOIDS);
    for _ in 0..BOIDS {
        let vx = spawn_rng.range(-1.0, 1.0);
        let vy = spawn_rng.range(-1.0, 1.0);
        let vel = Vec2::new(vx, vy).normalize();
        boids.push(Boid {
            pos: Vec2::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0),
            vel,
            accel: Vec2::zero(),
        });
    }
    boids
}

/// The host's regimes, run live. Used for the anchor checks.
fn run_regime(regime: Regime, ticks: usize) -> Ledger {
    run_coupled(regime, ticks, Drive::Live).0
}

/// The coupled machinery, shared by the live run and every replay.
/// In a replay the pluck coin is never created: if the journal is
/// complete and the machinery is honest, the derived jitter stream
/// is drawn in exactly the live order, because the field it reads
/// is the same field, struck at the same turns.
fn run_coupled(regime: Regime, ticks: usize, drive: Drive) -> (Ledger, Vec<Pluck>) {
    let mut boids = spawn();
    let pol_initial = polarization(&boids);
    let pair_initial = mean_pairwise_toroidal(&boids);

    let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
    let params = corpse_params();
    let mut pluck_rng = SplitMix64(0x915C_0DE1);
    let mut jitter_rng = SplitMix64(0x1177_E12A);
    let mut plucks: u64 = 0;
    let mut jitter_events: u64 = 0;
    let mut jitter_ticks: u64 = 0;
    let mut field_max_abs: f32 = 0.0;
    let mut checksums: Vec<u64> = Vec::with_capacity(ticks);
    let mut journal: Vec<Pluck> = Vec::new();
    // Replay cursor: journal entries are in (tick, boid) order.
    let mut cursor = 0usize;
    let replay_journal: &[Pluck] = match &drive {
        Drive::Live => &[],
        Drive::Replay(j) | Drive::Naive(j) => j,
    };
    let naive = matches!(drive, Drive::Naive(_));

    for tick in 0..ticks {
        grid.step();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let v = grid.get(x, y).abs();
                if v > field_max_abs {
                    field_max_abs = v;
                }
            }
        }

        let positions: Vec<Vec2> = boids.iter().map(|b| b.pos).collect();
        let velocities: Vec<Vec2> = boids.iter().map(|b| b.vel).collect();
        let mut jittered_this_tick = false;

        for i in 0..boids.len() {
            let force = compute_force(&positions, &velocities, i, &params);
            {
                let b = &mut boids[i];
                b.accel += force;
                b.vel += b.accel;
                b.vel = b.vel.limit(params.max_speed);
                b.pos += b.vel;
                b.accel = Vec2::zero();
                if b.pos.x < 0.0 {
                    b.pos.x += WIDTH as f64;
                }
                if b.pos.x >= WIDTH as f64 {
                    b.pos.x -= WIDTH as f64;
                }
                if b.pos.y < 0.0 {
                    b.pos.y += HEIGHT as f64;
                }
                if b.pos.y >= HEIGHT as f64 {
                    b.pos.y -= HEIGHT as f64;
                }

                if regime.listens() {
                    let gx = b.pos.x as usize;
                    let gy = b.pos.y as usize;
                    if gx < WIDTH && gy < HEIGHT && grid.get(gx, gy).abs() > WAVE_THRESHOLD {
                        let jx = jitter_rng.range(-0.5, 0.5);
                        let jy = jitter_rng.range(-0.5, 0.5);
                        b.accel += Vec2::new(jx, jy);
                        jitter_events += 1;
                        jittered_this_tick = true;
                    }
                }
            }

            match &drive {
                Drive::Live => {
                    if regime.plucks() && pluck_rng.next_f64() < PLUCK_PROB {
                        let gx = boids[i].pos.x as usize;
                        let gy = boids[i].pos.y as usize;
                        if gx < WIDTH && gy < HEIGHT {
                            grid.pluck(gx, gy, PLUCK_STRENGTH);
                            plucks += 1;
                            if regime == Regime::Coupled {
                                journal.push((tick as u16, i as u8, gx as u8, gy as u8));
                            }
                        }
                    }
                }
                Drive::Replay(_) => {
                    while cursor < replay_journal.len()
                        && replay_journal[cursor].0 == tick as u16
                        && replay_journal[cursor].1 == i as u8
                    {
                        let (_, _, gx, gy) = replay_journal[cursor];
                        grid.pluck(gx as usize, gy as usize, PLUCK_STRENGTH);
                        plucks += 1;
                        cursor += 1;
                    }
                }
                Drive::Naive(_) => {}
            }
        }

        if naive {
            while cursor < replay_journal.len() && replay_journal[cursor].0 == tick as u16 {
                let (_, _, gx, gy) = replay_journal[cursor];
                grid.pluck(gx as usize, gy as usize, PLUCK_STRENGTH);
                plucks += 1;
                cursor += 1;
            }
        }

        if jittered_this_tick {
            jitter_ticks += 1;
        }
        checksums.push(flock_checksum(&boids));
    }

    let mut field_final_abs_sum = 0.0f64;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            field_final_abs_sum += grid.get(x, y).abs() as f64;
        }
    }

    let ledger = Ledger {
        plucks,
        jitter_events,
        jitter_ticks,
        field_max_abs,
        field_final_abs_sum,
        pol_initial,
        pol_final: polarization(&boids),
        pair_initial,
        pair_final: mean_pairwise_toroidal(&boids),
        final_positions: boids.iter().map(|b| (b.pos.x, b.pos.y)).collect(),
        checksums,
    };
    (ledger, journal)
}

fn first_divergence(a: &[u64], b: &[u64]) -> Option<usize> {
    a.iter().zip(b.iter()).position(|(x, y)| x != y)
}

fn print_coupled(label: &str, l: &Ledger) {
    println!(
        "{}: plucks {}, jitter events {}, jitter ticks {}, field max |p| {:.4}, field final sum |p| {:.4}",
        label, l.plucks, l.jitter_events, l.jitter_ticks, l.field_max_abs, l.field_final_abs_sum
    );
    println!(
        "{}: polarization {:.4} -> {:.4}, mean pairwise toroidal distance {:.4} -> {:.4}",
        label, l.pol_initial, l.pol_final, l.pair_initial, l.pair_final
    );
}

fn headless() {
    println!("scarecrow-replay headless ledger");
    println!(
        "host: sono-flock (flocking + locus + resonance-audio PhysicsGrid {}x{}), {} boids, {} ticks, pluck coin {}, pluck strength {}, wave threshold {}",
        WIDTH, HEIGHT, BOIDS, TICKS, PLUCK_PROB, PLUCK_STRENGTH, WAVE_THRESHOLD
    );
    println!("principle: journal and replay (Harvest) — the journal holds only the plucks (tick, boid, cell); the flock, the field, and the jitter must be derived");

    // Anchors: the host, standing as published.
    let mute = run_regime(Regime::Mute, TICKS);
    let deaf = run_regime(Regime::Deaf, TICKS);
    println!(
        "anchor mute: plucks {}, field max |p| {:.4}, polarization {:.4} -> {:.4}",
        mute.plucks, mute.field_max_abs, mute.pol_initial, mute.pol_final
    );
    println!(
        "anchor deaf: plucks {}, field max |p| {:.4}, final positions == mute: {}",
        deaf.plucks,
        deaf.field_max_abs,
        deaf.final_positions == mute.final_positions
    );

    // The live coupled run, writing its journal as it goes.
    let (live, journal) = run_coupled(Regime::Coupled, TICKS, Drive::Live);
    print_coupled("live coupled", &live);
    let journal_bytes = journal.len() * JOURNAL_BYTES_PER_EVENT;
    let snapshot_bytes = WIDTH * HEIGHT * 4;
    println!(
        "journal: {} pluck events, {} bytes packed ({} bytes/event); one field snapshot would be {} bytes ({} cells x f32, present level only — and granary priced what a present-only store costs)",
        journal.len(),
        journal_bytes,
        JOURNAL_BYTES_PER_EVENT,
        snapshot_bytes,
        WIDTH * HEIGHT
    );

    // The replay: no coin, only the written strikes, each at its turn.
    let (replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Replay(&journal));
    print_coupled("replay (full journal)", &replay);
    println!(
        "replay == live: final positions {}, jitter events {} == {}, field final sum equal {}, first divergence {:?}",
        replay.final_positions == live.final_positions,
        replay.jitter_events,
        live.jitter_events,
        replay.field_final_abs_sum == live.field_final_abs_sum,
        first_divergence(&live.checksums, &replay.checksums)
    );

    // The naive replay: the same journal, read as if a tick were
    // an instant.
    let (naive_replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Naive(&journal));
    print_coupled("replay (naive, end of tick)", &naive_replay);
    println!(
        "naive replay: first divergence vs live at tick {:?}; final positions == live: {}",
        first_divergence(&live.checksums, &naive_replay.checksums),
        naive_replay.final_positions == live.final_positions
    );

    // The torn journal: the second half burned in the winter.
    let torn: Vec<Pluck> = journal.iter().copied().filter(|(t, _, _, _)| *t < TEAR_TICK).collect();
    let (torn_replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Replay(&torn));
    print_coupled("replay (torn at 600)", &torn_replay);
    println!(
        "torn journal: {} of {} events survived; first divergence vs live at tick {:?}; final positions == live: {}",
        torn.len(),
        journal.len(),
        first_divergence(&live.checksums, &torn_replay.checksums),
        torn_replay.final_positions == live.final_positions
    );

    // The shifted journal: the first pluck written one cell east.
    let mut shifted = journal.clone();
    if let Some(first) = shifted.first_mut() {
        first.2 = (first.2 + 1) % (WIDTH as u8);
    }
    let (shifted_replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Replay(&shifted));
    print_coupled("replay (first pluck +1 cell)", &shifted_replay);
    println!(
        "shifted journal: first pluck was at tick {}, by boid {}, at cell ({}, {}); first divergence vs live at tick {:?}; final positions == live: {}",
        journal.first().map(|p| p.0).unwrap_or(0),
        journal.first().map(|p| p.1).unwrap_or(0),
        journal.first().map(|p| p.2).unwrap_or(0),
        journal.first().map(|p| p.3).unwrap_or(0),
        first_divergence(&live.checksums, &shifted_replay.checksums),
        shifted_replay.final_positions == live.final_positions
    );

    let p1 = replay.final_positions == live.final_positions
        && replay.jitter_events == live.jitter_events
        && replay.jitter_ticks == live.jitter_ticks
        && replay.field_final_abs_sum == live.field_final_abs_sum
        && first_divergence(&live.checksums, &replay.checksums).is_none();
    let p2 = journal_bytes < snapshot_bytes;
    let naive_div = first_divergence(&live.checksums, &naive_replay.checksums);
    let p3 = naive_div.is_some() && naive_replay.final_positions != live.final_positions;
    let torn_div = first_divergence(&live.checksums, &torn_replay.checksums);
    let p4 = torn_div.map(|t| t >= TEAR_TICK as usize).unwrap_or(false)
        && torn_replay.final_positions != live.final_positions;
    let shifted_div = first_divergence(&live.checksums, &shifted_replay.checksums);
    let p5 = shifted_div.is_some() && shifted_replay.final_positions != live.final_positions;
    println!(
        "verdicts: P1 full journal replays exact: {}; P2 journal ({} B) smaller than one field snapshot ({} B): {}; P3 naive end-of-tick replay parts from live (first divergence {:?}): {}; P4 torn journal agrees until the tear then parts (first divergence {:?}): {}; P5 one shifted cell parts the flock (first divergence {:?}): {}",
        p1, journal_bytes, snapshot_bytes, p2, naive_div, p3, torn_div, p4, shifted_div, p5
    );
}

fn main() -> Result<()> {
    // --headless is handled BEFORE anything else; this specimen has
    // no TUI at all.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-replay: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_anchors_hold() {
        // The host's published ledger (sono-flock README, run 56):
        // mute polarization 0.1324 -> 0.9518 and a silent field;
        // deaf 566 plucks, field max 0.9595, positions == mute.
        let mute = run_regime(Regime::Mute, TICKS);
        assert_eq!(mute.plucks, 0);
        assert_eq!(mute.field_max_abs, 0.0);
        assert!((mute.pol_final - 0.9518).abs() < 0.0001, "mute pol {}", mute.pol_final);
        let deaf = run_regime(Regime::Deaf, TICKS);
        assert_eq!(deaf.plucks, 566);
        assert!((deaf.field_max_abs - 0.9595).abs() < 0.0001, "deaf max {}", deaf.field_max_abs);
        assert_eq!(deaf.final_positions, mute.final_positions);
    }

    #[test]
    fn live_coupled_matches_host() {
        // The host's coupled ledger: 566 plucks, 9,832 jitter events
        // on 1,138 ticks, polarization 0.8092, spread 21.7713.
        let (live, journal) = run_coupled(Regime::Coupled, TICKS, Drive::Live);
        assert_eq!(live.plucks, 566);
        assert_eq!(journal.len(), 566);
        assert_eq!(live.jitter_events, 9832);
        assert_eq!(live.jitter_ticks, 1138);
        assert!((live.pol_final - 0.8092).abs() < 0.0001, "pol {}", live.pol_final);
        assert!((live.pair_final - 21.7713).abs() < 0.0001, "pair {}", live.pair_final);
    }

    #[test]
    fn full_journal_replays_exact() {
        // The principle, pinned: a replay with no pluck coin at all
        // must land boid for boid where the live run landed.
        let (live, journal) = run_coupled(Regime::Coupled, TICKS, Drive::Live);
        let (replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Replay(&journal));
        assert_eq!(replay.final_positions, live.final_positions);
        assert_eq!(replay.jitter_events, live.jitter_events);
        assert_eq!(replay.jitter_ticks, live.jitter_ticks);
        assert_eq!(replay.field_final_abs_sum, live.field_final_abs_sum);
        assert_eq!(first_divergence(&live.checksums, &replay.checksums), None);
    }

    #[test]
    fn torn_journal_agrees_until_the_tear_then_parts() {
        let (live, journal) = run_coupled(Regime::Coupled, TICKS, Drive::Live);
        let torn: Vec<Pluck> = journal.iter().copied().filter(|(t, _, _, _)| *t < TEAR_TICK).collect();
        assert!(torn.len() < journal.len(), "the tear must actually remove entries");
        let (replay, _) = run_coupled(Regime::Coupled, TICKS, Drive::Replay(&torn));
        let div = first_divergence(&live.checksums, &replay.checksums)
            .expect("a torn journal must eventually part from the live run");
        assert!(div >= TEAR_TICK as usize, "diverged at {div}, before the tear");
        assert_ne!(replay.final_positions, live.final_positions);
    }

    #[test]
    fn journal_is_smaller_than_one_snapshot() {
        let (_, journal) = run_coupled(Regime::Coupled, TICKS, Drive::Live);
        assert!(journal.len() * JOURNAL_BYTES_PER_EVENT < WIDTH * HEIGHT * 4);
    }

    #[test]
    fn checksum_names_a_planted_difference() {
        // The divergence instrument itself: two flocks differing in
        // one boid's position must checksum differently.
        let a = spawn();
        let mut b = spawn();
        b[0].pos.x += 1.0;
        assert_ne!(flock_checksum(&a), flock_checksum(&b));
        assert_eq!(flock_checksum(&a), flock_checksum(&spawn()));
    }
}
