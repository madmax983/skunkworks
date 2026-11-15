//! scarecrow-late-window — The Scarecrow 🌾 plants
//! Harvest's journal and replay in
//! `scarecrow-after-window`' two-road field, and silences
//! road B only inside one window in the season's second
//! half: ticks 10,000–19,000. Before the window the
//! journal is exact, for ten thousand ticks. Inside it,
//! road B's entries are written 0.0 — their ticks kept,
//! their kicks nothing. After the window the journal is
//! exact again, for only five thousand ticks — the
//! shortest remainder any window in this row has left
//! standing.
//!
//! Five windows stand in this row already: 9,000 ticks
//! (`scarecrow-windowed-silence`, [1,000, 10,000)), 1,000
//! (`scarecrow-short-window`), 100
//! (`scarecrow-hundred-window`), 10
//! (`scarecrow-ten-window`), and 1,000 after the tender
//! tick (`scarecrow-after-window`, [2,000, 3,000)). Every
//! one of them parted the train and none of them healed:
//! the partings stood to ticks 23,992–23,998, and the
//! row's law is *lose nothing*. After-window's closing
//! seed asked for this planting in substance: *a late
//! window (10,000–19,000): a window in the season's
//! second half still parts the train under the law —
//! but does it stand to the season's end when so little
//! season remains, or does a short remaining season
//! forgive what a long one could not; and if it parts,
//! is its lag behind its first taken kick a fourth
//! number on the row's list of waits (32, 83, 130), or
//! does the list gain a pattern late?*
//!
//! The field survey counted the host's landings before
//! this crate was planted (Phase 1, in the open): road
//! B lands 292 times before tick 10,000, exactly 264
//! times inside [10,000, 19,000) — first at tick 10,000
//! itself, the window's own first tick, last at 18,998,
//! spaced 34–35 ticks apart — and 146 times at/after
//! 19,000. Two hundred and sixty-four landings: the
//! long window's dose, at the season's far end.
//!
//! The row has measured three lags between a first taken
//! kick and a surfacing parting: 32 ticks (the long
//! window: taken 1015, surfaced 1047), 83 (the ten:
//! taken 1049, surfaced 1132), 130 (the after window:
//! taken 2010, surfaced 2140). After-window's law,
//! nineteenth in the Scarecrow's notes: the lag is a
//! wait for the follower's next tender place, not a
//! distance the machinery keeps. If some pattern governs
//! the waits late — if the list is a sequence and not a
//! scatter — this window's parting opens at 10,032,
//! 10,083, or 10,130, one of the three old addresses
//! carried forward. If the waits are waits, it opens at
//! a fourth address no arithmetic names. Note where the
//! shortest forecast stands: 10,032 is three ticks
//! before the window's second landing (10,035) — a
//! surfacing there would be a one-kick ignition, as the
//! ten window's was.
//!
//! The wreck is priced by rate here, not by size: with
//! only 5,000 ticks after the window, a total in the
//! family's old band (1,519–1,596) is arithmetically out
//! of reach whatever the law does, so the honest measure
//! is the disagreed density after the window. The five
//! earlier windows left 69–70 disagreed ticks per 1,000
//! ticks of remaining season (979 in 14,000 after the
//! long window; 1,515 in 22,000 after the short; 1,457
//! in 21,000 after the after window). A late window that
//! forgives would show it in the rate, not the total.
//!
//! A parting is measured three ways: the first tick it
//! opens, the last tick it is open, and the count of
//! disagreed ticks in each region of the season (before
//! the window, inside it, after it, and in the tails at
//! and after 20,000, 22,000, and 23,000). The rejoin
//! tick is the first tick at or after 19,000 from which
//! the two trains agree to the season's end.
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
//! - Windowed-silenced: road B written 0.0 only on
//!   ticks in [10,000, 19,000). The seed's spelling.
//!   Every line kept; the 264 lines inside the window
//!   say nothing.
//! - Windowed-dropped: road B's entries inside the
//!   window removed from the book outright. If a 0.0
//!   entry is no entry — spurless' law — this book
//!   replays to the windowed-silenced train exact, and
//!   the two differ only in paper.
//!
//! The counting instrument is the Widow's own, copied
//! faithfully from `scarecrow-after-window` (super-cycles
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
/// nine thousand ticks in the season's second half,
/// opening at the season's midpoint and closing only
/// 5,000 ticks before the season's end — the shortest
/// remainder any window in this row has left standing.
const WINDOW_START: usize = 10_000;
const WINDOW_END: usize = 19_000;
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

/// The late-window silenced journal: the seed's spelling.
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

/// The windowed-dropped journal: road B's entries inside
/// the window removed outright; every other entry exact.
fn windowed_dropped_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .copied()
        .filter(|&(t, w)| !(w > 0.0 && t >= WINDOW_START && t < WINDOW_END))
        .collect()
}

