//! # Widow Heptad Diagonal 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-peak` measured the fine grid around the 7:5
//! champion (-1.0, delay 10) and found a tooth, not a plateau —
//! and one surprise: the grid's second tower was its diagonal
//! (-0.75, delay 9) at longest 51, above the plateau line, at the
//! *lighter* dose and the *earlier* hour — the same lean the 5:3
//! crown showed in `widow-pentad-peak`. Its README named this
//! instrument outright: *a fine grid centred on (-0.75, delay 9)
//! itself — is the diagonal a second tooth, or the crown's
//! shoulder seen from the wrong centre?* The 5:3 row answered
//! that exact question in `widow-pentad-diagonal`: the diagonal
//! was the crown's shoulder, and the tall cells traced a ridge.
//! This instrument recentres the fine grid on the 7:5 diagonal
//! and counts again, under the same two-gate rule, so the two
//! ratios' answers can be compared cell for cell.
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo
//! period ~39.60), follower Fast Spiking I=4.8 (solo period
//! ~28.01). Seven follower beats span ~196.08 against five
//! driver cycles' ~198.00 — near 7:5, the follower early by
//! ~1.92 ticks per super-cycle. A **super-cycle** is five
//! consecutive driver cycles; it is **perfect** when it holds
//! exactly seven follower beats AND each beat's phase lies
//! within ±0.06 of that beat's median phase for the regime.
//! An **episode** is a maximal run of perfect super-cycles.
//! Ledger counted over 24,000 ticks per cell (first 1,500
//! discarded; ~112 super-cycles). The champion (-1.0, delay 10)
//! stands in this grid as a diagonal neighbour of the centre.
//!
//! Shape definitions, declared before the survey ran (centre
//! longest C = 51): plateau line 41 (80% of C); isolation line
//! 25 (50% of C);
//! - SHOULDER OF THE CROWN: some neighbour's longest episode
//!   exceeds the centre's.
//! - PLATEAU: at least 2 orthogonal neighbours reach 41 and
//!   none exceeds the centre.
//! - ISOLATED TOOTH: no orthogonal reaches 41, every neighbour
//!   at or below 25, none exceeds the centre.
//! - TOOTH (with skirts): no orthogonal reaches 41, none
//!   exceeds the centre, but some neighbour stands above 25.
//! - SHOULDER: anything else.
//!
//! Predictions declared before the survey ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives recentring, as it survived magnification.
//! - P2: the centre reproduces the peak grid's diagonal exactly
//!   (perfect 86, longest 51) — the two instruments count one
//!   pair.
//! - P3: the champion diagonal — (-1.0, delay 10), now a
//!   diagonal neighbour of the centre — reproduces its ledger
//!   (perfect 92, longest 58) and thereby DETHRONES the centre:
//!   the shape is SHOULDER OF THE CROWN, as the 5:3 diagonal
//!   was. The peak grid's second tower was never a second
//!   tooth; it was a point on the crown's slope.
//! - P4: the two orthogonals already measured by the peak grid
//!   keep their numbers: (-1.0, delay 9) at 39 and
//!   (-0.75, delay 10) at 20, both below the plateau line 41.
//! - P5: the grid's mean longest episode beats the plane's
//!   inhibitory mean (23.4) — the crown's ground is genuinely
//!   elevated, whichever cell stands at the centre.
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
/// The centre's longest episode in the peak grid (its diagonal).
const CENTRE_LONGEST: usize = 51;
/// The centre's perfect count in the peak grid.
const CENTRE_PERFECT: usize = 86;
/// The champion's address — a diagonal neighbour in this grid.
const CHAMPION_WEIGHT: f32 = -1.0;
const CHAMPION_DELAY: usize = 10;
/// The champion's longest episode (widow-heptad-plane / -peak).
const CHAMPION_LONGEST: usize = 58;
/// The champion's perfect count in both prior instruments.
const CHAMPION_PERFECT: usize = 92;
/// Plateau line: 80% of the centre's longest, rounded up.
const PLATEAU_LINE: usize = 41;
/// Isolation line: 50% of the centre's longest, rounded down.
const ISOLATION_LINE: usize = 25;
/// The plane's inhibitory mean longest episode.
const PLANE_INHIBITORY_MEAN: f64 = 23.4;

/// Fine weights around the diagonal: quarter steps.
const WEIGHTS: [f32; 3] = [-1.0, -0.75, -0.5];
/// Fine delays around the diagonal: single ticks.
const DELAYS: [usize; 3] = [8, 9, 10];
/// The diagonal's own address (the peak grid's second tower).
const CENTRE_WEIGHT: f32 = -0.75;
const CENTRE_DELAY: usize = 9;

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

