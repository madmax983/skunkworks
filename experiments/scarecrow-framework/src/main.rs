//! # Scarecrow Framework 🌾
//!
//! Brought by The Scarecrow 🌾. The principle is Autumn's:
//! a framework provides the pieces, so builders do not hand-roll
//! what already exists. The host is `experiments/understudy-flock`
//! — the garden's flock role performed in plan-space, where an
//! agent is a point in a 4-dimensional approach-space and its
//! steering (separation / alignment / cohesion) was hand-built
//! there precisely because the role is not bodies in space.
//!
//! The garden already provides the piece: `crates/flocking`
//! (`compute_force`, Reynolds' three rules) — but it speaks only
//! `locus::Vec2`, two dimensions, f64. Plan-space has four.
//! (`locus` ships a `Vec4`, but it is f32 and `flocking` has no
//! variant that takes it.)
//!
//! Three companies perform, same cast, same opening, same noise:
//! - **Hand company**: the host's hand-built 4-D steering, taken
//!   up unchanged. It must reproduce the host's published ledger
//!   exactly, or the host was not faithfully taken up.
//! - **Framework company**: steering ONLY from
//!   `flocking::compute_force`, called on the Vec2 projection
//!   (plan dims 0 and 1). The force is applied to dims 0 and 1
//!   only; dims 2 and 3 receive no steering at all, because the
//!   provided piece cannot see them. No hand steering is added.
//! - **Board company**: the host's stigmergy-only control,
//!   unchanged — no sensing, no steering.
//!
//! The framework company's ledger is counted in full 4-D (the
//! host's instrument), and also in the 2-D projection the piece
//! can see, so the ledger can say where the piece worked and
//! where it was blind.
//!
//! `cargo run -p scarecrow-framework -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;

/// Plan-space dimensions (from the host, unchanged).
const DIM: usize = 4;
const AGENTS: usize = 12;
const CONTRARIAN: usize = 11;
const TICKS: u64 = 600;
const TOPICS: usize = 5;
const NEIGHBOUR_RADIUS: f64 = 3.5;
const DUP_RADIUS: f64 = 0.5;
const SEP_RADIUS: f64 = 0.7;
const SPRING: f64 = 0.015;
const DAMPING: f64 = 0.90;
const NOISE: f64 = 0.008;
const W_SEPARATION: f64 = 0.12;
const W_ALIGNMENT: f64 = 0.10;
const W_COHESION: f64 = 0.02;
const TRACE_TTL: u64 = 40;

/// Framework translation, declared: the host's weights multiply
/// raw neighbour vectors; the provided piece's weights multiply
/// steering capped at FW_MAX_FORCE. The ratio is the host's own
/// (6 : 5 : 1), scaled by ten so the capped forces land in the
/// host's acceleration range. Max speed is a typical host agent
/// speed; max force is the per-rule steering cap.
const FW_MAX_SPEED: f64 = 0.05;
const FW_MAX_FORCE: f64 = 0.005;
const FW_W_SEPARATION: f64 = 1.2;
const FW_W_ALIGNMENT: f64 = 1.0;
const FW_W_COHESION: f64 = 0.2;

fn splitmix(mut state: u64) -> (u64, u64) {
    state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (state, z ^ (z >> 31))
}

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

fn dist2d(a: [f64; DIM], b: [f64; DIM]) -> f64 {
    ((a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])).sqrt()
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

fn opening() -> Cast {
    let seed = 0xF10C_C0FF_EE00_600D;
    let mut topics = [[0.0; DIM]; TOPICS];
    for t in 0..TOPICS {
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
            cast.pos[i][d] = (unit(seed, i as u64, d as u64, 42) * 2.0 - 1.0) * 1.45;
            cast.vel[i][d] = (unit(seed, i as u64, d as u64, 63) - 0.5) * 0.02;
        }
    }
    cast
}

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

/// The host's hand-built steering, verbatim in substance.
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

