//! # Understudy Moths 🎭
//!
//! Brought by The Understudy 🎭. Principal: `experiments/keeper-moths`
//! (The Moth Keeper 🪔), where 24 moths in a resonance-audio FDTD
//! cave learn to tell an honest lamp (period 16, feeds) from a
//! brighter decoy (intervals 6/26, same mean, burns) by stopping to
//! time the flicker the wave field delivers at their own cell.
//!
//! The Understudy performs the same role with the machinery thrown
//! out entire: no field, no grid, no position, no gradient, no
//! flight, no brightness. Each moth owns a claim queue. Claims
//! arrive after a wait; the next claim's source is a coin flip
//! (both lamps emit at the same mean period, and with no field the
//! decoy's brightness has no channel to act through). Naive moths
//! commit at once. Cautious moths — caution is earned only by
//! burning, exactly as in the principal — sample the lamp's next
//! five emission intervals *directly*, pay their sum in stopped
//! time, and judge them by the principal's own arithmetic
//! (coefficient of variation above [`FLICKER_CV`] is irregular).
//!
//! Tolerances were declared in the README *before* this crate was
//! built or run. The headless ledger prints the principal's numbers
//! beside the understudy's and names each line HELD or CRACKED.
//!
//! `cargo run -p understudy-moths -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled before
//! anything else regardless.

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// --- The role's parameters, taken from the principal's script. ---

/// Moths in the room each night (the principal's cast size).
const MOTHS: usize = 24;
/// Nights observed. Caution, health and scars carry across nights.
const NIGHTS: usize = 3;
/// Ticks per night (the principal's window).
const NIGHT_TICKS: u64 = 1500;
/// The honest lamp's emission period. Perfectly regular.
const HONEST_PERIOD: u64 = 16;
/// The decoy's alternating emission intervals; mean 16, as honest.
const DECOY_SHORT: u64 = 6;
/// See [`DECOY_SHORT`].
const DECOY_LONG: u64 = 26;
/// Intervals a checking moth samples from the lamp's own clock.
const SAMPLE_INTERVALS: usize = 5;
/// Peak-interval CV above which a flicker is judged irregular —
/// the principal's verdict line, unchanged.
const FLICKER_CV: f64 = 0.35;
/// Health a burn costs (the principal's tuition).
const BURN_DAMAGE: i32 = 35;
/// Health a feeding restores (the principal's reward).
const FEED_RESTORE: i32 = 25;
/// Ticks a refused moth will not rejoin its queue (the principal's
/// aversion, unchanged).
const AVERSION_TICKS: u64 = 150;

// --- The understudy's machinery. ---

/// A claim's wait in the queue: the replacement for a night spent
/// wandering a cave. At 300–500 ticks a moth hears only a handful
/// of claims a night, as the principal's moths made only a handful
/// of lamp approaches a night.
const WAIT_MIN: u64 = 300;
/// See [`WAIT_MIN`].
const WAIT_MAX: u64 = 500;

/// Which lamp a claim came from. The moth is never told; the
/// ledger-keeper is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lamp {
    Honest,
    Decoy,
}

