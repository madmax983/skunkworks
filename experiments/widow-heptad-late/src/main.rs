//! # Widow Heptad Late 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-ridge` walked the 7:5 compensation line out of
//! the diagonal's grid and found a finite segment: two cells
//! long, crown at its eastern end, a cliff past the crown
//! (58 -> 34 at (-1.25, 11), recovering only to 37 at
//! (-1.50, 12)), and one-tick-early controls that beat their
//! on-line partners at two of the three extension weights. Its
//! README named this instrument outright: *walk one step off
//! the line at the crown's own weight — price the crown's
//! column at fine grain east of the crown, and the one-tick-
//! late cells at the extension weights, (-1.25, 12) and
//! (-1.50, 13). If the cliff is a property of the line, the
//! late controls should fall too; if it is a property of the
//! hour, one of them may stand where the line fell.*
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
//! ridge counted.
//!
//! The survey is three columns, read by delay. Two of the
//! crown column's eastern hours were already priced by other
//! instruments — the peak grid counted (-1.00, delay 11) at
//! longest 28 and the plane counted (-1.00, delay 12) at 13 —
//! so the column east of the crown so far reads 58 -> 28 ->
//! 13 and only its next hour is new:
//! - the crown's column at weight -1.00: the crown (delay
//!   10, anchor), delays 11 and 12 (anchors, from the peak
//!   and the plane), and delay 13 (new — the first hour past
//!   the plane's eastern edge);
//! - the -1.25 column: early control (delay 10, anchor),
//!   on-line (delay 11, anchor), late (delay 12, new);
//! - the -1.50 column: early control (delay 11, anchor),
//!   on-line (delay 12, anchor), late (delay 13, new).
//! Ten cells: seven anchors earlier instruments counted,
//! three new hours.
//!
//! Lines declared before the survey ran, from the crown
//! (-1.00, 10) at longest 58: plateau line 47 (80% of 58,
//! rounded up); isolation line 29 (50% of 58).
//!
//! Predictions declared before the new cells ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives the late hours, as it survived every grid.
//! - P2: the seven anchors reproduce their ledgers exactly:
//!   longest 58 / 28 / 13 down the crown's column, 39 / 34 at
//!   -1.25, 15 / 37 at -1.50; crown perfect 92.
//! - P3: the cliff is a property of the segment, not of the
//!   hour — both one-tick-late cells fall below the plateau
//!   line 47, as their on-line partners did.
//! - P4: the crown's column keeps falling east — the new
//!   hour (-1.00, delay 13) stands at or below the isolation
//!   line 29, continuing 58 -> 28 -> 13 without a recovery.
//!   The crown is a point, not a column.
//! - P5: lateness earns nothing at the extension weights —
//!   each late cell stands at or below its on-line partner at
//!   the same weight: (-1.25, 12) <= 34, (-1.50, 13) <= 37.
//!   (The early controls beat their partners at two weights;
//!   the late side is where the declared order should hold
//!   if arrival time is a price and not a lottery.)
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

/// The crown column's next hour, east of every prior grid:
/// the plane's delays stopped at 12 and the peak's at 11.
const CROWN_NEXT: (f32, usize) = (-1.0, 13);
/// One-tick-late cells at the two extension weights.
const LATE: [(f32, usize); 2] = [(-1.25, 12), (-1.5, 13)];
/// Anchors earlier instruments counted: (weight, delay,
/// longest). The crown and its column's eastern hours come
/// from the diagonal/peak/plane; the extension columns from
/// the ridge's walk and its early controls.
const ANCHORS: [(f32, usize, usize); 7] = [
    (-1.0, 10, 58),
    (-1.0, 11, 28),
    (-1.0, 12, 13),
    (-1.25, 10, 39),
    (-1.25, 11, 34),
    (-1.5, 11, 15),
    (-1.5, 12, 37),
];
/// The on-line partners of the late cells, for P5's comparison.
const LATE_PARTNERS: [(f32, usize, usize); 2] = [(-1.25, 11, 34), (-1.5, 12, 37)];

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
    let mut cells = Vec::with_capacity(ANCHORS.len() + 1 + LATE.len());
    for &(w, d, _) in &ANCHORS {
        cells.push(run_cell(w, d));
    }
    cells.push(run_cell(CROWN_NEXT.0, CROWN_NEXT.1));
    for &(w, d) in &LATE {
        cells.push(run_cell(w, d));
    }
    cells
}

