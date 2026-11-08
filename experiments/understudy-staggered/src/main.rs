//! # Understudy Staggered \U0001F3AD
//!
//! Brought by The Understudy \U0001F3AD. Principal:
//! `experiments/staggered-moths` (The Moth Keeper \U0001FA94),
//! which took `pooled-moths`' closing seed verbatim -- both
//! pools at 750, the taught pool's clock offset by 375 -- and
//! expired BOTH pools at lifetime 750 on the dawn-crossing
//! global clock of the FDTD cave, one or both pools carrying a
//! +375-tick offset on their expiry deadline. Its ledger:
//! staggered (taught deadline +375) fed 50, burns 9, checks
//! 116, events 611 (224 irregular / 387 steady), learners 18,
//! acquittals 145, expired 55 taught / 15 earned;
//! staggered-both (the coincident control, both deadlines
//! +375) fed 45, burns 13, checks 128, events 836 (266 / 570),
//! learners 18, acquittals 183, expired 58 / 19 -- against
//! waning-750's fed 48, burns 16, expired 58 / 27. Its P1, P2,
//! P5, P6 HELD (P2 exact, asserted); its P3 HELD past its line
//! (9, the best waning-family table in the cave); its P4
//! CRACKED and priced the crack: the control burns 13, not
//! >= 15 -- duration alone buys back 3 of the 7 burns
//! (16 -> 13), and staggering at the same longest deadline
//! buys back 4 more (13 -> 9). The ordering is the law:
//! staggered 9 < coincident-late 13 < coincident-750 16.
//!
//! The Understudy performs that stagger on the bus machinery
//! of `understudy-pooled`: no field, no grid, no positions, no
//! flight. Caution arrives by claim queue and is booked by
//! source in kind; each unit carries its gain tick on its
//! OWNER's personal clock, and expiry is discovered only at
//! claim tops -- this stage's own discovery (`understudy-
//! waning`) is that time here moves in claim-sized steps of
//! roughly 450 owner-ticks, so two lifetimes below one step
//! are one lifetime (waning-375 == waning-150, field for
//! field). The principal's stagger is 375 ticks -- BELOW one
//! claim-step. The eighteen inherited rooms are the queue
//! script unchanged, pinned exact by tests. The two new rooms
//! run this stage's waning-750 verbatim, differing only in
//! the offsets laid on the two deadlines:
//!
//! - **staggered** -- taught units expire at gain + 750 + 375
//!   owner-ticks; earned units at gain + 750. On paper the
//!   two forgettings land 375 apart; whether the stage's
//!   claim tops can SEE a 375-tick gap is the performance.
//! - **staggered-both** -- both pools expire at gain + 1125:
//!   the deadlines stay coincident, only the duration grows
//!   (this stage's waning-1500 neighbourhood, performed as
//!   1125 on both pools). The control, as the principal
//!   built it: if it matches staggered, duration -- not the
//!   stagger -- takes the credit on this stage too.
//!
//! This stage has already inverted the principal's family
//! bend twice: waning-750 is its safest waning room (fed 55,
//! burns 12, checks 97, events 671, learners 11, acquittals
//! 161, expired 62 taught / 16 earned), and its pooled split
//! collapsed onto existing rooms (pooled-taught == waning-750
//! in every headline; pooled-earned == waning-1500). The
//! question the swap puts to the principal: does the ordering
//! staggered < coincident-late < coincident-750 survive on a
//! stage whose step is wider than the stagger -- or do all
//! three rooms collapse onto one table, the deadlines
//! differing on paper while the claim tops discover every
//! expiry at the same visit?
//!
//! Tolerances and predictions were declared in the README
//! *before* this crate was built or run. The headless ledger
//! prints the principal's numbers beside this performance's
//! and names each line HELD or CRACKED.
//!
//! `cargo run -p understudy-staggered -- --headless` prints
//! the comparison ledger. There is no TUI; --headless is
//! handled before anything else regardless.

use std::collections::VecDeque;
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
    Waning1500,
    Waning750,
    Waning375,
    Waning150,
    PooledTaught,
    PooledEarned,
    Staggered,
    StaggeredBoth,
}

impl Room {
    /// The lifetime of a caution unit in a waning room, in
    /// ticks of its owner's personal clock; `None` elsewhere.
    fn waning_lifetime(self) -> Option<u64> {
        match self {
            Room::Waning1500 => Some(1500),
            Room::Waning750 => Some(750),
            Room::Waning375 => Some(375),
            Room::Waning150 => Some(150),
            Room::PooledTaught => Some(750),
            Room::PooledEarned => Some(750),
            Room::Staggered => Some(750),
            Room::StaggeredBoth => Some(750),
            _ => None,
        }
    }

