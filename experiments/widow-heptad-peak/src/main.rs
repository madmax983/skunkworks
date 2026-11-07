//! # Widow Heptad Peak 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-plane` surveyed the whole (weight, delay) plane
//! for the near-7:5 pair and crowned a champion: (-1.0, delay 10),
//! longest episode 58 super-cycles. But its grid was coarse —
//! weights a full half-step or more apart around the crown,
//! delays two ticks apart — so the champion's shape is unknown:
//! a single tall cell on a coarse grid can be a plateau seen
//! edge-on, a broad shoulder, or one isolated tooth. This
//! instrument returns to the crown with a finer rule: weights
//! -1.25 / -1.00 / -0.75 (quarter steps) × delays 9 / 10 / 11
//! (single ticks) — 9 cells, the champion at the centre —
//! counted at the same long window under the same two-gate
//! rule, so the shapes are comparable. It takes up the plane's
//! own closing seed verbatim: a fine grid around the champion
//! (-1.0, delay 10) — is 58 a tooth or a plateau?
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
//! discarded; ~112 super-cycles).
//!
//! Shape definitions, declared before the fine survey ran
//! (champion longest C = 58):
//! - PLATEAU: at least 2 of the champion's 4 orthogonal
//!   neighbours reach 80% of C (longest >= 47).
//! - TOOTH: no orthogonal neighbour reaches 80% of C.
//! - ISOLATED TOOTH: TOOTH and every neighbour (orthogonal
//!   and diagonal) falls at or below 50% of C (longest <= 29).
//! - SHOULDER: anything else.
//!
//! Predictions declared before the fine survey ran:
//! - P1: no fine cell earns TRUE LOCK 7:5.
//! - P2: the centre cell reproduces the plane exactly (perfect
//!   92/112, longest 58) — the two instruments count one pair.
//! - P3: the crown is a TOOTH, not a plateau — no orthogonal
//!   neighbour reaches 47. (The coarse neighbours were 32, 32,
//!   22, 13; a plateau would have shown through that grid.)
//! - P4: the fine grid's mean longest episode beats the plane's
//!   inhibitory mean (23.4) — the crown's ground is genuinely
//!   elevated, not one cell's accident.
//! - P5: the centre remains the fine grid's best cell — no
//!   neighbour outlasts 58.
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
/// The plane champion's longest episode (widow-heptad-plane).
const CHAMPION_LONGEST: usize = 58;
/// Plateau line: 80% of the champion's longest, rounded up.
const PLATEAU_LINE: usize = 47;
/// Isolation line: 50% of the champion's longest.
const ISOLATION_LINE: usize = 29;
/// The plane's inhibitory mean longest episode.
const PLANE_INHIBITORY_MEAN: f64 = 23.4;

/// Fine weights around the crown: quarter steps.
const WEIGHTS: [f32; 3] = [-1.25, -1.0, -0.75];
/// Fine delays around the crown: single ticks.
const DELAYS: [usize; 3] = [9, 10, 11];
/// The champion's own address.
const CENTRE_WEIGHT: f32 = -1.0;
const CENTRE_DELAY: usize = 10;

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
        (c.weight == -1.25 && c.delay == CENTRE_DELAY)
            || (c.weight == -0.75 && c.delay == CENTRE_DELAY)
            || (c.weight == CENTRE_WEIGHT && c.delay == 9)
            || (c.weight == CENTRE_WEIGHT && c.delay == 11)
    }).collect()
}

fn diagonal_neighbours(cells: &[CellLedger]) -> Vec<&CellLedger> {
    cells.iter().filter(|c| {
        (c.weight == -1.25 || c.weight == -0.75) && (c.delay == 9 || c.delay == 11)
    }).collect()
}

