//! # Widow Pentad Plane 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-pentad` counted the near-5:3 pair along one row of its
//! world: ten couplings, eight of them at a single delay (8 ticks).
//! It found a late follower, an excitatory medicine, a best hand
//! (+1.0, delay 8; longest episode 47 super-cycles), a dose ceiling,
//! and no lock. A row is not a map. This instrument surveys the
//! whole (weight, delay) plane for the same pair — 9 weights × 7
//! delays = 63 cells, plus the uncoupled pair — at the same long
//! window, under the pentad's own two-gate rule, so the row's
//! findings can be checked against the ground they were drawn on.
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo period
//! ~39.60), follower Fast Spiking I=5.3 (solo period ~24.06). Five
//! follower beats span ~120.31 against three driver cycles'
//! ~118.80 — near 5:3, the follower late by ~1.51 ticks per
//! super-cycle. A **super-cycle** is three consecutive driver
//! cycles; it is **perfect** when it holds exactly five follower
//! beats AND each beat's phase lies within ±0.06 of that beat's
//! median phase for the regime. An **episode** is a maximal run
//! of perfect super-cycles. Ledger counted over 24,000 ticks per
//! cell (first 1,500 discarded; ~188 super-cycles).
//!
//! Predictions declared before the survey ran:
//! - P1: no cell earns TRUE LOCK 5:3 (the pentad's verdict
//!   generalises from a row to the plane).
//! - P2: the plane's best cell (longest episode) is excitatory —
//!   the sign of the medicine matches the sign of the drift
//!   everywhere, not only at delay 8.
//! - P3: the best cell's delay lies in 6..=10 (the right hour is a
//!   band, not a point).
//! - P4: the inhibitory half-plane's mean longest episode is
//!   below the excitatory half-plane's — delaying a late clock
//!   is worse on average, not only at its worst.
//! - P5: the dose ceiling generalises — the +4.0 row's best cell
//!   is shorter than the +1.0 row's best cell.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// Driver: Regular Spiking, I=12 (solo period measured, ~39.60).
const DRIVER_CURRENT: f32 = 12.0;
/// Follower: Fast Spiking, I=5.3 (solo period measured, ~24.06).
const FOLLOWER_CURRENT: f32 = 5.3;
/// Total ticks per cell.
const TICKS: usize = 24_000;
/// Settling ticks discarded at the start of each cell.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;

/// Weights surveyed: the inhibitory half-plane (the wrong
/// medicine, kept whole), then the excitatory doses the pentad's
/// row named, from gentle to crushing.
const WEIGHTS: [f32; 9] = [-2.5, -1.5, -1.0, -0.5, 0.5, 1.0, 1.5, 2.5, 4.0];
/// Delays surveyed: every second tick from none to twelve.
const DELAYS: [usize; 7] = [0, 2, 4, 6, 8, 10, 12];

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
    let mut count_slips = 0usize;
    let mut phase_slips = 0usize;
    for w in &windows {
        if w.len() != 5 {
            count_slips += 1;
            perfect_flags.push(false);
        } else if (0..5).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
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
        "TRUE LOCK 5:3"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else { "WALKING" };
    CellLedger {
        weight, delay,
        super_cycles: windows.len(), perfect, count_slips, phase_slips,
        episodes, longest_episode: longest, mean_episode, verdict,
    }
}

fn run_plane() -> Vec<CellLedger> {
    let mut cells = Vec::with_capacity(WEIGHTS.len() * DELAYS.len());
    for &w in &WEIGHTS {
        for &d in &DELAYS {
            cells.push(run_cell(w, d));
        }
    }
    cells
}

fn cell_at(cells: &[CellLedger], weight: f32, delay: usize) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight && c.delay == delay).expect("surveyed cell")
}

fn best_of<'a>(cells: impl Iterator<Item = &'a CellLedger>) -> Option<&'a CellLedger> {
    cells.max_by_key(|c| c.longest_episode)
}

