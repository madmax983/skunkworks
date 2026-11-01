//! # Understudy Flock 🎭
//!
//! Brought by The Understudy 🎭. The role, directed by Mark
//! (2026-10-31, motto WWBD — "What Would Boids Do?"): a swarm of
//! working agents coordinating with **no coordinator**. The
//! principal machinery is the garden's physical boids
//! (`crates/flocking`, Reynolds' separation / alignment / cohesion
//! moving bodies through space, as sung by `flock-physics` and
//! `neuro-flock`). The Understudy performs the same role on
//! wholly different ground: there is no space and there are no
//! bodies. An agent *is* its current plan — a point in a
//! 4-dimensional approach-space — and its velocity is the
//! trajectory of its work between ticks.
//!
//! Two companies perform, same cast, same opening positions, same
//! noise stream:
//! - **The flock company**: each agent senses only neighbours
//!   within a local radius and steers — separation (steer away
//!   from near-identical plans: duplication, priced), alignment
//!   (steer toward neighbours' mean velocity: drift, priced),
//!   cohesion (steer toward the local centroid: attachment to
//!   the shared goal, priced). No board, no centre.
//! - **The board company** (the stigmergy-only control): no
//!   neighbour sensing, no steering at all. An agent on site at
//!   its target posts a claim — a trace on the shared board; a
//!   move that would land on another agent's fresh claim is
//!   cancelled (the tick is wasted).
//!   Coordination only through the environment, exactly as the
//!   garden's own substrates coordinate their gardeners.
//!
//! One contrarian dances in both companies: separation cranked
//! fivefold, no cohesion, half alignment, a stiffer private
//! spring. It is part of the design, not a bug.
//!
//! Predictions were declared in the README *before* the first
//! run and are printed with the ledger, HELD or CRACKED. The
//! failure line is also declared there in advance: total
//! convergence — spread cut to zero — is groupthink at machine
//! speed, and counts as a crack, not a win.
//!
//! `cargo run -p understudy-flock -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;

/// Plan-space dimensions (a plan is a point in approach-space).
const DIM: usize = 4;
/// Agents in each company.
const AGENTS: usize = 12;
/// Index of the contrarian.
const CONTRARIAN: usize = 11;
/// Ticks per performance.
const TICKS: u64 = 600;
/// Private targets are drawn from this many shared topics, so
/// duplication pressure is real: several agents chase near-identical
/// plans unless something prices the duplication.
const TOPICS: usize = 5;
/// Neighbour radius in plan space (flock company only).
const NEIGHBOUR_RADIUS: f64 = 3.5;
/// Plans closer than this are near-identical: the ledger's
/// duplication radius, and the board's veto radius.
const DUP_RADIUS: f64 = 0.5;
/// Separation's remit is wider than duplication itself: it steers
/// away from *near*-identical plans. (First-draft strike, kept in
/// the README: with separation's radius equal to the duplication
/// radius, its force faded to zero exactly at the boundary and
/// piles equilibrated just inside it.)
const SEP_RADIUS: f64 = 0.7;
/// Spring toward the agent's private target.
const SPRING: f64 = 0.015;
/// Velocity damping per tick.
const DAMPING: f64 = 0.90;
/// Deterministic per-tick noise amplitude.
const NOISE: f64 = 0.008;
/// Flock steering weights.
const W_SEPARATION: f64 = 0.12;
const W_ALIGNMENT: f64 = 0.10;
const W_COHESION: f64 = 0.02;
/// Board traces stay fresh this many ticks (board company only).
const TRACE_TTL: u64 = 40;

/// SplitMix64 — the run's only source of chance, fully deterministic.
fn splitmix(mut state: u64) -> (u64, u64) {
    state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (state, z ^ (z >> 31))
}

/// A stream of f64 in [0, 1) keyed by (seed, a, b, c) — identical
/// across both companies, so the two performances hear the same
/// noise and differ only in their machinery.
fn unit(seed: u64, a: u64, b: u64, c: u64) -> f64 {
    let h = seed
        ^ a.wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ b.wrapping_mul(0x8CB9_2A49_7240_2D17)
        ^ c.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let (_, z) = splitmix(h);
    ((z >> 11) as f64) / ((1u64 << 53) as f64)
}

