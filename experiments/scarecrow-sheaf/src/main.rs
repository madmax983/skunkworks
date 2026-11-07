//! scarecrow-sheaf — The Scarecrow 🌾 plants Harvest's journal
//! and replay in the Clockwork Widow's 🕰️ champion pair
//! (`widow-heptad-peak`, weight -1.0, delay 10, near-7:5).
//!
//! The pair is two neurons and one synapse. The driver (Regular
//! Spiking, I=12) receives nothing; it is a clock. The follower
//! (Fast Spiking, I=4.8) receives one thing: the synapse's kicks,
//! each -1.0, each arriving 11 ticks after the driver spike that
//! sent it (the parent crate pushes the spike into transit on
//! the step after the spike and counts the delay down there —
//! delay 10 lands on step spike+11; this planting measures that
//! arithmetic instead of assuming it, see P2).
//!
//! So the whole coupling between the two clocks fits in a
//! journal: the ticks the kicks landed. Nothing else crosses.
//! This planting writes that journal down three ways and replays
//! the follower — alone, one neuron, no synapse — from each:
//!
//! - Sheaf (deliveries): the journal of landing ticks, read off
//!   the synapse's own `active` flag as the live run steps.
//! - Sheaf (spikes): the journal of driver spike ticks; the
//!   replay re-derives the landings as spike + delay + 1.
//! - Immediate: the spike journal replayed with the delay
//!   forgotten — kicks land on the spike tick itself.
//! - Coarse: the delivery journal with every landing rounded
//!   to the nearest 5 ticks — a journal written from memory.
//!
//! The counting instrument is the Widow's own, copied faithfully
//! (super-cycles of five driver cycles, seven follower beats,
//! phase tolerance ±0.06, medians over the run's own windows),
//! and a test pins the live cell to the host's published centre:
//! perfect 92/112, count slips 3, phase slips 17, longest
//! episode 58, mean episode 18.4.
//!
//! The comparison the ledger must also print, plainly: one whole
//! checkpoint of this pair is tiny (two neurons' v/u/params and
//! a near-empty transit queue), and the journal is not. The
//! sheaf's virtue is not size. It is that the replay never opens
//! the follower: it needs the kicks, and only the kicks.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// Driver: Regular Spiking, I=12 (host's constants, verbatim).
const DRIVER_CURRENT: f32 = 12.0;
/// Follower: Fast Spiking, I=4.8.
const FOLLOWER_CURRENT: f32 = 4.8;
/// The champion cell: weight -1.0, delay 10.
const WEIGHT: f32 = -1.0;
const DELAY: usize = 10;
/// Total ticks, as the host counts.
const TICKS: usize = 24_000;
/// Settling ticks discarded by the instrument.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;
/// Journal record: one u16 tick per event.
const JOURNAL_BYTES_PER_EVENT: usize = 2;

#[derive(Debug, Clone)]
struct CellLedger {
    super_cycles: usize,
    perfect: usize,
    count_slips: usize,
    phase_slips: usize,
    episodes: usize,
    longest_episode: usize,
    mean_episode: f64,
    verdict: &'static str,
}

fn build_pair() -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    net.add_synapse_with_delay(driver, follower, WEIGHT, DELAY);
    net
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

/// The Widow's counting instrument, unchanged: driver and
/// follower spike times are the post-settle trains.
fn count_cell(driver_times: &[usize], follower_times: &[usize]) -> CellLedger {
    let n_super = driver_times.len().saturating_sub(5) / 5;
    let mut windows: Vec<Vec<f64>> = Vec::with_capacity(n_super);
    for k in 0..n_super {
        let (start, end) = (driver_times[5 * k], driver_times[5 * k + 5]);
        let beats: Vec<f64> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .map(|f| (f - start) as f64 / (end - start) as f64)
            .collect();
        windows.push(beats);
    }
    let mut medians = [0.0f64; 7];
    for j in 0..7 {
        let phases: Vec<f64> = windows
            .iter()
            .filter(|w| w.len() == 7)
            .map(|w| w[j])
            .collect();
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
    let verdict = if !perfect_flags.is_empty() && perfect == perfect_flags.len() {
        "TRUE LOCK 7:5"
    } else if longest >= EPISODIC_FLOOR {
        "EPISODIC"
    } else {
        "WALKING"
    };
    CellLedger {
        super_cycles: windows.len(),
        perfect,
        count_slips,
        phase_slips,
        episodes,
        longest_episode: longest,
        mean_episode,
        verdict,
    }
}

struct LiveRun {
    driver_spikes: Vec<usize>,
    deliveries: Vec<usize>,
    follower_spikes: Vec<usize>,
    ledger: CellLedger,
}

/// The live pair, stepped once, writing both journals as it goes:
/// driver spike ticks, and delivery ticks read off the synapse's
/// `active` flag after each step.
fn run_live() -> LiveRun {
    let mut net = build_pair();
    let mut driver_spikes: Vec<usize> = Vec::new();
    let mut deliveries: Vec<usize> = Vec::new();
    let mut follower_spikes: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[DRIVER_CURRENT, FOLLOWER_CURRENT]);
        if net.is_spiking(0) {
            driver_spikes.push(t);
        }
        if net.synapses[0].active {
            deliveries.push(t);
        }
        if net.is_spiking(1) {
            follower_spikes.push(t);
        }
    }
    let post = |v: &[usize]| -> Vec<usize> {
        v.iter().copied().filter(|&t| t >= SETTLE_TICKS).collect()
    };
    let ledger = count_cell(&post(&driver_spikes), &post(&follower_spikes));
    LiveRun {
        driver_spikes,
        deliveries,
        follower_spikes,
        ledger,
    }
}

