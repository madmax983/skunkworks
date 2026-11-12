//! # Widow Heptad Dose 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-tower` centred the peak grid on the late tower
//! (-1.25, delay 12) and found a tooth one hour wide: in delay
//! the tower's column reads 34 / **68** / 32, walled on both
//! sides at single-tick grain. Its README closed with two
//! seeds. This instrument takes the second, verbatim: *a finer
//! grain on the tower itself — eighth-dose steps at its own
//! hour (weights -1.375, -1.125, delay 12): is the tooth also
//! one dose wide, or does the weight axis keep a skirt the
//! delay axis refused?*
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
//! tower grid counted.
//!
//! The survey is one row of the tower's grid read at twice the
//! grain: five weights at the tower's own hour, delay 12 —
//! -1.50, **-1.375**, -1.25, **-1.125**, -1.00. Three are
//! anchors the tower grid already counted (the western quarter
//! neighbour at 37, the centre at 68 with perfect 104, the
//! eastern quarter neighbour at 13). Two are new: the eighth
//! steps either side of the centre, the only addresses at
//! this hour no instrument had priced. The shape lines are
//! the tower grid's, carried unchanged: plateau line 55 (80%
//! of 68, rounded up), isolation line 34 (50% of 68).
//!
//! Predictions declared before the new cells ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives magnification along a second axis.
//! - P2: the three anchors reproduce their ledgers exactly:
//!   longest 37 / 68 / 13 along the row, tower perfect 104.
//! - P3: the tooth is one dose wide too — neither eighth
//!   neighbour reaches the plateau line 55.
//! - P4: the centre stands — no cell in the row outlasts 68.
//! - P5: the skirt the quarter grid found is western at
//!   eighth grain too — (-1.375, 12) stands strictly taller
//!   than (-1.125, 12), as 37 stood over 13 at quarter grain.
//! - P6: the eastern eighth is ordinary ground —
//!   (-1.125, 12) stands at or below the isolation line 34,
//!   as every eastern address at this hour has so far.
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
/// Plateau line: 80% of the tower's longest (68), rounded up.
const PLATEAU_LINE: usize = 55;
/// Isolation line: 50% of the tower's longest (68).
const ISOLATION_LINE: usize = 34;

/// The tower's own hour: every cell in this survey sits here.
const TOWER_DELAY: usize = 12;
/// The dose row at the tower's hour: quarter anchors and the
/// two eighth steps between them and the centre.
const WEIGHTS: [f32; 5] = [-1.5, -1.375, -1.25, -1.125, -1.0];
/// The centre: the late tower itself (weight, longest).
const CENTRE: (f32, usize) = (-1.25, 68);
/// The two unpriced eighth steps: (weight) west and east.
const NEW_WEST: f32 = -1.375;
const NEW_EAST: f32 = -1.125;
/// Anchors the tower grid counted at this hour:
/// (weight, longest).
const ANCHORS: [(f32, usize); 3] = [(-1.5, 37), (-1.25, 68), (-1.0, 13)];

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

fn run_survey() -> Vec<CellLedger> {
    WEIGHTS.iter().map(|&w| run_cell(w, TOWER_DELAY)).collect()
}

fn cell_at(cells: &[CellLedger], weight: f32) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight).expect("surveyed cell")
}

