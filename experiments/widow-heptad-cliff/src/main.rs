//! # Widow Heptad Cliff 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model), via `widow-heptad-dose`.
//!
//! `widow-heptad-dose` read the tower's row at eighth-dose grain
//! and found a shoulder, not a tooth: at the tower's own hour
//! (delay 12) the row reads 37 / 59 / **68** / 51 / 13 along
//! weights -1.50 / -1.375 / -1.25 / -1.125 / -1.00. West, the
//! ground climbs; east, it shelves high at (-1.125, 51) and then
//! falls 38 in a single eighth of a dose to the low anchor at
//! (-1.00, 13). Dose's README closed with this seed, verbatim:
//! *the eastern cliff, priced at sixteenth grain: between
//! -1.125 (51) and -1.00 (13) the row falls 38 in one eighth —
//! at -1.0625, does the shelf hold, halve, or fall whole, and
//! is the cliff's face one step wide at any grain?* This
//! instrument stands on that one unpriced address.
//!
//! The pair is unchanged, and so is the counting: driver
//! Regular Spiking I=12 (solo period ~39.60), follower Fast
//! Spiking I=4.8 (solo ~28.01). Seven follower beats span
//! ~196.08 against five driver cycles' ~198.00 — near 7:5, the
//! follower **early by ~1.92 ticks per super-cycle**. A
//! **super-cycle** is five consecutive driver cycles; it is
//! **perfect** when it holds exactly seven follower beats AND
//! each beat's phase lies within ±0.06 of that beat's median
//! phase for the regime (the two-gate rule, unchanged). An
//! **episode** is a maximal run of consecutive perfect
//! super-cycles. Each cell counted over 24,000 ticks (first
//! 1,500 discarded; 112 super-cycles), exactly as dose counted.
//!
//! The survey is the dose row entire — five anchors dose
//! already counted — plus the one sixteenth step between the
//! shelf and the low anchor: **-1.0625 at delay 12**, the only
//! address inside the cliff's eighth no instrument had priced.
//! The shape lines are dose's, carried unchanged: plateau line
//! 55 (80% of 68, rounded up), isolation line 34 (50% of 68).
//! Two further lines price the face, declared before the run:
//! the **shelf-hold line 45** (within 6 of the shelf's 51 —
//! the shelf holds) and the **fall line 20** (within 7 of the
//! low anchor's 13 — the fall is whole). Between them lies the
//! slope band 21..=44: a reading there means the face is a
//! slope at sixteenth grain, not a step.
//!
//! Predictions declared before the new cell ran:
//! - P1: no cell earns TRUE LOCK 7:5. Magnifying a third axis
//!   segment does not teach the pair to hold.
//! - P2: the five anchors reproduce the dose row exactly:
//!   longest 37 / 59 / 68 / 51 / 13, centre perfect 104,
//!   shelf (-1.125) perfect 91.
//! - P3: the face is a STEP, not a slope — the sixteenth
//!   reads at or above the shelf-hold line 45, or at or below
//!   the fall line 20, never in the slope band 21..=44. The
//!   cliff that was one eighth wide is one step wide at
//!   sixteenth grain too; its face stands on one side of
//!   -1.0625 or the other, not astride it.
//! - P4: the centre stands — no cell in the row outlasts 68.
//! - P5: the descent in counts continues — the new cell's
//!   perfect count lies between the low anchor's 74 and the
//!   shelf's 91, inclusive. The counts step down the cliff in
//!   the same order the runs do.
//! - P6: the western shoulder is intact — (-1.375, 12) at 59
//!   remains the row's second-tallest cell, only the centre
//!   above it. Magnifying the east does not move the west.
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
/// The dose row at the tower's hour, entire, plus the one
/// sixteenth step inside its eastern cliff.
const WEIGHTS: [f32; 6] = [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.0];
/// The centre: the late tower itself (weight, longest).
const CENTRE: (f32, usize) = (-1.25, 68);
/// The shelf: the eastern eighth, high ground above the cliff.
const SHELF: (f32, usize) = (-1.125, 51);
/// The low anchor: the eastern quarter, the cliff's foot.
const LOW: (f32, usize) = (-1.0, 13);
/// The one unpriced sixteenth step, midway between shelf
/// and foot: (weight).
const NEW_MID: f32 = -1.0625;
/// Shelf-hold line: within 6 of the shelf's 51, the shelf
/// holds at sixteenth grain.
const SHELF_HOLD_LINE: usize = 45;
/// Fall line: within 7 of the foot's 13, the fall is whole
/// at sixteenth grain. Between FALL_LINE and SHELF_HOLD_LINE
/// lies the slope band.
const FALL_LINE: usize = 20;
/// The shelf's perfect count as dose counted it.
const SHELF_PERFECT: usize = 91;
/// The foot's perfect count as dose counted it.
const LOW_PERFECT: usize = 74;
/// Anchors dose counted at this hour: (weight, longest) —
/// the whole row, west shoulder to the cliff's foot.
const ANCHORS: [(f32, usize); 5] = [(-1.5, 37), (-1.375, 59), (-1.25, 68), (-1.125, 51), (-1.0, 13)];

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

