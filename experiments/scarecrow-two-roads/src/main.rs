//! scarecrow-two-roads — The Scarecrow 🌾 plants Harvest's journal
//! and replay in a pair whose coupling crosses on TWO roads:
//! `scarecrow-sheaf`'s champion follower now fed by two drivers
//! through two synapses of opposite sign.
//!
//! Road A is the sheaf's own road: driver A (Regular Spiking,
//! I=12) to the follower (Fast Spiking, I=4.8), weight -1.0,
//! delay 10 — landings at spike + 11. Road B is new: driver B
//! (Regular Spiking, I=14, a faster clock) to the same follower,
//! weight +0.75, delay 6 — landings at spike + 7.
//!
//! The two clocks are incommensurate (periods ~39.6 and ~34.2),
//! so the roads' landings drift through each other and sometimes
//! arrive on the SAME tick. The sheaf's journal was a list of
//! ticks because every entry carried the same kick. This journal
//! cannot be: each entry must carry its sign, or the replay
//! cannot tell a brake from a spur.
//!
//! Journals written and replayed (the follower alone each time,
//! one neuron, no synapses — the journals are the whole coupling):
//!
//! - Signed: (tick, weight) entries as they landed.
//! - Derived: landings rebuilt per road from the spike journals
//!   as spike + delay + 1, signs attached by road.
//! - Unsigned: the sheaf's spelling — ticks only, every entry
//!   replayed as -1.0. What the old journal would say here.
//! - Flipped: signs swapped, +0.75 <-> -1.0 by road.
//! - Netted: entries landing on one tick merged to their sum.
//! - Deduped: entries landing on one tick collapsed to the
//!   first entry only, the rest dropped.
//!
//! The counting instrument is the Widow's own, copied faithfully
//! from `scarecrow-sheaf` (super-cycles of five driver-A cycles,
//! seven follower beats, phase tolerance ±0.06), and a baseline
//! run of the single-road pair pins this crate to the sheaf's
//! published centre before the second road is allowed to speak.

use anyhow::Result;
use neuro_sim::{Izhikevich, Network};

/// Driver A: Regular Spiking, I=12 (the sheaf's clock).
const DRIVER_A_CURRENT: f32 = 12.0;
/// Driver B: Regular Spiking, I=14 (a faster second clock).
const DRIVER_B_CURRENT: f32 = 14.0;
/// Follower: Fast Spiking, I=4.8 (the sheaf's follower).
const FOLLOWER_CURRENT: f32 = 4.8;
/// Road A: inhibitory, the sheaf's champion road.
const WEIGHT_A: f32 = -1.0;
const DELAY_A: usize = 10;
/// Road B: excitatory, shorter.
const WEIGHT_B: f32 = 0.75;
const DELAY_B: usize = 6;
/// Total ticks, as the host counts.
const TICKS: usize = 24_000;
/// Settling ticks discarded by the instrument.
const SETTLE_TICKS: usize = 1_500;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;
/// Signed journal record: u16 tick + i8 weight in quarter units
/// (-1.0 -> -4, +0.75 -> +3).
const SIGNED_ENTRY_BYTES: usize = 3;
/// Unsigned journal record: the tick alone.
const UNSIGNED_ENTRY_BYTES: usize = 2;

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

/// The single-road pair, exactly as `scarecrow-sheaf` builds it.
fn build_single() -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    net.add_synapse_with_delay(driver, follower, WEIGHT_A, DELAY_A);
    net
}

/// The two-road field: drivers A and B, one follower, two
/// synapses of opposite sign onto the follower.
fn build_two() -> Network {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    net.add_synapse_with_delay(a, follower, WEIGHT_A, DELAY_A);
    net.add_synapse_with_delay(b, follower, WEIGHT_B, DELAY_B);
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

/// The Widow's counting instrument, unchanged from the sheaf:
/// driver and follower spike times are the post-settle trains.
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

fn post_settle(v: &[usize]) -> Vec<usize> {
    v.iter().copied().filter(|&t| t >= SETTLE_TICKS).collect()
}

/// The single-road baseline: the sheaf's pair, run whole.
fn run_single() -> CellLedger {
    let mut net = build_single();
    let mut driver_spikes: Vec<usize> = Vec::new();
    let mut follower_spikes: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[DRIVER_A_CURRENT, FOLLOWER_CURRENT]);
        if net.is_spiking(0) {
            driver_spikes.push(t);
        }
        if net.is_spiking(1) {
            follower_spikes.push(t);
        }
    }
    count_cell(&post_settle(&driver_spikes), &post_settle(&follower_spikes))
}