/// The follower alone: one Fast Spiking neuron, its constant
/// current, and a kick of WEIGHT added on each journaled tick.
/// No synapse, no driver — the journal is the whole coupling.
fn run_replay(kick_ticks: &[usize]) -> Vec<usize> {
    let mut net = Network::new();
    let n = net.add_neuron();
    net.neurons[n] = Izhikevich::new_fast_spiking();
    let mut is_kick = vec![false; TICKS];
    for &t in kick_ticks {
        if t < TICKS {
            is_kick[t] = true;
        }
    }
    let mut follower_spikes: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        let input = if is_kick[t] {
            FOLLOWER_CURRENT + WEIGHT
        } else {
            FOLLOWER_CURRENT
        };
        net.step(&[input]);
        if net.is_spiking(0) {
            follower_spikes.push(t);
        }
    }
    follower_spikes
}

/// First tick at which two spike trains disagree (a spike in one
/// and not the other), scanning the union in order.
fn first_divergence(a: &[usize], b: &[usize]) -> Option<usize> {
    let mut i = 0usize;
    let mut j = 0usize;
    loop {
        match (a.get(i), b.get(j)) {
            (Some(&x), Some(&y)) => {
                if x == y {
                    i += 1;
                    j += 1;
                } else {
                    return Some(x.min(y));
                }
            }
            (Some(&x), None) => return Some(x),
            (None, Some(&y)) => return Some(y),
            (None, None) => return None,
        }
    }
}

fn post_settle(v: &[usize]) -> Vec<usize> {
    v.iter().copied().filter(|&t| t >= SETTLE_TICKS).collect()
}

fn fmt_cell(name: &str, c: &CellLedger) {
    println!(
        "{}: perfect {}/{}, count slips {}, phase slips {}, episodes {}, longest {}, mean episode {:.1}, verdict {}",
        name, c.perfect, c.super_cycles, c.count_slips, c.phase_slips,
        c.episodes, c.longest_episode, c.mean_episode, c.verdict
    );
}

