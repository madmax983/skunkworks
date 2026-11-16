//! scarecrow-second-kick — The Scarecrow 🌾 plants
//! Harvest's journal and replay in
//! `scarecrow-second-kick`' two-road field, and silences
//! road B inside a ten-tick window at the second kick:
//! [10,035, 10,045), holding exactly one landing — the
//! one at tick 10,035 itself, the landing late-window's
//! surfacing did not wait for and one-kick's parting
//! never took.
//!
//! Seven windows stand in this row already: 9,000 ticks
//! (`scarecrow-windowed-silence`), 1,000
//! (`scarecrow-short-window`), 100
//! (`scarecrow-hundred-window`), 10
//! (`scarecrow-ten-window`), 1,000 after the tender
//! tick (`scarecrow-after-window`), 9,000 late
//! (`scarecrow-late-window`), and one kick at the gate
//! (`scarecrow-second-kick`, [10,000, 10,010)). Every one
//! of them parted the train and none of them healed:
//! the partings stood to ticks 23,992–23,999, and the
//! row's law is *lose nothing*. One-kick's closing
//! seed asked for this planting verbatim: *the second
//! kick alone — silence only [10,035, 10,045), one
//! landing at 10,035, the kick late-window's surfacing
//! did not wait for and one-kick's parting never took:
//! does a parting open from the second kick alone, and
//! at what address — if the gate kick is the tender
//! one, the second kick may part the train late, or
//! not at all, and either answer prices what made
//! 10,000 tender.*
//!
//! The field survey counted the host's landings before
//! this crate was planted (Phase 1, in the open): road
//! B lands 293 times before tick 10,035 (292 before
//! 10,000 and the gate kick itself at 10,000, which
//! this window leaves exact), exactly once inside
//! [10,035, 10,045) — at tick 10,035 itself — and its
//! next landing stands after the window has closed.
//! Inside the window the dose is one kick. Before it,
//! the journal is exact for 10,035 ticks — including
//! the gate kick at 10,000, taken whole. After it, the
//! journal is exact for the season's whole remainder.
//!
//! The gate kick was tender: it parted the train at
//! 10,005, five ticks after it landed, and the parting
//! stood to 23,999. This window asks whether tenderness
//! is a property of the address (10,000, the gate) or
//! of any single kick taken alone. If the second kick
//! parts too, at a wait inside the row's known list
//! (5, 32, 83, 130), then a lone kick is a lone kick
//! wherever it stands. If it parts late, or not at
//! all, then 10,000 was a tender place and 10,035 is
//! only a place — and the row learns what a tender
//! place is by finding one that is not.
//!
//! A parting is measured three ways: the first tick it
//! opens, the last tick it is open, and the count of
//! disagreed ticks in each region of the season
//! (before the window, inside it, after it, and in the
//! tails at and after 20,000, 22,000, and 23,000). The
//! rejoin tick is the first tick at or after 10,045
//! from which the two trains agree to the season's
//! end. This crate also replays late-window's spelling
//! ([10,000, 19,000)) and one-kick's spelling
//! ([10,000, 10,010)) as anchors, and counts where the
//! second-kick train and the one-kick train separate:
//! the second-kick journal is exact at 10,000 where
//! the one-kick journal is silent, so the two trains
//! must separate where one-kick's own parting opened,
//! at 10,005, if the anchors hold.
//!
//! Journals written and replayed (the follower alone
//! each time, one neuron, no synapses — the journals
//! are the whole coupling):
//!
//! - Signed: (tick, weight) entries as they landed.
//!   Control.
//! - Silenced (full season): road B written 0.0
//!   everywhere. The spurless anchor, re-measured here:
//!   it should be the baseline animal entire, and part
//!   from the truth at tick 65.
//! - Second-kick-silenced: road B written 0.0 only on
//!   ticks in [10,035, 10,045). The seed's spelling.
//!   Every line kept; the one line inside the window
//!   says nothing.
//! - Second-kick-dropped: road B's entry inside the
//!   window removed from the book outright. If a 0.0
//!   entry is no entry — spurless' law — this book
//!   replays to the second-kick-silenced train exact,
//!   and the two differ only in paper.
//! - Late-silenced: road B written 0.0 on ticks in
//!   [10,000, 19,000). Late-window's own spelling,
//!   re-measured here as an anchor: parting at 10,005,
//!   standing to 23,992, 940 disagreed ticks.
//! - One-kick-silenced: road B written 0.0 only on
//!   ticks in [10,000, 10,010). One-kick's own spelling,
//!   re-measured here as an anchor: parting at 10,005,
//!   standing to 23,999, 938 disagreed ticks.
//!
//! The counting instrument is the Widow's own, copied
//! faithfully from `scarecrow-second-kick` (super-cycles
//! of five driver-A cycles, seven follower beats, phase
//! tolerance ±0.06), and a baseline run of the
//! single-road pair pins this crate to the sheaf's
//! published centre.

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
/// The silence window: [WINDOW_START, WINDOW_END) —
/// ten ticks just past the season's midpoint, holding
/// exactly one road-B landing (at 10,035 itself) — the
/// second kick, the one late-window's surfacing did
/// not wait for and one-kick's parting never took.
const WINDOW_START: usize = 10_035;
const WINDOW_END: usize = 10_045;
/// Late-window's window, kept as this crate's anchor:
/// road B silenced on [10,000, 19,000).
const LATE_WINDOW_START: usize = 10_000;
const LATE_WINDOW_END: usize = 19_000;
/// One-kick's window, kept as this crate's second
/// anchor: road B silenced on [10,000, 10,010).
const ONEKICK_WINDOW_START: usize = 10_000;
const ONEKICK_WINDOW_END: usize = 10_010;
/// Phase tolerance for a beat to count as kept.
const PHASE_TOL: f64 = 0.06;
/// Minimum longest episode for the EPISODIC verdict.
const EPISODIC_FLOOR: usize = 10;
/// Signed journal record: u16 tick + i8 weight in quarter
/// units. A silenced entry is counted at the same 3 bytes:
/// writing 0.0 still takes a line, inside a window as
/// anywhere else. A dropped entry takes no line at all.
const SIGNED_ENTRY_BYTES: usize = 3;