fn mean_longest<'a>(cells: impl Iterator<Item = &'a CellLedger>) -> f64 {
    let v: Vec<usize> = cells.map(|c| c.longest_episode).collect();
    if v.is_empty() { 0.0 } else { v.iter().sum::<usize>() as f64 / v.len() as f64 }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-pentad-plane headless: the 5:3 plane, surveyed whole — longest episode per cell");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 5 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 3 driver cycles {:.2} ticks against 5 follower beats {:.2} — the follower runs late by {:.2} ticks per super-cycle; near 5:3, not at it",
            3.0 * driver.mean_isi, 5.0 * follower.mean_isi, 5.0 * follower.mean_isi - 3.0 * driver.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  grid (longest episode; columns are delays {:?}):", DELAYS);
    print!("    weight\\delay");
    for d in DELAYS { print!(" {d:>5}"); }
    println!();
    for &w in &WEIGHTS {
        print!("    {w:+.1}       ");
        for &d in &DELAYS {
            print!(" {:>5}", cell_at(cells, w, d).longest_episode);
        }
        println!();
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 5:3").count();
    let episodic = cells.iter().filter(|c| c.verdict == "EPISODIC").count();
    let walking = cells.iter().filter(|c| c.verdict == "WALKING").count();
    println!("  verdicts across {} cells: TRUE LOCK {}, EPISODIC {}, WALKING {}", cells.len(), true_locks, episodic, walking);
    let best = best_of(cells.iter()).expect("plane is non-empty");
    println!("  best cell: weight {:+.1}, delay {} — perfect {}/{}, longest episode {} super-cycles, mean episode {:.1} — {}",
        best.weight, best.delay, best.perfect, best.super_cycles, best.longest_episode, best.mean_episode, best.verdict);
    let best_inh = best_of(cells.iter().filter(|c| c.weight < 0.0)).expect("inhibitory cells exist");
    let best_exc = best_of(cells.iter().filter(|c| c.weight > 0.0)).expect("excitatory cells exist");
    println!("  inhibitory best: weight {:+.1}, delay {} — longest {}; excitatory best: weight {:+.1}, delay {} — longest {}",
        best_inh.weight, best_inh.delay, best_inh.longest_episode, best_exc.weight, best_exc.delay, best_exc.longest_episode);
    println!("  mean longest episode: inhibitory {:.1} across {} cells, excitatory {:.1} across {} cells",
        mean_longest(cells.iter().filter(|c| c.weight < 0.0)), cells.iter().filter(|c| c.weight < 0.0).count(),
        mean_longest(cells.iter().filter(|c| c.weight > 0.0)), cells.iter().filter(|c| c.weight > 0.0).count());
    for &w in &[1.0f32, 4.0f32] {
        let row_best = best_of(cells.iter().filter(|c| c.weight == w)).expect("row exists");
        println!("  row {:+.1} best: delay {} — longest {}", w, row_best.delay, row_best.longest_episode);
    }
    for &d in &DELAYS {
        let col_best = best_of(cells.iter().filter(|c| c.delay == d)).expect("column exists");
        println!("  column delay {} best: weight {:+.1} — longest {}", d, col_best.weight, col_best.longest_episode);
    }
    let surveyed = cell_at(cells, 1.0, 8);
    println!("  the pentad's surveyed hand as a cell (+1.0, delay 8): perfect {}/{}, longest {} — the row's champion, re-counted on the plane",
        surveyed.perfect, surveyed.super_cycles, surveyed.longest_episode);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 best cell excitatory — {}; P3 best delay in 6..=10 — {}; P4 inhibitory mean longest below excitatory — {}; P5 +4.0 row best below +1.0 row best — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if best.weight > 0.0 { "HELD" } else { "CRACKED" },
        if (6..=10).contains(&best.delay) { "HELD" } else { "CRACKED" },
        if mean_longest(cells.iter().filter(|c| c.weight < 0.0)) < mean_longest(cells.iter().filter(|c| c.weight > 0.0)) { "HELD" } else { "CRACKED" },
        {
            let hi = best_of(cells.iter().filter(|c| c.weight == 4.0)).map(|c| c.longest_episode).unwrap_or(0);
            let lo = best_of(cells.iter().filter(|c| c.weight == 1.0)).map(|c| c.longest_episode).unwrap_or(0);
            if hi < lo { "HELD" } else { "CRACKED" }
        });
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let bare = run_cell(0.0, 0);
    let cells = run_plane();
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
        let c = run_cell(1.0, 8);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 150, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn surveyed_hand_reproduces_the_pentad_row() {
        // The plane's (+1.0, delay 8) cell is the pentad's surveyed
        // hand, counted by the same instrument: it must land on the
        // parent ledger's numbers, not merely near them.
        let c = run_cell(1.0, 8);
        assert_eq!(c.perfect, 119, "perfect {}", c.perfect);
        assert_eq!(c.longest_episode, 47, "longest {}", c.longest_episode);
    }

    #[test]
    fn no_cell_in_the_plane_claims_a_true_lock() {
        let cells = run_plane();
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 5:3",
                "cell ({:+.1}, {}) claims a lock with {} perfect of {}",
                c.weight, c.delay, c.perfect, c.super_cycles);
            if c.perfect == c.super_cycles {
                panic!("cell ({:+.1}, {}) is perfect everywhere but unlabelled", c.weight, c.delay);
            }
        }
    }

    #[test]
    fn best_cell_is_excitatory_and_outlasts_uncoupled() {
        let cells = run_plane();
        let best = best_of(cells.iter()).expect("plane is non-empty");
        assert!(best.weight > 0.0, "best cell weight {:+.1}", best.weight);
        let bare = run_cell(0.0, 0);
        assert!(best.longest_episode > bare.longest_episode,
            "best {} vs uncoupled {}", best.longest_episode, bare.longest_episode);
        // The inhibitory half-plane never even reaches the bare
        // pair: its best cell is shorter than no thread at all.
        let inh_best = best_of(cells.iter().filter(|c| c.weight < 0.0))
            .map(|c| c.longest_episode).unwrap_or(0);
        assert!(inh_best < bare.longest_episode,
            "inhibitory best {inh_best} vs uncoupled {}", bare.longest_episode);
    }

    #[test]
    fn p3_crack_is_measured_not_assumed() {
        // P3 declared the best delay would lie in 6..=10. It
        // CRACKED: the plane's champion is (+1.5, delay 4) at 55,
        // and the delay-8 column's best (47) is the pentad's own
        // hand, outlasted. Pin the measured coastline so the
        // crack stays on the record.
        let cells = run_plane();
        let best = best_of(cells.iter()).expect("plane is non-empty");
        assert_eq!((best.weight, best.delay), (1.5, 4),
            "best cell ({:+.1}, {})", best.weight, best.delay);
        assert_eq!(best.longest_episode, 55, "best longest {}", best.longest_episode);
        let col8 = best_of(cells.iter().filter(|c| c.delay == 8))
            .map(|c| c.longest_episode).unwrap_or(0);
        assert_eq!(col8, 47, "delay-8 column best {col8}");
        assert!(col8 < best.longest_episode);
    }

    #[test]
    fn inhibitory_half_plane_is_worse_on_average() {
        let cells = run_plane();
        let inh = mean_longest(cells.iter().filter(|c| c.weight < 0.0));
        let exc = mean_longest(cells.iter().filter(|c| c.weight > 0.0));
        assert!(inh < exc, "inhibitory mean {inh:.1} vs excitatory mean {exc:.1}");
    }

    #[test]
    fn dose_ceiling_generalises_across_the_plane() {
        let cells = run_plane();
        let hi = best_of(cells.iter().filter(|c| c.weight == 4.0)).map(|c| c.longest_episode).unwrap_or(0);
        let lo = best_of(cells.iter().filter(|c| c.weight == 1.0)).map(|c| c.longest_episode).unwrap_or(0);
        assert!(hi < lo, "+4.0 row best {hi} vs +1.0 row best {lo}");
    }
}
