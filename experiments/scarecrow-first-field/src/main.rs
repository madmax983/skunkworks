//! scarecrow-first-field — The Scarecrow 🌾 plants
//! Harvest's journal and replay in the two-road field
//! of `scarecrow-first-window`, and silences road B
//! inside the first hundred ticks of the season:
//! [0, 100), the shortest length in the row, the
//! smallest window that still covers the tender
//! tick at 65.
//!
//! Eleven windows stand in this row already: 9,000
//! ticks (`scarecrow-windowed-silence`), 1,000
//! (`scarecrow-short-window`), 100 at a running
//! address (`scarecrow-hundred-window`), 10
//! (`scarecrow-ten-window`), 1,000 after the tender
//! tick (`scarecrow-after-window`), 9,000 late
//! (`scarecrow-late-window`), one kick at the gate
//! (`scarecrow-one-kick`, [10,000, 10,010)), the
//! second kick alone (`scarecrow-second-kick`,
//! [10,035, 10,045)), the last window
//! (`scarecrow-last-window`, [19,000, 24,000)), the
//! first window (`scarecrow-first-window`,
//! [0, 5,000)), and the first thousand
//! (`scarecrow-first-thousand`, [0, 1,000)). Every
//! one of them parted the train and none of them
//! healed: the partings stood to ticks
//! 23,992–23,999, and the row's law is *lose
//! nothing*. First-thousand's law, twenty-fifth in
//! my notes: standing belongs to the address, not
//! the dose — a fifth of the first window's dose
//! (30 landings) left a larger wreck (1,674
//! disagreed ticks) than the whole dose did
//! (1,594), and the animal born at one window's
//! edge is not the animal born at another's.
//!
//! The garden's open seed asked for this planting
//! verbatim: *the first field at the shortest
//! length — silence road B on [0, 100) only, the
//! smallest window that still covers the tender
//! tick at 65: does the parting still open at 65
//! and stand on the handful of landings that
//! window holds, or is there a dose below which
//! the birth buys nothing — and if the parting
//! does not open, is the tender tick itself
//! outside the window's keeping?*
//!
//! The survey counted the dose before planting
//! (Phase 1, in the open — landings only, no replay,
//! no parting measured): road B lands exactly
//! 4 times inside [0, 100), the first at
//! tick 10 and the last at 84 — a handful
//! against the first window's 147 and the first
//! thousand's 30 — and the rest of its 702
//! landings at/after 100, in the remainder. The
//! books of this field, of the first thousand, of
//! the first window, and of the full-season
//! silence are one book for the season's first
//! hundred ticks, so the parting — if the book
//! names it — opens at 65 in all of them. What has
//! never been priced is a standing bought this
//! cheaply: if there is a dose below which the
//! birth buys nothing, this field is standing in
//! it; if standing belongs to the address alone,
//! a handful of landings stands as 147 did.
//!
//! A parting is measured three ways: the first tick
//! it opens, the last tick it is open, and the count
//! of disagreed ticks in each region of the season
//! (inside the window [0, 100), after it
//! [100, 24,000) — there is no before it — and in
//! the tails at and after 20,000, 22,000, and
//! 23,000). This crate also replays first-window's
//! spelling ([0, 5,000)) as an anchor, and it
//! compares its own animal against first-window's:
//! the two trains are one train inside [0, 100);
//! whether they are one animal after 5,000 is
//! counted, not assumed.
//!
//! Journals written and replayed (the follower alone
//! each time, one neuron, no synapses — the journals
//! are the whole coupling):
//!
//! - Signed: (tick, weight) entries as they landed.
//!   Control.
//! - Silenced (full season): road B written 0.0
//!   everywhere. The spurless anchor, re-measured
//!   here: it should be the baseline animal entire,
//!   and part from the truth at tick 65.
//! - First-field-silenced: road B written 0.0 only
//!   on ticks in [0, 100). The seed's spelling.
//!   Every line kept; every line inside the window
//!   says nothing, and the window opens on the
//!   season's first tick and closes a hundred ticks
//!   later — before the instrument's settle ends.
//! - First-field-dropped: road B's entries inside
//!   the window removed from the book outright. If
//!   a 0.0 entry is no entry — spurless' law — this
//!   book replays to the first-field-silenced train
//!   exact, and the two differ only in paper.
//! - First-window-silenced: road B written 0.0 only
//!   on ticks in [0, 5,000). First-window's own
//!   spelling, re-measured here as an anchor: 147
//!   entries silenced, parting at 65, standing to
//!   23,997, 1,594 disagreed ticks.
//!
//! The counting instrument is the Widow's own,
//! copied faithfully from `scarecrow-first-window`
//! (super-cycles of five driver-A cycles, seven
//! follower beats, phase tolerance ±0.06), and a
//! baseline run of the single-road pair pins this
//! crate to the sheaf's published centre.

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
/// the first hundred ticks of the season, opening on
/// its first tick. There is no before: the follower
/// enters this window carrying no history at all,
/// and 23,900 exact ticks follow it — the longest
/// remainder in the row, bought with the smallest
/// dose: a handful of landings, and the window
/// closes before the instrument's settle ends.
const WINDOW_START: usize = 0;
const WINDOW_END: usize = 100;
/// First-window's window, kept as this crate's
/// anchor: road B silenced on [0, 5,000).
const FIRST_WINDOW_START: usize = 0;
const FIRST_WINDOW_END: usize = 5_000;
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