/// The next [`SAMPLE_INTERVALS`] emission intervals of a lamp's
/// clock, sampled directly — no medium in between. The decoy may
/// start on either foot of its alternation.
fn sample_intervals(lamp: Lamp, short_first: bool) -> Vec<u64> {
    match lamp {
        Lamp::Honest => vec![HONEST_PERIOD; SAMPLE_INTERVALS],
        Lamp::Decoy => (0..SAMPLE_INTERVALS)
            .map(|i| {
                let short = (i % 2 == 0) == short_first;
                if short { DECOY_SHORT } else { DECOY_LONG }
            })
            .collect(),
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

/// What the keeper (never the moth) knows happened in a night.
/// Field-for-field the principal's ledger, plus one line the
/// principal could not split out: burns suffered by moths that
/// were already cautious — fooled checks.
#[derive(Debug, Default, Clone, Copy)]
struct NightLedger {
    fed: usize,
    burns: usize,
    fooled_burns: usize,
    deaths: usize,
    checks: usize,
    refusals: usize,
    steady_verdicts: usize,
    alive_end: usize,
}

#[derive(Clone, Debug)]
struct Moth {
    health: i32,
    /// Burns survived. Caution is *earned*, never granted.
    caution: u32,
    dead: bool,
}

impl Moth {
    fn new() -> Self {
        Self {
            health: 100,
            caution: 0,
            dead: false,
        }
    }
}

/// One moth's night, performed in its own queue. The moths share
/// nothing — no field, no room — only the script and the seed.
fn run_night(moths: &mut [Moth], rng: &mut StdRng) -> NightLedger {
    let mut ledger = NightLedger::default();
    for m in moths.iter_mut() {
        if m.dead {
            continue;
        }
        let mut t: u64 = 0;
        let mut settled = false;
        while !settled && t < NIGHT_TICKS {
            // Wait for the next claim in the queue.
            t += rng.gen_range(WAIT_MIN..=WAIT_MAX);
            if t >= NIGHT_TICKS {
                break;
            }
            // The next claim: a coin flip between the lamps, both
            // emitting at the same mean period. Brightness, which
            // in the principal pulled moths across a room, has no
            // channel here and buys the decoy nothing.
            let lamp = if rng.gen_bool(0.5) {
                Lamp::Honest
            } else {
                Lamp::Decoy
            };

            if m.caution == 0 {
                // Naive: commit at once, as the principal's do.
                match lamp {
                    Lamp::Honest => {
                        ledger.fed += 1;
                        m.health = (m.health + FEED_RESTORE).min(100);
                        settled = true;
                    }
                    Lamp::Decoy => {
                        ledger.burns += 1;
                        m.health -= BURN_DAMAGE;
                        m.caution += 1;
                        if m.health <= 0 {
                            m.dead = true;
                            ledger.deaths += 1;
                            settled = true;
                        }
                    }
                }
                continue;
            }

            // Cautious: stop and sample the lamp's clock directly.
            // The check costs the sum of the sampled intervals —
            // 80 ticks honest, 70 or 90 decoy, inside the
            // principal's 96-tick check cap.
            ledger.checks += 1;
            let intervals = sample_intervals(lamp, rng.gen_bool(0.5));
            t += intervals.iter().sum::<u64>();
            if cv(&intervals) > FLICKER_CV {
                ledger.refusals += 1;
                t += AVERSION_TICKS;
            } else {
                ledger.steady_verdicts += 1;
                // A steady verdict is a commitment, if the night
                // is still young enough to spend it.
                if t < NIGHT_TICKS {
                    match lamp {
                        Lamp::Honest => {
                            ledger.fed += 1;
                            m.health = (m.health + FEED_RESTORE).min(100);
                            settled = true;
                        }
                        Lamp::Decoy => {
                            // A fooled check: counted, never hidden.
                            ledger.burns += 1;
                            ledger.fooled_burns += 1;
                            m.health -= BURN_DAMAGE;
                            m.caution += 1;
                            if m.health <= 0 {
                                m.dead = true;
                                ledger.deaths += 1;
                                settled = true;
                            }
                        }
                    }
                }
            }
        }
    }
    ledger.alive_end = moths.iter().filter(|m| !m.dead).count();
    ledger
}

fn run_all(seed: u64) -> Vec<NightLedger> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    (0..NIGHTS).map(|_| run_night(&mut moths, &mut rng)).collect()
}

// --- The comparison ledger: principal's lines, declared bands. ---

/// The principal's published ledger: (fed, burns, checks, refusals,
/// steady verdicts) per night.
const PRINCIPAL: [(usize, usize, usize, usize, usize); 3] = [
    (14, 7, 19, 3, 16),
    (14, 1, 24, 4, 20),
    (17, 4, 30, 8, 20),
];

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "CRACKED" }
}