/// The declared shape verdict for the measured dose row.
fn shape_verdict(cells: &[CellLedger]) -> &'static str {
    let west = cell_at(cells, NEW_WEST).longest_episode;
    let east = cell_at(cells, NEW_EAST).longest_episode;
    let at_plateau = [west, east].iter().filter(|&&l| l >= PLATEAU_LINE).count();
    if at_plateau == 2 {
        "PLATEAU IN DOSE"
    } else if at_plateau == 1 {
        "SHOULDER IN DOSE"
    } else if west <= ISOLATION_LINE && east <= ISOLATION_LINE {
        "ISOLATED TOOTH IN DOSE"
    } else {
        "TOOTH IN DOSE"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-dose headless: the tower at eighth-dose grain — its own hour (delay {TOWER_DELAY}) read at weights -1.50 / -1.375 / -1.25 / -1.125 / -1.00: is the tooth one dose wide, or does the weight axis keep a skirt the delay axis refused?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the dose row at the tower's hour (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}; * = anchor, + = new eighth step):");
    for &w in &WEIGHTS {
        let c = cell_at(cells, w);
        let mark = if w == CENTRE.0 {
            " (centre — the tower)"
        } else if w == NEW_WEST || w == NEW_EAST {
            " +"
        } else {
            " *"
        };
        println!("    row ({w:+.3}, delay {}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.delay,
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, l)| cell_at(cells, w).longest_episode == l)
        && cell_at(cells, CENTRE.0).perfect == 104;
    let shape = shape_verdict(cells);
    let centre_stands = cells.iter().all(|c| c.longest_episode <= CENTRE.1);
    let west = cell_at(cells, NEW_WEST).longest_episode;
    let east = cell_at(cells, NEW_EAST).longest_episode;
    let row_mean = cells.iter().map(|c| c.longest_episode as f64).sum::<f64>() / cells.len() as f64;
    println!("  shape verdict: {shape} (row mean longest {row_mean:.1}; west eighth {west}, east eighth {east})");
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce longest 37 / 68 / 13 with tower perfect 104 — {}; P3 tooth one dose wide (neither eighth at plateau {PLATEAU_LINE}; west {west}, east {east}) — {}; P4 centre stands (no cell above {}) — {}; P5 skirt is western (west {west} strictly above east {east}) — {}; P6 east eighth ordinary (at or below isolation {ISOLATION_LINE}; measured {east}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        if west < PLATEAU_LINE && east < PLATEAU_LINE { "HELD" } else { "CRACKED" },
        CENTRE.1,
        if centre_stands { "HELD" } else { "CRACKED" },
        if west > east { "HELD" } else { "CRACKED" },
        if east <= ISOLATION_LINE { "HELD" } else { "CRACKED" });
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
    fn solo_clocks_are_near_seven_to_five_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        let gap = 5.0 * driver.mean_isi - 7.0 * follower.mean_isi;
        assert!(gap > 0.5 && gap < 3.5, "super-cycle earliness {gap}");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let c = run_cell(-1.375, TOWER_DELAY);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn anchors_reproduce_the_tower_grid_row() {
        for &(w, longest) in &ANCHORS {
            let c = run_cell(w, TOWER_DELAY);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {TOWER_DELAY}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CENTRE.0, TOWER_DELAY).perfect, 104);
    }

    #[test]
    fn survey_is_the_declared_dose_row() {
        assert_eq!(WEIGHTS, [-1.5, -1.375, -1.25, -1.125, -1.0]);
        assert_eq!(TOWER_DELAY, 12);
        assert_eq!(CENTRE, (-1.25, 68));
        // The centre is the row's middle cell; the new cells
        // are the eighth steps either side of it, halfway to
        // the quarter anchors.
        assert_eq!(WEIGHTS[2], CENTRE.0);
        assert_eq!(NEW_WEST, CENTRE.0 - 0.125);
        assert_eq!(NEW_EAST, CENTRE.0 + 0.125);
        assert_eq!(ANCHORS.len(), 3);
        for &w in &WEIGHTS {
            let known = ANCHORS.iter().any(|&(aw, _)| aw == w);
            assert!(known || w == NEW_WEST || w == NEW_EAST, "cell {w} neither anchor nor new eighth");
        }
        // The shape lines are the tower grid's fractions of
        // the centre's height: 80% rounded up, 50% exact.
        assert_eq!(PLATEAU_LINE, 55);
        assert_eq!(ISOLATION_LINE, 34);
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured row, pinned so the record cannot heal:
        // at delay 12, weights -1.50 / -1.375 / -1.25 / -1.125
        // / -1.00 read longest 37 / 59 / 68 / 51 / 13. P1, P2,
        // P4, P5 held; P3 and P6 cracked — the west eighth
        // (-1.375) reaches 59, above the plateau line 55, so
        // the tooth is NOT one dose wide: in dose it keeps a
        // western shoulder, and the east eighth (-1.125) at 51
        // stands far above the isolation line 34, a high shelf
        // that falls off a cliff to 13 at the quarter anchor.
        // The shape verdict is SHOULDER IN DOSE.
        let cells = run_survey();
        let longest = |w: f32| cell_at(&cells, w).longest_episode;
        assert_eq!(longest(-1.5), 37);
        assert_eq!(longest(-1.375), 59);
        assert_eq!(longest(-1.25), 68);
        assert_eq!(longest(-1.125), 51);
        assert_eq!(longest(-1.0), 13);
        assert_eq!(cell_at(&cells, -1.25).perfect, 104);
        assert_eq!(cell_at(&cells, -1.375).perfect, 93);
        assert_eq!(cell_at(&cells, -1.125).perfect, 91);
        assert_eq!(shape_verdict(&cells), "SHOULDER IN DOSE");
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 5);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.3}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
