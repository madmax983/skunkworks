//! # Widow Episodes 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! The Widow's earlier instruments asked whether two clocks lock,
//! and answered yes or no. This one takes up the pair that refused
//! the question. A Fast Spiking follower at I=5 keeps a solo period
//! of ~26.32 ticks against the Regular Spiking driver's 39.62:
//! three follower beats span 78.97 ticks against two driver cycles'
//! 79.24 — near 3:2, and the follower runs *early* by ~0.27 ticks
//! per super-cycle, so the medicine is inhibition, not hurrying.
//! A prior survey (recorded in widow-island's run) found gentle
//! inhibition holds the pair at the 6,000-tick window and loses it
//! over 12,000–24,000: what survives at long windows is not a lock
//! but *episodes* — runs of consecutive super-cycles in which all
//! three follower beats keep their pattern — separated by slips.
//!
//! So this instrument changes the ledger, not the clocks. Over a
//! 24,000-tick window it counts, per coupling: super-cycles,
//! perfect super-cycles, episodes, the longest episode (in
//! super-cycles and in ticks), and the mean episode length. A true
//! lock — every super-cycle perfect — would still be reported as
//! one, honestly. Anything less is priced in the only currency the
//! pair actually pays: how long agreement lasts before it drifts
//! by a tooth.
//!
//! A super-cycle is one pair of consecutive driver cycles. It is
//! perfect when it holds exactly three follower beats AND each
//! beat's phase (on the super-cycle's own ring) lies within
//! ±0.06 of that beat's median phase for the regime. Count alone
//! would let a slowly rotating pattern pass; phase alone would let
//! a dropped beat pass. Both gates, or it is a slip.
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
/// this current keeps a period near — not at — two-thirds of the
/// driver's: three of its beats run slightly *early* against two
/// driver cycles, which is why the couplings that help here are
/// inhibitory. Excitatory weight on this pair is the wrong
/// medicine, and the ledger keeps one as a control.
const FOLLOWER_CURRENT: f32 = 5.0;
/// Total ticks per regime (1 tick = 1 step of the parent model
/// ≈ 1 ms). Four times the window at which gentle inhibition
/// *appeared* to lock — episodes are only visible at this length.
const TICKS: usize = 24_000;
/// Ticks thrown away at the start of each regime while the pair
/// settles onto whatever relationship the coupling allows.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept: its phase on the
/// super-cycle ring must lie within this distance of the regime's
/// median phase for that beat.
const PHASE_TOL: f64 = 0.06;
/// An episode must reach this many consecutive perfect
/// super-cycles before the verdict calls the regime EPISODIC
/// rather than WALKING; below it, agreement is weather, not
/// climate.
const EPISODIC_FLOOR: usize = 10;

/// Coupling regimes swept, in the order the Widow turned the key:
/// (name, synapse weight, synaptic delay in ticks). Negative
/// weights inhibit — they *delay* the early follower, which is
/// the direction this pair needs corrected.
const COUPLINGS: [(&str, f32, usize); 10] = [
    ("uncoupled", 0.0, 0),
    ("wrong medicine (+2.0, delay 0)", 2.0, 0),
    ("soft hand (-1.0, delay 8)", -1.0, 8),
    ("the surveyed hand (-1.5, delay 8)", -1.5, 8),
    ("firm hand (-2.0, delay 8)", -2.0, 8),
    ("heavy hand (-2.5, delay 8)", -2.5, 8),
    ("crushing hand (-3.5, delay 8)", -3.5, 8),
    ("surveyed hand, no delay (-1.5, delay 0)", -1.5, 0),
    ("surveyed hand, early (-1.5, delay 4)", -1.5, 4),
    ("surveyed hand, late (-1.5, delay 12)", -1.5, 12),
];

/// One clock measured alone: how many times it struck, and the mean
/// interval between strikes, over the measurement window.
#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

