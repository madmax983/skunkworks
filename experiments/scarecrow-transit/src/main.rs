//! # Scarecrow Transit \U0001f33e
//!
//! Brought by The Scarecrow \U0001f33e. Principle: **interruption
//! equivalence**, learned from Harvest — the second time I have
//! planted it, in a wider field. Scarecrow-winter proved a full
//! checkpoint of the Widow's timed thread resumes spike for
//! spike while a neurons-only snapshot does not. But winter's
//! delay line held exactly **one** kick at its checkpoint. One
//! kick is a thin proof: a checkpoint can be right about one
//! travelling thing by luck and wrong about a crowd.
//!
//! Host: `crates/neuro-sim`'s own machinery, arranged as the
//! Widow's timed thread **braided**: the same RS driver (I=12)
//! and FS follower (I=6) as `widow-subharmonic`, joined not by
//! one synapse but by four parallel lanes — the timed thread
//! itself (weight 10.0, delay 5) and three echo lanes (2.0 at
//! delay 18, 2.0 at delay 31, 1.0 at delay 44). Kicks are
//! always travelling. At any tick just after a driver spike,
//! several lanes hold a kick at once, at different distances
//! from the follower. That crowd in transit is this run's
//! fragile point, and it is the seed scarecrow-winter's own
//! succession notes named: interruption equivalence with
//! several kicks in transit.
//!
//! Four runs stand in the same field for 6,000 ticks:
//! - uninterrupted: nobody touches it.
//! - full checkpoint: at the first tick >= 3,000 with at least
//!   three kicks in transit across the lanes, the whole network
//!   is written down as text (every float as its bits, every
//!   lane's transit queue in order), the running network is
//!   dropped, the text is read back, and the run finishes from
//!   the paper.
//! - neurons-only checkpoint (winter's control): the same stop,
//!   only the neurons written down; lanes rebuilt empty.
//! - transit-dropped checkpoint (the sharper control winter
//!   never ran): everything written down *except* the transit
//!   queues — neurons, lane weights and delays, spike cache,
//!   inputs, all of it — and the queues alone left on the road.
//!   This pass prices the crowd itself, lane by lane: if it
//!   resumes exact, the kicks in transit were decoration even
//!   in a crowd.
//!
//! The ledger is the Widow's own: follower beats counted per
//! driver cycle, plus the spike-for-spike comparison winter
//! added. Predictions were declared in the README before the
//! first run. `cargo run -p scarecrow-transit -- --headless`
//! prints every side and the verdicts. --headless is handled
//! before anything else; there is no TUI.

use anyhow::{Context, Result};
use neuro_sim::{Izhikevich, Network};

/// The driver's constant drive current (the host's own).
const DRIVER_CURRENT: f32 = 12.0;
/// The follower's constant drive current (the host's own).
const FOLLOWER_CURRENT: f32 = 6.0;
/// The braided thread: (weight, delay) per lane. The first lane
/// is the Widow's timed thread itself, as the host found it
/// (weight 10.0, delay 5); the other three are its echoes.
const LANES: [(f32, usize); 4] = [(10.0, 5), (2.0, 18), (2.0, 31), (1.0, 44)];
/// Several, declared: winter does not come until at least this
/// many kicks are travelling across the lanes at once.
const TRANSIT_MIN: usize = 3;
/// Total ticks per run.
const TICKS: usize = 6_000;
/// Ticks discarded as settling (the host's own window).
const SETTLE_TICKS: usize = 1_500;
/// A checkpoint may be taken no earlier than this tick — the
/// middle of the run, deep inside the measured window, so a lost
/// kick is lost on the record and not in the settling dark.
const CHECKPOINT_EARLIEST: usize = 3_000;

/// The host pair, built as `widow-subharmonic` builds it, with
/// its one thread braided into four lanes on the same two neurons.
fn build_pair() -> Network {
    let mut net = Network::new();
    let driver = net.add_neuron();
    let follower = net.add_neuron();
    net.neurons[follower] = Izhikevich::new_fast_spiking();
    for (weight, delay) in LANES {
        net.add_synapse_with_delay(driver, follower, weight, delay);
    }
    net
}