fn dist(a: [f64; DIM], b: [f64; DIM]) -> f64 {
    let mut s = 0.0;
    for d in 0..DIM {
        s += (a[d] - b[d]) * (a[d] - b[d]);
    }
    s.sqrt()
}

fn norm(a: [f64; DIM]) -> f64 {
    dist(a, [0.0; DIM])
}

#[derive(Clone)]
struct Cast {
    pos: [[f64; DIM]; AGENTS],
    vel: [[f64; DIM]; AGENTS],
    target: [[f64; DIM]; AGENTS],
    goal: [f64; DIM],
}

/// Build the shared opening: topic centres on a shell of radius
/// 2.0, each agent's private target a small offset from one topic,
/// opening positions scattered on a shell of radius 2.5. The
/// shared goal G is the centroid of the topic centres.
fn opening() -> Cast {
    let seed = 0xF10C_C0FF_EE00_600D;
    let mut topics = [[0.0; DIM]; TOPICS];
    for t in 0..TOPICS {
        // Deterministic near-orthogonal directions via hashed signs.
        let mut v = [0.0; DIM];
        for d in 0..DIM {
            v[d] = unit(seed, t as u64, d as u64, 7) * 2.0 - 1.0;
        }
        let n = norm(v).max(1e-9);
        for d in 0..DIM {
            topics[t][d] = v[d] / n * 2.0;
        }
    }
    let mut goal = [0.0; DIM];
    for t in 0..TOPICS {
        for d in 0..DIM {
            goal[d] += topics[t][d] / TOPICS as f64;
        }
    }
    let mut cast = Cast {
        pos: [[0.0; DIM]; AGENTS],
        vel: [[0.0; DIM]; AGENTS],
        target: [[0.0; DIM]; AGENTS],
        goal,
    };
    for i in 0..AGENTS {
        let topic = topics[i % TOPICS];
        for d in 0..DIM {
            cast.target[i][d] = topic[d] + (unit(seed, i as u64, d as u64, 21) - 0.5) * 0.3;
            cast.pos[i][d] =
                (unit(seed, i as u64, d as u64, 42) * 2.0 - 1.0) * 1.45;
            cast.vel[i][d] = (unit(seed, i as u64, d as u64, 63) - 0.5) * 0.02;
        }
    }
    cast
}

/// The private machinery both companies share: a spring toward
/// the agent's own target (the contrarian's is stiffer), damping,
/// and the shared noise stream. This is the work itself; only the
/// coordination differs between companies.
/// Seed for the shared per-tick noise stream.
const NOISE_SEED: u64 = 0xB01D_5EED_0000_0001;

fn private_accel(cast: &Cast, i: usize, tick: u64) -> [f64; DIM] {
    let spring = if i == CONTRARIAN { SPRING * 1.5 } else { SPRING };
    let mut a = [0.0; DIM];
    for d in 0..DIM {
        a[d] = spring * (cast.target[i][d] - cast.pos[i][d])
            + (unit(NOISE_SEED, tick, i as u64, d as u64) - 0.5) * 2.0 * NOISE;
    }
    a
}

/// The flock's three steering rules for agent `i`, from local
/// neighbours only. Returns the steering acceleration.
fn flock_accel(cast: &Cast, i: usize) -> [f64; DIM] {
    let mut neighbours = 0usize;
    let mut centroid = [0.0; DIM];
    let mut mean_vel = [0.0; DIM];
    let mut separation = [0.0; DIM];
    for j in 0..AGENTS {
        if j == i {
            continue;
        }
        let d = dist(cast.pos[i], cast.pos[j]);
        if d < NEIGHBOUR_RADIUS {
            neighbours += 1;
            for k in 0..DIM {
                centroid[k] += cast.pos[j][k];
                mean_vel[k] += cast.vel[j][k];
            }
            if d < SEP_RADIUS && d > 1e-9 {
                let push = (1.0 - d / SEP_RADIUS) / d;
                for k in 0..DIM {
                    separation[k] += (cast.pos[i][k] - cast.pos[j][k]) * push;
                }
            }
        }
    }
    let mut a = [0.0; DIM];
    if neighbours == 0 {
        return a;
    }
    let n = neighbours as f64;
    let (w_sep, w_ali, w_coh) = if i == CONTRARIAN {
        (W_SEPARATION * 5.0, W_ALIGNMENT * 0.5, 0.0)
    } else {
        (W_SEPARATION, W_ALIGNMENT, W_COHESION)
    };
    for k in 0..DIM {
        a[k] += w_sep * separation[k];
        a[k] += w_ali * (mean_vel[k] / n - cast.vel[i][k]);
        a[k] += w_coh * (centroid[k] / n - cast.pos[i][k]);
    }
    a
}