/// The declared shape verdict for the cliff's face at
/// sixteenth grain, from the new cell's longest alone.
fn shape_verdict(cells: &[CellLedger]) -> &'static str {
    let mid = cell_at(cells, NEW_MID).longest_episode;
    if mid >= SHELF_HOLD_LINE {
        "SHELF HOLDS — FACE EAST OF THE SIXTEENTH"
    } else if mid <= FALL_LINE {
        "FACE WEST — THE FALL IS WHOLE AT THE SIXTEENTH"
    } else {
        "FACE IS A SLOPE AT SIXTEENTH GRAIN"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-cliff headless: the eastern cliff at sixteenth grain — the dose row entire at the tower's hour (delay {TOWER_DELAY}), weights -1.50 / -1.375 / -1.25 / -1.125 / -1.0625 / -1.00: does the shelf hold, halve, or fall whole at the one unpriced sixteenth, and is the cliff's face one step wide at any grain?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the row at the tower's hour (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}; shelf-hold line {SHELF_HOLD_LINE}, fall line {FALL_LINE}; * = anchor, + = new sixteenth step):");
    for &w in &WEIGHTS {
        let c = cell_at(cells, w);
        let mark = if w == CENTRE.0 {
            " (centre — the tower)"
        } else if w == NEW_MID {
            " + (the sixteenth — inside the cliff)"
        } else if w == SHELF.0 {
            " * (the shelf)"
        } else if w == LOW.0 {
            " * (the foot)"
        } else {
            " *"
        };
        println!("    row ({w:+.4}, delay {}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.delay,
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, l)| cell_at(cells, w).longest_episode == l)
        && cell_at(cells, CENTRE.0).perfect == 104
        && cell_at(cells, SHELF.0).perfect == 91;
    let shape = shape_verdict(cells);
    let centre_stands = cells.iter().all(|c| c.longest_episode <= CENTRE.1);
    let mid = cell_at(cells, NEW_MID);
    let mid_perfect = mid.perfect;
    let counts_descend = (LOW_PERFECT..=SHELF_PERFECT).contains(&mid_perfect);
    let west = cell_at(cells, -1.375).longest_episode;
    let second_tallest = {
        let mut ls: Vec<usize> = cells.iter().map(|c| c.longest_episode).collect();
        ls.sort_unstable();
        ls[ls.len() - 2]
    };
    let shoulder_intact = west == 59 && second_tallest == 59;
    let row_mean = cells.iter().map(|c| c.longest_episode as f64).sum::<f64>() / cells.len() as f64;
    println!("  shape verdict: {shape} (row mean longest {row_mean:.1}; the sixteenth reads longest {}, perfect {})", mid.longest_episode, mid_perfect);
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce dose row 37 / 59 / 68 / 51 / 13 with centre perfect 104 and shelf perfect 91 — {}; P3 face is a step (sixteenth at/above shelf-hold {SHELF_HOLD_LINE} or at/below fall {FALL_LINE}; measured {}) — {}; P4 centre stands (no cell above {}) — {}; P5 counts descend (sixteenth perfect between foot {LOW_PERFECT} and shelf {SHELF_PERFECT}; measured {mid_perfect}) — {}; P6 western shoulder intact (west eighth 59 and second-tallest; measured {west} / {second_tallest}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        mid.longest_episode,
        if mid.longest_episode >= SHELF_HOLD_LINE || mid.longest_episode <= FALL_LINE { "HELD" } else { "CRACKED" },
        CENTRE.1,
        if centre_stands { "HELD" } else { "CRACKED" },
        if counts_descend { "HELD" } else { "CRACKED" },
        if shoulder_intact { "HELD" } else { "CRACKED" });
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
        let c = run_cell(NEW_MID, TOWER_DELAY);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn anchors_reproduce_the_dose_row() {
        for &(w, longest) in &ANCHORS {
            let c = run_cell(w, TOWER_DELAY);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {TOWER_DELAY}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CENTRE.0, TOWER_DELAY).perfect, 104);
        assert_eq!(run_cell(SHELF.0, TOWER_DELAY).perfect, SHELF_PERFECT);
        assert_eq!(run_cell(LOW.0, TOWER_DELAY).perfect, LOW_PERFECT);
    }

    #[test]
    fn survey_is_the_declared_row() {
        assert_eq!(WEIGHTS, [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.0]);
        assert_eq!(TOWER_DELAY, 12);
        assert_eq!(CENTRE, (-1.25, 68));
        assert_eq!(SHELF, (-1.125, 51));
        assert_eq!(LOW, (-1.0, 13));
        // The new cell is the exact midpoint of the cliff's
        // eighth: halfway between shelf and foot.
        assert_eq!(NEW_MID, (SHELF.0 + LOW.0) / 2.0);
        assert_eq!(ANCHORS.len(), 5);
        for &w in &WEIGHTS {
            let known = ANCHORS.iter().any(|&(aw, _)| aw == w);
            assert!(known || w == NEW_MID, "cell {w} neither anchor nor the new sixteenth");
        }
        // The face lines, declared before the run: within 6
        // of the shelf is a hold, within 7 of the foot is a
        // whole fall; the band between is a slope.
        assert_eq!(SHELF_HOLD_LINE, SHELF.1 - 6);
        assert_eq!(FALL_LINE, LOW.1 + 7);
        assert!(FALL_LINE < SHELF_HOLD_LINE);
        assert_eq!(PLATEAU_LINE, 55);
        assert_eq!(ISOLATION_LINE, 34);
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured row, pinned so the record cannot heal:
        // at delay 12, weights -1.50 / -1.375 / -1.25 / -1.125
        // / -1.0625 / -1.00 read longest 37 / 59 / 68 / 51 /
        // 45 / 13. P1, P2, P3, P4, P6 held; P5 cracked — the
        // sixteenth (-1.0625) reads longest 45, exactly on the
        // shelf-hold line, so the shelf holds and the cliff's
        // face stands east of it, inside the last sixteenth;
        // but its perfect count is 98, ABOVE the shelf's 91 and
        // second only to the centre's 104, while its longest
        // run is the shorter: inside the cliff the counts and
        // the runs part company — the cell keeps time more
        // often than the shelf and holds it for less long.
        // The shape verdict is SHELF HOLDS — FACE EAST OF THE
        // SIXTEENTH.
        let cells = run_survey();
        let longest = |w: f32| cell_at(&cells, w).longest_episode;
        assert_eq!(longest(-1.5), 37);
        assert_eq!(longest(-1.375), 59);
        assert_eq!(longest(-1.25), 68);
        assert_eq!(longest(-1.125), 51);
        assert_eq!(longest(-1.0625), 45);
        assert_eq!(longest(-1.0), 13);
        assert_eq!(cell_at(&cells, -1.25).perfect, 104);
        assert_eq!(cell_at(&cells, -1.125).perfect, 91);
        assert_eq!(cell_at(&cells, -1.0625).perfect, 98);
        assert_eq!(cell_at(&cells, -1.0).perfect, 74);
        assert_eq!(shape_verdict(&cells), "SHELF HOLDS — FACE EAST OF THE SIXTEENTH");
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 6);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.4}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