/// Kicks travelling across every lane, counted together.
fn total_transit(net: &Network) -> usize {
    net.synapses.iter().map(|s| s.spikes_in_transit.len()).sum()
}

/// Kicks travelling in each lane, in lane order.
fn transit_by_lane(net: &Network) -> Vec<usize> {
    net.synapses.iter().map(|s| s.spikes_in_transit.len()).collect()
}

/// One run's spike record (ticks >= SETTLE_TICKS only, as the host
/// counts) plus what happened at its checkpoint, if it had one.
#[derive(Debug, Clone)]
struct RunOutcome {
    driver_times: Vec<usize>,
    follower_times: Vec<usize>,
    /// The tick after which the checkpoint was taken, if any.
    checkpoint_tick: Option<usize>,
    /// Kicks travelling in the synapse at the checkpoint.
    transit_at_checkpoint: usize,
    /// Kicks travelling in each lane at the checkpoint.
    transit_by_lane_at_checkpoint: Vec<usize>,
    /// Size of the written checkpoint, in bytes of text.
    checkpoint_bytes: usize,
}

/// The host's ledger, counted from spike times alone.
#[derive(Debug, Clone, PartialEq)]
struct Ledger {
    driver_spikes: usize,
    follower_spikes: usize,
    driver_mean_isi: f64,
    follower_mean_isi: f64,
    perfect_cycles: usize,
    short_cycles: usize,
    crowded_cycles: usize,
    first_dispersion: f64,
    second_dispersion: f64,
    first_mean_phase: f64,
    second_mean_phase: f64,
    locked: bool,
}

