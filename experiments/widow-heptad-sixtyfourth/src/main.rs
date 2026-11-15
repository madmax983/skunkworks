//! # Widow Heptad Sixtyfourth \U0001F570\uFE0F
//!
//! Brought by The Clockwork Widow \U0001F570\uFE0F. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model), via `widow-heptad-thirtysecond`.
//!
//! `widow-heptad-thirtysecond` stood at thirty-second grain on
//! the eastern slope of the tower's row and found the face was a
//! slope all along: at the tower's own hour (delay 12) the row
//! reads 37 / 59 / **68** / 51 / 45 / **36** / 13 along weights
//! -1.50 / -1.375 / -1.25 / -1.125 / -1.0625 / -1.03125 / -1.00.
//! The fall has an interior: 45 -> 36 -> 13. But its two halves
//! are not equal: the western tread falls 9 in its first
//! thirty-second (-1.0625 to -1.03125) and 23 in its second
//! (-1.03125 to -1.00), so the slope steepens eastward.
//! Thirtysecond's README closed with this seed, verbatim: *the
//! slope's own grain: the fall now reads 45 -> 36 -> 13 along
//! -1.0625 / -1.03125 / -1.00 \u2014 the eastern tread falls 9 in
//! its first thirty-second and 23 in its second, so the slope
//! steepens eastward; at -1.015625 (sixty-fourth grain, midway
//! between the thirty-second and the foot), does the tread hold
//! near 36, halve toward the foot, or fall whole \u2014 is the
//! slope's lower half one more step wearing a finer lens?* This
//! instrument stands on that one unpriced address.
//!
//! The pair is unchanged, and so is the counting: driver
//! Regular Spiking I=12 (solo period ~39.60), follower Fast
//! Spiking I=4.8 (solo ~28.01). Seven follower beats span
//! ~196.08 against five driver cycles' ~198.00 \u2014 near 7:5, the
//! follower **early by ~1.92 ticks per super-cycle**. A
//! **super-cycle** is five consecutive driver cycles; it is
//! **perfect** when it holds exactly seven follower beats AND
//! each beat's phase lies within \u00B10.06 of that beat's median
//! phase for the regime (the two-gate rule, unchanged). An
//! **episode** is a maximal run of consecutive perfect
//! super-cycles. Each cell counted over 24,000 ticks (first
//! 1,500 discarded; 112 super-cycles), exactly as thirtysecond
//! counted.
//!
//! The survey is the row entire as thirtysecond left it \u2014 seven
//! known cells, the five dose anchors plus cliff's ledge
//! (-1.0625, longest 45, perfect 98) and thirtysecond's tread
//! (-1.03125, longest 36, perfect 88), both now anchors in their
//! own right \u2014 plus the one sixty-fourth step between the
//! tread and the foot: **-1.015625 at delay 12**, the exact
//! midpoint of the second thirty-second, the only address
//! inside it no instrument had priced. The shape lines are
//! carried unchanged: plateau line 55, isolation line 34. Two
//! further lines price the lower slope, declared before the
//! run: the **tread-hold line 30** (within 6 of the tread's
//! 36 \u2014 the tread holds again) and the **fall line 20**
//! (within 7 of the foot's 13 \u2014 the fall is whole). Between
//! them lies the halving band 21..=29: a reading there means
//! the slope's lower half has an interior of its own \u2014 the
//! tread neither holds nor falls whole, it halves.
//!
//! Predictions declared before the new cell ran:
//! - P1: no cell earns TRUE LOCK 7:5. A fifth magnification
//!   does not teach the pair to hold.
//! - P2: the seven known cells reproduce the row exactly:
//!   longest 37 / 59 / 68 / 51 / 45 / 36 / 13, centre perfect
//!   104, shelf (-1.125) perfect 91, ledge (-1.0625) perfect
//!   98, tread (-1.03125) perfect 88.
//! - P3: the lower half HALVES \u2014 the sixty-fourth reads
//!   inside the halving band 21..=29, neither at or above the
//!   tread-hold line 30 nor at or below the fall line 20. The
//!   slope that steepens eastward spends its second
//!   thirty-second unevenly: less than half its fall in its
//!   first sixty-fourth, more in its second.
//! - P4: the centre stands \u2014 no cell in the row outlasts 68.
//! - P5: the counts descend at this grain \u2014 the new cell's
//!   perfect count lies between the foot's 74 and the tread's
//!   88, inclusive. East of the ledge the two ledgers resumed
//!   descending together at thirty-second grain; at the
//!   sixty-fourth, this planting says, they keep descending.
//! - P6: the western shoulder is intact \u2014 (-1.375, 12) at 59
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
/// The row at the tower's hour as cliff left it, entire, plus the one
/// thirty-second step inside its last sixteenth.
const WEIGHTS: [f32; 8] = [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.03125, -1.015625, -1.0];
/// The centre: the late tower itself (weight, longest).
const CENTRE: (f32, usize) = (-1.25, 68);
/// The shelf: the eastern eighth, high ground above the cliff.
const SHELF: (f32, usize) = (-1.125, 51);
/// The low anchor: the eastern quarter, the cliff's foot.
const LOW: (f32, usize) = (-1.0, 13);
/// The ledge: cliff's sixteenth, now an anchor in its own
/// right (weight, longest).
const LEDGE: (f32, usize) = (-1.0625, 45);
/// The tread: thirtysecond's step, now an anchor in its own
/// right (weight, longest).
const TREAD: (f32, usize) = (-1.03125, 36);
/// The one unpriced sixty-fourth step, midway between tread
/// and foot: (weight).
const NEW_MID: f32 = -1.015625;
/// Tread-hold line: within 6 of the tread's 36, the tread
/// holds at sixty-fourth grain.
const TREAD_HOLD_LINE: usize = 30;
/// Fall line: within 7 of the foot's 13, the fall is whole
/// at sixty-fourth grain. Between FALL_LINE and
/// TREAD_HOLD_LINE lies the halving band.
const FALL_LINE: usize = 20;
/// The shelf's perfect count as dose counted it.
const SHELF_PERFECT: usize = 91;
/// The ledge's perfect count as cliff counted it.
const LEDGE_PERFECT: usize = 98;
/// The tread's perfect count as thirtysecond counted it.
const TREAD_PERFECT: usize = 88;
/// The foot's perfect count as dose counted it.
const LOW_PERFECT: usize = 74;
/// Known cells at this hour: (weight, longest) — the whole
/// row as cliff left it, west shoulder to the cliff's foot,
/// the ledge included.
const ANCHORS: [(f32, usize); 7] = [(-1.5, 37), (-1.375, 59), (-1.25, 68), (-1.125, 51), (-1.0625, 45), (-1.03125, 36), (-1.0, 13)];

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