fn cell_at(cells: &[CellLedger], weight: f32, delay: usize) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight && c.delay == delay).expect("surveyed cell")
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-late headless: one tick late at the extension weights, and the crown's own column east of the crown — is the cliff a property of the line, or of the hour?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  anchors from the earlier instruments (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}):");
    for &(w, d, _) in &ANCHORS {
        let c = cell_at(cells, w, d);
        let mark = if (w, d) == (CROWN.0, CROWN.1) { " (crown)" } else { "" };
        println!("    anchor ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    println!("  the crown column's next hour, past the plane's eastern edge:");
    {
        let c = cell_at(cells, CROWN_NEXT.0, CROWN_NEXT.1);
        println!("    column ({:+.2}, delay {}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
            CROWN_NEXT.0, CROWN_NEXT.1, c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    println!("  one tick late at the extension weights:");
    for &(w, d) in &LATE {
        let c = cell_at(cells, w, d);
        println!("    late ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, d, l)| cell_at(cells, w, d).longest_episode == l)
        && cell_at(cells, CROWN.0, CROWN.1).perfect == 92;
    let late_max = LATE.iter().map(|&(w, d)| cell_at(cells, w, d).longest_episode).max().unwrap();
    let crown_next = cell_at(cells, CROWN_NEXT.0, CROWN_NEXT.1).longest_episode;
    let late_below_partners = LATE_PARTNERS.iter().all(|&(w, _, partner_longest)| {
        let late_delay = LATE.iter().find(|&&(lw, _)| lw == w).map(|&(_, ld)| ld).unwrap();
        cell_at(cells, w, late_delay).longest_episode <= partner_longest
    });
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce longest 58 / 28 / 13 / 39 / 34 / 15 / 37 with crown perfect 92 — {}; P3 cliff is the segment's, not the hour's (both late cells below plateau {PLATEAU_LINE}; best late longest {late_max}) — {}; P4 crown's column keeps falling east ((-1.00, 13) at or below isolation {ISOLATION_LINE}; measured {crown_next}) — {}; P5 lateness earns nothing (each late cell at or below its on-line partner 34 / 37) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        if late_max < PLATEAU_LINE { "HELD" } else { "CRACKED" },
        if crown_next <= ISOLATION_LINE { "HELD" } else { "CRACKED" },
        if late_below_partners { "HELD" } else { "CRACKED" });
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
        let c = run_cell(-1.0, 11);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn anchors_reproduce_the_ridge_ledger() {
        for &(w, d, longest) in &ANCHORS {
            let c = run_cell(w, d);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {d}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CROWN.0, CROWN.1).perfect, 92);
    }

    #[test]
    fn survey_is_the_declared_columns() {
        assert_eq!(ANCHORS.len(), 7);
        assert_eq!(LATE.len(), 2);
        // The crown column's anchors are consecutive hours
        // east from the crown itself: delays 10, 11, 12 at
        // the crown's weight, falling 58 / 28 / 13.
        let column: Vec<(f32, usize, usize)> = ANCHORS.iter()
            .copied().filter(|&(w, _, _)| w == CROWN.0).collect();
        assert_eq!(column, vec![(-1.0, 10, 58), (-1.0, 11, 28), (-1.0, 12, 13)]);
        // The new hour is the next hour east of the column's
        // last anchor, at the crown's weight.
        assert_eq!(CROWN_NEXT, (CROWN.0, 13));
        // Every late cell is exactly one tick late for an
        // on-line anchor at the same weight.
        for &(w, d) in &LATE {
            assert!(ANCHORS.contains(&(w, d - 1, cell_longest_hint(w, d - 1))),
                "late cell ({w}, {d}) has no on-line partner anchor");
        }
    }

    // The anchor longest for a (weight, delay), from the table.
    fn cell_longest_hint(w: f32, d: usize) -> usize {
        ANCHORS.iter().find(|&&(aw, ad, _)| aw == w && ad == d).map(|&(_, _, l)| l).unwrap_or(0)
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured survey, pinned so the record cannot
        // heal: anchors 58 / 28 / 13 down the crown's column,
        // 39 / 34 at -1.25, 15 / 37 at -1.50 (P2 HELD). The
        // new hours crack all three remaining predictions:
        // (-1.00, 13) at 31 stands two above the isolation
        // line — the column's fall 58 -> 28 -> 13 partially
        // recovers (P4 CRACKED); (-1.50, 13) at 25 falls
        // below its on-line partner, but (-1.25, 12) at 68 —
        // perfect 104/112, the family's highest count — rises
        // above the plateau line and above the crown itself
        // (P3 and P5 CRACKED). The cliff was a property of
        // the line's slope, not of the ground east of the
        // crown.
        let cells = run_survey();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-1.0, 10), 58);
        assert_eq!(longest(-1.0, 11), 28);
        assert_eq!(longest(-1.0, 12), 13);
        assert_eq!(longest(-1.0, 13), 31);
        assert_eq!(longest(-1.25, 10), 39);
        assert_eq!(longest(-1.25, 11), 34);
        assert_eq!(longest(-1.25, 12), 68);
        assert_eq!(longest(-1.5, 11), 15);
        assert_eq!(longest(-1.5, 12), 37);
        assert_eq!(longest(-1.5, 13), 25);
        assert_eq!(cell_at(&cells, -1.25, 12).perfect, 104);
        assert_eq!(cell_at(&cells, -1.0, 13).perfect, 90);
        assert_eq!(cell_at(&cells, -1.5, 13).perfect, 77);
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 10);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.2}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
