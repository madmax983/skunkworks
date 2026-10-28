//! # Widow Entrainment 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! Everything in this garden ticks, but almost nothing keeps time
//! together. This instrument takes two neurons whose *intrinsic*
//! periods differ — a Regular Spiking driver fed a strong current,
//! a Fast Spiking follower fed a weaker one — and joins them with a
//! single synapse whose weight the Widow raises tooth by tooth.
//!
//! At no coupling the follower keeps its own period and its phase
//! against the driver walks, beat by beat, until it has slipped a
//! whole cycle: drift, counted. At weak coupling it still wanders.
//! At strong coupling the driver's kick lands early enough in the
//! follower's cycle, every cycle, that the follower surrenders its
//! own period and fires once per driver beat at a fixed phase:
//! entrainment, counted. Sync that cannot be lost was never sync;
//! here it is lost twice before it is won.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// The driver's constant drive current. A Regular Spiking neuron
/// under this current keeps the driver's intrinsic period
/// (measured solo in the ledger, never assumed).
const DRIVER_CURRENT: f32 = 12.0;
/// The follower's constant drive current. Weaker, and delivered to a
/// Fast Spiking neuron (different recovery machinery: a=0.1, d=2),
/// so its intrinsic period is its own — near the driver's, not equal.
const FOLLOWER_CURRENT: f32 = 4.0;
/// Total ticks per regime (1 tick = 1 step of the parent model ≈ 1 ms).
const TICKS: usize = 6_000;
/// Ticks thrown away at the start of each regime while the pair
/// settles onto whatever relationship the coupling allows.
const SETTLE_TICKS: usize = 1_500;
/// Coupling weights swept, weakest first. The boundary between
/// wandering and locking lives between the last two teeth.
const COUPLINGS: [(&str, f32); 3] = [
    ("uncoupled", 0.0),
    ("weak thread", 1.0),
    ("strong thread", 35.0),
];

/// One clock measured alone: how many times it struck, and the mean
/// interval between strikes, over the measurement window.
#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

/// One coupling regime, measured.
#[derive(Debug, Clone)]
struct RegimeLedger {
    name: &'static str,
    weight: f32,
    driver_spikes: usize,
    follower_spikes: usize,
    driver_mean_isi: f64,
    follower_mean_isi: f64,
    /// Driver cycles (intervals between consecutive driver spikes)
    /// containing exactly one follower spike.
    matched_cycles: usize,
    /// Driver cycles containing no follower spike at all.
    missed_beats: usize,
    /// Extra follower spikes beyond the first inside a driver cycle.
    extra_beats: usize,
    /// Circular dispersion of follower phase within the driver cycle:
    /// 0.0 = every follower spike at the identical phase (a lock),
    /// approaching 1.0 = phase smeared around the whole cycle (drift).
    phase_dispersion: f64,
    /// Mean follower phase within the driver cycle, in [0, 1).
    mean_phase: f64,
    locked: bool,
}

fn build_pair(weight: f32) -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    // The driver keeps the parent's default Regular Spiking machinery.
    // The follower is re-bodied as Fast Spiking: same equations,
    // different recovery constants — a genuinely different clock,
    // not the same clock told to step slower.
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    if weight != 0.0 {
        net.add_synapse(driver, follower, weight);
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
    SoloClock { spikes: times.len(), mean_isi: mean_isi(&times) }
}