/// The first-field silenced journal: the seed's spelling.
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

/// The first-field-dropped journal: road B's entries
/// inside the window removed outright; every other
/// entry exact.
fn windowed_dropped_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .copied()
        .filter(|&(t, w)| !(w > 0.0 && t >= WINDOW_START && t < WINDOW_END))
        .collect()
}

/// First-window's own spelling, re-measured as an
/// anchor: road B written 0.0 only on ticks in
/// [0, 5,000).
fn first_silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| {
            if w > 0.0 && t >= FIRST_WINDOW_START && t < FIRST_WINDOW_END {
                (t, 0.0)
            } else {
                (t, w)
            }
        })
        .collect()
}

fn survey() {
    // Phase 1, in the open: count the host's landings only.
    // No replay is run, no parting is measured. This is the
    // survey the README's declarations are priced from.
    let live = run_live();
    let inside: Vec<usize> = live
        .b_landings
        .iter()
        .copied()
        .filter(|&t| t >= WINDOW_START && t < WINDOW_END)
        .collect();
    println!("survey: road B landings total {}", live.b_landings.len());
    println!(
        "survey: road B landings before {} {}",
        WINDOW_START,
        count_in(&live.b_landings, 0, WINDOW_START)
    );
    println!(
        "survey: road B landings inside [{}, {}) {}",
        WINDOW_START,
        WINDOW_END,
        inside.len()
    );
    println!("survey: first landing inside {:?}", inside.first());
    println!("survey: last landing inside {:?}", inside.last());
    println!(
        "survey: road B landings at/after {} {}",
        WINDOW_END,
        count_in(&live.b_landings, WINDOW_END, TICKS)
    );
    println!(
        "survey: road A landings {}, signed entries {}, follower spikes {}",
        live.a_landings.len(),
        live.journal.len(),
        live.follower_spikes.len()
    );
}

