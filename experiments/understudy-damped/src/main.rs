//! # Understudy Damped 🎭
//!
//! Brought by The Understudy 🎭. Principal: `experiments/damped-moths`
//! (The Moth Keeper 🪔), where one cast of moths played in two
//! FDTD rooms — a bell (fed 26, burns 39, slow 21, refusals 121 of
//! which tempo 3) and a damped room (fed 24, burns 34, slow 16,
//! refusals 39 of which tempo **0**) — and the quiet room's tempo
//! channel starved, because a lone lamp's smoothed wavefront rises
//! at most ~0.03–0.06 a tick, at or under the moths' onset
//! threshold: the ear had been listening to interference all along.
//!
//! The Understudy performs the same two rooms with no field at
//! all: each moth owns a claim queue, and a "room" is a property
//! of the delivery. The **clean queue** delivers the sampled
//! lamp's own onsets exactly — no amplitude exists, so nothing can
//! fall under a threshold. The **echo queue** is a party line: at
//! every post the delivered stream adds one self-echo (+53 ticks)
//! of each of the lamp's onsets and a leak (+7 ticks) of every
//! other lamp's onsets, unattributable — the bell's mixture,
//! restaged as deliveries.
//!
//! The script is the principal's, unchanged: 24 moths, 3 nights of
//! 1,500 ticks, caution earned only by burning, a check samples up
//! to 3 intervals inside a 160-tick cap, CV 0.35 is the flicker
//! line, tempo deviation 35% / burnt match 20%, tuition −35,
//! feeding +25, aversion 150. Tolerances and predictions were
//! declared in the README *before* this crate was built or run;
//! the headless ledger prints the principal's numbers beside this
//! performance's and names each line HELD or CRACKED.
//!
//! `cargo run -p understudy-damped -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// --- The role's parameters, taken from the principal's script. ---

/// Moths in the cast (the principal's cast size).
const MOTHS: usize = 24;
/// Nights observed. Caution, health and memories carry across.
const NIGHTS: usize = 3;
/// Ticks per night (the principal's window).
const NIGHT_TICKS: u64 = 1500;
/// The honest lamp's emission period. Perfectly regular.
const HONEST_PERIOD: u64 = 16;
/// The irregular decoy's alternating intervals; mean 16, as honest.
const DECOY_SHORT: u64 = 6;
/// See [`DECOY_SHORT`].
const DECOY_LONG: u64 = 26;
/// The slow lamp's emission period. Perfectly regular, wrong tempo.
const SLOW_PERIOD: u64 = 48;
/// Intervals a checking moth tries to sample (the principal's 3).
const SAMPLE_INTERVALS: usize = 3;
/// The principal's tempo-check cap: a sample listens at most this
/// many ticks.
const SAMPLE_CAP: u64 = 160;
/// Peak-interval CV above which a flicker is judged irregular —
/// the principal's verdict line, unchanged.
const FLICKER_CV: f64 = 0.35;
/// A steady mean deviating from the home tempo by more than this
/// fraction is refused on tempo grounds (the principal's line).
const TEMPO_DEVIATION: f64 = 0.35;
/// A steady mean within this fraction of a burnt tempo is refused
/// on tempo grounds (the principal's line).
const BURNT_MATCH: f64 = 0.20;
/// Health a burn costs (the principal's tuition).
const BURN_DAMAGE: i32 = 35;
/// Health a feeding restores (the principal's reward).
const FEED_RESTORE: i32 = 25;
/// Ticks a refused moth will not rejoin its queue (the principal's
/// aversion, unchanged).
const AVERSION_TICKS: u64 = 150;

// --- The understudy's machinery. ---

