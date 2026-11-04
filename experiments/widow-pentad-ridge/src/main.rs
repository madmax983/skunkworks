//! # Widow Pentad Ridge 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`.
//!
//! `widow-pentad-diagonal` found that its tall cells trace a
//! diagonal ridge: lighter doses arriving sooner balance heavier
//! doses arriving later. Three of its tall cells sit exactly on
//! one compensation line — delay = 2 + (weight − 1.00)/0.25:
//! (+1.00, 2) longest 40, (+1.25, 3) longest 46, (+1.50, 4)
//! longest 55, the crown. This instrument walks that line beyond
//! the diagonal's grid in both directions — (+0.50, 0) and
//! (+0.75, 1) behind, (+1.75, 5) and (+2.00, 6) beyond the
//! crown — beside two one-tick-early controls at the extension
//! weights, (+1.75, 4) and (+2.00, 5). If the ridge is a real
//! compensation (dose traded against arrival), the line should
//! keep its height past the crown; if the crown was a tooth that
//! merely leaned, the line falls off its far side.
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo
//! period ~39.60), follower Fast Spiking I=5.3 (solo ~24.06).
//! Five follower beats span ~120.31 against three driver cycles'
//! ~118.80 — near 5:3, the follower late by ~1.51 ticks per
//! super-cycle. A super-cycle is three driver cycles; perfect =
//! exactly five follower beats, each within ±0.06 of its median
//! phase. An episode is a maximal run of perfect super-cycles.
//! Counted over 24,000 ticks per cell (first 1,500 discarded;
//! ~188 super-cycles), exactly as the diagonal counted.
//!
//! Predictions declared before the extension cells ran:
//! - P1: no cell earns TRUE LOCK 5:3.
//! - P2: the three on-line anchors reproduce the diagonal's
//!   ledger exactly: longest 40 / 46 / 55.
//! - P3: the ridge continues past the crown — at least one of
//!   (+1.75, 5), (+2.00, 6) reaches the plateau line 37.
//! - P4: the ridge has a western foot — both (+0.50, 0) and
//!   (+0.75, 1) fall at or below the isolation line 23.
//! - P5: each one-tick-early control falls below its on-line
//!   partner at the same weight — arrival, not dose, is what
//!   the line is paying for.
//!
//! Every period is a named constant or a measured quantity.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

const DRIVER_CURRENT: f32 = 12.0;
const FOLLOWER_CURRENT: f32 = 5.3;
const TICKS: usize = 24_000;
const SETTLE_TICKS: usize = 1_500;
const PHASE_TOL: f64 = 0.06;
const EPISODIC_FLOOR: usize = 10;
/// Plateau line from the diagonal grid: 80% of its centre (46), rounded up.
const PLATEAU_LINE: usize = 37;
/// Isolation line from the diagonal grid: 50% of its centre.
const ISOLATION_LINE: usize = 23;

/// The compensation line, west to east: (weight, delay).
const LINE: [(f32, usize); 7] = [
    (0.5, 0), (0.75, 1), (1.0, 2), (1.25, 3), (1.5, 4), (1.75, 5), (2.0, 6),
];
/// One-tick-early controls at the two extension weights.
const CONTROLS: [(f32, usize); 2] = [(1.75, 4), (2.0, 5)];
/// On-line anchors and their diagonal-grid longest episodes.
const ANCHORS: [(f32, usize, usize); 3] = [(1.0, 2, 40), (1.25, 3, 46), (1.5, 4, 55)];

#[derive(Debug, Clone, Copy)]
struct SoloClock { spikes: usize, mean_isi: f64 }

#[derive(Debug, Clone)]
struct CellLedger {
    weight: f32, delay: usize, super_cycles: usize, perfect: usize,
    count_slips: usize, phase_slips: usize, episodes: usize,
    longest_episode: usize, mean_episode: f64, verdict: &'static str,
}

fn build_pair(weight: f32, delay: usize) -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    if weight != 0.0 {
        if delay == 0 { net.add_synapse(driver, follower, weight); }
        else { net.add_synapse_with_delay(driver, follower, weight, delay); }
    }
    net
}