struct LiveRun {
    a_spikes: Vec<usize>,
    b_spikes: Vec<usize>,
    /// Per-road landing ticks, read off each synapse's `active`.
    a_landings: Vec<usize>,
    b_landings: Vec<usize>,
    /// The signed journal: (tick, weight) in landing order —
    /// road A's entry before road B's on a shared tick, the
    /// order the live step applies them.
    journal: Vec<(usize, f32)>,
    follower_spikes: Vec<usize>,
    ledger: CellLedger,
}

fn run_live() -> LiveRun {
    let mut net = build_two();
    let mut a_spikes: Vec<usize> = Vec::new();
    let mut b_spikes: Vec<usize> = Vec::new();
    let mut a_landings: Vec<usize> = Vec::new();
    let mut b_landings: Vec<usize> = Vec::new();
    let mut journal: Vec<(usize, f32)> = Vec::new();
    let mut follower_spikes: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[DRIVER_A_CURRENT, DRIVER_B_CURRENT, FOLLOWER_CURRENT]);
        if net.is_spiking(0) {
            a_spikes.push(t);
        }
        if net.is_spiking(1) {
            b_spikes.push(t);
        }
        if net.synapses[0].active {
            a_landings.push(t);
            journal.push((t, WEIGHT_A));
        }
        if net.synapses[1].active {
            b_landings.push(t);
            journal.push((t, WEIGHT_B));
        }
        if net.is_spiking(2) {
            follower_spikes.push(t);
        }
    }
    let ledger = count_cell(&post_settle(&a_spikes), &post_settle(&follower_spikes));
    LiveRun {
        a_spikes,
        b_spikes,
        a_landings,
        b_landings,
        journal,
        follower_spikes,
        ledger,
    }
}

/// The follower alone: one Fast Spiking neuron, its constant
/// current, and on each tick the SUM of the journal's weights
/// for that tick. No synapses, no drivers — the journal is the
/// whole coupling, and a tick's entries are one sum to the ear.
fn run_replay(entries: &[(usize, f32)]) -> Vec<usize> {
    let mut net = Network::new();
    let n = net.add_neuron();
    net.neurons[n] = Izhikevich::new_fast_spiking();
    let mut kick = vec![0.0f32; TICKS];
    for &(t, w) in entries {
        if t < TICKS {
            kick[t] += w;
        }
    }
    let mut follower_spikes: Vec<usize> = Vec::new();
    for t in 0..TICKS {
        net.step(&[FOLLOWER_CURRENT + kick[t]]);
        if net.is_spiking(0) {
            follower_spikes.push(t);
        }
    }
    follower_spikes
}

/// Ticks carrying more than one journal entry (both roads
/// landing together), in order.
fn coincident_ticks(journal: &[(usize, f32)]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for w in journal.windows(2) {
        if w[0].0 == w[1].0 && out.last() != Some(&w[0].0) {
            out.push(w[0].0);
        }
    }
    out
}

/// First tick at which two spike trains disagree.
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

fn fmt_cell(name: &str, c: &CellLedger) {
    println!(
        "{}: perfect {}/{}, count slips {}, phase slips {}, episodes {}, longest {}, mean episode {:.1}, verdict {}",
        name, c.perfect, c.super_cycles, c.count_slips, c.phase_slips,
        c.episodes, c.longest_episode, c.mean_episode, c.verdict
    );
}

