//! # Widow Island 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model); subject: the 2:1 lock
//! found by `widow-subharmonic`.
//!
//! That instrument reported a single tooth: weight 10.0, delay 5
//! ticks, 112/112 cycles with exactly two follower beats, both
//! dispersions under 0.03 — LOCKED 2:1. Its README went further
//! and called the timed thread "the only tooth that holds." The
//! Widow does not forgive an unmeasured superlative, least of all
//! her own. This instrument is the correction, counted properly:
//! the whole (weight, delay) plane for the same pair of clocks,
//! swept on a grid — 12 weights × 13 delays, 156 regimes, every
//! one run for the parent instrument's full window and judged by
//! the parent instrument's exact definition of a lock (exactly
//! two follower beats in every driver cycle AND both beat
//! dispersions under 0.05).
//!
//! The pair, unchanged: a Regular Spiking driver at I=12 (solo
//! period 39.62 ticks) and a Fast Spiking follower at I=6 (solo
//! period 20.37; its pair makes 40.75 against the driver's 39.62,
//! ratio 1.029, running long by 1.13 ticks per driver cycle).
//!
//! What the map shows, counted in the ledger below: the tooth
//! was not a point. It was an island. A rectangular plateau —
//! weights 5.0, 7.5, and 10.0 across every swept delay from 2 to
//! 12 ticks — locks entire (27 of 27 cells). Around it, a fringe:
//! weight 10.0 also locks at delay 1; weight 3.0 locks at five
//! scattered delays with holes between them (it locks at delay 5
//! and slips at delay 4); and one stray tooth stands alone at
//! weight 12.5, delay 10, with walking water on every side. The
//! shores are as clean as the plateau: no weight locks at delay
//! 0 (timing is necessary — an undelayed kick never holds, at any
//! strength), nothing locks at delay 15 or 20, and nothing at
//! weight 15.0 or above locks anywhere. 34 of 156 cells lock.
//!
//! The lesson is not that the first survey was careless — it
//! swept sparsely and reported what it swept, then overstated
//! it. The lesson is that a boundary drawn from a handful of
//! points is a coastline guessed from a handful of landings.
//! Sweep the plane. Count every cell. Forgive nothing that
//! drifts — including a README.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// The driver's constant drive current (Regular Spiking, solo
/// period measured in the ledger, never assumed).
const DRIVER_CURRENT: f32 = 12.0;
/// The follower's constant drive current (Fast Spiking at I=6:
/// the near-2:1 clock from `widow-subharmonic`).
const FOLLOWER_CURRENT: f32 = 6.0;
/// Total ticks per regime — the parent instrument's window.
const TICKS: usize = 6_000;
/// Ticks discarded at the start of each regime as settling.
const SETTLE_TICKS: usize = 1_500;
/// A lock requires both beat dispersions under this line — the
/// parent instrument's definition, inherited unchanged so the
/// two ledgers can be read side by side.
const LOCK_DISPERSION: f64 = 0.05;
/// The weight axis of the survey, in the order the Widow turned
/// the key: from no thread at all to the iron thread that crowds.
const WEIGHTS: [f32; 12] = [0.0, 1.0, 2.0, 3.0, 5.0, 7.5, 10.0, 12.5, 15.0, 20.0, 25.0, 35.0];
/// The delay axis of the survey, in ticks: from the undelayed
/// kick to arrivals so late they miss the follower's cycle.
const DELAYS: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 15, 20];

/// One clock measured alone.
#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

/// One cell of the (weight, delay) plane, measured beat by beat.
#[derive(Debug, Clone)]
struct Cell {
    weight: f32,
    delay: usize,
    perfect_cycles: usize,
    short_cycles: usize,
    crowded_cycles: usize,
    first_dispersion: f64,
    second_dispersion: f64,
    first_mean_phase: f64,
    second_mean_phase: f64,
    follower_mean_isi: f64,
    locked: bool,
}

impl Cell {
    fn cycles(&self) -> usize {
        self.perfect_cycles + self.short_cycles + self.crowded_cycles
    }
}

fn build_pair(weight: f32, delay: usize) -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    if weight != 0.0 {
        net.add_synapse_with_delay(driver, follower, weight, delay);
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
    SoloClock {
        spikes: times.len(),
        mean_isi: mean_isi(&times),
    }
}