/// A claim's wait in the queue: the replacement for a night spent
/// wandering a cave (as in this theater's earlier queue roles).
const WAIT_MIN: u64 = 300;
/// See [`WAIT_MIN`].
const WAIT_MAX: u64 = 500;
/// The echo queue's self-echo delay: one reflection, arriving late.
const ECHO_DELAY: u64 = 53;
/// The echo queue's cross-lamp leak delay.
const LEAK_DELAY: u64 = 7;
/// Phase of the irregular lamp's first onset; its intervals then
/// alternate 6/26 from there.
const IRREGULAR_PHASE: u64 = 3;
/// Phase of the slow lamp's onsets (period 48 from this offset).
const SLOW_PHASE: u64 = 5;

/// Which lamp a claim came from. The moth is never told; the
/// ledger-keeper is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lamp {
    Honest,
    Irregular,
    Slow,
}

const LAMPS: [Lamp; 3] = [Lamp::Honest, Lamp::Irregular, Lamp::Slow];

/// The lamp's own emission period as experienced at a feeding or a
/// burn (the tempo a moth memorises). The irregular lamp teaches
/// no tempo and never reaches this through a burn; its mean is 16.
fn lamp_period(lamp: Lamp) -> u64 {
    match lamp {
        Lamp::Honest => HONEST_PERIOD,
        Lamp::Irregular => HONEST_PERIOD,
        Lamp::Slow => SLOW_PERIOD,
    }
}

/// A lamp's own onsets in `(from, to]`, from its global clock.
/// Honest: every 16 ticks from 0. Slow: every 48 from phase 5.
/// Irregular: intervals alternating 6/26 from phase 3.
fn true_onsets(lamp: Lamp, from: u64, to: u64) -> Vec<u64> {
    let mut out = Vec::new();
    match lamp {
        Lamp::Honest => {
            let mut t = (from / HONEST_PERIOD + 1) * HONEST_PERIOD;
            while t <= to {
                out.push(t);
                t += HONEST_PERIOD;
            }
        }
        Lamp::Slow => {
            let mut t = if from < SLOW_PHASE {
                SLOW_PHASE
            } else {
                SLOW_PHASE + ((from - SLOW_PHASE) / SLOW_PERIOD + 1) * SLOW_PERIOD
            };
            while t <= to {
                out.push(t);
                t += SLOW_PERIOD;
            }
        }
        Lamp::Irregular => {
            // Walk the alternation from its phase; cheap at these
            // horizons and exact by construction.
            let mut t = IRREGULAR_PHASE;
            let mut short = true;
            while t <= to {
                if t > from {
                    out.push(t);
                }
                t += if short { DECOY_SHORT } else { DECOY_LONG };
                short = !short;
            }
        }
    }
    out
}

/// The onset stream a queue delivers at `lamp`'s post in
/// `(from, to]`. Clean queue: the lamp's own onsets, exactly.
/// Echo queue: own onsets, one self-echo of each at +ECHO_DELAY,
/// and a leak of every other lamp's onsets at +LEAK_DELAY —
/// sorted, with coincident deliveries merged (two arrivals on
/// the same tick are one onset to an ear).
fn delivered_onsets(lamp: Lamp, from: u64, to: u64, echo: bool) -> Vec<u64> {
    let mut out = true_onsets(lamp, from, to);
    if echo {
        if from >= ECHO_DELAY {
            for t in true_onsets(lamp, from - ECHO_DELAY, to - ECHO_DELAY) {
                out.push(t + ECHO_DELAY);
            }
        } else {
            for t in true_onsets(lamp, 0, to - ECHO_DELAY.min(to)) {
                let e = t + ECHO_DELAY;
                if e > from && e <= to {
                    out.push(e);
                }
            }
        }
        for other in LAMPS {
            if other == lamp {
                continue;
            }
            if from >= LEAK_DELAY {
                for t in true_onsets(other, from - LEAK_DELAY, to - LEAK_DELAY) {
                    out.push(t + LEAK_DELAY);
                }
            } else {
                for t in true_onsets(other, 0, to - LEAK_DELAY.min(to)) {
                    let e = t + LEAK_DELAY;
                    if e > from && e <= to {
                        out.push(e);
                    }
                }
            }
        }
        out.sort_unstable();
        out.dedup();
    }
    out
}

