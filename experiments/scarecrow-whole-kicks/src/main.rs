//! scarecrow-whole-kicks — The Scarecrow 🌾 plants Harvest's
//! journal and replay in `scarecrow-two-roads`' two-road field,
//! and coarsens the journal on the WEIGHT axis instead of the
//! tick axis: the sheaf rounded landings to the nearest 5
//! ticks; this journal rounds every kick to a whole kick.
//! Signs are written exact. Road A (-1.0) was whole already.
//! Road B (+0.75) is remembered as +1.0 — every one of its 702
//! entries wrong by exactly +0.25, a signed error budget of
//! +175.5 the true season never delivered.
//!
//! Journals written and replayed (the follower alone each
//! time, one neuron, no synapses — the journals are the whole
//! coupling):
//!
//! - Signed: (tick, weight) entries as they landed. Control.
//! - Whole: every weight rounded to the nearest whole kick.
//! - Shrunk: road B written as +0.5 — the same error budget
//!   (-175.5) spent in the opposite direction.
//! - Netted-whole: shared ticks merged to their sum first
//!   (the two-roads law), then every sum rounded whole; a
//!   shared tick's -0.25 becomes 0.0 and both kicks vanish.
//! - Unsigned: the two-roads reference anchor, every kick
//!   read as -1.0 (its divergence mark: tick 64).
//!
//! The counting instrument is the Widow's own, copied
//! faithfully from `scarecrow-two-roads` (super-cycles of
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
/// Signed journal record: u16 tick + i8 weight in quarter units
/// (-1.0 -> -4, +0.75 -> +3). The whole journal is counted at
/// the same 3 bytes an entry: its weights are only signs now,
/// but this counting writes the sign in the same byte.
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

/// The whole journal: every weight rounded to the nearest
/// whole kick. -1.0 stays -1.0; +0.75 becomes +1.0.
fn whole_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    journal.iter().map(|&(t, w)| (t, w.round())).collect()
}

/// The shrunk journal: positive kicks remembered small —
/// +0.75 written as +0.5 (magnitude floored to the nearest
/// half); negative kicks were already whole and stay.
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

/// Shared ticks merged to their sum (the two-roads netting),
/// in landing order.
fn netted_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    let mut netted: Vec<(usize, f32)> = Vec::new();
    for &(t, w) in journal {
        if let Some(last) = netted.last_mut().filter(|(lt, _)| *lt == t) {
            last.1 += w;
        } else {
            netted.push((t, w));
        }
    }
    netted
}

/// The netted-whole journal: net first, then round whole.
fn netted_whole_journal(journal: &[(usize, f32)]) -> Vec<(usize, f32)> {
    whole_journal(&netted_journal(journal))
}

/// Signed error budget of a spelling against the truth:
/// the sum over entries of (written weight - true weight),
/// matched in landing order (same length, same ticks).
fn signed_error(truth: &[(usize, f32)], spelling: &[(usize, f32)]) -> f32 {
    truth
        .iter()
        .zip(spelling.iter())
        .map(|(&(_, tw), &(_, sw))| sw - tw)
        .sum()
}

