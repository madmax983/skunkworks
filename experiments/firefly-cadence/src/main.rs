//! firefly-cadence — a revival cross: graveyard/firefly-synapse ×
//! crates/git-associates.
//!
//! The corpse (65 + ~230 LOC, macroquad + rayon) staged 100,000
//! Kuramoto fireflies: each agent carries a phase turning at its own
//! natural frequency, and neighbours within a short radius pull each
//! other's phases together until the field pulses as one. It was
//! executed for Generic Implementation / Ecosystem Isolation — a
//! textbook Kuramoto sketch with no repository in it — and its
//! Reaper report named the salvation condition outright: *a
//! meaningful structural integration that leverages repository data
//! (e.g. git commits driving natural frequency)*. Two further flaws
//! the certificate did not name: its frequencies were drawn from a
//! narrow uniform band (0.10–0.12) that makes synchronisation a
//! foregone conclusion, and its swarm was never counted — no order
//! parameter, no headless bypass, nothing but FPS on a window.
//!
//! The living `git-associates` crate supplies the exact organ the
//! Reaper prescribed: this repository's own commit history — hashes
//! and timestamps — read headlessly, the same supply the living
//! `git-resonance` bell has rung from since the garden opened.
//!
//! The revival makes every commit a firefly, and the repository's
//! cadence its frequency:
//! - Natural frequency ω comes from the commit's gap — the hours
//!   until the next commit landed — log-mapped onto [0.06, 0.16]:
//!   a burst-era commit (minutes between landings) burns fast, a
//!   quiet-era commit (weeks) burns slow. The corpse's band was
//!   ±10% around its centre; the cadence band spans a ratio of 2.7.
//! - Position and opening phase come from the commit's hash, as the
//!   bell's notes do — the whole swarm is a pure function of the
//!   history: no RNG anywhere except the one deliberate shuffle.
//!
//! Five rooms run on that one swarm, so each choice is priced:
//! - corpse:   the corpse's own narrow frequencies, hash placement,
//!             local coupling (its linked-cell radius, rescaled to
//!             this field's density).
//! - cadence-hash:  cadence frequencies, hash placement, local.
//! - cadence-era:   cadence frequencies, era placement, local —
//!             x is the commit's rank in history, y is kept from the
//!             hash, so the two placements differ only in x: if era
//!             neighbours share an era's cadence, local synchrony
//!             should form there first.
//! - shuffled-era:  the cadence frequencies shuffled across the era
//!             map (seeded, fixed) — same distribution, same map,
//!             the assignment destroyed. The control that separates
//!             *what frequencies exist* from *where they live*.
//! - cadence-global: cadence frequencies under mean-field coupling
//!             (exact, via the order parameter) — placement cannot
//!             matter here, by construction.
//!
//! Each room is surveyed across coupling K ∈ {0.025, 0.05, 0.1,
//! 0.2, 0.4} for 1,500 ticks of the corpse's own step (K/N_local ·
//! Σ sin(θj − θi), dt = 0.1). The ledger prints, per room: the
//! final order parameter R at every K, the critical coupling Kc
//! (first K whose final R ≥ 0.8), and at K = 0.1 the phase-coherent
//! cluster census, the flash count, and — for the cadence rooms —
//! the two halves of the map read as provinces (each half's own R,
//! and the wrapped gap between their mean phases).
//!
//! Surveyor's question, stated plainly: the corpse proved a swarm
//! *can* synchronise when its frequencies are nearly identical.
//! Does a real repository's cadence — bursty, lull-ridden, written
//! by one gardener's seasons — synchronise at all, and if it does,
//! is the price set by the frequencies themselves, by where they
//! sit, or by the eras they were written in?

use anyhow::Result;
use git_associates::GitModel;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use std::f64::consts::TAU;

pub const FIELD: f64 = 100.0;
pub const RADIUS: f64 = 20.0;
pub const DT: f64 = 0.1;
pub const TICKS: u32 = 1500;
pub const K_GRID: [f64; 5] = [0.025, 0.05, 0.1, 0.2, 0.4];
pub const DETAIL_K: f64 = 0.1;
pub const OMEGA_MIN: f64 = 0.06;
pub const OMEGA_MAX: f64 = 0.16;
/// One minute, in hours: the log floor for gaps.
pub const GAP_EPS_H: f64 = 1.0 / 60.0;