/// What a check gathered: the intervals heard, the ticks the
/// listening cost, and whether the full 3-interval sample arrived.
struct Sample {
    intervals: Vec<u64>,
    cost: u64,
    full: bool,
}

/// A checking moth listens from tick `at` for up to
/// [`SAMPLE_INTERVALS`] intervals, inside the principal's cap.
/// Cost is the stopped time actually spent: until the onset that
/// completes the last interval, or the whole cap if the sample
/// never completes. Fewer than two intervals is a starved sample.
fn sample(lamp: Lamp, at: u64, echo: bool) -> Sample {
    let onsets = delivered_onsets(lamp, at, at + SAMPLE_CAP, echo);
    if onsets.len() > SAMPLE_INTERVALS {
        let intervals: Vec<u64> = onsets
            .windows(2)
            .take(SAMPLE_INTERVALS)
            .map(|w| w[1] - w[0])
            .collect();
        Sample {
            cost: onsets[SAMPLE_INTERVALS] - at,
            intervals,
            full: true,
        }
    } else if onsets.len() >= 2 {
        let intervals: Vec<u64> = onsets.windows(2).map(|w| w[1] - w[0]).collect();
        Sample {
            cost: SAMPLE_CAP,
            intervals,
            full: false,
        }
    } else {
        Sample {
            intervals: Vec::new(),
            cost: SAMPLE_CAP,
            full: false,
        }
    }
}

/// Coefficient of variation, computed exactly as the principal
/// computes it (population variance over the sampled intervals).
fn cv(intervals: &[u64]) -> f64 {
    if intervals.is_empty() {
        return 0.0;
    }
    let mean = intervals.iter().sum::<u64>() as f64 / intervals.len() as f64;
    if mean == 0.0 {
        return 0.0;
    }
    let var = intervals
        .iter()
        .map(|&v| (v as f64 - mean).powi(2))
        .sum::<f64>()
        / intervals.len() as f64;
    var.sqrt() / mean
}

fn mean(intervals: &[u64]) -> f64 {
    intervals.iter().sum::<u64>() as f64 / intervals.len() as f64
}

/// The tempo verdict on a steady sample, the principal's rules in
/// the principal's order: home tempo first, burnt tempo only for
/// a moth that has never fed. Returns true to refuse.
fn tempo_refuses(sample_mean: f64, home: Option<f64>, burnt: Option<f64>) -> bool {
    if let Some(h) = home {
        (sample_mean - h).abs() / h > TEMPO_DEVIATION
    } else if let Some(b) = burnt {
        (sample_mean - b).abs() / b <= BURNT_MATCH
    } else {
        false
    }
}

/// What the keeper (never the moth) knows happened in a night in
/// one room. Field-for-field the principal's ledger, plus the
/// sample-evidence counts its check table reports.
#[derive(Debug, Default, Clone, Copy)]
struct NightLedger {
    fed: usize,
    burns: usize,
    slow_burns: usize,
    fooled_burns: usize,
    deaths: usize,
    checks: usize,
    refusals: usize,
    tempo_refusals: usize,
    starved: usize,
    full_samples: usize,
    samples_ge2: usize,
    heard_sum_honest: f64,
    heard_n_honest: usize,
    heard_sum_slow: f64,
    heard_n_slow: usize,
    home_holders: usize,
    burnt_holders: usize,
    alive_end: usize,
}

#[derive(Clone, Debug)]
struct Moth {
    health: i32,
    /// Burns survived. Caution is *earned*, never granted.
    caution: u32,
    /// Tempo memorised by feeding (the honest lamp's 16).
    home_tempo: Option<f64>,
    /// Tempo memorised by a steady burn (the slow lamp's 48).
    burnt_tempo: Option<f64>,
    dead: bool,
}

