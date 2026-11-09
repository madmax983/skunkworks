//! # Widow Heptad Ridge 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-diagonal` found that its tall cells trace a
//! compensation diagonal: (-0.50, delay 8) longest 21, (-0.75,
//! delay 9) longest 51, (-1.00, delay 10) longest 58, the crown —
//! heavier inhibitory doses arriving later balancing lighter
//! doses arriving sooner. Three of its cells sit exactly on one
//! compensation line — delay = 8 + (-0.50 - weight)/0.25. Its
//! README named this instrument outright: *walk the
//! compensation line itself out of this grid in both directions
//! — does the line fall off a cliff past the crown (-1.25,
//! delay 11 / -1.50, delay 12), and does ordinary ground resume
//! west of (-0.50, 8) at (-0.25, delay 7)?* `widow-pentad-ridge`
//! walked the 5:3 line the same way and found a finite segment:
//! a ditch to the west, a cliff past its crown, and early
//! controls that matched or beat their on-line partners past
//! the crown. This instrument walks the 7:5 line, under the
//! opposite medicine, so the two ratios' extents can be
//! compared step for step.
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo
//! period ~39.60), follower Fast Spiking I=4.8 (solo ~28.01).
//! Seven follower beats span ~196.08 against five driver
//! cycles' ~198.00 — near 7:5, the follower early by ~1.92
//! ticks per super-cycle. A **super-cycle** is five consecutive
//! driver cycles; it is **perfect** when it holds exactly seven
//! follower beats AND each beat's phase lies within ±0.06 of
//! that beat's median phase for the regime (the two-gate rule,
//! unchanged). An **episode** is a maximal run of consecutive
//! perfect super-cycles. Each cell counted over 24,000 ticks
//! (first 1,500 discarded; 112 super-cycles), exactly as the
//! diagonal counted.
//!
//! The survey is the six on-line cells — (-0.25, 7), (-0.50, 8),
//! (-0.75, 9), (-1.00, 10), (-1.25, 11), (-1.50, 12) — beside
//! three **one-tick-early controls** at the three weights the
//! diagonal never surveyed: (-0.25, 6), (-1.25, 10),
//! (-1.50, 11).
//!
//! Lines declared before the survey ran, from the crown
//! (-1.00, 10) at longest 58: plateau line 47 (80% of 58,
//! rounded up); isolation line 29 (50% of 58).
//!
//! Predictions declared before the extension cells ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives the walk, as it survived every grid.
//! - P2: the three on-line anchors reproduce the diagonal's
//!   ledger exactly: longest 21 / 51 / 58 west to east.
//! - P3: the ridge continues past the crown — at least one of
//!   (-1.25, 11), (-1.50, 12) reaches the plateau line 47.
//! - P4: ordinary ground resumes in the west — (-0.25, 7)
//!   falls at or below the isolation line 29.
//! - P5: each one-tick-early control falls below its on-line
//!   partner at the same weight — arrival, not dose, is what
//!   the line is paying for.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// Driver: Regular Spiking, I=12 (solo period measured, ~39.60).
const DRIVER_CURRENT: f32 = 12.0;
/// Follower: Fast Spiking, I=4.8 (solo period measured, ~28.01).
const FOLLOWER_CURRENT: f32 = 4.8;
/// Total ticks per cell.
const TICKS: usize = 24_000;
/// Settling ticks discarded at the start of each cell.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;
/// Plateau line: 80% of the crown's longest (58), rounded up.
const PLATEAU_LINE: usize = 47;
/// Isolation line: 50% of the crown's longest (58).
const ISOLATION_LINE: usize = 29;
/// The crown's address and longest episode (the diagonal grid).
const CROWN: (f32, usize, usize) = (-1.0, 10, 58);

/// The compensation line, west to east: (weight, delay).
const LINE: [(f32, usize); 6] = [
    (-0.25, 7), (-0.5, 8), (-0.75, 9), (-1.0, 10), (-1.25, 11), (-1.5, 12),
];
/// One-tick-early controls at the three newly surveyed weights.
const CONTROLS: [(f32, usize); 3] = [(-0.25, 6), (-1.25, 10), (-1.5, 11)];
/// On-line anchors and their diagonal-grid longest episodes.
const ANCHORS: [(f32, usize, usize); 3] = [(-0.5, 8, 21), (-0.75, 9, 51), (-1.0, 10, 58)];

