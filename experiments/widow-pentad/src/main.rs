//! # Widow Pentad 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`
//! (the Izhikevich spiking-neuron model).
//!
//! The Widow has counted the even ratio (2:1, locked by a timed
//! thread) and the odd one that runs early (3:2, episodes only,
//! medicine inhibition). This instrument takes up the next named
//! ratio in her survey: **5:3**. A Fast Spiking follower at
//! I=5.3 keeps a solo period near 24.06 ticks against the Regular
//! Spiking driver's 39.60: five follower beats span ~120.31 ticks
//! against three driver cycles' ~118.86 — near 5:3, and the
//! follower runs *late* by ~1.45 ticks per super-cycle. Late is
//! the whole story, reversed from the 3:2 pair: the medicine here
//! is *excitation* (a synapse that hurries), and inhibition —
//! the only medicine the 3:2 pair ever needed — is kept as the
//! wrong-medicine control, where it should make things worse.
//!
//! A **super-cycle** is three consecutive driver cycles. It is
//! **perfect** when it holds exactly five follower beats AND each
//! beat's phase on the super-cycle's ring lies within ±0.06 of
//! that beat's median phase for the regime. An **episode** is a
//! maximal run of consecutive perfect super-cycles. The ledger is
//! counted over 24,000 ticks per regime (first 1,500 discarded;
//! ~189 super-cycles) — the same long window at which the 3:2
//! pair's apparent lock dissolved into episodes.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// Driver: Regular Spiking, I=12 (solo period measured, ~39.60).
const DRIVER_CURRENT: f32 = 12.0;
/// Follower: Fast Spiking, I=5.3 (solo period measured, ~24.06).
/// Five of its beats run late against three driver cycles.
const FOLLOWER_CURRENT: f32 = 5.3;
/// Total ticks per regime.
const TICKS: usize = 24_000;
/// Settling ticks discarded at the start of each regime.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;

/// Couplings swept: (name, weight, delay). Positive weights
/// excite — they hurry the late follower. One inhibitory regime
/// is kept as the wrong-medicine control.
const COUPLINGS: [(&str, f32, usize); 10] = [
    ("uncoupled", 0.0, 0),
    ("wrong medicine (-1.5, delay 8)", -1.5, 8),
    ("gentle hand (+0.5, delay 8)", 0.5, 8),
    ("surveyed hand (+1.0, delay 8)", 1.0, 8),
    ("firm hand (+1.5, delay 8)", 1.5, 8),
    ("heavy hand (+2.5, delay 8)", 2.5, 8),
    ("crushing hand (+4.0, delay 8)", 4.0, 8),
    ("surveyed hand, no delay (+1.0, delay 0)", 1.0, 0),
    ("surveyed hand, early (+1.0, delay 4)", 1.0, 4),
    ("surveyed hand, late (+1.0, delay 12)", 1.0, 12),
];

#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