impl Moth {
    fn new() -> Self {
        Self {
            health: 100,
            caution: 0,
            home_tempo: None,
            burnt_tempo: None,
            dead: false,
        }
    }
    /// A burn's tuition: caution, and — only if the lamp was
    /// steady — a burnt tempo. An irregular burn teaches no tempo.
    fn burn(&mut self, lamp: Lamp, ledger: &mut NightLedger) {
        ledger.burns += 1;
        if lamp == Lamp::Slow {
            ledger.slow_burns += 1;
            self.burnt_tempo = Some(lamp_period(lamp) as f64);
        }
        self.health -= BURN_DAMAGE;
        self.caution += 1;
        if self.health <= 0 {
            self.dead = true;
            ledger.deaths += 1;
        }
    }
    fn feed(&mut self, ledger: &mut NightLedger) {
        ledger.fed += 1;
        self.health = (self.health + FEED_RESTORE).min(100);
        self.home_tempo = Some(HONEST_PERIOD as f64);
    }
}

/// One moth's night in one queue room. `night_start` maps the
/// moth's local clock onto the lamps' global clocks. The moths
/// share the clocks and nothing else.
fn run_night(moths: &mut [Moth], rng: &mut StdRng, echo: bool, night_start: u64) -> NightLedger {
    let mut ledger = NightLedger::default();
    for m in moths.iter_mut() {
        if m.dead {
            continue;
        }
        let mut t: u64 = 0;
        let mut settled = false;
        while !settled && t < NIGHT_TICKS {
            t += rng.gen_range(WAIT_MIN..=WAIT_MAX);
            if t >= NIGHT_TICKS {
                break;
            }
            // The next claim: equal odds among the three lamps.
            // Brightness, which in the principal pulled moths
            // across a room toward the decoys, has no channel here.
            let lamp = match rng.gen_range(0..3) {
                0 => Lamp::Honest,
                1 => Lamp::Irregular,
                _ => Lamp::Slow,
            };

            if m.caution == 0 {
                // Naive: commit at once, as the principal's do.
                match lamp {
                    Lamp::Honest => {
                        m.feed(&mut ledger);
                        settled = true;
                    }
                    Lamp::Irregular | Lamp::Slow => {
                        m.burn(lamp, &mut ledger);
                        if m.dead {
                            settled = true;
                        }
                    }
                }
                continue;
            }

            // Cautious: stop and sample the lamp as delivered.
            ledger.checks += 1;
            let s = sample(lamp, night_start + t, echo);
            t += s.cost;
            if s.full {
                ledger.full_samples += 1;
            }
            if s.intervals.len() >= 2 {
                ledger.samples_ge2 += 1;
                match lamp {
                    Lamp::Honest => {
                        ledger.heard_sum_honest += mean(&s.intervals);
                        ledger.heard_n_honest += 1;
                    }
                    Lamp::Slow => {
                        ledger.heard_sum_slow += mean(&s.intervals);
                        ledger.heard_n_slow += 1;
                    }
                    Lamp::Irregular => {}
                }
            }
            if s.intervals.len() < 2 {
                // Starved: no evidence to judge on. The moth does
                // not approach blind; it waits out its aversion
                // and queues again. (Prediction P4 declares this
                // never happens on a queue.)
                ledger.starved += 1;
                t += AVERSION_TICKS;
            } else if cv(&s.intervals) > FLICKER_CV {
                ledger.refusals += 1;
                t += AVERSION_TICKS;
            } else if tempo_refuses(mean(&s.intervals), m.home_tempo, m.burnt_tempo) {
                ledger.refusals += 1;
                ledger.tempo_refusals += 1;
                t += AVERSION_TICKS;
            } else if t < NIGHT_TICKS {
                match lamp {
                    Lamp::Honest => {
                        m.feed(&mut ledger);
                        settled = true;
                    }
                    Lamp::Irregular | Lamp::Slow => {
                        // A check that passed and still burned:
                        // counted as fooled, never hidden.
                        ledger.fooled_burns += 1;
                        m.burn(lamp, &mut ledger);
                        if m.dead {
                            settled = true;
                        }
                    }
                }
            }
        }
    }
    ledger.home_holders = moths.iter().filter(|m| m.home_tempo.is_some()).count();
    ledger.burnt_holders = moths.iter().filter(|m| m.burnt_tempo.is_some()).count();
    ledger.alive_end = moths.iter().filter(|m| !m.dead).count();
    ledger
}