fn headless() {
    println!("scarecrow-two-roads headless ledger");
    println!(
        "host: scarecrow-sheaf's follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS
    );
    println!("principle: journal and replay (Harvest) — the journal's entries now carry sign; the follower is replayed alone from each spelling of the journal, never opened");

    // Baseline: the single road, the sheaf's centre.
    let base = run_single();
    fmt_cell("baseline-single-road", &base);

    let live = run_live();
    fmt_cell("live-two-roads", &live.ledger);
    println!(
        "journals: driver A spikes {}, driver B spikes {}, road A landings {}, road B landings {}, signed entries {}, follower spikes {} (all ticks, settle included)",
        live.a_spikes.len(), live.b_spikes.len(),
        live.a_landings.len(), live.b_landings.len(),
        live.journal.len(), live.follower_spikes.len()
    );
    let lag_a = live.a_landings.first().zip(live.a_spikes.first()).map(|(d, s)| d - s);
    let lag_b = live.b_landings.first().zip(live.b_spikes.first()).map(|(d, s)| d - s);
    println!(
        "delay arithmetic: road A first landing {:?} from first spike {:?} (lag {:?}, declared {} + 1); road B first landing {:?} from first spike {:?} (lag {:?}, declared {} + 1)",
        live.a_landings.first(), live.a_spikes.first(), lag_a, DELAY_A,
        live.b_landings.first(), live.b_spikes.first(), lag_b, DELAY_B
    );
    let coinc = coincident_ticks(&live.journal);
    println!(
        "coincidences: {} ticks carry both roads' kicks; first at {:?}",
        coinc.len(), coinc.first()
    );

    // Paper sizes, counted honestly as in the sheaf: a whole
    // checkpoint of the three-neuron field against the journals.
    let checkpoint_bytes = 3 * 6 * 4 + 8;
    let signed_bytes = live.journal.len() * SIGNED_ENTRY_BYTES;
    let unsigned_bytes = live.journal.len() * UNSIGNED_ENTRY_BYTES;
    println!(
        "paper sizes: one whole checkpoint of the field {} bytes (3 neurons x 6 f32 + flags); signed journal {} entries x {} B = {} bytes; unsigned journal {} entries x {} B = {} bytes; the sign costs {} bytes a season",
        checkpoint_bytes, live.journal.len(), SIGNED_ENTRY_BYTES, signed_bytes,
        live.journal.len(), UNSIGNED_ENTRY_BYTES, unsigned_bytes,
        signed_bytes - unsigned_bytes
    );

    let a_post = post_settle(&live.a_spikes);
    let cell_of = |train: &[usize]| count_cell(&a_post, &post_settle(train));

    // Signed deliveries.
    let replay_signed = run_replay(&live.journal);
    let div_signed = first_divergence(&live.follower_spikes, &replay_signed);
    fmt_cell("replay-signed", &cell_of(&replay_signed));
    println!(
        "replay-signed: follower train identical {}, first divergence {:?}",
        div_signed.is_none(), div_signed
    );

    // Derived from the spike journals, per road.
    let derived_a: Vec<usize> = live.a_spikes.iter().map(|s| s + DELAY_A + 1).filter(|&t| t < TICKS).collect();
    let derived_b: Vec<usize> = live.b_spikes.iter().map(|s| s + DELAY_B + 1).filter(|&t| t < TICKS).collect();
    let derived_ok = derived_a == live.a_landings && derived_b == live.b_landings;
    let mut derived_journal: Vec<(usize, f32)> = derived_a.iter().map(|&t| (t, WEIGHT_A)).collect();
    derived_journal.extend(derived_b.iter().map(|&t| (t, WEIGHT_B)));
    derived_journal.sort_by_key(|&(t, w)| (t, (w * 4.0) as i32));
    let replay_derived = run_replay(&derived_journal);
    let div_derived = first_divergence(&live.follower_spikes, &replay_derived);
    fmt_cell("replay-derived", &cell_of(&replay_derived));
    println!(
        "replay-derived: per-road derived landings == journaled landings {}, follower train identical {}, first divergence {:?}",
        derived_ok, div_derived.is_none(), div_derived
    );

    // Unsigned: the sheaf's spelling, every kick read as road A's.
    let unsigned: Vec<(usize, f32)> = live.journal.iter().map(|&(t, _)| (t, WEIGHT_A)).collect();
    let replay_unsigned = run_replay(&unsigned);
    let div_unsigned = first_divergence(&live.follower_spikes, &replay_unsigned);
    fmt_cell("replay-unsigned", &cell_of(&replay_unsigned));
    println!(
        "replay-unsigned: follower spikes {}, first divergence {:?} (first road-B landing at {:?})",
        replay_unsigned.len(), div_unsigned, live.b_landings.first()
    );

    // Flipped: each road read with the other's sign, magnitudes
    // kept by road — A lands as +1.0, B as -0.75.
    let flipped: Vec<(usize, f32)> = live.journal.iter().map(|&(t, w)| (t, -w)).collect();
    let replay_flipped = run_replay(&flipped);
    let div_flipped = first_divergence(&live.follower_spikes, &replay_flipped);
    fmt_cell("replay-flipped", &cell_of(&replay_flipped));
    println!(
        "replay-flipped: follower spikes {}, first divergence {:?}",
        replay_flipped.len(), div_flipped
    );

    // Netted: one entry per tick, carrying the tick's sum.
    let mut netted: Vec<(usize, f32)> = Vec::new();
    for &(t, w) in &live.journal {
        if let Some(last) = netted.last_mut().filter(|(lt, _)| *lt == t) {
            last.1 += w;
        } else {
            netted.push((t, w));
        }
    }
    let replay_netted = run_replay(&netted);
    let div_netted = first_divergence(&live.follower_spikes, &replay_netted);
    fmt_cell("replay-netted", &cell_of(&replay_netted));
    println!(
        "replay-netted: {} entries after merging, follower train identical {}, first divergence {:?}",
        netted.len(), div_netted.is_none(), div_netted
    );

    // Deduped: one entry per tick, the FIRST only — the second
    // road's coincident kicks dropped on the floor.
    let mut deduped: Vec<(usize, f32)> = Vec::new();
    for &(t, w) in &live.journal {
        if deduped.last().map(|&(lt, _)| lt) != Some(t) {
            deduped.push((t, w));
        }
    }
    let replay_deduped = run_replay(&deduped);
    let div_deduped = first_divergence(&live.follower_spikes, &replay_deduped);
    fmt_cell("replay-deduped", &cell_of(&replay_deduped));
    println!(
        "replay-deduped: {} entries after collapsing, follower spikes {}, first divergence {:?} (first coincidence at {:?})",
        deduped.len(), replay_deduped.len(), div_deduped, coinc.first()
    );

    let b1 = base.perfect == 92
        && base.super_cycles == 112
        && base.count_slips == 3
        && base.phase_slips == 17
        && base.longest_episode == 58
        && (base.mean_episode - 18.4).abs() < 0.05;
    let p1 = div_signed.is_none();
    let p2 = derived_ok && div_derived.is_none();
    let p3 = coinc.len() >= 5;
    let p4 = match (div_unsigned, live.b_landings.first()) {
        (Some(d), Some(&first_b)) => d >= first_b,
        _ => false,
    };
    let p5 = div_flipped.is_some();
    let p6 = div_netted.is_none();
    let p7 = div_deduped.is_some();
    let p8 = signed_bytes >= 10 * checkpoint_bytes;
    println!(
        "verdicts: B1 single-road baseline reproduces the sheaf centre exact: {}; P1 signed journal replays the follower exact: {}; P2 per-road derived landings match and replay exact: {}; P3 the roads coincide (>= 5 shared ticks; found {}): {}; P4 unsigned journal (sign forgotten) diverges, at or after the first road-B landing: {}; P5 flipped journal diverges: {}; P6 netted journal (coincidences merged to their sum) replays exact: {}; P7 deduped journal (second kick at a coincidence dropped) diverges: {}; P8 the signed journal is not the small paper ({} B >= 10x checkpoint {} B): {}",
        b1, p1, p2, coinc.len(), p3, p4, p5, p6, p7,
        signed_bytes, checkpoint_bytes, p8
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    // --headless is handled BEFORE anything else; this specimen
    // has no TUI at all.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-two-roads: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_road_baseline_is_the_sheaf_centre() {
        let base = run_single();
        assert_eq!(base.perfect, 92);
        assert_eq!(base.super_cycles, 112);
        assert_eq!(base.count_slips, 3);
        assert_eq!(base.phase_slips, 17);
        assert_eq!(base.longest_episode, 58);
        assert!((base.mean_episode - 18.4).abs() < 0.05);
    }

    #[test]
    fn signed_journal_replays_exact() {
        let live = run_live();
        let replay = run_replay(&live.journal);
        assert_eq!(replay, live.follower_spikes);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), None);
    }

    #[test]
    fn derived_landings_match_per_road() {
        let live = run_live();
        let derived_a: Vec<usize> = live.a_spikes.iter().map(|s| s + DELAY_A + 1).filter(|&t| t < TICKS).collect();
        let derived_b: Vec<usize> = live.b_spikes.iter().map(|s| s + DELAY_B + 1).filter(|&t| t < TICKS).collect();
        assert_eq!(derived_a, live.a_landings);
        assert_eq!(derived_b, live.b_landings);
    }

    #[test]
    fn netted_journal_replays_exact() {
        // A tick's entries reach the follower as one sum; merging
        // them on paper loses nothing the ear could hear.
        let live = run_live();
        let mut netted: Vec<(usize, f32)> = Vec::new();
        for &(t, w) in &live.journal {
            if let Some(last) = netted.last_mut().filter(|(lt, _)| *lt == t) {
                last.1 += w;
            } else {
                netted.push((t, w));
            }
        }
        let replay = run_replay(&netted);
        assert_eq!(replay, live.follower_spikes);
    }

    #[test]
    fn live_two_roads_totals_pinned() {
        // As found on the first run, pinned: the second road
        // costs the pair its long episodes (longest 58 -> 16)
        // and loosens the count (3 -> 12 count slips) while the
        // phase slips stand at 17, as if only one road wandered.
        let live = run_live();
        assert_eq!(live.ledger.perfect, 83);
        assert_eq!(live.ledger.super_cycles, 112);
        assert_eq!(live.ledger.count_slips, 12);
        assert_eq!(live.ledger.phase_slips, 17);
        assert_eq!(live.ledger.episodes, 9);
        assert_eq!(live.ledger.longest_episode, 16);
        assert!((live.ledger.mean_episode - 9.2).abs() < 0.05);
        assert_eq!(live.a_spikes.len(), 607);
        assert_eq!(live.b_spikes.len(), 702);
        assert_eq!(live.a_landings.len(), 607);
        assert_eq!(live.b_landings.len(), 702);
        assert_eq!(live.journal.len(), 1309);
        assert_eq!(live.follower_spikes.len(), 857);
    }

    #[test]
    fn coincidences_pinned() {
        let live = run_live();
        let coinc = coincident_ticks(&live.journal);
        assert_eq!(coinc.len(), 24);
        assert_eq!(coinc.first(), Some(&464));
    }

    #[test]
    fn unsigned_and_flipped_journals_diverge_where_declared() {
        let live = run_live();
        let unsigned: Vec<(usize, f32)> =
            live.journal.iter().map(|&(t, _)| (t, WEIGHT_A)).collect();
        let div_u = first_divergence(&live.follower_spikes, &run_replay(&unsigned));
        assert_eq!(div_u, Some(64));
        // Never wrong before the first tick its spelling differs.
        assert!(div_u.unwrap() >= live.b_landings[0]);
        let flipped: Vec<(usize, f32)> =
            live.journal.iter().map(|&(t, w)| (t, -w)).collect();
        assert_eq!(
            first_divergence(&live.follower_spikes, &run_replay(&flipped)),
            Some(35)
        );
    }

    #[test]
    fn deduped_journal_diverges_after_the_first_coincidence() {
        // Netted and deduped write the same number of lines
        // (1,285); only the shared lines' contents differ. The
        // deduped spelling parts at tick 543 — after the first
        // coincidence at 464, as declared — and still reads a
        // HIGHER perfect count than the truth (86 vs 83).
        let live = run_live();
        let mut deduped: Vec<(usize, f32)> = Vec::new();
        for &(t, w) in &live.journal {
            if deduped.last().map(|&(lt, _)| lt) != Some(t) {
                deduped.push((t, w));
            }
        }
        assert_eq!(deduped.len(), 1285);
        let replay = run_replay(&deduped);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), Some(543));
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }

    #[test]
    fn coincidence_counter_counts_shared_ticks_once() {
        let j = vec![(5, -1.0), (5, 0.75), (9, -1.0), (12, 0.75), (12, -1.0)];
        assert_eq!(coincident_ticks(&j), vec![5, 12]);
    }
}
