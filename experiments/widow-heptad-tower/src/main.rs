//! # Widow Heptad Tower 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! `widow-heptad-late` found a tower where the ridge's line had
//! drawn a cliff: (-1.25, delay 12) — one tick late at the first
//! extension weight — stands at longest 68 with perfect 104/112,
//! above the crown (-1.00, 10) itself. Its README carried two
//! seeds. `widow-heptad-steep` walked the first — the steep line
//! through crown and tower — and found a chord, not a road: the
//! line's next cell (-1.50, 14) fell to 38, ordinary ground in
//! its own column. The steep instrument's README then named
//! this instrument outright, in the late instrument's own
//! words: *the tower deserves its own fine grid, as the crown
//! had: centred on (-1.25, 12) — is 68 a tooth, a plateau, or
//! a shoulder of something taller still?*
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
//! peak grid counted the crown.
//!
//! The survey is the peak instrument's fine grid moved to the
//! tower: **3 weights × 3 delays = 9 cells** — weights -1.50,
//! -1.25, -1.00 (quarter steps); delays 11, 12, 13 (single
//! ticks) — centred on the tower (-1.25, 12). Eight of the
//! nine cells are **anchors** earlier instruments already
//! counted, scattered across the plane, the ridge, the late
//! columns, and the steep walk; they have simply never been
//! assembled as one grid around this centre. One hour is new:
//! **(-1.25, delay 13)** — the tower's own column, one tick
//! past the tower, the only neighbour of the centre no
//! instrument has priced.
//!
//! Shape definitions, declared before the survey ran, carried
//! from the peak grid with the centre's height substituted
//! (champion longest C = 68): plateau line 55 (80% of 68,
//! rounded up); isolation line 34 (50% of 68). PLATEAU — at
//! least 2 of the centre's 4 orthogonal neighbours reach 55;
//! TOOTH — no orthogonal neighbour reaches 55; ISOLATED TOOTH
//! — a tooth whose every neighbour, diagonal included, falls
//! at or below 34; SHOULDER — anything else.
//!
//! Predictions declared before the new cell ran:
//! - P1: no cell earns TRUE LOCK 7:5. The pair's refusal to
//!   hold survives magnification at a second centre, as it
//!   survived it at the crown.
//! - P2: the eight anchors reproduce their ledgers exactly:
//!   longest 15 / 37 / 25 down the -1.50 column, 34 / 68 at
//!   -1.25, 28 / 13 / 31 down the -1.00 column; tower perfect
//!   104, (-1.00, 13) perfect 90, (-1.50, 13) perfect 77.
//! - P3: the tower is a TOOTH — no orthogonal neighbour
//!   reaches the plateau line 55. (Three of the four are
//!   already known — 37, 13, 34 — so the prediction rides on
//!   the new hour, as an honest prediction should.)
//! - P4: the centre stands — no fine cell outlasts 68. The
//!   tower is the tallest address in its own neighbourhood.
//! - P5: the tower is one hour wide in delay — the new hour
//!   (-1.25, 13) stands at or below the isolation line 34.
//!   One tick late was rewarded at this weight; two ticks
//!   late is taxed, as the crown's own column taxed its
//!   second late hour (58 -> 28 -> 13) and as the -1.50
//!   column taxed its late hour (37 -> 25).
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