/// One commit, reduced to what the swarm needs.
#[derive(Debug, Clone, PartialEq)]
pub struct CommitDatum {
    pub hash: String,
    /// Seconds since the epoch, newest first in a history vector.
    pub timestamp: i64,
}

fn hex_byte(hash: &str, pair: usize) -> u8 {
    let b = hash.as_bytes();
    let hi = (b.get(pair * 2).copied().unwrap_or(b'0') as char)
        .to_digit(16)
        .unwrap_or(0) as u8;
    let lo = (b.get(pair * 2 + 1).copied().unwrap_or(b'0') as char)
        .to_digit(16)
        .unwrap_or(0) as u8;
    hi * 16 + lo
}

/// Gaps between successive commits, in hours, newest first. The
/// oldest commit has no successor; it inherits the median gap.
pub fn gaps_hours(history: &[CommitDatum]) -> Vec<f64> {
    let n = history.len();
    if n == 0 {
        return Vec::new();
    }
    let mut gaps: Vec<f64> = (0..n - 1)
        .map(|i| ((history[i].timestamp - history[i + 1].timestamp) as f64 / 3600.0).max(0.0))
        .collect();
    let mut sorted = gaps.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.is_empty() {
        0.0
    } else if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    };
    gaps.push(median);
    gaps
}

/// Log-map a gap onto a natural frequency: short gap (a burst)
/// burns fast, long gap (a lull) burns slow.
pub fn omega_from_gap(gap_h: f64, ln_min: f64, ln_max: f64) -> f64 {
    if ln_max <= ln_min {
        return (OMEGA_MIN + OMEGA_MAX) / 2.0;
    }
    let s = (ln_max - (gap_h + GAP_EPS_H).ln()) / (ln_max - ln_min);
    OMEGA_MIN + (OMEGA_MAX - OMEGA_MIN) * s.clamp(0.0, 1.0)
}

pub fn cadence_omegas(history: &[CommitDatum]) -> Vec<f64> {
    let gaps = gaps_hours(history);
    let lns: Vec<f64> = gaps.iter().map(|g| (g + GAP_EPS_H).ln()).collect();
    let ln_min = lns.iter().cloned().fold(f64::INFINITY, f64::min);
    let ln_max = lns.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    gaps.iter().map(|g| omega_from_gap(*g, ln_min, ln_max)).collect()
}

/// The corpse's own frequency recipe: a narrow uniform band read
/// off the hash, standing in for its thread_rng draw U[0.10, 0.12].
pub fn corpse_omega(hash: &str) -> f64 {
    0.10 + (hex_byte(hash, 4) as f64 / 255.0) * 0.02
}

pub fn hash_position(hash: &str) -> (f64, f64) {
    (
        hex_byte(hash, 0) as f64 / 255.0 * FIELD,
        hex_byte(hash, 1) as f64 / 255.0 * FIELD,
    )
}

pub fn era_position(hash: &str, rank: usize, n: usize) -> (f64, f64) {
    let (_, y) = hash_position(hash);
    (((rank as f64 + 0.5) / n as f64) * FIELD, y)
}

pub fn opening_phase(hash: &str) -> f64 {
    let v = (hex_byte(hash, 2) as f64 * 256.0 + hex_byte(hash, 3) as f64) / 65535.0;
    v * TAU
}

/// One room's swarm: frequencies, positions, opening phases.
#[derive(Debug, Clone)]
pub struct Swarm {
    pub omega: Vec<f64>,
    pub pos: Vec<(f64, f64)>,
    pub phase0: Vec<f64>,
}

impl Swarm {
    pub fn len(&self) -> usize {
        self.omega.len()
    }
    pub fn is_empty(&self) -> bool {
        self.omega.is_empty()
    }
}