fn mean_isi(times: &[usize]) -> f64 {
    if times.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

fn circular(phases: &[f64]) -> (f64, f64) {
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    for p in phases {
        let ang = p * std::f64::consts::TAU;
        sx += ang.cos();
        sy += ang.sin();
    }
    let n = phases.len().max(1) as f64;
    let r = ((sx / n).powi(2) + (sy / n).powi(2)).sqrt();
    let mut mean = (sy / n).atan2(sx / n) / std::f64::consts::TAU;
    if mean < 0.0 {
        mean += 1.0;
    }
    (1.0 - r, mean)
}

/// Run one cell of the plane and judge it by the parent's rule.
fn run_cell(weight: f32, delay: usize) -> Cell {
    let mut net = build_pair(weight, delay);
    let mut driver_times: Vec<usize> = Vec::new();
    let mut follower_times: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[DRIVER_CURRENT, FOLLOWER_CURRENT]);
        if t >= SETTLE_TICKS {
            if net.is_spiking(0) {
                driver_times.push(t);
            }
            if net.is_spiking(1) {
                follower_times.push(t);
            }
        }
    }
    let mut perfect = 0usize;
    let mut short = 0usize;
    let mut crowded = 0usize;
    let mut first_phases: Vec<f64> = Vec::new();
    let mut second_phases: Vec<f64> = Vec::new();
    for w in driver_times.windows(2) {
        let (start, end) = (w[0], w[1]);
        let inside: Vec<usize> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .collect();
        match inside.len() {
            2 => perfect += 1,
            n if n < 2 => short += 1,
            _ => crowded += 1,
        }
        if let Some(&f) = inside.first() {
            first_phases.push((f - start) as f64 / (end - start) as f64);
        }
        if let Some(&f) = inside.get(1) {
            second_phases.push((f - start) as f64 / (end - start) as f64);
        }
    }
    let (first_dispersion, first_mean_phase) = circular(&first_phases);
    let (second_dispersion, second_mean_phase) = circular(&second_phases);
    let cycles = perfect + short + crowded;
    let locked = cycles > 0
        && perfect == cycles
        && first_dispersion < LOCK_DISPERSION
        && second_dispersion < LOCK_DISPERSION;
    Cell {
        weight,
        delay,
        perfect_cycles: perfect,
        short_cycles: short,
        crowded_cycles: crowded,
        first_dispersion,
        second_dispersion,
        first_mean_phase,
        second_mean_phase,
        follower_mean_isi: mean_isi(&follower_times),
        locked,
    }
}

fn run_grid() -> Vec<Cell> {
    let mut cells = Vec::new();
    for &w in &WEIGHTS {
        for &d in &DELAYS {
            cells.push(run_cell(w, d));
        }
    }
    cells
}

fn cell_at<'a>(cells: &'a [Cell], weight: f32, delay: usize) -> &'a Cell {
    cells
        .iter()
        .find(|c| c.weight == weight && c.delay == delay)
        .expect("grid cell exists")
}