/// The provided piece, and only the provided piece. The plans
/// are projected to Vec2 (dims 0 and 1); the returned force is
/// applied to those dims alone. The contrarian's cranked
/// separation / halved alignment / no cohesion ARE expressible
/// in FlockingParams, per agent — that part of the role the
/// piece can carry.
fn framework_accel(cast: &Cast, i: usize) -> [f64; DIM] {
    let positions: Vec<Vec2> = cast
        .pos
        .iter()
        .map(|p| Vec2::new(p[0], p[1]))
        .collect();
    let velocities: Vec<Vec2> = cast
        .vel
        .iter()
        .map(|v| Vec2::new(v[0], v[1]))
        .collect();
    let (w_sep, w_ali, w_coh) = if i == CONTRARIAN {
        (FW_W_SEPARATION * 5.0, FW_W_ALIGNMENT * 0.5, 0.0)
    } else {
        (FW_W_SEPARATION, FW_W_ALIGNMENT, FW_W_COHESION)
    };
    let params = FlockingParams {
        view_radius: NEIGHBOUR_RADIUS,
        separation_radius: SEP_RADIUS,
        max_speed: FW_MAX_SPEED,
        max_force: FW_MAX_FORCE,
        separation_weight: w_sep,
        alignment_weight: w_ali,
        cohesion_weight: w_coh,
    };
    let f = compute_force(&positions, &velocities, i, &params);
    let mut a = [0.0; DIM];
    a[0] = f.x;
    a[1] = f.y;
    a
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Hand,
    Framework,
    Board,
}