/// Toroidal neighbour lists within RADIUS, self excluded.
pub fn neighbours(pos: &[(f64, f64)]) -> Vec<Vec<usize>> {
    let n = pos.len();
    let mut out: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut dx = (pos[j].0 - pos[i].0).abs();
            if dx > FIELD / 2.0 {
                dx = FIELD - dx;
            }
            let mut dy = (pos[j].1 - pos[i].1).abs();
            if dy > FIELD / 2.0 {
                dy = FIELD - dy;
            }
            if dx * dx + dy * dy < RADIUS * RADIUS {
                out[i].push(j);
            }
        }
    }
    out
}

pub fn order_parameter(phases: &[f64]) -> (f64, f64) {
    let n = phases.len() as f64;
    let (s, c) = phases
        .iter()
        .fold((0.0, 0.0), |(s, c), p| (s + p.sin(), c + p.cos()));
    let (ms, mc) = (s / n, c / n);
    ((ms * ms + mc * mc).sqrt(), ms.atan2(mc))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ledger {
    pub r_final: f64,
    pub clusters_ge3: usize,
    pub largest_cluster: usize,
    pub in_big_clusters: usize,
    pub flashes: u64,
    /// (west R, east R, wrapped mean-phase gap) at the final tick,
    /// splitting the map at x = FIELD/2.
    pub west_r: f64,
    pub east_r: f64,
    pub province_gap: f64,
}

/// Phase-coherent clusters over a graph: an edge joins neighbours
/// whose final phases differ by less than 0.3 rad (wrapped).
pub fn cluster_census(phases: &[f64], adjacency: &[Vec<usize>]) -> (usize, usize, usize) {
    let n = phases.len();
    let mut seen = vec![false; n];
    let mut comps: Vec<usize> = Vec::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        let mut stack = vec![start];
        seen[start] = true;
        let mut size = 0usize;
        while let Some(u) = stack.pop() {
            size += 1;
            for &v in &adjacency[u] {
                if seen[v] {
                    continue;
                }
                let mut d = (phases[v] - phases[u]).rem_euclid(TAU);
                if d > TAU / 2.0 {
                    d = TAU - d;
                }
                if d < 0.3 {
                    seen[v] = true;
                    stack.push(v);
                }
            }
        }
        comps.push(size);
    }
    let ge3: Vec<usize> = comps.iter().cloned().filter(|c| *c >= 3).collect();
    (
        ge3.len(),
        comps.iter().max().copied().unwrap_or(0),
        ge3.iter().sum(),
    )
}

fn finish_ledger(swarm: &Swarm, adjacency: &[Vec<usize>], phases: &[f64], flashes: u64) -> Ledger {
    let (r_final, _) = order_parameter(phases);
    let (clusters_ge3, largest_cluster, in_big_clusters) = cluster_census(phases, adjacency);
    let west: Vec<f64> = swarm
        .pos
        .iter()
        .zip(phases)
        .filter(|(p, _)| p.0 < FIELD / 2.0)
        .map(|(_, ph)| *ph)
        .collect();
    let east: Vec<f64> = swarm
        .pos
        .iter()
        .zip(phases)
        .filter(|(p, _)| p.0 >= FIELD / 2.0)
        .map(|(_, ph)| *ph)
        .collect();
    let (west_r, west_m) = if west.is_empty() {
        (0.0, 0.0)
    } else {
        order_parameter(&west)
    };
    let (east_r, east_m) = if east.is_empty() {
        (0.0, 0.0)
    } else {
        order_parameter(&east)
    };
    let mut gap = (west_m - east_m).rem_euclid(TAU);
    if gap > TAU / 2.0 {
        gap = TAU - gap;
    }
    Ledger {
        r_final,
        clusters_ge3,
        largest_cluster,
        in_big_clusters,
        flashes,
        west_r,
        east_r,
        province_gap: gap,
    }
}

