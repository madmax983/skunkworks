//! # Scarecrow Journal 🌾
//!
//! Brought by The Scarecrow 🌾. Principle: **journal and replay**,
//! learned from Harvest — work that is written down as it happens
//! can be replayed after the winter and must come back the same.
//! Work that lives only in memory does not survive.
//!
//! Host: `experiments/understudy-tempo`'s queue moths (The
//! Understudy 🎭 performing The Moth Keeper's 🪔 role). Twenty-four
//! moths, three nights, claims drawn from a seeded RNG stream. The
//! host's whole history — who burned, who learned caution, whose
//! home tempo was earned, and *where the RNG stream stands* —
//! exists only in memory. Nothing is written down. That is the
//! fragile point, and it is the row I planted in.
//!
//! The seed: the same script, the same seed, run three ways.
//!
//! 1. **Live.** Every draw the run makes (a claim's wait, its
//!    lamp, its coin) is written to a journal, in order, as it
//!    happens. Outcomes are *not* journaled — only the draws.
//! 2. **Full replay.** A second pass runs the same engine with
//!    the RNG thrown away: its only source of draws is the
//!    journal. If the journal is the whole field, the replay
//!    reproduces the live ledger field for field and every moth
//!    state exactly.
//! 3. **Torn replay.** The journal is cut in half (the winter
//!    tore the book). The replay runs until the paper ends and
//!    stops there: the divergence point is named by event,
//!    night, and moth, and the partial ledger is printed beside
//!    the live one. What was written survives; what was not,
//!    does not. Nothing after the tear is invented.
//!
//! The comparison and tolerances were declared in the README
//! before the first run. `cargo run -p scarecrow-journal --
//! --headless` prints both sides and the verdicts. --headless is
//! handled before anything else; there is no TUI.

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// --- The host's script, taken from understudy-tempo verbatim. ---

const MOTHS: usize = 24;
const NIGHTS: usize = 3;
const NIGHT_TICKS: u64 = 1500;
const HONEST_PERIOD: u64 = 16;
const DECOY_SHORT: u64 = 6;
const DECOY_LONG: u64 = 26;
const SLOW_PERIOD: u64 = 48;
const SAMPLE_INTERVALS: usize = 3;
const FLICKER_CV: f64 = 0.35;
const TEMPO_DEVIATION: f64 = 0.35;
const BURNT_MATCH: f64 = 0.20;
const BURN_DAMAGE: i32 = 35;
const FEED_RESTORE: i32 = 25;
const AVERSION_TICKS: u64 = 150;
const WAIT_MIN: u64 = 300;
const WAIT_MAX: u64 = 500;
/// The host's own seed ("TEMP"), unchanged: the live run must
/// reproduce the host's published ledger before any replay is
/// judged.
const SEED: u64 = 0x5445_4D50;

/// The host's published ledger totals (understudy-tempo README):
/// fed 14/12/14, burns 18/7/2, slow 7/4/2, checks 107 total,
/// refusals 71 of which tempo 30, alive 24/24/24.
const HOST_TOTALS: (usize, usize, usize, usize, usize, usize) = (40, 27, 13, 107, 71, 30);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lamp {
    Honest,
    Irregular,
    Slow,
}

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

fn tempo_refuses(sample_mean: f64, home: Option<f64>, burnt: Option<f64>) -> bool {
    if let Some(h) = home {
        (sample_mean - h).abs() / h > TEMPO_DEVIATION
    } else if let Some(b) = burnt {
        (sample_mean - b).abs() / b <= BURNT_MATCH
    } else {
        false
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq)]
struct Moth {
    health: i32,
    caution: u32,
    home_tempo: Option<f64>,
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

// --- The journal: every draw, written down as it happens. ---

/// One draw from the run's stream, in the order the engine asks
/// for it. A claim is three draws: its wait, its lamp, its coin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Draw {
    Wait(u64),
    Lamp(u8),
    Coin(bool),
}

