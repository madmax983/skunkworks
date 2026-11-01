//! # Scarecrow Retention 🌾
//!
//! Brought by The Scarecrow 🌾. Principle: **retention on
//! purpose**, learned from Harvest — a history that is never
//! pruned grows without bound, and a history pruned carelessly
//! is a history lost. The craft is in what you keep.
//!
//! Host: the journaled moths of `experiments/scarecrow-journal`
//! (The Scarecrow's own earlier planting on
//! `experiments/understudy-tempo`'s queue moths: 24 moths, 3
//! nights, seed 0x5445_4D50). That run proved the full journal
//! replays exactly: 440 draws, 2,079 bytes of paper, no outcomes
//! written. It also left the book growing one line per draw,
//! forever. That growth is this run's fragile point, and it is
//! the row named in scarecrow-journal's own succession seed:
//! prune the journal at a stated policy and measure what replay
//! loses; compose a dawn checkpoint with the short journal that
//! remains.
//!
//! The policy, stated plainly: **at each dawn, write a checkpoint
//! of the moths (health, caution, both tempo memories, dead or
//! alive) and prune the journal to the draws since that dawn.**
//! What is retained at any moment is one checkpoint plus less
//! than one night of draws. Three passes judge it:
//!
//! 1. **Full journal (the control).** Replay the whole book from
//!    fresh moths. It must reproduce the live run exactly — the
//!    parent planting's verdict, re-earned here.
//! 2. **Naive prune (the careless cut).** Keep only the draws
//!    since the last dawn, throw the checkpoint away, replay
//!    from fresh moths. The surviving paper is real paper, read
//!    from the wrong beginning: the ledger it builds must
//!    diverge from the live one. This prices the checkpoint.
//! 3. **Checkpoint + short journal (the policy).** Restore the
//!    dawn checkpoint (read back from its written text), replay
//!    only the retained suffix. The final night and the final
//!    moths must match the live run exactly. This is journal and
//!    checkpoint composed: neither alone is the run, together
//!    they are.
//!
//! A fourth, small pass keeps the composition honest: the
//! checkpoint alone, with no suffix at all, must fail at its
//! first draw — the checkpoint is a photograph, not a film.
//!
//! Predictions were declared in the README before the first run.
//! `cargo run -p scarecrow-retention -- --headless` prints every
//! side and the verdicts. --headless is handled before anything
//! else; there is no TUI.

use anyhow::Result;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// --- The host's script, taken from scarecrow-journal verbatim. ---

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
const SEED: u64 = 0x5445_4D50;

/// The host's published ledger totals (understudy-tempo via
/// scarecrow-journal): fed 40, burns 27, slow 13, checks 107,
/// refusals 71, tempo 30.
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
// (The parent planting's paper, unchanged: W / L / S lines.)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Draw {
    Wait(u64),
    Lamp(u8),
    Coin(bool),
}

impl Draw {
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

// --- The checkpoint: the moths at a dawn, written down. ---

fn tempo_field(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x}"),
        None => "-".to_string(),
    }
}