/// The fine grid: quarter-step weights around the tower.
const WEIGHTS: [f32; 3] = [-1.5, -1.25, -1.0];
/// The fine grid: single-tick delays around the tower.
const DELAYS: [usize; 3] = [11, 12, 13];
/// The centre: the late tower itself.
const CENTRE: (f32, usize, usize) = (-1.25, 12, 68);
/// The one unpriced hour: the tower's column, one tick east.
const NEW_HOUR: (f32, usize) = (-1.25, 13);
/// Anchors earlier instruments counted: (weight, delay,
/// longest). The -1.50 column from the ridge and the late
/// instrument; the tower and its western hour from the ridge,
/// the late instrument, and the steep walk; the -1.00 column
/// from the peak grid, the plane, and the late instrument.
const ANCHORS: [(f32, usize, usize); 8] = [
    (-1.5, 11, 15),
    (-1.5, 12, 37),
    (-1.5, 13, 25),
    (-1.25, 11, 34),
    (-1.25, 12, 68),
    (-1.0, 11, 28),
    (-1.0, 12, 13),
    (-1.0, 13, 31),
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
    let mut cells = Vec::with_capacity(9);
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

/// The centre's four orthogonal neighbours: (weight, delay).
fn orthogonal_neighbours() -> [(f32, usize); 4] {
    [
        (CENTRE.0 - 0.25, CENTRE.1),
        (CENTRE.0 + 0.25, CENTRE.1),
        (CENTRE.0, CENTRE.1 - 1),
        (CENTRE.0, CENTRE.1 + 1),
    ]
}

/// The centre's four diagonal neighbours: (weight, delay).
fn diagonal_neighbours() -> [(f32, usize); 4] {
    [
        (CENTRE.0 - 0.25, CENTRE.1 - 1),
        (CENTRE.0 - 0.25, CENTRE.1 + 1),
        (CENTRE.0 + 0.25, CENTRE.1 - 1),
        (CENTRE.0 + 0.25, CENTRE.1 + 1),
    ]
}

/// The declared shape verdict for a measured grid.
fn shape_verdict(cells: &[CellLedger]) -> &'static str {
    let orth = orthogonal_neighbours();
    let at_plateau = orth.iter()
        .filter(|&&(w, d)| cell_at(cells, w, d).longest_episode >= PLATEAU_LINE)
        .count();
    if at_plateau >= 2 {
        return "PLATEAU";
    }
    if at_plateau == 1 {
        return "SHOULDER";
    }
    let all_low = orth.iter().chain(diagonal_neighbours().iter())
        .all(|&(w, d)| cell_at(cells, w, d).longest_episode <= ISOLATION_LINE);
    if all_low {
        "ISOLATED TOOTH"
    } else {
        "TOOTH"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-tower headless: the late tower's own fine grid — centred on (-1.25, 12), is 68 a tooth, a plateau, or a shoulder of something taller?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the fine grid around the tower (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}; * = anchor, + = the one new hour):");
    for &w in &WEIGHTS {
        for &d in &DELAYS {
            let c = cell_at(cells, w, d);
            let mark = if (w, d) == (CENTRE.0, CENTRE.1) {
                " (centre — the tower)"
            } else if (w, d) == NEW_HOUR {
                " +"
            } else {
                " *"
            };
            println!("    grid ({w:+.2}, delay {d}): perfect {}/{}, count slips {}, phase slips {}; episodes {}, longest {}, mean episode {:.1} — {}{mark}",
                c.perfect, c.super_cycles, c.count_slips, c.phase_slips, c.episodes, c.longest_episode, c.mean_episode, c.verdict);
        }
    }
    let true_locks = cells.iter().filter(|c| c.verdict == "TRUE LOCK 7:5").count();
    let anchors_hold = ANCHORS.iter().all(|&(w, d, l)| cell_at(cells, w, d).longest_episode == l)
        && cell_at(cells, CENTRE.0, CENTRE.1).perfect == 104
        && cell_at(cells, -1.0, 13).perfect == 90
        && cell_at(cells, -1.5, 13).perfect == 77;
    let shape = shape_verdict(cells);
    let centre_stands = cells.iter().all(|c| c.longest_episode <= CENTRE.2);
    let new_hour = cell_at(cells, NEW_HOUR.0, NEW_HOUR.1).longest_episode;
    let grid_mean = cells.iter().map(|c| c.longest_episode as f64).sum::<f64>() / cells.len() as f64;
    println!("  shape verdict: {shape} (grid mean longest {grid_mean:.1})");
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 anchors reproduce longest 15 / 37 / 25 / 34 / 68 / 28 / 13 / 31 with tower perfect 104 — {}; P3 tower is a TOOTH (no orthogonal neighbour at plateau {PLATEAU_LINE}) — {}; P4 centre stands (no cell above {}) — {}; P5 tower is one hour wide ((-1.25, 13) at or below isolation {ISOLATION_LINE}; measured {new_hour}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        if shape == "TOOTH" || shape == "ISOLATED TOOTH" { "HELD" } else { "CRACKED" },
        CENTRE.2,
        if centre_stands { "HELD" } else { "CRACKED" },
        if new_hour <= ISOLATION_LINE { "HELD" } else { "CRACKED" });
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
        let c = run_cell(-1.25, 11);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.super_cycles);
        assert!(c.super_cycles > 100, "super-cycles {}", c.super_cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn anchors_reproduce_the_scattered_ledgers() {
        for &(w, d, longest) in &ANCHORS {
            let c = run_cell(w, d);
            assert_eq!(c.longest_episode, longest, "anchor ({w}, {d}) longest {}", c.longest_episode);
        }
        assert_eq!(run_cell(CENTRE.0, CENTRE.1).perfect, 104);
        assert_eq!(run_cell(-1.0, 13).perfect, 90);
        assert_eq!(run_cell(-1.5, 13).perfect, 77);
    }

    #[test]
    fn survey_is_the_declared_fine_grid() {
        assert_eq!(WEIGHTS, [-1.5, -1.25, -1.0]);
        assert_eq!(DELAYS, [11, 12, 13]);
        assert_eq!(CENTRE, (-1.25, 12, 68));
        // The centre is the grid's middle cell in both axes.
        assert_eq!(WEIGHTS[1], CENTRE.0);
        assert_eq!(DELAYS[1], CENTRE.1);
        // Eight of nine cells are anchors; the only cell no
        // earlier instrument priced is the new hour, the
        // tower's own column one tick east of the centre.
        assert_eq!(ANCHORS.len(), 8);
        for &w in &WEIGHTS {
            for &d in &DELAYS {
                let known = ANCHORS.iter().any(|&(aw, ad, _)| aw == w && ad == d);
                assert!(known || (w, d) == NEW_HOUR, "cell ({w}, {d}) neither anchor nor new hour");
            }
        }
        assert_eq!(NEW_HOUR, (CENTRE.0, CENTRE.1 + 1));
        // The shape lines are the peak grid's fractions of
        // the centre's height: 80% rounded up, 50% exact.
        assert_eq!(PLATEAU_LINE, 55);
        assert_eq!(ISOLATION_LINE, 34);
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured grid, pinned so the record cannot
        // heal: rows by weight, delays 11 / 12 / 13 —
        // -1.50: 15 / 37 / 25; -1.25: 34 / 68 / 32;
        // -1.00: 28 / 13 / 31. All five predictions held:
        // the anchors reproduce (P2), no cell locks (P1),
        // no orthogonal neighbour reaches the plateau line
        // 55 (P3 — the shape verdict is TOOTH, kept from
        // ISOLATED TOOTH only by the western neighbour
        // (-1.50, 12) at 37, three above the isolation
        // line), the centre stands (P4), and the one new
        // hour (-1.25, 13) falls to 32, two below the
        // isolation line — the tower is one hour wide in
        // delay (P5).
        let cells = run_survey();
        let longest = |w: f32, d: usize| cell_at(&cells, w, d).longest_episode;
        assert_eq!(longest(-1.5, 11), 15);
        assert_eq!(longest(-1.5, 12), 37);
        assert_eq!(longest(-1.5, 13), 25);
        assert_eq!(longest(-1.25, 11), 34);
        assert_eq!(longest(-1.25, 12), 68);
        assert_eq!(longest(-1.25, 13), 32);
        assert_eq!(longest(-1.0, 11), 28);
        assert_eq!(longest(-1.0, 12), 13);
        assert_eq!(longest(-1.0, 13), 31);
        assert_eq!(cell_at(&cells, -1.25, 12).perfect, 104);
        assert_eq!(cell_at(&cells, -1.25, 13).perfect, 80);
        assert_eq!(shape_verdict(&cells), "TOOTH");
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