    /// The taught pool's expiry-deadline offset, in
    /// owner-ticks: +375 in both staggered rooms, zero
    /// everywhere else.
    fn taught_offset(self) -> u64 {
        match self {
            Room::Staggered | Room::StaggeredBoth => 375,
            _ => 0,
        }
    }

    /// The earned pool's expiry-deadline offset, in
    /// owner-ticks: +375 only in the staggered-both control,
    /// zero elsewhere.
    fn earned_offset(self) -> u64 {
        match self {
            Room::StaggeredBoth => 375,
            _ => 0,
        }
    }

    /// Whether the TAUGHT pool expires by age in this room:
    /// every waning room, plus pooled-taught; never
    /// pooled-earned (its taught units persist forever).
    fn expires_taught(self) -> bool {
        self.waning_lifetime().is_some() && self != Room::PooledEarned
    }

    /// Whether the EARNED pool expires by age in this room:
    /// every waning room, plus pooled-earned; never
    /// pooled-taught (its earned units persist forever).
    fn expires_earned(self) -> bool {
        self.waning_lifetime().is_some() && self != Room::PooledTaught
    }
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
    WaningPacket,
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
    waning_events: usize,
    waning_irregular: usize,
    waning_steady: usize,
    waning_acquittals: usize,
    new_waning_learners: usize,
    waning_expired_taught: usize,
    waning_expired_earned: usize,
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
    waning_events: usize,
    waning_irregular: usize,
    waning_steady: usize,
    waning_acquittals: usize,
    waning_learners: usize,
    waning_expired_taught: usize,
    waning_expired_earned: usize,
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
    /// Gain ticks (owner's personal clock) of the earned
    /// caution units currently held, oldest first. Inert
    /// outside the waning rooms, where units are only pushed
    /// in those rooms. Invariant in a waning room:
    /// `caution` == earned_units.len() + taught_units.len().
    earned_units: VecDeque<u64>,
    /// Gain ticks of the taught caution units currently held,
    /// oldest first (waning rooms only).
    taught_units: VecDeque<u64>,
    /// This moth's personal clock: ticks its own turns have
    /// consumed, at most one night per night, running across
    /// dawns. Inert outside the waning rooms.
    clock: u64,
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
            earned_units: VecDeque::new(),
            taught_units: VecDeque::new(),
            clock: 0,
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
        if room.waning_lifetime().is_some() {
            // A delivered burn is earned caution, booked with
            // its gain tick on the recipient's personal clock.
            let gain = m.clock;
            m.earned_units.push_back(gain);
        }
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

/// Expire one moth's aged units on its personal clock: pop
/// from the front of each pool's queue while the front unit's
/// age has reached the lifetime. The loss is booked into the
/// discovering night's waning ledger.
fn expire_units(m: &mut Moth, now: u64, lifetime: u64, room: Room, ledger: &mut NightLedger) {
    while let Some(&g) = m.taught_units.front() {
        if !room.expires_taught() { break; }
        if now >= g + lifetime + room.taught_offset() {
            m.taught_units.pop_front();
            m.taught_caution = m.taught_caution.saturating_sub(1);
            m.caution = m.caution.saturating_sub(1);
            ledger.waning_expired_taught += 1;
        } else {
            break;
        }
    }
    while let Some(&g) = m.earned_units.front() {
        if !room.expires_earned() { break; }
        if now >= g + lifetime + room.earned_offset() {
            m.earned_units.pop_front();
            m.caution = m.caution.saturating_sub(1);
            ledger.waning_expired_earned += 1;
        } else {
            break;
        }
    }
}

/// Deliver one waning packet: identical re-judgement to
/// `broadcast_fading` -- irregular earns caution, booked to the
/// taught pool AND as a unit carrying its gain tick on the
/// recipient's personal clock; steady can acquit hearsay
/// caution, spending taught first and popping that unit's tick
/// with it (if no taught unit remains, the oldest earned unit
/// is spent). The clock, not the dawn, is this room's trait.
fn broadcast_waning(
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
        ledger.waning_events += 1;
        if irregular {
            ledger.waning_irregular += 1;
            if m.first_source.is_none() {
                m.first_source = Some(Source::WaningPacket);
                ledger.new_waning_learners += 1;
            }
            m.caution += 1;
            m.taught_caution += 1;
            let gain = m.clock;
            m.taught_units.push_back(gain);
        } else {
            ledger.waning_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                if m.taught_caution > 0 {
                    m.taught_caution -= 1;
                    m.taught_units.pop_front();
                } else {
                    m.earned_units.pop_front();
                }
                ledger.waning_acquittals += 1;
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
        // Waning dawn discovery: no dawn tax fires here -- but
        // a unit whose age ran out while its owner slept is
        // discovered at the dawn on its owner's personal clock
        // and booked into the new night, exactly where the
        // fading rooms book their tax.
        if night_idx > 0 {
            if let Some(lifetime) = room.waning_lifetime() {
                for m in moths.iter_mut() {
                    if m.dead {
                        continue;
                    }
                    let now = m.clock;
                    expire_units(m, now, lifetime, room, &mut ledger);
                }
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
            let turn_clock_base: u64 = moths[i].clock;
            let mut t: u64 = moths[i].shove_debt;
            moths[i].shove_debt = 0;
            ledger.shove_ticks_paid += t;
            let mut settled = false;
            while !settled && t < NIGHT_TICKS {
                // Waning: the acting moth's units age on its
                // own clock as its turn consumes ticks; expiry
                // is discovered at the top of each claim.
                if let Some(lifetime) = room.waning_lifetime() {
                    let now = turn_clock_base + t;
                    expire_units(&mut moths[i], now, lifetime, room, &mut ledger);
                }
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
                            if room.waning_lifetime().is_some() {
                                moths[i].earned_units.push_back(turn_clock_base + t);
                            }
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
                if room == Room::Evidence || room == Room::Paired || room == Room::Unshoved || room == Room::Shoved || room == Room::FadingTaught || room == Room::FadingEarned || room == Room::FadingBoth || room.waning_lifetime().is_some() {
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
                            if room.waning_lifetime().is_some() {
                                moths[i].earned_units.push_back(turn_clock_base + t);
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
                    }
                }
            }
            if room.waning_lifetime().is_some() {
                moths[i].clock = turn_clock_base + t.min(NIGHT_TICKS);
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
            if room.waning_lifetime().is_some() {
                for packet in &posted_packets {
                    broadcast_waning(i, packet, &mut moths, &mut rng, &mut ledger);
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
        totals.waning_events += l.waning_events;
        totals.waning_irregular += l.waning_irregular;
        totals.waning_steady += l.waning_steady;
        totals.waning_acquittals += l.waning_acquittals;
        totals.waning_learners += l.new_waning_learners;
        totals.waning_expired_taught += l.waning_expired_taught;
        totals.waning_expired_earned += l.waning_expired_earned;
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

fn print_waning_detail(name: &str, nights: &[NightLedger], totals: &RoomTotals) {
    for (n, l) in nights.iter().enumerate() {
        println!(
            "    waning detail '{name}' night {}: waning events {} (irregular {}, steady {}, acquittals {}, learners {}), expired taught {}, expired earned {}",
            n + 1, l.waning_events, l.waning_irregular, l.waning_steady,
            l.waning_acquittals, l.new_waning_learners,
            l.waning_expired_taught, l.waning_expired_earned
        );
    }
    println!(
        "    waning detail '{name}' totals: waning events {} (irregular {}, steady {}, acquittals {}, learners {}), expired taught {}, expired earned {}",
        totals.waning_events, totals.waning_irregular, totals.waning_steady,
        totals.waning_acquittals, totals.waning_learners,
        totals.waning_expired_taught, totals.waning_expired_earned
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
    let (w15_n, w15) = run_room(Room::Waning1500, SEED);
    let (w75_n, w75) = run_room(Room::Waning750, SEED);
    let (w37_n, w37) = run_room(Room::Waning375, SEED);
    let (w1_n, w1) = run_room(Room::Waning150, SEED);
    let (pt_n, pt) = run_room(Room::PooledTaught, SEED);
    let (pe_n, pe) = run_room(Room::PooledEarned, SEED);
    let (st_n, st) = run_room(Room::Staggered, SEED);
    let (sb_n, sb) = run_room(Room::StaggeredBoth, SEED);
    println!("understudy-staggered headless: the Staggered role, performed by queues \u{2014} no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX}; bus delivery burn {BURN_DELIVERY}, refusal {REFUSAL_DELIVERY}, evidence {EVIDENCE_DELIVERY}"
    );
    println!("  principal's ledger (waning-moths, cave): evidence fed 43 burns 12, checks 151 (packets 943; learners 18, acquittals 223); waning-1500 fed 43 burns 12, checks 151 (events 943: irr 342, steady 601; learners 18, acquittals 222; expired taught 42, earned 20); waning-750 fed 48 burns 16, checks 103 (events 502; learners 17, acquittals 105; expired taught 58, earned 27); waning-375 fed 53 burns 11, checks 68 (events 256; learners 16, acquittals 59; expired taught 50, earned 28); waning-150 fed 52 burns 11, checks 10 (events 49; learners 10, acquittals 4; expired taught 24, earned 25); fading-both (cave) burns 15");
    println!("  parent performance (understudy-fading, this stage): fading-taught fed 55 burns 13 (expired taught 99, earned 0); fading-earned fed 59 burns 15 (expired 0 / 19); fading-both fed 54 burns 14, checks 87 (events 596; expired taught 158, earned 18)");
    println!("  principal's ledger (pooled-moths, cave): waning-750 fed 48 burns 16, checks 103 (events 502; learners 17, acquittals 105; expired taught 58, earned 27); pooled-taught fed 43 burns 10, checks 134 (events 860: irr 324, steady 536; learners 18, acquittals 210; expired taught 111, earned 0); pooled-earned fed 54 burns 12, checks 110 (events 644: irr 202, steady 442; learners 18, acquittals 147; expired taught 0, earned 20); P3 CRACKED there \u{2014} neither pool carries the danger");
    println!("  this stage's waning-750 (pinned): fed 55 burns 12, checks 97 (events 671; learners 11, acquittals 161; expired taught 62, earned 16) \u{2014} the safest waning room on this stage");
    println!("  principal's ledger (staggered-moths, cave): waning-750 fed 48 burns 16 (expired taught 58, earned 27); staggered fed 50 burns 9, checks 116 (events 611: irr 224, steady 387; learners 18, acquittals 145; expired taught 55, earned 15); staggered-both fed 45 burns 13, checks 128 (events 836: irr 266, steady 570; learners 18, acquittals 183; expired taught 58, earned 19); ordering staggered 9 < coincident-late 13 < coincident-750 16; P4 CRACKED there \u{2014} duration priced at 3 burns, the stagger at 4");
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
    print_room("waning-1500 (evidence teaching; every caution unit expires 1500 ticks of its owner's time after it is gained; no dawn tax)", &w15_n, &w15);
    print_waning_detail("waning-1500", &w15_n, &w15);
    print_room("waning-750 (evidence teaching; unit lifetime 750 owner-ticks; no dawn tax)", &w75_n, &w75);
    print_waning_detail("waning-750", &w75_n, &w75);
    print_room("waning-375 (evidence teaching; unit lifetime 375 owner-ticks; no dawn tax)", &w37_n, &w37);
    print_waning_detail("waning-375", &w37_n, &w37);
    print_room("waning-150 (evidence teaching; unit lifetime 150 owner-ticks; no dawn tax)", &w1_n, &w1);
    print_waning_detail("waning-150", &w1_n, &w1);
    print_room("pooled-taught (evidence teaching; TAUGHT units expire 750 owner-ticks after gain; earned units never expire; no dawn tax)", &pt_n, &pt);
    print_waning_detail("pooled-taught", &pt_n, &pt);
    print_room("pooled-earned (evidence teaching; EARNED units expire 750 owner-ticks after gain; taught units never expire; no dawn tax)", &pe_n, &pe);
    print_waning_detail("pooled-earned", &pe_n, &pe);
    print_room("staggered (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +375; no dawn tax)", &st_n, &st);
    print_waning_detail("staggered", &st_n, &st);
    print_room("staggered-both (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +375, coincident control; no dawn tax)", &sb_n, &sb);
    print_waning_detail("staggered-both", &sb_n, &sb);
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
        "    P1 the channel works waning, in both kinds, in each waning room: 1500 irr {} / steady {} learners {} [{}]; 750 irr {} / steady {} learners {} [{}]; 375 irr {} / steady {} learners {} [{}]; 150 irr {} / steady {} learners {} [{}] (principal irr/steady 342/601, 160/342, 106/150, 24/25; learners 18/17/16/10)",
        w15.waning_irregular, w15.waning_steady, w15.waning_learners,
        verdict(w15.waning_irregular > 0 && w15.waning_steady > 0 && w15.waning_learners > 0),
        w75.waning_irregular, w75.waning_steady, w75.waning_learners,
        verdict(w75.waning_irregular > 0 && w75.waning_steady > 0 && w75.waning_learners > 0),
        w37.waning_irregular, w37.waning_steady, w37.waning_learners,
        verdict(w37.waning_irregular > 0 && w37.waning_steady > 0 && w37.waning_learners > 0),
        w1.waning_irregular, w1.waning_steady, w1.waning_learners,
        verdict(w1.waning_irregular > 0 && w1.waning_steady > 0 && w1.waning_learners > 0)
    );
    println!(
        "    P2 the clock doses: expired totals 1500={} 750={} 375={} 150={} [{} strictly 150 > 375 > 750 > 1500 > 0] (principal CRACKED: 62 / 85 / 78 / 49, not monotone)",
        w15.waning_expired_taught + w15.waning_expired_earned,
        w75.waning_expired_taught + w75.waning_expired_earned,
        w37.waning_expired_taught + w37.waning_expired_earned,
        w1.waning_expired_taught + w1.waning_expired_earned,
        verdict((w1.waning_expired_taught + w1.waning_expired_earned) > (w37.waning_expired_taught + w37.waning_expired_earned)
            && (w37.waning_expired_taught + w37.waning_expired_earned) > (w75.waning_expired_taught + w75.waning_expired_earned)
            && (w75.waning_expired_taught + w75.waning_expired_earned) > (w15.waning_expired_taught + w15.waning_expired_earned)
            && (w15.waning_expired_taught + w15.waning_expired_earned) > 0)
    );
    println!(
        "    P3 claim A: waning-150 burns {} > this stage's evidence burns {} [{}] (principal CRACKED: 11, not above its evidence 12)",
        w1.burns, ev.burns, verdict(w1.burns > ev.burns)
    );
    println!(
        "    P4 claim B: waning-1500 burns {} <= this stage's fading-both burns {} [{}] (principal HELD: 12 <= its fading-both 15)",
        w15.burns, fb.burns, verdict(w15.burns <= fb.burns)
    );
    println!(
        "    P5 the dose claim: burns150 {} >= burns375 {} >= burns750 {} >= burns1500 {} [{}] (principal CRACKED: 11 / 11 / 16 / 12 -- the curve bent in the middle)",
        w1.burns, w37.burns, w75.burns, w15.burns,
        verdict(w1.burns >= w37.burns && w37.burns >= w75.burns && w75.burns >= w15.burns)
    );
    println!(
        "    P6 a waning moth still eats: waning fed 1500={} 750={} 375={} 150={} [{} each >= 50] (principal declared >= 40 and held 43/48/53/52; queue evidence precedent fed 57)",
        w15.fed, w75.fed, w37.fed, w1.fed,
        verdict(w15.fed >= 50 && w75.fed >= 50 && w37.fed >= 50 && w1.fed >= 50)
    );
    println!(
        "    P7 the bend, pre-declared here from the principal's finding: waning-750 burns {} are the maximum of the four waning rooms (1500={} 375={} 150={}) [{}] (principal, post-hoc: 16 against 12/11/11)",
        w75.burns, w15.burns, w37.burns, w1.burns,
        verdict(w75.burns >= w15.burns && w75.burns >= w37.burns && w75.burns >= w1.burns)
    );
    println!(
        "    P8 the clock's alibi: waning-1500 night 1 expired taught {} / earned {} [{} both zero -- no unit is 1500 owner-ticks old inside its owner's first night] (principal HELD exactly, asserted by test)",
        w15_n[0].waning_expired_taught, w15_n[0].waning_expired_earned,
        verdict(w15_n[0].waning_expired_taught == 0 && w15_n[0].waning_expired_earned == 0)
    );
    println!(
        "    P1 (pooled) the channel works pooled, in both kinds, in each pooled room: taught irr {} / steady {} learners {} [{}]; earned irr {} / steady {} learners {} [{}] (principal 324/536 and 202/442; learners 18/18)",
        pt.waning_irregular, pt.waning_steady, pt.waning_learners,
        verdict(pt.waning_irregular > 0 && pt.waning_steady > 0 && pt.waning_learners > 0),
        pe.waning_irregular, pe.waning_steady, pe.waning_learners,
        verdict(pe.waning_irregular > 0 && pe.waning_steady > 0 && pe.waning_learners > 0)
    );
    println!(
        "    P2 (pooled) the split is exact: pooled-taught expired taught {} / earned {} [{} taught > 0, earned = 0]; pooled-earned expired taught {} / earned {} [{} taught = 0, earned > 0] (principal HELD exactly, asserted; 111/0 and 0/20)",
        pt.waning_expired_taught, pt.waning_expired_earned,
        verdict(pt.waning_expired_taught > 0 && pt.waning_expired_earned == 0),
        pe.waning_expired_taught, pe.waning_expired_earned,
        verdict(pe.waning_expired_taught == 0 && pe.waning_expired_earned > 0)
    );
    println!(
        "    P3 (pooled) the coincidence claim, pre-declared from the principal's finding: BOTH pooled rooms burn at or below this stage's waning-750 ({}): pooled-taught {} [{}], pooled-earned {} [{}] (principal: 10 and 12, both below its waning-750's 16 \u{2014} splitting the expiry never reproduces the danger)",
        w75.burns, pt.burns, verdict(pt.burns <= w75.burns), pe.burns, verdict(pe.burns <= w75.burns)
    );
    println!(
        "    P4 (pooled) neither pooled room starves the channel: pooled-taught events {} [{} >= 300], pooled-earned events {} [{} >= 300] (principal HELD: 860 / 644; this stage's waning-750 posted 671)",
        pt.waning_events, verdict(pt.waning_events >= 300),
        pe.waning_events, verdict(pe.waning_events >= 300)
    );
    println!(
        "    P5 (pooled) a split moth still eats: pooled-taught fed {} [{} >= 50], pooled-earned fed {} [{} >= 50] (principal declared >= 43 against its evidence 43 and held 43/54; queue evidence precedent fed 57)",
        pt.fed, verdict(pt.fed >= 50), pe.fed, verdict(pe.fed >= 50)
    );
    println!(
        "    P6 (pooled) the split has not yet had time to differ in night 1: pooled-taught night 1 fed {} burns {} vs pooled-earned night 1 fed {} burns {} [{} equal in both] (principal observed: both rooms share night 1 at 19/5 \u{2014} but this stage discovers expiry at claim tops on personal clocks, where a 750 expiry CAN land inside night 1)",
        pt_n[0].fed, pt_n[0].burns, pe_n[0].fed, pe_n[0].burns,
        verdict(pt_n[0].fed == pe_n[0].fed && pt_n[0].burns == pe_n[0].burns)
    );
    println!(
        "    P1 (staggered) the channel works staggered, in both kinds, in each new room: staggered irr {} / steady {} learners {} [{}]; staggered-both irr {} / steady {} learners {} [{}] (principal 224/387 and 266/570; learners 18/18)",
        st.waning_irregular, st.waning_steady, st.waning_learners,
        verdict(st.waning_irregular > 0 && st.waning_steady > 0 && st.waning_learners > 0),
        sb.waning_irregular, sb.waning_steady, sb.waning_learners,
        verdict(sb.waning_irregular > 0 && sb.waning_steady > 0 && sb.waning_learners > 0)
    );
    println!(
        "    P2 (staggered) the offset is real: both new rooms expire BOTH pools: staggered expired taught {} / earned {} [{} both > 0, no dawn tax {}]; staggered-both expired taught {} / earned {} [{} both > 0, no dawn tax {}] (principal HELD exactly, asserted; 55/15 and 58/19; offsets asserted by test)",
        st.waning_expired_taught, st.waning_expired_earned,
        verdict(st.waning_expired_taught > 0 && st.waning_expired_earned > 0), verdict(st.expired_taught + st.expired_earned == 0),
        sb.waning_expired_taught, sb.waning_expired_earned,
        verdict(sb.waning_expired_taught > 0 && sb.waning_expired_earned > 0), verdict(sb.expired_taught + sb.expired_earned == 0)
    );
    println!(
        "    P3 (staggered) the coincidence claim, performed: staggered burns {} at or below this stage's waning-750 ({}) [{}] (principal: staggered 9, seven below its waning-750's 16 \u{2014} the stagger buys back or holds the safest room's table)",
        st.burns, w75.burns, verdict(st.burns <= w75.burns)
    );
    println!(
        "    P4 (staggered) the control discriminates: staggered-both burns {} >= staggered burns {} [{}] (principal: control 13 above staggered 9 \u{2014} coincident deadlines at the same longest deadline never beat staggered ones; equality is this stage's collapse signature, kept as a pass only if P3 also holds)",
        sb.burns, st.burns, verdict(sb.burns >= st.burns)
    );
    println!(
        "    P5 (staggered) neither new room starves the channel: staggered events {} [{} >= 300], staggered-both events {} [{} >= 300] (principal HELD: 611 / 836; this stage's waning-750 posted 671)",
        st.waning_events, verdict(st.waning_events >= 300),
        sb.waning_events, verdict(sb.waning_events >= 300)
    );
    println!(
        "    P6 (staggered) a staggered moth still eats: staggered fed {} [{} >= 50], staggered-both fed {} [{} >= 50] (principal: 50 / 45, declared against its evidence 43; queue evidence precedent fed 57)",
        st.fed, verdict(st.fed >= 50), sb.fed, verdict(sb.fed >= 50)
    );
    println!(
        "    honest refusals across all rooms {} (counted, not a declared line on this stage; queue precedent 0)",
        solo.honest_refusals + wit.honest_refusals + chain.honest_refusals + placed.honest_refusals + ev.honest_refusals + th.honest_refusals + un.honest_refusals + pa.honest_refusals + sh.honest_refusals + ft.honest_refusals + fe.honest_refusals + fb.honest_refusals + w15.honest_refusals + w75.honest_refusals + w37.honest_refusals + w1.honest_refusals + pt.honest_refusals + pe.honest_refusals + st.honest_refusals + sb.honest_refusals
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
        for room in [Room::Solo, Room::Witness, Room::Chain, Room::Placed, Room::Evidence, Room::Throttled, Room::Paired, Room::Unshoved, Room::Shoved, Room::FadingTaught, Room::FadingEarned, Room::FadingBoth, Room::Waning1500, Room::Waning750, Room::Waning375, Room::Waning150, Room::PooledTaught, Room::PooledEarned, Room::Staggered, Room::StaggeredBoth] {
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

    #[test]
    fn waning_packets_travel_in_both_kinds_in_each_waning_room() {
        for room in [Room::Waning1500, Room::Waning750, Room::Waning375, Room::Waning150] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular waning packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady waning packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a waning packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_waning_clock_fires_in_each_waning_room() {
        for room in [Room::Waning1500, Room::Waning750, Room::Waning375, Room::Waning150] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught + t.waning_expired_earned > 0, "no unit ever expired in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in a waning room {room:?}: {t:?}");
        }
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert_eq!(ev.waning_expired_taught + ev.waning_expired_earned, 0, "the evidence room waned: {ev:?}");
    }

    #[test]
    fn waning_1500_expires_nothing_inside_night_one() {
        let (n, _) = run_room(Room::Waning1500, SEED);
        assert_eq!(n[0].waning_expired_taught, 0, "a taught unit expired inside night 1: {:?}", n[0]);
        assert_eq!(n[0].waning_expired_earned, 0, "an earned unit expired inside night 1: {:?}", n[0]);
    }

    #[test]
    fn the_waning_performance_is_deterministic() {
        let (_, a) = run_room(Room::Waning750, SEED);
        let (_, b) = run_room(Room::Waning750, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn waning_totals_pinned() {
        let (_, w15) = run_room(Room::Waning1500, SEED);
        assert_eq!(w15.fed, 59);
        assert_eq!(w15.burns, 15);
        assert_eq!(w15.checks, 91);
        assert_eq!(w15.waning_events, 631);
        assert_eq!(w15.waning_irregular, 284);
        assert_eq!(w15.waning_steady, 347);
        assert_eq!(w15.waning_acquittals, 143);
        assert_eq!(w15.waning_learners, 11);
        assert_eq!(w15.waning_expired_taught, 11);
        assert_eq!(w15.waning_expired_earned, 7);
        let (_, w75) = run_room(Room::Waning750, SEED);
        assert_eq!(w75.fed, 55);
        assert_eq!(w75.burns, 12);
        assert_eq!(w75.checks, 97);
        assert_eq!(w75.waning_events, 671);
        assert_eq!(w75.waning_irregular, 349);
        assert_eq!(w75.waning_steady, 322);
        assert_eq!(w75.waning_acquittals, 161);
        assert_eq!(w75.waning_learners, 11);
        assert_eq!(w75.waning_expired_taught, 62);
        assert_eq!(w75.waning_expired_earned, 16);
        let (_, w37) = run_room(Room::Waning375, SEED);
        assert_eq!(w37.fed, 58);
        assert_eq!(w37.burns, 26);
        assert_eq!(w37.checks, 71);
        assert_eq!(w37.waning_events, 468);
        assert_eq!(w37.waning_irregular, 225);
        assert_eq!(w37.waning_steady, 243);
        assert_eq!(w37.waning_acquittals, 70);
        assert_eq!(w37.waning_learners, 11);
        assert_eq!(w37.waning_expired_taught, 103);
        assert_eq!(w37.waning_expired_earned, 31);
        let (_, w1) = run_room(Room::Waning150, SEED);
        assert_eq!(w1.fed, 58);
        assert_eq!(w1.burns, 26);
        assert_eq!(w1.checks, 71);
        assert_eq!(w1.waning_events, 468);
        assert_eq!(w1.waning_expired_taught, 103);
        assert_eq!(w1.waning_expired_earned, 31);
        // The 375 and 150 rooms are behaviourally identical on
        // this stage (see README): a claim step is ~450 ticks,
        // so both lifetimes expire every pre-turn unit at the
        // second claim of a turn. The pin keeps them so.
        assert_eq!(w37.burns, w1.burns);
        assert_eq!(w37.waning_events, w1.waning_events);
    }

    #[test]
    fn the_waning_rooms_still_feed_live_and_burn() {
        for room in [Room::Waning1500, Room::Waning750, Room::Waning375, Room::Waning150] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn pooled_packets_travel_in_both_kinds_in_each_pooled_room() {
        for room in [Room::PooledTaught, Room::PooledEarned] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular pooled packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady pooled packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a pooled packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_pooled_split_is_exact() {
        // P2, asserted: each pooled room expires its named
        // pool and ONLY its named pool, on the age clock; the
        // dawn tax never fires in either.
        let (_, pt) = run_room(Room::PooledTaught, SEED);
        assert!(pt.waning_expired_taught > 0, "taught pool never waned in pooled-taught: {pt:?}");
        assert_eq!(pt.waning_expired_earned, 0, "earned pool waned in pooled-taught: {pt:?}");
        assert_eq!(pt.expired_taught + pt.expired_earned, 0, "a dawn tax fired in pooled-taught: {pt:?}");
        let (_, pe) = run_room(Room::PooledEarned, SEED);
        assert!(pe.waning_expired_earned > 0, "earned pool never waned in pooled-earned: {pe:?}");
        assert_eq!(pe.waning_expired_taught, 0, "taught pool waned in pooled-earned: {pe:?}");
        assert_eq!(pe.expired_taught + pe.expired_earned, 0, "a dawn tax fired in pooled-earned: {pe:?}");
    }

    #[test]
    fn the_pooled_performance_is_deterministic() {
        let (_, a) = run_room(Room::PooledTaught, SEED);
        let (_, b) = run_room(Room::PooledTaught, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn pooled_totals_pinned() {
        // Measured at planting, pinned as found. P3 CRACKED:
        // pooled-earned burns 15, ABOVE this stage's waning-750
        // (12) -- and pooled-taught reproduces waning-750's
        // headline table exactly, while pooled-earned
        // reproduces waning-1500's.
        let (_, pt) = run_room(Room::PooledTaught, SEED);
        assert_eq!(pt.fed, 55);
        assert_eq!(pt.burns, 12);
        assert_eq!(pt.checks, 97);
        assert_eq!(pt.waning_events, 671);
        assert_eq!(pt.waning_learners, 11);
        assert_eq!(pt.waning_acquittals, 164);
        assert_eq!(pt.waning_expired_taught, 62);
        assert_eq!(pt.waning_expired_earned, 0);
        let (_, pe) = run_room(Room::PooledEarned, SEED);
        assert_eq!(pe.fed, 59);
        assert_eq!(pe.burns, 15);
        assert_eq!(pe.checks, 91);
        assert_eq!(pe.waning_events, 631);
        assert_eq!(pe.waning_learners, 11);
        assert_eq!(pe.waning_acquittals, 144);
        assert_eq!(pe.waning_expired_taught, 0);
        assert_eq!(pe.waning_expired_earned, 16);
    }

    #[test]
    fn staggered_packets_travel_in_both_kinds_in_each_staggered_room() {
        for room in [Room::Staggered, Room::StaggeredBoth] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular staggered packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady staggered packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a staggered packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn staggered_offsets_are_as_declared() {
        assert_eq!(Room::Staggered.taught_offset(), 375);
        assert_eq!(Room::Staggered.earned_offset(), 0);
        assert_eq!(Room::StaggeredBoth.taught_offset(), 375);
        assert_eq!(Room::StaggeredBoth.earned_offset(), 375);
        assert_eq!(Room::Waning750.taught_offset(), 0);
        assert_eq!(Room::Waning750.earned_offset(), 0);
        assert_eq!(Room::Staggered.waning_lifetime(), Some(750));
        assert_eq!(Room::StaggeredBoth.waning_lifetime(), Some(750));
    }

    #[test]
    fn staggered_both_pools_expire_and_no_dawn_tax() {
        // P2, asserted: each staggered room expires both pools
        // on the age clock; the dawn tax never fires in either.
        for room in [Room::Staggered, Room::StaggeredBoth] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_staggered_performance_is_deterministic() {
        let (_, a) = run_room(Room::Staggered, SEED);
        let (_, b) = run_room(Room::Staggered, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn staggered_totals_pinned() {
        // Measured at planting, pinned as found. The two rooms
        // are identical in every headline field and every
        // night; they differ ONLY in the earned expiry census
        // (15 vs 10) -- the collapse, pinned whole.
        let (_, st) = run_room(Room::Staggered, SEED);
        assert_eq!(st.fed, 54);
        assert_eq!(st.burns, 11);
        assert_eq!(st.checks, 100);
        assert_eq!(st.waning_events, 695);
        assert_eq!(st.waning_learners, 11);
        assert_eq!(st.waning_acquittals, 162);
        assert_eq!(st.waning_expired_taught, 48);
        assert_eq!(st.waning_expired_earned, 15);
        let (_, sb) = run_room(Room::StaggeredBoth, SEED);
        assert_eq!(sb.fed, 54);
        assert_eq!(sb.burns, 11);
        assert_eq!(sb.checks, 100);
        assert_eq!(sb.waning_events, 695);
        assert_eq!(sb.waning_learners, 11);
        assert_eq!(sb.waning_acquittals, 162);
        assert_eq!(sb.waning_expired_taught, 48);
        assert_eq!(sb.waning_expired_earned, 10);
    }

    #[test]
    fn the_staggered_rooms_still_feed_live_and_burn() {
        for room in [Room::Staggered, Room::StaggeredBoth] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn the_pooled_rooms_still_feed_live_and_burn() {
        for room in [Room::PooledTaught, Room::PooledEarned] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }
}