/// One room, whole run: a fresh cast, three nights, the room's
/// delivery rule held constant. Both rooms play from the same
/// seed — the principal's twin-room discipline.
fn run_room(echo: bool, seed: u64) -> Vec<NightLedger> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    (0..NIGHTS)
        .map(|n| run_night(&mut moths, &mut rng, echo, n as u64 * NIGHT_TICKS))
        .collect()
}

/// The queue's ring-down analog: deliveries per true onset at
/// the honest post over a long window. Clean is 1.00 by
/// construction; the echo figure is measured, not asserted.
fn deliveries_per_true_onset(echo: bool) -> f64 {
    let (from, to) = (0u64, 4800u64);
    let truth = true_onsets(Lamp::Honest, from, to).len() as f64;
    let delivered = delivered_onsets(Lamp::Honest, from, to, echo).len() as f64;
    delivered / truth
}

// --- The comparison ledger: principal's lines, declared bands. ---

/// The principal's published ledgers per night:
/// (fed, burns, slow_burns, checks, refusals, tempo_refusals, alive).
const PRINCIPAL_BELL: [(usize, usize, usize, usize, usize, usize, usize); 3] = [
    (12, 16, 7, 42, 26, 0, 23),
    (9, 12, 8, 66, 39, 1, 23),
    (5, 11, 6, 87, 56, 2, 21),
];
/// See [`PRINCIPAL_BELL`]; the damped room.
const PRINCIPAL_DAMPED: [(usize, usize, usize, usize, usize, usize, usize); 3] = [
    (11, 12, 5, 37, 9, 0, 23),
    (7, 17, 7, 64, 13, 0, 21),
    (6, 5, 4, 64, 17, 0, 21),
];

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "CRACKED" }
}

fn totals(nights: &[NightLedger], f: fn(&NightLedger) -> usize) -> usize {
    nights.iter().map(|l| f(l)).sum()
}

fn print_room(name: &str, nights: &[NightLedger]) {
    for (n, l) in nights.iter().enumerate() {
        println!(
            "  {name} night {}: fed {}, burns {} (slow {}, fooled checks {}), deaths {}, checks {}, refusals {} (tempo {}), starved samples {}, holders home {} / burnt {}, alive at dawn {}",
            n + 1,
            l.fed,
            l.burns,
            l.slow_burns,
            l.fooled_burns,
            l.deaths,
            l.checks,
            l.refusals,
            l.tempo_refusals,
            l.starved,
            l.home_holders,
            l.burnt_holders,
            l.alive_end
        );
    }
    let heard_honest: f64 = {
        let (s, n) = nights
            .iter()
            .fold((0.0, 0usize), |a, l| (a.0 + l.heard_sum_honest, a.1 + l.heard_n_honest));
        if n > 0 { s / n as f64 } else { 0.0 }
    };
    let heard_slow: f64 = {
        let (s, n) = nights
            .iter()
            .fold((0.0, 0usize), |a, l| (a.0 + l.heard_sum_slow, a.1 + l.heard_n_slow));
        if n > 0 { s / n as f64 } else { 0.0 }
    };
    println!(
        "  {name} sample evidence (keeper-bucketed by intended lamp): samples >=2 intervals {}, full 3-interval {}, heard mean near honest {:.2} (emitted 16), near slow {:.2} (emitted 48, {} samples)",
        totals(nights, |l| l.samples_ge2),
        totals(nights, |l| l.full_samples),
        heard_honest,
        heard_slow,
        nights.iter().map(|l| l.heard_n_slow).sum::<usize>()
    );
}