fn mean_isi(times: &[usize]) -> f64 {
    if times.len() < 2 { return 0.0; }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

fn measure_solo(fast_spiking: bool, current: f32) -> SoloClock {
    let mut net = Network::new();
    let n = net.add_neuron();
    if fast_spiking { net.neurons[n] = Izhikevich::new_fast_spiking(); }
    let mut times: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[current]);
        if t >= SETTLE_TICKS && net.is_spiking(n) { times.push(t); }
    }
    SoloClock { spikes: times.len(), mean_isi: mean_isi(&times) }
}

fn circ_dist(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 1.0;
    d.min(1.0 - d)
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() { return 0.0; }
    let mut v: Vec<f64> = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn run_cell(weight: f32, delay: usize) -> CellLedger {
    let mut net = build_pair(weight, delay);
    let mut driver_times: Vec<usize> = Vec::new();
    let mut follower_times: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[DRIVER_CURRENT, FOLLOWER_CURRENT]);
        if t >= SETTLE_TICKS {
            if net.is_spiking(0) { driver_times.push(t); }
            if net.is_spiking(1) { follower_times.push(t); }
        }
    }
    let n_super = driver_times.len().saturating_sub(3) / 3;
    let mut windows: Vec<Vec<f64>> = Vec::with_capacity(n_super);
    for k in 0..n_super {
        let (start, end) = (driver_times[3 * k], driver_times[3 * k + 3]);
        let beats: Vec<f64> = follower_times.iter().copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64).collect();
        windows.push(beats);
    }
    let mut medians = [0.0f64; 5];
    for j in 0..5 {
        let phases: Vec<f64> = windows.iter().filter(|w| w.len() == 5).map(|w| w[j]).collect();
        medians[j] = median(&phases);
    }
    let mut perfect_flags: Vec<bool> = Vec::with_capacity(windows.len());
    let (mut count_slips, mut phase_slips) = (0usize, 0usize);
    for w in &windows {
        if w.len() != 5 { count_slips += 1; perfect_flags.push(false); }
        else if (0..5).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
            phase_slips += 1; perfect_flags.push(false);
        } else { perfect_flags.push(true); }
    }
    let perfect = perfect_flags.iter().filter(|&&p| p).count();
    let (mut episodes, mut longest, mut run, mut total) = (0usize, 0usize, 0usize, 0usize);
    for &p in &perfect_flags {
        if p { run += 1; total += 1; } else {
            if run > 0 { episodes += 1; longest = longest.max(run); }
            run = 0;
        }
    }
    if run > 0 { episodes += 1; longest = longest.max(run); }
    let mean_episode = if episodes > 0 { total as f64 / episodes as f64 } else { 0.0 };
    let verdict = if !perfect_flags.is_empty() && perfect == perfect_flags.len() {
        "TRUE LOCK 5:3"
    } else if longest >= EPISODIC_FLOOR { "EPISODIC" } else { "WALKING" };
    CellLedger { weight, delay, super_cycles: windows.len(), perfect,
        count_slips, phase_slips, episodes, longest_episode: longest,
        mean_episode, verdict }
}

fn run_survey() -> Vec<CellLedger> {
    let mut cells = Vec::new();
    for &(w, d) in LINE.iter().chain(CONTROLS.iter()) { cells.push(run_cell(w, d)); }
    cells
}