#[derive(Debug, Clone, PartialEq)]
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
/// Returns the cell and the follower's full spike train.
fn run_single() -> (CellLedger, Vec<usize>) {
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
    let cell = count_cell(&post_settle(&driver_spikes), &post_settle(&follower_spikes));
    (cell, follower_spikes)
}

struct LiveRun {
    a_spikes: Vec<usize>,
    b_spikes: Vec<usize>,
    a_landings: Vec<usize>,
    b_landings: Vec<usize>,
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
/// for that tick. No synapses, no drivers — the journal is
/// the whole coupling.
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

/// Ticks carrying more than one journal entry, in order.
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

/// All ticks on which two spike trains disagree: ticks in
/// either train but not both, in order. (Spike trains hold
/// each tick at most once.)
fn symdiff_ticks(a: &[usize], b: &[usize]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    loop {
        match (a.get(i), b.get(j)) {
            (Some(&x), Some(&y)) => {
                if x == y {
                    i += 1;
                    j += 1;
                } else if x < y {
                    out.push(x);
                    i += 1;
                } else {
                    out.push(y);
                    j += 1;
                }
            }
            (Some(&x), None) => {
                out.push(x);
                i += 1;
            }
            (None, Some(&y)) => {
                out.push(y);
                j += 1;
            }
            (None, None) => return out,
        }
    }
}

fn count_in(ticks: &[usize], lo: usize, hi: usize) -> usize {
    ticks.iter().filter(|&&t| t >= lo && t < hi).count()
}

fn fmt_cell(name: &str, c: &CellLedger) {
    println!(
        "{}: perfect {}/{}, count slips {}, phase slips {}, episodes {}, longest {}, mean episode {:.1}, verdict {}",
        name, c.perfect, c.super_cycles, c.count_slips, c.phase_slips,
        c.episodes, c.longest_episode, c.mean_episode, c.verdict
    );
}

/// The full-season silenced journal (spurless' spelling):
/// road B written 0.0 everywhere, ticks kept.
fn silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| if w > 0.0 { (t, 0.0) } else { (t, w) })
        .collect()
}

/// The second-kick silenced journal: the seed's spelling.
/// Road B's entries on ticks inside the window are written
/// 0.0; every other entry is exact. All lines kept.
fn windowed_silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| {
            if w > 0.0 && t >= WINDOW_START && t < WINDOW_END {
                (t, 0.0)
            } else {
                (t, w)
            }
        })
        .collect()
}

