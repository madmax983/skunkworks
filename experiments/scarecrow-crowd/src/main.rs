//! scarecrow-crowd — The Scarecrow 🌾 plants Harvest's journal
//! and replay in `sono-flock` (The Cartographer of Lost Things 🗺️)
//! a second time, in a denser field.
//!
//! `scarecrow-replay` wrote the coupled flock's season down as
//! only its plucks — (tick, boid, cell) — and the replay landed
//! exact. One prediction cracked there, and it named this run:
//! the naive replay, which applies each tick's strikes at the end
//! of the tick, was *also* exact. Across 566 strikes, not one was
//! heard within its own tick. The within-tick channel exists in
//! the parent crate's surface — `PhysicsGrid::pluck` adds to the
//! field's present level at once, and a boid later in the flock
//! order reads the field in the same tick it was struck — but at
//! 50 boids in an 80x40 field it carried nothing. The journal's
//! boid index stood as named, priced insurance nobody claimed.
//!
//! This planting asks what density does to that channel. The
//! host's machinery, field, constants, and streams are unchanged;
//! only the flock's size varies: 50 (the host's own crowd, the
//! anchor), 200, and 400 boids. Every crowd runs three ways:
//! - Live:   the host's coupled run, writing its journal, with a
//!           counter standing on the channel itself (see below).
//! - Replay: no pluck coin — the journal strikes the field, each
//!           strike at its own boid's turn.
//! - Naive:  the same journal, strikes at the end of their tick.
//!
//! The channel counter, kept on the live run: at each boid's
//! jitter read, the field value it hears is compared against the
//! value it would have heard without this tick's strikes so far
//! (the run keeps a per-tick tally of same-tick strike strength
//! per cell; a strike adds exactly +0.5, so the tally is exact).
//! - contact:  the boid's cell was struck earlier this same tick.
//! - decisive: it hears the field *because* of the same-tick
//!             strikes — |heard| > 0.2 while |unstuck| <= 0.2.
//! - masked:   the strikes *silenced* a hearing — |heard| <= 0.2
//!             while |unstuck| > 0.2 (a +0.5 strike laid over a
//!             negative wave can pull a loud cell under the line).
//!
//! The journal widens in this field: a boid index no longer fits
//! in one byte past 255 birds, so an entry is (tick u16, boid
//! u16, x u8, y u8) — 6 bytes, one more than the sparse field's
//! 5. Density taxes the journal's name field before it taxes
//! anything else; the ledger says by how much, and whether the
//! journal still costs less than one snapshot of the field.
//!
//! A per-tick checksum over every boid's position bits names the
//! first tick two runs disagree — divergence has an address.

use anyhow::Result;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use resonance_audio::PhysicsGrid;

const WIDTH: usize = 80;
const HEIGHT: usize = 40;
const TICKS: usize = 1200;
const PLUCK_PROB: f64 = 0.01;
const PLUCK_STRENGTH: f32 = 0.5;
const WAVE_THRESHOLD: f32 = 0.2;
const DENSITIES: [usize; 3] = [50, 200, 400];
/// Packed journal record: tick (2 bytes) + boid (2) + x (1) + y (1).
const JOURNAL_BYTES_PER_EVENT: usize = 6;

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
    // The host's other two regimes exist for parity with
    // sono-flock's ablation; this survey runs only the coupled
    // regime, at three densities.
    #[allow(dead_code)]
    Mute,
    #[allow(dead_code)]
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

/// FNV-1a over every boid's position bits, in flock order.
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
type Pluck = (u16, u16, u8, u8);

/// How the coupled run's plucks are decided.
enum Drive<'a> {
    Live,
    Replay(&'a [Pluck]),
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
    /// Within-tick channel counter (live and true replay only;
    /// the naive run reads a field its strikes have not touched
    /// yet this tick, so its counter is zero by construction).
    contacts: u64,
    decisive: u64,
    masked: u64,
    final_positions: Vec<(f64, f64)>,
    checksums: Vec<u64>,
}