fn headless() {
    println!("scarecrow-whole-kicks headless ledger");
    println!(
        "host: scarecrow-two-roads' follower (FS I={}) under two roads — A: RS I={} weight {} delay {}; B: RS I={} weight {} delay {}; {} ticks, settle {}",
        FOLLOWER_CURRENT, DRIVER_A_CURRENT, WEIGHT_A, DELAY_A,
        DRIVER_B_CURRENT, WEIGHT_B, DELAY_B, TICKS, SETTLE_TICKS
    );
    println!("principle: journal and replay (Harvest) — the journal's ticks are exact; its weights are remembered whole. The follower is replayed alone from each spelling, never opened");

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
    let coinc = coincident_ticks(&live.journal);
    println!(
        "coincidences: {} ticks carry both roads' kicks; first at {:?}",
        coinc.len(), coinc.first()
    );

    let checkpoint_bytes = 3 * 6 * 4 + 8;
    let signed_bytes = live.journal.len() * SIGNED_ENTRY_BYTES;
    let unsigned_bytes = live.journal.len() * UNSIGNED_ENTRY_BYTES;
    println!(
        "paper sizes: one whole checkpoint of the field {} bytes (3 neurons x 6 f32 + flags); signed journal {} entries x {} B = {} bytes; whole journal counted at the same {} B an entry = {} bytes (coarsening the weight saves no paper); unsigned journal {} entries x {} B = {} bytes for comparison",
        checkpoint_bytes, live.journal.len(), SIGNED_ENTRY_BYTES, signed_bytes,
        SIGNED_ENTRY_BYTES, signed_bytes,
        live.journal.len(), UNSIGNED_ENTRY_BYTES, unsigned_bytes
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

    // Whole: the seed's spelling.
    let whole = whole_journal(&live.journal);
    let err_whole = signed_error(&live.journal, &whole);
    let replay_whole = run_replay(&whole);
    let div_whole = first_divergence(&live.follower_spikes, &replay_whole);
    let cell_whole = cell_of(&replay_whole);
    fmt_cell("replay-whole", &cell_whole);
    println!(
        "replay-whole: follower spikes {}, first divergence {:?}, signed error budget {:+.1} over {} road-B entries (+0.25 each)",
        replay_whole.len(), div_whole, err_whole, live.b_landings.len()
    );

    // Shrunk: the twin, error spent downward.
    let shrunk = shrunk_journal(&live.journal);
    let err_shrunk = signed_error(&live.journal, &shrunk);
    let replay_shrunk = run_replay(&shrunk);
    let div_shrunk = first_divergence(&live.follower_spikes, &replay_shrunk);
    let cell_shrunk = cell_of(&replay_shrunk);
    fmt_cell("replay-shrunk", &cell_shrunk);
    println!(
        "replay-shrunk: follower spikes {}, first divergence {:?}, signed error budget {:+.1} over {} road-B entries (-0.25 each)",
        replay_shrunk.len(), div_shrunk, err_shrunk, live.b_landings.len()
    );
    println!(
        "the twins compared: whole parts at {:?}, shrunk parts at {:?}; whole cell perfect {}, shrunk cell perfect {} — same absolute budget, opposite signs",
        div_whole, div_shrunk, cell_whole.perfect, cell_shrunk.perfect
    );

    // Netted-whole: sums first, then rounded whole.
    let netted_whole = netted_whole_journal(&live.journal);
    let erased = netted_whole.iter().filter(|&&(_, w)| w == 0.0).count();
    let replay_nw = run_replay(&netted_whole);
    let div_nw = first_divergence(&live.follower_spikes, &replay_nw);
    fmt_cell("replay-netted-whole", &cell_of(&replay_nw));
    println!(
        "replay-netted-whole: {} entries after merging, {} of them written as 0.0 (both kicks at a shared tick erased), follower spikes {}, first divergence {:?} (first coincidence at {:?})",
        netted_whole.len(), erased, replay_nw.len(), div_nw, coinc.first()
    );

    println!(
        "netted-whole vs whole: follower trains identical {} — the replay sums each tick before the ear hears it, so a shared tick's whole kicks (+1.0 and -1.0) already cancel to the 0.0 the netted spelling writes; the erasure is the whole journal's own arithmetic, not an extra error",
        replay_nw == replay_whole
    );

    // Unsigned: the family's reference anchor.
    let unsigned: Vec<(usize, f32)> = live.journal.iter().map(|&(t, _)| (t, WEIGHT_A)).collect();
    let err_unsigned = signed_error(&live.journal, &unsigned);
    let replay_unsigned = run_replay(&unsigned);
    let div_unsigned = first_divergence(&live.follower_spikes, &replay_unsigned);
    fmt_cell("replay-unsigned", &cell_of(&replay_unsigned));
    println!(
        "replay-unsigned: follower spikes {}, first divergence {:?} (first road-B landing at {:?}), signed error budget {:+.1}",
        replay_unsigned.len(), div_unsigned, live.b_landings.first(), err_unsigned
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
    let p1 = div_signed.is_none();
    let p2 = match (div_whole, live.b_landings.first()) {
        (Some(d), Some(&first_b)) => d >= first_b,
        _ => false,
    };
    let p3 = match (div_whole, div_unsigned) {
        (Some(w), Some(u)) => w > u,
        _ => false,
    };
    let p4 = cell_whole.verdict == "EPISODIC";
    let p5 = div_shrunk.is_some();
    let p6 = div_nw.is_some();
    let p7 = signed_bytes >= 10 * checkpoint_bytes;
    let p8 = cell_whole.perfect > live.ledger.perfect
        || cell_whole.longest_episode > live.ledger.longest_episode
        || cell_whole.count_slips < live.ledger.count_slips
        || cell_whole.phase_slips < live.ledger.phase_slips;
    println!(
        "verdicts: B1 hosts pin exact (sheaf centre + two-roads live ledger): {}; P1 signed journal replays the follower exact: {}; P2 whole journal diverges at or after the first road-B landing: {}; P3 whole journal parts later than the unsigned journal's mark: {}; P4 whole journal keeps the EPISODIC verdict: {}; P5 shrunk journal (error spent downward) diverges too: {}; P6 netted-whole journal diverges: {}; P7 coarsening the weight saves no paper (whole {} B == signed {} B, >= 10x checkpoint {} B): {}; P8 whole journal beats the truth on at least one headline column: {}",
        b1, p1, p2, p3, p4, p5, p6,
        signed_bytes, signed_bytes, checkpoint_bytes, p7, p8
    );
    println!("live-two-roads totals: perfect 83/112, count slips 12, phase slips 17, episodes 9, longest 16, mean episode 9.2, follower spikes 857, signed entries 1309, coincidences 24 (first at 464)");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-whole-kicks: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
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
    fn whole_journal_rounds_only_road_b() {
        let j = vec![(5, -1.0), (9, 0.75), (12, -1.0), (20, 0.75)];
        assert_eq!(
            whole_journal(&j),
            vec![(5, -1.0), (9, 1.0), (12, -1.0), (20, 1.0)]
        );
        assert_eq!(
            shrunk_journal(&j),
            vec![(5, -1.0), (9, 0.5), (12, -1.0), (20, 0.5)]
        );
        assert!((signed_error(&j, &whole_journal(&j)) - 0.5).abs() < 1e-6);
        assert!((signed_error(&j, &shrunk_journal(&j)) + 0.5).abs() < 1e-6);
    }

    #[test]
    fn netted_whole_erases_a_shared_tick() {
        // A brake and a spur on one tick sum to -0.25, and
        // -0.25 remembered whole is nothing at all.
        let j = vec![(5, -1.0), (5, 0.75), (9, -1.0)];
        assert_eq!(
            netted_whole_journal(&j),
            vec![(5, 0.0), (9, -1.0)]
        );
    }

    #[test]
    fn divergence_detector_names_a_planted_difference() {
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 9, 20]), None);
        assert_eq!(first_divergence(&[3, 9, 20], &[3, 10, 20]), Some(9));
        assert_eq!(first_divergence(&[3, 9], &[3, 9, 20]), Some(20));
    }

    fn cell_of_local(live: &LiveRun, train: &[usize]) -> CellLedger {
        count_cell(&post_settle(&live.a_spikes), &post_settle(train))
    }

    #[test]
    fn live_and_whole_totals_pinned() {
        // As found on the first run, pinned. P3 CRACKED by
        // equality: the whole journal parts at tick 64, the
        // unsigned journal's own tick (and the sheaf
        // coarse-5's), not later — a quarter-kick of error
        // and a sign error seven times its size move the
        // same first spike. The whole season still beats
        // the truth on count slips (11 < 12) while its
        // perfect count falls (75 < 83) — P8 held by that
        // one column.
        let live = run_live();
        assert_eq!(live.ledger.perfect, 83);
        let whole = whole_journal(&live.journal);
        assert!((signed_error(&live.journal, &whole) - 175.5).abs() < 0.01);
        let replay = run_replay(&whole);
        assert_eq!(replay.len(), 857);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), Some(64));
        let cell = cell_of_local(&live, &replay);
        assert_eq!(cell.perfect, 75);
        assert_eq!(cell.count_slips, 11);
        assert_eq!(cell.phase_slips, 26);
        assert_eq!(cell.episodes, 14);
        assert_eq!(cell.longest_episode, 12);
        assert_eq!(cell.verdict, "EPISODIC");
    }

    #[test]
    fn shrunk_twin_parts_later_and_outscores_the_truth() {
        // As found: the same budget spent downward parts at
        // 91, not 64, and its cell beats the live season on
        // every headline column (perfect 91 > 83, longest
        // 18 > 16, count slips 10 < 12, phase slips 11 < 17)
        // while being wrong from tick 91. Direction is data.
        let live = run_live();
        let shrunk = shrunk_journal(&live.journal);
        assert!((signed_error(&live.journal, &shrunk) + 175.5).abs() < 0.01);
        let replay = run_replay(&shrunk);
        assert_eq!(replay.len(), 855);
        assert_eq!(first_divergence(&live.follower_spikes, &replay), Some(91));
        let cell = cell_of_local(&live, &replay);
        assert_eq!(cell.perfect, 91);
        assert_eq!(cell.count_slips, 10);
        assert_eq!(cell.phase_slips, 11);
        assert_eq!(cell.longest_episode, 18);
        assert!(cell.perfect > live.ledger.perfect);
        assert!(cell.longest_episode > live.ledger.longest_episode);
    }

    #[test]
    fn netted_whole_is_the_whole_journal() {
        // As found: netting shared ticks to their sum and
        // then rounding whole writes 1,285 entries, 24 of
        // them 0.0 — and replays to the whole journal's
        // train exactly, because the replay already sums
        // each tick. The erasure adds no error of its own.
        let live = run_live();
        let nw = netted_whole_journal(&live.journal);
        assert_eq!(nw.len(), 1285);
        assert_eq!(nw.iter().filter(|&&(_, w)| w == 0.0).count(), 24);
        assert_eq!(run_replay(&nw), run_replay(&whole_journal(&live.journal)));
    }

    #[test]
    fn unsigned_anchor_parts_at_64_here_too() {
        let live = run_live();
        let unsigned: Vec<(usize, f32)> =
            live.journal.iter().map(|&(t, _)| (t, WEIGHT_A)).collect();
        assert_eq!(
            first_divergence(&live.follower_spikes, &run_replay(&unsigned)),
            Some(64)
        );
    }
}
