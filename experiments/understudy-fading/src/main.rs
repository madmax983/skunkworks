//! # Understudy Fading \U0001F3AD
//!
//! Brought by The Understudy \U0001F3AD. Principal: `experiments/fading-moths`
//! (The Moth Keeper \U0001FA94), whose eleven FDTD rooms booked caution by
//! source -- an EARNED pool (own burns, witnessed burns) and a TAUGHT
//! pool (evidence packets, a parallel counter; an acquittal spends
//! taught first) -- and taxed one pool to zero at each dawn after the
//! first. The principal's ledger: fading-taught burns 13 (expired
//! taught 41 / earned 0), fading-earned burns 10 (expired 0 / 13),
//! fading-both burns 15 (expired 28 / 19), against its evidence
//! room's 12. Its P5 CRACKED: taxing the earned pool cost nothing --
//! at any dawn the earned pool is the small pool (13 units across two
//! dawns against the taught pool's 41), so the scarce pool was never
//! the one doing the carrying.
//!
//! The Understudy performs that role on the bus machinery of
//! `understudy-shoved`: no field, no grid, no positions, no gradient.
//! Caution here arrives by claim queue and message bus, and the pools
//! are booked the same way in kind: caution from a moth's own burns
//! and from delivered burn messages is EARNED; caution from
//! re-judging an evidence packet is TAUGHT (a parallel counter,
//! `taught_caution`; an acquittal spends taught first). The first
//! nine rooms are the inherited queue script unchanged, pinned exact
//! by tests -- the pool bookkeeping is inert there, because no dawn
//! in those rooms ever taxes. The three new rooms run this stage's
//! evidence room verbatim, differing only at the dawn:
//!
//! - **fading-taught** -- taught caution expires at each dawn.
//! - **fading-earned** -- earned caution expires at each dawn.
//! - **fading-both** -- both pools expire at each dawn.
//!
//! The question the swap puts to the principal: was the census --
//! three parts hearsay to one part scar tissue at every dawn -- a
//! fact about the cave's geometry (packets witnessed within a
//! radius, burns rare because moths must physically arrive), or
//! does any stage that teaches by flood and burns by appointment
//! book the same census? And if the earned pool is small here too,
//! does taxing it again cost nothing?
//!
//! Tolerances and predictions were declared in the README *before*
//! this crate was built or run. The headless ledger prints the
//! principal's numbers beside this performance's and names each
//! line HELD or CRACKED.
//!
//! `cargo run -p understudy-fading -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled
//! before anything else regardless.

use anyhow::Result;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

// --- The role's parameters, taken from the principal's script. ---

const MOTHS: usize = 24;
const NIGHTS: usize = 3;
const NIGHT_TICKS: u64 = 1500;
const HONEST_PERIOD: u64 = 16;
const DECOY_SHORT: u64 = 6;
const DECOY_LONG: u64 = 26;
const SAMPLE_INTERVALS: usize = 5;
const FLICKER_CV: f64 = 0.35;
const BURN_DAMAGE: i32 = 35;
const FEED_RESTORE: i32 = 25;
const AVERSION_TICKS: u64 = 150;

// --- The understudy's machinery. ---

const WAIT_MIN: u64 = 300;
const WAIT_MAX: u64 = 500;
/// Delivery probability of a burn message: burns are quiet.
/// The principal delivered ~13 witness events from 12 burns.
const BURN_DELIVERY: f64 = 0.05;
/// Delivery probability of a refusal message: refusals are public.
/// The principal delivered 495 chain events from 67 refusals.
const REFUSAL_DELIVERY: f64 = 0.30;
/// Delivery probability of an evidence packet: public, like a
/// refusal. The principal delivered 943 packets from 151 checks
/// (~6.2 per check); 23 recipients at 0.30 gives ~6.9 per check.
const EVIDENCE_DELIVERY: f64 = 0.30;

