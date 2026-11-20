//! # Widow Heptad Fourthousandninetysixth \U0001f570\ufe0f
//!
//! Brought by The Clockwork Widow \U0001f570\ufe0f. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model), via
//! `widow-heptad-twothousandfortyeighth`.
//!
//! `widow-heptad-twothousandfortyeighth` priced the last
//! 1024th at 2048th grain and found the fall reversed: at
//! the tower's own hour (delay 12) the row reads 37 / 59 /
//! **68** / 51 / 45 / 36 / 46 / 25 / 36 / 47 / 21 / **35**
//! / 13 along weights -1.50 / -1.375 / -1.25 / -1.125 /
//! -1.0625 / -1.03125 / -1.015625 / -1.0078125 /
//! -1.00390625 / -1.001953125 / -1.0009765625 /
//! -1.00048828125 / -1.00, and the count that had dipped
//! to **58** at the 1024th overshot the foot's own 74 to
//! stand at **90** - both ledgers rising together at the
//! smallest grain that family had priced, the dip behind
//! them, the foot the low ground on both ledgers.
//! Twothousandfortyeighth's README closed with this seed,
//! verbatim: *the last 2048th: the fall's remainder - 22,
//! from 35 to 13 - now stands between -1.00048828125 and
//! -1.00, larger than the whole interval this instrument
//! priced, for the fifth time in this family. At
//! -1.000244140625 (4096th grain, midway between the
//! 2048th and the foot), does the reversal stand (near
//! 35), halve at the new grain (24, the exact midpoint of
//! 35 and 13), or fall whole to the foot - and does the
//! count, having overshot the foot to 90, stand near the
//! plateau counts (94) or drop back through the foot's 74
//! toward the 1024th's 58?* This instrument stands on
//! that one unpriced address.
//!
//! The pair is unchanged, and so is the counting: driver
//! Regular Spiking I=12 (solo period ~39.60), follower
//! Fast Spiking I=4.8 (solo ~28.01). Seven follower beats
//! span ~196.08 against five driver cycles' ~198.00 -
//! near 7:5, the follower **early by ~1.92 ticks per
//! super-cycle**. A **super-cycle** is five consecutive
//! driver cycles; it is **perfect** when it holds exactly
//! seven follower beats AND each beat's phase lies within
//! +/-0.06 of that beat's median phase for the regime
//! (the two-gate rule, unchanged). An **episode** is a
//! maximal run of consecutive perfect super-cycles. Each
//! cell counted over 24,000 ticks (first 1,500
//! discarded; 112 super-cycles), exactly as
//! twothousandfortyeighth counted.
//!
//! The survey is the row entire as
//! twothousandfortyeighth left it - thirteen known
//! cells, the five dose anchors plus cliff's ledge
//! (-1.0625, longest 45, perfect 98), thirtysecond's
//! tread (-1.03125, longest 36, perfect 88),
//! sixtyfourth's second ledge (-1.015625, longest 46,
//! perfect 93), hundredtwentyeighth's step (-1.0078125,
//! longest 25, perfect 68), twohundredfiftysixth's step
//! (-1.00390625, longest 36, perfect 94),
//! fivehundredtwelfth's step (-1.001953125, longest 47,
//! perfect 94), onethousandtwentyfourth's step
//! (-1.0009765625, longest 21, perfect 58) and
//! twothousandfortyeighth's step (-1.00048828125,
//! longest 35, perfect 90), all anchors in their own
//! right - plus the one 4096th step between the 2048th
//! and the foot: **-1.000244140625 at delay 12**, the
//! exact midpoint of the last 2048th, the only address
//! inside it no instrument had priced. The shape lines
//! are carried unchanged: plateau line 55, isolation
//! line 34. Two further lines price the reversal's
//! remainder, declared before the run: the **hold line
//! 32** (within 3 of the 2048th's 35 - the reversal
//! stands) and the **fall line 16** (within 3 of the
//! foot's 13 - the fall is whole). Between them stands
//! the halving band 17..=31, centred on **24**, the
//! exact midpoint of 35 and 13. At 2048th grain the
//! halving had narrowed to a point and the fall declined
//! it upward; at 4096th grain it is a band again, and
//! this planting names its centre before pricing it.
//!
//! Predictions declared before the new cell ran:
//! - P1: no cell earns TRUE LOCK 7:5. An eleventh
//!   magnification does not teach the pair to hold.
//! - P2: the thirteen known cells reproduce the row
//!   exactly: longest 37 / 59 / 68 / 51 / 45 / 36 / 46 /
//!   25 / 36 / 47 / 21 / 35 / 13, centre perfect 104,
//!   shelf (-1.125) perfect 91, ledge (-1.0625) perfect
//!   98, tread (-1.03125) perfect 88, second ledge
//!   (-1.015625) perfect 93, the 128th (-1.0078125)
//!   perfect 68, the 256th (-1.00390625) perfect 94, the
//!   512th (-1.001953125) perfect 94, the 1024th
//!   (-1.0009765625) perfect 58, the 2048th
//!   (-1.00048828125) perfect 90, foot (-1.00)
//!   perfect 74.
//! - P3: the fall HALVES AT THE NEW GRAIN - the 4096th
//!   reads exactly 24, the exact midpoint of the
//!   2048th's 35 and the foot's 13. The reversal spent
//!   one grain standing back up; what remains is 22, and
//!   this planting names the band's centre before
//!   pricing it: if the descent keeps any proportion at
//!   all, it keeps it here.
//! - P4: the centre stands - no cell in the row outlasts
//!   68.
//! - P5: the count HOLDS BETWEEN FOOT AND 2048TH - the
//!   new cell's perfect count lies in 75..=90: above the
//!   foot's own 74, at or below the 2048th's 90. The
//!   count overshot the foot at the farther address; as
//!   the address approaches the foot, the count settles
//!   between the two readings it stands between.
//! - P6: the western shoulder is intact - (-1.375, 12)
//!   at 59 remains the row's second-tallest cell, only
//!   the centre above it.
//!
//! Every period in this file is a named constant or a
//! measured quantity. A magic number is a clock with no
//! face.

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
/// 4096th step inside its last 2048th.
const WEIGHTS: [f32; 14] = [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.03125, -1.015625, -1.0078125, -1.00390625, -1.001953125, -1.0009765625, -1.00048828125, -1.000244140625, -1.0];
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
/// The second ledge: sixtyfourth's step, now an anchor in
/// its own right (weight, longest).
const SECOND_LEDGE: (f32, usize) = (-1.015625, 46);
/// The 128th step: hundredtwentyeighth's step, now an
/// anchor in its own right (weight, longest).
const THE_128TH: (f32, usize) = (-1.0078125, 25);
/// The 256th step: twohundredfiftysixth's step, now an
/// anchor in its own right (weight, longest).
const THE_256TH: (f32, usize) = (-1.00390625, 36);
/// The 512th step: fivehundredtwelfth's step, now an
/// anchor in its own right (weight, longest).
const THE_512TH: (f32, usize) = (-1.001953125, 47);
/// The 1024th step: onethousandtwentyfourth's step, now an
/// anchor in its own right (weight, longest).
const THE_1024TH: (f32, usize) = (-1.0009765625, 21);
/// The 2048th step: twothousandfortyeighth's step, now an
/// anchor in its own right (weight, longest).
const THE_2048TH: (f32, usize) = (-1.00048828125, 35);
/// The one unpriced 4096th step, midway between the 2048th
/// and the foot: (weight).
const NEW_MID: f32 = -1.000244140625;
/// Hold line: within 3 of the 2048th's 35, the reversal
/// stands at 4096th grain.
const TREAD_HOLD_LINE: usize = 32;
/// Fall line: within 3 of the foot's 13, the fall is whole
/// at 4096th grain. Between FALL_LINE and TREAD_HOLD_LINE
/// stands the halving band 17..=31, centred on 24, the
/// exact midpoint of 35 and 13.
const FALL_LINE: usize = 16;
/// The shelf's perfect count as dose counted it.
const SHELF_PERFECT: usize = 91;
/// The ledge's perfect count as cliff counted it.
const LEDGE_PERFECT: usize = 98;
/// The tread's perfect count as thirtysecond counted it.
const TREAD_PERFECT: usize = 88;
/// The second ledge's perfect count as sixtyfourth counted it.
const SECOND_LEDGE_PERFECT: usize = 93;
/// The foot's perfect count as dose counted it.
const LOW_PERFECT: usize = 74;
/// The 128th's perfect count as hundredtwentyeighth
/// counted it - below the foot's own, the dip.
const MID_PERFECT: usize = 68;
/// The 256th's perfect count as twohundredfiftysixth
/// counted it - the overshoot, one above the second ledge.
const STEP256_PERFECT: usize = 94;
/// The 512th's perfect count as fivehundredtwelfth
/// counted it - exactly the 256th's own, unmoved.
const STEP512_PERFECT: usize = 94;
/// The 1024th's perfect count as onethousandtwentyfourth
/// counted it - the count finally moved, below the foot.
const STEP1024_PERFECT: usize = 58;
/// The 2048th's perfect count as twothousandfortyeighth
/// counted it - the count overshot the foot entire.
const STEP2048_PERFECT: usize = 90;
/// Known cells at this hour: (weight, longest) — the whole
/// row as cliff left it, west shoulder to the cliff's foot,
/// the ledge included.
const ANCHORS: [(f32, usize); 13] = [(-1.5, 37), (-1.375, 59), (-1.25, 68), (-1.125, 51), (-1.0625, 45), (-1.03125, 36), (-1.015625, 46), (-1.0078125, 25), (-1.00390625, 36), (-1.001953125, 47), (-1.0009765625, 21), (-1.00048828125, 35), (-1.0, 13)];

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

