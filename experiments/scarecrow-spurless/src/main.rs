//! scarecrow-spurless — The Scarecrow 🌾 plants Harvest's
//! journal and replay in `scarecrow-whole-kicks`' two-road
//! field, and takes the shrunk direction to its limit: road
//! B's ticks are kept, its weight is written 0.0. The spur is
//! dropped entirely. Every one of road B's 702 entries is
//! wrong by exactly -0.75 — a signed error budget of -526.5,
//! three times the shrunk journal's, all of it spent downward.
//!
//! The ladder this crate climbs down, from `scarecrow-whole-kicks`:
//! road B written +1.0 read 75/112; the truth's +0.75 read
//! 83/112; +0.5 read 91/112. This crate asks where 0.0 lands —
//! and notes what a silenced journal's replay *is*: a follower
//! receiving road A's kicks and nothing else, which is the
//! single-road baseline's own coupling, spelled as a journal.
//! If the replay machinery is honest, the silenced season
//! should not merely score near the baseline. It should *be*
//! the baseline, spike for spike.
//!
//! Journals written and replayed (the follower alone each
//! time, one neuron, no synapses — the journals are the whole
//! coupling):
//!
//! - Signed: (tick, weight) entries as they landed. Control.
//! - Whole: every weight rounded whole (+0.75 -> +1.0). The
//!   ladder's top rung, re-measured here as an anchor.
//! - Shrunk: road B written as +0.5. The ladder's third rung,
//!   re-measured here as an anchor.
//! - Silenced: road B written as 0.0 — its ticks kept, its
//!   kicks nothing. The seed's spelling.
//! - Dropped: road B's entries removed from the book
//!   entirely — 607 entries where the silenced book keeps
//!   1,309. If a 0.0 entry is truly nothing, the two books
//!   replay to the same train, and the only difference left
//!   between them is paper.
//!
//! The counting instrument is the Widow's own, copied
//! faithfully from `scarecrow-whole-kicks` (super-cycles of
//! five driver-A cycles, seven follower beats, phase
//! tolerance ±0.06), and a baseline run of the single-road
//! pair pins this crate to the sheaf's published centre.

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
/// Signed journal record: u16 tick + i8 weight in quarter
/// units. A silenced entry is counted at the same 3 bytes:
/// writing 0.0 still takes a line. A dropped entry takes
/// no line at all — that is the whole of its saving.
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
/// Returns the cell and the follower's full spike train, so
/// a replay can be compared against the live animal spike
/// for spike, not only through the instrument's summary.
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

/// The whole journal: every weight rounded to the nearest
/// whole kick. -1.0 stays -1.0; +0.75 becomes +1.0.
fn whole_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal.iter().map(|&(t, w)| (t, w.round())).collect()
}

/// The shrunk journal: positive kicks remembered small —
/// +0.75 written as +0.5; negative kicks were already whole.
fn shrunk_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| {
            if w > 0.0 {
                (t, (w * 2.0).trunc() / 2.0)
            } else {
                (t, w)
            }
        })
        .collect()
}

/// The silenced journal: the seed's spelling. Road B's
/// ticks are kept; its weight is written 0.0. Road A's
/// entries are untouched. The book keeps every line it
/// ever had — 702 of them now say nothing.
fn silenced_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal
        .iter()
        .map(|&(t, w)| if w > 0.0 { (t, 0.0) } else { (t, w) })
        .collect()
}

/// The dropped journal: road B's entries removed outright.
/// What remains is road A's book alone — which is also,
/// entry for entry, the journal the single-road baseline
/// would have written.
fn dropped_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal.iter().copied().filter(|&(_, w)| w <= 0.0).collect()
}

/// Signed error budget of a spelling against the truth:
/// the sum over entries of (written weight - true weight),
/// matched in landing order (same length, same ticks).
/// Only meaningful for spellings that keep every entry.
fn signed_error(truth: &[(usize, f32)], spelling: &[(usize, f32)]) -> f32 {
    truth
        .iter()
        .zip(spelling.iter())
        .map(|(&(_, tw), &(_, sw))| sw - tw)
        .sum()
}