const SEED: u64 = 0x4D4F_5448;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lamp {
    Honest,
    Decoy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Room {
    Solo,
    Witness,
    Chain,
    Placed,
    Evidence,
    Throttled,
    Paired,
    Shoved,
    Unshoved,
    FadingTaught,
    FadingEarned,
    FadingBoth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    SelfBurn,
    WitnessBurn,
    ChainRefusal,
    PlacedRefusal,
    EvidencePacket,
    ThrottledPacket,
    PairedPacket,
    UnshovedPacket,
    FadingPacket,
}

#[derive(Debug, Clone, Copy)]
enum EventKind {
    Burn,
    Refusal,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    kind: EventKind,
    lamp: Lamp,
    from: usize,
}

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

#[derive(Debug, Default, Clone, Copy)]
struct NightLedger {
    fed: usize,
    burns: usize,
    deaths: usize,
    checks: usize,
    refusals: usize,
    steady_verdicts: usize,
    averted: usize,
    honest_refusals: usize,
    witness_events: usize,
    new_witness_learners: usize,
    chain_events: usize,
    new_chain_learners: usize,
    placed_events: usize,
    new_placed_learners: usize,
    evidence_events: usize,
    evidence_irregular: usize,
    evidence_steady: usize,
    evidence_acquittals: usize,
    new_evidence_learners: usize,
    throttled_events: usize,
    throttled_irregular: usize,
    throttled_steady: usize,
    throttled_acquittals: usize,
    new_throttled_learners: usize,
    packets_sent: usize,
    packets_suppressed: usize,
    paired_events: usize,
    paired_irregular: usize,
    paired_steady: usize,
    paired_acquittals: usize,
    new_paired_learners: usize,
    shoved_events: usize,
    shoved_irregular: usize,
    shoved_steady: usize,
    shoved_acquittals: usize,
    new_shoved_first: usize,
    unshoved_events: usize,
    unshoved_irregular: usize,
    unshoved_steady: usize,
    unshoved_acquittals: usize,
    new_unshoved_learners: usize,
    shove_ticks_paid: u64,
    fading_events: usize,
    fading_irregular: usize,
    fading_steady: usize,
    fading_acquittals: usize,
    new_fading_learners: usize,
    expired_taught: usize,
    expired_earned: usize,
    alive_end: usize,
}

#[derive(Debug, Default, Clone, Copy)]
struct RoomTotals {
    fed: usize,
    burns: usize,
    checks: usize,
    refusals: usize,
    averted: usize,
    honest_refusals: usize,
    witness_events: usize,
    chain_events: usize,
    placed_events: usize,
    evidence_events: usize,
    evidence_irregular: usize,
    evidence_steady: usize,
    evidence_acquittals: usize,
    throttled_events: usize,
    throttled_irregular: usize,
    throttled_steady: usize,
    throttled_acquittals: usize,
    throttled_learners: usize,
    packets_sent: usize,
    packets_suppressed: usize,
    paired_events: usize,
    paired_irregular: usize,
    paired_steady: usize,
    paired_acquittals: usize,
    paired_learners: usize,
    shoved_events: usize,
    shoved_irregular: usize,
    shoved_steady: usize,
    shoved_acquittals: usize,
    shoved_first: usize,
    unshoved_events: usize,
    unshoved_irregular: usize,
    unshoved_steady: usize,
    unshoved_acquittals: usize,
    unshoved_learners: usize,
    shove_ticks_paid: u64,
    fading_events: usize,
    fading_irregular: usize,
    fading_steady: usize,
    fading_acquittals: usize,
    fading_learners: usize,
    expired_taught: usize,
    expired_earned: usize,
    witness_learners: usize,
    chain_learners: usize,
    placed_learners: usize,
    evidence_learners: usize,
    alive_end: usize,
}

#[derive(Clone, Debug)]
struct Moth {
    health: i32,
    caution: u32,
    dead: bool,
    first_source: Option<Source>,
    /// Placed room only: named aversions held, per lamp.
    decoy_aversions: u32,
    honest_aversions: u32,
    /// Burns this moth paid for itself (the acquittal rule asks).
    personal_burns: u32,
    /// Paired room only: ticks owed to a taught aversion. The next
    /// turn starts this many ticks late — the queue-native recoil.
    shove_debt: u64,
    /// Has a debt-only (Shoved room) packet ever reached this moth?
    debt_received: bool,
    /// The taught pool: caution currently held that came from
    /// re-judging an evidence packet, in any packet room. Earned
    /// caution is `caution` minus this. An acquittal spends taught
    /// first. Inert in the inherited rooms (no dawn taxes there);
    /// load-bearing only in the three fading rooms.
    taught_caution: u32,
}

impl Moth {
    fn new() -> Self {
        Self {
            health: 100,
            caution: 0,
            dead: false,
            first_source: None,
            decoy_aversions: 0,
            honest_aversions: 0,
            personal_burns: 0,
            shove_debt: 0,
            debt_received: false,
            taught_caution: 0,
        }
    }
}

/// Deliver one posted event to the bus subscribers. Deliveries are
/// drawn per recipient; a delivered message teaches caution, and a
/// first caution is credited to its source. In the placed room a
/// delivered refusal also hands over the lamp's identity, exactly.
fn broadcast(
    room: Room,
    ev: Event,
    moths: &mut [Moth],
    rng: &mut StdRng,
    ledger: &mut NightLedger,
) {
    let (prob, source) = match (room, ev.kind) {
        (_, EventKind::Burn) => (BURN_DELIVERY, Source::WitnessBurn),
        (Room::Chain, EventKind::Refusal) => (REFUSAL_DELIVERY, Source::ChainRefusal),
        (Room::Placed, EventKind::Refusal) => (REFUSAL_DELIVERY, Source::PlacedRefusal),
        _ => return,
    };
    for (j, m) in moths.iter_mut().enumerate() {
        if j == ev.from || m.dead {
            continue;
        }
        if !rng.gen_bool(prob) {
            continue;
        }
        match (room, ev.kind) {
            (_, EventKind::Burn) => {
                ledger.witness_events += 1;
                if m.caution == 0 {
                    ledger.new_witness_learners += 1;
                }
            }
            (Room::Chain, EventKind::Refusal) => {
                ledger.chain_events += 1;
                if m.caution == 0 {
                    ledger.new_chain_learners += 1;
                }
            }
            (Room::Placed, EventKind::Refusal) => {
                ledger.placed_events += 1;
                if m.caution == 0 {
                    ledger.new_placed_learners += 1;
                }
                // The novel trait, on queue machinery: the message
                // names its lamp. The copy is exact at every hand.
                match ev.lamp {
                    Lamp::Decoy => m.decoy_aversions += 1,
                    Lamp::Honest => m.honest_aversions += 1,
                }
            }
            _ => unreachable!(),
        }
        if m.caution == 0 && m.first_source.is_none() {
            m.first_source = Some(source);
        }
        m.caution += 1;
    }
}

/// Deliver one evidence packet: the checker's actual sampled
/// intervals, nothing else — no verdict, no lamp name. Each
/// recipient re-computes the CV on the principal's 0.35 line:
/// irregular earns caution (a first caution is credited to the
/// packet); steady can acquit hearsay caution — a recipient with
/// no personal burn whose caution > 0 loses one.
fn broadcast_evidence(
    from: usize,
    intervals: &[u64],
    moths: &mut [Moth],
    rng: &mut StdRng,
    ledger: &mut NightLedger,
) {
    let irregular = intervals.len() >= 3 && cv(intervals) > FLICKER_CV;
    for (j, m) in moths.iter_mut().enumerate() {
        if j == from || m.dead {
            continue;
        }
        if !rng.gen_bool(EVIDENCE_DELIVERY) {
            continue;
        }
        ledger.evidence_events += 1;
        if irregular {
            ledger.evidence_irregular += 1;
            if m.first_source.is_none() {
                m.first_source = Some(Source::EvidencePacket);
                ledger.new_evidence_learners += 1;
            }
            m.caution += 1;
            m.taught_caution += 1;
        } else {
            ledger.evidence_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                if m.taught_caution > 0 {
                    m.taught_caution -= 1;
                }
                ledger.evidence_acquittals += 1;
            }
        }
    }
}


/// Deliver one throttled packet: identical re-judgement to
/// `broadcast_evidence`, but deliveries are booked to the throttled
/// ledger and a first caution is credited to the throttled packet.
fn broadcast_throttled(
    from: usize,
    intervals: &[u64],
    moths: &mut [Moth],
    rng: &mut StdRng,
    ledger: &mut NightLedger,
) {
    let irregular = intervals.len() >= 3 && cv(intervals) > FLICKER_CV;
    for (j, m) in moths.iter_mut().enumerate() {
        if j == from || m.dead {
            continue;
        }
        if !rng.gen_bool(EVIDENCE_DELIVERY) {
            continue;
        }
        ledger.throttled_events += 1;
        if irregular {
            ledger.throttled_irregular += 1;
            if m.first_source.is_none() {
                m.first_source = Some(Source::ThrottledPacket);
                ledger.new_throttled_learners += 1;
            }
            m.caution += 1;
            m.taught_caution += 1;
        } else {
            ledger.throttled_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                if m.taught_caution > 0 {
                    m.taught_caution -= 1;
                }
                ledger.throttled_acquittals += 1;
            }
        }
    }
}