/// The rectangular heart of the island: the plateau weights and
/// the delay span they lock across entire.
fn is_plateau(weight: f32, delay: usize) -> bool {
    [5.0, 7.5, 10.0].contains(&weight) && [2, 3, 4, 5, 6, 7, 8, 10, 12].contains(&delay)
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, cells: &[Cell]) {
    println!("widow-island headless: the 2:1 lock, surveyed whole — every cell of the (weight, delay) plane counted");
    println!("  window: {TICKS} ticks per regime, first {SETTLE_TICKS} discarded as settling; a lock = exactly two follower beats in every driver cycle AND both dispersions < {LOCK_DISPERSION}");
    println!(
        "  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks",
        driver.spikes, driver.mean_isi
    );
    println!(
        "  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks",
        follower.spikes, follower.mean_isi
    );
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!(
            "  pair period (2 x follower): {:.2} ticks against driver {:.2} — ratio {:.3}, near 2:1 and not at it; the follower's pair runs long by {:.2} ticks per driver cycle",
            2.0 * follower.mean_isi,
            driver.mean_isi,
            (2.0 * follower.mean_isi) / driver.mean_isi,
            2.0 * follower.mean_isi - driver.mean_isi
        );
    }
    println!("  the map (L = LOCKED 2:1, . = WALKING; rows are synapse weight, columns synaptic delay in ticks):");
    print!("    weight\\delay");
    for d in DELAYS {
        print!(" {d:>3}");
    }
    println!();
    for &w in &WEIGHTS {
        print!("    {w:12.1}");
        for &d in &DELAYS {
            let c = cell_at(cells, w, d);
            print!(" {}", if c.locked { "  L" } else { "  ." });
        }
        let row_locks = cells.iter().filter(|c| c.weight == w && c.locked).count();
        println!("   ({row_locks} locking)");
    }
    let total = cells.len();
    let locking = cells.iter().filter(|c| c.locked).count();
    let plateau_cells: Vec<&Cell> = cells.iter().filter(|c| is_plateau(c.weight, c.delay)).collect();
    let plateau_locking = plateau_cells.iter().filter(|c| c.locked).count();
    let fringe_locking = cells
        .iter()
        .filter(|c| c.locked && !is_plateau(c.weight, c.delay))
        .count();
    let delay0_locking = cells.iter().filter(|c| c.delay == 0 && c.locked).count();
    let far_delay_locking = cells
        .iter()
        .filter(|c| (c.delay == 15 || c.delay == 20) && c.locked)
        .count();
    let heavy_locking = cells.iter().filter(|c| c.weight >= 15.0 && c.locked).count();
    println!("  locking cells: {locking}/{total}");
    println!(
        "  the plateau (weights 5.0/7.5/10.0 x delays 2..=12): {plateau_locking}/{} cells lock — a plateau, not a tooth",
        plateau_cells.len()
    );
    println!("  fringe locking cells (outside the plateau): {fringe_locking}");
    println!("  shores: locks at delay 0: {delay0_locking}; locks at delay 15 or 20: {far_delay_locking}; locks at weight >= 15.0: {heavy_locking}");
    for (w, d, label) in [
        (10.0, 5, "the parent instrument's tooth, re-counted"),
        (10.0, 0, "the same thread, undelayed"),
        (3.0, 5, "a fringe landing"),
        (3.0, 4, "the hole beside it"),
        (12.5, 10, "the stray tooth"),
        (35.0, 5, "the iron thread, timed — still water"),
    ] {
        let c = cell_at(cells, w, d);
        println!(
            "  anchor cell (weight {w}, delay {d}) — {label}: follower period {:.2}, cycles exactly-two {}/{}, short {}, crowded {}, first beat phase {:.3} (dispersion {:.4}), second beat phase {:.3} (dispersion {:.4}) — {}",
            c.follower_mean_isi,
            c.perfect_cycles,
            c.cycles(),
            c.short_cycles,
            c.crowded_cycles,
            c.first_mean_phase,
            c.first_dispersion,
            c.second_mean_phase,
            c.second_dispersion,
            if c.locked { "LOCKED 2:1" } else { "WALKING" }
        );
    }
    println!("  verdict: the tooth was an island — a 27-cell plateau, a scattered fringe, one stray tooth, and clean shores on every side");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let cells = run_grid();
    print_ledger(&driver, &follower, &cells);
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_clocks_are_near_two_to_one_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        assert!((driver.mean_isi - 39.62).abs() < 1.0, "driver {}", driver.mean_isi);
        assert!((follower.mean_isi - 20.37).abs() < 1.0, "follower {}", follower.mean_isi);
        let pair_ratio = (2.0 * follower.mean_isi) / driver.mean_isi;
        assert!(pair_ratio > 1.01 && pair_ratio < 1.06, "pair ratio {pair_ratio}");
    }

    #[test]
    fn parent_tooth_recounts_as_locked() {
        let c = run_cell(10.0, 5);
        assert!(c.locked, "the parent's tooth must re-count locked: {c:?}");
        assert_eq!(c.perfect_cycles, c.cycles());
    }

    #[test]
    fn plateau_locks_entire() {
        for &w in &[5.0f32, 7.5, 10.0] {
            for &d in &[2usize, 3, 4, 5, 6, 7, 8, 10, 12] {
                let c = run_cell(w, d);
                assert!(c.locked, "plateau cell (weight {w}, delay {d}) must lock: {c:?}");
            }
        }
    }

    #[test]
    fn no_weight_locks_undelayed() {
        for &w in &WEIGHTS {
            let c = run_cell(w, 0);
            assert!(!c.locked, "delay 0 must never lock (weight {w}): {c:?}");
        }
    }

    #[test]
    fn heavy_weights_lock_nowhere() {
        for &w in &[15.0f32, 20.0, 25.0, 35.0] {
            for &d in &DELAYS {
                let c = run_cell(w, d);
                assert!(!c.locked, "weight {w} must lock nowhere (delay {d}): {c:?}");
            }
        }
    }

    #[test]
    fn fringe_has_holes() {
        // The fringe is not a slope: weight 3.0 lands at delay 5
        // and slips at delay 4, one tick apart.
        assert!(run_cell(3.0, 5).locked, "fringe landing (3.0, 5) must lock");
        assert!(!run_cell(3.0, 4).locked, "the hole (3.0, 4) must walk");
    }

    #[test]
    fn stray_tooth_stands_alone() {
        assert!(run_cell(12.5, 10).locked, "the stray tooth (12.5, 10) must lock");
        for &d in &[8usize, 12] {
            assert!(!run_cell(12.5, d).locked, "water beside the stray tooth (12.5, {d}) must walk");
        }
        assert!(!run_cell(15.0, 10).locked, "water above the stray tooth must walk");
    }

    #[test]
    fn uncoupled_walks() {
        let c = run_cell(0.0, 0);
        assert!(!c.locked, "an uncoupled pair cannot be locked: {c:?}");
        assert!(
            c.first_dispersion >= LOCK_DISPERSION || c.second_dispersion >= LOCK_DISPERSION,
            "the uncoupled beats must smear: {c:?}"
        );
    }
}
