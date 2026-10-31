//! # Widow Fringe 🕰️
//!
//! Brought by The Clockwork Widow 🕰️. Parent: `crates/neuro-sim`;
//! subject: the fringe of the 2:1 island surveyed by `widow-island`.
//!
//! `widow-island` judged every cell of the (weight, delay) plane
//! at 6,000 ticks and reported 34 locking cells: a 27-cell plateau,
//! a scattered fringe at weight 3.0 (delays 3, 5, 6, 8, 10 — with a
//! hole at delay 4), one landing at (10.0, 1), and a stray tooth at
//! (12.5, 10). A lock verdict at 112 cycles is a photograph. This
//! instrument is the longer exposure: the same pair (Regular
//! Spiking driver I=12, period 39.62; Fast Spiking follower I=6,
//! solo period 20.37, pair running long by 1.13 ticks per driver
//! cycle), the island's own cells, run for 24,000 ticks — and the
//! ledger borrowed from `widow-episodes`: not locked-or-not, but
//! *episodes*. A driver cycle is perfect when it holds exactly
//! two follower beats AND each beat's phase lies within ±0.06 of
//! that beat's median phase for the regime (the two-gate rule:
//! count alone lets a rotating pattern pass; phase alone lets a
//! dropped beat pass). An episode is a run of perfect cycles.
//!
//! The question is priced plainly: which of the island's fringe
//! landings were locks, and which were only long visits? A cell
//! that holds every one of ~566 cycles earns TRUE LOCK 2:1 under
//! the parent definition (all cycles perfect, both dispersions
//! under 0.05). Anything less is reported by how long it holds
//! at a time.
//!
//! Every period in this file is a named constant or a measured
//! quantity. A magic number is a clock with no face.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// The driver's constant drive current (Regular Spiking).
const DRIVER_CURRENT: f32 = 12.0;
/// The follower's constant drive current (Fast Spiking at I=6:
/// the near-2:1 clock from `widow-subharmonic`/`widow-island`).
const FOLLOWER_CURRENT: f32 = 6.0;
/// Total ticks per regime — four times the island's window.
const TICKS: usize = 24_000;
/// Ticks discarded at the start of each regime as settling.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept (the episodes
/// instrument's two-gate rule).
const PHASE_TOL: f64 = 0.06;
/// The parent lock definition's dispersion line, inherited
/// unchanged so the ledgers read side by side.
const LOCK_DISPERSION: f64 = 0.05;
/// An episode must reach this many consecutive perfect cycles
/// before the verdict calls the regime EPISODIC rather than
/// WALKING.
const EPISODIC_FLOOR: usize = 25;

/// Cells re-counted, in the order the Widow walked the shore:
/// (name, synapse weight, synaptic delay in ticks). Every cell
/// is one the island judged, named by its island verdict.
const CELLS: [(&str, f32, usize); 13] = [
    ("uncoupled (island: walking)", 0.0, 0),
    ("plateau heart (10.0, 5) (island: LOCKED)", 10.0, 5),
    ("plateau corner (5.0, 2) (island: LOCKED)", 5.0, 2),
    ("fringe landing (3.0, 3) (island: LOCKED)", 3.0, 3),
    ("the hole (3.0, 4) (island: walking)", 3.0, 4),
    ("fringe landing (3.0, 5) (island: LOCKED)", 3.0, 5),
    ("fringe landing (3.0, 6) (island: LOCKED)", 3.0, 6),
    ("fringe landing (3.0, 8) (island: LOCKED)", 3.0, 8),
    ("fringe landing (3.0, 10) (island: LOCKED)", 3.0, 10),
    ("fringe landing (10.0, 1) (island: LOCKED)", 10.0, 1),
    ("the stray tooth (12.5, 10) (island: LOCKED)", 12.5, 10),
    ("water beside the stray (12.5, 8) (island: walking)", 12.5, 8),
    ("iron thread, timed (35.0, 5) (island: walking)", 35.0, 5),
];

#[derive(Debug, Clone, Copy)]
struct SoloClock {
    spikes: usize,
    mean_isi: f64,
}

#[derive(Debug, Clone)]
struct CellLedger {
    name: &'static str,
    weight: f32,
    delay: usize,
    follower_mean_isi: f64,
    cycles: usize,
    perfect: usize,
    count_slips: usize,
    phase_slips: usize,
    episodes: usize,
    longest_episode: usize,
    mean_episode: f64,
    median_phases: [f64; 2],
    dispersions: [f64; 2],
    mean_cycle_ticks: f64,
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
    if times.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

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

fn run_cell(name: &'static str, weight: f32, delay: usize) -> CellLedger {
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

    // Driver cycles: windows between consecutive driver spikes.
    let mut windows: Vec<Vec<f64>> = Vec::new();
    let mut cycle_lens: Vec<f64> = Vec::new();
    for w in driver_times.windows(2) {
        let (start, end) = (w[0], w[1]);
        let beats: Vec<f64> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64)
            .collect();
        windows.push(beats);
        cycle_lens.push((end - start) as f64);
    }

    let mut medians = [0.0f64; 2];
    let mut dispersions = [1.0f64; 2];
    for j in 0..2 {
        let phases: Vec<f64> = windows.iter().filter(|w| w.len() == 2).map(|w| w[j]).collect();
        medians[j] = median(&phases);
        dispersions[j] = dispersion(&phases);
    }