/// The declared shape verdict for the reversal's
/// remainder at 4096th grain, from the new cell's
/// longest alone.
fn shape_verdict(cells: &[CellLedger]) -> &'static str {
    let mid = cell_at(cells, NEW_MID).longest_episode;
    if mid > THE_2048TH.1 {
        // Above the 2048th entire: the reversal that stood
        // up at 2048th grain grows at the next grain too.
        "REVERSAL GROWS AT 4096TH GRAIN"
    } else if mid >= TREAD_HOLD_LINE {
        "THE 2048TH HOLDS AT 4096TH GRAIN"
    } else if mid == 24 {
        // The exact midpoint of 35 and 13: the centre of
        // the halving band at this grain, and the fall
        // stands on it.
        "FALL HALVES AT 4096TH GRAIN"
    } else if mid > LOW.1 {
        // 14..=31 except 24: below the hold line, not yet
        // whole - the fall resumes without keeping the
        // band's centre.
        "FALL RESUMES AT 4096TH GRAIN"
    } else {
        "FALL IS WHOLE AT 4096TH GRAIN"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, bare: &CellLedger, cells: &[CellLedger]) {
    println!("widow-heptad-fourthousandninetysixth headless: the last 2048th at 4096th grain — the row entire at the tower's hour (delay {TOWER_DELAY}), weights -1.50 / -1.375 / -1.25 / -1.125 / -1.0625 / -1.03125 / -1.015625 / -1.0078125 / -1.00390625 / -1.001953125 / -1.0009765625 / -1.00048828125 / -1.000244140625 / -1.00: does the reversal stand at the one unpriced 4096th (near 35), halve at the new grain (24, the exact midpoint), or fall whole to the foot — and does the count, having overshot the foot to 90, stand near the plateau counts (94) or drop back through the foot's 74 toward the 1024th's 58?");
    println!("  window: {TICKS} ticks per cell, first {SETTLE_TICKS} discarded as settling; a super-cycle is perfect with exactly 7 follower beats, each within ±{PHASE_TOL} of its median phase");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 5 driver cycles {:.2} ticks against 7 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 7:5, not at it",
            5.0 * driver.mean_isi, 7.0 * follower.mean_isi, 5.0 * driver.mean_isi - 7.0 * follower.mean_isi);
    }
    println!("  uncoupled pair: super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles, mean episode {:.1} — {}",
        bare.super_cycles, bare.perfect, bare.count_slips, bare.phase_slips, bare.episodes, bare.longest_episode, bare.mean_episode, bare.verdict);
    println!("  the row at the tower's hour (plateau line {PLATEAU_LINE}, isolation line {ISOLATION_LINE}; hold line {TREAD_HOLD_LINE}, fall line {FALL_LINE}; * = known cell, + = new 4096th step):");
    for &w in &WEIGHTS {
        let c = cell_at(cells, w);
        let mark = if w == THE_2048TH.0 {
            " * (the 2048th — twothousandfortyeighth's step)"
        } else if w == THE_1024TH.0 {
            " * (the 1024th — onethousandtwentyfourth's step)"
        } else if w == THE_512TH.0 {
            " * (the 512th — fivehundredtwelfth's step)"
        } else if w == THE_256TH.0 {
            " * (the 256th — twohundredfiftysixth's step)"
        } else if w == THE_128TH.0 {
            " * (the 128th — hundredtwentyeighth's step)"
        } else if w == CENTRE.0 {
            " (centre — the tower)"
        } else if w == NEW_MID {
            " + (the 4096th — inside the last 2048th)"
        } else if w == SECOND_LEDGE.0 {
            " * (the second ledge — sixtyfourth's step)"
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
        && cell_at(cells, TREAD.0).perfect == TREAD_PERFECT
        && cell_at(cells, SECOND_LEDGE.0).perfect == SECOND_LEDGE_PERFECT
        && cell_at(cells, THE_128TH.0).perfect == MID_PERFECT
        && cell_at(cells, THE_256TH.0).perfect == STEP256_PERFECT
        && cell_at(cells, THE_512TH.0).perfect == STEP512_PERFECT
        && cell_at(cells, THE_1024TH.0).perfect == STEP1024_PERFECT
        && cell_at(cells, THE_2048TH.0).perfect == STEP2048_PERFECT
        && cell_at(cells, LOW.0).perfect == LOW_PERFECT;
    let shape = shape_verdict(cells);
    let centre_stands = cells.iter().all(|c| c.longest_episode <= CENTRE.1);
    let mid = cell_at(cells, NEW_MID);
    let mid_perfect = mid.perfect;
    let counts_recover = mid_perfect > LOW_PERFECT && mid_perfect <= STEP2048_PERFECT;
    let west = cell_at(cells, -1.375).longest_episode;
    let second_tallest = {
        let mut ls: Vec<usize> = cells.iter().map(|c| c.longest_episode).collect();
        ls.sort_unstable();
        ls[ls.len() - 2]
    };
    let shoulder_intact = west == 59 && second_tallest == 59;
    let row_mean = cells.iter().map(|c| c.longest_episode as f64).sum::<f64>() / cells.len() as f64;
    println!("  shape verdict: {shape} (row mean longest {row_mean:.1}; the 4096th reads longest {}, perfect {})", mid.longest_episode, mid_perfect);
    println!("  verdicts across {} cells: TRUE LOCK {}", cells.len(), true_locks);
    println!("  predictions: P1 no TRUE LOCK — {}; P2 known cells reproduce row 37 / 59 / 68 / 51 / 45 / 36 / 46 / 25 / 36 / 47 / 21 / 35 / 13 with centre perfect 104, shelf perfect {SHELF_PERFECT}, ledge perfect {LEDGE_PERFECT}, tread perfect {TREAD_PERFECT}, second-ledge perfect {SECOND_LEDGE_PERFECT}, 128th perfect {MID_PERFECT}, 256th perfect {STEP256_PERFECT}, 512th perfect {STEP512_PERFECT}, 1024th perfect {STEP1024_PERFECT}, 2048th perfect {STEP2048_PERFECT} and foot perfect {LOW_PERFECT} — {}; P3 fall halves at the new grain (4096th exactly 24, the midpoint of 35 and 13; measured {}) — {}; P4 centre stands (no cell above {}) — {}; P5 count holds between foot and 2048th (4096th perfect above the foot {LOW_PERFECT} and at or below the 2048th {STEP2048_PERFECT}; measured {mid_perfect}) — {}; P6 western shoulder intact (west eighth 59 and second-tallest; measured {west} / {second_tallest}) — {}",
        if true_locks == 0 { "HELD" } else { "CRACKED" },
        if anchors_hold { "HELD" } else { "CRACKED" },
        mid.longest_episode,
        if mid.longest_episode == 24 { "HELD" } else { "CRACKED" },
        CENTRE.1,
        if centre_stands { "HELD" } else { "CRACKED" },
        if counts_recover { "HELD" } else { "CRACKED" },
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
        assert_eq!(run_cell(SECOND_LEDGE.0, TOWER_DELAY).perfect, SECOND_LEDGE_PERFECT);
        assert_eq!(run_cell(THE_128TH.0, TOWER_DELAY).perfect, MID_PERFECT);
        assert_eq!(run_cell(THE_256TH.0, TOWER_DELAY).perfect, STEP256_PERFECT);
        assert_eq!(run_cell(THE_512TH.0, TOWER_DELAY).perfect, STEP512_PERFECT);
        assert_eq!(run_cell(THE_1024TH.0, TOWER_DELAY).perfect, STEP1024_PERFECT);
        assert_eq!(run_cell(THE_2048TH.0, TOWER_DELAY).perfect, STEP2048_PERFECT);
        assert_eq!(run_cell(LOW.0, TOWER_DELAY).perfect, LOW_PERFECT);
    }

    #[test]
    fn survey_is_the_declared_row() {
        assert_eq!(WEIGHTS, [-1.5, -1.375, -1.25, -1.125, -1.0625, -1.03125, -1.015625, -1.0078125, -1.00390625, -1.001953125, -1.0009765625, -1.00048828125, -1.000244140625, -1.0]);
        assert_eq!(TOWER_DELAY, 12);
        assert_eq!(CENTRE, (-1.25, 68));
        assert_eq!(SHELF, (-1.125, 51));
        assert_eq!(LEDGE, (-1.0625, 45));
        assert_eq!(TREAD, (-1.03125, 36));
        assert_eq!(SECOND_LEDGE, (-1.015625, 46));
        assert_eq!(LOW, (-1.0, 13));
        // The new cell is the exact midpoint of the last
        // 2048th: halfway between the 2048th and the foot.
        assert_eq!(THE_128TH, (-1.0078125, 25));
        assert_eq!(THE_256TH, (-1.00390625, 36));
        assert_eq!(THE_512TH, (-1.001953125, 47));
        assert_eq!(THE_1024TH, (-1.0009765625, 21));
        assert_eq!(THE_2048TH, (-1.00048828125, 35));
        assert_eq!(NEW_MID, (THE_2048TH.0 + LOW.0) / 2.0);
        assert_eq!(ANCHORS.len(), 13);
        for &w in &WEIGHTS {
            let known = ANCHORS.iter().any(|&(aw, _)| aw == w);
            assert!(known || w == NEW_MID, "cell {w} neither known nor the new 4096th");
        }
        // The fall lines, declared before the run: within 3
        // of the 2048th is a hold, within 3 of the foot is a
        // whole fall; between them stands the halving band
        // 17..=31, centred on 24, the midpoint of 35 and 13.
        assert_eq!(TREAD_HOLD_LINE, THE_2048TH.1 - 3);
        assert_eq!(FALL_LINE, LOW.1 + 3);
        assert!(FALL_LINE < TREAD_HOLD_LINE);
        assert_eq!(PLATEAU_LINE, 55);
        assert_eq!(ISOLATION_LINE, 34);
    }

    #[test]
    fn measured_survey_is_pinned_as_found() {
        // The measured row, pinned so the record cannot heal:
        // at delay 12, weights -1.50 / -1.375 / -1.25 / -1.125
        // / -1.0625 / -1.03125 / -1.015625 / -1.0078125 /
        // -1.00390625 / -1.001953125 / -1.0009765625 /
        // -1.00048828125 / -1.000244140625 / -1.00 read longest
        // 37 / 59 / 68 / 51 / 45 / 36 / 46 / 25 / 36 / 47 / 21 /
        // 35 / 63 / 13.
        // P1, P2, P4, P5 held; P3 and P6 cracked. The 4096th
        // (-1.000244140625) reads longest 63: not the halving
        // point 24 this planting named, not a hold near 35 -
        // twenty-eight above the 2048th entire, five below the
        // centre itself, second-tallest in the row. The fall
        // that reversed at 2048th grain does not merely stand
        // at 4096th grain; it grows. What remains, 63 -> 13,
        // is 50 - larger than the whole interval this
        // instrument priced, a sixth refinancing, and the
        // largest remainder this family has ever carried out
        // of a magnification. Its perfect count is 88 - inside
        // the declared band 75..=90 (P5 held): the count
        // settled between foot and 2048th exactly as priced,
        // while the run, priced by the same planting to halve,
        // climbed past both its neighbours. The two ledgers
        // have parted at the smallest grain yet: count
        // obedient, run ascending. And the western shoulder
        // (P6) falls not because the west moved - (-1.375, 12)
        // still reads 59 entire - but because the east's new
        // cell stands above it: second place changed hands
        // without the west losing a beat. The shape verdict
        // is REVERSAL GROWS AT 4096TH GRAIN.
        let cells = run_survey();
        let longest = |w: f32| cell_at(&cells, w).longest_episode;
        assert_eq!(longest(-1.5), 37);
        assert_eq!(longest(-1.375), 59);
        assert_eq!(longest(-1.25), 68);
        assert_eq!(longest(-1.125), 51);
        assert_eq!(longest(-1.0625), 45);
        assert_eq!(longest(-1.03125), 36);
        assert_eq!(longest(-1.015625), 46);
        assert_eq!(longest(-1.0078125), 25);
        assert_eq!(longest(-1.00390625), 36);
        assert_eq!(longest(-1.001953125), 47);
        assert_eq!(longest(-1.0009765625), 21);
        assert_eq!(longest(-1.00048828125), 35);
        assert_eq!(longest(-1.000244140625), 63);
        assert_eq!(longest(-1.0), 13);
        assert_eq!(cell_at(&cells, -1.25).perfect, 104);
        assert_eq!(cell_at(&cells, -1.125).perfect, 91);
        assert_eq!(cell_at(&cells, -1.0625).perfect, 98);
        assert_eq!(cell_at(&cells, -1.03125).perfect, 88);
        assert_eq!(cell_at(&cells, -1.015625).perfect, 93);
        assert_eq!(cell_at(&cells, -1.0078125).perfect, 68);
        assert_eq!(cell_at(&cells, -1.00390625).perfect, 94);
        assert_eq!(cell_at(&cells, -1.001953125).perfect, 94);
        assert_eq!(cell_at(&cells, -1.0009765625).perfect, 58);
        assert_eq!(cell_at(&cells, -1.00048828125).perfect, 90);
        assert_eq!(cell_at(&cells, -1.000244140625).perfect, 88);
        assert_eq!(cell_at(&cells, -1.0).perfect, 74);
        assert_eq!(shape_verdict(&cells), "REVERSAL GROWS AT 4096TH GRAIN");
    }

    #[test]
    fn no_surveyed_cell_claims_a_true_lock() {
        let cells = run_survey();
        assert_eq!(cells.len(), 14);
        for c in &cells {
            assert_ne!(c.verdict, "TRUE LOCK 7:5",
                "cell ({:+.4}, {}) claims a lock", c.weight, c.delay);
        }
    }
}