/// One coupling regime, measured in episodes.
#[derive(Debug, Clone)]
struct RegimeLedger {
    name: &'static str,
    weight: f32,
    delay: usize,
    driver_spikes: usize,
    follower_spikes: usize,
    follower_mean_isi: f64,
    /// Super-cycles in the window (pairs of driver cycles).
    super_cycles: usize,
    /// Super-cycles holding exactly three beats, all phases kept.
    perfect: usize,
    /// Super-cycles failing on count alone (not three beats).
    count_slips: usize,
    /// Super-cycles with three beats but a phase outside tolerance.
    phase_slips: usize,
    /// Number of maximal runs of consecutive perfect super-cycles.
    episodes: usize,
    /// The longest episode, in super-cycles.
    longest_episode: usize,
    /// Mean episode length, in super-cycles.
    mean_episode: f64,
    /// Median phase of each of the three beats, and the circular
    /// dispersion of each beat's phases across 3-beat super-cycles.
    median_phases: [f64; 3],
    dispersions: [f64; 3],
    /// Mean super-cycle length in ticks (for pricing episodes).
    mean_super_ticks: f64,
    verdict: &'static str,
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

/// Circular distance between two phases in [0, 1).
fn circ_dist(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % 1.0;
    d.min(1.0 - d)
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut v: Vec<f64> = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

/// Circular dispersion of a set of phases in [0, 1).
fn dispersion(phases: &[f64]) -> f64 {
    if phases.is_empty() {
        return 1.0;
    }
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    for p in phases {
        let ang = p * std::f64::consts::TAU;
        sx += ang.cos();
        sy += ang.sin();
    }
    let n = phases.len() as f64;
    let r = ((sx / n).powi(2) + (sy / n).powi(2)).sqrt();
    1.0 - r
}

/// Run one coupling regime and take its episode ledger.
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

    // Super-cycles: windows between every second driver spike.
    // Collect each window's follower beats as phases on its ring.
    let n_super = driver_times.len().saturating_sub(2) / 2;
    let mut windows: Vec<Vec<f64>> = Vec::with_capacity(n_super);
    let mut super_lens: Vec<f64> = Vec::with_capacity(n_super);
    for k in 0..n_super {
        let (start, end) = (driver_times[2 * k], driver_times[2 * k + 2]);
        let beats: Vec<f64> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64)
            .collect();
        windows.push(beats);
        super_lens.push((end - start) as f64);
    }

    // Median phase per beat position, over 3-beat windows only.
    let mut medians = [0.0f64; 3];
    let mut dispersions = [1.0f64; 3];
    for j in 0..3 {
        let phases: Vec<f64> = windows
            .iter()
            .filter(|w| w.len() == 3)
            .map(|w| w[j])
            .collect();
        medians[j] = median(&phases);
        dispersions[j] = dispersion(&phases);
    }

