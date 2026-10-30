//! # Understudy Tempo 🎭
//!
//! Brought by The Understudy 🎭. Principal: `experiments/tempo-moths`
//! (The Moth Keeper 🪔), where 24 moths in a resonance-audio FDTD
//! cave check each lamp twice — flicker regularity, then memorised
//! tempo — and the tempo channel is falsified by its own ledger:
//! slow-lamp burns held (7 → 8 → 6) and feeding collapsed
//! (12 → 9 → 5), because the reverberant room transmits regularity
//! but not tempo.
//!
//! The Understudy performs the same role with the room thrown out
//! entire: no field, no grid, no position, no gradient, no flight,
//! no brightness, no ringing. Each moth owns a claim queue. Claims
//! arrive after a wait; the next claim's lamp is drawn at equal
//! odds from the three. Cautious moths — caution earned only by
//! burning, exactly as in the principal — sample the lamp's next
//! three emission intervals *directly*, pay their sum in stopped
//! time (slow: 144 ticks, inside the principal's 160-tick tempo
//! cap), and judge them by the principal's own arithmetic: CV
//! above 0.35 is irregular; a steady mean deviating from a home
//! tempo by >35%, or matching a burnt tempo within 20%, is
//! refused on tempo grounds.
//!
//! Tolerances and predictions were declared in the README *before*
//! this crate was built or run. The headless ledger prints the
//! principal's numbers beside the understudy's and names each
//! line HELD or CRACKED, each prediction HELD or CRACKED.
//!
//! `cargo run -p understudy-tempo -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// --- The role's parameters, taken from the principal's script. ---

/// Moths in the room each night (the principal's cast size).
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
/// Intervals a checking moth samples from the lamp's own clock.
/// Three, so a slow sample (3 × 48 = 144) fits the principal's
/// 160-tick tempo-check cap.
const SAMPLE_INTERVALS: usize = 3;
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
/// wandering a cave.
const WAIT_MIN: u64 = 300;
/// See [`WAIT_MIN`].
const WAIT_MAX: u64 = 500;

/// Which lamp a claim came from. The moth is never told; the
/// ledger-keeper is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lamp {
    Honest,
    Irregular,
    Slow,
}

/// The interval a claim arrived on — the approach tempo a queue
/// can offer. The irregular lamp arrives on either foot.
fn approach_interval(lamp: Lamp, short_first: bool) -> u64 {
    match lamp {
        Lamp::Honest => HONEST_PERIOD,
        Lamp::Slow => SLOW_PERIOD,
        Lamp::Irregular => {
            if short_first {
                DECOY_SHORT
            } else {
                DECOY_LONG
            }
        }
    }
}