impl Draw {
    /// The written form: one short line per draw. This is the
    /// paper the winter has to survive on.
    fn to_line(self) -> String {
        match self {
            Draw::Wait(w) => format!("W {w}"),
            Draw::Lamp(l) => format!("L {l}"),
            Draw::Coin(c) => format!("S {}", if c { 1 } else { 0 }),
        }
    }
    fn from_line(line: &str) -> Option<Draw> {
        let (kind, val) = line.split_once(' ')?;
        match kind {
            "W" => val.parse().ok().map(Draw::Wait),
            "L" => val.parse().ok().map(Draw::Lamp),
            "S" => Some(Draw::Coin(val == "1")),
            _ => None,
        }
    }
}

fn journal_to_text(journal: &[Draw]) -> String {
    journal
        .iter()
        .map(|d| d.to_line())
        .collect::<Vec<_>>()
        .join("\n")
}

fn journal_from_text(text: &str) -> Vec<Draw> {
    text.lines().filter_map(Draw::from_line).collect()
}

/// Where a replay's paper ran out (or tore): the global event
/// index it failed at, and whose claim it was in the middle of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Divergence {
    event_index: usize,
    night: usize,
    moth: usize,
}

/// A source of draws. The live source rolls the seeded RNG and
/// writes every draw to the journal. A replay source has no RNG
/// at all — it reads the paper, and the paper is all it has.
trait DrawSource {
    fn wait(&mut self) -> Option<u64>;
    fn lamp(&mut self) -> Option<u8>;
    fn coin(&mut self) -> Option<bool>;
}

struct LiveSource {
    rng: StdRng,
    journal: Vec<Draw>,
}

impl DrawSource for LiveSource {
    fn wait(&mut self) -> Option<u64> {
        let w = self.rng.gen_range(WAIT_MIN..=WAIT_MAX);
        self.journal.push(Draw::Wait(w));
        Some(w)
    }
    fn lamp(&mut self) -> Option<u8> {
        // The host draws `gen_range(0..3)` with the integer type
        // inferred as i32, and StdRng samples each width
        // differently — the draw's width is part of the stream's
        // state, so the journal's live source draws i32 exactly
        // as the host does and records the value.
        let l = self.rng.gen_range(0..3_i32);
        self.journal.push(Draw::Lamp(l as u8));
        Some(l as u8)
    }
    fn coin(&mut self) -> Option<bool> {
        let c = self.rng.gen_bool(0.5);
        self.journal.push(Draw::Coin(c));
        Some(c)
    }
}

struct ReplaySource {
    draws: Vec<Draw>,
    pos: usize,
}

impl ReplaySource {
    fn next(&mut self) -> Option<Draw> {
        let d = self.draws.get(self.pos).copied();
        if d.is_some() {
            self.pos += 1;
        }
        d
    }
}

impl DrawSource for ReplaySource {
    fn wait(&mut self) -> Option<u64> {
        match self.next() {
            Some(Draw::Wait(w)) => Some(w),
            _ => None,
        }
    }
    fn lamp(&mut self) -> Option<u8> {
        match self.next() {
            Some(Draw::Lamp(l)) => Some(l),
            _ => None,
        }
    }
    fn coin(&mut self) -> Option<bool> {
        match self.next() {
            Some(Draw::Coin(c)) => Some(c),
            _ => None,
        }
    }
}

// --- The engine: the host's night, drawing only from a source. ---

#[derive(Debug)]
struct RunOutcome {
    /// One ledger per night *started*; the last is partial if
    /// the source ran out mid-night.
    nights: Vec<NightLedger>,
    moths: Vec<Moth>,
    /// Draws consumed from the source.
    events_consumed: usize,
    /// Set when the source ran out before the run finished.
    divergence: Option<Divergence>,
}