fn checkpoint_to_text(moths: &[Moth]) -> String {
    moths
        .iter()
        .map(|m| {
            format!(
                "M {} {} {} {} {}",
                m.health,
                m.caution,
                tempo_field(m.home_tempo),
                tempo_field(m.burnt_tempo),
                if m.dead { 1 } else { 0 }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn checkpoint_from_text(text: &str) -> Option<Vec<Moth>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.split(' ').collect();
        if f.len() != 6 || f[0] != "M" {
            return None;
        }
        let parse_tempo = |s: &str| -> Option<Option<f64>> {
            if s == "-" {
                Some(None)
            } else {
                s.parse::<f64>().ok().map(Some)
            }
        };
        out.push(Moth {
            health: f[1].parse().ok()?,
            caution: f[2].parse().ok()?,
            home_tempo: parse_tempo(f[3])?,
            burnt_tempo: parse_tempo(f[4])?,
            dead: f[5] == "1",
        });
    }
    Some(out)
}

// --- Draw sources: live (rolls and writes) / replay (reads). ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Divergence {
    event_index: usize,
    night: usize,
    moth: usize,
}

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
        // The host draws `gen_range(0..3)` inferred as i32; the
        // draw's width is part of the stream's state (the parent
        // planting's strike), so the width is kept exactly.
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

// --- The engine: one night at a time, so dawns can be held. ---

fn run_one_night(
    source: &mut dyn DrawSource,
    moths: &mut [Moth],
    night: usize,
) -> (NightLedger, Option<Divergence>) {
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
                div = Some(Divergence { event_index: usize::MAX, night, moth: mi });
                break 'moths;
            };
            t += w;
            if t >= NIGHT_TICKS {
                break;
            }
            let Some(lamp_draw) = source.lamp() else {
                div = Some(Divergence { event_index: usize::MAX, night, moth: mi });
                break 'moths;
            };
            let lamp = match lamp_draw {
                0 => Lamp::Honest,
                1 => Lamp::Irregular,
                _ => Lamp::Slow,
            };
            let Some(short_first) = source.coin() else {
                div = Some(Divergence { event_index: usize::MAX, night, moth: mi });
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
    (ledger, div)
}

struct FullRun {
    nights: Vec<NightLedger>,
    moths: Vec<Moth>,
    divergence: Option<Divergence>,
    events_consumed: usize,
}

fn run_nights(source: &mut dyn DrawSource, moths: &mut Vec<Moth>, count: usize) -> FullRun {
    let mut nights = Vec::new();
    let mut divergence = None;
    for night in 0..count {
        let (ledger, div) = run_one_night(source, moths, night);
        nights.push(ledger);
        if div.is_some() {
            divergence = div;
            break;
        }
    }
    FullRun { nights, moths: moths.clone(), divergence, events_consumed: 0 }
}

/// The live run, held at every dawn: after each night, the
/// journal's length and a checkpoint of the moths.
struct LiveRun {
    outcome: FullRun,
    journal: Vec<Draw>,
    text: String,
    /// (journal events at dawn, checkpoint text, checkpoint bytes)
    dawns: Vec<(usize, String, usize)>,
}

fn run_live() -> LiveRun {
    let mut src = LiveSource { rng: StdRng::seed_from_u64(SEED), journal: Vec::new() };
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let mut nights = Vec::new();
    let mut dawns = Vec::new();
    let mut divergence = None;
    for night in 0..NIGHTS {
        let (ledger, div) = run_one_night(&mut src, &mut moths, night);
        nights.push(ledger);
        let ck = checkpoint_to_text(&moths);
        // +1 per line for the newline the paper would carry.
        let ck_bytes = ck.len();
        dawns.push((src.journal.len(), ck, ck_bytes));
        if div.is_some() {
            divergence = div;
            break;
        }
    }
    let text = journal_to_text(&src.journal);
    let outcome = FullRun {
        nights,
        moths,
        divergence,
        events_consumed: src.journal.len(),
    };
    LiveRun { outcome, journal: src.journal, text, dawns }
}

fn run_replay(draws: &[Draw], moths: &mut Vec<Moth>, count: usize) -> FullRun {
    let mut src = ReplaySource { draws: draws.to_vec(), pos: 0 };
    let mut out = run_nights(&mut src, moths, count);
    out.events_consumed = src.pos;
    if let Some(d) = out.divergence.as_mut() {
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

fn moth_diffs(a: &[Moth], b: &[Moth]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x != y).count()
}

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "DIVERGED" }
}

/// Everything the headless ledger judges, computed once.
struct Ledger {
    live: LiveRun,
    full: FullRun,
    naive: FullRun,
    composed_night: NightLedger,
    composed: FullRun,
    ckpt_only: FullRun,
    suffix_start: usize,
    suffix: Vec<Draw>,
    suffix_bytes: usize,
    ckpt_bytes: usize,
    retained_bytes: usize,
}