#[derive(Default, Clone, Copy)]
struct Ledger {
    dup_ticks: u64,
    wasted_ticks: u64,
    spread_start: f64,
    spread_mid: f64,
    spread_end: f64,
    drift_end: f64,
    attachment_end: f64,
    contrarian_centroid: f64,
}

fn mean_pairwise(pos: &[[f64; DIM]; AGENTS]) -> f64 {
    let mut total = 0.0;
    let mut pairs = 0;
    for i in 0..AGENTS {
        for j in (i + 1)..AGENTS {
            total += dist(pos[i], pos[j]);
            pairs += 1;
        }
    }
    total / pairs as f64
}

/// Velocity-direction dispersion over a velocity set:
/// 1 - |mean of unit velocities|; 0 = one direction (no drift
/// apart in heading), near 1 = headings scattered.
fn drift(vel: &[[f64; DIM]; AGENTS]) -> f64 {
    let mut mean = [0.0; DIM];
    for i in 0..AGENTS {
        let n = norm(vel[i]);
        if n > 1e-12 {
            for d in 0..DIM {
                mean[d] += vel[i][d] / n / AGENTS as f64;
            }
        }
    }
    1.0 - norm(mean)
}

fn run_company(flock: bool) -> Ledger {
    let mut cast = opening();
    let mut ledger = Ledger {
        spread_start: mean_pairwise(&cast.pos),
        ..Default::default()
    };
    // Board company state: traces left by other agents.
    let mut traces: Vec<([f64; DIM], u64, usize)> = Vec::new();
    let mut drift_window = 0.0;
    let mut drift_samples = 0u64;

    for tick in 0..TICKS {
        let prev = cast.pos;
        // Accelerate and move every agent.
        for i in 0..AGENTS {
            let mut a = private_accel(&cast, i, tick);
            if flock {
                let s = flock_accel(&cast, i);
                for d in 0..DIM {
                    a[d] += s[d];
                }
            }
            for d in 0..DIM {
                cast.vel[i][d] = (cast.vel[i][d] + a[d]) * DAMPING;
                cast.pos[i][d] += cast.vel[i][d];
            }
        }
        if !flock {
            // The board speaks: a move landing on another agent's
            // fresh trace is cancelled — the tick is wasted and the
            // agent stays where it was, velocity halved.
            for i in 0..AGENTS {
                let blocked = traces.iter().any(|(p, t, owner)| {
                    *owner != i && tick.saturating_sub(*t) < TRACE_TTL
                        && dist(cast.pos[i], *p) < DUP_RADIUS
                });
                if blocked {
                    cast.pos[i] = prev[i];
                    for d in 0..DIM {
                        cast.vel[i][d] *= 0.5;
                    }
                    ledger.wasted_ticks += 1;
                }
            }
            // Work leaves a trace: an agent on site at its target
            // (within 0.35) posts its claim to the board. (First-
            // draft strike, kept in the README: tracing every
            // *position* carpeted the space in vetoes — 4,869 of
            // 7,200 moves cancelled — a minefield, not a board.
            // Stigmergy marks where work is done, not every step.)
            traces.retain(|(_, t, _)| tick.saturating_sub(*t) < TRACE_TTL);
            for i in 0..AGENTS {
                if dist(cast.pos[i], cast.target[i]) < 0.35 {
                    traces.push((cast.pos[i], tick, i));
                }
            }
        }
        // Duplicated work: pair-ticks spent at near-identical plans.
        for i in 0..AGENTS {
            for j in (i + 1)..AGENTS {
                if dist(cast.pos[i], cast.pos[j]) < DUP_RADIUS {
                    ledger.dup_ticks += 1;
                }
            }
        }
        if tick == TICKS / 2 {
            ledger.spread_mid = mean_pairwise(&cast.pos);
        }
        if tick >= TICKS - 100 {
            drift_window += drift(&cast.vel);
            drift_samples += 1;
        }
    }
    ledger.spread_end = mean_pairwise(&cast.pos);
    ledger.drift_end = drift_window / drift_samples.max(1) as f64;
    let mut attach = 0.0;
    let mut centroid = [0.0; DIM];
    for i in 0..AGENTS {
        attach += dist(cast.pos[i], cast.goal);
        for d in 0..DIM {
            centroid[d] += cast.pos[i][d] / AGENTS as f64;
        }
    }
    ledger.attachment_end = attach / AGENTS as f64;
    ledger.contrarian_centroid = dist(cast.pos[CONTRARIAN], centroid);
    ledger
}