    let mut perfect_flags: Vec<bool> = Vec::with_capacity(windows.len());
    let mut count_slips = 0usize;
    let mut phase_slips = 0usize;
    for w in &windows {
        if w.len() != 2 {
            count_slips += 1;
            perfect_flags.push(false);
        } else if (0..2).any(|j| circ_dist(w[j], medians[j]) > PHASE_TOL) {
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
    let mean_episode = if episodes > 0 { total_in_episodes as f64 / episodes as f64 } else { 0.0 };
    let mean_cycle_ticks = if cycle_lens.is_empty() {
        0.0
    } else {
        cycle_lens.iter().sum::<f64>() / cycle_lens.len() as f64
    };

    let verdict = if !perfect_flags.is_empty()
        && perfect == perfect_flags.len()
        && dispersions[0] < LOCK_DISPERSION
        && dispersions[1] < LOCK_DISPERSION
    {
        "TRUE LOCK 2:1"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else {
        "WALKING"
    };

    CellLedger {
        name,
        weight,
        delay,
        follower_mean_isi: mean_isi(&follower_times),
        cycles: windows.len(),
        perfect,
        count_slips,
        phase_slips,
        episodes,
        longest_episode: longest,
        mean_episode,
        median_phases: medians,
        dispersions,
        mean_cycle_ticks,
        verdict,
    }
}

fn run_all() -> (SoloClock, SoloClock, Vec<CellLedger>) {
    let driver = measure_solo(false, DRIVER_CURRENT);
    let follower = measure_solo(true, FOLLOWER_CURRENT);
    let cells = CELLS.iter().map(|&(n, w, d)| run_cell(n, w, d)).collect();
    (driver, follower, cells)
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, cells: &[CellLedger]) {
    println!("widow-fringe headless: the 2:1 island's fringe, re-counted at four times the island's window — locks, or long visits?");
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
            "  pair period (2 x follower): {:.2} ticks against driver {:.2} — ratio {:.3}; the follower's pair runs long by {:.2} ticks per driver cycle",
            2.0 * follower.mean_isi,
            driver.mean_isi,
            (2.0 * follower.mean_isi) / driver.mean_isi,
            2.0 * follower.mean_isi - driver.mean_isi
        );
    }
    println!(
        "  a cycle is perfect with exactly 2 follower beats, each within ±{PHASE_TOL} of its median phase; TRUE LOCK 2:1 additionally requires every cycle perfect and both dispersions < {LOCK_DISPERSION} (the island's own definition)"
    );
    for c in cells {
        println!(
            "  cell '{}' (synapse weight {:.1}, delay {} ticks): follower period {:.2}",
            c.name, c.weight, c.delay, c.follower_mean_isi
        );
        println!(
            "    cycles {}, perfect {}, count slips {}, phase slips {}; episodes {}, longest {} cycles (~{:.0} ticks), mean episode {:.1}; beat phases {:.3}/{:.3} (dispersions {:.4}/{:.4}) — {}",
            c.cycles,
            c.perfect,
            c.count_slips,
            c.phase_slips,
            c.episodes,
            c.longest_episode,
            c.longest_episode as f64 * c.mean_cycle_ticks,
            c.mean_episode,
            c.median_phases[0],
            c.median_phases[1],
            c.dispersions[0],
            c.dispersions[1],
            c.verdict
        );
    }
    let island_locked: Vec<&CellLedger> = cells.iter().filter(|c| c.name.contains("island: LOCKED")).collect();
    let still = island_locked.iter().filter(|c| c.verdict == "TRUE LOCK 2:1").count();
    println!(
        "  verdict: of {} cells the island called LOCKED, {} hold every cycle at this window; the rest are priced above in episodes",
        island_locked.len(),
        still
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let (driver, follower, cells) = run_all();
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
        let ratio = (2.0 * follower.mean_isi) / driver.mean_isi;
        assert!(ratio > 1.01 && ratio < 1.06, "ratio {ratio}");
    }

    #[test]
    fn episode_accounting_partitions_the_window() {
        let c = run_cell("fringe landing (3.0, 5) (island: LOCKED)", 3.0, 5);
        assert_eq!(c.perfect + c.count_slips + c.phase_slips, c.cycles);
        assert!(c.cycles > 400, "cycles {}", c.cycles);
        assert!(c.mean_episode >= 1.0 && c.mean_episode <= c.longest_episode as f64);
    }

    #[test]
    fn plateau_heart_holds_at_the_long_window() {
        // The island's heart, re-counted four times longer: it
        // must still be (nearly) perfect — the plateau is ground,
        // not weather.
        let c = run_cell("plateau heart (10.0, 5) (island: LOCKED)", 10.0, 5);
        // Measured: 536/567 perfect (94.5%), longest episode 72.
        // Ground, yes — but ground that breathes: 31 phase slips
        // live inside its tight aggregate dispersions.
        assert!(
            c.perfect as f64 >= 0.90 * c.cycles as f64,
            "plateau heart perfect {}/{}",
            c.perfect,
            c.cycles
        );
        assert!(c.longest_episode >= 50, "longest {}", c.longest_episode);
    }

    #[test]
    fn the_hole_does_not_claim_a_lock() {
        let c = run_cell("the hole (3.0, 4) (island: walking)", 3.0, 4);
        assert_ne!(c.verdict, "TRUE LOCK 2:1");
        assert!(c.perfect < c.cycles);
    }

    #[test]
    fn uncoupled_walks() {
        let c = run_cell("uncoupled (island: walking)", 0.0, 0);
        assert_ne!(c.verdict, "TRUE LOCK 2:1");
        assert!(c.perfect < c.cycles);
    }

    #[test]
    fn no_cell_claims_a_lock_it_did_not_earn() {
        for &(name, w, d) in CELLS.iter() {
            let c = run_cell(name, w, d);
            if c.verdict == "TRUE LOCK 2:1" {
                assert_eq!(c.perfect, c.cycles, "{name} claims a lock with slips");
                assert!(c.dispersions[0] < LOCK_DISPERSION && c.dispersions[1] < LOCK_DISPERSION);
            }
        }
    }
}