#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

#[derive(Debug, Clone)]
struct CellLedger {
    weight: f32,
    delay: usize,
    super_cycles: usize,
    perfect: usize,
    count_slips: usize,
    phase_slips: usize,
    episodes: usize,
    longest_episode: usize,
    mean_episode: f64,
    verdict: &'static str,
}

fn build_pair(weight: f32, delay: usize) -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    if weight != 0.0 {
        if delay == 0 {
            net.add_synapse(driver, follower, weight);
        } else {
            net.add_synapse_with_delay(driver, follower, weight, delay);
        }
    }
    net
}

fn measure_solo(fast_spiking: bool, current: f32) -> SoloClock {
    let mut net = Network::new();
    let n = net.add_neuron();
    if fast_spiking {
        net.neurons[n] = Izhikevich::new_fast_spiking();
    }
    let mut times: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[current]);
        if t >= SETTLE_TICKS && net.is_spiking(n) {
            times.push(t);
        }
    }
    SoloClock { spikes: times.len(), mean_isi: mean_isi(&times) }
}

fn mean_isi(times: &[usize]) -> f64 {
    if times.len() < 2 { return 0.0; }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
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
    let n_super = driver_times.len().saturating_sub(5) / 5;
    let mut windows: Vec<Vec<f64>> = Vec::with_capacity(n_super);
    for k in 0..n_super {
        let (start, end) = (driver_times[5 * k], driver_times[5 * k + 5]);
        let beats: Vec<f64> = follower_times.iter().copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64).collect();
        windows.push(beats);
    }
    let mut medians = [0.0f64; 7];
    for j in 0..7 {
        let phases: Vec<f64> = windows.iter().filter(|w| w.len() == 7).map(|w| w[j]).collect();
        medians[j] = median(&phases);
    }
    let mut perfect_flags: Vec<bool> = Vec::with_capacity(windows.len());
    let mut count_slips = 0usize;
    let mut phase_slips = 0usize;
    for w in &windows {
        if w.len() != 7 {
            count_slips += 1;
            perfect_flags.push(false);
        } else if (0..7).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
            phase_slips += 1;
            perfect_flags.push(false);
        } else {
            perfect_flags.push(true);
        }
    }
    let perfect = perfect_flags.iter().filter(|&&p| p).count();
    let mut episodes = 0usize;
    let mut longest = 0usize;
    let mut run = 0usize;
    let mut total_in_episodes = 0usize;
    for &p in &perfect_flags {
        if p { run += 1; total_in_episodes += 1; } else {
            if run > 0 { episodes += 1; longest = longest.max(run); }
            run = 0;
        }
    }
    if run > 0 { episodes += 1; longest = longest.max(run); }
    let mean_episode = if episodes > 0 { total_in_episodes as f64 / episodes as f64 } else { 0.0 };
    let verdict = if !perfect_flags.is_empty() && perfect == perfect_flags.len() {
        "TRUE LOCK 7:5"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else { "WALKING" };
    CellLedger {
        weight, delay,
        super_cycles: windows.len(), perfect, count_slips, phase_slips,
        episodes, longest_episode: longest, mean_episode, verdict,
    }
}

fn run_walk() -> Vec<CellLedger> {
    let mut cells = Vec::with_capacity(LINE.len() + CONTROLS.len());
    for &(w, d) in LINE.iter().chain(CONTROLS.iter()) {
        cells.push(run_cell(w, d));
    }
    cells
}