/// The declared shape verdict for the slope's lower half at
/// sixty-fourth grain, from the new cell's longest alone.
fn shape_verdict(cells: &[CellLedger]) -> &'static str {
    let mid = cell_at(cells, NEW_MID).longest_episode;
    if mid > TREAD.1 + 6 {
        // Above the tread-hold band entire (the band is within
        // 6 of the tread's 36, i.e. 30..=42): the slope does
        // not hold here, it rises — a second ledge.
        "SLOPE RISES AT SIXTY-FOURTH GRAIN — A SECOND LEDGE"
    } else if mid >= TREAD_HOLD_LINE {
        "TREAD HOLDS AT SIXTY-FOURTH GRAIN"
    } else if mid <= FALL_LINE {
        "FALL IS WHOLE AT SIXTY-FOURTH GRAIN"
    } else {
        "SLOPE HALVES AT SIXTY-FOURTH GRAIN"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-sixtyfourth headless: the slope's lower half at sixty-fourth grain — the row entire at the tower's hour (delay {TOWER_DELAY}), weights -1.50 / -1.375 / -1.25 / -1.125 / -1.0625 / -1.03125 / -1.015625 / -1.00: does the tread hold near 36 at the one unpriced sixty-fourth, halve toward the foot, or fall whole — is the slope's lower half one more step wearing a finer lens?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the row at the tower's hour (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}; tread-hold line {TREAD_HOLD_LINE}, fall line {FALL_LINE}; * = known cell, + = new sixty-fourth step):");
    for &w in &WEIGHTS {
        let c = cell_at(cells, w);
        let mark = if w == CENTRE.0 {
            " (centre — the tower)"
        } else if w == NEW_MID {
            " + (the sixty-fourth — inside the second thirty-second)"
        } else if w == TREAD.0 {
            " * (the tread — thirtysecond's step)"
        } else if w == LEDGE.0 {
            " * (the ledge — cliff's sixteenth)"
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
        && cell_at(cells, SHELF.0).perfect == SHELF_PERFECT
        && cell_at(cells, LEDGE.0).perfect == LEDGE_PERFECT
        && cell_at(cells, TREAD.0).perfect == TREAD_PERFECT;
    let shape = shape_verdict(cells);
    let centre_stands = cells.iter().all(|c| c.longest_episode <= CENTRE.1);
    let mid = cell_at(cells, NEW_MID);
    let mid_perfect = mid.perfect;
    let counts_descend = (LOW_PERFECT..=TREAD_PERFECT).contains(&mid_perfect);
    let west = cell_at(cells, -1.375).longest_episode;
    let second_tallest = {
        let mut ls: Vec<usize> = cells.iter().map(|c| c.longest_episode).collect();
        ls.sort_unstable();
        ls[ls.len() - 2]
    };
    let shoulder_intact = west == 59 && second_tallest == 59;
    let row_mean = cells.iter().map(|c| c.longest_episode as f64).sum::<f64>() / cells.len() as f64;
    println!("  shape verdict: {shape} (row mean longest {row_mean:.1}; the sixty-fourth reads longest {}, perfect {})", mid.longest_episode, mid_perfect);
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 known cells reproduce row 37 / 59 / 68 / 51 / 45 / 36 / 13 with centre perfect 104, shelf perfect {SHELF_PERFECT}, ledge perfect {LEDGE_PERFECT} and tread perfect {TREAD_PERFECT} — {}; P3 lower half halves (sixty-fourth inside halving band 21..=29; measured {}) — {}; P4 centre stands (no cell above {}) — {}; P5 counts descend (sixty-fourth perfect between foot {LOW_PERFECT} and tread {TREAD_PERFECT}; measured {mid_perfect}) — {}; P6 western shoulder intact (west eighth 59 and second-tallest; measured {west} / {second_tallest}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        mid.longest_episode,
        if (21..=29).contains(&mid.longest_episode) { "HELD" } else { "CRACKED" },
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
    fn anchors_reproduce_the_known_row() {
        for &(w, longest) in &ANCHORS {
            let c = run_cell(w, TOWER_DELAY);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {TOWER_DELAY}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CENTRE.0, TOWER_DELAY).perfect, 104);
        assert_eq!(run_cell(SHELF.0, TOWER_DELAY).perfect, SHELF_PERFECT);
        assert_eq!(run_cell(LEDGE.0, TOWER_DELAY).perfect, LEDGE_PERFECT);
        assert_eq!(run_cell(TREAD.0, TOWER_DELAY).perfect, TREAD_PERFECT);
        assert_eq!(run_cell(LOW.0, TOWER_DELAY).perfect, LOW_PERFECT);
    }

    #[test]
    fn survey_is_the_declared_row() {
        assert_eq!(WEIGHTS, [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.03125, -1.015625, -1.0]);
        assert_eq!(TOWER_DELAY, 12);
        assert_eq!(CENTRE, (-1.25, 68));
        assert_eq!(SHELF, (-1.125, 51));
        assert_eq!(LEDGE, (-1.0625, 45));
        assert_eq!(TREAD, (-1.03125, 36));
        assert_eq!(LOW, (-1.0, 13));
        // The new cell is the exact midpoint of the second
        // thirty-second: halfway between tread and foot.
        assert_eq!(NEW_MID, (TREAD.0 + LOW.0) / 2.0);
        assert_eq!(ANCHORS.len(), 7);
        for &w in &WEIGHTS {
            let known = ANCHORS.iter().any(|&(aw, _)| aw == w);
            assert!(known || w == NEW_MID, "cell {w} neither known nor the new sixty-fourth");
        }
        // The slope lines, declared before the run: within 6
        // of the tread is a hold, within 7 of the foot is a
        // whole fall; the band between is the halving band.
        assert_eq!(TREAD_HOLD_LINE, TREAD.1 - 6);
        assert_eq!(FALL_LINE, LOW.1 + 7);
        assert!(FALL_LINE < TREAD_HOLD_LINE);
        assert_eq!(PLATEAU_LINE, 55);
        assert_eq!(ISOLATION_LINE, 34);
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured row, pinned so the record cannot heal:
        // at delay 12, weights -1.50 / -1.375 / -1.25 / -1.125
        // / -1.0625 / -1.03125 / -1.015625 / -1.00 read longest
        // 37 / 59 / 68 / 51 / 45 / 36 / 46 / 13. P1, P2, P4, P6
        // held; P3 and P5 cracked — the sixty-fourth (-1.015625)
        // reads longest 46, above the tread-hold band entire and
        // one above the ledge's 45: inside the slope's lower
        // half stands a second ledge, and the eastern descent is
        // not monotone (51 -> 45 -> 36 -> 46 -> 13). Its perfect
        // count is 93, above the tread's 88, so at this cell the
        // two ledgers step up together — unlike the ledge, where
        // they parted. The shape verdict is SLOPE RISES AT
        // SIXTY-FOURTH GRAIN — A SECOND LEDGE.
        let cells = run_survey();
        let longest = |w: f32| cell_at(&cells, w).longest_episode;
        assert_eq!(longest(-1.5), 37);
        assert_eq!(longest(-1.375), 59);
        assert_eq!(longest(-1.25), 68);
        assert_eq!(longest(-1.125), 51);
        assert_eq!(longest(-1.0625), 45);
        assert_eq!(longest(-1.03125), 36);
        assert_eq!(longest(-1.015625), 46);
        assert_eq!(longest(-1.0), 13);
        assert_eq!(cell_at(&cells, -1.25).perfect, 104);
        assert_eq!(cell_at(&cells, -1.125).perfect, 91);
        assert_eq!(cell_at(&cells, -1.0625).perfect, 98);
        assert_eq!(cell_at(&cells, -1.03125).perfect, 88);
        assert_eq!(cell_at(&cells, -1.015625).perfect, 93);
        assert_eq!(cell_at(&cells, -1.0).perfect, 74);
        assert_eq!(shape_verdict(&cells), "SLOPE RISES AT SIXTY-FOURTH GRAIN — A SECOND LEDGE");
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 8);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.4}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
