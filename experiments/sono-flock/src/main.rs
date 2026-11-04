//! sono-flock — a revival cross: graveyard/sono-boids × crates/flocking
//! + crates/locus + crates/resonance-audio.
//!
//! The corpse (267 LOC, edition 2021) coupled a boid flock to an FDTD
//! wave field in both directions: boids plucked the field at their
//! cells, and loud cells jittered the boids flying through them. It
//! was executed for a single named compilation error — E0603, a
//! private-module access: it imported `resonance_audio::audio::*`,
//! and `audio` was a private module. It never compiled in its final
//! form, and its loop was never counted.
//!
//! The living garden now holds every organ it reached for:
//! `flocking::compute_force` with the corpse's own parameter values,
//! `locus::Vec2`, and `resonance_audio::PhysicsGrid` — the field the
//! corpse tried to reach through the private door, public all along
//! at the crate root (its `AudioModel` sibling is re-exported at the
//! root today too; the E0603 door is simply open now).
//!
//! Three regimes run the corpse's loop on identical spawn, identical
//! streams, so the coupling is priced by ablation:
//! - Mute:    the flock flies, nobody plucks; the field stays silent.
//! - Deaf:    boids pluck, the field rings, boids never read it.
//! - Coupled: the corpse's full loop — plucks out, jitter back.
//!
//! Named adaptations (kept honest, as this garden keeps them):
//! - The corpse's `AudioModel` channel machinery is replaced by one
//!   `PhysicsGrid::step()` per tick, in the open, with no audio
//!   thread and no snapshot lag; the pluck and the read use the
//!   grid's public `pluck`/`get`.
//! - The corpse's three `thread_rng` draws (spawn velocity, pluck
//!   coin, wave jitter) become three independent SplitMix64 streams
//!   with fixed seeds, one per purpose, shared across regimes — so
//!   Mute and Deaf must agree boid for boid, and any divergence in
//!   Coupled is the wave's doing and nothing else's.
//! - The corpse's wave read used the snapshot from the previous
//!   audio buffer; here a boid reads the field stepped at the top
//!   of its own tick. The threshold (0.2), the jitter range
//!   (±0.5 per axis), the pluck coin (0.01), and the pluck strength
//!   (0.5) are the corpse's numbers, unretuned.

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

/// Deterministic SplitMix64 — one stream per purpose, so regimes
/// that share a purpose share its draws exactly.
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
    fn name(self) -> &'static str {
        match self {
            Regime::Mute => "mute",
            Regime::Deaf => "deaf",
            Regime::Coupled => "coupled",
        }
    }
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

/// The corpse's flocking parameters, verbatim.
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

/// Toroidal axis distance on a wrapped world of the given extent.
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

#[derive(Debug, Clone)]
struct Ledger {
    regime: Regime,
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
}

fn run(regime: Regime, ticks: usize) -> Ledger {
    // Spawn: the corpse released all 50 boids from the centre with
    // random unit velocities. The spawn stream is shared by all
    // regimes, so the three flocks start boid-for-boid identical.
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

    for _tick in 0..ticks {
        grid.step();
        // Track the loudest cell the field ever reaches.
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
            let b = &mut boids[i];
            b.accel += force;
            // The corpse's update: accelerate, cap, move, wrap.
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

            // The return path: a loud cell shakes the boid in it.
            // The force lands in the acceleration, so — exactly as
            // in the corpse — it is spent on the *next* tick.
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

            // The forward path: a boid now and then strikes the field.
            if regime.plucks() && pluck_rng.next_f64() < PLUCK_PROB {
                let gx = b.pos.x as usize;
                let gy = b.pos.y as usize;
                if gx < WIDTH && gy < HEIGHT {
                    grid.pluck(gx, gy, PLUCK_STRENGTH);
                    plucks += 1;
                }
            }
        }
        if jittered_this_tick {
            jitter_ticks += 1;
        }
    }

    let mut field_final_abs_sum = 0.0f64;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            field_final_abs_sum += grid.get(x, y).abs() as f64;
        }
    }

    Ledger {
        regime,
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
    }
}