fn headless() {
    println!("scarecrow-spurless headless ledger");
    println!(
        "host: scarecrow-whole-kicks' follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS
    );
    println!("principle: journal and replay (Harvest) — the journal's ticks are exact; road B's weight is written 0.0, the limiting shrink. The follower is replayed alone from each spelling, never opened");

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

    // Whole: the ladder's top rung, the anchor.
    let whole = whole_journal(&live.journal);
    let replay_whole = run_replay(&whole);
    let div_whole = first_divergence(&live.follower_spikes, &replay_whole);
    let cell_whole = cell_of(&replay_whole);
    fmt_cell("replay-whole", &cell_whole);
    println!(
        "replay-whole: follower spikes {}, first divergence {:?}, signed error budget {:+.1} (road B written +1.0)",
        replay_whole.len(), div_whole, signed_error(&live.journal, &whole)
    );

    // Shrunk: the ladder's third rung, the anchor.
    let shrunk = shrunk_journal(&live.journal);
    let replay_shrunk = run_replay(&shrunk);
    let div_shrunk = first_divergence(&live.follower_spikes, &replay_shrunk);
    let cell_shrunk = cell_of(&replay_shrunk);
    fmt_cell("replay-shrunk", &cell_shrunk);
    println!(
        "replay-shrunk: follower spikes {}, first divergence {:?}, signed error budget {:+.1} (road B written +0.5)",
        replay_shrunk.len(), div_shrunk, signed_error(&live.journal, &shrunk)
    );

    // Silenced: the seed's spelling. Ticks kept, weight 0.0.
    let silenced = silenced_journal(&live.journal);
    let err_silenced = signed_error(&live.journal, &silenced);
    let replay_silenced = run_replay(&silenced);
    let div_silenced = first_divergence(&live.follower_spikes, &replay_silenced);
    let cell_silenced = cell_of(&replay_silenced);
    fmt_cell("replay-silenced", &cell_silenced);
    println!(
        "replay-silenced: {} entries ({} of them written 0.0), follower spikes {}, first divergence {:?}, signed error budget {:+.1} over {} road-B entries (-0.75 each)",
        silenced.len(),
        silenced.iter().filter(|&&(_, w)| w == 0.0).count(),
        replay_silenced.len(), div_silenced, err_silenced, live.b_landings.len()
    );
    println!(
        "replay-silenced vs baseline: follower trains identical {} (first divergence {:?}); cells identical {} — a silenced journal, replayed, is the single-road animal itself, not an approximation of it",
        replay_silenced == base_train,
        first_divergence(&base_train, &replay_silenced),
        cell_silenced == base
    );

    // Dropped: road B's entries removed from the book.
    let dropped = dropped_journal(&live.journal);
    let dropped_bytes = dropped.len() * SIGNED_ENTRY_BYTES;
    let replay_dropped = run_replay(&dropped);
    let div_dropped = first_divergence(&live.follower_spikes, &replay_dropped);
    let cell_dropped = cell_of(&replay_dropped);
    fmt_cell("replay-dropped", &cell_dropped);
    println!(
        "replay-dropped: {} entries, follower spikes {}, first divergence {:?}; paper {} entries x {} B = {} bytes against the silenced book's {} bytes — dropping the lines is the only saving in this row, and it buys {} bytes",
        dropped.len(), replay_dropped.len(), div_dropped,
        dropped.len(), SIGNED_ENTRY_BYTES, dropped_bytes, signed_bytes,
        signed_bytes - dropped_bytes
    );
    println!(
        "replay-silenced vs replay-dropped: follower trains identical {} — a 0.0 entry is no entry, as far as the ear is concerned; the two books differ only in paper",
        replay_silenced == replay_dropped
    );

    println!(
        "the ladder on the perfect column: whole (+1.0) {} < truth (+0.75) {} < shrunk (+0.5) {} < silenced (0.0) {} — the true weight is not the best weight on the truth's own instrument",
        cell_whole.perfect, live.ledger.perfect, cell_shrunk.perfect, cell_silenced.perfect
    );

    let b1 = base.perfect == 92
        && base.super_cycles == 112
        && base.count_slips == 3
        && base.phase_slips == 17
        && base.longest_episode == 58
        && (base.mean_episode - 18.4).abs() < 0.05
        && live.ledger.perfect == 83
        && live.follower_spikes.len() == 857
        && live.journal.len() == 1309
        && coinc.len() == 24;
    let b2 = div_whole == Some(64)
        && cell_whole.perfect == 75
        && div_shrunk == Some(91)
        && cell_shrunk.perfect == 91
        && replay_shrunk.len() == 855;
    let p1 = div_signed.is_none();
    let p2 = match (div_silenced, live.b_landings.first()) {
        (Some(d), Some(&first_b)) => d >= first_b,
        _ => false,
    };
    let p3 = replay_silenced == replay_dropped;
    let p4 = replay_silenced == base_train && cell_silenced == base;
    let p5 = cell_whole.perfect < live.ledger.perfect
        && live.ledger.perfect < cell_shrunk.perfect
        && cell_shrunk.perfect < cell_silenced.perfect;
    let p6 = cell_silenced.perfect > live.ledger.perfect
        && cell_silenced.longest_episode > live.ledger.longest_episode
        && cell_silenced.count_slips < live.ledger.count_slips
        && cell_silenced.phase_slips <= live.ledger.phase_slips;
    let silenced_bytes = silenced.len() * SIGNED_ENTRY_BYTES;
    let p7 = silenced_bytes == signed_bytes && dropped_bytes < signed_bytes;
    let p8 = match div_silenced {
        Some(d) => d <= 91,
        None => false,
    };
    println!(
        "verdicts: B1 hosts pin exact (sheaf centre + two-roads live ledger): {}; B2 ladder anchors reproduce whole-kicks exact (whole parts 64 reads 75, shrunk parts 91 reads 91): {}; P1 signed journal replays the follower exact: {}; P2 silenced journal diverges at or after the first road-B landing: {}; P3 silenced and dropped replay to the same train (a 0.0 entry is no entry): {}; P4 silenced season IS the baseline (train and cell identical to the live single-road pair): {}; P5 the ladder is monotone on perfect (whole < truth < shrunk < silenced): {}; P6 silenced beats or matches the truth on every headline column, strictly on perfect, longest and count slips: {}; P7 writing nothing costs full paper (silenced {} B == signed {} B) and only dropping lines saves ({} B): {}; P8 silenced parts no later than the shrunk journal's tender tick 91: {}",
        b1, b2, p1, p2, p3, p4, p5, p6,
        silenced_bytes, signed_bytes, dropped_bytes, p7, p8
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-spurless: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
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
    fn silencing_zeroes_only_road_b() {
        let j = vec![(5, -1.0), (9, 0.75), (12, -1.0), (20, 0.75)];
        assert_eq!(
            silenced_journal(&j),
            vec![(5, -1.0), (9, 0.0), (12, -1.0), (20, 0.0)]
        );
        assert!((signed_error(&j, &silenced_journal(&j)) + 1.5).abs() < 1e-6);
    }

    #[test]
    fn dropping_removes_only_road_b() {
        let j = vec![(5, -1.0), (9, 0.75), (12, -1.0), (20, 0.75)];
        assert_eq!(dropped_journal(&j), vec![(5, -1.0), (12, -1.0)]);
    }

    #[test]
    fn silenced_season_is_the_baseline_animal() {
        // As found on the first run, pinned. P4 HELD in its
        // strongest form: the silenced replay's follower
        // train is the live single-road pair's train, spike
        // for spike over all 24,000 ticks (850 spikes), and
        // its cell is the baseline cell field for field. The
        // silenced book parts from the two-road truth at
        // tick 65 — one tick after the family's tender tick
        // 64, where the whole and unsigned spellings part.
        let (base, base_train) = run_single();
        let live = run_live();
        let silenced = silenced_journal(&live.journal);
        assert_eq!(silenced.len(), 1309);
        assert_eq!(silenced.iter().filter(|&&(_, w)| w == 0.0).count(), 702);
        assert!((signed_error(&live.journal, &silenced) + 526.5).abs() < 0.01);
        let replay = run_replay(&silenced);
        assert_eq!(replay, base_train);
        assert_eq!(replay.len(), 850);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), Some(65));
        let cell = count_cell(&post_settle(&live.a_spikes), &post_settle(&replay));
        assert_eq!(cell, base);
        assert_eq!(cell.perfect, 92);
        assert_eq!(cell.longest_episode, 58);
    }

    #[test]
    fn dropped_book_same_train_less_paper() {
        // As found: dropping road B's lines outright leaves
        // 607 entries and replays to the silenced train
        // exactly — a 0.0 entry is no entry to the ear. The
        // books differ only in paper: 1,821 bytes against
        // the silenced book's 3,927.
        let live = run_live();
        let dropped = dropped_journal(&live.journal);
        assert_eq!(dropped.len(), 607);
        assert_eq!(dropped.len() * SIGNED_ENTRY_BYTES, 1821);
        assert_eq!(
            run_replay(&dropped),
            run_replay(&silenced_journal(&live.journal))
        );
    }

    #[test]
    fn ladder_anchors_and_monotone_ordering() {
        // As found: the anchors reproduce whole-kicks exact
        // (B2), and the perfect column climbs as the
        // remembered spur shrinks (P5): whole 75 < truth 83
        // < shrunk 91 < silenced 92. The true weight is not
        // the best weight on the truth's own columns (P6).
        let live = run_live();
        let cell_of = |train: &[usize]| {
            count_cell(&post_settle(&live.a_spikes), &post_settle(train))
        };
        let whole = run_replay(&whole_journal(&live.journal));
        assert_eq!(first_divergence(&live.follower_spikes, &whole), Some(64));
        assert_eq!(cell_of(&whole).perfect, 75);
        let shrunk = run_replay(&shrunk_journal(&live.journal));
        assert_eq!(shrunk.len(), 855);
        assert_eq!(first_divergence(&live.follower_spikes, &shrunk), Some(91));
        assert_eq!(cell_of(&shrunk).perfect, 91);
        let silenced = run_replay(&silenced_journal(&live.journal));
        let cell_silenced = cell_of(&silenced);
        assert!(cell_of(&whole).perfect < live.ledger.perfect);
        assert!(live.ledger.perfect < cell_of(&shrunk).perfect);
        assert!(cell_of(&shrunk).perfect < cell_silenced.perfect);
        assert!(cell_silenced.longest_episode > live.ledger.longest_episode);
        assert!(cell_silenced.count_slips < live.ledger.count_slips);
        assert!(cell_silenced.phase_slips <= live.ledger.phase_slips);
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }
}