fn shape_of(cells: &[CellLedger]) -> &'static str {
    let orth = orthogonal_neighbours(cells);
    let diag = diagonal_neighbours(cells);
    let at_plateau = orth.iter().filter(|c| c.longest_episode >= PLATEAU_LINE).count();
    if at_plateau >= 2 {
        "PLATEAU"
    } else if at_plateau == 0
        && orth.iter().chain(diag.iter()).all(|c| c.longest_episode <= ISOLATION_LINE)
    {
        "ISOLATED TOOTH"
    } else if at_plateau == 0 {
        "TOOTH (with skirts)"
    } else {
        "SHOULDER"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-peak headless: the 7:5 crown at fine grain — is (-1.0, delay 10) a tooth or a plateau?");
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
    println!("  centre (champion, -1.00, delay 10): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
        centre.perfect, centre.super_cycles, centre.count_slips, centre.phase_slips, centre.episodes, centre.longest_episode, centre.mean_episode, centre.verdict);
    for n in orthogonal_neighbours(cells) {
        println!("  orthogonal neighbour ({:+.2}, delay {}): perfect {}/{}, longest {}, mean episode {:.1} — {}",
            n.weight, n.delay, n.perfect, n.super_cycles, n.longest_episode, n.mean_episode, n.verdict);
    }
    for n in diagonal_neighbours(cells) {
        println!("  diagonal neighbour ({:+.2}, delay {}): perfect {}/{}, longest {}, mean episode {:.1} — {}",
            n.weight, n.delay, n.perfect, n.super_cycles, n.longest_episode, n.mean_episode, n.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let best = best_of(cells.iter()).expect("fine grid is non-empty");
    let fine_mean = mean_longest(cells.iter());
    let orth_at_plateau = orthogonal_neighbours(cells).iter().filter(|c| c.longest_episode >= PLATEAU_LINE).count();
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  best fine cell: weight {:+.2}, delay {} — longest {}", best.weight, best.delay, best.longest_episode);
    println!("  fine-grid mean longest episode: {:.1} across {} cells (plane inhibitory mean {:.1})", fine_mean, cells.len(), PLANE_INHIBITORY_MEAN);
    println!("  orthogonal neighbours at plateau line (>= {}): {} of 4", PLATEAU_LINE, orth_at_plateau);
    println!("  SHAPE: {}", shape_of(cells));
    println!("  predictions: P1 no TRUE LOCK — {}; P2 centre reproduces plane (perfect 92, longest 58) — {}; P3 TOOTH not plateau (0 orthogonal >= {}) — {}; P4 fine mean > plane inhibitory mean {:.1} — {}; P5 centre remains best — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if centre.perfect == 92 && centre.longest_episode == CHAMPION_LONGEST { "HELD" } else { "CRACKED" },
        PLATEAU_LINE,
        if orth_at_plateau == 0 { "HELD" } else { "CRACKED" },
        PLANE_INHIBITORY_MEAN,
        if fine_mean > PLANE_INHIBITORY_MEAN { "HELD" } else { "CRACKED" },
        if best.weight == CENTRE_WEIGHT && best.delay == CENTRE_DELAY { "HELD" } else { "CRACKED" });
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
    fn centre_reproduces_the_plane_champion() {
        let c = run_cell(CENTRE_WEIGHT, CENTRE_DELAY);
        assert_eq!(c.perfect, 92, "perfect {}", c.perfect);
        assert_eq!(c.longest_episode, 58, "longest {}", c.longest_episode);
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
    fn fine_grid_is_genuinely_finer_than_the_plane() {
        for &w in &WEIGHTS {
            for &d in &DELAYS {
                if (w, d) == (CENTRE_WEIGHT, CENTRE_DELAY) { continue; }
                let on_plane = [-4.0f32, -2.5, -1.5, -1.0, -0.5, 0.5, 1.0, 1.5, 2.5].contains(&w)
                    && [0usize, 2, 4, 6, 8, 10, 12].contains(&d);
                assert!(!on_plane, "fine cell ({w}, {d}) was already on the plane grid");
            }
        }
    }

    #[test]
    fn fine_grid_is_pinned_as_measured() {
        // The measured crown, pinned so the record cannot heal:
        // centre 58; orthogonal neighbours 39 / 39 / 28 / 20
        // (none at the plateau line 47 — a tooth); the diagonal
        // (-0.75, delay 9) at 51 is the grid's second tower,
        // above the plateau line, which is why the shape is a
        // tooth with skirts and not an isolated tooth.
        let cells = run_grid();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-1.25, 9), 36);
        assert_eq!(longest(-1.25, 10), 39);
        assert_eq!(longest(-1.25, 11), 34);
        assert_eq!(longest(-1.0, 9), 39);
        assert_eq!(longest(-1.0, 10), 58);
        assert_eq!(longest(-1.0, 11), 28);
        assert_eq!(longest(-0.75, 9), 51);
        assert_eq!(longest(-0.75, 10), 20);
        assert_eq!(longest(-0.75, 11), 32);
        assert_eq!(shape_of(&cells), "TOOTH (with skirts)");
        assert_eq!(mean_longest(cells.iter()), 337.0 / 9.0);
    }

    #[test]
    fn shape_classifier_uses_the_declared_lines() {
        let cells = run_grid();
        let orth = orthogonal_neighbours(&cells);
        assert_eq!(orth.len(), 4);
        assert_eq!(diagonal_neighbours(&cells).len(), 4);
        let at_plateau = orth.iter().filter(|c| c.longest_episode >= PLATEAU_LINE).count();
        let shape = shape_of(&cells);
        if at_plateau >= 2 {
            assert_eq!(shape, "PLATEAU");
        } else if at_plateau == 1 {
            assert_eq!(shape, "SHOULDER");
        }
    }
}