fn mean_isi(times: &[usize]) -> f64 {
    if times.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

/// Run one coupling regime and take its ledger.
fn run_regime(name: &'static str, weight: f32) -> RegimeLedger {
    let mut net = build_pair(weight);
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

    // Walk the driver cycles; count what the follower did inside each.
    let mut matched = 0usize;
    let mut missed = 0usize;
    let mut extra = 0usize;
    let mut phases: Vec<f64> = Vec::new();
    for w in driver_times.windows(2) {
        let (start, end) = (w[0], w[1]);
        let inside: Vec<usize> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .collect();
        match inside.len() {
            0 => missed += 1,
            1 => matched += 1,
            n => {
                matched += 1;
                extra += n - 1;
            }
        }
        for f in inside {
            phases.push((f - start) as f64 / (end - start) as f64);
        }
    }

    // Circular statistics: phases live on a ring, not a line.
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    for p in &phases {
        let ang = p * std::f64::consts::TAU;
        sx += ang.cos();
        sy += ang.sin();
    }
    let n = phases.len().max(1) as f64;
    let r = ((sx / n).powi(2) + (sy / n).powi(2)).sqrt();
    let dispersion = 1.0 - r;
    let mut mean_phase = (sy / n).atan2(sx / n) / std::f64::consts::TAU;
    if mean_phase < 0.0 {
        mean_phase += 1.0;
    }

    let cycles = driver_times.len().saturating_sub(1).max(1);
    let locked = missed == 0 && extra == 0 && matched == cycles && dispersion < 0.05;

    RegimeLedger {
        name,
        weight,
        driver_spikes: driver_times.len(),
        follower_spikes: follower_times.len(),
        driver_mean_isi: mean_isi(&driver_times),
        follower_mean_isi: mean_isi(&follower_times),
        matched_cycles: matched,
        missed_beats: missed,
        extra_beats: extra,
        phase_dispersion: dispersion,
        mean_phase,
        locked,
    }
}

fn run_all() -> (SoloClock, SoloClock, Vec<RegimeLedger>) {
    let driver_solo = measure_solo(false, DRIVER_CURRENT);
    let follower_solo = measure_solo(true, FOLLOWER_CURRENT);
    let regimes = COUPLINGS.iter().map(|&(name, w)| run_regime(name, w)).collect();
    (driver_solo, follower_solo, regimes)
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, regimes: &[RegimeLedger]) {
    println!("widow-entrainment headless: two clocks, one thread, counted in beats");
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
            "  intrinsic ratio (follower/driver): {:.3} — near, and not equal; that gap is the drift",
            follower.mean_isi / driver.mean_isi
        );
    }
    for r in regimes {
        println!(
            "  regime '{}' (synapse weight {:.1}): driver {} spikes (period {:.2}), follower {} spikes (period {:.2})",
            r.name, r.weight, r.driver_spikes, r.driver_mean_isi, r.follower_spikes, r.follower_mean_isi
        );
        println!(
            "    cycles matched {}/{}, missed beats {}, extra beats {}, mean phase {:.3}, phase dispersion {:.4} — {}",
            r.matched_cycles,
            r.matched_cycles + r.missed_beats,
            r.missed_beats,
            r.extra_beats,
            r.mean_phase,
            r.phase_dispersion,
            if r.locked { "LOCKED" } else { "DRIFTING" }
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
    fn the_two_clocks_keep_different_time_alone() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        assert!(driver.spikes > 50, "driver barely ticked: {}", driver.spikes);
        assert!(follower.spikes > 50, "follower barely ticked: {}", follower.spikes);
        let ratio = follower.mean_isi / driver.mean_isi;
        assert!(
            (ratio - 1.0).abs() > 0.05,
            "the clocks agree alone (ratio {ratio:.3}); there is no drift to forgive"
        );
    }

    #[test]
    fn uncoupled_the_follower_drifts() {
        let r = run_regime("uncoupled", 0.0);
        assert!(!r.locked, "an uncoupled follower claimed a lock: {r:?}");
        assert!(
            r.missed_beats + r.extra_beats > 0,
            "no beat was missed or doubled while uncoupled — the phases never slipped: {r:?}"
        );
        assert!(
            r.phase_dispersion > 0.2,
            "uncoupled phase should smear around the cycle, dispersion {:.4}",
            r.phase_dispersion
        );
    }

    #[test]
    fn strong_coupling_entrains() {
        let r = run_regime("strong thread", 35.0);
        assert!(r.locked, "the strong thread did not lock: {r:?}");
        assert_eq!(r.missed_beats, 0);
        assert_eq!(r.extra_beats, 0);
        // The follower surrendered its own period for the driver's.
        assert!(
            (r.follower_mean_isi - r.driver_mean_isi).abs() < 0.5,
            "locked, yet periods differ: follower {:.2} vs driver {:.2}",
            r.follower_mean_isi,
            r.driver_mean_isi
        );
    }

    #[test]
    fn entrainment_is_a_boundary_not_a_switch() {
        // Somewhere between the weak tooth and the strong tooth the
        // relationship changes: dispersion must fall as coupling rises.
        let weak = run_regime("weak thread", 1.0);
        let strong = run_regime("strong thread", 35.0);
        assert!(!weak.locked, "the weak thread already holds — the boundary is a lie: {weak:?}");
        assert!(
            strong.phase_dispersion < weak.phase_dispersion,
            "tightening the thread did not tighten the phase: weak {:.4} vs strong {:.4}",
            weak.phase_dispersion,
            strong.phase_dispersion
        );
    }
}