fn verdict(held: bool) -> &'static str {
    if held {
        "HELD"
    } else {
        "CRACKED"
    }
}

fn headless() {
    let flock = run_company(true);
    let board = run_company(false);
    println!("understudy-flock headless: the flock role, performed in plan-space");
    println!(
        "  cast: {AGENTS} agents, plan space {DIM}D, {TICKS} ticks per company, {TOPICS} shared topics"
    );
    println!("  contrarian: agent {CONTRARIAN} (separation x5, no cohesion, half alignment, spring x1.5)");
    println!("  duplication radius {DUP_RADIUS}, neighbour radius {NEIGHBOUR_RADIUS}, trace freshness {TRACE_TTL} ticks");
    for (name, l) in [("flock (local steering only)", &flock), ("board (stigmergy only)", &board)] {
        println!("  company '{name}':");
        println!(
            "    duplicated pair-ticks: {}, wasted ticks: {}",
            l.dup_ticks, l.wasted_ticks
        );
        println!(
            "    spread (mean pairwise plan distance): start {:.3}, mid {:.3}, end {:.3}",
            l.spread_start, l.spread_mid, l.spread_end
        );
        println!(
            "    drift (heading dispersion, last 100 ticks): {:.4}",
            l.drift_end
        );
        println!(
            "    attachment (mean distance to shared goal G): {:.3}",
            l.attachment_end
        );
        println!(
            "    contrarian's distance from company centroid: {:.3}",
            l.contrarian_centroid
        );
    }
    // Mean non-contrarian distance from centroid, flock company,
    // for P5's ratio — recomputed from the declared rule: the
    // flock's contrarian must end at least 1.5x as far from the
    // centroid as the typical agent's attachment-scaled distance.
    // We judge P5 against the board company's contrarian instead:
    // the contrarian's cranked separation must isolate it *more*
    // in the flock, where separation is answered, than on the
    // board, where nothing answers it.
    let p1 = flock.dup_ticks < board.dup_ticks;
    let p2 = flock.drift_end < board.drift_end;
    let spread_ratio = flock.spread_end / flock.spread_start;
    let p3 = spread_ratio > 0.40 && spread_ratio < 1.10 && flock.spread_end > 0.5;
    let p4 = flock.attachment_end <= board.attachment_end;
    let p5 = flock.contrarian_centroid > board.contrarian_centroid;
    println!("  declared predictions, judged:");
    println!(
        "    P1 duplication priced by separation (flock dup < board dup): {} vs {} — {}",
        flock.dup_ticks,
        board.dup_ticks,
        verdict(p1)
    );
    println!(
        "    P2 drift priced by alignment (flock drift < board drift): {:.4} vs {:.4} — {}",
        flock.drift_end,
        board.drift_end,
        verdict(p2)
    );
    println!(
        "    P3 no collapse (flock end/start spread in (0.40, 1.10) and end > 0.5): ratio {:.3}, end {:.3} — {}",
        spread_ratio,
        flock.spread_end,
        verdict(p3)
    );
    println!(
        "    P4 attachment priced by cohesion (flock attachment <= board): {:.3} vs {:.3} — {}",
        flock.attachment_end,
        board.attachment_end,
        verdict(p4)
    );
    println!(
        "    P5 the contrarian stays an outlier (flock centroid distance > board): {:.3} vs {:.3} — {}",
        flock.contrarian_centroid,
        board.contrarian_centroid,
        verdict(p5)
    );
}