fn run_all(source: &mut dyn DrawSource) -> RunOutcome {
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let mut nights: Vec<NightLedger> = Vec::new();
    let mut consumed_before = 0usize;
    // The engine cannot ask the source how many draws it took;
    // it counts claims itself via the divergence bookkeeping in
    // run_night. events_consumed is filled from the sources by
    // the callers' wrappers below.
    let _ = &mut consumed_before;
    for night in 0..NIGHTS {
        let mut ledger = NightLedger::default();
        let mut div: Option<Divergence> = None;
        'moths: for (mi, m) in moths.iter_mut().enumerate() {
            if m.dead {
                continue;
            }
            let mut t: u64 = 0;
            let mut settled = false;
            while !settled && t < NIGHT_TICKS {
                let Some(w) = source.wait() else {
                    div = Some(Divergence {
                        event_index: usize::MAX, // patched by caller wrappers
                        night,
                        moth: mi,
                    });
                    break 'moths;
                };
                t += w;
                if t >= NIGHT_TICKS {
                    break;
                }
                let Some(lamp_draw) = source.lamp() else {
                    div = Some(Divergence {
                        event_index: usize::MAX,
                        night,
                        moth: mi,
                    });
                    break 'moths;
                };
                let lamp = match lamp_draw {
                    0 => Lamp::Honest,
                    1 => Lamp::Irregular,
                    _ => Lamp::Slow,
                };
                let Some(short_first) = source.coin() else {
                    div = Some(Divergence {
                        event_index: usize::MAX,
                        night,
                        moth: mi,
                    });
                    break 'moths;
                };
                let approach = approach_interval(lamp, short_first);

                if m.caution == 0 {
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
        nights.push(ledger);
        if let Some(d) = div {
            return RunOutcome {
                nights,
                moths,
                events_consumed: 0,
                divergence: Some(d),
            };
        }
    }
    RunOutcome {
        nights,
        moths,
        events_consumed: 0,
        divergence: None,
    }
}

fn run_live() -> (RunOutcome, Vec<Draw>, String) {
    let mut src = LiveSource {
        rng: StdRng::seed_from_u64(SEED),
        journal: Vec::new(),
    };
    let mut out = run_all(&mut src);
    out.events_consumed = src.journal.len();
    let text = journal_to_text(&src.journal);
    (out, src.journal, text)
}

fn run_replay(journal: &[Draw]) -> RunOutcome {
    let mut src = ReplaySource {
        draws: journal.to_vec(),
        pos: 0,
    };
    let mut out = run_all(&mut src);
    out.events_consumed = src.pos;
    if let Some(d) = out.divergence.as_mut() {
        // The event the paper failed at is the first unconsumed
        // index: everything before it was read, this one was not
        // there (or was the wrong kind of draw).
        d.event_index = src.pos;
    }
    out
}

fn totals(nights: &[NightLedger]) -> (usize, usize, usize, usize, usize, usize) {
    let t = |f: fn(&NightLedger) -> usize| nights.iter().map(|l| f(l)).sum::<usize>();
    (
        t(|l| l.fed),
        t(|l| l.burns),
        t(|l| l.slow_burns),
        t(|l| l.checks),
        t(|l| l.refusals),
        t(|l| l.tempo_refusals),
    )
}

fn differing_fields(a: &[NightLedger], b: &[NightLedger]) -> usize {
    let mut n = 0usize;
    for (x, y) in a.iter().zip(b.iter()) {
        n += (x.fed != y.fed) as usize
            + (x.burns != y.burns) as usize
            + (x.slow_burns != y.slow_burns) as usize
            + (x.fooled_burns != y.fooled_burns) as usize
            + (x.deaths != y.deaths) as usize
            + (x.checks != y.checks) as usize
            + (x.refusals != y.refusals) as usize
            + (x.tempo_refusals != y.tempo_refusals) as usize
            + (x.steady_verdicts != y.steady_verdicts) as usize
            + (x.home_holders != y.home_holders) as usize
            + (x.burnt_holders != y.burnt_holders) as usize
            + (x.alive_end != y.alive_end) as usize;
    }
    n + 12 * a.len().abs_diff(b.len())
}

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "DIVERGED" }
}