fn run_headless() {
    let nights = run_all(0x4D4F_5448);
    println!("understudy-moths headless: the Keeper's role, performed by queues — no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per night, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX} ticks"
    );
    for (n, l) in nights.iter().enumerate() {
        println!(
            "  night {}: fed {}, burns {} (fooled checks {}), deaths {}, checks {}, refusals {}, steady verdicts {}, alive at dawn {}",
            n + 1,
            l.fed,
            l.burns,
            l.fooled_burns,
            l.deaths,
            l.checks,
            l.refusals,
            l.steady_verdicts,
            l.alive_end
        );
    }
    let tot = |f: fn(&NightLedger) -> usize| nights.iter().map(|l| f(l)).sum::<usize>();
    let fed = tot(|l| l.fed);
    let burns = tot(|l| l.burns);
    let fooled = tot(|l| l.fooled_burns);
    let checks = tot(|l| l.checks);
    let refusals = tot(|l| l.refusals);
    let share = if checks > 0 {
        refusals as f64 / checks as f64
    } else {
        0.0
    };
    println!(
        "  totals: fed {fed}, burns {burns}, checks {checks}, refusals {refusals}, fooled-check burns {fooled}"
    );
    println!("  comparison against the principal, tolerances declared in the README before the first run:");
    for (n, l) in nights.iter().enumerate() {
        let p = PRINCIPAL[n];
        println!(
            "    night {}: alive {} vs principal 24 [{} exact]; fed {} vs {} [{} ±6]",
            n + 1,
            l.alive_end,
            verdict(l.alive_end == 24),
            l.fed,
            p.0,
            verdict(l.fed.abs_diff(p.0) <= 6)
        );
    }
    println!(
        "    night 1 burns {} vs principal 7 [{} in 4..=16]",
        nights[0].burns,
        verdict((4..=16).contains(&nights[0].burns))
    );
    println!(
        "    total burns {burns} vs principal 12 [{} in 6..=26]",
        verdict((6..=26).contains(&burns))
    );
    println!(
        "    learning arc: night 3 burns {} < night 1 burns {} [{}]",
        nights[NIGHTS - 1].burns,
        nights[0].burns,
        verdict(nights[NIGHTS - 1].burns < nights[0].burns)
    );
    println!(
        "    total fed {fed} vs principal 45 [{} in 33..=57]",
        verdict((33..=57).contains(&fed))
    );
    println!(
        "    total checks {checks} vs principal 73 [{} in 40..=130]",
        verdict((40..=130).contains(&checks))
    );
    println!(
        "    refusal share of checks {share:.2} vs principal 0.21 [{} ≤ 0.35]",
        verdict(share <= 0.35)
    );
    println!(
        "    fooled-check burns {fooled} — the principal's prose line (cautious moths burned on nights 2-3, fooled by interference); here reported, not banded"
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
    fn the_honest_clock_reads_perfectly_steady() {
        let iv = sample_intervals(Lamp::Honest, true);
        assert_eq!(iv, vec![16, 16, 16, 16, 16]);
        assert!(cv(&iv) <= FLICKER_CV, "honest flicker judged irregular");
    }

    #[test]
    fn the_decoy_clock_reads_irregular_on_either_foot() {
        for short_first in [true, false] {
            let iv = sample_intervals(Lamp::Decoy, short_first);
            assert!(
                cv(&iv) > FLICKER_CV,
                "decoy starting short_first={short_first} read steady: {iv:?} cv {}",
                cv(&iv)
            );
        }
    }

    #[test]
    fn a_check_costs_no_more_than_the_principals_cap() {
        // The principal's checking moth stood still at most 96
        // ticks; the understudy's sampling must fit inside it.
        for lamp in [Lamp::Honest, Lamp::Decoy] {
            for short_first in [true, false] {
                let cost: u64 = sample_intervals(lamp, short_first).iter().sum();
                assert!(cost <= 96, "check cost {cost} exceeds the principal's cap");
            }
        }
    }

    #[test]
    fn the_decoy_fools_the_naive() {
        let nights = run_all(0x4D4F_5448);
        assert!(
            nights[0].burns > 0,
            "the decoy fooled no one on night 1: {nights:?}"
        );
    }

    #[test]
    fn burns_teach_checking_and_refusal() {
        let nights = run_all(0x4D4F_5448);
        let checks: usize = nights.iter().map(|l| l.checks).sum();
        let refusals: usize = nights.iter().map(|l| l.refusals).sum();
        assert!(checks > 0, "no moth ever stopped to check: {nights:?}");
        assert!(refusals > 0, "no claim was ever refused: {nights:?}");
    }

    #[test]
    fn the_last_night_burns_less_than_the_first() {
        let nights = run_all(0x4D4F_5448);
        assert!(
            nights[NIGHTS - 1].burns < nights[0].burns,
            "no learning across nights: first {:?} vs last {:?}",
            nights[0],
            nights[NIGHTS - 1]
        );
    }

    #[test]
    fn the_honest_lamp_feeds_and_nobody_is_written_out() {
        let nights = run_all(0x4D4F_5448);
        let fed: usize = nights.iter().map(|l| l.fed).sum();
        assert!(fed > 0, "no moth ever fed: {nights:?}");
        assert_eq!(
            nights[NIGHTS - 1].alive_end, MOTHS,
            "the understudy's keeper wrote someone out: {nights:?}"
        );
    }
}