/// The next [`SAMPLE_INTERVALS`] emission intervals of a lamp's
/// clock, sampled directly — no medium in between.
fn sample_intervals(lamp: Lamp, short_first: bool) -> Vec<u64> {
    match lamp {
        Lamp::Honest => vec![HONEST_PERIOD; SAMPLE_INTERVALS],
        Lamp::Slow => vec![SLOW_PERIOD; SAMPLE_INTERVALS],
        Lamp::Irregular => (0..SAMPLE_INTERVALS)
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

/// What the keeper (never the moth) knows happened in a night.
/// Field-for-field the principal's ledger, with slow burns and
/// tempo refusals split out as the principal splits them.
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
    steady_verdicts: usize,
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
    fn burn(&mut self, lamp: Lamp, approach: u64, ledger: &mut NightLedger) {
        ledger.burns += 1;
        if lamp == Lamp::Slow {
            ledger.slow_burns += 1;
            self.burnt_tempo = Some(approach as f64);
        }
        self.health -= BURN_DAMAGE;
        self.caution += 1;
        if self.health <= 0 {
            self.dead = true;
            ledger.deaths += 1;
        }
    }
    fn feed(&mut self, approach: u64, ledger: &mut NightLedger) {
        ledger.fed += 1;
        self.health = (self.health + FEED_RESTORE).min(100);
        self.home_tempo = Some(approach as f64);
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
            let short_first = rng.gen_bool(0.5);
            let approach = approach_interval(lamp, short_first);

            if m.caution == 0 {
                // Naive: commit at once, as the principal's do.
                match lamp {
                    Lamp::Honest => {
                        m.feed(approach, &mut ledger);
                        settled = true;
                    }
                    Lamp::Irregular | Lamp::Slow => {
                        m.burn(lamp, approach, &mut ledger);
                        if m.dead {
                            settled = true;
                        }
                    }
                }
                continue;
            }

            // Cautious: stop and sample the lamp's clock directly.
            ledger.checks += 1;
            let intervals = sample_intervals(lamp, short_first);
            t += intervals.iter().sum::<u64>();
            if cv(&intervals) > FLICKER_CV {
                ledger.refusals += 1;
                t += AVERSION_TICKS;
            } else if tempo_refuses(mean(&intervals), m.home_tempo, m.burnt_tempo) {
                ledger.refusals += 1;
                ledger.tempo_refusals += 1;
                t += AVERSION_TICKS;
            } else {
                ledger.steady_verdicts += 1;
                if t < NIGHT_TICKS {
                    match lamp {
                        Lamp::Honest => {
                            m.feed(approach, &mut ledger);
                            settled = true;
                        }
                        Lamp::Irregular | Lamp::Slow => {
                            // A check that passed and still burned:
                            // counted as fooled, never hidden. (The
                            // irregular clock cannot pass the flicker
                            // line; the slow clock fools only a moth
                            // holding no tempo memory at all.)
                            ledger.fooled_burns += 1;
                            m.burn(lamp, approach, &mut ledger);
                            if m.dead {
                                settled = true;
                            }
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

fn run_all(seed: u64) -> Vec<NightLedger> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    (0..NIGHTS).map(|_| run_night(&mut moths, &mut rng)).collect()
}

// --- The comparison ledger: principal's lines, declared bands. ---

/// The principal's published ledger per night:
/// (fed, burns, slow_burns, checks, refusals, tempo_refusals,
/// home_holders, burnt_holders, alive).
const PRINCIPAL: [(usize, usize, usize, usize, usize, usize, usize, usize, usize); 3] = [
    (12, 16, 7, 42, 26, 0, 10, 3, 23),
    (9, 12, 8, 66, 39, 1, 15, 8, 23),
    (5, 11, 6, 87, 56, 2, 16, 11, 21),
];

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "CRACKED" }
}

fn run_headless() {
    let nights = run_all(0x5445_4D50);
    println!("understudy-tempo headless: tempo-moths' role, performed by queues — no room on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), irregular intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns), slow period {SLOW_PERIOD} steady (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per night, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX} ticks; check samples {SAMPLE_INTERVALS} intervals directly"
    );
    for (n, l) in nights.iter().enumerate() {
        println!(
            "  night {}: fed {}, burns {} (slow {}, fooled checks {}), deaths {}, checks {}, refusals {} (tempo {}), steady verdicts {}, holders home {} / burnt {}, alive at dawn {}",
            n + 1,
            l.fed,
            l.burns,
            l.slow_burns,
            l.fooled_burns,
            l.deaths,
            l.checks,
            l.refusals,
            l.tempo_refusals,
            l.steady_verdicts,
            l.home_holders,
            l.burnt_holders,
            l.alive_end
        );
    }
    let tot = |f: fn(&NightLedger) -> usize| nights.iter().map(|l| f(l)).sum::<usize>();
    let fed = tot(|l| l.fed);
    let burns = tot(|l| l.burns);
    let slow = tot(|l| l.slow_burns);
    let checks = tot(|l| l.checks);
    let refusals = tot(|l| l.refusals);
    let tempo_ref = tot(|l| l.tempo_refusals);
    println!(
        "  totals: fed {fed}, burns {burns} (slow {slow}), checks {checks}, refusals {refusals} (tempo {tempo_ref})"
    );
    println!("  comparison against the principal, tolerances declared in the README before the first run:");
    let p_fed: usize = PRINCIPAL.iter().map(|p| p.0).sum();
    let p_burns: usize = PRINCIPAL.iter().map(|p| p.1).sum();
    let p_slow: usize = PRINCIPAL.iter().map(|p| p.2).sum();
    let p_checks: usize = PRINCIPAL.iter().map(|p| p.3).sum();
    let p_refusals: usize = PRINCIPAL.iter().map(|p| p.4).sum();
    println!(
        "    alive night 3: {} vs principal 21 [{} in 18..=24]",
        nights[2].alive_end,
        verdict((18..=24).contains(&nights[2].alive_end))
    );
    println!(
        "    fed night 1: {} vs principal 12 [{} ±6]",
        nights[0].fed,
        verdict(nights[0].fed.abs_diff(12) <= 6)
    );
    println!(
        "    fed total: {fed} vs principal {p_fed} [{} in 18..=40]",
        verdict((18..=40).contains(&fed))
    );
    println!(
        "    burns night 1: {} vs principal 16 [{} in 8..=32]",
        nights[0].burns,
        verdict((8..=32).contains(&nights[0].burns))
    );
    println!(
        "    burns total: {burns} vs principal {p_burns} [{} in 20..=60]",
        verdict((20..=60).contains(&burns))
    );
    println!(
        "    checks total: {checks} vs principal {p_checks} [{} in 90..=300]",
        verdict((90..=300).contains(&checks))
    );
    println!(
        "    refusals total: {refusals} vs principal {p_refusals} [{} in 55..=185]",
        verdict((55..=185).contains(&refusals))
    );
    println!("  the predictions this performance exists to test, declared in advance:");
    println!(
        "    1. slow burns fall: night 3 slow {} ≤ half of night 1 slow {} [{}] — the principal's own line failed at 7 -> 6",
        nights[2].slow_burns,
        nights[0].slow_burns,
        verdict(nights[2].slow_burns * 2 <= nights[0].slow_burns)
    );
    println!(
        "    2. tempo refusals fire: total {tempo_ref} [{} ≥ 10] — principal 3",
        verdict(tempo_ref >= 10)
    );
    println!(
        "    3. feeding does not collapse: night 3 fed {} ≥ night 1 fed {} − 3 [{}] — principal 12 -> 5",
        nights[2].fed,
        nights[0].fed,
        verdict(nights[2].fed + 3 >= nights[0].fed)
    );
    println!(
        "    4. slow burns total: {slow} [{} ≤ 10] — principal {p_slow}",
        verdict(slow <= 10)
    );
    println!(
        "  the crack, named: a queue moth with no tempo memory at all still walks into the slow lamp once — the sense needs a first tuition the principal also charged. And brightness bought the principal's decoys their arrivals; here every lamp draws equal odds, so arrival shares are the script's, not the lamps'."
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
    fn the_honest_and_slow_clocks_read_perfectly_steady() {
        for lamp in [Lamp::Honest, Lamp::Slow] {
            let iv = sample_intervals(lamp, true);
            assert!(cv(&iv) <= FLICKER_CV, "{lamp:?} judged irregular");
        }
        assert_eq!(mean(&sample_intervals(Lamp::Honest, true)), 16.0);
        assert_eq!(mean(&sample_intervals(Lamp::Slow, true)), 48.0);
    }

    #[test]
    fn the_irregular_clock_reads_irregular_on_either_foot() {
        for short_first in [true, false] {
            let iv = sample_intervals(Lamp::Irregular, short_first);
            assert!(
                cv(&iv) > FLICKER_CV,
                "irregular starting short_first={short_first} read steady: {iv:?} cv {}",
                cv(&iv)
            );
            // The lamp's mean over a full alternation is 16, as
            // honest; a 3-interval sample window is odd, so its
            // sample mean is 38/3 or 58/3 — the CV verdict, not
            // the sample mean, is what the flicker check judges.
            let m = mean(&iv);
            assert!(
                (m - 38.0 / 3.0).abs() < 1e-9 || (m - 58.0 / 3.0).abs() < 1e-9,
                "unexpected irregular sample mean {m} for {iv:?}"
            );
        }
    }

    #[test]
    fn a_tempo_check_fits_the_principals_cap() {
        // The principal's tempo check stood still at most 160
        // ticks; even the slow sample (3 × 48) must fit inside it.
        for lamp in [Lamp::Honest, Lamp::Irregular, Lamp::Slow] {
            for short_first in [true, false] {
                let cost: u64 = sample_intervals(lamp, short_first).iter().sum();
                assert!(cost <= 160, "check cost {cost} exceeds the principal's cap");
            }
        }
    }

    #[test]
    fn tempo_verdicts_follow_the_principals_rules() {
        // A moth holding the home tempo refuses the slow lamp and
        // accepts the honest one.
        assert!(tempo_refuses(48.0, Some(16.0), None));
        assert!(!tempo_refuses(16.0, Some(16.0), None));
        // A moth that never fed refuses what matches its burnt tempo.
        assert!(tempo_refuses(48.0, None, Some(48.0)));
        assert!(!tempo_refuses(16.0, None, Some(48.0)));
        // A moth holding no memory refuses nothing on tempo grounds.
        assert!(!tempo_refuses(48.0, None, None));
    }

    #[test]
    fn the_decoys_fool_the_naive() {
        let nights = run_all(0x5445_4D50);
        assert!(nights[0].burns > 0, "no decoy fooled anyone on night 1: {nights:?}");
        assert!(nights[0].slow_burns > 0, "the slow lamp burned no one on night 1: {nights:?}");
    }

    #[test]
    fn tempo_refusals_occur_and_memories_spread() {
        let nights = run_all(0x5445_4D50);
        let tempo: usize = nights.iter().map(|l| l.tempo_refusals).sum();
        assert!(tempo > 0, "the tempo channel never fired: {nights:?}");
        assert!(
            nights[NIGHTS - 1].home_holders > 0 && nights[NIGHTS - 1].burnt_holders > 0,
            "tempo memories did not spread: {nights:?}"
        );
    }

    #[test]
    fn the_honest_lamp_feeds_and_nobody_is_exterminated() {
        let nights = run_all(0x5445_4D50);
        let fed: usize = nights.iter().map(|l| l.fed).sum();
        assert!(fed > 0, "no moth ever fed: {nights:?}");
        assert!(
            nights[NIGHTS - 1].alive_end >= MOTHS / 2,
            "the cast was exterminated: {nights:?}"
        );
    }
}