fn cell_at(cells: &[CellLedger], weight: f32, delay: usize) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight && c.delay == delay).expect("surveyed cell")
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-pentad-ridge headless: walking the diagonal's compensation line past the crown — ridge, or a tooth that leaned?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded; a super-cycle is perfect with exactly 5 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 3 driver cycles {:.2} ticks against 5 follower beats {:.2} — the follower runs late by {:.2} ticks per super-cycle; near 5:3, not at it",
            3.0 * driver.mean_isi, 5.0 * follower.mean_isi, 5.0 * follower.mean_isi - 3.0 * driver.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the compensation line (delay = 2 + (weight - 1.00)/0.25), west to east:");
    for &(w, d) in &LINE {
        let c = cell_at(cells, w, d);
        println!("    on-line ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    for &(w, d) in &CONTROLS {
        let c = cell_at(cells, w, d);
        println!("    control ({w:+.2}, delay {d}, one tick early): perfect {}/{}, longest {}, mean episode {:.1} — {}",
            c.perfect, c.super_cycles, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 5:3").count();
    let anchors_ok = ANCHORS.iter().all(|&(w, d, l)| cell_at(cells, w, d).longest_episode == l);
    let beyond = [cell_at(cells, 1.75, 5), cell_at(cells, 2.0, 6)];
    let ridge_continues = beyond.iter().any(|c| c.longest_episode >= PLATEAU_LINE);
    let west = [cell_at(cells, 0.5, 0), cell_at(cells, 0.75, 1)];
    let western_foot = west.iter().all(|c| c.longest_episode <= ISOLATION_LINE);
    let controls_below = CONTROLS.iter().all(|&(w, d)| {
        cell_at(cells, w, d).longest_episode
            < cell_at(cells, w, d + 1).longest_episode
    });
    let crown = cell_at(cells, 1.5, 4);
    let past_crown_max = beyond.iter().map(|c| c.longest_episode).max().unwrap_or(0);
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  crown (+1.50, delay 4): longest {} — best past-crown on-line cell: longest {}", crown.longest_episode, past_crown_max);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce (40/46/55) — {}; P3 ridge continues past crown (some extension >= plateau {}) — {}; P4 western foot (both west cells <= isolation {}) — {}; P5 early controls below on-line partners — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_ok { "HELD" } else { "CRACKED" },
        PLATEAU_LINE, if ridge_continues { "HELD" } else { "CRACKED" },
        ISOLATION_LINE, if western_foot { "HELD" } else { "CRACKED" },
        if controls_below { "HELD" } else { "CRACKED" });
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let bare = run_cell(0.0, 0);
    let cells = run_survey();
    print_ledger(&driver, &follower, &bare, &cells);
    if !headless { println!("  (count it headless any time: --headless)"); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_clocks_are_near_five_to_three_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        let gap = 5.0 * follower.mean_isi - 3.0 * driver.mean_isi;
        assert!(gap > 0.5 && gap < 3.0, "super-cycle lateness {gap}");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let c = run_cell(1.25, 3);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 150, "super-cycles {}", c.super_cycles);
    }

    #[test]
    fn anchors_reproduce_the_diagonal_ledger() {
        for &(w, d, l) in &ANCHORS {
            let c = run_cell(w, d);
            assert_eq!(c.longest_episode, l, "anchor ({w:+.2}, {d}) longest {}", c.longest_episode);
        }
    }

    #[test]
    fn no_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 9);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 5:3", "cell ({:+.2}, {}) claims a lock", c.weight, c.delay);
        }
    }

    #[test]
    fn survey_is_pinned_as_measured() {
        // The measured walk, pinned so the record cannot heal:
        // the ridge is a finite segment. On-line longest episodes
        // west to east: 27, 12, 40, 46, 55 (the crown), then the
        // line falls — 16, 34 — and neither extension reaches the
        // plateau line 37. The early controls: (+1.75, 4) at 25
        // stands ABOVE its on-line partner (+1.75, 5) at 16, and
        // (+2.00, 5) ties its partner at 34 — past the crown the
        // compensation the line was paying for has expired.
        let cells = run_survey();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(0.5, 0), 27);
        assert_eq!(longest(0.75, 1), 12);
        assert_eq!(longest(1.0, 2), 40);
        assert_eq!(longest(1.25, 3), 46);
        assert_eq!(longest(1.5, 4), 55);
        assert_eq!(longest(1.75, 5), 16);
        assert_eq!(longest(2.0, 6), 34);
        assert_eq!(longest(1.75, 4), 25);
        assert_eq!(longest(2.0, 5), 34);
    }

    #[test]
    fn survey_shape_is_the_declared_line() {
        assert_eq!(LINE.len(), 7);
        assert_eq!(CONTROLS, [(1.75, 4), (2.0, 5)]);
        for &(w, d) in &LINE {
            assert_eq!(d as i32, 2 + ((w - 1.0) / 0.25) as i32, "({w}, {d}) off the line");
        }
    }
}