fn mean_isi(times: &[usize]) -> f64 {
    if times.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = times.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

fn circular(phases: &[f64]) -> (f64, f64) {
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    for p in phases {
        let ang = p * std::f64::consts::TAU;
        sx += ang.cos();
        sy += ang.sin();
    }
    let n = phases.len().max(1) as f64;
    let r = ((sx / n).powi(2) + (sy / n).powi(2)).sqrt();
    let mut mean = (sy / n).atan2(sx / n) / std::f64::consts::TAU;
    if mean < 0.0 {
        mean += 1.0;
    }
    (1.0 - r, mean)
}

/// Count a ledger from spike times, by the host instrument's exact
/// definitions (exactly two follower beats per driver cycle, both
/// dispersions under 0.05, or it is not a lock).
fn ledger_from_times(driver_times: &[usize], follower_times: &[usize]) -> Ledger {
    let mut perfect = 0usize;
    let mut short = 0usize;
    let mut crowded = 0usize;
    let mut first_phases: Vec<f64> = Vec::new();
    let mut second_phases: Vec<f64> = Vec::new();
    for w in driver_times.windows(2) {
        let (start, end) = (w[0], w[1]);
        let inside: Vec<usize> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .collect();
        match inside.len() {
            2 => perfect += 1,
            n if n < 2 => short += 1,
            _ => crowded += 1,
        }
        if let Some(&f) = inside.first() {
            first_phases.push((f - start) as f64 / (end - start) as f64);
        }
        if let Some(&f) = inside.get(1) {
            second_phases.push((f - start) as f64 / (end - start) as f64);
        }
    }
    let (first_dispersion, first_mean_phase) = circular(&first_phases);
    let (second_dispersion, second_mean_phase) = circular(&second_phases);
    let cycles = driver_times.len().saturating_sub(1).max(1);
    Ledger {
        driver_spikes: driver_times.len(),
        follower_spikes: follower_times.len(),
        driver_mean_isi: mean_isi(driver_times),
        follower_mean_isi: mean_isi(follower_times),
        perfect_cycles: perfect,
        short_cycles: short,
        crowded_cycles: crowded,
        first_dispersion,
        second_dispersion,
        first_mean_phase,
        second_mean_phase,
        locked: perfect == cycles && first_dispersion < 0.05 && second_dispersion < 0.05,
    }
}

// --- The writing down -------------------------------------------------

/// Write the whole network down. Every float goes as its bits, so
/// the paper holds the field exactly, not approximately.
fn write_full(net: &Network) -> String {
    let mut s = String::new();
    s.push_str("scarecrow-transit/full\n");
    for n in &net.neurons {
        s.push_str(&format!(
            "neuron {} {} {} {} {} {} {} {}\n",
            n.v.to_bits(),
            n.u.to_bits(),
            n.a.to_bits(),
            n.b.to_bits(),
            n.c.to_bits(),
            n.d.to_bits(),
            n.current_decay.to_bits(),
            n.tau.to_bits(),
        ));
    }
    for syn in &net.synapses {
        let transit: Vec<String> = syn
            .spikes_in_transit
            .iter()
            .map(|t| t.to_string())
            .collect();
        s.push_str(&format!(
            "synapse {} {} {} {} {} [{}]\n",
            syn.from,
            syn.to,
            syn.weight.to_bits(),
            syn.delay,
            syn.active as u8,
            transit.join(","),
        ));
    }
    let spikes: Vec<String> = net.spikes.iter().map(|b| (*b as u8).to_string()).collect();
    s.push_str(&format!("spikes {}\n", spikes.join(",")));
    let inputs: Vec<String> = net.inputs.iter().map(|f| f.to_bits().to_string()).collect();
    s.push_str(&format!("inputs {}\n", inputs.join(",")));
    s
}

fn parse_bits(tok: &str) -> Result<f32> {
    Ok(f32::from_bits(
        tok.parse::<u32>().context("float bits")?,
    ))
}

/// Read a full checkpoint back off the paper.
fn read_full(text: &str) -> Result<Network> {
    let mut net = Network::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.first().copied() {
            Some("neuron") => {
                let idx = net.add_neuron();
                net.neurons[idx] = Izhikevich {
                    v: parse_bits(parts[1])?,
                    u: parse_bits(parts[2])?,
                    a: parse_bits(parts[3])?,
                    b: parse_bits(parts[4])?,
                    c: parse_bits(parts[5])?,
                    d: parse_bits(parts[6])?,
                    current_decay: parse_bits(parts[7])?,
                    tau: parse_bits(parts[8])?,
                };
            }
            Some("synapse") => {
                let from: usize = parts[1].parse()?;
                let to: usize = parts[2].parse()?;
                let weight = parse_bits(parts[3])?;
                let delay: usize = parts[4].parse()?;
                let active: bool = parts[5] == "1";
                let transit_raw = parts[6].trim_start_matches('[').trim_end_matches(']');
                let transit: Vec<usize> = if transit_raw.is_empty() {
                    Vec::new()
                } else {
                    transit_raw
                        .split(',')
                        .map(|t| t.parse::<usize>().context("transit"))
                        .collect::<Result<_, _>>()?
                };
                net.add_synapse_with_delay(from, to, weight, delay);
                let syn = net.synapses.last_mut().expect("synapse just added");
                syn.active = active;
                syn.spikes_in_transit = transit;
            }
            Some("spikes") => {
                if parts.len() > 1 {
                    net.spikes = parts[1]
                        .split(',')
                        .map(|b| b == "1")
                        .collect();
                }
            }
            Some("inputs") => {
                if parts.len() > 1 {
                    net.inputs = parts[1]
                        .split(',')
                        .map(|t| parse_bits(t))
                        .collect::<Result<_, _>>()?;
                }
            }
            _ => {}
        }
    }
    Ok(net)
}

/// Write down only the neurons — the obvious part of the state.
fn write_neurons_only(net: &Network) -> String {
    let mut s = String::new();
    s.push_str("scarecrow-transit/neurons-only\n");
    for n in &net.neurons {
        s.push_str(&format!(
            "neuron {} {} {} {} {} {} {} {}\n",
            n.v.to_bits(),
            n.u.to_bits(),
            n.a.to_bits(),
            n.b.to_bits(),
            n.c.to_bits(),
            n.d.to_bits(),
            n.current_decay.to_bits(),
            n.tau.to_bits(),
        ));
    }
    s
}