/// The second-kick-dropped journal: road B's entry inside
/// the window removed outright; every other entry exact.
fn windowed_dropped_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .copied()
        .filter(|&(t, w)| !(w > 0.0 && t >= WINDOW_START && t < WINDOW_END))
        .collect()
}


/// Late-window's own spelling, re-measured as an anchor:
/// road B written 0.0 on ticks in [10,000, 19,000).
fn late_silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| {
            if w > 0.0 && t >= LATE_WINDOW_START && t < LATE_WINDOW_END {
                (t, 0.0)
            } else {
                (t, w)
            }
        })
        .collect()
}


/// One-kick's own spelling, re-measured as an anchor:
/// road B written 0.0 on ticks in [10,000, 10,010).
fn onekick_silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| {
            if w > 0.0 && t >= ONEKICK_WINDOW_START && t < ONEKICK_WINDOW_END {
                (t, 0.0)
            } else {
                (t, w)
            }
        })
        .collect()
}

fn headless() {
    println!("scarecrow-second-kick headless ledger");
    println!(
        "host: scarecrow-second-kick's follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}; silence window [{}, {}) — ten ticks, one landing, the second kick",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS,
        WINDOW_START, WINDOW_END
    );
    println!("principle: journal and replay (Harvest) — the journal is exact everywhere except ten ticks past the gate; inside them road B's one landing — the second kick, at 10035 — is written 0.0. The follower is replayed alone from each spelling, never opened. Survey, counted before planting: road B lands exactly once inside [10035, 10045), at tick 10035 itself, 293 times before the window (the gate kick at 10000 among them, taken exact here), and its next landing stands after this window has closed. One-kick's window at the gate parted at 10005 (lag 5, the fourth wait) and stood to 23999 on one kick alone; this window asks whether the second kick, taken alone, is tender too, or whether tenderness belonged to the gate");

    let (base, base_train) = run_single();
    fmt_cell("baseline-single-road", &base);
    println!(
        "baseline-single-road: follower spikes {} (all ticks, settle included)",
        base_train.len()
    );

    let live = run_live();
    fmt_cell("live-two-roads", &live.ledger);
    println!(
        "journals: driver A spikes {}, driver B spikes {}, road A landings {}, road B landings {}, signed entries {}, follower spikes {} (all ticks, settle included)",
        live.a_spikes.len(), live.b_spikes.len(),
        live.a_landings.len(), live.b_landings.len(),
        live.journal.len(), live.follower_spikes.len()
    );
    let coinc = coincident_ticks(&live.journal);
    println!(
        "coincidences: {} ticks carry both roads' kicks; first at {:?}",
        coinc.len(), coinc.first()
    );
    let b_in_window = live
        .b_landings
        .iter()
        .filter(|&&t| t >= WINDOW_START && t < WINDOW_END)
        .count();
    let first_b_in_window = live
        .b_landings
        .iter()
        .copied()
        .find(|&t| t >= WINDOW_START && t < WINDOW_END);
    let next_b_after = live
        .b_landings
        .iter()
        .copied()
        .find(|&t| t >= WINDOW_END);
    println!(
        "window: road B landings before {} {}, inside [{}, {}) {}, at/after {} {}; first inside {:?}, next landing at/after the window {:?}",
        WINDOW_START,
        count_in(&live.b_landings, 0, WINDOW_START),
        WINDOW_START,
        WINDOW_END,
        b_in_window,
        WINDOW_END,
        count_in(&live.b_landings, WINDOW_END, TICKS),
        first_b_in_window,
        next_b_after
    );

    let checkpoint_bytes = 3 * 6 * 4 + 8;
    let signed_bytes = live.journal.len() * SIGNED_ENTRY_BYTES;
    println!(
        "paper sizes: one whole checkpoint of the field {} bytes (3 neurons x 6 f32 + flags); signed journal {} entries x {} B = {} bytes",
        checkpoint_bytes, live.journal.len(), SIGNED_ENTRY_BYTES, signed_bytes
    );

    let a_post = post_settle(&live.a_spikes);
    let cell_of = |train: &[usize]| count_cell(&a_post, &post_settle(train));

    // Signed: the control.
    let replay_signed = run_replay(&live.journal);
    let div_signed = first_divergence(&live.follower_spikes, &replay_signed);
    fmt_cell("replay-signed", &cell_of(&replay_signed));
    println!(
        "replay-signed: follower train identical {}, first divergence {:?}",
        div_signed.is_none(), div_signed
    );

    // Full-season silenced: the spurless anchor.
    let silenced = silenced_journal(&live.journal);
    let replay_silenced = run_replay(&silenced);
    let div_silenced = first_divergence(&live.follower_spikes, &replay_silenced);
    fmt_cell("replay-silenced-full", &cell_of(&replay_silenced));
    println!(
        "replay-silenced-full: follower spikes {}, first divergence from live {:?}; identical to baseline train {} — the anchor from scarecrow-spurless, re-measured",
        replay_silenced.len(), div_silenced, replay_silenced == base_train
    );

    // Late-silenced: late-window's own spelling, an anchor —
    // 264 kicks, parting at 10005, standing to 23992, 940
    // disagreed ticks.
    let late = late_silenced_journal(&live.journal);
    let replay_late = run_replay(&late);
    let cell_late = cell_of(&replay_late);
    fmt_cell("replay-late-silenced", &cell_late);
    let diffs_late = symdiff_ticks(&live.follower_spikes, &replay_late);
    println!(
        "replay-late-silenced: {} entries ({} written 0.0 inside [{}, {})), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, total disagreed {} — late-window's ledger, re-measured in this crate",
        late.len(),
        late.iter().filter(|&&(_, w)| w == 0.0).count(),
        LATE_WINDOW_START, LATE_WINDOW_END,
        replay_late.len(), live.follower_spikes.len(),
        diffs_late.first(), diffs_late.last(), diffs_late.len()
    );

    // One-kick-silenced: one-kick's own spelling, the second
    // anchor — parting at 10005, standing to 23999, 938
    // disagreed ticks.
    let onekick = onekick_silenced_journal(&live.journal);
    let replay_onekick = run_replay(&onekick);
    let cell_onekick = cell_of(&replay_onekick);
    fmt_cell("replay-onekick-silenced", &cell_onekick);
    let diffs_onekick = symdiff_ticks(&live.follower_spikes, &replay_onekick);
    println!(
        "replay-onekick-silenced: {} entries ({} written 0.0 inside [{}, {})), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, total disagreed {} — one-kick's ledger, re-measured in this crate",
        onekick.len(),
        onekick.iter().filter(|&&(_, w)| w == 0.0).count(),
        ONEKICK_WINDOW_START, ONEKICK_WINDOW_END,
        replay_onekick.len(), live.follower_spikes.len(),
        diffs_onekick.first(), diffs_onekick.last(), diffs_onekick.len()
    );

    // Second-kick-silenced: the seed's spelling.
    let windowed = windowed_silenced_journal(&live.journal);
    let replay_windowed = run_replay(&windowed);
    let cell_windowed = cell_of(&replay_windowed);
    fmt_cell("replay-secondkick-silenced", &cell_windowed);
    let diffs = symdiff_ticks(&live.follower_spikes, &replay_windowed);
    let first_div = diffs.first().copied();
    let last_div = diffs.last().copied();
    let d_before = count_in(&diffs, 0, WINDOW_START);
    let d_inside = count_in(&diffs, WINDOW_START, WINDOW_END);
    let d_after = count_in(&diffs, WINDOW_END, TICKS);
    let d_after20 = count_in(&diffs, 20_000, TICKS);
    let d_after22 = count_in(&diffs, 22_000, TICKS);
    let d_after23 = count_in(&diffs, 23_000, TICKS);
    // The rejoin tick: the first tick at or after the window's
    // end from which the two trains agree to the season's end.
    let rejoin_tick = if diffs.is_empty() {
        WINDOW_END
    } else {
        match diffs.iter().filter(|&&t| t >= WINDOW_END).last() {
            None => WINDOW_END,
            Some(&l) => l + 1,
        }
    };
    println!(
        "replay-secondkick-silenced: {} entries ({} written 0.0 inside the window), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, rejoin tick {} (first tick >= {} from which the trains agree to the end; {} = no rejoin observed)",
        windowed.len(),
        b_in_window,
        replay_windowed.len(),
        live.follower_spikes.len(),
        first_div, last_div, rejoin_tick, WINDOW_END, TICKS
    );
    println!(
        "second-kick parting by region (disagreed ticks): before [{}, {}) {}, inside [{}, {}) {}, after [{}, {}) {}, at/after 20000 {}, at/after 22000 {}, at/after 23000 {}; total disagreed {}",
        0, WINDOW_START, d_before,
        WINDOW_START, WINDOW_END, d_inside,
        WINDOW_END, TICKS, d_after,
        d_after20, d_after22, d_after23, diffs.len()
    );
    println!(
        "follower spikes by region: live before {} inside {} after {}; second-kick before {} inside {} after {}",
        count_in(&live.follower_spikes, 0, WINDOW_START),
        count_in(&live.follower_spikes, WINDOW_START, WINDOW_END),
        count_in(&live.follower_spikes, WINDOW_END, TICKS),
        count_in(&replay_windowed, 0, WINDOW_START),
        count_in(&replay_windowed, WINDOW_START, WINDOW_END),
        count_in(&replay_windowed, WINDOW_END, TICKS)
    );
    // The second-kick train against the one-kick train: the
    // second-kick journal is exact at 10000, where the
    // one-kick journal is silent, and silent at 10035, where
    // the one-kick journal is exact. Until 10035 the
    // second-kick train is the live train, so the two must
    // separate exactly where one-kick's parting opened.
    let diffs_vs_onekick = symdiff_ticks(&replay_windowed, &replay_onekick);
    println!(
        "second-kick vs one-kick train: disagreed ticks {}, first at {:?}, disagreed before 10005 {} — the second-kick train is the live train until 10035, so the separation opens where one-kick's own parting opened",
        diffs_vs_onekick.len(), diffs_vs_onekick.first(),
        count_in(&diffs_vs_onekick, 0, 10_005)
    );
    // Inside the window the second-kick animal eats the
    // baseline's diet for ten ticks. Count its disagreements
    // with the baseline train inside the window.
    let diffs_base = symdiff_ticks(&base_train, &replay_windowed);
    println!(
        "second-kick vs baseline inside the window: disagreed ticks {} in [{}, {}) — the windowed animal enters the window in the truth's state, not the baseline's, and the two are counted apart here",
        count_in(&diffs_base, WINDOW_START, WINDOW_END), WINDOW_START, WINDOW_END
    );

    // Second-kick-dropped: the same silence, written as absence.
    let dropped = windowed_dropped_journal(&live.journal);
    let dropped_bytes = dropped.len() * SIGNED_ENTRY_BYTES;
    let replay_dropped = run_replay(&dropped);
    fmt_cell("replay-secondkick-dropped", &cell_of(&replay_dropped));
    println!(
        "replay-secondkick-dropped: {} entries, follower spikes {}, identical to second-kick-silenced train {}; paper {} B against the second-kick-silenced book's {} B — absence inside the window saves {} B, silence saves none",
        dropped.len(), replay_dropped.len(),
        replay_dropped == replay_windowed,
        dropped_bytes, signed_bytes, signed_bytes - dropped_bytes
    );

    let b1 = base.perfect == 92
        && base.super_cycles == 112
        && base.count_slips == 3
        && base.phase_slips == 17
        && base.longest_episode == 58
        && (base.mean_episode - 18.4).abs() < 0.05;
    let b2 = live.ledger.perfect == 83
        && live.ledger.count_slips == 12
        && live.ledger.phase_slips == 17
        && live.ledger.episodes == 9
        && live.ledger.longest_episode == 16
        && live.follower_spikes.len() == 857
        && live.journal.len() == 1309
        && coinc.len() == 24
        && coinc.first() == Some(&464);
    let b3 = replay_silenced == base_train && div_silenced == Some(65);
    // B4 (anchor): late-window's spelling, re-measured in
    // this crate, reproduces late-window's ledger entire —
    // 264 silenced, parting at 10005, last disagreed 23992,
    // 940 disagreed ticks.
    let b4 = diffs_late.first() == Some(&10_005)
        && diffs_late.last() == Some(&23_992)
        && diffs_late.len() == 940
        && late.iter().filter(|&&(_, w)| w == 0.0).count() == 264;
    // B5 (anchor): one-kick's spelling, re-measured in this
    // crate, reproduces one-kick's ledger entire — 1
    // silenced, parting at 10005, last disagreed 23999, 938
    // disagreed ticks.
    let b5 = diffs_onekick.first() == Some(&10_005)
        && diffs_onekick.last() == Some(&23_999)
        && diffs_onekick.len() == 938
        && onekick.iter().filter(|&&(_, w)| w == 0.0).count() == 1;
    let p1 = div_signed.is_none();
    let p2 = d_before == 0 && first_div.map(|d| d >= WINDOW_START).unwrap_or(false);
    // P3 (causality): the only silenced landing stands at
    // tick 10,035 — the window's own first tick — so nothing
    // upstream of it was touched: no disagreed tick can
    // stand before 10,035. Here the causality fence and the
    // window's edge are the same tick; P2 and P3 state one
    // fact from two sides, and both are kept because the
    // row has always kept both.
    let p3 = count_in(&diffs, 0, 10_035) == 0;
    // P4 (the seed's ignition): the second kick parts the
    // train on its own, at a wait inside the row's known
    // list — the waits stand at 5, 32, 83 and 130, so the
    // first disagreed tick stands in (10035, 10165]. If no
    // parting opens, or it opens later than any wait this
    // row has recorded, then tenderness was the gate's,
    // not any lone kick's.
    let p4 = first_div.map(|d| d > 10_035 && d <= 10_165).unwrap_or(false);
    // P5 (the seed's standing): if the second kick parts,
    // the parting stands the way every parting in this row
    // has stood — disagreed ticks at/after 23,000 and the
    // last disagreed tick at/after 23,900. A parting that
    // closes once the exact journal resumes at 10,045 is
    // this row's first healing, and it cracks here.
    let p5 = d_after23 > 0 && last_div.map(|l| l >= 23_900).unwrap_or(false);
    let p6 = replay_dropped == replay_windowed;
    // P7 (the separation opens at one-kick's address): the
    // second-kick train is the live train until 10035, and
    // the one-kick train parts from the live train at
    // 10005 — so the two windowed trains separate first at
    // exactly 10005, on the gate kick the second-kick
    // journal kept and the one-kick journal silenced.
    let p7 = diffs_vs_onekick.first() == Some(&10_005);
    let windowed_bytes = windowed.len() * SIGNED_ENTRY_BYTES;
    let p8 = windowed_bytes == signed_bytes
        && signed_bytes - dropped_bytes == 3 * b_in_window
        && b_in_window == 1
        && first_b_in_window == Some(10_035)
        && next_b_after == Some(10069);
    // P9 (the wreck's size): if the second kick stands the
    // way the gate kick stood, the wreck is the family's
    // wreck at the family's rate — total disagreed ticks
    // in 700..=1100, the second-kick cell's perfect count
    // in the burned seasons' band 70..=90. A healing, or
    // no parting at all, reads far below the band; that is
    // the crack this prediction is priced to catch.
    let p9 = (700..=1100).contains(&diffs.len())
        && (70..=90).contains(&cell_windowed.perfect);
    println!(
        "verdicts: B1 single-road baseline reproduces the sheaf centre exact: {}; B2 live two-road season reproduces two-roads/spurless exact: {}; B3 full-season silenced anchor is the baseline animal and parts at 65: {}; B4 late-window anchor reproduces late-window's ledger entire (264 silenced, parting 10005, last 23992, 940 disagreed): {}; B5 one-kick anchor reproduces one-kick's ledger entire (1 silenced, parting 10005, last 23999, 938 disagreed): {}; P1 signed journal replays the follower exact: {}; P2 second-kick journal is exact before the window (no disagreed tick before {}): {}; P3 causality — no disagreed tick before the first silenced landing at 10035, the window's own first tick: {}; P4 the second kick parts on its own at a known wait — first disagreed tick {:?} in (10035, 10165]: {}; P5 the standing needs no following silence (disagreed at/after 23000 = {}, last disagreed {:?} >= 23900; rejoin tick {}): {}; P6 second-kick-dropped replays to the second-kick-silenced train exact (a 0.0 entry is no entry, in second-kick form): {}; P7 the separation from the one-kick train opens at one-kick's own address — first separation {:?} == 10005 (separations {}): {}; P8 silence in a window costs full paper ({} B == signed {} B) and absence saves exactly 3 B for the one silenced landing ({} landing, first at {:?}, next landing {:?}, saving {} B): {}; P9 the wreck is the family's size on the second kick too (total disagreed {} in 700..=1100; perfect {} in 70..=90): {}",
        b1, b2, b3, b4, b5, p1, WINDOW_START, p2, p3,
        first_div, p4, d_after23, last_div, rejoin_tick, p5, p6,
        diffs_vs_onekick.first(), diffs_vs_onekick.len(), p7,
        windowed_bytes, signed_bytes, b_in_window, first_b_in_window, next_b_after, signed_bytes - dropped_bytes, p8,
        diffs.len(), cell_windowed.perfect, p9
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-second-kick: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_road_baseline_is_the_sheaf_centre() {
        let (base, _) = run_single();
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
    fn windowed_spellings_touch_only_road_b_inside_the_window() {
        let j = vec![
            (5, -1.0),
            (10_034, 0.75),
            (10_035, 0.75),
            (10_040, -1.0),
            (10_044, 0.75),
            (10_045, 0.75),
            (19_000, 0.75),
        ];
        assert_eq!(
            windowed_silenced_journal(&j),
            vec![
                (5, -1.0),
                (10_034, 0.75),
                (10_035, 0.0),
                (10_040, -1.0),
                (10_044, 0.0),
                (10_045, 0.75),
                (19_000, 0.75),
            ]
        );
        assert_eq!(
            windowed_dropped_journal(&j),
            vec![(5, -1.0), (10_034, 0.75), (10_040, -1.0), (10_045, 0.75), (19_000, 0.75)]
        );
    }

    #[test]
    fn symdiff_and_region_counts_on_planted_trains() {
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 9, 20]), Vec::<usize>::new());
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 10, 20]), vec![9, 10]);
        assert_eq!(symdiff_ticks(&[3, 9], &[3, 9, 20]), vec![20]);
        assert_eq!(count_in(&[10_034, 10_035, 10_044, 10_045, 19_000], 10_035, 10_045), 2);
    }

    #[test]
    fn full_silenced_anchor_is_the_baseline_animal() {
        let (_, base_train) = run_single();
        let live = run_live();
        let replay = run_replay(&silenced_journal(&live.journal));
        assert_eq!(replay, base_train);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), Some(65));
    }

    #[test]
    fn windowed_dropped_is_windowed_silenced() {
        // Spurless' law in window form: a 0.0 entry is no
        // entry. Dropping the window's road-B line changes
        // the paper and nothing the ear hears.
        let live = run_live();
        assert_eq!(
            run_replay(&windowed_dropped_journal(&live.journal)),
            run_replay(&windowed_silenced_journal(&live.journal))
        );
    }

    #[test]
    fn windowed_journal_is_exact_before_the_window() {
        // The spelling differs from the truth only inside
        // [10035, 10045), and the replay machinery is exact,
        // so no disagreed tick can stand before the window.
        let live = run_live();
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(count_in(&diffs, 0, WINDOW_START), 0);
        assert!(diffs.first().map(|&d| d >= WINDOW_START).unwrap_or(true));
    }

    #[test]
    fn nothing_parts_before_the_first_silenced_landing() {
        // Causality, this window's own fence: the only
        // entry this spelling touches lands at tick 10035,
        // the window's own first tick. Every tick before
        // it is replayed from an exact journal in an exact
        // state, so no disagreed tick can stand before it.
        let live = run_live();
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(count_in(&diffs, 0, 10_035), 0);
    }

    #[test]
    fn the_second_kick_window_silences_one_landing_at_10035() {
        // The survey's count, asserted in the flesh: road B
        // lands exactly once inside [10035, 10045), at
        // 10035 itself — and its next landing stands after
        // the window has closed, at the surveyed tick.
        let live = run_live();
        let inside: Vec<usize> = live
            .b_landings
            .iter()
            .copied()
            .filter(|&t| t >= WINDOW_START && t < WINDOW_END)
            .collect();
        assert_eq!(inside, vec![10_035]);
        let next = live
            .b_landings
            .iter()
            .copied()
            .find(|&t| t >= WINDOW_END);
        assert_eq!(next, Some(10069));
    }

    #[test]
    fn late_window_anchor_reproduces_late_windows_ledger() {
        // The anchor the row cites, re-measured in this
        // crate: late-window's spelling silences 264
        // landings, parts at 10005, stands to 23992, and
        // disagrees on 940 ticks.
        let live = run_live();
        let late = late_silenced_journal(&live.journal);
        assert_eq!(late.iter().filter(|&&(_, w)| w == 0.0).count(), 264);
        let replay = run_replay(&late);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&10_005));
        assert_eq!(diffs.last(), Some(&23_992));
        assert_eq!(diffs.len(), 940);
    }

    #[test]
    fn one_kick_anchor_reproduces_one_kicks_ledger() {
        // The second anchor, re-measured in this crate:
        // one-kick's spelling silences 1 landing at 10000,
        // parts at 10005, stands to 23999, and disagrees
        // on 938 ticks.
        let live = run_live();
        let onekick = onekick_silenced_journal(&live.journal);
        assert_eq!(onekick.iter().filter(|&&(_, w)| w == 0.0).count(), 1);
        let replay = run_replay(&onekick);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&10_005));
        assert_eq!(diffs.last(), Some(&23_999));
        assert_eq!(diffs.len(), 938);
    }

    #[test]
    fn second_kick_boundaries_in_the_spelling_test() {
        // The window is [10035, 10045): an entry at 10044
        // is silenced, an entry at 10045 is exact.
        let j = vec![(10_034, 0.75), (10_044, 0.75), (10_045, 0.75)];
        assert_eq!(
            windowed_silenced_journal(&j),
            vec![(10_034, 0.75), (10_044, 0.0), (10_045, 0.75)]
        );
        assert_eq!(
            windowed_dropped_journal(&j),
            vec![(10_034, 0.75), (10_045, 0.75)]
        );
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }
    #[test]
    fn second_kick_season_as_found() {
        // As found on the first run, pinned. Every prediction
        // held: the window silences exactly one landing (at
        // 10035) and the train parts at 10120 — a wait of 85
        // ticks, inside the declared band priced from the
        // row's known waits (5, 32, 83, 130) — and stands to
        // 23992, late-window's own last tick: 946 disagreed
        // ticks in all, none before the window, none inside
        // it, 946 after it, 72 at/after 23,000. Its cell
        // reads perfect 77, slips 16 and 19, episodes 13,
        // longest 16. Follower spikes 857, the truth's count
        // exact in every region (359/0/498). The second-kick
        // and one-kick trains separate first at 10005 —
        // one-kick's own surfacing, priced from the other
        // side — and disagree on 956 ticks in all. The
        // dropped spelling: 1,308 entries, 3,924 bytes —
        // absence saves 3 bytes and loses the season.
        let live = run_live();
        let windowed = windowed_silenced_journal(&live.journal);
        assert_eq!(
            windowed.iter().filter(|&&(_, w)| w == 0.0).count(),
            1
        );
        let replay = run_replay(&windowed);
        assert_eq!(replay.len(), 857);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&10_120));
        assert_eq!(diffs.last(), Some(&23_992));
        assert_eq!(count_in(&diffs, 0, WINDOW_START), 0);
        assert_eq!(count_in(&diffs, WINDOW_START, WINDOW_END), 0);
        assert_eq!(count_in(&diffs, WINDOW_END, TICKS), 946);
        assert_eq!(count_in(&diffs, 20_000, TICKS), 281);
        assert_eq!(count_in(&diffs, 23_000, TICKS), 72);
        assert_eq!(diffs.len(), 946);
        let cell = count_cell(&post_settle(&live.a_spikes), &post_settle(&replay));
        assert_eq!(cell.perfect, 77);
        assert_eq!(cell.count_slips, 16);
        assert_eq!(cell.phase_slips, 19);
        assert_eq!(cell.episodes, 13);
        assert_eq!(cell.longest_episode, 16);
        let onekick_replay = run_replay(&onekick_silenced_journal(&live.journal));
        let vs_onekick = symdiff_ticks(&replay, &onekick_replay);
        assert_eq!(vs_onekick.first(), Some(&10_005));
        assert_eq!(vs_onekick.len(), 956);
        let dropped = windowed_dropped_journal(&live.journal);
        assert_eq!(dropped.len(), 1308);
        assert_eq!(dropped.len() * SIGNED_ENTRY_BYTES, 3924);
    }
}

