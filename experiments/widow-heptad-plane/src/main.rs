//! # Widow Heptad Plane 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad` counted the near-7:5 pair along one row of its
//! world: ten couplings, seven of them at a single delay (8 ticks).
//! It found an early follower, an inhibitory medicine, a firm hand
//! (-1.5, delay 8; longest episode 45 super-cycles, perfect 89/112),
//! a wrong medicine that ties the uncoupled count while collapsing
//! its longest episode (17 -> 6), a dose ceiling, and no lock. A
//! row is not a map. This instrument surveys the whole (weight,
//! delay) plane for the same pair — 9 weights × 7 delays = 63
//! cells, plus the uncoupled pair — at the same long window, under
//! the heptad's own two-gate rule, so the row's findings can be
//! checked against the ground they were drawn on.
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo period
//! ~39.60), follower Fast Spiking I=4.8 (solo period ~28.01).
//! Seven follower beats span ~196.08 against five driver cycles'
//! ~198.00 — near 7:5, the follower early by ~1.92 ticks per
//! super-cycle. A **super-cycle** is five consecutive driver
//! cycles; it is **perfect** when it holds exactly seven follower
//! beats AND each beat's phase lies within ±0.06 of that beat's
//! median phase for the regime. An **episode** is a maximal run
//! of perfect super-cycles. Ledger counted over 24,000 ticks per
//! cell (first 1,500 discarded; ~112 super-cycles).
//!
//! Predictions declared before the survey ran:
//! - P1: no cell earns TRUE LOCK 7:5 (the heptad's verdict
//!   generalises from a row to the plane).
//! - P2: the plane's best cell (longest episode) is inhibitory —
//!   the sign of the medicine matches the sign of the drift
//!   everywhere, not only at delay 8.
//! - P3: the plane's best cell outlasts the row's champion —
//!   the firm hand's 45 is a row's number, and the plane holds
//!   a longer agreement somewhere the row never looked.
//! - P4: the inhibitory half-plane's mean longest episode is
//!   above the excitatory half-plane's — hurrying an early
//!   clock is worse on average, not only at its worst.
//! - P5: the dose ceiling generalises, mirrored — the -4.0
//!   row's best cell is shorter than the -1.0 row's best cell.
//! - P6: the firm hand re-counted as a cell, (-1.5, delay 8),
//!   reproduces the heptad row's ledger exactly (perfect 89,
//!   longest 45) — the two instruments count the same pair.
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