fn run_grid() -> Vec<CellLedger> {
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

fn orthogonal_neighbours(cells: &[CellLedger]) -> Vec<&CellLedger> {
    cells.iter().filter(|c| {
        (c.weight == -1.0 && c.delay == CENTRE_DELAY)
            || (c.weight == -0.5 && c.delay == CENTRE_DELAY)
            || (c.weight == CENTRE_WEIGHT && c.delay == 8)
            || (c.weight == CENTRE_WEIGHT && c.delay == 10)
    }).collect()
}

fn diagonal_neighbours(cells: &[CellLedger]) -> Vec<&CellLedger> {
    cells.iter().filter(|c| {
        (c.weight == -1.0 || c.weight == -0.5) && (c.delay == 8 || c.delay == 10)
    }).collect()
}

fn all_neighbours(cells: &[CellLedger]) -> Vec<&CellLedger> {
    cells.iter().filter(|c| !(c.weight == CENTRE_WEIGHT && c.delay == CENTRE_DELAY)).collect()
}

fn shape_of(cells: &[CellLedger]) -> &'static str {
    let centre = cell_at(cells, CENTRE_WEIGHT, CENTRE_DELAY);
    let orth = orthogonal_neighbours(cells);
    let neighbours = all_neighbours(cells);
    let at_plateau = orth.iter().filter(|c| c.longest_episode >= PLATEAU_LINE).count();
    if neighbours.iter().any(|c| c.longest_episode > centre.longest_episode) {
        "SHOULDER OF THE CROWN"
    } else if at_plateau >= 2 {
        "PLATEAU"
    } else if at_plateau == 0
        && neighbours.iter().all(|c| c.longest_episode <= ISOLATION_LINE)
    {
        "ISOLATED TOOTH"
    } else if at_plateau == 0 {
        "TOOTH (with skirts)"
    } else {
        "SHOULDER"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-diagonal headless: a fine grid centred on the peak's diagonal (-0.75, delay 9) — second tooth, or the crown's shoulder?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  fine grid (longest episode; columns are delays {:?}):", DELAYS);
    print!("    weight\\delay");
    for d in DELAYS { print!(" {d:>5}"); }
    println!();
    for &w in &WEIGHTS {
        print!("    {w:+.2}      ");
        for &d in &DELAYS {
            print!(" {:>5}", cell_at(cells, w, d).longest_episode);
        }
        println!();
    }
    println!("  fine grid (perfect super-cycles of 112):");
    for &w in &WEIGHTS {
        print!("    {w:+.2}      ");
        for &d in &DELAYS {
            print!(" {:>5}", cell_at(cells, w, d).perfect);
        }
        println!();
    }
    let centre = cell_at(cells, CENTRE_WEIGHT, CENTRE_DELAY);
    println!("  centre (the diagonal, -0.75, delay 9): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
        centre.perfect, centre.super_cycles, centre.count_slips, centre.phase_slips, centre.episodes, centre.longest_episode, centre.mean_episode, centre.verdict);
    for n in orthogonal_neighbours(cells) {
        println!("  orthogonal neighbour ({:+.2}, delay {}): perfect {}/{}, longest {}, mean episode {:.1} — {}",
            n.weight, n.delay, n.perfect, n.super_cycles, n.longest_episode, n.mean_episode, n.verdict);
    }
    for n in diagonal_neighbours(cells) {
        println!("  diagonal neighbour ({:+.2}, delay {}): perfect {}/{}, longest {}, mean episode {:.1} — {}",
            n.weight, n.delay, n.perfect, n.super_cycles, n.longest_episode, n.mean_episode, n.verdict);
    }
    let champion = cell_at(cells, CHAMPION_WEIGHT, CHAMPION_DELAY);
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let best = best_of(cells.iter()).expect("fine grid is non-empty");
    let grid_mean = mean_longest(cells.iter());
    let orth_at_plateau = orthogonal_neighbours(cells).iter().filter(|c| c.longest_episode >= PLATEAU_LINE).count();
    let west = cell_at(cells, -1.0, 9);
    let south = cell_at(cells, -0.75, 10);
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  best grid cell: weight {:+.2}, delay {} — longest {}", best.weight, best.delay, best.longest_episode);
    println!("  grid mean longest episode: {:.1} across {} cells (plane inhibitory mean {:.1})", grid_mean, cells.len(), PLANE_INHIBITORY_MEAN);
    println!("  orthogonal neighbours at plateau line (>= {}): {} of 4", PLATEAU_LINE, orth_at_plateau);
    println!("  SHAPE: {}", shape_of(cells));
    println!("  predictions: P1 no TRUE LOCK — {}; P2 centre reproduces peak diagonal (perfect {}, longest {}) — {}; P3 champion diagonal reproduces (perfect {}, longest {}) and dethrones centre, shape SHOULDER OF THE CROWN — {}; P4 measured orthogonals hold ((-1.0, 9) = 39, (-0.75, 10) = 20) — {}; P5 grid mean > plane inhibitory mean {:.1} — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        CENTRE_PERFECT, CENTRE_LONGEST,
        if centre.perfect == CENTRE_PERFECT && centre.longest_episode == CENTRE_LONGEST { "HELD" } else { "CRACKED" },
        CHAMPION_PERFECT, CHAMPION_LONGEST,
        if champion.perfect == CHAMPION_PERFECT && champion.longest_episode == CHAMPION_LONGEST && shape_of(cells) == "SHOULDER OF THE CROWN" { "HELD" } else { "CRACKED" },
        if west.longest_episode == 39 && south.longest_episode == 20 { "HELD" } else { "CRACKED" },
        PLANE_INHIBITORY_MEAN,
        if grid_mean > PLANE_INHIBITORY_MEAN { "HELD" } else { "CRACKED" });
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let bare = run_cell(0.0, 0);
    let cells = run_grid();
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
        let c = run_cell(CENTRE_WEIGHT, CENTRE_DELAY);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn centre_reproduces_the_peak_diagonal() {
        let c = run_cell(CENTRE_WEIGHT, CENTRE_DELAY);
        assert_eq!(c.perfect, CENTRE_PERFECT, "perfect {}", c.perfect);
        assert_eq!(c.longest_episode, CENTRE_LONGEST, "longest {}", c.longest_episode);
    }

    #[test]
    fn champion_diagonal_reproduces_and_dethrones() {
        let cells = run_grid();
        let champion = cell_at(&cells, CHAMPION_WEIGHT, CHAMPION_DELAY);
        assert_eq!(champion.perfect, CHAMPION_PERFECT, "perfect {}", champion.perfect);
        assert_eq!(champion.longest_episode, CHAMPION_LONGEST, "longest {}", champion.longest_episode);
        assert_eq!(shape_of(&cells), "SHOULDER OF THE CROWN");
    }

    #[test]
    fn no_fine_cell_claims_a_true_lock() {
        let cells = run_grid();
        assert_eq!(cells.len(), 9);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.2}, {}) claims a lock", c.weight, c.delay);
        }
    }

    #[test]
    fn grid_is_centred_on_the_diagonal() {
        assert_eq!((CENTRE_WEIGHT, CENTRE_DELAY), (-0.75, 9));
        assert_eq!((CHAMPION_WEIGHT, CHAMPION_DELAY), (-1.0, 10));
        // Five addresses in this grid were never surveyed at fine
        // grain before: the whole -0.50 row and the delay-8 column
        // outside the centre row's west cell.
        for (w, d) in [(-0.5f32, 8usize), (-0.5, 9), (-0.5, 10), (-1.0, 8), (-0.75, 8)] {
            assert!(WEIGHTS.contains(&w) && DELAYS.contains(&d), "cell ({w}, {d}) missing");
        }
    }

    #[test]
    fn measured_orthogonals_hold() {
        let cells = run_grid();
        assert_eq!(cell_at(&cells, -1.0, 9).longest_episode, 39);
        assert_eq!(cell_at(&cells, -0.75, 10).longest_episode, 20);
    }

    #[test]
    fn fine_grid_is_pinned_as_measured() {
        // The measured grid, pinned so the record cannot heal:
        // centre 51; the champion diagonal (-1.0, 10) at 58
        // dethrones it (SHOULDER OF THE CROWN); the measured
        // orthogonals 39 / 20 hold; the new orthogonals stand at
        // 34 (delay 8) and 33 (-0.50 row's hour) — no orthogonal
        // reaches the plateau line 41, and the lightest row
        // (21 / 33 / 32) holds no second tower: the lean ends
        // at the crown.
        let cells = run_grid();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-1.0, 8), 22);
        assert_eq!(longest(-1.0, 9), 39);
        assert_eq!(longest(-1.0, 10), 58);
        assert_eq!(longest(-0.75, 8), 34);
        assert_eq!(longest(-0.75, 9), 51);
        assert_eq!(longest(-0.75, 10), 20);
        assert_eq!(longest(-0.5, 8), 21);
        assert_eq!(longest(-0.5, 9), 33);
        assert_eq!(longest(-0.5, 10), 32);
        assert_eq!(shape_of(&cells), "SHOULDER OF THE CROWN");
        assert_eq!(mean_longest(cells.iter()), 310.0 / 9.0);
    }

    #[test]
    fn shape_classifier_uses_the_declared_lines() {
        let cells = run_grid();
        let orth = orthogonal_neighbours(&cells);
        assert_eq!(orth.len(), 4);
        assert_eq!(diagonal_neighbours(&cells).len(), 4);
        assert_eq!(all_neighbours(&cells).len(), 8);
        let centre = cell_at(&cells, CENTRE_WEIGHT, CENTRE_DELAY);
        let dethroned = all_neighbours(&cells).iter().any(|c| c.longest_episode > centre.longest_episode);
        if dethroned {
            assert_eq!(shape_of(&cells), "SHOULDER OF THE CROWN");
        }
    }
}