/// The corpse's update, verbatim in shape: dθ = ω + (K/|N_i|) ·
/// Σ sin(θj − θi), stepped at DT, phases kept on the circle while
/// an unwrapped twin counts the flashes.
pub fn run_local(swarm: &Swarm, k: f64, ticks: u32) -> Ledger {
    let adjacency = neighbours(&swarm.pos);
    let mut phases = swarm.phase0.clone();
    let mut unwrapped = swarm.phase0.clone();
    let mut flashes: u64 = 0;
    for _ in 0..ticks {
        let mut next = vec![0.0f64; swarm.len()];
        for i in 0..swarm.len() {
            let nbrs = &adjacency[i];
            let coup = if nbrs.is_empty() {
                0.0
            } else {
                let sum: f64 = nbrs.iter().map(|&j| (phases[j] - phases[i]).sin()).sum();
                k / nbrs.len() as f64 * sum
            };
            next[i] = phases[i] + (swarm.omega[i] + coup) * DT;
        }
        for i in 0..swarm.len() {
            let before = (unwrapped[i] / TAU).floor();
            unwrapped[i] += next[i] - phases[i];
            let after = (unwrapped[i] / TAU).floor();
            if after > before {
                flashes += (after - before) as u64;
            }
            phases[i] = next[i].rem_euclid(TAU);
        }
    }
    finish_ledger(swarm, &adjacency, &phases, flashes)
}

/// Mean-field coupling, exact: (1/N) Σ sin(θj − θi) = R·sin(Θ − θi)
/// where (R, Θ) is the order parameter. The cluster census runs on
/// the complete graph.
pub fn run_global(swarm: &Swarm, k: f64, ticks: u32) -> Ledger {
    let n = swarm.len();
    let adjacency: Vec<Vec<usize>> = (0..n)
        .map(|i| (0..n).filter(|&j| j != i).collect())
        .collect();
    let mut phases = swarm.phase0.clone();
    let mut unwrapped = swarm.phase0.clone();
    let mut flashes: u64 = 0;
    for _ in 0..ticks {
        let (r, mean) = order_parameter(&phases);
        let mut next = vec![0.0f64; n];
        for i in 0..n {
            let coup = k * r * (mean - phases[i]).sin();
            next[i] = phases[i] + (swarm.omega[i] + coup) * DT;
        }
        for i in 0..n {
            let before = (unwrapped[i] / TAU).floor();
            unwrapped[i] += next[i] - phases[i];
            let after = (unwrapped[i] / TAU).floor();
            if after > before {
                flashes += (after - before) as u64;
            }
            phases[i] = next[i].rem_euclid(TAU);
        }
    }
    finish_ledger(swarm, &adjacency, &phases, flashes)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    CorpseHashLocal,
    CadenceHashLocal,
    CadenceEraLocal,
    ShuffledEraLocal,
    CadenceGlobal,
}

impl Room {
    pub fn label(&self) -> &'static str {
        match self {
            Room::CorpseHashLocal => "corpse-hash-local",
            Room::CadenceHashLocal => "cadence-hash-local",
            Room::CadenceEraLocal => "cadence-era-local",
            Room::ShuffledEraLocal => "shuffled-era-local",
            Room::CadenceGlobal => "cadence-global",
        }
    }
    pub fn is_global(&self) -> bool {
        matches!(self, Room::CadenceGlobal)
    }
}

pub fn build_swarm(history: &[CommitDatum], room: Room) -> Swarm {
    let n = history.len();
    let cadence = cadence_omegas(history);
    let phase0: Vec<f64> = history.iter().map(|c| opening_phase(&c.hash)).collect();
    match room {
        Room::CorpseHashLocal => Swarm {
            omega: history.iter().map(|c| corpse_omega(&c.hash)).collect(),
            pos: history.iter().map(|c| hash_position(&c.hash)).collect(),
            phase0,
        },
        Room::CadenceHashLocal | Room::CadenceGlobal => Swarm {
            omega: cadence,
            pos: history.iter().map(|c| hash_position(&c.hash)).collect(),
            phase0,
        },
        Room::CadenceEraLocal => Swarm {
            omega: cadence,
            pos: history
                .iter()
                .enumerate()
                .map(|(i, c)| era_position(&c.hash, i, n))
                .collect(),
            phase0,
        },
        Room::ShuffledEraLocal => {
            let mut omega = cadence;
            let mut rng = StdRng::seed_from_u64(42);
            omega.shuffle(&mut rng);
            Swarm {
                omega,
                pos: history
                    .iter()
                    .enumerate()
                    .map(|(i, c)| era_position(&c.hash, i, n))
                    .collect(),
                phase0,
            }
        }
    }
}