fn print_ledger(l: &Ledger) {
    println!(
        "regime {}: plucks {}, jitter events {}, jitter ticks {}, field max |p| {:.4}, field final sum |p| {:.2}",
        l.regime.name(),
        l.plucks,
        l.jitter_events,
        l.jitter_ticks,
        l.field_max_abs,
        l.field_final_abs_sum
    );
    println!(
        "regime {}: polarization {:.4} -> {:.4}, mean pairwise toroidal distance {:.4} -> {:.4}",
        l.regime.name(),
        l.pol_initial,
        l.pol_final,
        l.pair_initial,
        l.pair_final
    );
}

fn headless() {
    println!("sono-flock headless ledger");
    println!(
        "world {}x{}, {} boids, {} ticks, pluck coin {}, pluck strength {}, wave threshold {}",
        WIDTH, HEIGHT, BOIDS, TICKS, PLUCK_PROB, PLUCK_STRENGTH, WAVE_THRESHOLD
    );
    let mute = run(Regime::Mute, TICKS);
    let deaf = run(Regime::Deaf, TICKS);
    let coupled = run(Regime::Coupled, TICKS);
    print_ledger(&mute);
    print_ledger(&deaf);
    print_ledger(&coupled);
    let deaf_matches_mute = deaf.final_positions == mute.final_positions;
    let coupled_matches_deaf = coupled.final_positions == deaf.final_positions;
    println!("deaf final positions == mute final positions: {}", deaf_matches_mute);
    println!("coupled final positions == deaf final positions: {}", coupled_matches_deaf);
    println!(
        "polarization gain (mute): {:+.4}",
        mute.pol_final - mute.pol_initial
    );
}

fn main() -> Result<()> {
    // --headless is handled BEFORE anything else; this specimen has
    // no TUI at all, so the bypass the corpse never got is the front
    // door here.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("sono-flock: run with --headless for the ledger (no TUI in this revival; the corpse's window was its only mouth, and that was the problem).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_boid_feels_no_force() {
        let params = corpse_params();
        let pos = vec![Vec2::new(40.0, 20.0)];
        let vel = vec![Vec2::new(1.0, 0.0)];
        let f = compute_force(&pos, &vel, 0, &params);
        assert_eq!(f, Vec2::zero());
    }

    #[test]
    fn mute_field_is_exactly_silent() {
        let l = run(Regime::Mute, TICKS);
        assert_eq!(l.plucks, 0);
        assert_eq!(l.field_max_abs, 0.0);
        assert_eq!(l.field_final_abs_sum, 0.0);
        assert_eq!(l.jitter_events, 0);
    }

    #[test]
    fn deaf_flock_is_mute_flock_boid_for_boid() {
        // The forward path alone cannot move a single boid: with
        // separate streams per purpose, a flock that plucks but
        // never listens must land exactly where the silent flock
        // lands, while its field rings above the corpse's threshold.
        let mute = run(Regime::Mute, TICKS);
        let deaf = run(Regime::Deaf, TICKS);
        assert_eq!(deaf.final_positions, mute.final_positions);
        assert!(deaf.plucks > 0, "deaf flock must actually pluck");
        assert!(
            deaf.field_max_abs > WAVE_THRESHOLD,
            "deaf field must cross the jitter threshold somewhere: {}",
            deaf.field_max_abs
        );
    }

    #[test]
    fn coupled_loop_actually_bites() {
        // The corpse's claim, priced: the wave must reach the boids
        // (jitter events > 0) and the flock that hears it must not
        // land where the deaf flock landed.
        let deaf = run(Regime::Deaf, TICKS);
        let coupled = run(Regime::Coupled, TICKS);
        assert!(coupled.jitter_events > 0, "the wave never reached a boid");
        assert_ne!(coupled.final_positions, deaf.final_positions);
        assert_eq!(coupled.plucks, deaf.plucks, "same pluck stream, same count");
    }

    #[test]
    fn flock_aligns() {
        let l = run(Regime::Mute, TICKS);
        assert!(
            l.pol_final > l.pol_initial + 0.1,
            "polarization {} -> {}",
            l.pol_initial,
            l.pol_final
        );
    }

    #[test]
    fn run_is_deterministic() {
        let a = run(Regime::Coupled, 200);
        let b = run(Regime::Coupled, 200);
        assert_eq!(a.final_positions, b.final_positions);
        assert_eq!(a.jitter_events, b.jitter_events);
        assert_eq!(a.plucks, b.plucks);
    }
}