    // Walk the windows: perfect, count slip, or phase slip.
    let mut perfect_flags: Vec<bool> = Vec::with_capacity(windows.len());
    let mut count_slips = 0usize;
    let mut phase_slips = 0usize;
    for w in &windows {
        if w.len() != 3 {
            count_slips += 1;
            perfect_flags.push(false);
        } else if (0..3).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
            phase_slips += 1;
            perfect_flags.push(false);
        } else {
            perfect_flags.push(true);
        }
    }
    let perfect = perfect_flags.iter().filter(|&&p| p).count();

    // Episodes: maximal runs of consecutive perfect super-cycles.
    let mut episodes = 0usize;
    let mut longest = 0usize;
    let mut run = 0usize;
    let mut total_in_episodes = 0usize;
    for &p in &perfect_flags {
        if p {
            run += 1;
            total_in_episodes += 1;
        } else {
            if run > 0 {
                episodes += 1;
                longest = longest.max(run);
            }
            run = 0;
        }
    }
    if run > 0 {
        episodes += 1;
        longest = longest.max(run);
    }
    let mean_episode = if episodes > 0 {
        total_in_episodes as f64 / episodes as f64
    } else {
        0.0
    };

    let mean_super_ticks = if super_lens.is_empty() {
        0.0
    } else {
        super_lens.iter().sum::<f64>() / super_lens.len() as f64
    };

    let verdict = if !perfect_flags.is_empty() && perfect == perfect_flags.len() {
        "TRUE LOCK 3:2"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else {
        "WALKING"
    };

    RegimeLedger {
        name,
        weight,
        delay,
        driver_spikes: driver_times.len(),
        follower_spikes: follower_times.len(),
        follower_mean_isi: mean_isi(&follower_times),
        super_cycles: windows.len(),
        perfect,
        count_slips,
        phase_slips,
        episodes,
        longest_episode: longest,
        mean_episode,
        median_phases: medians,
        dispersions,
        mean_super_ticks,
        verdict,
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
    println!("widow-episodes headless: the 3:2 that would not hold, priced by how long it almost holds");
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
            "  super-cycle: 2 driver cycles {:.2} ticks against 3 follower beats {:.2} — the follower runs early by {:.2} ticks per super-cycle; near 3:2, not at it",
            2.0 * driver.mean_isi,
            3.0 * follower.mean_isi,
            2.0 * driver.mean_isi - 3.0 * follower.mean_isi
        );
    }
    println!(
        "  a super-cycle is perfect with exactly 3 follower beats, each within ±{PHASE_TOL} of its median phase; an episode is a run of perfect super-cycles"
    );
    for r in regimes {
        println!(
            "  regime '{}' (synapse weight {:.1}, delay {} ticks): driver {} spikes, follower {} spikes (period {:.2})",
            r.name, r.weight, r.delay, r.driver_spikes, r.follower_spikes, r.follower_mean_isi
        );
        println!(
            "    super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles (~{:.0} ticks), mean episode {:.1}; beat phases {:.3}/{:.3}/{:.3} (dispersions {:.4}/{:.4}/{:.4}) — {}",
            r.super_cycles,
            r.perfect,
            r.count_slips,
            r.phase_slips,
            r.episodes,
            r.longest_episode,
            r.longest_episode as f64 * r.mean_super_ticks,
            r.mean_episode,
            r.median_phases[0],
            r.median_phases[1],
            r.median_phases[2],
            r.dispersions[0],
            r.dispersions[1],
            r.dispersions[2],
            r.verdict
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
    fn solo_clocks_are_near_three_to_two_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        // Three follower beats against two driver cycles: close,
        // measurably not equal, and the follower is the early one.
        let gap = 2.0 * driver.mean_isi - 3.0 * follower.mean_isi;
        assert!(gap > 0.05 && gap < 1.0, "super-cycle gap {gap}");
        let ratio = (2.0 * driver.mean_isi) / (3.0 * follower.mean_isi);
        assert!(ratio > 1.0 && ratio < 1.01, "ratio {ratio}");
    }

    #[test]
    fn uncoupled_pair_walks_in_episodes_not_a_lock() {
        let r = run_regime("uncoupled", 0.0, 0);
        assert!(r.super_cycles > 200, "super-cycles {}", r.super_cycles);
        // Not a lock: slips exist. Not chaos: episodes exist.
        assert!(r.perfect < r.super_cycles, "uncoupled claimed perfection");
        assert!(r.count_slips + r.phase_slips > 0);
        assert!(r.longest_episode >= 2, "longest {}", r.longest_episode);
        assert_ne!(r.verdict, "TRUE LOCK 3:2");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let r = run_regime("the surveyed hand (-1.5, delay 8)", -1.5, 8);
        assert_eq!(r.perfect + r.count_slips + r.phase_slips, r.super_cycles);
        assert!(r.episodes >= 1);
        assert!(r.longest_episode >= 1);
        assert!(r.mean_episode >= 1.0 && r.mean_episode <= r.longest_episode as f64);
    }

    #[test]
    fn surveyed_inhibition_outlasts_the_uncoupled_pair() {
        // The prior survey's claim, re-measured at this window:
        // gentle inhibition at delay 8 buys longer agreement than
        // no thread at all.
        let bare = run_regime("uncoupled", 0.0, 0);
        let held = run_regime("the surveyed hand (-1.5, delay 8)", -1.5, 8);
        assert!(
            held.longest_episode > bare.longest_episode,
            "surveyed longest {} vs uncoupled {}",
            held.longest_episode,
            bare.longest_episode
        );
    }

    #[test]
    fn wrong_medicine_does_not_lock() {
        // Excitatory weight hurries an already-early follower:
        // whatever it does, it must not produce a true lock.
        let r = run_regime("wrong medicine (+2.0, delay 0)", 2.0, 0);
        assert_ne!(r.verdict, "TRUE LOCK 3:2");
        assert!(r.perfect < r.super_cycles);
    }

    #[test]
    fn no_regime_claims_a_true_lock_it_did_not_earn() {
        // The instrument's honesty gate: across the whole sweep,
        // a TRUE LOCK verdict requires every super-cycle perfect.
        for &(name, w, d) in COUPLINGS.iter() {
            let r = run_regime(name, w, d);
            if r.verdict == "TRUE LOCK 3:2" {
                assert_eq!(r.perfect, r.super_cycles, "{name} claims a lock with slips");
            }
        }
    }
}