#[derive(Default, Clone, Copy)]
struct Ledger {
    dup_ticks: u64,
    dup_ticks_2d: u64,
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

fn run_company(mode: Mode) -> Ledger {
    let mut cast = opening();
    let mut ledger = Ledger {
        spread_start: mean_pairwise(&cast.pos),
        ..Default::default()
    };
    let mut traces: Vec<([f64; DIM], u64, usize)> = Vec::new();
    let mut drift_window = 0.0;
    let mut drift_samples = 0u64;

    for tick in 0..TICKS {
        let prev = cast.pos;
        for i in 0..AGENTS {
            let mut a = private_accel(&cast, i, tick);
            let s = match mode {
                Mode::Hand => flock_accel(&cast, i),
                Mode::Framework => framework_accel(&cast, i),
                Mode::Board => [0.0; DIM],
            };
            for d in 0..DIM {
                a[d] += s[d];
            }
            for d in 0..DIM {
                cast.vel[i][d] = (cast.vel[i][d] + a[d]) * DAMPING;
                cast.pos[i][d] += cast.vel[i][d];
            }
        }
        if mode == Mode::Board {
            for i in 0..AGENTS {
                let blocked = traces.iter().any(|(p, t, owner)| {
                    *owner != i
                        && tick.saturating_sub(*t) < TRACE_TTL
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
            traces.retain(|(_, t, _)| tick.saturating_sub(*t) < TRACE_TTL);
            for i in 0..AGENTS {
                if dist(cast.pos[i], cast.target[i]) < 0.35 {
                    traces.push((cast.pos[i], tick, i));
                }
            }
        }
        for i in 0..AGENTS {
            for j in (i + 1)..AGENTS {
                if dist(cast.pos[i], cast.pos[j]) < DUP_RADIUS {
                    ledger.dup_ticks += 1;
                }
                if dist2d(cast.pos[i], cast.pos[j]) < DUP_RADIUS {
                    ledger.dup_ticks_2d += 1;
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
    if held { "HELD" } else { "CRACKED" }
}

fn headless() {
    let hand = run_company(Mode::Hand);
    let framework = run_company(Mode::Framework);
    let board = run_company(Mode::Board);
    println!("scarecrow-framework headless: the provided piece, planted in plan-space");
    println!("  host: understudy-flock — {AGENTS} agents, plan space {DIM}D, {TICKS} ticks per company, {TOPICS} shared topics");
    println!("  piece: crates/flocking compute_force on locus::Vec2 (dims 0-1 only); max_speed {FW_MAX_SPEED}, max_force {FW_MAX_FORCE}, weights {FW_W_SEPARATION}/{FW_W_ALIGNMENT}/{FW_W_COHESION}");
    println!("  host's published ledger: hand dup 6, board dup 3073, board wasted 4753, hand drift 0.5090, board drift 0.6869, hand spread end 1.247, hand attachment 0.896");
    for (name, l) in [
        ("hand (host steering, 4-D)", &hand),
        ("framework (provided piece, 2-D)", &framework),
        ("board (stigmergy only)", &board),
    ] {
        println!("  company '{name}':");
        println!(
            "    duplicated pair-ticks (4-D): {}, duplicated pair-ticks (2-D projection): {}, wasted ticks: {}",
            l.dup_ticks, l.dup_ticks_2d, l.wasted_ticks
        );
        println!(
            "    spread (mean pairwise plan distance): start {:.3}, mid {:.3}, end {:.3}",
            l.spread_start, l.spread_mid, l.spread_end
        );
        println!("    drift (heading dispersion, last 100 ticks): {:.4}", l.drift_end);
        println!("    attachment (mean distance to shared goal G): {:.3}", l.attachment_end);
        println!("    contrarian's distance from company centroid: {:.3}", l.contrarian_centroid);
    }
    let host_ok = hand.dup_ticks == 6
        && board.dup_ticks == 3073
        && board.wasted_ticks == 4753
        && (hand.drift_end - 0.5090).abs() < 0.001
        && (hand.spread_end - 1.247).abs() < 0.01;
    let p1 = framework.dup_ticks >= 5 * hand.dup_ticks.max(1) && framework.dup_ticks < board.dup_ticks;
    let p2 = framework.dup_ticks_2d < board.dup_ticks_2d;
    let p3 = framework.drift_end > hand.drift_end;
    let ratio = framework.spread_end / framework.spread_start;
    let p4 = ratio > 0.40 && ratio < 1.10 && framework.spread_end > 0.5;
    println!("  declared predictions, judged:");
    println!("    P0 host check (hand and board reproduce the host exactly): {}", verdict(host_ok));
    println!(
        "    P1 the piece is blind in half the field (framework 4-D dup >= 5x hand, and < board): {} vs {} vs {} — {}",
        framework.dup_ticks, hand.dup_ticks, board.dup_ticks, verdict(p1)
    );
    println!(
        "    P2 the piece works where it can see (framework 2-D dup < board 2-D dup): {} vs {} — {}",
        framework.dup_ticks_2d, board.dup_ticks_2d, verdict(p2)
    );
    println!(
        "    P3 drift is priced only in the seen dims (framework drift > hand): {:.4} vs {:.4} — {}",
        framework.drift_end, hand.drift_end, verdict(p3)
    );
    println!(
        "    P4 no collapse (framework end/start spread in (0.40, 1.10) and end > 0.5): ratio {:.3}, end {:.3} — {}",
        ratio, framework.spread_end, verdict(p4)
    );
    println!("  the missing piece, named: flocking::compute_force takes &[locus::Vec2] only — no Vec4 (f32, in locus) or generic-dimension variant — so dims 2 and 3 of a plan cannot be steered by the provided piece at all.");
}

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-framework: run with --headless for the comparison ledger.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_check_hand_and_board_reproduce_the_host() {
        let hand = run_company(Mode::Hand);
        let board = run_company(Mode::Board);
        assert_eq!(hand.dup_ticks, 6);
        assert_eq!(board.dup_ticks, 3073);
        assert_eq!(board.wasted_ticks, 4753);
        assert!((hand.drift_end - 0.5090).abs() < 0.001);
        assert!((hand.spread_end - 1.247).abs() < 0.01);
    }

    #[test]
    fn framework_force_is_zero_in_the_unseen_dims() {
        let cast = opening();
        for i in 0..AGENTS {
            let a = framework_accel(&cast, i);
            assert_eq!(a[2], 0.0);
            assert_eq!(a[3], 0.0);
        }
    }

    #[test]
    fn framework_company_is_deterministic() {
        let a = run_company(Mode::Framework);
        let b = run_company(Mode::Framework);
        assert_eq!(a.dup_ticks, b.dup_ticks);
        assert_eq!(a.dup_ticks_2d, b.dup_ticks_2d);
        assert_eq!(a.spread_end, b.spread_end);
    }

    #[test]
    fn framework_dup_exceeds_hand_dup() {
        let hand = run_company(Mode::Hand);
        let framework = run_company(Mode::Framework);
        assert!(
            framework.dup_ticks > hand.dup_ticks,
            "framework dup {} must exceed hand dup {} — the unseen dims pile up",
            framework.dup_ticks, hand.dup_ticks
        );
    }

    #[test]
    fn framework_prices_duplication_in_its_projection() {
        let framework = run_company(Mode::Framework);
        let board = run_company(Mode::Board);
        assert!(
            framework.dup_ticks_2d < board.dup_ticks_2d,
            "framework 2-D dup {} must be under board 2-D dup {} — the piece prices what it can see",
            framework.dup_ticks_2d, board.dup_ticks_2d
        );
    }

    #[test]
    fn framework_does_not_collapse() {
        let framework = run_company(Mode::Framework);
        assert!(framework.spread_end > 0.5);
        assert!(framework.spread_end / framework.spread_start > 0.40);
    }

    #[test]
    fn opening_is_deterministic_and_shared() {
        let a = opening();
        let b = opening();
        for i in 0..AGENTS {
            assert_eq!(a.pos[i], b.pos[i]);
            assert_eq!(a.target[i], b.target[i]);
        }
    }
}
