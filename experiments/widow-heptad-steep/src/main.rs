//! # Widow Heptad Steep 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-late` priced the crown's column and the
//! one-tick-late cells at the extension weights and found the
//! ridge's cliff was a property of the line it walked in on:
//! (-1.25, delay 12) — one tick late — stands at longest 68,
//! above the crown itself (58), with perfect 104/112, the
//! family's highest count. Its README named this instrument
//! outright: *follow the steeper exchange rate: the line
//! through the crown (-1.00, 10) and the late tower (-1.25,
//! 12) runs two ticks per quarter-dose — its next cell east
//! is (-1.50, delay 14). Does a second tower stand there, and
//! is the "segment" a longer road on a steeper slope?*
//!
//! The pair is unchanged: driver Regular Spiking I=12 (solo
//! period ~39.60), follower Fast Spiking I=4.8 (solo ~28.01).
//! Seven follower beats span ~196.08 against five driver
//! cycles' ~198.00 — near 7:5, the follower early by ~1.92
//! ticks per super-cycle. A **super-cycle** is five
//! consecutive driver cycles; it is **perfect** when it holds
//! exactly seven follower beats AND each beat's phase lies
//! within ±0.06 of that beat's median phase for the regime
//! (the two-gate rule, unchanged). An **episode** is a maximal
//! run of consecutive perfect super-cycles. Each cell counted
//! over 24,000 ticks (first 1,500 discarded; 112
//! super-cycles), exactly as the late instrument counted.
//!
//! The survey walks the steep line end to end and keeps the
//! old ground beside it:
//! - on the line, west to east: (-0.75, 8) new — the line's
//!   western step, one quarter-dose lighter and two ticks
//!   sooner than the crown; the crown (-1.00, 10, anchor);
//!   the late tower (-1.25, 12, anchor, the new champion);
//!   (-1.50, 14) new — the seed's cell, the second tower's
//!   address if there is one; (-1.75, 16) new — one step
//!   further east; (-2.00, 18) new — the far step, where a
//!   road that is only a segment should have ended;
//! - off the line, the -1.50 column's known hours as the
//!   control for the seed's cell: (-1.50, 12) at 37 and
//!   (-1.50, 13) at 25 (anchors), and the tower's on-line
//!   partner (-1.25, 11) at 34 (anchor).
//! Nine cells: five anchors earlier instruments counted,
//! four new hours on the steep line.
//!
//! Lines declared before the survey ran, from the late tower
//! (-1.25, 12) at longest 68: plateau line 55 (80% of 68,
//! rounded up); isolation line 34 (50% of 68).
//!
//! Predictions declared before the new cells ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives the steep line, as it survived every grid.
//! - P2: the five anchors reproduce their ledgers exactly:
//!   longest 58 at the crown, 34 at (-1.25, 11), 68 at the
//!   late tower, 37 / 25 down the -1.50 column's known hours;
//!   crown perfect 92, tower perfect 104.
//! - P3: a second tower stands on the steep line —
//!   (-1.50, 14) reaches the plateau line 55. Two points were
//!   a coincidence; three collinear heights are a road.
//! - P4: the road continues east, diminished — (-1.75, 16)
//!   stands at or above the isolation line 34. A segment that
//!   ended at the second tower would make P3 a pair, not a
//!   road.
//! - P5: the road is finite to the west — the western step
//!   (-0.75, 8) stands below the plateau line 55. The steep
//!   line was drawn through two eastern heights; west of the
//!   crown the old diagonal's gentler slope already priced
//!   this ground, and the steep line should not inherit it.
//! - P6: the road ends in the far east — (-2.00, 18) stands
//!   below the isolation line 34. Every segment this family
//!   has walked has ended somewhere; a line with no end at
//!   all would be a different claim than a longer road.
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
/// Plateau line: 80% of the late tower's longest (68), rounded up.
const PLATEAU_LINE: usize = 55;
/// Isolation line: 50% of the late tower's longest (68).
const ISOLATION_LINE: usize = 34;
/// The crown's address and longest episode (the diagonal grid).
const CROWN: (f32, usize, usize) = (-1.0, 10, 58);
/// The late tower's address and longest episode (heptad-late).
const TOWER: (f32, usize, usize) = (-1.25, 12, 68);

/// The steep line's new hours, west to east: the western step,
/// the seed's cell, and the two eastern steps past it. Each is
/// exactly one quarter-dose and two ticks from its neighbour
/// on the line through the crown and the tower.
const LINE_NEW: [(f32, usize); 4] = [(-0.75, 8), (-1.5, 14), (-1.75, 16), (-2.0, 18)];
/// The seed's cell: the steep line's next address east of the
/// tower, named in heptad-late's README.
const SEED_CELL: (f32, usize) = (-1.5, 14);
/// Anchors earlier instruments counted: (weight, delay,
/// longest). The crown and the tower from the diagonal and
/// late instruments; the tower's on-line partner from the
/// ridge; the -1.50 column's known hours from the ridge and
/// the late instrument.
const ANCHORS: [(f32, usize, usize); 5] = [
    (-1.0, 10, 58),
    (-1.25, 11, 34),
    (-1.25, 12, 68),
    (-1.5, 12, 37),
    (-1.5, 13, 25),
];

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
    let mut cells = Vec::with_capacity(ANCHORS.len() + LINE_NEW.len());
    for &(w, d, _) in &ANCHORS {
        cells.push(run_cell(w, d));
    }
    for &(w, d) in &LINE_NEW {
        cells.push(run_cell(w, d));
    }
    cells
}