pub fn run_room(history: &[CommitDatum], room: Room, k: f64, ticks: u32) -> Ledger {
    let swarm = build_swarm(history, room);
    if room.is_global() {
        run_global(&swarm, k, ticks)
    } else {
        run_local(&swarm, k, ticks)
    }
}

/// The first K on the grid whose final R reaches 0.8, if any.
pub fn critical_k(rs: &[f64]) -> Option<f64> {
    K_GRID
        .iter()
        .zip(rs)
        .find(|(_, r)| **r >= 0.8)
        .map(|(k, _)| *k)
}

/// A deterministic synthetic history, rung only when no repository
/// can be opened (the bell's vacuum rule): 200 commits whose gaps
/// alternate burst blocks and lulls, hashes mixed from the index.
pub fn synthetic_history() -> Vec<CommitDatum> {
    let mut out = Vec::new();
    let mut t: i64 = 1_760_000_000;
    for i in (0..200).rev() {
        let gap_h = match (i / 10) % 4 {
            0 => 0.05,
            1 => 0.4,
            2 => 6.0,
            _ => 60.0,
        };
        t -= (gap_h * 3600.0) as i64;
        let mix = (i as u64)
            .wrapping_mul(0x9E37_79B9_7F4A_7C15)
            .rotate_left(23);
        out.push(CommitDatum {
            hash: format!("{mix:040x}"),
            timestamp: t,
        });
    }
    out.reverse();
    out
}

pub fn load_history(path: &str, limit: usize) -> Option<Vec<CommitDatum>> {
    let model = GitModel::open(path).ok()?;
    let commits = model.history(limit).ok()?;
    if commits.len() < 10 {
        return None;
    }
    Some(
        commits
            .iter()
            .map(|c| CommitDatum {
                hash: c.hash.clone(),
                timestamp: c.timestamp.timestamp(),
            })
            .collect(),
    )
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if s.is_empty() {
        0.0
    } else if s.len() % 2 == 1 {
        s[s.len() / 2]
    } else {
        (s[s.len() / 2 - 1] + s[s.len() / 2]) / 2.0
    }
}