fn spawn(n_boids: usize) -> Vec<Boid> {
    let mut spawn_rng = SplitMix64(0x5050_4F4E);
    let mut boids: Vec<Boid> = Vec::with_capacity(n_boids);
    for _ in 0..n_boids {
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

fn run_coupled(n_boids: usize, regime: Regime, ticks: usize, drive: Drive) -> (Ledger, Vec<Pluck>) {
    let mut boids = spawn(n_boids);
    let pol_initial = polarization(&boids);
    let pair_initial = mean_pairwise_toroidal(&boids);

    let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
    let params = corpse_params();
    let mut pluck_rng = SplitMix64(0x915C_0DE1);
    let mut jitter_rng = SplitMix64(0x1177_E12A);
    let mut plucks: u64 = 0;
    let mut jitter_events: u64 = 0;
    let mut jitter_ticks: u64 = 0;
    let mut contacts: u64 = 0;
    let mut decisive: u64 = 0;
    let mut masked: u64 = 0;
    let mut field_max_abs: f32 = 0.0;
    let mut checksums: Vec<u64> = Vec::with_capacity(ticks);
    let mut journal: Vec<Pluck> = Vec::new();
    let mut cursor = 0usize;
    let replay_journal: &[Pluck] = match &drive {
        Drive::Live => &[],
        Drive::Replay(j) | Drive::Naive(j) => j,
    };
    let naive = matches!(drive, Drive::Naive(_));
    let count_channel = !naive;
    // Same-tick strike tally per cell, exact: a strike adds
    // exactly PLUCK_STRENGTH to the cell's present level, and no
    // step intervenes within a tick.
    let mut tick_tally = vec![0.0f32; WIDTH * HEIGHT];
    let mut touched: Vec<usize> = Vec::new();

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
                    if gx < WIDTH && gy < HEIGHT {
                        let heard = grid.get(gx, gy);
                        if count_channel {
                            let tally = tick_tally[gy * WIDTH + gx];
                            if tally != 0.0 {
                                contacts += 1;
                                let unstruck = heard - tally;
                                if heard.abs() > WAVE_THRESHOLD
                                    && unstruck.abs() <= WAVE_THRESHOLD
                                {
                                    decisive += 1;
                                } else if heard.abs() <= WAVE_THRESHOLD
                                    && unstruck.abs() > WAVE_THRESHOLD
                                {
                                    masked += 1;
                                }
                            }
                        }
                        if heard.abs() > WAVE_THRESHOLD {
                            let jx = jitter_rng.range(-0.5, 0.5);
                            let jy = jitter_rng.range(-0.5, 0.5);
                            b.accel += Vec2::new(jx, jy);
                            jitter_events += 1;
                            jittered_this_tick = true;
                        }
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
                            let idx = gy * WIDTH + gx;
                            if tick_tally[idx] == 0.0 {
                                touched.push(idx);
                            }
                            tick_tally[idx] += PLUCK_STRENGTH;
                            if regime == Regime::Coupled {
                                journal.push((tick as u16, i as u16, gx as u8, gy as u8));
                            }
                        }
                    }
                }
                Drive::Replay(_) => {
                    while cursor < replay_journal.len()
                        && replay_journal[cursor].0 == tick as u16
                        && replay_journal[cursor].1 == i as u16
                    {
                        let (_, _, gx, gy) = replay_journal[cursor];
                        grid.pluck(gx as usize, gy as usize, PLUCK_STRENGTH);
                        plucks += 1;
                        let idx = gy as usize * WIDTH + gx as usize;
                        if tick_tally[idx] == 0.0 {
                            touched.push(idx);
                        }
                        tick_tally[idx] += PLUCK_STRENGTH;
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

        for idx in touched.drain(..) {
            tick_tally[idx] = 0.0;
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
        contacts,
        decisive,
        masked,
        final_positions: boids.iter().map(|b| (b.pos.x, b.pos.y)).collect(),
        checksums,
    };
    (ledger, journal)
}

fn first_divergence(a: &[u64], b: &[u64]) -> Option<usize> {
    a.iter().zip(b.iter()).position(|(x, y)| x != y)
}

struct DensityResult {
    live: Ledger,
    journal_len: usize,
    replay_exact: bool,
    replay_channel_same: bool,
    naive_div: Option<usize>,
    naive_final_same: bool,
}

fn survey_density(n: usize) -> DensityResult {
    let (live, journal) = run_coupled(n, Regime::Coupled, TICKS, Drive::Live);
    let (replay, _) = run_coupled(n, Regime::Coupled, TICKS, Drive::Replay(&journal));
    let (naive_replay, _) = run_coupled(n, Regime::Coupled, TICKS, Drive::Naive(&journal));
    DensityResult {
        replay_exact: replay.final_positions == live.final_positions
            && replay.jitter_events == live.jitter_events
            && replay.jitter_ticks == live.jitter_ticks
            && replay.field_final_abs_sum == live.field_final_abs_sum
            && first_divergence(&live.checksums, &replay.checksums).is_none(),
        replay_channel_same: replay.contacts == live.contacts
            && replay.decisive == live.decisive
            && replay.masked == live.masked,
        naive_div: first_divergence(&live.checksums, &naive_replay.checksums),
        naive_final_same: naive_replay.final_positions == live.final_positions,
        journal_len: journal.len(),
        live,
    }
}

fn headless() {
    println!("scarecrow-crowd headless ledger");
    println!(
        "host: sono-flock (flocking + locus + resonance-audio PhysicsGrid {}x{}), {} ticks, pluck coin {}, pluck strength {}, wave threshold {}; densities {:?} boids",
        WIDTH, HEIGHT, TICKS, PLUCK_PROB, PLUCK_STRENGTH, WAVE_THRESHOLD, DENSITIES
    );
    println!("principle: journal and replay (Harvest) — the journal holds only the plucks (tick, boid, cell); the flock, the field, and the jitter must be derived; the naive replay prices the journal's within-tick order");
    let snapshot_bytes = WIDTH * HEIGHT * 4;

    let mut results = Vec::new();
    for &n in &DENSITIES {
        let r = survey_density(n);
        let l = &r.live;
        let journal_bytes = r.journal_len * JOURNAL_BYTES_PER_EVENT;
        println!(
            "density {}: live plucks {}, jitter events {}, jitter ticks {}, field max |p| {:.4}, field final sum |p| {:.4}, polarization {:.4} -> {:.4}, spread {:.4} -> {:.4}",
            n, l.plucks, l.jitter_events, l.jitter_ticks, l.field_max_abs,
            l.field_final_abs_sum, l.pol_initial, l.pol_final, l.pair_initial, l.pair_final
        );
        println!(
            "density {}: within-tick channel — contacts {}, decisive {}, masked {}",
            n, l.contacts, l.decisive, l.masked
        );
        println!(
            "density {}: journal {} events, {} bytes packed ({} bytes/event); one field snapshot {} bytes; journal smaller than snapshot: {}",
            n, r.journal_len, journal_bytes, JOURNAL_BYTES_PER_EVENT, snapshot_bytes,
            journal_bytes < snapshot_bytes
        );
        println!(
            "density {}: replay exact {}, replay channel counter identical {}; naive first divergence {:?}, naive final == live {}",
            n, r.replay_exact, r.replay_channel_same, r.naive_div, r.naive_final_same
        );
        results.push(r);
    }

    let d50 = &results[0];
    let d400 = &results[2];
    let b1 = d50.live.plucks == 566
        && d50.live.jitter_events == 9832
        && d50.live.jitter_ticks == 1138
        && (d50.live.pol_final - 0.8092).abs() < 0.0001
        && (d50.live.pair_final - 21.7713).abs() < 0.0001;
    let p1 = results.iter().all(|r| r.replay_exact);
    let p2 = results[0].live.contacts < results[1].live.contacts
        && results[1].live.contacts < results[2].live.contacts;
    let p3 = d50.live.decisive == 0 && d400.live.decisive > 0;
    let p4 = d400.naive_div.is_some() && !d400.naive_final_same;
    let p5 = d50.journal_len * JOURNAL_BYTES_PER_EVENT < snapshot_bytes
        && d400.journal_len * JOURNAL_BYTES_PER_EVENT > snapshot_bytes;
    let p6 = d50.naive_div.is_none() && d50.naive_final_same;
    println!(
        "verdicts: B1 host anchor at 50 exact: {}; P1 full journal replays exact at every density: {}; P2 contacts rise with density ({} < {} < {}): {}; P3 channel silent at 50 (decisive {}) and awake at 400 (decisive {}): {}; P4 naive replay parts at 400 (first divergence {:?}): {}; P5 journal crosses one snapshot between 50 ({} B) and 400 ({} B) vs {} B: {}; P6 naive still exact at 50: {}",
        b1, p1,
        results[0].live.contacts, results[1].live.contacts, results[2].live.contacts, p2,
        d50.live.decisive, d400.live.decisive, p3,
        d400.naive_div, p4,
        d50.journal_len * JOURNAL_BYTES_PER_EVENT,
        d400.journal_len * JOURNAL_BYTES_PER_EVENT, snapshot_bytes, p5,
        p6
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
    println!("scarecrow-crowd: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_anchor_at_50_exact() {
        // The host's published coupled ledger (sono-flock /
        // scarecrow-replay): 566 plucks, 9,832 jitter events on
        // 1,138 ticks, polarization 0.8092, spread 21.7713 — and
        // the within-tick channel silent: zero decisive crossings.
        let (live, journal) = run_coupled(50, Regime::Coupled, TICKS, Drive::Live);
        assert_eq!(live.plucks, 566);
        assert_eq!(journal.len(), 566);
        assert_eq!(live.jitter_events, 9832);
        assert_eq!(live.jitter_ticks, 1138);
        assert!((live.pol_final - 0.8092).abs() < 0.0001, "pol {}", live.pol_final);
        assert!((live.pair_final - 21.7713).abs() < 0.0001, "pair {}", live.pair_final);
        assert_eq!(live.decisive, 0);
    }

    #[test]
    fn full_journal_replays_exact_at_200() {
        // The principle, pinned at four times the host's density:
        // a replay with no pluck coin must land boid for boid
        // where the live run landed, channel counter included.
        let (live, journal) = run_coupled(200, Regime::Coupled, TICKS, Drive::Live);
        let (replay, _) = run_coupled(200, Regime::Coupled, TICKS, Drive::Replay(&journal));
        assert_eq!(replay.final_positions, live.final_positions);
        assert_eq!(replay.jitter_events, live.jitter_events);
        assert_eq!(replay.jitter_ticks, live.jitter_ticks);
        assert_eq!(replay.field_final_abs_sum, live.field_final_abs_sum);
        assert_eq!(first_divergence(&live.checksums, &replay.checksums), None);
        assert_eq!(replay.contacts, live.contacts);
        assert_eq!(replay.decisive, live.decisive);
        assert_eq!(replay.masked, live.masked);
    }

    #[test]
    fn contacts_rise_with_density() {
        // The channel counter is a density instrument: a denser
        // crowd must produce strictly more within-tick contacts
        // than the host's sparse field.
        let (sparse, _) = run_coupled(50, Regime::Coupled, TICKS, Drive::Live);
        let (dense, _) = run_coupled(200, Regime::Coupled, TICKS, Drive::Live);
        assert!(dense.contacts > sparse.contacts,
            "contacts {} (200) vs {} (50)", dense.contacts, sparse.contacts);
    }

    #[test]
    fn checksum_names_a_planted_difference() {
        let a = spawn(50);
        let mut b = spawn(50);
        b[0].pos.x += 1.0;
        assert_ne!(flock_checksum(&a), flock_checksum(&b));
        assert_eq!(flock_checksum(&a), flock_checksum(&spawn(50)));
    }
}
