//! # Widow Subharmonic 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! The Widow's first instrument (`widow-entrainment`) studied the
//! 1:1 boundary and set one clock aside: a Fast Spiking neuron at
//! I=6 keeps a period a hair off half the Regular Spiking driver's
//! — near 2:1, not at it, and excitatory coupling "only hurries
//! it." This instrument picks that clock back up and asks the
//! harder question properly: can one synapse hold a follower at
//! *two beats per driver beat* — a subharmonic lock — when its
//! natural pair of periods runs slightly long for the driver's
//! cycle?
//!
//! The follower's solo period (measured, never assumed) is 20.37
//! ticks; two of them make 40.75 against the driver's 39.62. Every
//! driver cycle the follower's second beat lands ~1.13 ticks later
//! than the last, so uncoupled its phase walks the whole ring.
//! The driver's kick, arriving once per cycle, can in principle
//! take that lateness back — hurrying is precisely the medicine
//! here — but the kick reaches only one of the follower's two
//! beats per cycle, and where it lands in the follower's own cycle
//! decides whether it corrects or merely shoves.
//!
//! A 2:1 lock is counted beat by beat, each on its own ring: the
//! first follower beat in every driver cycle must keep one phase,
//! and the second beat another (the two need not be evenly spaced
//! — a lock is a kept pattern, not a metronome's symmetry). The
//! verdict requires exactly two follower beats in every driver
//! cycle AND both beat dispersions under 0.05. Anything less is a
//! wander wearing a costume.
//!
//! What the sweep found, counted in the ledger below: weight alone
//! almost holds (at 10.0 the follower's period is dragged to
//! exactly half the driver's, 19.81, but four cycles in 112 still
//! slip a beat); more weight breaks it the other way, hurrying the
//! follower into crowded cycles. The tooth that holds is the same
//! weight *timed* — a five-tick synaptic delay moves the kick into
//! the half of the follower's cycle where it corrects instead of
//! shoves, and every cycle closes with exactly two beats, both
//! phases kept. It was never the strength. It was when the kick
//! arrived.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// The driver's constant drive current. A Regular Spiking neuron
/// under this current keeps the driver's intrinsic period
/// (measured solo in the ledger, never assumed).
const DRIVER_CURRENT: f32 = 12.0;
/// The follower's constant drive current. A Fast Spiking neuron at
/// this current keeps a period near — not at — half the driver's:
/// the commensurate trap from the Widow's pulse survey, now the
/// subject rather than the warning.
const FOLLOWER_CURRENT: f32 = 6.0;
/// Total ticks per regime (1 tick = 1 step of the parent model ≈ 1 ms).
const TICKS: usize = 6_000;
/// Ticks thrown away at the start of each regime while the pair
/// settles onto whatever relationship the coupling allows.
const SETTLE_TICKS: usize = 1_500;
/// The timed thread's synaptic delay, in ticks. Five ticks moves
/// the driver's kick from the follower's early recovery — where it
/// only shoves the next beat sooner and destabilises the pair —
/// into the window where it takes back exactly the pair's lateness.
const TIMED_DELAY: usize = 5;
/// Coupling regimes swept, in the order the Widow turned the key:
/// (name, synapse weight, synaptic delay in ticks).
const COUPLINGS: [(&str, f32, usize); 5] = [
    ("uncoupled", 0.0, 0),
    ("fine thread", 2.0, 0),
    ("taut thread", 10.0, 0),
    ("timed thread", 10.0, TIMED_DELAY),
    ("iron thread", 35.0, 0),
];

/// One clock measured alone: how many times it struck, and the mean
/// interval between strikes, over the measurement window.
#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

/// One coupling regime, measured beat by beat.
#[derive(Debug, Clone)]
struct RegimeLedger {
    name: &'static str,
    weight: f32,
    delay: usize,
    driver_spikes: usize,
    follower_spikes: usize,
    driver_mean_isi: f64,
    follower_mean_isi: f64,
    /// Driver cycles containing exactly two follower spikes.
    perfect_cycles: usize,
    /// Driver cycles containing fewer than two follower spikes.
    short_cycles: usize,
    /// Driver cycles containing more than two follower spikes.
    crowded_cycles: usize,
    /// Circular dispersion of the first follower beat's phase
    /// within the driver cycle (0 = kept exactly, →1 = smeared).
    first_dispersion: f64,
    /// Circular dispersion of the second follower beat's phase.
    second_dispersion: f64,
    /// Mean phase of the first / second follower beat, in [0, 1).
    first_mean_phase: f64,
    second_mean_phase: f64,
    locked: bool,
}

fn build_pair(weight: f32, delay: usize) -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    // The driver keeps the parent's default Regular Spiking machinery.
    // The follower is re-bodied as Fast Spiking: same equations,
    // different recovery constants — a genuinely different clock.
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

/// Measure one neuron alone (no synapse) under a constant current.
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

/// Circular dispersion and mean of a set of phases in [0, 1).
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