fn run_headless() {
    let seed = 0x4441_4D50; // "DAMP"
    let clean = run_room(false, seed);
    let echo = run_room(true, seed);
    println!("understudy-damped headless: damped-moths' two rooms, performed by claim queues — no field on stage");
    println!(
        "  lamps on global clocks: honest period {HONEST_PERIOD} (feeds), irregular {DECOY_SHORT}/{DECOY_LONG} alternating (burns), slow period {SLOW_PERIOD} steady (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; check samples up to {SAMPLE_INTERVALS} intervals inside a {SAMPLE_CAP}-tick cap; both rooms play from seed {seed:#x}"
    );
    println!(
        "  deliveries per true onset at the honest post (the queue's ring-down; principal: bell 5480 ticks, damped 1881): clean {:.2}, echo {:.2}",
        deliveries_per_true_onset(false),
        deliveries_per_true_onset(true)
    );
    print_room("clean queue ", &clean);
    print_room("echo queue  ", &echo);

    let c_fed = totals(&clean, |l| l.fed);
    let c_slow = totals(&clean, |l| l.slow_burns);
    let c_ref = totals(&clean, |l| l.refusals);
    let c_tempo = totals(&clean, |l| l.tempo_refusals);
    let c_starved = totals(&clean, |l| l.starved);
    let e_ref = totals(&echo, |l| l.refusals);
    let e_tempo = totals(&echo, |l| l.tempo_refusals);
    let e_slow = totals(&echo, |l| l.slow_burns);
    let e_starved = totals(&echo, |l| l.starved);
    let p_damped_fed: usize = PRINCIPAL_DAMPED.iter().map(|p| p.0).sum();
    let p_damped_slow: usize = PRINCIPAL_DAMPED.iter().map(|p| p.2).sum();
    let p_bell_ref: usize = PRINCIPAL_BELL.iter().map(|p| p.4).sum();

    println!("  comparison against the principal, tolerances declared in the README before the first run:");
    println!(
        "    B1 clean fed total: {c_fed} vs principal damped {p_damped_fed} [{} in 18..=32]",
        verdict((18..=32).contains(&c_fed))
    );
    println!(
        "    B2 clean slow burns total: {c_slow} vs principal damped {p_damped_slow} [{} in 8..=22]",
        verdict((8..=22).contains(&c_slow))
    );
    println!(
        "    B3 echo refusals total: {e_ref} vs principal bell {p_bell_ref} [{} >= 60] and vs clean {c_ref} [{} echo > clean]",
        verdict(e_ref >= 60),
        verdict(e_ref > c_ref)
    );
    println!(
        "    B4 alive at night 3: clean {} / echo {} vs principal 21 / 21 [{} both in 18..=24]",
        clean[2].alive_end,
        echo[2].alive_end,
        verdict((18..=24).contains(&clean[2].alive_end) && (18..=24).contains(&echo[2].alive_end))
    );
    println!("  the predictions this performance exists to test, declared in advance:");
    println!(
        "    P1 the channel revives: clean tempo refusals {c_tempo} [{} >= 15] — principal damped 0",
        verdict(c_tempo >= 15)
    );
    println!(
        "    P2 interference mutes tempo: echo tempo refusals {e_tempo} [{} <= 8] — principal bell 3; echo slow burns {e_slow} >= clean slow burns {c_slow} [{}]",
        verdict(e_tempo <= 8),
        verdict(e_slow >= c_slow)
    );
    println!(
        "    P3 learning shows in the clean room: slow burns night 1 {} -> night 3 {} [{} night 3 < night 1] — principal damped 5 -> 4 on flicker evidence alone",
        clean[0].slow_burns,
        clean[2].slow_burns,
        verdict(clean[2].slow_burns < clean[0].slow_burns)
    );
    println!(
        "    P4 a queue cannot starve a check: starved samples clean {c_starved} / echo {e_starved} [{} both exactly 0] — principal damped gathered 13 full samples in three nights",
        verdict(c_starved == 0 && e_starved == 0)
    );
    println!(
        "  the crack, named in advance and kept whatever the numbers say: the principal's quiet room starved its tempo ear because a wavefront can fall under a threshold; a queue has no amplitude, so its quiet room cannot starve the sense — it can only feed it exactly, or (on the party line) forge it. Scarcity was the field's accent. Corruption is the queue's."
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: this experiment
    // has no TUI, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    run_headless();
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_queue_delivers_each_lamp_exactly() {
        // Honest and slow samples read their true periods, at any
        // starting tick, on the clean queue.
        for at in [0u64, 7, 100, 1234] {
            let h = sample(Lamp::Honest, at, false);
            assert!(h.intervals.iter().all(|&i| i == HONEST_PERIOD), "honest at {at}: {:?}", h.intervals);
            assert_eq!(mean(&h.intervals), 16.0);
            let s = sample(Lamp::Slow, at, false);
            assert!(s.intervals.len() >= 2, "slow sample starved at {at}");
            assert!(s.intervals.iter().all(|&i| i == SLOW_PERIOD), "slow at {at}: {:?}", s.intervals);
            assert_eq!(mean(&s.intervals), 48.0);
        }
    }

    #[test]
    fn the_irregular_clock_reads_irregular_in_both_rooms() {
        for echo in [false, true] {
            let iv = sample(Lamp::Irregular, 500, echo);
            assert!(
                cv(&iv.intervals) > FLICKER_CV,
                "irregular read steady (echo={echo}): {:?} cv {}",
                iv.intervals,
                cv(&iv.intervals)
            );
        }
    }

    #[test]
    fn echo_queue_contaminates_every_post() {
        // Deliveries per true onset: the party line talks more
        // than the lamp does.
        assert!(deliveries_per_true_onset(true) > 2.0);
        assert_eq!(deliveries_per_true_onset(false), 1.0);
        // At the slow post, the mixture reads fast: the heard mean
        // sits far under the lamp's true 48 — the honest lamp's
        // leaked 16-grid dominates the sample.
        let s = sample(Lamp::Slow, 2000, true);
        assert!(s.intervals.len() >= 2);
        assert!(
            mean(&s.intervals) < 24.0,
            "echo slow sample read {} — the mixture should read fast",
            mean(&s.intervals)
        );
    }

    #[test]
    fn a_sample_never_exceeds_the_principals_cap_by_much() {
        // Listening cost is bounded by the cap plus at most one
        // interval's overshoot (the completing onset lands where
        // it lands; here intervals are <= 48, cap 160).
        for echo in [false, true] {
            for lamp in LAMPS {
                for at in [0u64, 333, 4096] {
                    let s = sample(lamp, at, echo);
                    assert!(s.cost <= SAMPLE_CAP + SLOW_PERIOD, "cost {} (echo={echo}, {lamp:?}, at {at})", s.cost);
                }
            }
        }
    }

    #[test]
    fn tempo_verdicts_follow_the_principals_rules() {
        assert!(tempo_refuses(48.0, Some(16.0), None));
        assert!(!tempo_refuses(16.0, Some(16.0), None));
        assert!(tempo_refuses(48.0, None, Some(48.0)));
        assert!(!tempo_refuses(16.0, None, Some(48.0)));
        assert!(!tempo_refuses(48.0, None, None));
    }

    #[test]
    fn the_decoys_fool_the_naive_in_both_rooms() {
        for echo in [false, true] {
            let nights = run_room(echo, 0x4441_4D50);
            assert!(nights[0].burns > 0, "no decoy fooled anyone on night 1 (echo={echo})");
            assert!(nights[0].slow_burns > 0, "the slow lamp burned no one on night 1 (echo={echo})");
        }
    }

    #[test]
    fn no_check_starves_on_a_queue() {
        for echo in [false, true] {
            let nights = run_room(echo, 0x4441_4D50);
            assert_eq!(
                nights.iter().map(|l| l.starved).sum::<usize>(),
                0,
                "a queue starved a check (echo={echo})"
            );
            assert!(
                nights[NIGHTS - 1].alive_end >= MOTHS / 2,
                "the cast was exterminated (echo={echo})"
            );
        }
    }
}