/// Deliver one shoved or unshoved packet: identical re-judgement
/// to `broadcast_evidence` — irregular earns caution, steady can
/// acquit hearsay caution. The one difference between the rooms:
/// in the Paired room an irregular verdict also hands the
/// recipient the taught aversion, as a clock debt of
/// AVERSION_TICKS (refreshed, as the cave refreshes it, never
/// stacked). In the Unshoved room the lesson arrives as
/// information and nothing else moves.
fn broadcast_packets(
    room: Room,
    from: usize,
    intervals: &[u64],
    moths: &mut [Moth],
    rng: &mut StdRng,
    ledger: &mut NightLedger,
) {
    let irregular = intervals.len() >= 3 && cv(intervals) > FLICKER_CV;
    for (j, m) in moths.iter_mut().enumerate() {
        if j == from || m.dead {
            continue;
        }
        if !rng.gen_bool(EVIDENCE_DELIVERY) {
            continue;
        }
        match room {
            Room::Paired => {
                ledger.paired_events += 1;
                if irregular {
                    ledger.paired_irregular += 1;
                    if m.first_source.is_none() {
                        m.first_source = Some(Source::PairedPacket);
                        ledger.new_paired_learners += 1;
                    }
                    m.caution += 1;
                    m.taught_caution += 1;
                    m.shove_debt = AVERSION_TICKS;
                } else {
                    ledger.paired_steady += 1;
                    if m.personal_burns == 0 && m.caution > 0 {
                        m.caution -= 1;
                        if m.taught_caution > 0 {
                            m.taught_caution -= 1;
                        }
                        ledger.paired_acquittals += 1;
                    }
                }
            }
            Room::Unshoved => {
                ledger.unshoved_events += 1;
                if irregular {
                    ledger.unshoved_irregular += 1;
                    if m.first_source.is_none() {
                        m.first_source = Some(Source::UnshovedPacket);
                        ledger.new_unshoved_learners += 1;
                    }
                    m.caution += 1;
                    m.taught_caution += 1;
                } else {
                    ledger.unshoved_steady += 1;
                    if m.personal_burns == 0 && m.caution > 0 {
                        m.caution -= 1;
                        if m.taught_caution > 0 {
                            m.taught_caution -= 1;
                        }
                        ledger.unshoved_acquittals += 1;
                    }
                }
            }
            Room::Shoved => {
                ledger.shoved_events += 1;
                if irregular {
                    ledger.shoved_irregular += 1;
                    if !m.debt_received && m.personal_burns == 0 {
                        ledger.new_shoved_first += 1;
                    }
                    m.debt_received = true;
                    // The principal's whole trait, on this stage:
                    // the debt, and nothing else. No caution is
                    // paid, no checker is recruited, no first
                    // caution is credited — the push carries no
                    // claim.
                    m.shove_debt = AVERSION_TICKS;
                } else {
                    ledger.shoved_steady += 1;
                    // No acquittal branch: a steady packet in this
                    // room un-teaches nothing, because the irregular
                    // packets taught nothing. shoved_acquittals is
                    // booked by no line of this arm, ever.
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Deliver one fading packet: identical re-judgement to
/// `broadcast_evidence` -- irregular earns caution, booked to the
/// taught pool the moment it teaches; steady can acquit hearsay
/// caution, spending taught first. Deliveries are booked to the
/// fading ledger. The dawn, not the packet, is this room's trait.
fn broadcast_fading(
    from: usize,
    intervals: &[u64],
    moths: &mut [Moth],
    rng: &mut StdRng,
    ledger: &mut NightLedger,
) {
    let irregular = intervals.len() >= 3 && cv(intervals) > FLICKER_CV;
    for (j, m) in moths.iter_mut().enumerate() {
        if j == from || m.dead {
            continue;
        }
        if !rng.gen_bool(EVIDENCE_DELIVERY) {
            continue;
        }
        ledger.fading_events += 1;
        if irregular {
            ledger.fading_irregular += 1;
            if m.first_source.is_none() {
                m.first_source = Some(Source::FadingPacket);
                ledger.new_fading_learners += 1;
            }
            m.caution += 1;
            m.taught_caution += 1;
        } else {
            ledger.fading_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                if m.taught_caution > 0 {
                    m.taught_caution -= 1;
                }
                ledger.fading_acquittals += 1;
            }
        }
    }
}

fn run_room(room: Room, seed: u64) -> (Vec<NightLedger>, RoomTotals) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let mut nights = Vec::new();
    for night_idx in 0..NIGHTS {
        let mut ledger = NightLedger::default();
        // The dawn tax (fading rooms only): at each dawn after the
        // first, one caution pool is taxed to zero. The expiry is
        // booked into the new night's ledger -- the dawn is where
        // the loss is discovered, exactly as the principal books it.
        // No RNG is drawn here, so night 1 of every fading room is
        // the evidence room's night 1, draw for draw.
        if night_idx > 0 {
            match room {
                Room::FadingTaught => {
                    for m in moths.iter_mut() {
                        if m.dead {
                            continue;
                        }
                        ledger.expired_taught += m.taught_caution as usize;
                        m.caution = m.caution.saturating_sub(m.taught_caution);
                        m.taught_caution = 0;
                    }
                }
                Room::FadingEarned => {
                    for m in moths.iter_mut() {
                        if m.dead {
                            continue;
                        }
                        let earned = m.caution.saturating_sub(m.taught_caution);
                        ledger.expired_earned += earned as usize;
                        m.caution = m.taught_caution;
                    }
                }
                Room::FadingBoth => {
                    for m in moths.iter_mut() {
                        if m.dead {
                            continue;
                        }
                        ledger.expired_taught += m.taught_caution as usize;
                        ledger.expired_earned +=
                            m.caution.saturating_sub(m.taught_caution) as usize;
                        m.caution = 0;
                        m.taught_caution = 0;
                    }
                }
                _ => {}
            }
        }
        // Throttled room: has this checker already posted tonight?
        let mut sent_tonight = [false; MOTHS];
        for i in 0..MOTHS {
            if moths[i].dead {
                continue;
            }
            // Events this moth posts during its turn; the bus
            // delivers them before the next moth's turn — a queue
            // is sequential, where the principal's cave was
            // simultaneous. That difference is part of the stage.
            let mut posted: Vec<Event> = Vec::new();
            let mut posted_packets: Vec<Vec<u64>> = Vec::new();
            // A taught aversion is paid here: the turn starts
            // shove_debt ticks late. The debt is zero in every
            // rooms except Paired and Shoved, so the pinned rooms are
            // untouched by this line.
            let mut t: u64 = moths[i].shove_debt;
            moths[i].shove_debt = 0;
            ledger.shove_ticks_paid += t;
            let mut settled = false;
            while !settled && t < NIGHT_TICKS {
                t += rng.gen_range(WAIT_MIN..=WAIT_MAX);
                if t >= NIGHT_TICKS {
                    break;
                }
                let lamp = if rng.gen_bool(0.5) {
                    Lamp::Honest
                } else {
                    Lamp::Decoy
                };
                // Placed pre-emption: a named aversion refuses the
                // named lamp's next claim without a check. The
                // refusal itself is posted — depth accumulates on
                // the bus exactly as it did in the cave.
                if room == Room::Placed {
                    let held = match lamp {
                        Lamp::Decoy => moths[i].decoy_aversions,
                        Lamp::Honest => moths[i].honest_aversions,
                    };
                    if held > 0 {
                        match lamp {
                            Lamp::Decoy => moths[i].decoy_aversions -= 1,
                            Lamp::Honest => moths[i].honest_aversions -= 1,
                        }
                        ledger.averted += 1;
                        ledger.refusals += 1;
                        posted.push(Event {
                            kind: EventKind::Refusal,
                            lamp,
                            from: i,
                        });
                        t += AVERSION_TICKS;
                        continue;
                    }
                }
                if moths[i].caution == 0 {
                    match lamp {
                        Lamp::Honest => {
                            ledger.fed += 1;
                            moths[i].health = (moths[i].health + FEED_RESTORE).min(100);
                            settled = true;
                        }
                        Lamp::Decoy => {
                            ledger.burns += 1;
                            moths[i].health -= BURN_DAMAGE;
                            moths[i].caution += 1;
                            moths[i].personal_burns += 1;
                            if moths[i].first_source.is_none() {
                                moths[i].first_source = Some(Source::SelfBurn);
                            }
                            posted.push(Event {
                                kind: EventKind::Burn,
                                lamp,
                                from: i,
                            });
                            if moths[i].health <= 0 {
                                moths[i].dead = true;
                                ledger.deaths += 1;
                                settled = true;
                            }
                        }
                    }
                    continue;
                }
                // Cautious: sample the lamp's clock directly.
                ledger.checks += 1;
                let intervals = sample_intervals(lamp, rng.gen_bool(0.5));
                if room == Room::Evidence || room == Room::Paired || room == Room::Unshoved || room == Room::Shoved || room == Room::FadingTaught || room == Room::FadingEarned || room == Room::FadingBoth {
                    posted_packets.push(intervals.clone());
                }
                if room == Room::Throttled {
                    if sent_tonight[i] {
                        ledger.packets_suppressed += 1;
                    } else {
                        sent_tonight[i] = true;
                        ledger.packets_sent += 1;
                        posted_packets.push(intervals.clone());
                    }
                }
                t += intervals.iter().sum::<u64>();
                if cv(&intervals) > FLICKER_CV {
                    ledger.refusals += 1;
                    if lamp == Lamp::Honest {
                        ledger.honest_refusals += 1;
                    }
                    posted.push(Event {
                        kind: EventKind::Refusal,
                        lamp,
                        from: i,
                    });
                    t += AVERSION_TICKS;
                } else {
                    ledger.steady_verdicts += 1;
                    if t < NIGHT_TICKS {
                        match lamp {
                            Lamp::Honest => {
                                ledger.fed += 1;
                                moths[i].health = (moths[i].health + FEED_RESTORE).min(100);
                                settled = true;
                            }
                            Lamp::Decoy => {
                                // A fooled check. On exact delivery
                                // the decoy's CV never reads steady,
                                // so this line is expected to stay 0
                                // — counted, never hidden.
                                ledger.burns += 1;
                                moths[i].health -= BURN_DAMAGE;
                                moths[i].caution += 1;
                                moths[i].personal_burns += 1;
                                posted.push(Event {
                                    kind: EventKind::Burn,
                                    lamp,
                                    from: i,
                                });
                                if moths[i].health <= 0 {
                                    moths[i].dead = true;
                                    ledger.deaths += 1;
                                    settled = true;
                                }
                            }
                        }
                    }
                }
            }
            if room != Room::Solo {
                for ev in posted {
                    broadcast(room, ev, &mut moths, &mut rng, &mut ledger);
                }
            }
            if room == Room::Evidence {
                for packet in &posted_packets {
                    broadcast_evidence(i, packet, &mut moths, &mut rng, &mut ledger);
                }
            }
            if room == Room::Throttled {
                for packet in &posted_packets {
                    broadcast_throttled(i, packet, &mut moths, &mut rng, &mut ledger);
                }
            }
            if room == Room::Paired || room == Room::Unshoved || room == Room::Shoved {
                for packet in &posted_packets {
                    broadcast_packets(room, i, packet, &mut moths, &mut rng, &mut ledger);
                }
            }
            if room == Room::FadingTaught || room == Room::FadingEarned || room == Room::FadingBoth {
                for packet in &posted_packets {
                    broadcast_fading(i, packet, &mut moths, &mut rng, &mut ledger);
                }
            }
        }
        ledger.alive_end = moths.iter().filter(|m| !m.dead).count();
        nights.push(ledger);
    }
    let mut totals = RoomTotals {
        alive_end: nights[NIGHTS - 1].alive_end,
        ..Default::default()
    };
    for l in &nights {
        totals.fed += l.fed;
        totals.burns += l.burns;
        totals.checks += l.checks;
        totals.refusals += l.refusals;
        totals.averted += l.averted;
        totals.honest_refusals += l.honest_refusals;
        totals.witness_events += l.witness_events;
        totals.chain_events += l.chain_events;
        totals.placed_events += l.placed_events;
        totals.evidence_events += l.evidence_events;
        totals.evidence_irregular += l.evidence_irregular;
        totals.evidence_steady += l.evidence_steady;
        totals.evidence_acquittals += l.evidence_acquittals;
        totals.evidence_learners += l.new_evidence_learners;
        totals.throttled_events += l.throttled_events;
        totals.throttled_irregular += l.throttled_irregular;
        totals.throttled_steady += l.throttled_steady;
        totals.throttled_acquittals += l.throttled_acquittals;
        totals.throttled_learners += l.new_throttled_learners;
        totals.packets_sent += l.packets_sent;
        totals.packets_suppressed += l.packets_suppressed;
        totals.paired_events += l.paired_events;
        totals.paired_irregular += l.paired_irregular;
        totals.paired_steady += l.paired_steady;
        totals.paired_acquittals += l.paired_acquittals;
        totals.paired_learners += l.new_paired_learners;
        totals.shoved_events += l.shoved_events;
        totals.shoved_irregular += l.shoved_irregular;
        totals.shoved_steady += l.shoved_steady;
        totals.shoved_acquittals += l.shoved_acquittals;
        totals.shoved_first += l.new_shoved_first;
        totals.unshoved_events += l.unshoved_events;
        totals.unshoved_irregular += l.unshoved_irregular;
        totals.unshoved_steady += l.unshoved_steady;
        totals.unshoved_acquittals += l.unshoved_acquittals;
        totals.unshoved_learners += l.new_unshoved_learners;
        totals.shove_ticks_paid += l.shove_ticks_paid;
        totals.fading_events += l.fading_events;
        totals.fading_irregular += l.fading_irregular;
        totals.fading_steady += l.fading_steady;
        totals.fading_acquittals += l.fading_acquittals;
        totals.fading_learners += l.new_fading_learners;
        totals.expired_taught += l.expired_taught;
        totals.expired_earned += l.expired_earned;
        totals.witness_learners += l.new_witness_learners;
        totals.chain_learners += l.new_chain_learners;
        totals.placed_learners += l.new_placed_learners;
    }
    (nights, totals)
}

fn verdict(ok: bool) -> &'static str {
    if ok { "HELD" } else { "CRACKED" }
}

fn print_room(name: &str, nights: &[NightLedger], totals: &RoomTotals) {
    println!("  regime '{name}':");
    for (n, l) in nights.iter().enumerate() {
        println!(
            "    night {}: fed {}, burns {}, deaths {}, checks {}, refusals {}, averted {}, witness events {}, chain events {}, placed events {}, evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), throttled events {} (irregular {}, steady {}, acquittals {}, learners {}, sent {}, suppressed {}), paired events {} (irregular {}, steady {}, acquittals {}, learners {}), unshoved events {} (irregular {}, steady {}, acquittals {}, learners {}), shoved events {} (irregular {}, steady {}, acquittals {}, first-shoves {}), fading events {} (irregular {}, steady {}, acquittals {}, learners {}), expired taught {}, expired earned {}, debt ticks paid {}, alive at dawn {}",
            n + 1, l.fed, l.burns, l.deaths, l.checks, l.refusals, l.averted,
            l.witness_events, l.chain_events, l.placed_events, l.evidence_events,
            l.evidence_irregular, l.evidence_steady, l.evidence_acquittals,
            l.new_evidence_learners, l.throttled_events, l.throttled_irregular,
            l.throttled_steady, l.throttled_acquittals, l.new_throttled_learners,
            l.packets_sent, l.packets_suppressed, l.paired_events,
            l.paired_irregular, l.paired_steady, l.paired_acquittals,
            l.new_paired_learners, l.unshoved_events, l.unshoved_irregular,
            l.unshoved_steady, l.unshoved_acquittals, l.new_unshoved_learners,
            l.shoved_events, l.shoved_irregular, l.shoved_steady,
            l.shoved_acquittals, l.new_shoved_first, l.fading_events,
            l.fading_irregular, l.fading_steady, l.fading_acquittals,
            l.new_fading_learners, l.expired_taught, l.expired_earned,
            l.shove_ticks_paid, l.alive_end
        );
    }
    println!(
        "    totals: fed {}, burns {}, checks {}, refusals {}, averted {}, honest refusals {}, witness events {} (learners {}), chain events {} (learners {}), placed events {} (learners {}), evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), throttled events {} (irregular {}, steady {}, acquittals {}, learners {}), packets sent {}, suppressed {}, paired events {} (irregular {}, steady {}, acquittals {}, learners {}), unshoved events {} (irregular {}, steady {}, acquittals {}, learners {}), shoved events {} (irregular {}, steady {}, acquittals {}, first-shoves {}), fading events {} (irregular {}, steady {}, acquittals {}, learners {}), expired taught {}, expired earned {}, alive {}, debt ticks paid {}",
        totals.fed, totals.burns, totals.checks, totals.refusals, totals.averted,
        totals.honest_refusals, totals.witness_events, totals.witness_learners,
        totals.chain_events, totals.chain_learners, totals.placed_events,
        totals.placed_learners, totals.evidence_events, totals.evidence_irregular,
        totals.evidence_steady, totals.evidence_acquittals, totals.evidence_learners,
        totals.throttled_events, totals.throttled_irregular, totals.throttled_steady,
        totals.throttled_acquittals, totals.throttled_learners, totals.packets_sent,
        totals.packets_suppressed, totals.paired_events, totals.paired_irregular,
        totals.paired_steady, totals.paired_acquittals, totals.paired_learners,
        totals.unshoved_events, totals.unshoved_irregular, totals.unshoved_steady,
        totals.unshoved_acquittals, totals.unshoved_learners, totals.shoved_events,
        totals.shoved_irregular, totals.shoved_steady, totals.shoved_acquittals,
        totals.shoved_first, totals.fading_events, totals.fading_irregular,
        totals.fading_steady, totals.fading_acquittals, totals.fading_learners,
        totals.expired_taught, totals.expired_earned, totals.alive_end,
        totals.shove_ticks_paid
    );
}

fn run_headless() {
    let (solo_n, solo) = run_room(Room::Solo, SEED);
    let (wit_n, wit) = run_room(Room::Witness, SEED);
    let (chain_n, chain) = run_room(Room::Chain, SEED);
    let (placed_n, placed) = run_room(Room::Placed, SEED);
    let (ev_n, ev) = run_room(Room::Evidence, SEED);
    let (th_n, th) = run_room(Room::Throttled, SEED);
    let (un_n, un) = run_room(Room::Unshoved, SEED);
    let (pa_n, pa) = run_room(Room::Paired, SEED);
    let (sh_n, sh) = run_room(Room::Shoved, SEED);
    let (ft_n, ft) = run_room(Room::FadingTaught, SEED);
    let (fe_n, fe) = run_room(Room::FadingEarned, SEED);
    let (fb_n, fb) = run_room(Room::FadingBoth, SEED);
    println!("understudy-fading headless: the Fading role, performed by queues \u{2014} no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX}; bus delivery burn {BURN_DELIVERY}, refusal {REFUSAL_DELIVERY}, evidence {EVIDENCE_DELIVERY}"
    );
    println!("  principal's ledger (fading-moths, cave): evidence fed 43 burns 12, checks 151 (packets 943; learners 18, acquittals 223); fading-taught fed 40 burns 13, checks 136 (events 891: irr 332, steady 559; learners 17, acquittals 212; expired taught 41, earned 0); fading-earned fed 44 burns 10, checks 133 (events 841: irr 259, steady 582; learners 18, acquittals 190; expired taught 0, earned 13); fading-both fed 46 burns 15, checks 103 (events 393: irr 131, steady 262; learners 13, acquittals 94; expired taught 28, earned 19)");
    println!("  this stage's precedent (pinned): evidence room fed 57 burns 16, checks 92 (packets 634: irregular 277, steady 357; learners 11, acquittals 156)");
    print_room("solo (caution only from your own burns)", &solo_n, &solo);
    print_room("witness (caution also from a delivered burn message)", &wit_n, &wit);
    print_room("chain (caution also from a delivered refusal message)", &chain_n, &chain);
    print_room("placed (a delivered refusal also names its lamp)", &placed_n, &placed);
    print_room("evidence (every check posts its intervals; recipients re-judge)", &ev_n, &ev);
    print_room("throttled (only a checker's first check each night posts)", &th_n, &th);
    print_room("unshoved (every check posts; an irregular packet teaches caution and nothing else)", &un_n, &un);
    print_room("paired (every check posts; an irregular packet teaches caution AND a 150-tick clock debt)", &pa_n, &pa);
    print_room("shoved (every check posts; an irregular packet teaches a 150-tick clock debt and NOTHING else)", &sh_n, &sh);
    print_room("fading-taught (evidence teaching; taught caution expires at each dawn, earned persists)", &ft_n, &ft);
    print_room("fading-earned (evidence teaching; earned caution expires at each dawn, taught persists)", &fe_n, &fe);
    print_room("fading-both (evidence teaching; both caution pools expire at each dawn)", &fb_n, &fb);
    println!("  comparison against the principal, tolerances declared in the README before the first run:");
    println!(
        "    B1 solo burns {} vs queue precedent 20 [{} in 6..=26]; solo alive {} [{} = 24]",
        solo.burns, verdict((6..=26).contains(&solo.burns)), solo.alive_end, verdict(solo.alive_end == 24)
    );
    println!(
        "    B2 witness burns {} < solo burns {} [{}] (principal 8 < 12)",
        wit.burns, solo.burns, verdict(wit.burns < solo.burns)
    );
    println!(
        "    B3 chain refusal events {} > 5x witness burn events {} [{}] (principal 495 vs 13)",
        chain.chain_events, wit.witness_events,
        verdict(chain.chain_events > 5 * wit.witness_events.max(1) && wit.witness_events > 0)
    );
    println!(
        "    B4 placed first-cautions {} >= chain first-cautions {} [{}] (principal 20 >= 18)",
        placed.placed_learners, chain.chain_learners,
        verdict(placed.placed_learners >= chain.chain_learners)
    );
    println!(
        "    P1 the channel works fading, in both kinds, in each fading room: taught irr {} / steady {} learners {} [{}]; earned irr {} / steady {} learners {} [{}]; both irr {} / steady {} learners {} [{}] (principal 332/559, 259/582, 131/262; learners 17/18/13)",
        ft.fading_irregular, ft.fading_steady, ft.fading_learners,
        verdict(ft.fading_irregular > 0 && ft.fading_steady > 0 && ft.fading_learners > 0),
        fe.fading_irregular, fe.fading_steady, fe.fading_learners,
        verdict(fe.fading_irregular > 0 && fe.fading_steady > 0 && fe.fading_learners > 0),
        fb.fading_irregular, fb.fading_steady, fb.fading_learners,
        verdict(fb.fading_irregular > 0 && fb.fading_steady > 0 && fb.fading_learners > 0)
    );
    println!(
        "    P2 night 1 identity: fading-taught night 1 fed {} burns {} checks {} refusals {} [{}]; fading-earned [{}]; fading-both [{}] \u{2014} each equals this stage's evidence night 1 (fed {}, burns {}, checks {}, refusals {}) field for field (principal HELD exactly)",
        ft_n[0].fed, ft_n[0].burns, ft_n[0].checks, ft_n[0].refusals,
        verdict(ft_n[0].fed == ev_n[0].fed && ft_n[0].burns == ev_n[0].burns && ft_n[0].checks == ev_n[0].checks && ft_n[0].refusals == ev_n[0].refusals),
        verdict(fe_n[0].fed == ev_n[0].fed && fe_n[0].burns == ev_n[0].burns && fe_n[0].checks == ev_n[0].checks && fe_n[0].refusals == ev_n[0].refusals),
        verdict(fb_n[0].fed == ev_n[0].fed && fb_n[0].burns == ev_n[0].burns && fb_n[0].checks == ev_n[0].checks && fb_n[0].refusals == ev_n[0].refusals),
        ev_n[0].fed, ev_n[0].burns, ev_n[0].checks, ev_n[0].refusals
    );
    println!(
        "    P3 the dawn tax fires by source exactly: fading-taught expired taught {} / earned {} [{}]; fading-earned {} / {} [{}]; fading-both {} / {} [{}] (principal 41/0, 0/13, 28/19)",
        ft.expired_taught, ft.expired_earned,
        verdict(ft.expired_taught > 0 && ft.expired_earned == 0),
        fe.expired_taught, fe.expired_earned,
        verdict(fe.expired_taught == 0 && fe.expired_earned > 0),
        fb.expired_taught, fb.expired_earned,
        verdict(fb.expired_taught > 0 && fb.expired_earned > 0)
    );
    println!(
        "    P4 claim A: fading-taught burns {} > this stage's evidence burns {} [{}] (principal HELD by one burn: 13 vs 12 \u{2014} taught caution carried across a dawn is worth about one burn)",
        ft.burns, ev.burns, verdict(ft.burns > ev.burns)
    );
    println!(
        "    P5 claim B: fading-earned burns {} > this stage's evidence burns {} [{}] (principal CRACKED: 10, BELOW its evidence 12 \u{2014} taxing the earned pool cost nothing in the cave)",
        fe.burns, ev.burns, verdict(fe.burns > ev.burns)
    );
    println!(
        "    P6 fading-both burns {} >= max(fading-taught {}, fading-earned {}) [{}] (principal HELD: 15 >= max(13, 10))",
        fb.burns, ft.burns, fe.burns,
        verdict(fb.burns >= ft.burns.max(fe.burns))
    );
    println!(
        "    P7 a forgetting moth still eats: fading fed taught {} / earned {} / both {} [{} each >= 50] (principal declared >= 40 against its evidence 43 and winked: 40/44/46 \u{2014} the most-forgetting room fed most; queue evidence precedent fed 57)",
        ft.fed, fe.fed, fb.fed,
        verdict(ft.fed >= 50 && fe.fed >= 50 && fb.fed >= 50)
    );
    println!(
        "    P8 the census: expired taught (fading-taught) {} > expired earned (fading-earned) {} [{}] (principal 41 > 13 \u{2014} at any dawn a moth holds roughly three times as much hearsay as scar tissue; if the pools book even here, the census is a property of the teaching, not the cave)",
        ft.expired_taught, fe.expired_earned,
        verdict(ft.expired_taught > fe.expired_earned)
    );
    println!(
        "    honest refusals across all rooms {} (counted, not a declared line on this stage; queue precedent 0)",
        solo.honest_refusals + wit.honest_refusals + chain.honest_refusals + placed.honest_refusals + ev.honest_refusals + th.honest_refusals + un.honest_refusals + pa.honest_refusals + sh.honest_refusals + ft.honest_refusals + fe.honest_refusals + fb.honest_refusals
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
        assert!(cv(&iv) <= FLICKER_CV);
    }

    #[test]
    fn the_decoy_clock_reads_irregular_on_either_foot() {
        for short_first in [true, false] {
            let iv = sample_intervals(Lamp::Decoy, short_first);
            assert!(cv(&iv) > FLICKER_CV, "decoy read steady: {iv:?}");
        }
    }

    #[test]
    fn a_check_costs_no_more_than_the_principals_cap() {
        for lamp in [Lamp::Honest, Lamp::Decoy] {
            for short_first in [true, false] {
                let cost: u64 = sample_intervals(lamp, short_first).iter().sum();
                assert!(cost <= 96, "check cost {cost} exceeds the principal's cap");
            }
        }
    }

    #[test]
    fn the_solo_room_burns_and_learns() {
        let (_, solo) = run_room(Room::Solo, SEED);
        assert!(solo.burns > 0, "solo room never burned: {solo:?}");
        assert!(solo.refusals > 0, "solo room never refused: {solo:?}");
        assert!(solo.fed > 0, "solo room never fed: {solo:?}");
        assert_eq!(solo.witness_events + solo.chain_events + solo.placed_events + solo.evidence_events + solo.throttled_events, 0);
    }

    #[test]
    fn the_first_five_rooms_reproduce_the_queue_precedent() {
        let (_, solo) = run_room(Room::Solo, SEED);
        assert_eq!(solo.burns, 20);
        assert_eq!(solo.fed, 61);
        let (_, chain) = run_room(Room::Chain, SEED);
        assert_eq!(chain.chain_events, 305);
        let (_, placed) = run_room(Room::Placed, SEED);
        assert_eq!(placed.placed_events, 360);
        assert_eq!(placed.averted, 50);
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert_eq!(ev.burns, 16);
        assert_eq!(ev.evidence_events, 634);
        assert_eq!(ev.evidence_acquittals, 156);
        assert_eq!(ev.evidence_learners, 11);
    }

    #[test]
    fn throttled_packets_travel_in_both_kinds() {
        let (_, th) = run_room(Room::Throttled, SEED);
        assert!(th.throttled_events > 0, "no throttled packet was ever delivered: {th:?}");
        assert!(th.throttled_irregular > 0, "no irregular packet delivered: {th:?}");
        assert!(th.throttled_steady > 0, "no steady packet delivered: {th:?}");
        assert_eq!(th.throttled_irregular + th.throttled_steady, th.throttled_events);
    }

    #[test]
    fn the_throttle_cap_holds() {
        let (_, th) = run_room(Room::Throttled, SEED);
        assert!(th.packets_sent <= 72, "more than one packet per checker per night: {th:?}");
        assert!(th.packets_suppressed > 0, "nothing was ever suppressed: {th:?}");
        assert_eq!(th.packets_sent + th.packets_suppressed, th.checks);
    }

    #[test]
    fn throttled_teaches_a_first_caution() {
        let (_, th) = run_room(Room::Throttled, SEED);
        assert!(th.throttled_learners > 0, "no moth's first caution was a throttled packet: {th:?}");
    }

    #[test]
    fn rationing_thins_the_flood_and_the_acquittals() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        let (_, th) = run_room(Room::Throttled, SEED);
        assert!(th.throttled_events < ev.evidence_events, "throttled flood not thinner: {th:?} vs {ev:?}");
        assert!(th.throttled_acquittals > 0, "no acquittal in the throttled room: {th:?}");
        assert!(th.throttled_acquittals < ev.evidence_acquittals, "acquittals not thinned: {th:?} vs {ev:?}");
    }

    #[test]
    fn the_throttled_room_still_feeds_lives_and_burns() {
        let (_, th) = run_room(Room::Throttled, SEED);
        assert!(th.fed > 0, "throttled room never fed: {th:?}");
        assert!(th.burns > 0, "throttled room never burned: {th:?}");
        assert_eq!(th.alive_end, 24, "throttled room lost a moth: {th:?}");
    }

    #[test]
    fn no_room_ever_refuses_the_honest_lamp() {
        for room in [Room::Solo, Room::Witness, Room::Chain, Room::Placed, Room::Evidence, Room::Throttled, Room::Paired, Room::Unshoved, Room::Shoved, Room::FadingTaught, Room::FadingEarned, Room::FadingBoth] {
            let (_, totals) = run_room(room, SEED);
            assert_eq!(totals.honest_refusals, 0, "{room:?} refused the honest lamp");
        }
    }

    #[test]
    fn the_performance_is_deterministic() {
        let (_, a) = run_room(Room::FadingBoth, SEED);
        let (_, b) = run_room(Room::FadingBoth, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.fading_events, b.fading_events);
        assert_eq!(a.expired_taught, b.expired_taught);
        assert_eq!(a.expired_earned, b.expired_earned);
    }

    #[test]
    fn throttled_totals_pinned() {
        let (_, th) = run_room(Room::Throttled, SEED);
        assert_eq!(th.fed, 54);
        assert_eq!(th.burns, 7);
        assert_eq!(th.checks, 95);
        assert_eq!(th.throttled_events, 418);
        assert_eq!(th.throttled_irregular, 220);
        assert_eq!(th.throttled_steady, 198);
        assert_eq!(th.throttled_acquittals, 109);
        assert_eq!(th.throttled_learners, 14);
        assert_eq!(th.packets_sent, 63);
        assert_eq!(th.packets_suppressed, 32);
    }

    #[test]
    fn paired_and_unshoved_totals_pinned() {
        let (_, pa) = run_room(Room::Paired, SEED);
        assert_eq!(pa.fed, 54);
        assert_eq!(pa.burns, 10);
        assert_eq!(pa.checks, 95);
        assert_eq!(pa.paired_events, 662);
        assert_eq!(pa.paired_irregular, 318);
        assert_eq!(pa.paired_steady, 344);
        assert_eq!(pa.paired_acquittals, 184);
        assert_eq!(pa.paired_learners, 11);
        assert_eq!(pa.shove_ticks_paid, 9750);
        let (_, un) = run_room(Room::Unshoved, SEED);
        assert_eq!(un.fed, 57);
        assert_eq!(un.burns, 16);
        assert_eq!(un.checks, 92);
        assert_eq!(un.unshoved_events, 634);
        assert_eq!(un.unshoved_acquittals, 156);
        assert_eq!(un.unshoved_learners, 11);
        assert_eq!(un.shove_ticks_paid, 0);
    }

    #[test]
    fn the_unshoved_room_is_the_old_evidence_room_exactly() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        let (_, un) = run_room(Room::Unshoved, SEED);
        assert_eq!(ev.fed, un.fed);
        assert_eq!(ev.burns, un.burns);
        assert_eq!(ev.checks, un.checks);
        assert_eq!(ev.evidence_events, un.unshoved_events);
        assert_eq!(ev.evidence_acquittals, un.unshoved_acquittals);
        assert_eq!(ev.evidence_learners, un.unshoved_learners);
    }

    #[test]
    fn the_debt_channel_carries_both_kinds() {
        let (_, sh) = run_room(Room::Shoved, SEED);
        assert!(sh.shoved_irregular > 0, "no irregular shoved packet delivered: {sh:?}");
        assert!(sh.shoved_steady > 0, "no steady shoved packet delivered: {sh:?}");
        assert_eq!(sh.shoved_irregular + sh.shoved_steady, sh.shoved_events);
    }

    #[test]
    fn the_debt_is_actually_paid_and_teaches_nothing() {
        let (_, sh) = run_room(Room::Shoved, SEED);
        assert!(sh.shove_ticks_paid > 0, "no debt was ever paid in the shoved room: {sh:?}");
        assert_eq!(sh.shoved_acquittals, 0, "the shoved room acquitted somebody: {sh:?}");
        assert!(sh.shoved_first > 0, "no moth's first shove arrived by packet: {sh:?}");
    }

    #[test]
    fn shoved_totals_pinned() {
        let (_, sh) = run_room(Room::Shoved, SEED);
        assert_eq!(sh.fed, 49);
        assert_eq!(sh.burns, 16);
        assert_eq!(sh.checks, 104);
        assert_eq!(sh.shoved_events, 728);
        assert_eq!(sh.shoved_irregular, 411);
        assert_eq!(sh.shoved_steady, 317);
        assert_eq!(sh.shoved_acquittals, 0);
        assert_eq!(sh.shoved_first, 19);
        assert_eq!(sh.shove_ticks_paid, 9750);
    }

    #[test]
    fn the_shoved_room_still_feeds_lives_and_burns() {
        let (_, sh) = run_room(Room::Shoved, SEED);
        assert!(sh.fed > 0, "shoved room never fed: {sh:?}");
        assert!(sh.burns > 0, "shoved room never burned: {sh:?}");
        assert_eq!(sh.alive_end, 24, "shoved room lost a moth: {sh:?}");
    }

    #[test]
    fn fading_packets_travel_in_both_kinds_in_each_fading_room() {
        for room in [Room::FadingTaught, Room::FadingEarned, Room::FadingBoth] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fading_irregular > 0, "no irregular fading packet in {room:?}: {t:?}");
            assert!(t.fading_steady > 0, "no steady fading packet in {room:?}: {t:?}");
            assert_eq!(t.fading_irregular + t.fading_steady, t.fading_events);
            assert!(t.fading_learners > 0, "no moth first learned from a fading packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn night_one_is_the_evidence_night_in_each_fading_room() {
        let (ev_n, _) = run_room(Room::Evidence, SEED);
        for room in [Room::FadingTaught, Room::FadingEarned, Room::FadingBoth] {
            let (n, _) = run_room(room, SEED);
            assert_eq!(n[0].fed, ev_n[0].fed, "{room:?} night 1 fed differs");
            assert_eq!(n[0].burns, ev_n[0].burns, "{room:?} night 1 burns differ");
            assert_eq!(n[0].checks, ev_n[0].checks, "{room:?} night 1 checks differ");
            assert_eq!(n[0].refusals, ev_n[0].refusals, "{room:?} night 1 refusals differ");
            assert_eq!(n[0].expired_taught + n[0].expired_earned, 0, "a dawn tax fired inside night 1 in {room:?}");
        }
    }

    #[test]
    fn the_dawn_tax_fires_by_source_exactly() {
        let (_, ft) = run_room(Room::FadingTaught, SEED);
        assert!(ft.expired_taught > 0, "taught pool never taxed in fading-taught: {ft:?}");
        assert_eq!(ft.expired_earned, 0, "earned pool taxed in fading-taught: {ft:?}");
        let (_, fe) = run_room(Room::FadingEarned, SEED);
        assert_eq!(fe.expired_taught, 0, "taught pool taxed in fading-earned: {fe:?}");
        assert!(fe.expired_earned > 0, "earned pool never taxed in fading-earned: {fe:?}");
        let (_, fb) = run_room(Room::FadingBoth, SEED);
        assert!(fb.expired_taught > 0, "taught pool never taxed in fading-both: {fb:?}");
        assert!(fb.expired_earned > 0, "earned pool never taxed in fading-both: {fb:?}");
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert_eq!(ev.expired_taught + ev.expired_earned, 0, "the evidence room was taxed: {ev:?}");
    }

    #[test]
    fn fading_totals_pinned() {
        let (_, ft) = run_room(Room::FadingTaught, SEED);
        assert_eq!(ft.fed, 55);
        assert_eq!(ft.burns, 13);
        assert_eq!(ft.checks, 92);
        assert_eq!(ft.fading_events, 633);
        assert_eq!(ft.fading_irregular, 315);
        assert_eq!(ft.fading_steady, 318);
        assert_eq!(ft.fading_acquittals, 128);
        assert_eq!(ft.fading_learners, 11);
        assert_eq!(ft.expired_taught, 99);
        assert_eq!(ft.expired_earned, 0);
        let (_, fe) = run_room(Room::FadingEarned, SEED);
        assert_eq!(fe.fed, 59);
        assert_eq!(fe.burns, 15);
        assert_eq!(fe.checks, 91);
        assert_eq!(fe.fading_events, 631);
        assert_eq!(fe.fading_irregular, 284);
        assert_eq!(fe.fading_steady, 347);
        assert_eq!(fe.fading_acquittals, 143);
        assert_eq!(fe.fading_learners, 11);
        assert_eq!(fe.expired_taught, 0);
        assert_eq!(fe.expired_earned, 19);
        let (_, fb) = run_room(Room::FadingBoth, SEED);
        assert_eq!(fb.fed, 54);
        assert_eq!(fb.burns, 14);
        assert_eq!(fb.checks, 87);
        assert_eq!(fb.fading_events, 596);
        assert_eq!(fb.fading_irregular, 311);
        assert_eq!(fb.fading_steady, 285);
        assert_eq!(fb.fading_acquittals, 124);
        assert_eq!(fb.fading_learners, 11);
        assert_eq!(fb.expired_taught, 158);
        assert_eq!(fb.expired_earned, 18);
    }

    #[test]
    fn the_fading_rooms_still_feed_live_and_burn() {
        for room in [Room::FadingTaught, Room::FadingEarned, Room::FadingBoth] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }
}