fn cell_at(cells: &[CellLedger], weight: f32, delay: usize) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight && c.delay == delay).expect("surveyed cell")
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-ridge headless: walking the 7:5 compensation line out of the diagonal's grid in both directions — a road, or a finite segment with the crown at its end?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the compensation line, west to east (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}):");
    for &(w, d) in &LINE {
        let c = cell_at(cells, w, d);
        let mark = if (w, d) == (CROWN.0, CROWN.1) { " (crown)" } else if ANCHORS.contains(&(w, d, c.longest_episode)) { " (anchor)" } else { "" };
        println!("    on-line ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    println!("  one-tick-early controls:");
    for &(w, d) in &CONTROLS {
        let c = cell_at(cells, w, d);
        println!("    control ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, d, l)| cell_at(cells, w, d).longest_episode == l);
    let east_max = [cell_at(cells, -1.25, 11), cell_at(cells, -1.5, 12)]
        .iter().map(|c| c.longest_episode).max().unwrap();
    let west = cell_at(cells, -0.25, 7);
    let controls_below = CONTROLS.iter().all(|&(w, d)| {
        let on_line = LINE.iter().find(|&&(lw, _)| lw == w).map(|&(_, ld)| cell_at(cells, w, ld));
        match on_line {
            Some(partner) => cell_at(cells, w, d).longest_episode < partner.longest_episode,
            None => false,
        }
    });
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce longest 21 / 51 / 58 — {}; P3 ridge continues past crown (an eastern extension reaches plateau {PLATEAU_LINE}; best eastern longest {east_max}) — {}; P4 ordinary ground in the west ((-0.25, 7) at or below isolation {ISOLATION_LINE}; measured {}) — {}; P5 every one-tick-early control below its on-line partner — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        if east_max >= PLATEAU_LINE { "HELD" } else { "CRACKED" },
        west.longest_episode,
        if west.longest_episode <= ISOLATION_LINE { "HELD" } else { "CRACKED" },
        if controls_below { "HELD" } else { "CRACKED" });
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let bare = run_cell(0.0, 0);
    let cells = run_walk();
    print_ledger(&driver, &follower, &bare, &cells);
    if !headless { println!("  (count it headless any time: --headless)"); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_clocks_are_near_seven_to_five_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        let gap = 5.0 * driver.mean_isi - 7.0 * follower.mean_isi;
        assert!(gap > 0.5 && gap < 3.5, "super-cycle earliness {gap}");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let c = run_cell(-0.75, 9);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn anchors_reproduce_the_diagonal_ledger() {
        for &(w, d, longest) in &ANCHORS {
            let c = run_cell(w, d);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {d}) longest {}", c.longest_episode);
        }
    }

    #[test]
    fn line_and_controls_are_the_declared_walk() {
        assert_eq!(LINE.len(), 6);
        assert_eq!(CONTROLS.len(), 3);
        // The line: each step east is a quarter-dose heavier and
        // one tick later — the compensation, stated as arithmetic.
        for pair in LINE.windows(2) {
            assert_eq!(pair[1].0 - pair[0].0, -0.25);
            assert_eq!(pair[1].1 - pair[0].1, 1);
        }
        // Every control is one tick early for an on-line weight
        // the diagonal never surveyed.
        for &(w, d) in &CONTROLS {
            assert!(LINE.contains(&(w, d + 1)), "control ({w}, {d}) has no on-line partner");
        }
    }

    #[test]
    fn measured_walk_is_pinned_as_found() {
        // The measured walk, pinned so the record cannot heal:
        // on-line west to east 17, 21, 51, 58, 34, 37 — the tall
        // segment is (-0.75, 9) to the crown (-1.00, 10); past
        // the crown the line falls 58 -> 34 and recovers only to
        // 37, both below the plateau line 47 (P3 CRACKED); west
        // of (-0.50, 8) the line is at the bare pair's own 17
        // (P4 HELD). Controls 33 / 39 / 15: two of three beat
        // their on-line partners (P5 CRACKED).
        let cells = run_walk();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-0.25, 7), 17);
        assert_eq!(longest(-0.5, 8), 21);
        assert_eq!(longest(-0.75, 9), 51);
        assert_eq!(longest(-1.0, 10), 58);
        assert_eq!(longest(-1.25, 11), 34);
        assert_eq!(longest(-1.5, 12), 37);
        assert_eq!(longest(-0.25, 6), 33);
        assert_eq!(longest(-1.25, 10), 39);
        assert_eq!(longest(-1.5, 11), 15);
        assert_eq!(cell_at(&cells, -1.0, 10).perfect, 92);
        assert_eq!(cell_at(&cells, -0.75, 9).perfect, 86);
    }

    #[test]
    fn no_walked_cell_claims_a_true_lock() {
        let cells = run_walk();
        assert_eq!(cells.len(), 9);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.2}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