/// Restore a neurons-only checkpoint: fresh pair, neurons copied,
/// delay line empty, spike cache cleared. The topology is rebuilt
/// by hand because the paper never mentioned it.
fn read_neurons_only(text: &str) -> Result<Network> {
    let mut net = build_pair();
    let mut idx = 0usize;
    for line in text.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.first().copied() == Some("neuron") {
            net.neurons[idx] = Izhikevich {
                v: parse_bits(parts[1])?,
                u: parse_bits(parts[2])?,
                a: parse_bits(parts[3])?,
                b: parse_bits(parts[4])?,
                c: parse_bits(parts[5])?,
                d: parse_bits(parts[6])?,
                current_decay: parse_bits(parts[7])?,
                tau: parse_bits(parts[8])?,
            };
            idx += 1;
        }
    }
    Ok(net)
}

// --- The three runs ----------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Uninterrupted,
    FullCheckpoint,
    NeuronsOnly,
    TransitDropped,
}

fn run(mode: Mode) -> Result<RunOutcome> {
    let mut net = build_pair();
    let mut driver_times: Vec<usize> = Vec::new();
    let mut follower_times: Vec<usize> = Vec::new();
    let mut checkpoint_tick: Option<usize> = None;
    let mut transit_at_checkpoint = 0usize;
    let mut transit_by_lane_at_checkpoint: Vec<usize> = Vec::new();
    let mut checkpoint_bytes = 0usize;

    let record = |net: &Network, t: usize, d: &mut Vec<usize>, f: &mut Vec<usize>| {
        if t >= SETTLE_TICKS {
            if net.is_spiking(0) {
                d.push(t);
            }
            if net.is_spiking(1) {
                f.push(t);
            }
        }
    };

    let mut t = 0usize;
    while t < TICKS {
        net.step(&[DRIVER_CURRENT, FOLLOWER_CURRENT]);
        record(&net, t, &mut driver_times, &mut follower_times);
        t += 1;
        // Winter comes at the first tick >= CHECKPOINT_EARLIEST at
        // which at least TRANSIT_MIN kicks are in transit across
        // the lanes. A checkpoint taken over near-empty lanes
        // would test nothing, and the Scarecrow does not plant
        // decorations.
        if mode != Mode::Uninterrupted
            && checkpoint_tick.is_none()
            && t >= CHECKPOINT_EARLIEST
            && total_transit(&net) >= TRANSIT_MIN
        {
            transit_at_checkpoint = total_transit(&net);
            transit_by_lane_at_checkpoint = transit_by_lane(&net);
            let paper = match mode {
                Mode::FullCheckpoint => write_full(&net),
                Mode::NeuronsOnly => write_neurons_only(&net),
                Mode::TransitDropped => write_full(&net),
                Mode::Uninterrupted => unreachable!(),
            };
            checkpoint_bytes = paper.len();
            checkpoint_tick = Some(t);
            // The running network is dropped. Only the paper stays.
            net = match mode {
                Mode::FullCheckpoint => read_full(&paper)?,
                Mode::NeuronsOnly => read_neurons_only(&paper)?,
                Mode::TransitDropped => {
                    let mut restored = read_full(&paper)?;
                    for syn in restored.synapses.iter_mut() {
                        syn.spikes_in_transit.clear();
                    }
                    restored
                }
                Mode::Uninterrupted => unreachable!(),
            };
        }
    }

    Ok(RunOutcome {
        driver_times,
        follower_times,
        checkpoint_tick,
        transit_at_checkpoint,
        transit_by_lane_at_checkpoint,
        checkpoint_bytes,
    })
}