fn print_night(tag: &str, n: usize, l: &NightLedger) {
    println!(
        "  {tag} night {}: fed {}, burns {} (slow {}, fooled checks {}), deaths {}, checks {}, refusals {} (tempo {}), steady verdicts {}, holders home {} / burnt {}, alive at dawn {}",
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

fn run_headless() {
    let (live, journal, text) = run_live();
    let waits = journal.iter().filter(|d| matches!(d, Draw::Wait(_))).count();
    let lamps = journal.iter().filter(|d| matches!(d, Draw::Lamp(_))).count();
    let coins = journal.iter().filter(|d| matches!(d, Draw::Coin(_))).count();

    println!("scarecrow-journal headless: understudy-tempo's moths, journaled — the book, the replay, the tear");
    println!(
        "  host: {MOTHS} moths, {NIGHTS} nights x {NIGHT_TICKS} ticks, seed {SEED:#x}; lamps honest {HONEST_PERIOD} / irregular {DECOY_SHORT}/{DECOY_LONG} / slow {SLOW_PERIOD}; every draw written down as it happens, outcomes never journaled"
    );
    for (n, l) in live.nights.iter().enumerate() {
        print_night("live", n, l);
    }
    let lt = totals(&live.nights);
    println!(
        "  live totals: fed {}, burns {} (slow {}), checks {}, refusals {} (tempo {})",
        lt.0, lt.1, lt.2, lt.3, lt.4, lt.5
    );
    println!(
        "  host check: live totals vs the host's published totals (fed {}, burns {}, slow {}, checks {}, refusals {}, tempo {}) [{} — exact, declared]",
        HOST_TOTALS.0,
        HOST_TOTALS.1,
        HOST_TOTALS.2,
        HOST_TOTALS.3,
        HOST_TOTALS.4,
        HOST_TOTALS.5,
        verdict(lt == HOST_TOTALS)
    );
    println!(
        "  journal: {} events ({} waits, {} lamps, {} coins), {} bytes of paper, {} claims",
        journal.len(),
        waits,
        lamps,
        coins,
        text.len(),
        lamps
    );

    // Full replay: no RNG, and not even the in-memory journal —
    // the replay reads the journal back from its written text,
    // so what is tested is the paper, not a memory of the paper.
    let paper = journal_from_text(&text);
    let replay = run_replay(&paper);
    let field_diffs = differing_fields(&live.nights, &replay.nights);
    let moth_diffs = live
        .moths
        .iter()
        .zip(replay.moths.iter())
        .filter(|(a, b)| a != b)
        .count();
    println!(
        "  full replay: completed {}, events consumed {} of {}, night-ledger fields differing {}, moths differing {} of {}",
        replay.divergence.is_none(),
        replay.events_consumed,
        journal.len(),
        field_diffs,
        moth_diffs,
        MOTHS
    );
    println!(
        "  full replay verdict: [{} — exact, declared: zero differing fields, zero differing moths]",
        verdict(replay.divergence.is_none() && field_diffs == 0 && moth_diffs == 0)
    );

    // The torn book: first half of the paper, nothing more.
    let cut = paper.len() / 2;
    let torn = run_replay(&paper[..cut]);
    let tt = totals(&torn.nights);
    if let Some(d) = torn.divergence {
        println!(
            "  torn replay: paper cut at event {} of {}; replay stopped at event {} — night {}, moth {} — and invented nothing after it",
            cut,
            journal.len(),
            d.event_index,
            d.night + 1,
            d.moth + 1
        );
    }
    println!(
        "  torn totals: fed {}, burns {} (slow {}), checks {}, refusals {} (tempo {}) — against live fed {}, burns {}, checks {}",
        tt.0, tt.1, tt.2, tt.3, tt.4, tt.5, lt.0, lt.1, lt.3
    );
    println!(
        "  torn verdict: [{} — declared: the torn replay must NOT complete, and its first divergence is the cut itself]",
        verdict(
            torn.divergence.is_some()
                && torn.divergence.map(|d| d.event_index) == Some(cut)
                && tt != lt
        )
    );
    println!(
        "  the row, read plainly: the journal held {} draws and no outcomes, and the replay rebuilt {} fed, {} burns and {} tempo refusals from paper alone. The stream's position was state, and the paper carried it. Cut the paper and the run ends mid-claim, exactly where the writing ends.",
        journal.len(), lt.0, lt.1, lt.5
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
    fn the_live_run_reproduces_the_hosts_published_ledger() {
        let (live, _, _) = run_live();
        assert_eq!(totals(&live.nights), HOST_TOTALS, "live run drifted from the host");
        assert_eq!(live.nights.len(), NIGHTS);
        assert!(live.divergence.is_none());
    }

    #[test]
    fn the_full_replay_reproduces_the_live_run_exactly() {
        let (live, journal, _) = run_live();
        let replay = run_replay(&journal);
        assert!(replay.divergence.is_none(), "replay ran out of paper: {:?}", replay.divergence);
        assert_eq!(replay.events_consumed, journal.len(), "replay left paper unread");
        assert_eq!(differing_fields(&live.nights, &replay.nights), 0);
        assert_eq!(live.moths, replay.moths, "a moth came back different");
    }

    #[test]
    fn the_journal_round_trips_through_its_written_form() {
        let (_, journal, text) = run_live();
        assert_eq!(journal_from_text(&text), journal);
        assert!(!text.is_empty());
    }

    #[test]
    fn without_the_journal_there_is_no_run_to_replay() {
        // Load-bearing check: an empty journal replays to an
        // immediate stop at event 0 and an empty first night —
        // the paper is not decoration.
        let replay = run_replay(&[]);
        let d = replay.divergence.expect("empty journal must diverge");
        assert_eq!(d.event_index, 0);
        assert_eq!(totals(&replay.nights), (0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn the_torn_journal_diverges_exactly_at_the_tear() {
        let (live, journal, _) = run_live();
        let cut = journal.len() / 2;
        let torn = run_replay(&journal[..cut]);
        let d = torn.divergence.expect("torn journal must diverge");
        assert_eq!(d.event_index, cut, "divergence is not at the tear");
        assert_ne!(totals(&torn.nights), totals(&live.nights));
        // Everything the torn replay did count, it counted as the
        // live run did, up to the tear: its completed nights match.
        let full_nights = d.night;
        assert_eq!(
            differing_fields(&live.nights[..full_nights], &torn.nights[..full_nights]),
            0,
            "the surviving paper misremembers its own prefix"
        );
    }

    #[test]
    fn outcomes_are_not_in_the_journal_only_draws() {
        // The journal's alphabet is three letters; a fed or a burn
        // is nowhere written. The replay recomputes them.
        let (_, _, text) = run_live();
        for line in text.lines() {
            assert!(
                line.starts_with("W ") || line.starts_with("L ") || line.starts_with("S "),
                "unexpected journal line: {line}"
            );
        }
    }

    #[test]
    fn the_journal_records_claims_in_threes() {
        let (_, journal, _) = run_live();
        let waits = journal.iter().filter(|d| matches!(d, Draw::Wait(_))).count();
        let lamps = journal.iter().filter(|d| matches!(d, Draw::Lamp(_))).count();
        let coins = journal.iter().filter(|d| matches!(d, Draw::Coin(_))).count();
        assert_eq!(lamps, coins, "every claim that drew a lamp drew its coin");
        assert!(waits >= lamps, "waits also end nights without a claim");
        assert!(lamps > 0, "no claims were journaled");
    }
}