#[derive(Debug, Clone)]
struct RegimeLedger {
    name: &'static str,
    weight: f32,
    delay: usize,
    driver_spikes: usize,
    follower_spikes: usize,
    follower_mean_isi: f64,
    super_cycles: usize,
    perfect: usize,
    count_slips: usize,
    phase_slips: usize,
    episodes: usize,
    longest_episode: usize,
    mean_episode: f64,
    median_phases: [f64; 5],
    dispersions: [f64; 5],
    mean_super_ticks: f64,
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

fn dispersion(phases: &[f64]) -> f64 {
    if phases.is_empty() { return 1.0; }
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

fn run_regime(name: &'static str, weight: f32, delay: usize) -> RegimeLedger {
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
    // Super-cycles: windows between every third driver spike.
    let n_super = driver_times.len().saturating_sub(3) / 3;
    let mut windows: Vec<Vec<f64>> = Vec::with_capacity(n_super);
    let mut super_lens: Vec<f64> = Vec::with_capacity(n_super);
    for k in 0..n_super {
        let (start, end) = (driver_times[3 * k], driver_times[3 * k + 3]);
        let beats: Vec<f64> = follower_times.iter().copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64).collect();
        windows.push(beats);
        super_lens.push((end - start) as f64);
    }
    let mut medians = [0.0f64; 5];
    let mut dispersions = [1.0f64; 5];
    for j in 0..5 {
        let phases: Vec<f64> = windows.iter().filter(|w| w.len() == 5).map(|w| w[j]).collect();
        medians[j] = median(&phases);
        dispersions[j] = dispersion(&phases);
    }
    let mut perfect_flags: Vec<bool> = Vec::with_capacity(windows.len());
    let mut count_slips = 0usize;
    let mut phase_slips = 0usize;
    for w in &windows {
        if w.len() != 5 {
            count_slips += 1;
            perfect_flags.push(false);
        } else if (0..5).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
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
    let mean_super_ticks = if super_lens.is_empty() { 0.0 } else { super_lens.iter().sum::<f64>() / super_lens.len() as f64 };
    let verdict = if !perfect_flags.is_empty() && perfect == perfect_flags.len() {
        "TRUE LOCK 5:3"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else { "WALKING" };
    RegimeLedger {
        name, weight, delay,
        driver_spikes: driver_times.len(),
        follower_spikes: follower_times.len(),
        follower_mean_isi: mean_isi(&follower_times),
        super_cycles: windows.len(), perfect, count_slips, phase_slips,
        episodes, longest_episode: longest, mean_episode,
        median_phases: medians, dispersions, mean_super_ticks, verdict,
    }
}

fn run_all() -> (SoloClock, SoloClock, Vec<RegimeLedger>) {
    let driver_solo = measure_solo(false, DRIVER_CURRENT);
    let follower_solo = measure_solo(true, FOLLOWER_CURRENT);
    let regimes = COUPLINGS.iter().map(|&(n, w, d)| run_regime(n, w, d)).collect();
    (driver_solo, follower_solo, regimes)
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, regimes: &[RegimeLedger]) {
    println!("widow-pentad headless: the 5:3 that runs late, priced by how long it almost holds");
    println!("  window: {TICKS} ticks per regime, first {SETTLE_TICKS} discarded as settling");
    println!("  driver clock (Regular Spiking, I={DRIVER_CURRENT}): solo {} spikes, mean period {:.2} ticks", driver.spikes, driver.mean_isi);
    println!("  follower clock (Fast Spiking, I={FOLLOWER_CURRENT}): solo {} spikes, mean period {:.2} ticks", follower.spikes, follower.mean_isi);
    if driver.mean_isi > 0.0 && follower.mean_isi > 0.0 {
        println!("  super-cycle: 3 driver cycles {:.2} ticks against 5 follower beats {:.2} — the follower runs late by {:.2} ticks per super-cycle; near 5:3, not at it",
            3.0 * driver.mean_isi, 5.0 * follower.mean_isi, 5.0 * follower.mean_isi - 3.0 * driver.mean_isi);
    }
    println!("  a super-cycle is perfect with exactly 5 follower beats, each within ±{PHASE_TOL} of its median phase; an episode is a run of perfect super-cycles");
    for r in regimes {
        println!("  regime '{}' (synapse weight {:+.1}, delay {} ticks): driver {} spikes, follower {} spikes (period {:.2})",
            r.name, r.weight, r.delay, r.driver_spikes, r.follower_spikes, r.follower_mean_isi);
        println!("    super-cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} super-cycles (~{:.0} ticks), mean episode {:.1}; beat phases {:.3}/{:.3}/{:.3}/{:.3}/{:.3} (dispersions {:.4}/{:.4}/{:.4}/{:.4}/{:.4}) — {}",
            r.super_cycles, r.perfect, r.count_slips, r.phase_slips, r.episodes, r.longest_episode,
            r.longest_episode as f64 * r.mean_super_ticks, r.mean_episode,
            r.median_phases[0], r.median_phases[1], r.median_phases[2], r.median_phases[3], r.median_phases[4],
            r.dispersions[0], r.dispersions[1], r.dispersions[2], r.dispersions[3], r.dispersions[4], r.verdict);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|a| a == "--headless");
    let (driver, follower, regimes) = run_all();
    print_ledger(&driver, &follower, &regimes);
    if !headless { println!("  (count it headless any time: --headless)"); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solo_clocks_are_near_five_to_three_and_not_at_it() {
        let driver = measure_solo(false, DRIVER_CURRENT);
        let follower = measure_solo(true, FOLLOWER_CURRENT);
        let gap = 5.0 * follower.mean_isi - 3.0 * driver.mean_isi;
        assert!(gap > 0.5 && gap < 3.0, "super-cycle lateness {gap}");
    }

    #[test]
    fn uncoupled_pair_is_not_a_lock() {
        let r = run_regime("uncoupled", 0.0, 0);
        assert!(r.super_cycles > 150, "super-cycles {}", r.super_cycles);
        assert!(r.perfect < r.super_cycles);
        assert_ne!(r.verdict, "TRUE LOCK 5:3");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let r = run_regime("surveyed hand (+1.0, delay 8)", 1.0, 8);
        assert_eq!(r.perfect + r.count_slips + r.phase_slips, r.super_cycles);
        assert!(r.longest_episode >= 1);
        assert!(r.mean_episode >= 1.0 && r.mean_episode <= r.longest_episode as f64);
    }

    #[test]
    fn wrong_medicine_does_not_lock() {
        let r = run_regime("wrong medicine (-1.5, delay 8)", -1.5, 8);
        assert_ne!(r.verdict, "TRUE LOCK 5:3");
        assert!(r.perfect < r.super_cycles);
    }

    #[test]
    fn best_excitation_outlasts_uncoupled() {
        let bare = run_regime("uncoupled", 0.0, 0);
        // At least one excitatory regime in the sweep must buy
        // longer agreement than no thread at all, or the whole
        // excitation reading is wrong (checked in test, not assumed).
        let best = COUPLINGS.iter().filter(|(_, w, _)| *w > 0.0)
            .map(|&(n, w, d)| run_regime(n, w, d).longest_episode).max().unwrap_or(0);
        assert!(best > bare.longest_episode, "best excitatory longest {best} vs uncoupled {}", bare.longest_episode);
    }

    #[test]
    fn no_regime_claims_a_true_lock_it_did_not_earn() {
        for &(name, w, d) in COUPLINGS.iter() {
            let r = run_regime(name, w, d);
            if r.verdict == "TRUE LOCK 5:3" {
                assert_eq!(r.perfect, r.super_cycles, "{name} claims a lock with slips");
            }
        }
    }
}