fn headless() {
    println!("scarecrow-first-field headless ledger");
    println!(
        "host: scarecrow-first-window's follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}; silence window [{}, {}) — the season's first hundred ticks, opening on its first tick; 23,900 exact ticks are its remainder",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS,
        WINDOW_START, WINDOW_END
    );
    println!("principle: journal and replay (Harvest) — the journal is exact everywhere except the season's first hundred ticks; inside that window every road B landing is written 0.0, and the exact journal then runs for 23,900 ticks. The follower is replayed alone from each spelling, never opened. The first window silenced 147 landings at this address and the first thousand silenced 30, and both partings stood; this field silences a handful — and the questions are the seed's own: does the parting still open at 65 and stand on the handful of landings this window holds, or is there a dose below which the birth buys nothing?");

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
    let last_b_in_window = live
        .b_landings
        .iter()
        .copied()
        .filter(|&t| t >= WINDOW_START && t < WINDOW_END)
        .last();
    println!(
        "window: road B landings before {} {} (there is no before — the window opens on the season's first tick), inside [{}, {}) {}, at/after {} {}; first inside {:?}, last inside {:?}",
        WINDOW_START,
        count_in(&live.b_landings, 0, WINDOW_START),
        WINDOW_START,
        WINDOW_END,
        b_in_window,
        WINDOW_END,
        count_in(&live.b_landings, WINDOW_END, TICKS),
        first_b_in_window,
        last_b_in_window
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

    // First-silenced: first-window's own spelling, the
    // anchor — 147 kicks, parting at 65, standing to
    // 23997, 1594 disagreed ticks.
    let firstw = first_silenced_journal(&live.journal);
    let replay_firstw = run_replay(&firstw);
    let cell_firstw = cell_of(&replay_firstw);
    fmt_cell("replay-firstwindow-silenced", &cell_firstw);
    let diffs_firstw = symdiff_ticks(&live.follower_spikes, &replay_firstw);
    println!(
        "replay-firstwindow-silenced: {} entries ({} written 0.0 inside [{}, {})), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, total disagreed {} — first-window's ledger, re-measured in this crate",
        firstw.len(),
        firstw.iter().filter(|&&(_, w)| w == 0.0).count(),
        FIRST_WINDOW_START, FIRST_WINDOW_END,
        replay_firstw.len(), live.follower_spikes.len(),
        diffs_firstw.first(), diffs_firstw.last(), diffs_firstw.len()
    );

    // First-window-silenced: the seed's spelling.
    let windowed = windowed_silenced_journal(&live.journal);
    let replay_windowed = run_replay(&windowed);
    let cell_windowed = cell_of(&replay_windowed);
    fmt_cell("replay-firstfield-silenced", &cell_windowed);
    let diffs = symdiff_ticks(&live.follower_spikes, &replay_windowed);
    let first_div = diffs.first().copied();
    let last_div = diffs.last().copied();
    let d_inside = count_in(&diffs, WINDOW_START, WINDOW_END);
    let d_after = count_in(&diffs, WINDOW_END, TICKS);
    let d_after20 = count_in(&diffs, 20_000, TICKS);
    let d_after22 = count_in(&diffs, 22_000, TICKS);
    let d_after23 = count_in(&diffs, 23_000, TICKS);
    // The rejoin tick, as this row's instrument defines it:
    // the first tick at or after the window's end from which
    // the two trains agree to the season's end. 23,900
    // exact ticks follow this window; if the parting
    // closes anywhere, it closes in them, and this
    // reading is where the closing would stand.
    let rejoin_tick = if diffs.is_empty() {
        WINDOW_END
    } else {
        match diffs.iter().filter(|&&t| t >= WINDOW_END).last() {
            None => WINDOW_END,
            Some(&l) => l + 1,
        }
    };
    println!(
        "replay-firstfield-silenced: {} entries ({} written 0.0 inside the window), follower spikes {} (live {}), first divergence {:?}, last divergence {:?}, rejoin tick {} (24000 = no rejoin in 23,900 exact ticks)",
        windowed.len(),
        b_in_window,
        replay_windowed.len(),
        live.follower_spikes.len(),
        first_div, last_div, rejoin_tick
    );
    println!(
        "first-field parting by region (disagreed ticks): before the window {} (there is none — the window opens at tick 0), inside [{}, {}) {}, after [{}, {}) {}, at/after 20000 {}, at/after 22000 {}, at/after 23000 {}; total disagreed {}",
        0,
        WINDOW_START, WINDOW_END, d_inside,
        WINDOW_END, TICKS, d_after,
        d_after20, d_after22, d_after23, diffs.len()
    );
    println!(
        "follower spikes by region: live inside {} after {}; first-field inside {} after {}; baseline inside {} after {}",
        count_in(&live.follower_spikes, WINDOW_START, WINDOW_END),
        count_in(&live.follower_spikes, WINDOW_END, TICKS),
        count_in(&replay_windowed, WINDOW_START, WINDOW_END),
        count_in(&replay_windowed, WINDOW_END, TICKS),
        count_in(&base_train, WINDOW_START, WINDOW_END),
        count_in(&base_train, WINDOW_END, TICKS)
    );
    // Inside the window the first-field animal IS the
    // baseline animal: both followers start from the same
    // seed state and hear only road A until tick 100.
    // After the window the exact journal resumes into a
    // state it did not grow, and a third animal is born.
    // Count both facts.
    let diffs_base = symdiff_ticks(&base_train, &replay_windowed);
    println!(
        "first-field vs baseline: disagreed ticks inside the window {} in [{}, {}), after the window {} in [{}, {}) — inside, the silenced animal is the baseline animal entire; the third animal is born at the window's far edge, not inside it",
        count_in(&diffs_base, WINDOW_START, WINDOW_END), WINDOW_START, WINDOW_END,
        count_in(&diffs_base, WINDOW_END, TICKS), WINDOW_END, TICKS
    );

    // Counted, not assumed: is the animal born at this
    // field's edge (100) the same animal first-window's
    // was born at its edge (5000)? The two trains are
    // one book inside [0, 100); between the edges they
    // hear different books; after 5000 both hear the
    // exact journal — if they still disagree there,
    // each edge bore its own animal.
    let diffs_fw = symdiff_ticks(&replay_firstw, &replay_windowed);
    println!(
        "first-field vs first-window: disagreed ticks inside [{}, {}) {}, between the edges [{}, {}) {}, at/after {} {} — one train where the books are one book; after both edges, the count says whether one animal or two were born",
        WINDOW_START, WINDOW_END, count_in(&diffs_fw, WINDOW_START, WINDOW_END),
        WINDOW_END, FIRST_WINDOW_END, count_in(&diffs_fw, WINDOW_END, FIRST_WINDOW_END),
        FIRST_WINDOW_END, count_in(&diffs_fw, FIRST_WINDOW_END, TICKS)
    );

    // First-window-dropped: the same silence, written as absence.
    let dropped = windowed_dropped_journal(&live.journal);
    let dropped_bytes = dropped.len() * SIGNED_ENTRY_BYTES;
    let replay_dropped = run_replay(&dropped);
    fmt_cell("replay-firstfield-dropped", &cell_of(&replay_dropped));
    println!(
        "replay-firstfield-dropped: {} entries, follower spikes {}, identical to first-field-silenced train {}; paper {} B against the first-field-silenced book's {} B — absence inside the window saves {} B, silence saves none",
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
    // B4 (anchor): first-window's spelling, re-measured in
    // this crate, reproduces first-window's ledger entire —
    // 147 silenced, parting at 65, last disagreed 23997,
    // 1594 disagreed ticks.
    let b4 = diffs_firstw.first() == Some(&65)
        && diffs_firstw.last() == Some(&23_997)
        && diffs_firstw.len() == 1594
        && firstw.iter().filter(|&&(_, w)| w == 0.0).count() == 147;
    let p1 = div_signed.is_none();
    // P2 (the birth): inside the window the first-field
    // train is the baseline train exact — both followers
    // start from the seed state and hear only road A
    // until the window closes.
    let p2 = count_in(&diffs_base, WINDOW_START, WINDOW_END) == 0;
    // P3 (causality): no disagreed tick stands before the
    // first silenced landing inside the window — the
    // spelling touches nothing upstream of it, and at
    // this address there is almost nothing upstream at
    // all.
    let first_in = first_b_in_window.unwrap_or(WINDOW_START);
    let p3 = count_in(&diffs, 0, first_in) == 0;
    // P4 (the seed's ignition): the parting opens where
    // the full-season silence opened — at tick 65 —
    // because for the season's first 100 ticks this
    // field's book, the first window's book, and the
    // full-season book are the same book, and 65 stands
    // inside the window.
    let p4 = first_div == Some(65);
    // P5 (the seed's standing): the parting stands
    // through 23,900 exact ticks on a handful of
    // landings — disagreed ticks stand at/after 23,000
    // and the last disagreed tick stands at/after
    // 23,900, where every earlier window's parting
    // stood (23,992–23,999). If there is a dose below
    // which the birth buys nothing, this cracks.
    let p5 = d_after23 > 0 && last_div.map(|l| l >= 23_900).unwrap_or(false);
    let p6 = replay_dropped == replay_windowed;
    let windowed_bytes = windowed.len() * SIGNED_ENTRY_BYTES;
    let p7 = windowed_bytes == signed_bytes
        && signed_bytes - dropped_bytes == 3 * b_in_window;
    // P8 (the wreck's size, priced over the longest
    // standing): the wreck is the family's wreck at the
    // family's rate — the family disagrees on ~6.7–7.0%
    // of the ticks a parting stands in (1594 in 23,932
    // for the first window; 1674 in 23,927 for the
    // first thousand; 940 in 13,887; 339 in 4,880), and
    // this parting stands for ~23,927 ticks (65 to
    // ~23,992), pricing the total in 1400..=1850 — and
    // the first-field cell's perfect count in 60..=90
    // (the burned seasons read 73–86; this season is
    // parted from tick 65, before the settle even ends,
    // on a handful of landings).
    let p8 = (1400..=1850).contains(&diffs.len())
        && (60..=90).contains(&cell_windowed.perfect);
    // P9 (the counts, at the birth): the tallies stay
    // close even where the train does not — follower
    // spikes total within 8 of the truth's 857, and
    // inside the window the count is the baseline's
    // count exact, where the train itself is exact.
    let p9 = (replay_windowed.len() as i64 - live.follower_spikes.len() as i64).abs() <= 8
        && count_in(&replay_windowed, WINDOW_START, WINDOW_END)
            == count_in(&base_train, WINDOW_START, WINDOW_END);
    println!(
        "verdicts: B1 single-road baseline reproduces the sheaf centre exact: {}; B2 live two-road season reproduces two-roads/spurless exact: {}; B3 full-season silenced anchor is the baseline animal and parts at 65: {}; B4 first-window anchor reproduces first-window's ledger entire (147 silenced, parting 65, last 23997, 1594 disagreed): {}; P1 signed journal replays the follower exact: {}; P2 inside the window the first-field train is the baseline train exact (disagreed vs baseline inside = {}): {}; P3 causality — no disagreed tick before the first silenced landing at {}: {}; P4 the parting opens at 65, the full-season silence's own address (first disagreed tick {:?}): {}; P5 the parting stands through 23,900 exact ticks on a handful of landings (disagreed at/after 23000 = {}, last disagreed {:?} >= 23900; rejoin tick {}): {}; P6 first-field-dropped replays to the first-field-silenced train exact (a 0.0 entry is no entry, in first-field form): {}; P7 silence in a window costs full paper ({} B == signed {} B) and absence saves exactly 3 B per silenced landing ({} landings, saving {} B): {}; P8 the wreck is the family's rate over the longest standing (total disagreed {} in 1400..=1850; perfect {} in 60..=90; verdict {}): {}; P9 the counts stay close at the birth (first-field spikes {} against live {}; inside-window count {} == baseline {}): {}",
        b1, b2, b3, b4, p1, count_in(&diffs_base, WINDOW_START, WINDOW_END), p2, first_in, p3,
        first_div, p4, d_after23, last_div, rejoin_tick, p5, p6,
        windowed_bytes, signed_bytes, b_in_window, signed_bytes - dropped_bytes, p7,
        diffs.len(), cell_windowed.perfect, cell_windowed.verdict, p8,
        replay_windowed.len(), live.follower_spikes.len(),
        count_in(&replay_windowed, WINDOW_START, WINDOW_END), count_in(&base_train, WINDOW_START, WINDOW_END), p9
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--survey") {
        survey();
        return Ok(());
    }
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-first-field: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
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
            (0, 0.75),
            (5, -1.0),
            (99, 0.75),
            (100, 0.75),
            (20_000, -1.0),
        ];
        assert_eq!(
            windowed_silenced_journal(&j),
            vec![
                (0, 0.0),
                (5, -1.0),
                (99, 0.0),
                (100, 0.75),
                (20_000, -1.0),
            ]
        );
        assert_eq!(
            windowed_dropped_journal(&j),
            vec![(5, -1.0), (100, 0.75), (20_000, -1.0)]
        );
        // The first-window anchor's spelling touches only
        // road B inside [0, 5000).
        assert_eq!(
            first_silenced_journal(&j),
            vec![
                (0, 0.0),
                (5, -1.0),
                (99, 0.0),
                (100, 0.0),
                (20_000, -1.0),
            ]
        );
    }

    #[test]
    fn symdiff_and_region_counts_on_planted_trains() {
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 9, 20]), Vec::<usize>::new());
        assert_eq!(symdiff_ticks(&[3, 9, 20], &[3, 10, 20]), vec![9, 10]);
        assert_eq!(symdiff_ticks(&[3, 9], &[3, 9, 20]), vec![20]);
        assert_eq!(count_in(&[0, 99, 100, 23_999], 0, 100), 2);
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
    fn inside_the_window_the_silenced_animal_is_the_baseline() {
        // The birth: both followers start from the seed
        // state and hear only road A until tick 100, so
        // inside the window the two trains cannot differ.
        let (_, base_train) = run_single();
        let live = run_live();
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&base_train, &replay);
        assert_eq!(count_in(&diffs, WINDOW_START, WINDOW_END), 0);
    }

    #[test]
    fn nothing_parts_before_the_first_silenced_landing() {
        // Causality, this window's own fence: the first
        // entry this spelling touches is the first road-B
        // landing of the season. Every tick before it is
        // replayed from an exact journal in an exact
        // state, so no disagreed tick can stand before it.
        let live = run_live();
        let first_in = live
            .b_landings
            .iter()
            .copied()
            .find(|&t| t >= WINDOW_START)
            .expect("road B lands inside the first field");
        let replay = run_replay(&windowed_silenced_journal(&live.journal));
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(count_in(&diffs, 0, first_in), 0);
    }

    #[test]
    fn first_window_anchor_reproduces_first_windows_ledger() {
        // The parent, re-measured in this crate:
        // first-window's spelling silences 147 landings,
        // parts at 65, stands to 23997, and disagrees
        // on 1594 ticks.
        let live = run_live();
        let firstw = first_silenced_journal(&live.journal);
        assert_eq!(firstw.iter().filter(|&&(_, w)| w == 0.0).count(), 147);
        let replay = run_replay(&firstw);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&65));
        assert_eq!(diffs.last(), Some(&23_997));
        assert_eq!(diffs.len(), 1594);
    }

    #[test]
    fn the_first_field_silences_n_landings_first_at_10() {
        // The survey's count, asserted in the flesh: road B
        // lands exactly 4 times inside [0, 100),
        // the first at tick 10 and the last at 84 —
        // and the rest of its landings in the remainder
        // at/after 100.
        let live = run_live();
        let inside: Vec<usize> = live
            .b_landings
            .iter()
            .copied()
            .filter(|&t| t >= WINDOW_START && t < WINDOW_END)
            .collect();
        assert_eq!(inside.len(), 4);
        assert_eq!(inside.first(), Some(&10));
        assert_eq!(inside.last(), Some(&84));
        assert_eq!(count_in(&live.b_landings, WINDOW_END, TICKS), 702 - 4);
    }

    #[test]
    fn first_field_season_as_found() {
        // As found on the first run, pinned. Every
        // prediction held: the window silences exactly
        // 4 landings (first at 10, last at 84) — a
        // handful against the first window's 147 and
        // the first thousand's 30 — and the train parts
        // at 65, the full-season silence's own address,
        // a wait of 55 behind the first silenced
        // landing, and stands through 23,900 exact
        // ticks to 23998, one tick later than the
        // first window's own last tick: 1600 disagreed
        // ticks in all, only 2 inside the window and
        // 1598 after it, 258 at/after 20,000 and 66
        // at/after 23,000, rejoin tick 23999. There is
        // no dose below which the birth buys nothing
        // at this address: four landings bought the
        // family's whole wreck (1600 against the first
        // window's 1594 on 147 landings). Inside the
        // window the silenced train is the baseline
        // train exact (0 disagreed); after the window
        // it disagrees with the baseline on 1601
        // ticks — the third animal is born at the
        // edge. Against first-window's train it agrees
        // exact inside [0, 100) (0), disagrees on 336
        // ticks between the edges, and on 1262
        // at/after 5000 — the animal born at 100 is
        // not the animal born at 5000. Its cell reads
        // perfect 83, slips 8 and 21, episodes 13,
        // longest 16 — a burned season by the
        // instrument's count. Follower spikes 857, the
        // truth's count exact, by region too (inside
        // 4 = 4, after 853 = 853). The dropped
        // spelling: 1,305 entries, 3,915 bytes —
        // absence saves 12 bytes and loses the
        // season's first field.
        let live = run_live();
        let windowed = windowed_silenced_journal(&live.journal);
        assert_eq!(
            windowed.iter().filter(|&&(_, w)| w == 0.0).count(),
            4
        );
        let replay = run_replay(&windowed);
        assert_eq!(replay.len(), 857);
        let diffs = symdiff_ticks(&live.follower_spikes, &replay);
        assert_eq!(diffs.first(), Some(&65));
        assert_eq!(diffs.last(), Some(&23_998));
        assert_eq!(count_in(&diffs, WINDOW_START, WINDOW_END), 2);
        assert_eq!(count_in(&diffs, WINDOW_END, TICKS), 1598);
        assert_eq!(count_in(&diffs, 20_000, TICKS), 258);
        assert_eq!(count_in(&diffs, 23_000, TICKS), 66);
        assert_eq!(diffs.len(), 1600);
        let (_, base_train) = run_single();
        let diffs_base = symdiff_ticks(&base_train, &replay);
        assert_eq!(count_in(&diffs_base, WINDOW_START, WINDOW_END), 0);
        assert_eq!(count_in(&diffs_base, WINDOW_END, TICKS), 1601);
        let replay_firstw = run_replay(&first_silenced_journal(&live.journal));
        let diffs_fw = symdiff_ticks(&replay_firstw, &replay);
        assert_eq!(count_in(&diffs_fw, WINDOW_START, WINDOW_END), 0);
        assert_eq!(count_in(&diffs_fw, WINDOW_END, FIRST_WINDOW_END), 336);
        assert_eq!(count_in(&diffs_fw, FIRST_WINDOW_END, TICKS), 1262);
        let cell = count_cell(&post_settle(&live.a_spikes), &post_settle(&replay));
        assert_eq!(cell.perfect, 83);
        assert_eq!(cell.count_slips, 8);
        assert_eq!(cell.phase_slips, 21);
        assert_eq!(cell.episodes, 13);
        assert_eq!(cell.longest_episode, 16);
        let dropped = windowed_dropped_journal(&live.journal);
        assert_eq!(dropped.len(), 1305);
        assert_eq!(dropped.len() * SIGNED_ENTRY_BYTES, 3915);
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }
}