fn headless() {
    println!("scarecrow-sheaf headless ledger");
    println!(
        "host: widow-heptad-peak champion cell (neuro-sim pair, driver RS I={}, follower FS I={}, weight {}, delay {}), {} ticks, settle {}",
        DRIVER_CURRENT, FOLLOWER_CURRENT, WEIGHT, DELAY, TICKS, SETTLE_TICKS
    );
    println!("principle: journal and replay (Harvest) — the journal holds only the kicks that cross the synapse; the follower is replayed alone from the journal, never opened");

    let live = run_live();
    fmt_cell("live", &live.ledger);
    println!(
        "journals: driver spikes {}, deliveries {}, follower spikes {} (all ticks, settle included)",
        live.driver_spikes.len(),
        live.deliveries.len(),
        live.follower_spikes.len()
    );
    // The measured delay arithmetic: first delivery minus the
    // first spike that could have sent it.
    let first_lag = live.deliveries.first().zip(live.driver_spikes.first()).map(|(d, s)| d - s);
    println!(
        "delay arithmetic: first delivery tick {:?}, first driver spike {:?}, measured lag {:?} (declared delay {} + 1)",
        live.deliveries.first(), live.driver_spikes.first(), first_lag, DELAY
    );

    // Whole-checkpoint size, counted honestly: two neurons at
    // six f32 (v, u, a, b, c, d), the transit queue's entries,
    // and the spikes/inputs flags.
    let checkpoint_bytes = 2 * 6 * 4 + 8;
    let spike_journal_bytes = live.driver_spikes.len() * JOURNAL_BYTES_PER_EVENT;
    let delivery_journal_bytes = live.deliveries.len() * JOURNAL_BYTES_PER_EVENT;
    println!(
        "paper sizes: one whole checkpoint of the pair {} bytes (2 neurons x 6 f32 + flags; transit queue holds at most 1 kick of {} bytes); spike journal {} events = {} bytes; delivery journal {} events = {} bytes",
        checkpoint_bytes, JOURNAL_BYTES_PER_EVENT,
        live.driver_spikes.len(), spike_journal_bytes,
        live.deliveries.len(), delivery_journal_bytes
    );

    let driver_post = post_settle(&live.driver_spikes);

    // Sheaf (deliveries).
    let replay_d = run_replay(&live.deliveries);
    let div_d = first_divergence(&live.follower_spikes, &replay_d);
    let cell_d = count_cell(&driver_post, &post_settle(&replay_d));
    fmt_cell("replay-deliveries", &cell_d);
    println!(
        "replay-deliveries: follower train identical {}, first divergence {:?}",
        div_d.is_none(), div_d
    );

    // Sheaf (spikes): landings re-derived as spike + delay + 1.
    let derived: Vec<usize> = live
        .driver_spikes
        .iter()
        .map(|s| s + DELAY + 1)
        .filter(|&t| t < TICKS)
        .collect();
    let derived_matches = derived == live.deliveries;
    let replay_s = run_replay(&derived);
    let div_s = first_divergence(&live.follower_spikes, &replay_s);
    let cell_s = count_cell(&driver_post, &post_settle(&replay_s));
    fmt_cell("replay-spikes", &cell_s);
    println!(
        "replay-spikes: derived landings == journaled landings {}, follower train identical {}, first divergence {:?}",
        derived_matches, div_s.is_none(), div_s
    );

    // Immediate: the delay forgotten.
    let replay_i = run_replay(&live.driver_spikes);
    let div_i = first_divergence(&live.follower_spikes, &replay_i);
    let cell_i = count_cell(&driver_post, &post_settle(&replay_i));
    fmt_cell("replay-immediate", &cell_i);
    println!(
        "replay-immediate: follower spikes {}, first divergence {:?}",
        replay_i.len(), div_i
    );

    // Coarse: landings rounded to the nearest 5 ticks.
    let coarse: Vec<usize> = live
        .deliveries
        .iter()
        .map(|&d| ((d + 2) / 5) * 5)
        .collect();
    let replay_c = run_replay(&coarse);
    let div_c = first_divergence(&live.follower_spikes, &replay_c);
    let cell_c = count_cell(&driver_post, &post_settle(&replay_c));
    fmt_cell("replay-coarse5", &cell_c);
    println!(
        "replay-coarse5: follower spikes {}, first divergence {:?}",
        replay_c.len(), div_c
    );

    let l = &live.ledger;
    let b1 = l.perfect == 92
        && l.super_cycles == 112
        && l.count_slips == 3
        && l.phase_slips == 17
        && l.longest_episode == 58
        && (l.mean_episode - 18.4).abs() < 0.05;
    let p1 = div_d.is_none()
        && cell_d.perfect == l.perfect
        && cell_d.longest_episode == l.longest_episode
        && cell_d.count_slips == l.count_slips
        && cell_d.phase_slips == l.phase_slips;
    let p2 = derived_matches && div_s.is_none();
    let p3 = div_i.is_some() && (cell_i.perfect != l.perfect || cell_i.longest_episode != l.longest_episode);
    let p4 = div_c.is_some() && cell_c.verdict == "EPISODIC";
    let p5 = spike_journal_bytes >= 10 * checkpoint_bytes;
    println!(
        "verdicts: B1 host centre exact: {}; P1 delivery journal replays the follower exact: {}; P2 spike journal + delay arithmetic (spike + {} + 1) replays exact: {}; P3 immediate journal (delay forgotten) diverges and its cell differs: {}; P4 coarse journal (landings rounded to 5) diverges yet keeps the EPISODIC verdict: {}; P5 the journal is not the small paper (spike journal {} B >= 10x checkpoint {} B): {}",
        b1, p1, DELAY, p2, p3, p4, spike_journal_bytes, checkpoint_bytes, p5
    );
}

fn main() -> Result<()> {
    // --headless is handled BEFORE anything else; this specimen
    // has no TUI at all.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-sheaf: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_centre_exact() {
        // The host's published centre cell (widow-heptad-peak):
        // perfect 92/112, 3 count slips, 17 phase slips,
        // longest episode 58, mean episode 18.4.
        let live = run_live();
        assert_eq!(live.ledger.perfect, 92);
        assert_eq!(live.ledger.super_cycles, 112);
        assert_eq!(live.ledger.count_slips, 3);
        assert_eq!(live.ledger.phase_slips, 17);
        assert_eq!(live.ledger.longest_episode, 58);
        assert!((live.ledger.mean_episode - 18.4).abs() < 0.05);
    }

    #[test]
    fn delivery_journal_replays_exact() {
        // The principle, pinned: the follower, replayed alone
        // from the landing ticks alone, spikes on exactly the
        // live run's ticks, all 24,000 of them.
        let live = run_live();
        let replay = run_replay(&live.deliveries);
        assert_eq!(replay, live.follower_spikes);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), None);
    }

    #[test]
    fn spike_journal_rebuilds_the_landings() {
        // The delay is arithmetic, not state: landings are the
        // spike ticks shifted by delay + 1, exactly.
        let live = run_live();
        let derived: Vec<usize> = live
            .driver_spikes
            .iter()
            .map(|s| s + DELAY + 1)
            .filter(|&t| t < TICKS)
            .collect();
        assert_eq!(derived, live.deliveries);
    }

    #[test]
    fn forgotten_delay_diverges() {
        // A journal that drops the delay is a different coupling:
        // the replayed follower must part from the live one.
        let live = run_live();
        let replay = run_replay(&live.driver_spikes);
        assert!(first_divergence(&live.follower_spikes, &replay).is_some());
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }
}