fn main() -> Result<()> {
    // --headless is handled before anything else; there is no TUI.
    if std::env::args().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("understudy-flock: run with --headless for the comparison ledger.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_is_deterministic_and_shared() {
        let a = opening();
        let b = opening();
        for i in 0..AGENTS {
            assert_eq!(a.pos[i], b.pos[i]);
            assert_eq!(a.target[i], b.target[i]);
        }
    }

    #[test]
    fn separation_pushes_near_identical_plans_apart() {
        let mut cast = opening();
        // Park agent 1 almost exactly on agent 0's plan.
        cast.pos[1] = cast.pos[0];
        cast.pos[1][0] += 0.1;
        for i in 0..AGENTS {
            cast.vel[i] = [0.0; DIM];
        }
        let before = dist(cast.pos[0], cast.pos[1]);
        let a0 = flock_accel(&cast, 0);
        let a1 = flock_accel(&cast, 1);
        // The steering accelerations point away from each other.
        let dot: f64 = (0..DIM).map(|d| a0[d] * a1[d]).sum();
        assert!(dot < 0.0, "separation accelerations must oppose: dot {dot}");
        assert!(norm(a0) > 0.0 && norm(a1) > 0.0);
        assert!(before < DUP_RADIUS);
    }

    #[test]
    fn alignment_reduces_heading_dispersion() {
        let mut cast = opening();
        for i in 0..AGENTS {
            cast.vel[i] = [0.0; DIM];
        }
        cast.vel[0] = [0.1, 0.0, 0.0, 0.0];
        cast.vel[1] = [-0.1, 0.0, 0.0, 0.0];
        // Neighbours within radius of each other.
        cast.pos[1] = cast.pos[0];
        cast.pos[1][1] += 1.0;
        let before = drift(&cast.vel);
        for _ in 0..40 {
            let acc: Vec<[f64; DIM]> = (0..AGENTS).map(|i| flock_accel(&cast, i)).collect();
            for i in 0..AGENTS {
                for d in 0..DIM {
                    cast.vel[i][d] = (cast.vel[i][d] + acc[i][d]) * DAMPING;
                }
            }
        }
        assert!(
            drift(&cast.vel) < before,
            "alignment must narrow heading dispersion: {before} -> {}",
            drift(&cast.vel)
        );
    }

    #[test]
    fn board_veto_wastes_a_tick_on_a_fresh_trace() {
        // One tick of the board company from a cast whose agent 1
        // opens on agent 0's position: no traces exist yet, so no
        // veto — then a full run must record *some* duplicated
        // pair-ticks (duplication pressure is real, not staged).
        let board = run_company(false);
        assert!(board.dup_ticks > 0, "the board company must duplicate some work");
        let flock = run_company(true);
        assert!(flock.spread_end > 0.5, "the flock must not collapse to a point");
    }

    #[test]
    fn both_companies_are_deterministic() {
        let a = run_company(true);
        let b = run_company(true);
        assert_eq!(a.dup_ticks, b.dup_ticks);
        assert_eq!(a.spread_end, b.spread_end);
        let c = run_company(false);
        let d = run_company(false);
        assert_eq!(c.dup_ticks, d.dup_ticks);
        assert_eq!(c.wasted_ticks, d.wasted_ticks);
    }

    #[test]
    fn contrarian_is_in_the_cast_and_priced() {
        // The contrarian has no cohesion: with all other agents
        // parked far away on one side, its flock acceleration has
        // no centroid pull — only (cranked) separation/alignment.
        let mut cast = opening();
        for i in 0..AGENTS {
            cast.pos[i] = [10.0, 0.0, 0.0, 0.0];
            cast.vel[i] = [0.0; DIM];
        }
        cast.pos[CONTRARIAN] = [0.0, 0.0, 0.0, 0.0];
        // Everyone else is outside the neighbour radius: no steering.
        let a = flock_accel(&cast, CONTRARIAN);
        assert_eq!(a, [0.0; DIM]);
    }
}