/// Weights surveyed: the inhibitory doses the heptad's row
/// named, from gentle to crushing, then the excitatory
/// half-plane (the wrong medicine, kept whole).
const WEIGHTS: [f32; 9] = [-4.0, -2.5, -1.5, -1.0, -0.5, 0.5, 1.0, 1.5, 2.5];
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
    println!("widow-heptad-plane headless: the 7:5 plane, surveyed whole — longest episode per cell");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
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
    println!("  grid (perfect count; same layout):");
    print!("    weight\\delay");
    for d in DELAYS { print!(" {d:>5}"); }
    println!();
    for &w in &WEIGHTS {
        print!("    {w:+.1}       ");
        for &d in &DELAYS {
            print!(" {:>5}", cell_at(cells, w, d).perfect);
        }
        println!();
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
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
    for &w in &[-4.0f32, -1.0f32] {
        let row_best = best_of(cells.iter().filter(|c| c.weight == w)).expect("row exists");
        println!("  row {:+.1} best: delay {} — longest {}", w, row_best.delay, row_best.longest_episode);
    }
    for &d in &DELAYS {
        let col_best = best_of(cells.iter().filter(|c| c.delay == d)).expect("column exists");
        println!("  column delay {} best: weight {:+.1} — longest {}", d, col_best.weight, col_best.longest_episode);
    }
    let firm = cell_at(cells, -1.5, 8);
    println!("  the heptad's firm hand as a cell (-1.5, delay 8): perfect {}/{}, longest {} — the row's champion, re-counted on the plane",
        firm.perfect, firm.super_cycles, firm.longest_episode);
    for &(w, d) in &[(-1.0f32, 0usize), (-1.0, 4), (-1.0, 8), (1.5, 8)] {
        let c = cell_at(cells, w, d);
        println!("  heptad row cell ({:+.1}, delay {}): perfect {}/{}, longest {}", w, d, c.perfect, c.super_cycles, c.longest_episode);
    }
    println!("  predictions: P1 no TRUE LOCK — {}; P2 best cell inhibitory — {}; P3 best longest above the row champion's 45 — {}; P4 inhibitory mean longest above excitatory — {}; P5 -4.0 row best below -1.0 row best — {}; P6 firm hand reproduces 89/45 — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if best.weight < 0.0 { "HELD" } else { "CRACKED" },
        if best.longest_episode > 45 { "HELD" } else { "CRACKED" },
        if mean_longest(cells.iter().filter(|c| c.weight < 0.0)) > mean_longest(cells.iter().filter(|c| c.weight > 0.0)) { "HELD" } else { "CRACKED" },
        {
            let hi = best_of(cells.iter().filter(|c| c.weight == -4.0)).map(|c| c.longest_episode).unwrap_or(0);
            let lo = best_of(cells.iter().filter(|c| c.weight == -1.0)).map(|c| c.longest_episode).unwrap_or(0);
            if hi < lo { "HELD" } else { "CRACKED" }
        },
        if firm.perfect == 89 && firm.longest_episode == 45 { "HELD" } else { "CRACKED" });
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
    fn solo_clocks_are_near_seven_to_five_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        let gap = 5.0 * driver.mean_isi - 7.0 * follower.mean_isi;
        assert!(gap > 0.5 && gap < 3.5, "super-cycle earliness {gap}");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let c = run_cell(-1.5, 8);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn firm_hand_reproduces_the_heptad_row() {
        // The plane's (-1.5, delay 8) cell is the heptad's firm
        // hand, counted by the same instrument: it must land on
        // the parent ledger's numbers, not merely near them.
        let c = run_cell(-1.5, 8);
        assert_eq!(c.perfect, 89, "perfect {}", c.perfect);
        assert_eq!(c.longest_episode, 45, "longest {}", c.longest_episode);
    }

    #[test]
    fn no_cell_in_the_plane_claims_a_true_lock() {
        let cells = run_plane();
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.1}, {}) claims a lock with {} perfect of {}",
                c.weight, c.delay, c.perfect, c.super_cycles);
            if c.perfect == c.super_cycles {
                panic!("cell ({:+.1}, {}) is perfect everywhere but unlabelled", c.weight, c.delay);
            }
        }
    }

    #[test]
    fn plane_champion_is_measured_not_assumed() {
        // The plane's champion is (-1.0, delay 10) at longest 58 —
        // not the row's firm hand (-1.5, delay 8) at 45, which
        // stands as the delay-8 column's best and no more. Pin
        // the measured coastline so the record cannot quietly
        // heal itself back into the row's story.
        let cells = run_plane();
        let best = best_of(cells.iter()).expect("plane is non-empty");
        assert_eq!((best.weight, best.delay), (-1.0, 10),
            "best cell ({:+.1}, {})", best.weight, best.delay);
        assert_eq!(best.longest_episode, 58, "best longest {}", best.longest_episode);
        assert_eq!(best.perfect, 92, "best perfect {}", best.perfect);
        let col8 = best_of(cells.iter().filter(|c| c.delay == 8))
            .map(|c| c.longest_episode).unwrap_or(0);
        assert_eq!(col8, 45, "delay-8 column best {col8}");
        assert!(col8 < best.longest_episode);
    }

    #[test]
    fn wrong_medicine_never_beats_doing_nothing_on_duration() {
        // Across the whole excitatory half-plane, no cell's
        // longest episode exceeds the uncoupled pair's 17 — the
        // best of them (+0.5, delay 4) only ties it. Hurrying an
        // early clock buys counts, never duration.
        let cells = run_plane();
        let exc_best = best_of(cells.iter().filter(|c| c.weight > 0.0))
            .map(|c| c.longest_episode).unwrap_or(0);
        let bare = run_cell(0.0, 0);
        assert!(exc_best <= bare.longest_episode,
            "excitatory best {exc_best} vs uncoupled {}", bare.longest_episode);
    }

    #[test]
    fn inhibitory_half_plane_is_better_on_average_and_the_ceiling_holds() {
        let cells = run_plane();
        let inh = mean_longest(cells.iter().filter(|c| c.weight < 0.0));
        let exc = mean_longest(cells.iter().filter(|c| c.weight > 0.0));
        assert!(inh > exc, "inhibitory mean {inh:.1} vs excitatory mean {exc:.1}");
        let crushing = best_of(cells.iter().filter(|c| c.weight == -4.0)).map(|c| c.longest_episode).unwrap_or(0);
        let surveyed = best_of(cells.iter().filter(|c| c.weight == -1.0)).map(|c| c.longest_episode).unwrap_or(0);
        assert!(crushing < surveyed, "-4.0 row best {crushing} vs -1.0 row best {surveyed}");
    }

    #[test]
    fn best_cell_outlasts_uncoupled() {
        let cells = run_plane();
        let best = best_of(cells.iter()).expect("plane is non-empty");
        let bare = run_cell(0.0, 0);
        assert!(best.longest_episode > bare.longest_episode,
            "best {} vs uncoupled {}", best.longest_episode, bare.longest_episode);
    }
}