fn cell_at(cells: &[CellLedger], weight: f32, delay: usize) -> &CellLedger {
    cells.iter().find(|c| c.weight == weight && c.delay == delay).expect("surveyed cell")
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-steep headless: the steep line through the crown and the late tower, walked end to end — does a second tower stand at (-1.50, 14), and is the segment a longer road on a steeper slope?");
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
        let mark = if (w, d) == (CROWN.0, CROWN.1) { " (crown)" } else if (w, d) == (TOWER.0, TOWER.1) { " (late tower)" } else { "" };
        println!("    anchor ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    println!("  the steep line's new hours, west to east (two ticks later per quarter-dose heavier):");
    for &(w, d) in &LINE_NEW {
        let c = cell_at(cells, w, d);
        let mark = if (w, d) == SEED_CELL { " (the seed's cell)" } else { "" };
        println!("    line ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
            c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, d, l)| cell_at(cells, w, d).longest_episode == l)
        && cell_at(cells, CROWN.0, CROWN.1).perfect == 92
        && cell_at(cells, TOWER.0, TOWER.1).perfect == 104;
    let seed = cell_at(cells, SEED_CELL.0, SEED_CELL.1).longest_episode;
    let east = cell_at(cells, -1.75, 16).longest_episode;
    let west = cell_at(cells, -0.75, 8).longest_episode;
    let far = cell_at(cells, -2.0, 18).longest_episode;
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce longest 58 / 34 / 68 / 37 / 25 with crown perfect 92 and tower perfect 104 — {}; P3 second tower on the steep line ((-1.50, 14) at or above plateau {PLATEAU_LINE}; measured {seed}) — {}; P4 road continues east diminished ((-1.75, 16) at or above isolation {ISOLATION_LINE}; measured {east}) — {}; P5 road finite to the west ((-0.75, 8) below plateau {PLATEAU_LINE}; measured {west}) — {}; P6 road ends in the far east ((-2.00, 18) below isolation {ISOLATION_LINE}; measured {far}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        if seed >= PLATEAU_LINE { "HELD" } else { "CRACKED" },
        if east >= ISOLATION_LINE { "HELD" } else { "CRACKED" },
        if west < PLATEAU_LINE { "HELD" } else { "CRACKED" },
        if far < ISOLATION_LINE { "HELD" } else { "CRACKED" });
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
    fn anchors_reproduce_the_family_ledger() {
        for &(w, d, longest) in &ANCHORS {
            let c = run_cell(w, d);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {d}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CROWN.0, CROWN.1).perfect, 92);
        assert_eq!(run_cell(TOWER.0, TOWER.1).perfect, 104);
    }

    #[test]
    fn survey_is_the_steep_line_and_its_controls() {
        assert_eq!(ANCHORS.len(), 5);
        assert_eq!(LINE_NEW.len(), 4);
        // The steep line through the crown and the tower:
        // every step east is one quarter-dose heavier and
        // exactly two ticks later.
        let line: [(f32, usize); 6] = [
            (-0.75, 8), (CROWN.0, CROWN.1), (TOWER.0, TOWER.1),
            (-1.5, 14), (-1.75, 16), (-2.0, 18),
        ];
        for w in line.windows(2) {
            assert_eq!(w[1].0 - w[0].0, -0.25);
            assert_eq!(w[1].1 - w[0].1, 2);
        }
        // The seed's cell is the line's first new hour east
        // of the tower.
        assert_eq!(SEED_CELL, (-1.5, 14));
        // Every new hour lies on that line; none duplicates
        // an anchor.
        for &(w, d) in &LINE_NEW {
            assert!(line.contains(&(w, d)), "new hour ({w}, {d}) off the line");
            assert!(!ANCHORS.contains(&(w, d, cell_longest_hint(w, d))),
                "new hour ({w}, {d}) duplicates an anchor");
        }
    }

    // The anchor longest for a (weight, delay), from the table.
    fn cell_longest_hint(w: f32, d: usize) -> usize {
        ANCHORS.iter().find(|&&(aw, ad, _)| aw == w && ad == d).map(|&(_, _, l)| l).unwrap_or(0)
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured survey, pinned so the record cannot
        // heal: anchors 58 / 34 / 68 / 37 / 25 (P2 HELD, with
        // crown perfect 92 and tower perfect 104). The steep
        // line's new hours crack both road predictions: the
        // seed's cell (-1.50, 14) stands at 38, below the
        // plateau line — no second tower; it is one above its
        // own column's on-line hour (-1.50, 12) at 37, ordinary
        // ground wearing the line's address (P3 CRACKED). One
        // step further east (-1.75, 16) falls to 17, below the
        // isolation line (P4 CRACKED). The ends behave as
        // declared: the western step (-0.75, 8) at 34, exactly
        // the isolation line and below the plateau (P5 HELD),
        // and the far step (-2.00, 18) at 26, below isolation
        // (P6 HELD) — though its perfect count 94 is the
        // survey's second highest, count and duration parting
        // company again. The steep line was a chord through
        // two heights, not a road.
        let cells = run_survey();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-1.0, 10), 58);
        assert_eq!(longest(-1.25, 11), 34);
        assert_eq!(longest(-1.25, 12), 68);
        assert_eq!(longest(-1.5, 12), 37);
        assert_eq!(longest(-1.5, 13), 25);
        assert_eq!(longest(-0.75, 8), 34);
        assert_eq!(longest(-1.5, 14), 38);
        assert_eq!(longest(-1.75, 16), 17);
        assert_eq!(longest(-2.0, 18), 26);
        assert_eq!(cell_at(&cells, -1.5, 14).perfect, 86);
        assert_eq!(cell_at(&cells, -0.75, 8).perfect, 78);
        assert_eq!(cell_at(&cells, -1.75, 16).perfect, 61);
        assert_eq!(cell_at(&cells, -2.0, 18).perfect, 94);
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 9);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.2}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