/// Run one coupling regime and take its ledger.
fn run_regime(name: &'static str, weight: f32, delay: usize) -> RegimeLedger {
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

    // Walk the driver cycles; count the follower's beats inside
    // each, and record the first and second beats' phases on their
    // own rings.
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
    let cycles = driver_times.len().saturating_sub(1).max(1);
    let locked = perfect == cycles && first_dispersion < 0.05 && second_dispersion < 0.05;

    RegimeLedger {
        name,
        weight,
        delay,
        driver_spikes: driver_times.len(),
        follower_spikes: follower_times.len(),
        driver_mean_isi: mean_isi(&driver_times),
        follower_mean_isi: mean_isi(&follower_times),
        perfect_cycles: perfect,
        short_cycles: short,
        crowded_cycles: crowded,
        first_dispersion,
        second_dispersion,
        first_mean_phase,
        second_mean_phase,
        locked,
    }
}

fn run_all() -> (SoloClock, SoloClock, Vec<RegimeLedger>) {
    let driver_solo = measure_solo(false, DRIVER_CURRENT);
    let follower_solo = measure_solo(true, FOLLOWER_CURRENT);
    let regimes = COUPLINGS
        .iter()
        .map(|&(name, w, d)| run_regime(name, w, d))
        .collect();
    (driver_solo, follower_solo, regimes)
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, regimes: &[RegimeLedger]) {
    println!("widow-subharmonic headless: two beats against one, each counted on its own ring");
    println!("  window: {TICKS} ticks per regime, first {SETTLE_TICKS} discarded as settling");
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
    for r in regimes {
        let cycles = r.perfect_cycles + r.short_cycles + r.crowded_cycles;
        println!(
            "  regime '{}' (synapse weight {:.1}, delay {} ticks): driver {} spikes (period {:.2}), follower {} spikes (period {:.2})",
            r.name, r.weight, r.delay, r.driver_spikes, r.driver_mean_isi, r.follower_spikes,
            r.follower_mean_isi
        );
        println!(
            "    driver cycles with exactly two follower beats {}/{}, short {}, crowded {}, first beat phase {:.3} (dispersion {:.4}), second beat phase {:.3} (dispersion {:.4}) — {}",
            r.perfect_cycles,
            cycles,
            r.short_cycles,
            r.crowded_cycles,
            r.first_mean_phase,
            r.first_dispersion,
            r.second_mean_phase,
            r.second_dispersion,
            if r.locked { "LOCKED 2:1" } else { "WALKING" }
        );
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let (driver, follower, regimes) = run_all();
    print_ledger(&driver, &follower, &regimes);
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
        assert!(
            (driver.mean_isi - 39.62).abs() < 1.0,
            "driver period {}",
            driver.mean_isi
        );
        assert!(
            (follower.mean_isi - 20.37).abs() < 1.0,
            "follower period {}",
            follower.mean_isi
        );
        // The pair runs long: near 2:1, measurably not at it.
        let pair_ratio = (2.0 * follower.mean_isi) / driver.mean_isi;
        assert!(
            pair_ratio > 1.01 && pair_ratio < 1.06,
            "pair ratio {pair_ratio}"
        );
    }

    #[test]
    fn uncoupled_follower_walks() {
        let r = run_regime("uncoupled", 0.0, 0);
        assert!(!r.locked, "an uncoupled pair cannot be locked: {r:?}");
        assert!(
            r.short_cycles + r.crowded_cycles > 0
                || r.first_dispersion >= 0.05
                || r.second_dispersion >= 0.05,
            "the uncoupled follower must slip cycles or smear phase: {r:?}"
        );
    }

    #[test]
    fn timed_thread_locks_two_to_one() {
        let r = run_regime("timed thread", 10.0, TIMED_DELAY);
        let cycles = r.perfect_cycles + r.short_cycles + r.crowded_cycles;
        assert_eq!(r.perfect_cycles, cycles, "every cycle two beats: {r:?}");
        assert!(r.locked, "the timed thread must lock: {r:?}");
        assert!(
            (r.follower_mean_isi - r.driver_mean_isi / 2.0).abs() < 0.5,
            "follower period {} against half the driver's {}",
            r.follower_mean_isi,
            r.driver_mean_isi
        );
    }

    #[test]
    fn iron_thread_breaks_the_lock_the_other_way() {
        let r = run_regime("iron thread", 35.0, 0);
        assert!(!r.locked, "the iron thread must not lock: {r:?}");
        assert!(
            r.crowded_cycles > 0,
            "too much hurrying crowds cycles: {r:?}"
        );
    }

    #[test]
    fn taut_thread_almost_holds_but_slips() {
        let r = run_regime("taut thread", 10.0, 0);
        assert!(!r.locked, "the undelayed taut thread must slip: {r:?}");
        assert!(
            r.follower_mean_isi < 20.37,
            "the kick still pulls the period in: {}",
            r.follower_mean_isi
        );
    }
}