fn compute() -> Ledger {
    let live = run_live();
    // Control: the whole book, from fresh moths, off the paper.
    let paper = journal_from_text(&live.text);
    let mut fresh: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let full = run_replay(&paper, &mut fresh, NIGHTS);

    // The policy's cut: the last dawn (after night 2).
    let (suffix_start, ckpt_text, ckpt_bytes) = live.dawns[NIGHTS - 2].clone();
    let suffix: Vec<Draw> = paper[suffix_start..].to_vec();
    let suffix_bytes = journal_to_text(&suffix).len();

    // Naive prune: the suffix alone, from fresh moths.
    let mut fresh2: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let naive = run_replay(&suffix, &mut fresh2, NIGHTS);

    // The policy: checkpoint read back from its text + suffix.
    let mut restored = checkpoint_from_text(&ckpt_text).expect("checkpoint parses");
    let composed = run_replay(&suffix, &mut restored, 1);
    let composed_night = composed.nights[0];

    // The checkpoint alone: no suffix at all.
    let mut restored2 = checkpoint_from_text(&ckpt_text).expect("checkpoint parses");
    let ckpt_only = run_replay(&[], &mut restored2, 1);

    let retained_bytes = ckpt_bytes + suffix_bytes;
    Ledger {
        live,
        full,
        naive,
        composed_night,
        composed,
        ckpt_only,
        suffix_start,
        suffix,
        suffix_bytes,
        ckpt_bytes,
        retained_bytes,
    }
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
    let led = compute();
    let live = &led.live;
    let lt = totals(&live.outcome.nights);
    let full_bytes = live.text.len();

    println!("scarecrow-retention headless: scarecrow-journal's book, pruned on purpose — what the cut keeps, what it costs");
    println!(
        "  host: {MOTHS} moths, {NIGHTS} nights x {NIGHT_TICKS} ticks, seed {SEED:#x}; policy: at each dawn, checkpoint the moths and prune the journal to the draws since that dawn"
    );
    for (n, l) in live.outcome.nights.iter().enumerate() {
        print_night("live", n, l);
    }
    println!(
        "  live totals: fed {}, burns {} (slow {}), checks {}, refusals {} (tempo {})",
        lt.0, lt.1, lt.2, lt.3, lt.4, lt.5
    );
    println!(
        "  host check: live totals vs the host's published totals (fed {}, burns {}, slow {}, checks {}, refusals {}, tempo {}) [{} — exact, declared]",
        HOST_TOTALS.0, HOST_TOTALS.1, HOST_TOTALS.2, HOST_TOTALS.3, HOST_TOTALS.4, HOST_TOTALS.5,
        verdict(lt == HOST_TOTALS)
    );
    println!("  growth, unpruned: the book grows one line per draw and is never cut");
    let mut prev = 0usize;
    for (i, (events, _, _)) in live.dawns.iter().enumerate() {
        let upto = &live.journal[..*events];
        let bytes = journal_to_text(upto).len();
        println!(
            "    after night {}: {} events ({} this night), {} bytes of paper",
            i + 1,
            events,
            events - prev,
            bytes
        );
        prev = *events;
    }
    println!(
        "  full journal: {} events, {} bytes of paper",
        live.journal.len(),
        full_bytes
    );

    // Pass 1: the control.
    let f_fields = differing_fields(&live.outcome.nights, &led.full.nights);
    let f_moths = moth_diffs(&live.outcome.moths, &led.full.moths);
    println!(
        "  P1 full replay: events consumed {} of {}, night-ledger fields differing {}, moths differing {} of {} [{} — declared exact]",
        led.full.events_consumed,
        live.journal.len(),
        f_fields,
        f_moths,
        MOTHS,
        verdict(led.full.divergence.is_none() && f_fields == 0 && f_moths == 0)
    );

    // Pass 2: the careless cut.
    let n_fields = differing_fields(&live.outcome.nights, &led.naive.nights);
    let n_moths = moth_diffs(&live.outcome.moths, &led.naive.moths);
    let nt = totals(&led.naive.nights);
    println!(
        "  P2 naive prune: suffix alone ({} events, {} bytes), replayed from fresh moths — ran {} night(s) before the paper ran out: totals fed {}, burns {} (slow {}), checks {}, refusals {} (tempo {}); fields differing {}, moths differing {} of {} [{} — declared: totals differ AND at least half the moths differ]",
        led.suffix.len(),
        led.suffix_bytes,
        led.naive.nights.len(),
        nt.0, nt.1, nt.2, nt.3, nt.4, nt.5,
        n_fields,
        n_moths,
        MOTHS,
        verdict(nt != lt && n_moths >= MOTHS / 2)
    );

    // Pass 3: the policy.
    let c_fields = differing_fields(&[live.outcome.nights[NIGHTS - 1]], &[led.composed_night]);
    let c_moths = moth_diffs(&live.outcome.moths, &led.composed.moths);
    println!(
        "  P3 checkpoint + suffix: checkpoint {} bytes ({} moths at dawn of night {}), suffix {} events / {} bytes (the first {} events pruned), final night fields differing {}, final moths differing {} of {}, events consumed {} of {} [{} — declared exact]",
        led.ckpt_bytes,
        MOTHS,
        NIGHTS,
        led.suffix.len(),
        led.suffix_bytes,
        led.suffix_start,
        c_fields,
        c_moths,
        MOTHS,
        led.composed.events_consumed,
        led.suffix.len(),
        verdict(led.composed.divergence.is_none() && c_fields == 0 && c_moths == 0)
    );

    // Pass 4: the photograph alone.
    let ck_div = led.ckpt_only.divergence.map(|d| d.event_index);
    println!(
        "  P4 checkpoint alone: replay with the checkpoint and no draws stopped at suffix event {:?} (night {}, moth {}) — a photograph cannot finish a night [{} — declared: diverges at the first draw]",
        ck_div,
        led.ckpt_only.divergence.map(|d| d.night + 1).unwrap_or(0),
        led.ckpt_only.divergence.map(|d| d.moth + 1).unwrap_or(0),
        verdict(ck_div == Some(0))
    );

    // The price of the policy, in paper.
    let pct = 100.0 * led.retained_bytes as f64 / full_bytes as f64;
    println!(
        "  P5 the retained book: checkpoint {} bytes + suffix {} bytes = {} bytes against the full {} bytes ({:.1}% kept, {} bytes pruned) [{} — declared: retained under 65% of full]",
        led.ckpt_bytes,
        led.suffix_bytes,
        led.retained_bytes,
        full_bytes,
        pct,
        full_bytes - led.retained_bytes,
        verdict(led.retained_bytes * 100 < full_bytes * 65)
    );
    println!(
        "  the row, read plainly: the full book held {} draws and replayed the run entire. Cut to one night of draws with no checkpoint, the same paper built a different run ({} of {} moths differ). Kept with the dawn checkpoint, the short book — {} bytes of {} — replayed the last night and every moth exactly. Retention is not the cut. Retention is knowing what the cut must keep."
    , live.journal.len(), n_moths, MOTHS, led.retained_bytes, full_bytes);
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
        let live = run_live();
        assert_eq!(totals(&live.outcome.nights), HOST_TOTALS);
        assert_eq!(live.journal.len(), 440, "the parent's book, re-earned");
        assert!(live.outcome.divergence.is_none());
    }

    #[test]
    fn the_full_replay_is_exact_the_control() {
        let led = compute();
        assert!(led.full.divergence.is_none());
        assert_eq!(led.full.events_consumed, led.live.journal.len());
        assert_eq!(differing_fields(&led.live.outcome.nights, &led.full.nights), 0);
        assert_eq!(moth_diffs(&led.live.outcome.moths, &led.full.moths), 0);
    }

    #[test]
    fn the_naive_prune_loses_the_run() {
        // Load-bearing: the suffix without the checkpoint is real
        // paper read from the wrong beginning — it must NOT come
        // back as the live run.
        let led = compute();
        assert_ne!(totals(&led.naive.nights), totals(&led.live.outcome.nights));
        assert!(
            moth_diffs(&led.live.outcome.moths, &led.naive.moths) >= MOTHS / 2,
            "the careless cut should misremember at least half the moths"
        );
    }

    #[test]
    fn the_checkpoint_plus_suffix_replays_exactly() {
        let led = compute();
        assert!(led.composed.divergence.is_none());
        assert_eq!(led.composed.events_consumed, led.suffix.len());
        assert_eq!(
            differing_fields(&[led.live.outcome.nights[NIGHTS - 1]], &[led.composed_night]),
            0
        );
        assert_eq!(moth_diffs(&led.live.outcome.moths, &led.composed.moths), 0);
    }

    #[test]
    fn the_checkpoint_alone_cannot_finish_a_night() {
        let led = compute();
        let d = led.ckpt_only.divergence.expect("no draws must diverge");
        assert_eq!(d.event_index, 0, "it fails at the first draw it is asked for");
    }

    #[test]
    fn the_retained_book_is_smaller_and_the_cut_is_real() {
        let led = compute();
        let full_bytes = led.live.text.len();
        assert!(led.retained_bytes < full_bytes);
        assert!(
            led.retained_bytes * 100 < full_bytes * 65,
            "retained {} of {} bytes — the declared bound",
            led.retained_bytes,
            full_bytes
        );
        assert!(led.suffix_start > 0, "something was actually pruned");
        assert_eq!(led.suffix_start + led.suffix.len(), led.live.journal.len());
    }

    #[test]
    fn the_checkpoint_round_trips_through_its_written_form() {
        let live = run_live();
        for (_, text, _) in &live.dawns {
            let moths = checkpoint_from_text(text).expect("checkpoint parses");
            assert_eq!(checkpoint_to_text(&moths), *text);
            assert_eq!(moths.len(), MOTHS);
        }
        // The final dawn's checkpoint is the live run's final state.
        let last = checkpoint_from_text(&live.dawns[NIGHTS - 1].1).unwrap();
        assert_eq!(last, live.outcome.moths);
    }
}