fn headless() {
    println!("scarecrow-late-window headless ledger");
    println!(
        "host: scarecrow-spurless' follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}; silence window [{}, {})",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS,
        WINDOW_START, WINDOW_END
    );
    println!("principle: journal and replay (Harvest) — the journal is exact before and after the window; inside it road B is written 0.0. The follower is replayed alone from each spelling, never opened. Survey, counted before planting: road B lands 264 times inside [10000, 19000), first at tick 10000 itself, last at 18998 — a window in the season's second half with only 5000 ticks after it; the row's measured lags are 32, 83, 130, so a patterned lag opens this parting at 10032, 10083, or 10130");

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
    println!(
        "window: road B landings before {} {}, inside [{}, {}) {}, at/after {} {}; first inside {:?}",
        WINDOW_START,
        count_in(&live.b_landings, 0, WINDOW_START),
        WINDOW_START,
        WINDOW_END,
        b_in_window,
        WINDOW_END,
        count_in(&live.b_landings, WINDOW_END, TICKS),
        first_b_in_window
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

    // Windowed-silenced: the seed's spelling.
    let windowed = windowed_silenced_journal(&live.journal);
    let replay_windowed = run_replay(&windowed);
    let cell_windowed = cell_of(&replay_windowed);
    fmt_cell("replay-windowed-silenced", &cell_windowed);
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
    // If disagreements run to the last tick, this reads TICKS:
    // no rejoin was observed inside the season. If the trains
    // never disagree at all, this reads WINDOW_END.
    let rejoin_tick = if diffs.is_empty() {
        WINDOW_END
    } else {
        match diffs.iter().filter(|&&t| t >= WINDOW_END).last() {
            None => WINDOW_END,
            Some(&l) => l + 1,
        }
    };
    println!(
        "replay-windowed-silenced: {} entries ({} written 0.0 inside the window), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, rejoin tick {} (first tick >= {} from which the trains agree to the end; {} = no rejoin observed)",
        windowed.len(),
        b_in_window,
        replay_windowed.len(),
        live.follower_spikes.len(),
        first_div, last_div, rejoin_tick, WINDOW_END, TICKS
    );
    println!(
        "windowed parting by region (disagreed ticks): before [{}, {}) {}, inside [{}, {}) {}, after [{}, {}) {}, at/after 20000 {}, at/after 22000 {}, at/after 23000 {}; total disagreed {}",
        0, WINDOW_START, d_before,
        WINDOW_START, WINDOW_END, d_inside,
        WINDOW_END, TICKS, d_after,
        d_after20, d_after22, d_after23, diffs.len()
    );
    println!(
        "follower spikes by region: live before {} inside {} after {}; windowed before {} inside {} after {}",
        count_in(&live.follower_spikes, 0, WINDOW_START),
        count_in(&live.follower_spikes, WINDOW_START, WINDOW_END),
        count_in(&live.follower_spikes, WINDOW_END, TICKS),
        count_in(&replay_windowed, 0, WINDOW_START),
        count_in(&replay_windowed, WINDOW_START, WINDOW_END),
        count_in(&replay_windowed, WINDOW_END, TICKS)
    );
    // Inside the window the windowed animal eats the baseline's
    // diet. Is it the baseline animal? Count its disagreements
    // with the baseline train inside the window.
    let diffs_base = symdiff_ticks(&base_train, &replay_windowed);
    println!(
        "windowed vs baseline inside the window: disagreed ticks {} in [{}, {}) — the windowed animal enters the window in the truth's state, not the baseline's, and the two are counted apart here",
        count_in(&diffs_base, WINDOW_START, WINDOW_END), WINDOW_START, WINDOW_END
    );

    // Windowed-dropped: the same silence, written as absence.
    let dropped = windowed_dropped_journal(&live.journal);
    let dropped_bytes = dropped.len() * SIGNED_ENTRY_BYTES;
    let replay_dropped = run_replay(&dropped);
    fmt_cell("replay-windowed-dropped", &cell_of(&replay_dropped));
    println!(
        "replay-windowed-dropped: {} entries, follower spikes {}, identical to windowed-silenced train {}; paper {} B against the windowed-silenced book's {} B — absence inside the window saves {} B, silence saves none",
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
    let p1 = div_signed.is_none();
    let p2 = d_before == 0 && first_div.map(|d| d >= WINDOW_START).unwrap_or(false);
    // P3 (causality): the first silenced landing stands at
    // tick 10,000 — the window's own first tick — so nothing
    // upstream of it was touched: no disagreed tick can
    // stand before 10,000. Here the causality fence and the
    // window's edge are the same tick; P2 and P3 state one
    // fact from two sides, and both are kept because the
    // row has always kept both.
    let p3 = count_in(&diffs, 0, 10_000) == 0;
    // P4 (ignition, under the row's law): lose nothing —
    // a window in the second half ignites too, and the
    // parting does not heal even with only 5,000 ticks
    // left to stand in: disagreed ticks stand at/after
    // 23,000 and the last disagreed tick stands at/after
    // 23,900, where every earlier window's parting stood
    // (23,992–23,998). If the short remainder forgives,
    // it shows here first.
    let p4 = d_after23 > 0 && last_div.map(|l| l >= 23_900).unwrap_or(false);
    // P5 (the seed's question): the lag gains a pattern
    // late. The row's waits are 32, 83, 130; a patterned
    // list repeats one of them, opening this parting at
    // 10,032, 10,083, or 10,130. After-window's law says
    // the lag is a wait, not a distance — so this
    // prediction declares the wait is a FOURTH number:
    // the parting opens, and its lag behind the first
    // taken kick (10,000) is none of 32, 83, 130.
    let lag = first_div.map(|d| d - WINDOW_START);
    let p5 = matches!(lag, Some(l) if l != 32 && l != 83 && l != 130);
    let p6 = replay_dropped == replay_windowed;
    let p7 = count_in(&diffs_base, WINDOW_START, WINDOW_END) > 0;
    let windowed_bytes = windowed.len() * SIGNED_ENTRY_BYTES;
    let p8 = windowed_bytes == signed_bytes
        && signed_bytes - dropped_bytes == 3 * b_in_window
        && b_in_window == 264
        && first_b_in_window == Some(10_000);
    // P9 (the wreck's rate): the five earlier windows
    // left 69–70 disagreed ticks per 1,000 ticks of
    // season after their windows (979/14,000; 1,515/
    // 22,000; 1,457/21,000). Priced at that rate, the
    // 5,000 ticks after this window carry 300..=400
    // disagreed ticks; a forgiving remainder reads
    // below the band, a harsher one above it. The
    // windowed cell's perfect count stands in the
    // burned seasons' band, widened at the top for the
    // after window's crown (73–86): 70..=90.
    let p9 = (300..=400).contains(&d_after)
        && (70..=90).contains(&cell_windowed.perfect);
    println!(
        "verdicts: B1 single-road baseline reproduces the sheaf centre exact: {}; B2 live two-road season reproduces two-roads/spurless exact: {}; B3 full-season silenced anchor is the baseline animal and parts at 65: {}; P1 signed journal replays the follower exact: {}; P2 windowed journal is exact before the window (no disagreed tick before {}): {}; P3 causality — no disagreed tick before the first silenced landing at 10000, the window's own first tick: {}; P4 ignition in the second half does not heal in a short remainder (disagreed at/after 23000 = {}, last disagreed {:?} >= 23900; rejoin tick {}): {}; P5 the lag is a fourth wait — first disagreed tick {:?}, lag {:?} behind 10000, none of 32/83/130 (forecast addresses 10032/10083/10130): {}; P6 windowed-dropped replays to the windowed-silenced train exact (a 0.0 entry is no entry, in window form): {}; P7 the windowed animal is not the baseline animal inside the window (disagreed ticks vs baseline inside > 0): {}; P8 silence in a window costs full paper ({} B == signed {} B) and absence saves exactly 3 B per silenced landing ({} landings, first at {:?}, saving {} B): {}; P9 the wreck's rate stands in the family's band (disagreed after the window {} in 300..=400 over 5000 ticks — 60..=80 per 1000 against the family's 69–70; perfect {} in 70..=90): {}",
        b1, b2, b3, p1, WINDOW_START, p2, p3,
        d_after23, last_div, rejoin_tick, p4, first_div, lag, p5, p6, p7,
        windowed_bytes, signed_bytes, b_in_window, first_b_in_window, signed_bytes - dropped_bytes, p8,
        d_after, cell_windowed.perfect, p9
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-late-window: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
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
            (9_999, 0.75),
            (10_000, 0.75),
            (10_045, -1.0),
            (10_010, 0.75),
            (19_000, 0.75),
            (20, -1.0),
        ];
        assert_eq!(
            windowed_silenced_journal(&j),
            vec![
                (5, -1.0),
                (9_999, 0.75),
                (10_000, 0.0),
                (10_045, -1.0),
                (10_010, 0.0),
                (19_000, 0.75),
                (20, -1.0),
            ]
        );
        assert_eq!(
            windowed_dropped_journal(&j),
            vec![(5, -1.0), (9_999, 0.75), (10_045, -1.0), (19_000, 0.75), (20, -1.0)]
        );
    }

    #[test]
    fn symdiff_and_region_counts_on_planted_trains() {
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 9, 20]), Vec::<usize>::new());
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 10, 20]), vec![9, 10]);
        assert_eq!(symdiff_ticks(&[3, 9], &[3, 9, 20]), vec![20]);
        assert_eq!(count_in(&[5, 9_999, 10_000, 10_010, 19_000], 10_000, 19_000), 2);
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
        // entry. Dropping the window's road-B lines changes
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
        // [10000, 19000), and the replay machinery is exact,
        // so no disagreed tick can stand before the window.
        let live = run_live();
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(count_in(&diffs, 0, WINDOW_START), 0);
        assert!(diffs.first().map(|&d| d >= WINDOW_START).unwrap_or(true));
    }

    #[test]
    fn nothing_parts_before_the_first_silenced_landing() {
        // Causality, this window's own fence: the first
        // entry this spelling touches lands at tick 10000,
        // the window's own first tick. Every tick before
        // it is replayed from an exact journal in an exact
        // state, so no disagreed tick can stand before it.
        let live = run_live();
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(count_in(&diffs, 0, 10_000), 0);
    }

    #[test]
    fn the_late_window_silences_264_landings_first_at_10000() {
        // The survey's count, asserted in the flesh: road B
        // lands 264 times inside [10000, 19000), first at
        // 10000 itself — the window's own first tick — and
        // last at 18998.
        let live = run_live();
        let inside: Vec<usize> = live
            .b_landings
            .iter()
            .copied()
            .filter(|&t| t >= WINDOW_START && t < WINDOW_END)
            .collect();
        assert_eq!(inside.len(), 264);
        assert_eq!(inside.first(), Some(&10_000));
        assert_eq!(inside.last(), Some(&18_998));
    }

    #[test]
    fn late_window_boundaries_in_the_spelling_test() {
        // The window is [10000, 19000): entries at 18999
        // are silenced, entries at 19000 are exact. (The
        // season-as-found pin is added after the first run,
        // in a follow-up edit that touches no prediction.)
        let j = vec![(9_999, 0.75), (18_999, 0.75), (19_000, 0.75)];
        assert_eq!(
            windowed_silenced_journal(&j),
            vec![(9_999, 0.75), (18_999, 0.0), (19_000, 0.75)]
        );
        assert_eq!(
            windowed_dropped_journal(&j),
            vec![(9_999, 0.75), (19_000, 0.75)]
        );
    }

    #[test]
    fn late_window_season_as_found() {
        // As found on the first run, pinned. Every prediction
        // held, and P5's hold is the headline: the window
        // silences 264 landings (first at 10000, the window's
        // own first tick) and the train parts at 10005 — a lag
        // of 5 ticks behind the first taken kick, a fourth
        // wait beside 32, 83, 130, surfaced before the second
        // silenced landing (10035) even arrived — and does
        // not heal in the short remainder: 607 disagreed
        // ticks inside the window, 333 after it (66.6 per
        // 1,000, the family's own rate), 71 at/after 23,000,
        // the last at 23992. Its cell reads the truth's own
        // headline (perfect 83, slips 12 and 17) with longest
        // 26 against the truth's 16. Follower spikes 855,
        // two short of the truth's 857, with the inside
        // counts 320 against 321 while the trains disagree
        // on 607 ticks inside. The dropped spelling: 1,045
        // entries, 3,135 bytes — absence saves 792 bytes
        // and loses the season.
        let live = run_live();
        let windowed = windowed_silenced_journal(&live.journal);
        assert_eq!(
            windowed.iter().filter(|&&(_, w)| w == 0.0).count(),
            264
        );
        let replay = run_replay(&windowed);
        assert_eq!(replay.len(), 855);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&10_005));
        assert_eq!(diffs.last(), Some(&23_992));
        assert_eq!(count_in(&diffs, 0, WINDOW_START), 0);
        assert_eq!(count_in(&diffs, WINDOW_START, WINDOW_END), 607);
        assert_eq!(count_in(&diffs, WINDOW_END, TICKS), 333);
        assert_eq!(count_in(&diffs, 20_000, TICKS), 268);
        assert_eq!(count_in(&diffs, 23_000, TICKS), 71);
        assert_eq!(diffs.len(), 940);
        let cell = count_cell(&post_settle(&live.a_spikes), &post_settle(&replay));
        assert_eq!(cell.perfect, 83);
        assert_eq!(cell.count_slips, 12);
        assert_eq!(cell.phase_slips, 17);
        assert_eq!(cell.episodes, 8);
        assert_eq!(cell.longest_episode, 26);
        let dropped = windowed_dropped_journal(&live.journal);
        assert_eq!(dropped.len(), 1045);
        assert_eq!(dropped.len() * SIGNED_ENTRY_BYTES, 3135);
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }
}