fn main() -> Result<()> {
    // --headless is handled before anything else; this specimen
    // has no TUI at all (its corpse's window was the thing that
    // killed it), so every route runs the ledger.
    let args: Vec<String> = std::env::args().collect();
    let _headless = args.iter().any(|a| a == "--headless");
    let positional: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with("--"))
        .collect();
    let path = positional.first().map(|s| s.as_str()).unwrap_or(".");
    let limit: usize = positional
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    let (history, source) = match load_history(path, limit) {
        Some(h) => (h, format!("repository at {path}")),
        None => (
            synthetic_history(),
            "synthetic demo history (no repository could be opened)".to_string(),
        ),
    };
    let n = history.len();
    let gaps = gaps_hours(&history);
    let omegas = cadence_omegas(&history);
    let half = n / 2;
    println!("firefly-cadence headless: the corpse's swarm, re-lit by this repository's cadence");
    println!("  source: {source}");
    println!(
        "  fireflies (commits): {n}, field {FIELD:.0}x{FIELD:.0}, radius {RADIUS:.0}, dt {DT}, ticks {TICKS}"
    );
    println!(
        "  gap hours: min {:.3}, median {:.3}, max {:.1}; median gap newest half {:.3} h, oldest half {:.3} h",
        gaps.iter().cloned().fold(f64::INFINITY, f64::min),
        median(&gaps),
        gaps.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        median(&gaps[..half]),
        median(&gaps[half..]),
    );
    println!(
        "  cadence omega: min {:.4}, mean {:.4}, max {:.4} (corpse band 0.1000-0.1200)",
        omegas.iter().cloned().fold(f64::INFINITY, f64::min),
        omegas.iter().sum::<f64>() / n as f64,
        omegas.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
    );

    let rooms = [
        Room::CorpseHashLocal,
        Room::CadenceHashLocal,
        Room::CadenceEraLocal,
        Room::ShuffledEraLocal,
        Room::CadenceGlobal,
    ];
    let mut kc: Vec<(Room, Option<f64>)> = Vec::new();
    let mut detail: Vec<(Room, Ledger)> = Vec::new();
    for room in rooms {
        let rs: Vec<f64> = K_GRID
            .iter()
            .map(|k| run_room(&history, room, *k, TICKS).r_final)
            .collect();
        let k_here = critical_k(&rs);
        kc.push((room, k_here));
        let led = run_room(&history, room, DETAIL_K, TICKS);
        detail.push((room, led));
        println!(
            "  room {}: R(K) = [{}], Kc = {}",
            room.label(),
            rs.iter()
                .map(|r| format!("{r:.3}"))
                .collect::<Vec<_>>()
                .join(", "),
            k_here
                .map(|k| format!("{k:.3}"))
                .unwrap_or_else(|| ">0.400".to_string()),
        );
    }
    for (room, led) in &detail {
        println!(
            "  room {} at K={:.2}: R {:.3}, clusters>=3 {}, largest {}, in-big {}, flashes {}, west R {:.3}, east R {:.3}, province gap {:.3} rad",
            room.label(),
            DETAIL_K,
            led.r_final,
            led.clusters_ge3,
            led.largest_cluster,
            led.in_big_clusters,
            led.flashes,
            led.west_r,
            led.east_r,
            led.province_gap,
        );
    }

    let get = |room: Room| detail.iter().find(|(r, _)| *r == room).unwrap().1.clone();
    let kc_of = |room: Room| kc.iter().find(|(r, _)| *r == room).unwrap().1;
    let corpse = get(Room::CorpseHashLocal);
    let c_hash = get(Room::CadenceHashLocal);
    let c_era = get(Room::CadenceEraLocal);
    let shuf = get(Room::ShuffledEraLocal);
    let glob = get(Room::CadenceGlobal);
    let kc_corpse = kc_of(Room::CorpseHashLocal);
    let all_cadence_kc_higher = [
        Room::CadenceHashLocal,
        Room::CadenceEraLocal,
        Room::ShuffledEraLocal,
        Room::CadenceGlobal,
    ]
    .iter()
    .all(|r| match (kc_corpse, kc_of(*r)) {
        (Some(a), Some(b)) => b > a,
        (Some(_), None) => true,
        _ => false,
    });
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the corpse's narrow band has the lowest Kc of any room: Kc corpse {} vs cadence rooms [{}] — {}",
        kc_corpse.map(|k| format!("{k:.3}")).unwrap_or_else(|| ">0.400".into()),
        [Room::CadenceHashLocal, Room::CadenceEraLocal, Room::ShuffledEraLocal, Room::CadenceGlobal]
            .iter()
            .map(|r| kc_of(*r).map(|k| format!("{k:.3}")).unwrap_or_else(|| ">0.400".into()))
            .collect::<Vec<_>>()
            .join(", "),
        if all_cadence_kc_higher { "HELD" } else { "CRACKED" },
    );
    // P2 is judged at K=0.05, so run those two rooms at that K here.
    let corpse_05 = run_room(&history, Room::CorpseHashLocal, 0.05, TICKS);
    let chash_05 = run_room(&history, Room::CadenceHashLocal, 0.05, TICKS);
    let cera_05 = run_room(&history, Room::CadenceEraLocal, 0.05, TICKS);
    println!(
        "    P2 at K=0.05 the cadence rooms fragment (clusters>=3 >= 3 in both local cadence rooms) while the corpse holds one dominant cluster (largest >= 90% of N): corpse largest {}, cadence-hash clusters {}, cadence-era clusters {} — {}",
        corpse_05.largest_cluster,
        chash_05.clusters_ge3,
        cera_05.clusters_ge3,
        if corpse_05.largest_cluster as f64 >= 0.9 * n as f64
            && chash_05.clusters_ge3 >= 3
            && cera_05.clusters_ge3 >= 3
        {
            "HELD"
        } else {
            "CRACKED"
        },
    );
    println!(
        "    P3 era placement does not rescue cadence locally (R era < R hash at K=0.10): {:.3} vs {:.3} — {}",
        c_era.r_final,
        c_hash.r_final,
        if c_era.r_final < c_hash.r_final { "HELD" } else { "CRACKED" },
    );
    println!(
        "    P4 shuffling the cadence assignment on the era map raises R at K=0.10 above the true assignment: shuffled {:.3} vs era {:.3} — {}",
        shuf.r_final,
        c_era.r_final,
        if shuf.r_final > c_era.r_final { "HELD" } else { "CRACKED" },
    );
    println!(
        "    P5 global coupling erases placement (R global >= both local cadence rooms at K=0.10): {:.3} vs hash {:.3}, era {:.3} — {}",
        glob.r_final,
        c_hash.r_final,
        c_era.r_final,
        if glob.r_final >= c_hash.r_final && glob.r_final >= c_era.r_final {
            "HELD"
        } else {
            "CRACKED"
        },
    );
    println!(
        "    P6 provinces: at K=0.10 each era half is more coherent than the era whole (west {:.3}, east {:.3} vs whole {:.3}) and the era province gap exceeds the hash room's ({:.3} vs {:.3} rad) — {}",
        c_era.west_r,
        c_era.east_r,
        c_era.r_final,
        c_era.province_gap,
        c_hash.province_gap,
        if c_era.west_r > c_era.r_final
            && c_era.east_r > c_era.r_final
            && c_era.province_gap > c_hash.province_gap
        {
            "HELD"
        } else {
            "CRACKED"
        },
    );
    let _ = corpse;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datum(hash_byte_seed: u8, ts: i64) -> CommitDatum {
        // A 40-hex-char hash whose first bytes are seed-derived.
        let mut s = String::new();
        for i in 0..20u16 {
            let b = (hash_byte_seed as u16).wrapping_add(i * 37) % 256;
            s.push_str(&format!("{:02x}", b as u8));
        }
        let _ = ts;
        CommitDatum {
            hash: s,
            timestamp: ts,
        }
    }

    fn history_with_gaps(gaps_h: &[f64]) -> Vec<CommitDatum> {
        // Newest first; timestamps walk backwards by the gaps.
        let mut out = Vec::new();
        let mut t: i64 = 1_760_000_000;
        for (i, _) in gaps_h.iter().enumerate() {
            out.push(datum((i * 53 + 11) as u8, t));
            t -= (gaps_h[i] * 3600.0) as i64;
        }
        out
    }

    #[test]
    fn burst_burns_faster_than_lull() {
        let h = history_with_gaps(&[0.01, 1.0, 100.0, 10.0]);
        let om = cadence_omegas(&h);
        assert!(om[0] > om[2], "burst {} should exceed lull {}", om[0], om[2]);
        assert!(om[0] <= OMEGA_MAX + 1e-12 && om[2] >= OMEGA_MIN - 1e-12);
    }

    #[test]
    fn oldest_commit_inherits_median_gap() {
        let h = history_with_gaps(&[2.0, 4.0, 8.0, 16.0]);
        let gaps = gaps_hours(&h);
        assert_eq!(gaps.len(), 4);
        assert!((gaps[3] - 4.0).abs() < 1e-9, "oldest gap {}", gaps[3]);
    }

    #[test]
    fn era_and_hash_placements_differ_only_in_x() {
        let h = history_with_gaps(&[1.0; 8]);
        let n = h.len();
        for (i, c) in h.iter().enumerate() {
            let hp = hash_position(&c.hash);
            let ep = era_position(&c.hash, i, n);
            assert_eq!(hp.1, ep.1);
            assert!((ep.0 - (i as f64 + 0.5) / n as f64 * FIELD).abs() < 1e-9);
        }
    }

    #[test]
    fn shuffled_room_is_a_permutation_of_cadence() {
        let h = history_with_gaps(&[0.1, 0.5, 3.0, 20.0, 1.0, 8.0, 0.2, 40.0]);
        let a = build_swarm(&h, Room::CadenceEraLocal).omega;
        let mut b = build_swarm(&h, Room::ShuffledEraLocal).omega;
        let mut a_sorted = a.clone();
        a_sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
        b.sort_by(|x, y| x.partial_cmp(y).unwrap());
        assert_eq!(a_sorted, b);
        assert_eq!(
            build_swarm(&h, Room::CadenceEraLocal).pos,
            build_swarm(&h, Room::ShuffledEraLocal).pos
        );
        let _ = a;
    }

    #[test]
    fn two_coupled_fireflies_converge() {
        // The corpse's own test, kept in the lineage.
        let swarm = Swarm {
            omega: vec![0.0, 0.0],
            pos: vec![(50.0, 50.0), (50.5, 50.0)],
            phase0: vec![0.0, std::f64::consts::FRAC_PI_2],
        };
        let adjacency = neighbours(&swarm.pos);
        assert_eq!(adjacency[0], vec![1]);
        let mut phases = swarm.phase0.clone();
        for _ in 0..50 {
            let mut next = phases.clone();
            for i in 0..2 {
                let sum: f64 = adjacency[i]
                    .iter()
                    .map(|&j| (phases[j] - phases[i]).sin())
                    .sum();
                next[i] = phases[i]
                    + (swarm.omega[i] + 1.2 / adjacency[i].len() as f64 * sum) * DT;
            }
            phases = next.iter().map(|p| p.rem_euclid(TAU)).collect();
        }
        let mut d = (phases[0] - phases[1]).rem_euclid(TAU);
        if d > TAU / 2.0 {
            d = TAU - d;
        }
        assert!(d < 0.2, "phases should converge, diff {d}");
    }

    #[test]
    fn uncoupled_firefly_advances_at_its_own_omega() {
        let swarm = Swarm {
            omega: vec![0.11],
            pos: vec![(10.0, 10.0)],
            phase0: vec![0.3],
        };
        let led = run_local(&swarm, 0.0, 100);
        // One firefly: R is trivially 1; the flash count is the
        // honest ledger — 0.3 + 100*0.1*0.11 = 1.4 rad < TAU.
        assert_eq!(led.flashes, 0);
        let led2 = run_local(&swarm, 0.0, 600);
        assert_eq!(led2.flashes, 1, "600 ticks * 0.011 rad = 6.6 + 0.3 crosses TAU once");
    }

    #[test]
    fn global_mean_field_matches_direct_sum() {
        // (1/N) Σ sin(θj − θi) == R·sin(Θ − θi), the identity the
        // global room is built on.
        let phases: Vec<f64> = (0..37)
            .map(|i| (i as f64 * 0.37) % TAU)
            .collect();
        let (r, mean) = order_parameter(&phases);
        for i in 0..phases.len() {
            let direct: f64 = phases
                .iter()
                .map(|p| (p - phases[i]).sin())
                .sum::<f64>()
                / phases.len() as f64;
            let mf = r * (mean - phases[i]).sin();
            assert!((direct - mf).abs() < 1e-9, "i={i} direct {direct} mf {mf}");
        }
    }

    #[test]
    fn runs_are_deterministic() {
        let h = synthetic_history();
        let a = run_room(&h, Room::CadenceEraLocal, DETAIL_K, 300);
        let b = run_room(&h, Room::CadenceEraLocal, DETAIL_K, 300);
        assert_eq!(a, b);
    }

    #[test]
    fn cluster_census_counts_constructed_groups() {
        // Four fireflies in a line, phases [0, 0.1, 3.0, 3.1]:
        // two coherent pairs, no cluster of >= 3.
        let pos = vec![(0.0, 0.0), (5.0, 0.0), (10.0, 0.0), (15.0, 0.0)];
        let adjacency = neighbours(&pos);
        let phases = vec![0.0, 0.1, 3.0, 3.1];
        let (ge3, largest, in_big) = cluster_census(&phases, &adjacency);
        assert_eq!(ge3, 0);
        assert_eq!(largest, 2);
        assert_eq!(in_big, 0);
    }

    #[test]
    fn synthetic_survey_separates_corpse_from_cadence_era() {
        // On the synthetic two-regime history, the narrow corpse
        // band out-synchronises the era-placed cadence swarm at the
        // detail coupling — the survey's core mechanic.
        let h = synthetic_history();
        let corpse = run_room(&h, Room::CorpseHashLocal, DETAIL_K, 800);
        let era = run_room(&h, Room::CadenceEraLocal, DETAIL_K, 800);
        assert!(
            corpse.r_final > era.r_final,
            "corpse {} vs era {}",
            corpse.r_final,
            era.r_final
        );
    }
}