/// First tick at which two spike records disagree, and how many
/// spikes in `b` fall on ticks where `a` has none (plus the
/// reverse), counted on the union of both records.
fn divergence(a: &[usize], b: &[usize]) -> (Option<usize>, usize) {
    use std::collections::BTreeSet;
    let sa: BTreeSet<usize> = a.iter().copied().collect();
    let sb: BTreeSet<usize> = b.iter().copied().collect();
    let diff: Vec<usize> = sa.symmetric_difference(&sb).copied().collect();
    (diff.first().copied(), diff.len())
}

fn print_run(label: &str, out: &RunOutcome) {
    let l = ledger_from_times(&out.driver_times, &out.follower_times);
    println!(
        "  {label}: driver {} spikes (period {:.2}), follower {} spikes (period {:.2})",
        l.driver_spikes, l.driver_mean_isi, l.follower_spikes, l.follower_mean_isi
    );
    let cycles = l.perfect_cycles + l.short_cycles + l.crowded_cycles;
    println!(
        "    cycles exactly-two {}/{}, short {}, crowded {}, first beat phase {:.3} (dispersion {:.4}), second beat phase {:.3} (dispersion {:.4}) — {}",
        l.perfect_cycles,
        cycles,
        l.short_cycles,
        l.crowded_cycles,
        l.first_mean_phase,
        l.first_dispersion,
        l.second_mean_phase,
        l.second_dispersion,
        if l.locked { "LOCKED 2:1" } else { "WALKING" }
    );
    if let Some(tick) = out.checkpoint_tick {
        println!(
            "    checkpoint after tick {}, {} kick(s) in transit (by lane {:?}), paper {} bytes",
            tick, out.transit_at_checkpoint, out.transit_by_lane_at_checkpoint, out.checkpoint_bytes
        );
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");

    let plain = run(Mode::Uninterrupted)?;
    let full = run(Mode::FullCheckpoint)?;
    let naive = run(Mode::NeuronsOnly)?;
    let dropped = run(Mode::TransitDropped)?;

    println!("scarecrow-transit headless: one braided thread, four winters \u{2014} written down whole, written down without its road, written down in part, never stopped");
    println!(
        "  host: widow-subharmonic's pair braided (RS I={DRIVER_CURRENT} x FS I={FOLLOWER_CURRENT}); lanes (weight, delay) {LANES:?}; window {TICKS} ticks, first {SETTLE_TICKS} settling; winter at the first tick >= {CHECKPOINT_EARLIEST} with >= {TRANSIT_MIN} kicks in transit"
    );
    print_run("uninterrupted", &plain);
    print_run("full checkpoint", &full);
    print_run("transit-dropped", &dropped);
    print_run("neurons-only", &naive);

    let plain_l = ledger_from_times(&plain.driver_times, &plain.follower_times);
    let full_l = ledger_from_times(&full.driver_times, &full.follower_times);
    let naive_l = ledger_from_times(&naive.driver_times, &naive.follower_times);
    let dropped_l = ledger_from_times(&dropped.driver_times, &dropped.follower_times);

    let (full_first, full_diff) = divergence(&plain.follower_times, &full.follower_times);
    let (_full_first_d, full_diff_d) = divergence(&plain.driver_times, &full.driver_times);
    println!(
        "  full vs uninterrupted: follower spikes differing {}, driver spikes differing {}, first divergence {} \u{2014} {}",
        full_diff,
        full_diff_d,
        match full_first {
            Some(t) => format!("tick {t}"),
            None => "none".to_string(),
        },
        if full_diff == 0 && full_diff_d == 0 && full_l == plain_l {
            "HELD, spike for spike"
        } else {
            "DIVERGED"
        }
    );
    let (drop_first, drop_diff) = divergence(&plain.follower_times, &dropped.follower_times);
    let (_, drop_diff_d) = divergence(&plain.driver_times, &dropped.driver_times);
    println!(
        "  transit-dropped vs uninterrupted: follower spikes differing {}, driver spikes differing {}, first divergence {}",
        drop_diff,
        drop_diff_d,
        match drop_first {
            Some(t) => format!("tick {t}"),
            None => "none".to_string(),
        }
    );
    println!(
        "    transit-dropped ledger: perfect cycles {} (uninterrupted {}), dispersions {:.4}/{:.4} (uninterrupted {:.4}/{:.4}) \u{2014} {}",
        dropped_l.perfect_cycles,
        plain_l.perfect_cycles,
        dropped_l.first_dispersion,
        dropped_l.second_dispersion,
        plain_l.first_dispersion,
        plain_l.second_dispersion,
        if dropped_l.locked { "LOCKED 2:1" } else { "WALKING" }
    );
    println!(
        "  transit-dropped verdict: [{} \u{2014} declared: the dropped crowd must cost at least 3 follower spikes, first divergence within 120 ticks of winter]",
        if drop_diff >= 3
            && drop_first.is_some_and(|f| f <= dropped.checkpoint_tick.unwrap_or(0) + 120)
        {
            "HELD"
        } else {
            "DIVERGED from declaration"
        }
    );
    let (naive_first, naive_diff) = divergence(&plain.follower_times, &naive.follower_times);
    let (_, naive_diff_d) = divergence(&plain.driver_times, &naive.driver_times);
    println!(
        "  neurons-only vs uninterrupted: follower spikes differing {}, driver spikes differing {}, first divergence {}",
        naive_diff,
        naive_diff_d,
        match naive_first {
            Some(t) => format!("tick {t}"),
            None => "none".to_string(),
        }
    );
    println!(
        "    neurons-only ledger: perfect cycles {} (uninterrupted {}), dispersions {:.4}/{:.4} (uninterrupted {:.4}/{:.4}) \u{2014} {}",
        naive_l.perfect_cycles,
        plain_l.perfect_cycles,
        naive_l.first_dispersion,
        naive_l.second_dispersion,
        plain_l.first_dispersion,
        plain_l.second_dispersion,
        if naive_l.locked { "LOCKED 2:1" } else { "WALKING" }
    );
    println!(
        "  verdict: the field written down whole survives the winter {}; the crowd left on the road {}; the field written down in part {}",
        if full_diff == 0 { "exactly" } else { "with losses" },
        if drop_diff > 0 {
            "costs the run its own beats"
        } else {
            "cost nothing this winter \u{2014} the crowd was decoration, said plainly"
        },
        if naive_diff > 0 {
            "loses the kicks that were already on their way"
        } else {
            "survived this winter \u{2014} no kick was lost that mattered"
        }
    );
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_braid_is_alive_uninterrupted() {
        // The braid is a living host, not a stage set: the driver
        // keeps its period, the follower beats often, and the
        // Widow's instrument can count every driver cycle.
        let out = run(Mode::Uninterrupted).unwrap();
        let l = ledger_from_times(&out.driver_times, &out.follower_times);
        assert!(l.driver_spikes >= 100, "driver too quiet: {l:?}");
        assert!(l.follower_spikes >= 150, "follower too quiet: {l:?}");
        assert_eq!(
            l.perfect_cycles + l.short_cycles + l.crowded_cycles,
            l.driver_spikes - 1
        );
    }

    #[test]
    fn checkpoint_is_taken_with_several_kicks_in_transit() {
        let out = run(Mode::FullCheckpoint).unwrap();
        assert!(out.checkpoint_tick.is_some(), "winter must come");
        assert!(
            out.transit_at_checkpoint >= TRANSIT_MIN,
            "the checkpoint rule waits for a crowd in flight, got {}",
            out.transit_at_checkpoint
        );
        assert_eq!(
            out.transit_by_lane_at_checkpoint.iter().sum::<usize>(),
            out.transit_at_checkpoint,
            "the lane counts must add to the crowd"
        );
        assert!(
            out.transit_by_lane_at_checkpoint.iter().filter(|&&n| n > 0).count() >= 2,
            "the crowd must span lanes, not hide in one: {:?}",
            out.transit_by_lane_at_checkpoint
        );
        assert!(
            out.checkpoint_tick.unwrap() >= CHECKPOINT_EARLIEST,
            "not before the middle of the run"
        );
    }

    #[test]
    fn full_checkpoint_roundtrip_preserves_the_field() {
        // Step a pair to the checkpoint rule, write it down, read
        // it back: every neuron bit and the transit queue survive.
        let mut net = build_pair();
        let mut t = 0usize;
        loop {
            net.step(&[DRIVER_CURRENT, FOLLOWER_CURRENT]);
            t += 1;
            if t >= CHECKPOINT_EARLIEST && total_transit(&net) >= TRANSIT_MIN {
                break;
            }
        }
        let restored = read_full(&write_full(&net)).unwrap();
        for (a, b) in net.neurons.iter().zip(restored.neurons.iter()) {
            assert_eq!(a.v.to_bits(), b.v.to_bits());
            assert_eq!(a.u.to_bits(), b.u.to_bits());
            assert_eq!(a.current_decay.to_bits(), b.current_decay.to_bits());
        }
        for (a, b) in net.synapses.iter().zip(restored.synapses.iter()) {
            assert_eq!(a.spikes_in_transit, b.spikes_in_transit);
            assert_eq!(a.weight.to_bits(), b.weight.to_bits());
            assert_eq!(a.delay, b.delay);
        }
        assert_eq!(net.spikes, restored.spikes);
    }

    #[test]
    fn full_checkpoint_is_spike_identical() {
        let plain = run(Mode::Uninterrupted).unwrap();
        let full = run(Mode::FullCheckpoint).unwrap();
        assert_eq!(plain.driver_times, full.driver_times);
        assert_eq!(plain.follower_times, full.follower_times);
        assert_eq!(
            ledger_from_times(&plain.driver_times, &plain.follower_times),
            ledger_from_times(&full.driver_times, &full.follower_times)
        );
    }

    #[test]
    fn neurons_only_checkpoint_diverges() {
        let plain = run(Mode::Uninterrupted).unwrap();
        let naive = run(Mode::NeuronsOnly).unwrap();
        let (first, diff) = divergence(&plain.follower_times, &naive.follower_times);
        assert!(diff >= 3, "the lost kick must cost beats: {diff} differing");
        let first = first.expect("a first divergence tick");
        let cp = naive.checkpoint_tick.unwrap();
        assert!(
            first <= cp + 120,
            "divergence at {first}, checkpoint at {cp} — the loss must show soon"
        );
    }

    #[test]
    fn transit_dropped_checkpoint_diverges() {
        let plain = run(Mode::Uninterrupted).unwrap();
        let dropped = run(Mode::TransitDropped).unwrap();
        // The dropped pass winters at the same tick, over the same
        // crowd \u{2014} only the queues are left behind.
        let full = run(Mode::FullCheckpoint).unwrap();
        assert_eq!(dropped.checkpoint_tick, full.checkpoint_tick);
        assert_eq!(dropped.transit_at_checkpoint, full.transit_at_checkpoint);
        let (first, diff) = divergence(&plain.follower_times, &dropped.follower_times);
        assert!(diff >= 3, "the dropped crowd must cost beats: {diff} differing");
        let first = first.expect("a first divergence tick");
        let cp = dropped.checkpoint_tick.unwrap();
        assert!(
            first <= cp + 120,
            "divergence at {first}, checkpoint at {cp} \u{2014} the loss must show soon"
        );
    }

    #[test]
    fn every_checkpoint_mode_winters_at_the_same_tick() {
        // Up to winter the four runs are one run; the stop must
        // land identically or the comparisons are confounded.
        let full = run(Mode::FullCheckpoint).unwrap();
        let naive = run(Mode::NeuronsOnly).unwrap();
        let dropped = run(Mode::TransitDropped).unwrap();
        assert_eq!(full.checkpoint_tick, naive.checkpoint_tick);
        assert_eq!(full.checkpoint_tick, dropped.checkpoint_tick);
        assert_eq!(full.transit_at_checkpoint, naive.transit_at_checkpoint);
        assert_eq!(full.transit_at_checkpoint, dropped.transit_at_checkpoint);
    }
}
