//! # Understudy Nearwide \U0001F3AD
//!
//! Brought by The Understudy \U0001F3AD. Principal:
//! `experiments/understudy-facegrain` (this stage's own
//! last performance), whose closing seed this planting
//! takes up verbatim in substance -- the stagger at
//! +675 on the queue stage, midway between the edge
//! (+600) and the wide offset (+750): if the tax arrives
//! there at all, it arrives inside the last 150 ticks;
//! does staggered-675 read 10, 15, or the first interior
//! price this stage has ever posted, and does its taught
//! census fall with the price or ahead of it?
//!
//! The stage's ledger going in (pinned, this stage):
//! staggered-step (+450) burns 10, taught census 35,
//! acquittals 169; staggered-475 burns 10, census 35,
//! acquittals 169 (the step room entire); staggered-500
//! burns 10, census 33, acquittals 171; staggered-face
//! (+525) burns 10, census 32, acquittals 172;
//! staggered-edge (+600) burns 10, census 21,
//! acquittals 175; staggered-wide (+750) burns 15 (the
//! five-burn tax), census 11, acquittals 141. The mirror
//! side reads 12 at every width; coincident rooms have
//! followed their taught staggers entire at every width
//! and cracked their line against the mirror five
//! addresses running. So across the whole measured row
//! the price has moved exactly once -- somewhere in the
//! last 150 ticks between +600 and +750 -- while the
//! taught census has fallen the whole way: 35, 35, 33,
//! 32, 21, toward 11. +675 stands at the midpoint of
//! that last interval, the last unmeasured address
//! before the wide offset.
//!
//! The machinery is the claim-queue stage carried
//! whole: no field, no grid, no positions, no flight.
//! Caution is booked by source; each unit carries its
//! gain tick on its OWNER's personal clock, expiry
//! discovered only at claim tops. The thirty-nine
//! inherited rooms are the queue script unchanged,
//! pinned exact by tests. The three new rooms run this
//! stage's waning-750 verbatim, differing only in the
//! offset, set to +675 owner-ticks:
//!
//! - **staggered-675** -- taught expire at gain + 750 +
//!   675 owner-ticks; earned at gain + 750.
//! - **mirrored-675** -- earned expire at gain + 750 +
//!   675; taught at gain + 750. The mirror, near wide.
//! - **coincident-675** -- BOTH deadlines offset +675,
//!   coincident at the longest deadline (2175 owner-
//!   ticks); only duration changes. The control.
//!
//! Tolerances and predictions were declared in the
//! README *before* this crate was built or run. The
//! headless ledger prints the stage's pinned numbers
//! beside this performance's and names each line HELD
//! or CRACKED.
//!
//! `cargo run -p understudy-nearwide -- --headless`
//! prints the comparison ledger. There is no TUI.

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
    Mirrored,
    StaggeredWide,
    MirroredWide,
    CoincidentWide,
    StaggeredStep,
    MirroredStep,
    CoincidentStep,
    StaggeredEdge,
    MirroredEdge,
    CoincidentEdge,
    StaggeredFace,
    MirroredFace,
    CoincidentFace,
    Staggered475,
    Mirrored475,
    Coincident475,
    Staggered500,
    Mirrored500,
    Coincident500,
    Staggered675,
    Mirrored675,
    Coincident675,
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
            Room::Mirrored => Some(750),
            Room::StaggeredWide => Some(750),
            Room::MirroredWide => Some(750),
            Room::CoincidentWide => Some(750),
            Room::StaggeredStep => Some(750),
            Room::MirroredStep => Some(750),
            Room::CoincidentStep => Some(750),
            Room::StaggeredEdge => Some(750),
            Room::MirroredEdge => Some(750),
            Room::CoincidentEdge => Some(750),
            Room::StaggeredFace => Some(750),
            Room::MirroredFace => Some(750),
            Room::CoincidentFace => Some(750),
            Room::Staggered475 => Some(750),
            Room::Mirrored475 => Some(750),
            Room::Coincident475 => Some(750),
            Room::Staggered500 => Some(750),
            Room::Mirrored500 => Some(750),
            Room::Coincident500 => Some(750),
            Room::Staggered675 => Some(750),
            Room::Mirrored675 => Some(750),
            Room::Coincident675 => Some(750),
            _ => None,
        }
    }

    /// The taught pool's expiry-deadline offset, in
    /// owner-ticks: +375 in both staggered rooms, zero
    /// everywhere else.
    fn taught_offset(self) -> u64 {
        match self {
            Room::Staggered | Room::StaggeredBoth => 375,
            Room::StaggeredWide | Room::CoincidentWide => 750,
            Room::StaggeredStep | Room::CoincidentStep => 450,
            Room::StaggeredEdge | Room::CoincidentEdge => 600,
            Room::StaggeredFace | Room::CoincidentFace => 525,
            Room::Staggered475 | Room::Coincident475 => 475,
            Room::Staggered500 | Room::Coincident500 => 500,
            Room::Staggered675 | Room::Coincident675 => 675,
            _ => 0,
        }
    }

    /// The earned pool's expiry-deadline offset, in
    /// owner-ticks: +375 only in the staggered-both control,
    /// zero elsewhere.
    fn earned_offset(self) -> u64 {
        match self {
            Room::StaggeredBoth => 375,
            Room::Mirrored => 375,
            Room::MirroredWide | Room::CoincidentWide => 750,
            Room::MirroredStep | Room::CoincidentStep => 450,
            Room::MirroredEdge | Room::CoincidentEdge => 600,
            Room::MirroredFace | Room::CoincidentFace => 525,
            Room::Mirrored475 | Room::Coincident475 => 475,
            Room::Mirrored500 | Room::Coincident500 => 500,
            Room::Mirrored675 | Room::Coincident675 => 675,
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
    let (mi_n, mi) = run_room(Room::Mirrored, SEED);
    let (sw_n, sw) = run_room(Room::StaggeredWide, SEED);
    let (mw_n, mw) = run_room(Room::MirroredWide, SEED);
    let (cw_n, cw) = run_room(Room::CoincidentWide, SEED);
    let (ss_n, ss) = run_room(Room::StaggeredStep, SEED);
    let (ms_n, ms) = run_room(Room::MirroredStep, SEED);
    let (cs_n, cs) = run_room(Room::CoincidentStep, SEED);
    let (se_n, se) = run_room(Room::StaggeredEdge, SEED);
    let (me_n, me) = run_room(Room::MirroredEdge, SEED);
    let (ce_n, ce) = run_room(Room::CoincidentEdge, SEED);
    let (sfa_n, sfa) = run_room(Room::StaggeredFace, SEED);
    let (mfa_n, mfa) = run_room(Room::MirroredFace, SEED);
    let (cfa_n, cfa) = run_room(Room::CoincidentFace, SEED);
    let (s475_n, s475) = run_room(Room::Staggered475, SEED);
    let (m475_n, m475) = run_room(Room::Mirrored475, SEED);
    let (c475_n, c475) = run_room(Room::Coincident475, SEED);
    let (s500_n, s500) = run_room(Room::Staggered500, SEED);
    let (m500_n, m500) = run_room(Room::Mirrored500, SEED);
    let (c500_n, c500) = run_room(Room::Coincident500, SEED);
    let (s675_n, s675) = run_room(Room::Staggered675, SEED);
    let (m675_n, m675) = run_room(Room::Mirrored675, SEED);
    let (c675_n, c675) = run_room(Room::Coincident675, SEED);
    println!("understudy-nearwide headless: the Nearwide role, performed by queues \u{2014} no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX}; bus delivery burn {BURN_DELIVERY}, refusal {REFUSAL_DELIVERY}, evidence {EVIDENCE_DELIVERY}"
    );
    println!("  principal's ledger (understudy-facegrain, this stage): staggered-475 fed 54 burns 10, checks 101 (events 700: irr 366, steady 334; learners 11, acquittals 169; expired taught 35, earned 15); mirrored-475 fed 55 burns 12, checks 97 (events 671: irr 349, steady 322; learners 11, acquittals 162; expired taught 62, earned 11); coincident-475 fed 54 burns 10, checks 101 (events 700; learners 11, acquittals 169; expired taught 35, earned 10); staggered-500 burns 10, acquittals 171, census 33/15; mirrored-500 burns 12, census 62/11; coincident-500 burns 10, acquittals 171, census 33/10 \u{2014} P1, P2, P3, P4, P6, P7 HELD there; P5 CRACKED for the fourth and fifth times (the controls followed their taught staggers down below their mirrors)");
    println!("  this stage's taught line (pinned): staggered-step (+450) burns 10, census 35, acquittals 169; staggered-face (+525) burns 10, census 32, acquittals 172; staggered-edge (+600) burns 10, census 21, acquittals 175; staggered-wide (+750) burns 15 (the five-burn tax), census 11, acquittals 141; mirror side 12 at every width; waning-750 burns 12 \u{2014} +675 stands midway in the last interval, between the edge and the wide offset: does it read 10, 15, or the first interior price this stage has ever posted, and does its census fall with the price or ahead of it?");
    println!("  principal's ledger (facegrain-moths, cave): staggered-475 fed 48 burns 14, checks 141 (events 773: irr 291, steady 482; learners 18, acquittals 183; expired taught 46, earned 20); mirrored-475 fed 46 burns 11, checks 129 (events 662: irr 269, steady 393; learners 18, acquittals 162; expired taught 99, earned 12); coincident-475 fed 48 burns 14, checks 121 (events 585: irr 190, steady 395; learners 16, acquittals 141; expired taught 50, earned 18); staggered-500 fed 56 burns 16, checks 119 (events 656: irr 235, steady 421; learners 18, acquittals 155; expired taught 62, earned 18); mirrored-500 fed 46 burns 11, checks 129 (events 662: irr 269, steady 393; learners 18, acquittals 162; expired taught 99, earned 12); coincident-500 fed 46 burns 16, checks 140 (events 800: irr 267, steady 533; learners 18, acquittals 176; expired taught 71, earned 19) \u{2014} P1, P2, P4, P5, P6 HELD there; P3 CRACKED (staggered-475 burns 14, an intermediate price: the fall whole at 75-tick grain is a stair at 25-tick grain, 9 / 14 / 16 / 16, both falling lines sharing both steps) and P7 CRACKED (the grains disagree on both falling lines, 14 != 16: the interval has an interior)");
    println!("  this stage's taught line (pinned): staggered (+375) burns 11 collapsed; staggered-step (+450) burns 10, census 35; staggered-face (+525) burns 10, census 32; staggered-edge (+600) burns 10, census 21; staggered-wide (+750) burns 15 (five-burn tax); mirror side 12 at every width; waning-750 burns 12 \u{2014} +475 and +500 stand inside the interval's first half, between the step and the face: do they read 10 too, and do their censuses separate at all this close to the step?");
    println!("  principal's ledger (cliffface-moths, cave): staggered-face fed 56 burns 16, checks 119 (events 656: irr 235, steady 421; learners 18, acquittals 156; expired taught 61, earned 18); mirrored-face fed 46 burns 11, checks 129 (events 662: irr 269, steady 393; learners 18, acquittals 162; expired taught 99, earned 12); coincident-face fed 46 burns 16, checks 140 (events 800: irr 267, steady 533; learners 18, acquittals 177; expired taught 70, earned 19) \u{2014} ALL SIX HELD there: the face is EARLY and sheer, staggered-face already at the unstaggered 16 at +525, mirror gap -5, coincident face early too");
    println!("  this stage's taught line (pinned): staggered (+375) burns 11 collapsed; staggered-step (+450) burns 10; staggered-edge (+600) burns 10 (step price entire, census 21); staggered-wide (+750) burns 15 (five-burn tax); mirror side 12 at every width; waning-750 burns 12 \u{2014} +525 sits between step and edge, both 10: does the face hold, interior-price, or move only in census?");
    println!("  principal's ledger (shelfedge-moths, cave): staggered-edge fed 56 burns 16, checks 119 (events 656: irr 235, steady 421; learners 18, acquittals 159; expired taught 46, earned 18); mirrored-edge fed 46 burns 12, checks 124 (events 778: irr 253, steady 525; learners 17, acquittals 165; expired taught 89, earned 19); coincident-edge fed 46 burns 16, checks 140 (events 800: irr 267, steady 533; learners 18, acquittals 178; expired taught 60, earned 20) \u{2014} P1, P2, P6 HELD there; P3 CRACKED (staggered-edge 16, not 10..=12: the price cliffed past dissolution to waning-750's own 16), P4 CRACKED (mirror gap -4: two prices passing in the dark), P5 CRACKED (coincident-edge 16, not 12..=13: the cliff belongs to deadline 2100's neighbourhood)");
    println!("  this stage's taught line (pinned): staggered (+375) fed 54 burns 11 = staggered-both 11 (collapse); staggered-step (+450, one step) fed 54 burns 10, coincident-step beside it entire (the one-burn separation); staggered-wide (+750) fed 59 burns 15 = coincident-wide 15 (the five-burn tax); the mirror side reads 12 at +375, +450 and +750 alike, and waning-750 reads 12 \u{2014} +600 is between one step and two: where does the one-burn separation become the five-burn tax?");
    println!("  principal's ledger (waning-moths, cave): evidence fed 43 burns 12, checks 151 (packets 943; learners 18, acquittals 223); waning-1500 fed 43 burns 12, checks 151 (events 943: irr 342, steady 601; learners 18, acquittals 222; expired taught 42, earned 20); waning-750 fed 48 burns 16, checks 103 (events 502; learners 17, acquittals 105; expired taught 58, earned 27); waning-375 fed 53 burns 11, checks 68 (events 256; learners 16, acquittals 59; expired taught 50, earned 28); waning-150 fed 52 burns 11, checks 10 (events 49; learners 10, acquittals 4; expired taught 24, earned 25); fading-both (cave) burns 15");
    println!("  parent performance (understudy-fading, this stage): fading-taught fed 55 burns 13 (expired taught 99, earned 0); fading-earned fed 59 burns 15 (expired 0 / 19); fading-both fed 54 burns 14, checks 87 (events 596; expired taught 158, earned 18)");
    println!("  principal's ledger (pooled-moths, cave): waning-750 fed 48 burns 16, checks 103 (events 502; learners 17, acquittals 105; expired taught 58, earned 27); pooled-taught fed 43 burns 10, checks 134 (events 860: irr 324, steady 536; learners 18, acquittals 210; expired taught 111, earned 0); pooled-earned fed 54 burns 12, checks 110 (events 644: irr 202, steady 442; learners 18, acquittals 147; expired taught 0, earned 20); P3 CRACKED there \u{2014} neither pool carries the danger");
    println!("  this stage's waning-750 (pinned): fed 55 burns 12, checks 97 (events 671; learners 11, acquittals 161; expired taught 62, earned 16) \u{2014} the safest waning room on this stage");
    println!("  principal's ledger (staggered-moths, cave): waning-750 fed 48 burns 16 (expired taught 58, earned 27); staggered fed 50 burns 9, checks 116 (events 611: irr 224, steady 387; learners 18, acquittals 145; expired taught 55, earned 15); staggered-both fed 45 burns 13, checks 128 (events 836: irr 266, steady 570; learners 18, acquittals 183; expired taught 58, earned 19); ordering staggered 9 < coincident-late 13 < coincident-750 16; P4 CRACKED there \u{2014} duration priced at 3 burns, the stagger at 4");
    println!("  principal's ledger (mirrored-moths, cave): mirrored fed 46 burns 11, checks 129 (events 662: irr 269, steady 393; learners 18, acquittals 162; expired taught 99, earned 12); ordering staggered 9 < mirrored 11 < coincident-late 13 < coincident-750 16 \u{2014} symmetric in kind, asymmetric in price: mirror gap 2, coincidence gap 4; all six predictions HELD there (P2 exact, asserted: offsets taught 0 / earned 375)");
    println!("  this stage's staggered (pinned): staggered fed 54 burns 11, checks 100 (events 695; learners 11, acquittals 162; expired taught 48, earned 15); staggered-both fed 54 burns 11, checks 100 (events 695; learners 11, acquittals 162; expired taught 48, earned 10) \u{2014} the stagger collapsed on this stage: duration 1 burn, the stagger 0");
    println!("  principal's ledger (wide-stagger-moths, cave): staggered-wide fed 54 burns 12, checks 110 (events 644: irr 202, steady 442; learners 18, acquittals 145; expired taught 31, earned 20); mirrored-wide fed 46 burns 12, checks 125 (events 779: irr 251, steady 528; learners 17, acquittals 166; expired taught 89, earned 17); coincident-wide fed 43 burns 12, checks 151 (events 943: irr 342, steady 601; learners 18, acquittals 222; expired taught 42, earned 20) \u{2014} all three wide rooms burn 12 as one room; the mirror gap closed 2 to 0 and the waiting apart fell 4 to 0; P1, P2, P3, P5, P6 HELD there, P4 CRACKED (no wide stagger beats its control)");
    println!("  this stage's narrow staggers (pinned): staggered fed 54 burns 11, checks 100 (events 695; learners 11, acquittals 162; expired taught 48, earned 15); staggered-both fed 54 burns 11 (expired taught 48, earned 10); mirrored fed 55 burns 12, checks 97 (events 671; learners 11, acquittals 162; expired taught 62, earned 11) \u{2014} at +375 both performances collapsed below this stage's ~450-tick claim-step: staggered onto its control's headline, mirrored onto waning-750's");
    println!("  principal's ledger (claimstep-moths, cave): staggered-step fed 50 burns 9, checks 116 (events 611: irr 224, steady 387; learners 18, acquittals 145; expired taught 55, earned 15); mirrored-step fed 46 burns 11, checks 129 (events 662: irr 269, steady 393; learners 18, acquittals 162; expired taught 99, earned 12); coincident-step fed 45 burns 13, checks 128 (events 836: irr 266, steady 570; learners 18, acquittals 183; expired taught 55, earned 19) \u{2014} the step rooms reproduce their +375 parents entire in the cave (staggered 9, mirrored 11, staggered-both 13); P1, P2, P4, P5, P6 HELD there, P3 CRACKED (staggered-step 9, not 10..=12: the price has not begun to turn at the step; the optimum is a shelf)");
    println!("  this stage's stagger history (pinned): at +375 staggered fed 54 burns 11 = staggered-both 11 (collapse onto the control's headline), mirrored fed 55 burns 12 = waning-750 12 (collapse onto waning-750 entire); at +750 staggered-wide fed 59 burns 15 = coincident-wide 15 (the taught side taxed past waning-750), mirrored-wide fed 55 burns 12 = waning-750 12 (the mirror side unmoved a third time) \u{2014} +450 is exactly one claim-step on this stage: the first offset it can barely see");
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
    print_room("mirrored (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +375; no dawn tax)", &mi_n, &mi);
    print_waning_detail("mirrored", &mi_n, &mi);
    print_room("staggered-wide (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +750, wider than one claim-step; no dawn tax)", &sw_n, &sw);
    print_waning_detail("staggered-wide", &sw_n, &sw);
    print_room("mirrored-wide (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +750, the wide mirror stagger; no dawn tax)", &mw_n, &mw);
    print_waning_detail("mirrored-wide", &mw_n, &mw);
    print_room("coincident-wide (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +750, coincident control at the longest deadline; no dawn tax)", &cw_n, &cw);
    print_waning_detail("coincident-wide", &cw_n, &cw);
    print_room("staggered-step (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +450, exactly one claim-step; no dawn tax)", &ss_n, &ss);
    print_waning_detail("staggered-step", &ss_n, &ss);
    print_room("mirrored-step (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +450, the mirror at the step; no dawn tax)", &ms_n, &ms);
    print_waning_detail("mirrored-step", &ms_n, &ms);
    print_room("coincident-step (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +450, coincident control at the longest deadline 1950; no dawn tax)", &cs_n, &cs);
    print_waning_detail("coincident-step", &cs_n, &cs);
    print_room("staggered-edge (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +600, midway between the step and the wide offset; no dawn tax)", &se_n, &se);
    print_waning_detail("staggered-edge", &se_n, &se);
    print_room("mirrored-edge (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +600, the mirror at the edge; no dawn tax)", &me_n, &me);
    print_waning_detail("mirrored-edge", &me_n, &me);
    print_room("coincident-edge (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +600, coincident control at the longest deadline 2100; no dawn tax)", &ce_n, &ce);
    print_waning_detail("coincident-edge", &ce_n, &ce);
    print_room("staggered-face (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +525, midway between the step and the edge; no dawn tax)", &sfa_n, &sfa);
    print_waning_detail("staggered-face", &sfa_n, &sfa);
    print_room("mirrored-face (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +525, the mirror at the face; no dawn tax)", &mfa_n, &mfa);
    print_waning_detail("mirrored-face", &mfa_n, &mfa);
    print_room("coincident-face (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +525, coincident control at the longest deadline 2025; no dawn tax)", &cfa_n, &cfa);
    print_waning_detail("coincident-face", &cfa_n, &cfa);
    print_room("staggered-475 (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +475, 25 owner-ticks past the claim-step inside the cliff interval at finer grain; no dawn tax)", &s475_n, &s475);
    print_waning_detail("staggered-475", &s475_n, &s475);
    print_room("mirrored-475 (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +475, the mirror at the finer grain; no dawn tax)", &m475_n, &m475);
    print_waning_detail("mirrored-475", &m475_n, &m475);
    print_room("coincident-475 (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +475, coincident control at the longest deadline 1975; no dawn tax)", &c475_n, &c475);
    print_waning_detail("coincident-475", &c475_n, &c475);
    print_room("staggered-500 (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +500, 50 owner-ticks past the claim-step inside the cliff interval at finer grain; no dawn tax)", &s500_n, &s500);
    print_waning_detail("staggered-500", &s500_n, &s500);
    print_room("mirrored-500 (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +500, the mirror at the finer grain; no dawn tax)", &m500_n, &m500);
    print_waning_detail("mirrored-500", &m500_n, &m500);
    print_room("coincident-500 (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +500, coincident control at the longest deadline 2000; no dawn tax)", &c500_n, &c500);
    print_waning_detail("coincident-500", &c500_n, &c500);
    print_room("staggered-675 (evidence teaching; both pools lifetime 750 owner-ticks; TAUGHT deadline offset +675, midway between the edge and the wide offset inside the last 150-tick interval; no dawn tax)", &s675_n, &s675);
    print_waning_detail("staggered-675", &s675_n, &s675);
    print_room("mirrored-675 (evidence teaching; both pools lifetime 750 owner-ticks; EARNED deadline offset +675, the mirror near the wide offset; no dawn tax)", &m675_n, &m675);
    print_waning_detail("mirrored-675", &m675_n, &m675);
    print_room("coincident-675 (evidence teaching; both pools lifetime 750 owner-ticks; BOTH deadlines offset +675, coincident control at the longest deadline 2175; no dawn tax)", &c675_n, &c675);
    print_waning_detail("coincident-675", &c675_n, &c675);
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
        "    P1 (mirrored) the channel works mirrored, in both kinds: mirrored irr {} / steady {} learners {} [{}] (principal 269/393; learners 18)",
        mi.waning_irregular, mi.waning_steady, mi.waning_learners,
        verdict(mi.waning_irregular > 0 && mi.waning_steady > 0 && mi.waning_learners > 0)
    );
    println!(
        "    P2 (mirrored) the offset is real: mirrored expires BOTH pools: expired taught {} / earned {} [{} both > 0, no dawn tax {}] (principal HELD exactly, asserted; 99/12; offsets asserted by test: taught 0 / earned 375, the mirror image of staggered's 375 / 0)",
        mi.waning_expired_taught, mi.waning_expired_earned,
        verdict(mi.waning_expired_taught > 0 && mi.waning_expired_earned > 0), verdict(mi.expired_taught + mi.expired_earned == 0)
    );
    println!(
        "    P3 (mirrored, symmetry performed) mirrored burns {} within three of this stage's staggered ({}) [{} in {}..={}] (principal: mirrored 11 within three of staggered 9 \u{2014} the buyback a property of the waiting apart, not of which pool waits)",
        mi.burns, st.burns, verdict(mi.burns.abs_diff(st.burns) <= 3), st.burns.saturating_sub(3), st.burns + 3
    );
    println!(
        "    P4 (mirrored) the control discriminates: staggered-both burns {} >= mirrored burns {} [{}] (principal strict: mirrored 11 < control 13 \u{2014} coincident deadlines at the same longest deadline never beat a stagger, in either direction; equality is this stage's collapse signature, kept as a pass only if P3 also holds)",
        sb.burns, mi.burns, verdict(sb.burns >= mi.burns)
    );
    println!(
        "    P5 (mirrored) the mirrored room does not starve the channel: mirrored events {} [{} >= 300] (principal HELD: 662; this stage's staggered posted 695)",
        mi.waning_events, verdict(mi.waning_events >= 300)
    );
    println!(
        "    P6 (mirrored) a mirrored moth still eats: mirrored fed {} [{} >= 50] (principal: 46, declared against its evidence 43; this stage's staggered fed 54)",
        mi.fed, verdict(mi.fed >= 50)
    );
    println!(
        "    P1 (wide) the channel works wide, in both kinds, in each wide room: staggered-wide irr {} / steady {} learners {} [{}]; mirrored-wide irr {} / steady {} learners {} [{}]; coincident-wide irr {} / steady {} learners {} [{}] (principal 202/442, 251/528, 342/601; learners 18/17/18)",
        sw.waning_irregular, sw.waning_steady, sw.waning_learners,
        verdict(sw.waning_irregular > 0 && sw.waning_steady > 0 && sw.waning_learners > 0),
        mw.waning_irregular, mw.waning_steady, mw.waning_learners,
        verdict(mw.waning_irregular > 0 && mw.waning_steady > 0 && mw.waning_learners > 0),
        cw.waning_irregular, cw.waning_steady, cw.waning_learners,
        verdict(cw.waning_irregular > 0 && cw.waning_steady > 0 && cw.waning_learners > 0)
    );
    println!(
        "    P2 (wide) the wide offset is real: each wide room expires BOTH pools: staggered-wide expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-wide {} / {} [{} both > 0, no dawn tax {}]; coincident-wide {} / {} [{} both > 0, no dawn tax {}] (principal HELD, asserted; 31/20, 89/17, 42/20; offsets asserted by test: 750/0, 0/750, 750/750)",
        sw.waning_expired_taught, sw.waning_expired_earned,
        verdict(sw.waning_expired_taught > 0 && sw.waning_expired_earned > 0), verdict(sw.expired_taught + sw.expired_earned == 0),
        mw.waning_expired_taught, mw.waning_expired_earned,
        verdict(mw.waning_expired_taught > 0 && mw.waning_expired_earned > 0), verdict(mw.expired_taught + mw.expired_earned == 0),
        cw.waning_expired_taught, cw.waning_expired_earned,
        verdict(cw.waning_expired_taught > 0 && cw.waning_expired_earned > 0), verdict(cw.expired_taught + cw.expired_earned == 0)
    );
    println!(
        "    P3 (wide, the mirror gap at the wide offset) mirrored-wide burns {} within three of staggered-wide burns {} [{} in {}..={}] (principal: the gap CLOSED in the cave, 12 against 12, inside its declared band of 4; this stage's narrow mirror gap was 1, inside the same band of 3 declared here)",
        mw.burns, sw.burns, verdict(mw.burns.abs_diff(sw.burns) <= 3), sw.burns.saturating_sub(3), sw.burns + 3
    );
    println!(
        "    P4 (wide, the control at the longest deadline) coincident-wide burns {} >= staggered-wide {} [{}] and >= mirrored-wide {} [{}] (principal CRACKED its strict line: coincident-wide 12 EQUALS both wide staggers 12 \u{2014} equality here reproduces the principal's wide collapse; a control BELOW either stagger cracks this line outright)",
        cw.burns, sw.burns, verdict(cw.burns >= sw.burns), mw.burns, verdict(cw.burns >= mw.burns)
    );
    println!(
        "    P5 (wide) no wide room starves the channel: staggered-wide events {} [{} >= 300], mirrored-wide events {} [{} >= 300], coincident-wide events {} [{} >= 300] (principal HELD: 644 / 779 / 943; this stage's narrow staggers posted 695 / 671)",
        sw.waning_events, verdict(sw.waning_events >= 300),
        mw.waning_events, verdict(mw.waning_events >= 300),
        cw.waning_events, verdict(cw.waning_events >= 300)
    );
    println!(
        "    P6 (wide) a wide-staggered moth still eats: staggered-wide fed {} [{} >= 50], mirrored-wide fed {} [{} >= 50], coincident-wide fed {} [{} >= 50] (principal: 54 / 46 / 43, declared against its evidence 43; this stage's narrow rooms fed 54 / 55, its evidence 57)",
        sw.fed, verdict(sw.fed >= 50), mw.fed, verdict(mw.fed >= 50), cw.fed, verdict(cw.fed >= 50)
    );
    println!(
        "    P1 (step) the channel works at the step, in both kinds, in each step room: staggered-step irr {} / steady {} learners {} [{}]; mirrored-step irr {} / steady {} learners {} [{}]; coincident-step irr {} / steady {} learners {} [{}] (principal 224/387, 269/393, 266/570; learners 18/18/18)",
        ss.waning_irregular, ss.waning_steady, ss.waning_learners,
        verdict(ss.waning_irregular > 0 && ss.waning_steady > 0 && ss.waning_learners > 0),
        ms.waning_irregular, ms.waning_steady, ms.waning_learners,
        verdict(ms.waning_irregular > 0 && ms.waning_steady > 0 && ms.waning_learners > 0),
        cs.waning_irregular, cs.waning_steady, cs.waning_learners,
        verdict(cs.waning_irregular > 0 && cs.waning_steady > 0 && cs.waning_learners > 0)
    );
    println!(
        "    P2 (step) the step offset is real: each step room expires BOTH pools: staggered-step expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-step {} / {} [{} both > 0, no dawn tax {}]; coincident-step {} / {} [{} both > 0, no dawn tax {}] (principal HELD, asserted; 55/15, 99/12, 55/19; offsets asserted by test: 450/0, 0/450, 450/450)",
        ss.waning_expired_taught, ss.waning_expired_earned,
        verdict(ss.waning_expired_taught > 0 && ss.waning_expired_earned > 0), verdict(ss.expired_taught + ss.expired_earned == 0),
        ms.waning_expired_taught, ms.waning_expired_earned,
        verdict(ms.waning_expired_taught > 0 && ms.waning_expired_earned > 0), verdict(ms.expired_taught + ms.expired_earned == 0),
        cs.waning_expired_taught, cs.waning_expired_earned,
        verdict(cs.waning_expired_taught > 0 && cs.waning_expired_earned > 0), verdict(cs.expired_taught + cs.expired_earned == 0)
    );
    println!(
        "    P3 (step, the mirror gap at the step) mirrored-step burns {} within three of staggered-step burns {} [{} in {}..={}] (principal: the gap at the step still read 2 in the cave, 11 against 9, inside its declared band of 2; this stage's narrow mirror gap was 1 and its wide gap -3, inside the band of 3 declared here)",
        ms.burns, ss.burns, verdict(ms.burns.abs_diff(ss.burns) <= 3), ss.burns.saturating_sub(3), ss.burns + 3
    );
    println!(
        "    P4 (step, the control at the longest deadline) coincident-step burns {} >= staggered-step {} [{}] and >= mirrored-step {} [{}] (principal HELD: coincident-step 13 above both step staggers 9 / 11 in the cave \u{2014} a control BELOW either stagger cracks this line outright; equality with a stagger reproduces this stage's earlier collapses)",
        cs.burns, ss.burns, verdict(cs.burns >= ss.burns), ms.burns, verdict(cs.burns >= ms.burns)
    );
    println!(
        "    P5 (step) no step room starves the channel: staggered-step events {} [{} >= 300], mirrored-step events {} [{} >= 300], coincident-step events {} [{} >= 300] (principal HELD: 611 / 662 / 836; this stage's wide rooms posted 631 / 671 / 631)",
        ss.waning_events, verdict(ss.waning_events >= 300),
        ms.waning_events, verdict(ms.waning_events >= 300),
        cs.waning_events, verdict(cs.waning_events >= 300)
    );
    println!(
        "    P6 (step) a step-staggered moth still eats: staggered-step fed {} [{} >= 50], mirrored-step fed {} [{} >= 50], coincident-step fed {} [{} >= 50] (principal: 50 / 46 / 45, declared against its evidence 43; this stage's wide rooms fed 59 / 55 / 59, its narrow rooms 54 / 55)",
        ss.fed, verdict(ss.fed >= 50), ms.fed, verdict(ms.fed >= 50), cs.fed, verdict(cs.fed >= 50)
    );
    println!(
        "    S1 (counted, not declared) the separation at the step, against this stage's own rooms: staggered-step burns {} vs staggered (+375) {} and staggered-wide (+750) {}; mirrored-step {} vs mirrored {} and mirrored-wide {}; coincident-step {} vs staggered-both (+375 control) {} and coincident-wide {} \u{2014} whether the step rooms reproduce their +375 parents (the cave's shelf), separate toward the wide rooms, or read as their own rooms is reported as found in the README",
        ss.burns, st.burns, sw.burns, ms.burns, mi.burns, mw.burns, cs.burns, sb.burns, cw.burns
    );
    println!(
        "    P1 (edge) the channel works at the edge, in both kinds, in each edge room: staggered-edge irr {} / steady {} learners {} [{}]; mirrored-edge irr {} / steady {} learners {} [{}]; coincident-edge irr {} / steady {} learners {} [{}] (principal 235/421, 253/525, 267/533; learners 18/17/18)",
        se.waning_irregular, se.waning_steady, se.waning_learners,
        verdict(se.waning_irregular > 0 && se.waning_steady > 0 && se.waning_learners > 0),
        me.waning_irregular, me.waning_steady, me.waning_learners,
        verdict(me.waning_irregular > 0 && me.waning_steady > 0 && me.waning_learners > 0),
        ce.waning_irregular, ce.waning_steady, ce.waning_learners,
        verdict(ce.waning_irregular > 0 && ce.waning_steady > 0 && ce.waning_learners > 0)
    );
    println!(
        "    P2 (edge) the edge offset is real: each edge room expires BOTH pools: staggered-edge expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-edge {} / {} [{} both > 0, no dawn tax {}]; coincident-edge {} / {} [{} both > 0, no dawn tax {}] (principal HELD, asserted: 46/18, 89/19, 60/20; offsets asserted by test: 600/0, 0/600, 600/600)",
        se.waning_expired_taught, se.waning_expired_earned,
        verdict(se.waning_expired_taught > 0 && se.waning_expired_earned > 0), verdict(se.expired_taught + se.expired_earned == 0),
        me.waning_expired_taught, me.waning_expired_earned,
        verdict(me.waning_expired_taught > 0 && me.waning_expired_earned > 0), verdict(me.expired_taught + me.expired_earned == 0),
        ce.waning_expired_taught, ce.waning_expired_earned,
        verdict(ce.waning_expired_taught > 0 && ce.waning_expired_earned > 0), verdict(ce.expired_taught + ce.expired_earned == 0)
    );
    println!(
        "    P3 (edge, the turn between one step and two) staggered-edge burns {} [{} in 11..=14] (this stage's taught line: +375 11 collapsed, +450 10 the one-burn separation, +750 15 the five-burn tax \u{2014} a landing at 10 or below says the fall runs nearly to the wide offset and the tax arrives all at the end; a landing at 15 says the tax is fully charged by the midpoint; the principal CRACKED its own turn line at 16, past dissolution entire)",
        se.burns, verdict((11..=14).contains(&se.burns))
    );
    println!(
        "    P4 (edge, the mirror gap at the edge) mirrored-edge burns {} within three of staggered-edge burns {} [{} in {}..={}] (principal CRACKED: the cave gap read -4 at the edge, 12 against 16, outside its declared band of 1; this stage's gaps: narrow 1 in the wide script's terms, step 2, wide -3, inside the band of 3 declared here)",
        me.burns, se.burns, verdict(me.burns.abs_diff(se.burns) <= 3), se.burns.saturating_sub(3), se.burns + 3
    );
    println!(
        "    P5 (edge, the control at the longest deadline) coincident-edge burns {} >= staggered-edge {} [{}] and >= mirrored-edge {} [{}] (principal CRACKED its coincident line: 16, not 12..=13, charged full at deadline 2100 \u{2014} on this stage a control BELOW either stagger cracks this line outright; equality with a stagger reproduces this stage's collapses)",
        ce.burns, se.burns, verdict(ce.burns >= se.burns), me.burns, verdict(ce.burns >= me.burns)
    );
    println!(
        "    P6 (edge) no edge room starves the channel or the table: staggered-edge events {} [{} >= 300] fed {} [{} >= 50]; mirrored-edge events {} [{} >= 300] fed {} [{} >= 50]; coincident-edge events {} [{} >= 300] fed {} [{} >= 50] (principal HELD: events 656 / 778 / 800, fed 56 / 46 / 46; this stage's step rooms posted events 700 / 671 / 700, fed 54 / 55 / 54)",
        se.waning_events, verdict(se.waning_events >= 300), se.fed, verdict(se.fed >= 50),
        me.waning_events, verdict(me.waning_events >= 300), me.fed, verdict(me.fed >= 50),
        ce.waning_events, verdict(ce.waning_events >= 300), ce.fed, verdict(ce.fed >= 50)
    );
    println!(
        "    S1 (edge, counted, not declared) the taught line at the edge, against this stage's own rooms: staggered-edge burns {} vs staggered (+375) {}, staggered-step (+450) {} and staggered-wide (+750) {}; mirrored-edge {} vs mirrored {} / mirrored-step {} / mirrored-wide {} (all 12 so far); coincident-edge {} vs staggered-both (+375 control) {}, coincident-step {} and coincident-wide {} \u{2014} whether the taught side keeps falling, turns up toward the wide tax, or reads a price of its own is reported as found in the README",
        se.burns, st.burns, ss.burns, sw.burns, me.burns, mi.burns, ms.burns, mw.burns, ce.burns, sb.burns, cs.burns, cw.burns
    );
    println!(
        "    P1 (face) the channel works at the face, in both kinds, in each face room: staggered-face irr {} / steady {} learners {} [{}]; mirrored-face irr {} / steady {} learners {} [{}]; coincident-face irr {} / steady {} learners {} [{}] (principal 235/421, 269/393, 267/533; learners 18/18/18)",
        sfa.waning_irregular, sfa.waning_steady, sfa.waning_learners,
        verdict(sfa.waning_irregular > 0 && sfa.waning_steady > 0 && sfa.waning_learners > 0),
        mfa.waning_irregular, mfa.waning_steady, mfa.waning_learners,
        verdict(mfa.waning_irregular > 0 && mfa.waning_steady > 0 && mfa.waning_learners > 0),
        cfa.waning_irregular, cfa.waning_steady, cfa.waning_learners,
        verdict(cfa.waning_irregular > 0 && cfa.waning_steady > 0 && cfa.waning_learners > 0)
    );
    println!(
        "    P2 (face) the face offset is real: each face room expires BOTH pools: staggered-face expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-face {} / {} [{} both > 0, no dawn tax {}]; coincident-face {} / {} [{} both > 0, no dawn tax {}] (principal HELD, asserted: 61/18, 99/12, 70/19; offsets asserted by test: 525/0, 0/525, 525/525)",
        sfa.waning_expired_taught, sfa.waning_expired_earned,
        verdict(sfa.waning_expired_taught > 0 && sfa.waning_expired_earned > 0), verdict(sfa.expired_taught + sfa.expired_earned == 0),
        mfa.waning_expired_taught, mfa.waning_expired_earned,
        verdict(mfa.waning_expired_taught > 0 && mfa.waning_expired_earned > 0), verdict(mfa.expired_taught + mfa.expired_earned == 0),
        cfa.waning_expired_taught, cfa.waning_expired_earned,
        verdict(cfa.waning_expired_taught > 0 && cfa.waning_expired_earned > 0), verdict(cfa.expired_taught + cfa.expired_earned == 0)
    );
    println!(
        "    P3 (face, the price between step and edge) staggered-face burns {} [{} in 10..=11] (this stage's taught line: +375 11, +450 10, +600 10, +750 15 \u{2014} a landing at 10-11 says the step price holds across the whole interval and the face has no interior price here; a landing at 12+ says an interior or tax price exists between step and edge; the principal HELD its early-face line at 16 in the cave)",
        sfa.burns, verdict((10..=11).contains(&sfa.burns))
    );
    println!(
        "    P4 (face, the mirror gap at the face) mirrored-face burns {} within three of staggered-face burns {} [{} in {}..={}] (principal HELD with gap -5 in the cave, outside a band of 3 in the other direction; this stage's gaps: step 2, edge 2)",
        mfa.burns, sfa.burns, verdict(mfa.burns.abs_diff(sfa.burns) <= 3), sfa.burns.saturating_sub(3), sfa.burns + 3
    );
    println!(
        "    P5 (face, the control at the longest deadline) coincident-face burns {} >= staggered-face {} [{}] and >= mirrored-face {} [{}] (principal HELD its coincident line at 16 in the cave \u{2014} on this stage a control BELOW either stagger cracks this line outright; equality with the taught stagger reproduces this stage's collapses, as at step and edge)",
        cfa.burns, sfa.burns, verdict(cfa.burns >= sfa.burns), mfa.burns, verdict(cfa.burns >= mfa.burns)
    );
    println!(
        "    P6 (face) no face room starves the channel or the table: staggered-face events {} [{} >= 300] fed {} [{} >= 50]; mirrored-face events {} [{} >= 300] fed {} [{} >= 50]; coincident-face events {} [{} >= 300] fed {} [{} >= 50] (principal HELD: events 656 / 662 / 800, fed 56 / 46 / 46; this stage's edge rooms posted events 700 / 671 / 700, fed 54 / 55 / 54)",
        sfa.waning_events, verdict(sfa.waning_events >= 300), sfa.fed, verdict(sfa.fed >= 50),
        mfa.waning_events, verdict(mfa.waning_events >= 300), mfa.fed, verdict(mfa.fed >= 50),
        cfa.waning_events, verdict(cfa.waning_events >= 300), cfa.fed, verdict(cfa.fed >= 50)
    );
    println!(
        "    S1 (face, counted, not declared) the taught line at the face, against this stage's own rooms: staggered-face burns {} vs staggered (+375) {}, staggered-step (+450) {}, staggered-edge (+600) {} and staggered-wide (+750) {}; mirrored-face {} vs mirrored {} / mirrored-step {} / mirrored-edge {} / mirrored-wide {} (all 12 so far); coincident-face {} vs coincident-step {}, coincident-edge {} and coincident-wide {} \u{2014} whether the face holds the step price, posts an interior price, or moves only in census is reported as found in the README",
        sfa.burns, st.burns, ss.burns, se.burns, sw.burns, mfa.burns, mi.burns, ms.burns, me.burns, mw.burns, cfa.burns, cs.burns, ce.burns, cw.burns
    );
    println!(
        "    P1 (grain) the channel works at both grains, in both kinds, in each grain room: staggered-475 irr {} / steady {} learners {} [{}]; mirrored-475 irr {} / steady {} learners {} [{}]; coincident-475 irr {} / steady {} learners {} [{}]; staggered-500 irr {} / steady {} learners {} [{}]; mirrored-500 irr {} / steady {} learners {} [{}]; coincident-500 irr {} / steady {} learners {} [{}] (principal 291/482, 269/393, 190/395, 235/421, 269/393, 267/533; learners 18/18/16/18/18/18)",
        s475.waning_irregular, s475.waning_steady, s475.waning_learners,
        verdict(s475.waning_irregular > 0 && s475.waning_steady > 0 && s475.waning_learners > 0),
        m475.waning_irregular, m475.waning_steady, m475.waning_learners,
        verdict(m475.waning_irregular > 0 && m475.waning_steady > 0 && m475.waning_learners > 0),
        c475.waning_irregular, c475.waning_steady, c475.waning_learners,
        verdict(c475.waning_irregular > 0 && c475.waning_steady > 0 && c475.waning_learners > 0),
        s500.waning_irregular, s500.waning_steady, s500.waning_learners,
        verdict(s500.waning_irregular > 0 && s500.waning_steady > 0 && s500.waning_learners > 0),
        m500.waning_irregular, m500.waning_steady, m500.waning_learners,
        verdict(m500.waning_irregular > 0 && m500.waning_steady > 0 && m500.waning_learners > 0),
        c500.waning_irregular, c500.waning_steady, c500.waning_learners,
        verdict(c500.waning_irregular > 0 && c500.waning_steady > 0 && c500.waning_learners > 0)
    );
    println!(
        "    P2 (grain) the grain offsets are real: each grain room expires BOTH pools: staggered-475 expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-475 {} / {} [{} both > 0, no dawn tax {}]; coincident-475 {} / {} [{} both > 0, no dawn tax {}]; staggered-500 {} / {} [{} both > 0, no dawn tax {}]; mirrored-500 {} / {} [{} both > 0, no dawn tax {}]; coincident-500 {} / {} [{} both > 0, no dawn tax {}] (principal HELD, asserted: 46/20, 99/12, 50/18, 62/18, 99/12, 71/19; offsets asserted by test: 475/0, 0/475, 475/475, 500/0, 0/500, 500/500)",
        s475.waning_expired_taught, s475.waning_expired_earned,
        verdict(s475.waning_expired_taught > 0 && s475.waning_expired_earned > 0), verdict(s475.expired_taught + s475.expired_earned == 0),
        m475.waning_expired_taught, m475.waning_expired_earned,
        verdict(m475.waning_expired_taught > 0 && m475.waning_expired_earned > 0), verdict(m475.expired_taught + m475.expired_earned == 0),
        c475.waning_expired_taught, c475.waning_expired_earned,
        verdict(c475.waning_expired_taught > 0 && c475.waning_expired_earned > 0), verdict(c475.expired_taught + c475.expired_earned == 0),
        s500.waning_expired_taught, s500.waning_expired_earned,
        verdict(s500.waning_expired_taught > 0 && s500.waning_expired_earned > 0), verdict(s500.expired_taught + s500.expired_earned == 0),
        m500.waning_expired_taught, m500.waning_expired_earned,
        verdict(m500.waning_expired_taught > 0 && m500.waning_expired_earned > 0), verdict(m500.expired_taught + m500.expired_earned == 0),
        c500.waning_expired_taught, c500.waning_expired_earned,
        verdict(c500.waning_expired_taught > 0 && c500.waning_expired_earned > 0), verdict(c500.expired_taught + c500.expired_earned == 0)
    );
    println!(
        "    P3 (grain, the price inside the interval's first half) staggered-475 burns {} [{} in 10..=11] and staggered-500 burns {} [{} in 10..=11] (this stage's taught line: +450 10, +525 10, +600 10, +750 15 \u{2014} a landing at 10-11 at both grains says the interval's first half has no interior price here either; a landing at 12+ at either grain is the first interior price this stage has ever posted; the principal CRACKED its whole-fall line at exactly these addresses: 14 at +475, 16 at +500)",
        s475.burns, verdict((10..=11).contains(&s475.burns)), s500.burns, verdict((10..=11).contains(&s500.burns))
    );
    println!(
        "    P4 (grain, the mirror gaps at finer grain) mirrored-475 burns {} within three of staggered-475 burns {} [{} in {}..={}]; mirrored-500 burns {} within three of staggered-500 burns {} [{} in {}..={}] (principal gaps -3 at +475 and -5 at +500 in the cave; this stage's gaps: 2 at the step, 2 at the face, 2 at the edge)",
        m475.burns, s475.burns, verdict(m475.burns.abs_diff(s475.burns) <= 3), s475.burns.saturating_sub(3), s475.burns + 3,
        m500.burns, s500.burns, verdict(m500.burns.abs_diff(s500.burns) <= 3), s500.burns.saturating_sub(3), s500.burns + 3
    );
    println!(
        "    P5 (grain, the controls at the longer deadlines) coincident-475 burns {} >= staggered-475 {} [{}] and >= mirrored-475 {} [{}]; coincident-500 burns {} >= staggered-500 {} [{}] and >= mirrored-500 {} [{}] (principal HELD both grains in the cave: 14 >= 14 / 11 and 16 >= 16 / 11 \u{2014} on this stage a control BELOW either stagger cracks this line outright, and the controls at the step, the face and the edge all fell below their mirrors)",
        c475.burns, s475.burns, verdict(c475.burns >= s475.burns), m475.burns, verdict(c475.burns >= m475.burns),
        c500.burns, s500.burns, verdict(c500.burns >= s500.burns), m500.burns, verdict(c500.burns >= m500.burns)
    );
    println!(
        "    P6 (grain) no grain room starves the channel or the table: staggered-475 events {} [{} >= 300] fed {} [{} >= 50]; mirrored-475 events {} [{} >= 300] fed {} [{} >= 50]; coincident-475 events {} [{} >= 300] fed {} [{} >= 50]; staggered-500 events {} [{} >= 300] fed {} [{} >= 50]; mirrored-500 events {} [{} >= 300] fed {} [{} >= 50]; coincident-500 events {} [{} >= 300] fed {} [{} >= 50] (principal HELD: events 773 / 662 / 585 / 656 / 662 / 800, fed 48 / 46 / 48 / 56 / 46 / 46 against its table of 43; this stage's face rooms posted events 700 / 671 / 700, fed 54 / 55 / 54)",
        s475.waning_events, verdict(s475.waning_events >= 300), s475.fed, verdict(s475.fed >= 50),
        m475.waning_events, verdict(m475.waning_events >= 300), m475.fed, verdict(m475.fed >= 50),
        c475.waning_events, verdict(c475.waning_events >= 300), c475.fed, verdict(c475.fed >= 50),
        s500.waning_events, verdict(s500.waning_events >= 300), s500.fed, verdict(s500.fed >= 50),
        m500.waning_events, verdict(m500.waning_events >= 300), m500.fed, verdict(m500.fed >= 50),
        c500.waning_events, verdict(c500.waning_events >= 300), c500.fed, verdict(c500.fed >= 50)
    );
    println!(
        "    P7 (grain, the grains agree \u{2014} the principal's own P7, which CRACKED in the cave at 14 != 16) staggered-475 burns {} == staggered-500 burns {} [{}] and coincident-475 burns {} == coincident-500 burns {} [{}] (a stage that prices in whole claim-sized steps should be unable to tell 25 owner-ticks of grain apart; a disagreement at either line is this stage's first interior distinction)",
        s475.burns, s500.burns, verdict(s475.burns == s500.burns), c475.burns, c500.burns, verdict(c475.burns == c500.burns)
    );
    println!(
        "    S1 (grain, counted, not declared) the bookkeeping inside the interval's first half, against this stage's own rooms: staggered-475 taught census {} / acquittals {} and staggered-500 taught census {} / acquittals {}, against staggered-step (+450) census {} / acquittals {}, staggered-face (+525) {} / {} and staggered-edge (+600) {} / {} \u{2014} whether the census separates at all this close to the step, and where each grain's census stands between the step's and the face's, is reported as found in the README; mirrored earned censuses at the grains {} / {} extend the family sequence 16 / 11 / 11 / 9 / 10 / 11",
        s475.waning_expired_taught, s475.waning_acquittals, s500.waning_expired_taught, s500.waning_acquittals,
        ss.waning_expired_taught, ss.waning_acquittals, sfa.waning_expired_taught, sfa.waning_acquittals, se.waning_expired_taught, se.waning_acquittals,
        m475.waning_expired_earned, m500.waning_expired_earned
    );
    println!(
        "    P1 (nearwide) the channel works at +675, in both kinds, in each new room: staggered-675 irr {} / steady {} learners {} [{}]; mirrored-675 irr {} / steady {} learners {} [{}]; coincident-675 irr {} / steady {} learners {} [{}] (stage precedents: every stagger room posts both kinds and 11 learners)",
        s675.waning_irregular, s675.waning_steady, s675.waning_learners,
        verdict(s675.waning_irregular > 0 && s675.waning_steady > 0 && s675.waning_learners > 0),
        m675.waning_irregular, m675.waning_steady, m675.waning_learners,
        verdict(m675.waning_irregular > 0 && m675.waning_steady > 0 && m675.waning_learners > 0),
        c675.waning_irregular, c675.waning_steady, c675.waning_learners,
        verdict(c675.waning_irregular > 0 && c675.waning_steady > 0 && c675.waning_learners > 0)
    );
    println!(
        "    P2 (nearwide) the +675 offsets are real: each new room expires BOTH pools: staggered-675 expired taught {} / earned {} [{} both > 0, no dawn tax {}]; mirrored-675 {} / {} [{} both > 0, no dawn tax {}]; coincident-675 {} / {} [{} both > 0, no dawn tax {}] (offsets asserted by test: 675/0, 0/675, 675/675)",
        s675.waning_expired_taught, s675.waning_expired_earned,
        verdict(s675.waning_expired_taught > 0 && s675.waning_expired_earned > 0), verdict(s675.expired_taught + s675.expired_earned == 0),
        m675.waning_expired_taught, m675.waning_expired_earned,
        verdict(m675.waning_expired_taught > 0 && m675.waning_expired_earned > 0), verdict(m675.expired_taught + m675.expired_earned == 0),
        c675.waning_expired_taught, c675.waning_expired_earned,
        verdict(c675.waning_expired_taught > 0 && c675.waning_expired_earned > 0), verdict(c675.expired_taught + c675.expired_earned == 0)
    );
    println!(
        "    P3 (nearwide, the price in the last interval) staggered-675 burns {} [{} in 10..=11] (this stage's taught line: +450 10, +475 10, +500 10, +525 10, +600 10, +750 15 \u{2014} a landing at 10-11 says the step price holds at the last interval's midpoint too and the tax lives in the final 75 ticks; a landing at 12..=14 is the first interior price this stage has ever posted; a landing at 15 says the tax has already arrived by +675)",
        s675.burns, verdict((10..=11).contains(&s675.burns))
    );
    println!(
        "    P4 (nearwide, the mirror gap near wide) mirrored-675 burns {} within three of staggered-675 burns {} [{} in {}..={}] (this stage's gaps: 2 at the step, the face, the edge and both grains; the wide rooms read staggered 15 against mirrored 12, a gap of -3 the other way)",
        m675.burns, s675.burns, verdict(m675.burns.abs_diff(s675.burns) <= 3), s675.burns.saturating_sub(3), s675.burns + 3
    );
    println!(
        "    P5 (nearwide, the control at deadline 2175) coincident-675 burns {} >= staggered-675 {} [{}] and >= mirrored-675 {} [{}] (on this stage a control BELOW either stagger cracks this line outright, and the controls at 1950, 1975, 2000, 2025 and 2100 all equalled their taught staggers and fell below their mirrors)",
        c675.burns, s675.burns, verdict(c675.burns >= s675.burns), m675.burns, verdict(c675.burns >= m675.burns)
    );
    println!(
        "    P6 (nearwide) no new room starves the channel or the table: staggered-675 events {} [{} >= 300] fed {} [{} >= 50]; mirrored-675 events {} [{} >= 300] fed {} [{} >= 50]; coincident-675 events {} [{} >= 300] fed {} [{} >= 50] (stage precedents: stagger rooms post events 631-700, fed 54-59)",
        s675.waning_events, verdict(s675.waning_events >= 300), s675.fed, verdict(s675.fed >= 50),
        m675.waning_events, verdict(m675.waning_events >= 300), m675.fed, verdict(m675.fed >= 50),
        c675.waning_events, verdict(c675.waning_events >= 300), c675.fed, verdict(c675.fed >= 50)
    );
    println!(
        "    P7 (nearwide, the census in the last interval) staggered-675 taught census {} [{} in 12..=20, strictly between the edge's 21 and the wide room's 11] and its acquittals {} against the edge's {} and the wide room's {} (declared with P3: the census falls AHEAD of the price \u{2014} it has already left the edge's count toward the wide room's while the price still reads the step's; a census of 21 says the bookkeeping waits with the price, a census of 11 or below says it has arrived entire before the price moves)",
        s675.waning_expired_taught, verdict((12..=20).contains(&s675.waning_expired_taught)), s675.waning_acquittals, se.waning_acquittals, sw.waning_acquittals
    );
    println!(
        "    S1 (nearwide, counted, not declared) coincident-675 follows which room, field for field: coincident fed {} / burns {} / checks {} / events {} / acquittals {} / census {}/{} against staggered-675 fed {} / burns {} / checks {} / events {} / acquittals {} / census {}/{} and mirrored-675 fed {} / burns {} / checks {} / events {} / acquittals {} / census {}/{}; mirrored-675's earned census {} extends the mirror family sequence 16 / 11 / 11 / 9 / 10 / 11 / 11 / 11 (waning / +375 / +450 / +750 / +600 / +525 / +475 / +500)",
        c675.fed, c675.burns, c675.checks, c675.waning_events, c675.waning_acquittals, c675.waning_expired_taught, c675.waning_expired_earned,
        s675.fed, s675.burns, s675.checks, s675.waning_events, s675.waning_acquittals, s675.waning_expired_taught, s675.waning_expired_earned,
        m675.fed, m675.burns, m675.checks, m675.waning_events, m675.waning_acquittals, m675.waning_expired_taught, m675.waning_expired_earned,
        m675.waning_expired_earned
    );
    println!(
        "    honest refusals across all rooms {} (counted, not a declared line on this stage; queue precedent 0)",
        solo.honest_refusals + wit.honest_refusals + chain.honest_refusals + placed.honest_refusals + ev.honest_refusals + th.honest_refusals + un.honest_refusals + pa.honest_refusals + sh.honest_refusals + ft.honest_refusals + fe.honest_refusals + fb.honest_refusals + w15.honest_refusals + w75.honest_refusals + w37.honest_refusals + w1.honest_refusals + pt.honest_refusals + pe.honest_refusals + st.honest_refusals + sb.honest_refusals + mi.honest_refusals + sw.honest_refusals + mw.honest_refusals + cw.honest_refusals + ss.honest_refusals + ms.honest_refusals + cs.honest_refusals + se.honest_refusals + me.honest_refusals + ce.honest_refusals + sfa.honest_refusals + mfa.honest_refusals + cfa.honest_refusals + s475.honest_refusals + m475.honest_refusals + c475.honest_refusals + s500.honest_refusals + m500.honest_refusals + c500.honest_refusals + s675.honest_refusals + m675.honest_refusals + c675.honest_refusals
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
        for room in [Room::Solo, Room::Witness, Room::Chain, Room::Placed, Room::Evidence, Room::Throttled, Room::Paired, Room::Unshoved, Room::Shoved, Room::FadingTaught, Room::FadingEarned, Room::FadingBoth, Room::Waning1500, Room::Waning750, Room::Waning375, Room::Waning150, Room::PooledTaught, Room::PooledEarned, Room::Staggered, Room::StaggeredBoth, Room::Mirrored, Room::StaggeredWide, Room::MirroredWide, Room::CoincidentWide, Room::StaggeredStep, Room::MirroredStep, Room::CoincidentStep, Room::StaggeredEdge, Room::MirroredEdge, Room::CoincidentEdge, Room::StaggeredFace, Room::MirroredFace, Room::CoincidentFace, Room::Staggered475, Room::Mirrored475, Room::Coincident475, Room::Staggered500, Room::Mirrored500, Room::Coincident500, Room::Staggered675, Room::Mirrored675, Room::Coincident675] {
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
    fn mirrored_packets_travel_in_both_kinds() {
        let (_, t) = run_room(Room::Mirrored, SEED);
        assert!(t.waning_irregular > 0, "no irregular mirrored packet: {t:?}");
        assert!(t.waning_steady > 0, "no steady mirrored packet: {t:?}");
        assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
        assert!(t.waning_learners > 0, "no moth first learned from a mirrored packet: {t:?}");
    }

    #[test]
    fn mirrored_offsets_are_as_declared() {
        assert_eq!(Room::Mirrored.taught_offset(), 0);
        assert_eq!(Room::Mirrored.earned_offset(), 375);
        assert_eq!(Room::Staggered.taught_offset(), 375);
        assert_eq!(Room::Staggered.earned_offset(), 0);
        assert_eq!(Room::Mirrored.waning_lifetime(), Some(750));
    }

    #[test]
    fn mirrored_both_pools_expire_and_no_dawn_tax() {
        let (_, t) = run_room(Room::Mirrored, SEED);
        assert!(t.waning_expired_taught > 0, "taught pool never waned in mirrored: {t:?}");
        assert!(t.waning_expired_earned > 0, "earned pool never waned in mirrored: {t:?}");
        assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in mirrored: {t:?}");
    }

    #[test]
    fn the_mirrored_performance_is_deterministic() {
        let (_, a) = run_room(Room::Mirrored, SEED);
        let (_, b) = run_room(Room::Mirrored, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn mirrored_totals_pinned() {
        // Measured at planting, pinned as found. Mirrored reads
        // waning-750's headline entire (fed 55, burns 12, checks
        // 97, events 671) and differs only in the earned census
        // (11 against 16) and one acquittal (162 against 161).
        let (_, mi) = run_room(Room::Mirrored, SEED);
        assert_eq!(mi.fed, 55);
        assert_eq!(mi.burns, 12);
        assert_eq!(mi.checks, 97);
        assert_eq!(mi.waning_events, 671);
        assert_eq!(mi.waning_learners, 11);
        assert_eq!(mi.waning_acquittals, 162);
        assert_eq!(mi.waning_expired_taught, 62);
        assert_eq!(mi.waning_expired_earned, 11);
    }

    #[test]
    fn the_mirrored_room_still_feeds_lives_and_burns() {
        let (_, t) = run_room(Room::Mirrored, SEED);
        assert!(t.fed > 0, "mirrored never fed: {t:?}");
        assert!(t.burns > 0, "mirrored never burned: {t:?}");
        assert_eq!(t.alive_end, 24, "mirrored lost a moth: {t:?}");
    }

    #[test]
    fn wide_packets_travel_in_both_kinds_in_each_wide_room() {
        for room in [Room::StaggeredWide, Room::MirroredWide, Room::CoincidentWide] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular wide packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady wide packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a wide packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn wide_offsets_are_as_declared() {
        assert_eq!(Room::StaggeredWide.taught_offset(), 750);
        assert_eq!(Room::StaggeredWide.earned_offset(), 0);
        assert_eq!(Room::MirroredWide.taught_offset(), 0);
        assert_eq!(Room::MirroredWide.earned_offset(), 750);
        assert_eq!(Room::CoincidentWide.taught_offset(), 750);
        assert_eq!(Room::CoincidentWide.earned_offset(), 750);
        assert_eq!(Room::StaggeredWide.waning_lifetime(), Some(750));
        assert_eq!(Room::MirroredWide.waning_lifetime(), Some(750));
        assert_eq!(Room::CoincidentWide.waning_lifetime(), Some(750));
    }

    #[test]
    fn wide_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::StaggeredWide, Room::MirroredWide, Room::CoincidentWide] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_wide_performance_is_deterministic() {
        let (_, a) = run_room(Room::MirroredWide, SEED);
        let (_, b) = run_room(Room::MirroredWide, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_wide_rooms_still_feed_live_and_burn() {
        for room in [Room::StaggeredWide, Room::MirroredWide, Room::CoincidentWide] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn wide_totals_pinned() {
        // Measured at planting, pinned as found. The wide role
        // does NOT read as one room on this stage: staggered-wide
        // and coincident-wide share one headline entire (fed 59,
        // burns 15, checks 91, events 631) and differ only in
        // acquittals (141 against 143) and the earned census
        // (16 against 7); mirrored-wide reads waning-750's
        // headline for the third time in its family (fed 55,
        // burns 12, checks 97, events 671) and differs only in
        // acquittals (163) and the earned census (9, shrinking
        // 16 -> 11 -> 9 as the earned offset grows 0 -> 375 ->
        // 750). The mirror gap at the wide offset is -3 here:
        // the mirror is the cheap side, reversed from the cave.
        let (_, sw) = run_room(Room::StaggeredWide, SEED);
        assert_eq!(sw.fed, 59);
        assert_eq!(sw.burns, 15);
        assert_eq!(sw.checks, 91);
        assert_eq!(sw.waning_events, 631);
        assert_eq!(sw.waning_learners, 11);
        assert_eq!(sw.waning_acquittals, 141);
        assert_eq!(sw.waning_expired_taught, 11);
        assert_eq!(sw.waning_expired_earned, 16);
        let (_, mw) = run_room(Room::MirroredWide, SEED);
        assert_eq!(mw.fed, 55);
        assert_eq!(mw.burns, 12);
        assert_eq!(mw.checks, 97);
        assert_eq!(mw.waning_events, 671);
        assert_eq!(mw.waning_learners, 11);
        assert_eq!(mw.waning_acquittals, 163);
        assert_eq!(mw.waning_expired_taught, 62);
        assert_eq!(mw.waning_expired_earned, 9);
        let (_, cw) = run_room(Room::CoincidentWide, SEED);
        assert_eq!(cw.fed, 59);
        assert_eq!(cw.burns, 15);
        assert_eq!(cw.checks, 91);
        assert_eq!(cw.waning_events, 631);
        assert_eq!(cw.waning_learners, 11);
        assert_eq!(cw.waning_acquittals, 143);
        assert_eq!(cw.waning_expired_taught, 11);
        assert_eq!(cw.waning_expired_earned, 7);
    }

    #[test]
    fn step_packets_travel_in_both_kinds_in_each_step_room() {
        for room in [Room::StaggeredStep, Room::MirroredStep, Room::CoincidentStep] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular step packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady step packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a step packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn step_offsets_are_as_declared() {
        assert_eq!(Room::StaggeredStep.taught_offset(), 450);
        assert_eq!(Room::StaggeredStep.earned_offset(), 0);
        assert_eq!(Room::MirroredStep.taught_offset(), 0);
        assert_eq!(Room::MirroredStep.earned_offset(), 450);
        assert_eq!(Room::CoincidentStep.taught_offset(), 450);
        assert_eq!(Room::CoincidentStep.earned_offset(), 450);
        assert_eq!(Room::StaggeredStep.waning_lifetime(), Some(750));
        assert_eq!(Room::MirroredStep.waning_lifetime(), Some(750));
        assert_eq!(Room::CoincidentStep.waning_lifetime(), Some(750));
    }

    #[test]
    fn step_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::StaggeredStep, Room::MirroredStep, Room::CoincidentStep] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_step_performance_is_deterministic() {
        let (_, a) = run_room(Room::MirroredStep, SEED);
        let (_, b) = run_room(Room::MirroredStep, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_step_rooms_still_feed_live_and_burn() {
        for room in [Room::StaggeredStep, Room::MirroredStep, Room::CoincidentStep] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn step_totals_pinned() {
        // Measured at planting, pinned as found. The step role
        // separates by exactly one burn on the taught side:
        // staggered-step burns 10, one below staggered (+375,
        // 11) and five below staggered-wide (+750, 15), with
        // its own events (700) and taught census (35, against
        // 48 at +375 and 11 at +750). coincident-step shares
        // staggered-step's headline entire (fed 54, burns 10,
        // checks 101, events 700, acquittals 169, nights
        // identical) and differs only in the earned census
        // (10 against 15) -- the control collapses onto the
        // taught stagger at the step, as at +375. mirrored-step
        // reads waning-750's headline for the fourth time in
        // its family (fed 55, burns 12, checks 97, events 671
        // exact, split 349/322 exact) and is identical to
        // mirrored (+375) in every headline field and both
        // censuses (62/11), differing only in acquittals
        // (162, the same as mirrored's own 162; waning-750
        // reads 161). The mirror gap at the step is 2 here
        // (12 against 10), the cave's own reading.
        let (_, ss) = run_room(Room::StaggeredStep, SEED);
        assert_eq!(ss.fed, 54);
        assert_eq!(ss.burns, 10);
        assert_eq!(ss.checks, 101);
        assert_eq!(ss.waning_events, 700);
        assert_eq!(ss.waning_learners, 11);
        assert_eq!(ss.waning_acquittals, 169);
        assert_eq!(ss.waning_expired_taught, 35);
        assert_eq!(ss.waning_expired_earned, 15);
        let (_, ms) = run_room(Room::MirroredStep, SEED);
        assert_eq!(ms.fed, 55);
        assert_eq!(ms.burns, 12);
        assert_eq!(ms.checks, 97);
        assert_eq!(ms.waning_events, 671);
        assert_eq!(ms.waning_learners, 11);
        assert_eq!(ms.waning_acquittals, 162);
        assert_eq!(ms.waning_expired_taught, 62);
        assert_eq!(ms.waning_expired_earned, 11);
        let (_, cs) = run_room(Room::CoincidentStep, SEED);
        assert_eq!(cs.fed, 54);
        assert_eq!(cs.burns, 10);
        assert_eq!(cs.checks, 101);
        assert_eq!(cs.waning_events, 700);
        assert_eq!(cs.waning_learners, 11);
        assert_eq!(cs.waning_acquittals, 169);
        assert_eq!(cs.waning_expired_taught, 35);
        assert_eq!(cs.waning_expired_earned, 10);
    }

    #[test]
    fn edge_packets_travel_in_both_kinds_in_each_edge_room() {
        for room in [Room::StaggeredEdge, Room::MirroredEdge, Room::CoincidentEdge] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular edge packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady edge packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from an edge packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn edge_offsets_are_as_declared() {
        assert_eq!(Room::StaggeredEdge.taught_offset(), 600);
        assert_eq!(Room::StaggeredEdge.earned_offset(), 0);
        assert_eq!(Room::MirroredEdge.taught_offset(), 0);
        assert_eq!(Room::MirroredEdge.earned_offset(), 600);
        assert_eq!(Room::CoincidentEdge.taught_offset(), 600);
        assert_eq!(Room::CoincidentEdge.earned_offset(), 600);
        assert_eq!(Room::StaggeredEdge.waning_lifetime(), Some(750));
        assert_eq!(Room::MirroredEdge.waning_lifetime(), Some(750));
        assert_eq!(Room::CoincidentEdge.waning_lifetime(), Some(750));
    }

    #[test]
    fn edge_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::StaggeredEdge, Room::MirroredEdge, Room::CoincidentEdge] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_edge_performance_is_deterministic() {
        let (_, a) = run_room(Room::MirroredEdge, SEED);
        let (_, b) = run_room(Room::MirroredEdge, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_edge_rooms_still_feed_live_and_burn() {
        for room in [Room::StaggeredEdge, Room::MirroredEdge, Room::CoincidentEdge] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn edge_totals_pinned() {
        // Measured at planting, pinned as found. The edge role
        // does NOT turn on this stage: staggered-edge burns 10,
        // exactly the step's price, sharing staggered-step's
        // headline entire (fed 54, burns 10, checks 101, events
        // 700, split 366/334, nights identical) and differing
        // in acquittals (175 against 169) and the taught
        // census (21, against 35 at the step and 11 at the
        // wide offset -- the census keeps resolving while the
        // price does not move). coincident-edge follows the
        // taught stagger down entire again (fed 54, burns 10,
        // checks 101, events 700, acquittals 175), differing
        // only in the earned census (10 against 15).
        // mirrored-edge reads waning-750's headline for the
        // fifth time in its family (fed 55, burns 12, checks
        // 97, events 671 exact, split 349/322 exact), its
        // earned census now 10 (16 / 11 / 11 / 9 / 10 across
        // waning / +375 / +450 / +750 / +600). The mirror gap
        // at the edge is 2 here (12 against 10), as at the
        // step -- in the cave it read -4.
        let (_, se) = run_room(Room::StaggeredEdge, SEED);
        assert_eq!(se.fed, 54);
        assert_eq!(se.burns, 10);
        assert_eq!(se.checks, 101);
        assert_eq!(se.waning_events, 700);
        assert_eq!(se.waning_learners, 11);
        assert_eq!(se.waning_acquittals, 175);
        assert_eq!(se.waning_expired_taught, 21);
        assert_eq!(se.waning_expired_earned, 15);
        let (_, me) = run_room(Room::MirroredEdge, SEED);
        assert_eq!(me.fed, 55);
        assert_eq!(me.burns, 12);
        assert_eq!(me.checks, 97);
        assert_eq!(me.waning_events, 671);
        assert_eq!(me.waning_learners, 11);
        assert_eq!(me.waning_acquittals, 163);
        assert_eq!(me.waning_expired_taught, 62);
        assert_eq!(me.waning_expired_earned, 10);
        let (_, ce) = run_room(Room::CoincidentEdge, SEED);
        assert_eq!(ce.fed, 54);
        assert_eq!(ce.burns, 10);
        assert_eq!(ce.checks, 101);
        assert_eq!(ce.waning_events, 700);
        assert_eq!(ce.waning_learners, 11);
        assert_eq!(ce.waning_acquittals, 175);
        assert_eq!(ce.waning_expired_taught, 21);
        assert_eq!(ce.waning_expired_earned, 10);
    }

    #[test]
    fn face_packets_travel_in_both_kinds_in_each_face_room() {
        for room in [Room::StaggeredFace, Room::MirroredFace, Room::CoincidentFace] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular face packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady face packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a face packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn face_offsets_are_as_declared() {
        assert_eq!(Room::StaggeredFace.taught_offset(), 525);
        assert_eq!(Room::StaggeredFace.earned_offset(), 0);
        assert_eq!(Room::MirroredFace.taught_offset(), 0);
        assert_eq!(Room::MirroredFace.earned_offset(), 525);
        assert_eq!(Room::CoincidentFace.taught_offset(), 525);
        assert_eq!(Room::CoincidentFace.earned_offset(), 525);
        assert_eq!(Room::StaggeredFace.waning_lifetime(), Some(750));
        assert_eq!(Room::MirroredFace.waning_lifetime(), Some(750));
        assert_eq!(Room::CoincidentFace.waning_lifetime(), Some(750));
    }

    #[test]
    fn face_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::StaggeredFace, Room::MirroredFace, Room::CoincidentFace] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_face_performance_is_deterministic() {
        let (_, a) = run_room(Room::MirroredFace, SEED);
        let (_, b) = run_room(Room::MirroredFace, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_face_rooms_still_feed_live_and_burn() {
        for room in [Room::StaggeredFace, Room::MirroredFace, Room::CoincidentFace] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn grain_packets_travel_in_both_kinds_in_each_grain_room() {
        for room in [Room::Staggered475, Room::Mirrored475, Room::Coincident475, Room::Staggered500, Room::Mirrored500, Room::Coincident500] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular grain packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady grain packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a grain packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn grain_offsets_are_as_declared() {
        assert_eq!(Room::Staggered475.taught_offset(), 475);
        assert_eq!(Room::Staggered475.earned_offset(), 0);
        assert_eq!(Room::Mirrored475.taught_offset(), 0);
        assert_eq!(Room::Mirrored475.earned_offset(), 475);
        assert_eq!(Room::Coincident475.taught_offset(), 475);
        assert_eq!(Room::Coincident475.earned_offset(), 475);
        assert_eq!(Room::Staggered500.taught_offset(), 500);
        assert_eq!(Room::Staggered500.earned_offset(), 0);
        assert_eq!(Room::Mirrored500.taught_offset(), 0);
        assert_eq!(Room::Mirrored500.earned_offset(), 500);
        assert_eq!(Room::Coincident500.taught_offset(), 500);
        assert_eq!(Room::Coincident500.earned_offset(), 500);
        assert_eq!(Room::Staggered475.waning_lifetime(), Some(750));
        assert_eq!(Room::Mirrored475.waning_lifetime(), Some(750));
        assert_eq!(Room::Coincident475.waning_lifetime(), Some(750));
        assert_eq!(Room::Staggered500.waning_lifetime(), Some(750));
        assert_eq!(Room::Mirrored500.waning_lifetime(), Some(750));
        assert_eq!(Room::Coincident500.waning_lifetime(), Some(750));
    }

    #[test]
    fn grain_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::Staggered475, Room::Mirrored475, Room::Coincident475, Room::Staggered500, Room::Mirrored500, Room::Coincident500] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_grain_performance_is_deterministic() {
        let (_, a) = run_room(Room::Mirrored500, SEED);
        let (_, b) = run_room(Room::Mirrored500, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_grain_rooms_still_feed_live_and_burn() {
        for room in [Room::Staggered475, Room::Mirrored475, Room::Coincident475, Room::Staggered500, Room::Mirrored500, Room::Coincident500] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn grain_totals_pinned() {
        // Measured at planting, pinned as found. The grains
        // do NOT stair on this stage: staggered-475 burns 10
        // and staggered-500 burns 10, the step price entire
        // at both finer grains (the cave read 14 and 16 at
        // these very addresses). staggered-475 reproduces
        // staggered-step field for field entire -- fed 54,
        // burns 10, checks 101, events 700, split 366/334,
        // acquittals 169, census 35/15: at 25 owner-ticks
        // past the step this stage cannot tell the grain
        // room from the step room at all. staggered-500
        // keeps the same headline and separates only in the
        // bookkeeping: acquittals 171, taught census 33 --
        // between the step's 35 and the face's 32, so the
        // census DOES resolve inside the interval's first
        // half, one step of it (35 -> 33 -> 32 -> 21 across
        // +450 / +500 / +525 / +600), while the price does
        // not move at any grain (+450, +475, +500, +525,
        // +600 all read 10). The coincident rooms follow
        // their taught staggers entire again (475: 169
        // acquittals, census 35/10; 500: 171, census 33/10),
        // cracking P5 against their mirrors for the fourth
        // and fifth times. mirrored-475 and mirrored-500
        // both read waning-750's headline for the seventh
        // and eighth times in their family (fed 55, burns
        // 12, checks 97, events 671 exact, split 349/322
        // exact, taught census 62), their earned census 11
        // at both grains (16 / 11 / 11 / 9 / 10 / 11 / 11 /
        // 11 across waning / +375 / +450 / +750 / +600 /
        // +525 / +475 / +500). The mirror gaps at the
        // grains are 2 and 2 here; in the cave they read
        // -3 and -5.
        let (_, s) = run_room(Room::Staggered475, SEED);
        assert_eq!(s.fed, 54);
        assert_eq!(s.burns, 10);
        assert_eq!(s.checks, 101);
        assert_eq!(s.waning_events, 700);
        assert_eq!(s.waning_learners, 11);
        assert_eq!(s.waning_acquittals, 169);
        assert_eq!(s.waning_expired_taught, 35);
        assert_eq!(s.waning_expired_earned, 15);
        let (_, m) = run_room(Room::Mirrored475, SEED);
        assert_eq!(m.fed, 55);
        assert_eq!(m.burns, 12);
        assert_eq!(m.checks, 97);
        assert_eq!(m.waning_events, 671);
        assert_eq!(m.waning_learners, 11);
        assert_eq!(m.waning_acquittals, 162);
        assert_eq!(m.waning_expired_taught, 62);
        assert_eq!(m.waning_expired_earned, 11);
        let (_, c) = run_room(Room::Coincident475, SEED);
        assert_eq!(c.fed, 54);
        assert_eq!(c.burns, 10);
        assert_eq!(c.checks, 101);
        assert_eq!(c.waning_events, 700);
        assert_eq!(c.waning_learners, 11);
        assert_eq!(c.waning_acquittals, 169);
        assert_eq!(c.waning_expired_taught, 35);
        assert_eq!(c.waning_expired_earned, 10);
        let (_, s) = run_room(Room::Staggered500, SEED);
        assert_eq!(s.fed, 54);
        assert_eq!(s.burns, 10);
        assert_eq!(s.checks, 101);
        assert_eq!(s.waning_events, 700);
        assert_eq!(s.waning_learners, 11);
        assert_eq!(s.waning_acquittals, 171);
        assert_eq!(s.waning_expired_taught, 33);
        assert_eq!(s.waning_expired_earned, 15);
        let (_, m) = run_room(Room::Mirrored500, SEED);
        assert_eq!(m.fed, 55);
        assert_eq!(m.burns, 12);
        assert_eq!(m.checks, 97);
        assert_eq!(m.waning_events, 671);
        assert_eq!(m.waning_learners, 11);
        assert_eq!(m.waning_acquittals, 162);
        assert_eq!(m.waning_expired_taught, 62);
        assert_eq!(m.waning_expired_earned, 11);
        let (_, c) = run_room(Room::Coincident500, SEED);
        assert_eq!(c.fed, 54);
        assert_eq!(c.burns, 10);
        assert_eq!(c.checks, 101);
        assert_eq!(c.waning_events, 700);
        assert_eq!(c.waning_learners, 11);
        assert_eq!(c.waning_acquittals, 171);
        assert_eq!(c.waning_expired_taught, 33);
        assert_eq!(c.waning_expired_earned, 10);
    }

    #[test]
    fn nearwide_packets_travel_in_both_kinds_in_each_new_room() {
        for room in [Room::Staggered675, Room::Mirrored675, Room::Coincident675] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_irregular > 0, "no irregular nearwide packet in {room:?}: {t:?}");
            assert!(t.waning_steady > 0, "no steady nearwide packet in {room:?}: {t:?}");
            assert_eq!(t.waning_irregular + t.waning_steady, t.waning_events);
            assert!(t.waning_learners > 0, "no moth first learned from a nearwide packet in {room:?}: {t:?}");
        }
    }

    #[test]
    fn nearwide_offsets_are_as_declared() {
        assert_eq!(Room::Staggered675.taught_offset(), 675);
        assert_eq!(Room::Staggered675.earned_offset(), 0);
        assert_eq!(Room::Mirrored675.taught_offset(), 0);
        assert_eq!(Room::Mirrored675.earned_offset(), 675);
        assert_eq!(Room::Coincident675.taught_offset(), 675);
        assert_eq!(Room::Coincident675.earned_offset(), 675);
        assert_eq!(Room::Staggered675.waning_lifetime(), Some(750));
        assert_eq!(Room::Mirrored675.waning_lifetime(), Some(750));
        assert_eq!(Room::Coincident675.waning_lifetime(), Some(750));
    }

    #[test]
    fn nearwide_both_pools_expire_and_no_dawn_tax() {
        for room in [Room::Staggered675, Room::Mirrored675, Room::Coincident675] {
            let (_, t) = run_room(room, SEED);
            assert!(t.waning_expired_taught > 0, "taught pool never waned in {room:?}: {t:?}");
            assert!(t.waning_expired_earned > 0, "earned pool never waned in {room:?}: {t:?}");
            assert_eq!(t.expired_taught + t.expired_earned, 0, "a dawn tax fired in {room:?}: {t:?}");
        }
    }

    #[test]
    fn the_nearwide_performance_is_deterministic() {
        let (_, a) = run_room(Room::Mirrored675, SEED);
        let (_, b) = run_room(Room::Mirrored675, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.waning_events, b.waning_events);
        assert_eq!(a.waning_expired_taught, b.waning_expired_taught);
        assert_eq!(a.waning_expired_earned, b.waning_expired_earned);
    }

    #[test]
    fn the_nearwide_rooms_still_feed_live_and_burn() {
        for room in [Room::Staggered675, Room::Mirrored675, Room::Coincident675] {
            let (_, t) = run_room(room, SEED);
            assert!(t.fed > 0, "{room:?} never fed: {t:?}");
            assert!(t.burns > 0, "{room:?} never burned: {t:?}");
            assert_eq!(t.alive_end, 24, "{room:?} lost a moth: {t:?}");
        }
    }

    #[test]
    fn nearwide_totals_pinned() {
        // Measured at planting, pinned as found. The last
        // interval has no interior on this stage either --
        // and it is not a slope: staggered-675 reproduces
        // staggered-wide ENTIRE (fed 59, burns 15, checks
        // 91, events 631, split 284/347, acquittals 141,
        // census 11/16), so the five-burn tax and the
        // census's whole fall (21 -> 11, acquittals 175 ->
        // 141) both arrive somewhere in (600, 675], west
        // of the interval's midpoint -- P3 cracked (15, not
        // 10..=11) and P7 cracked the arriving way (census
        // 11 entire, not 12..=20 ahead of the price).
        // coincident-675 follows the wide control's
        // headline entire too (59 / 15 / 91 / 631,
        // acquittals 143, taught census 11), its earned
        // census 9 against coincident-wide's 7 -- and P5
        // HELD for the first time in six addresses,
        // because at the taxed price the control no longer
        // falls below its mirror. mirrored-675 reads the
        // waning-750 headline for the ninth time in its
        // family (55 / 12 / 97 / 671, split 349/322,
        // acquittals 163, taught census 62), its earned
        // census 10 extending the sequence 16 / 11 / 11 /
        // 9 / 10 / 11 / 11 / 11 / 10.
        let (_, s) = run_room(Room::Staggered675, SEED);
        assert_eq!(s.fed, 59);
        assert_eq!(s.burns, 15);
        assert_eq!(s.checks, 91);
        assert_eq!(s.waning_events, 631);
        assert_eq!(s.waning_learners, 11);
        assert_eq!(s.waning_acquittals, 141);
        assert_eq!(s.waning_expired_taught, 11);
        assert_eq!(s.waning_expired_earned, 16);
        let (_, m) = run_room(Room::Mirrored675, SEED);
        assert_eq!(m.fed, 55);
        assert_eq!(m.burns, 12);
        assert_eq!(m.checks, 97);
        assert_eq!(m.waning_events, 671);
        assert_eq!(m.waning_learners, 11);
        assert_eq!(m.waning_acquittals, 163);
        assert_eq!(m.waning_expired_taught, 62);
        assert_eq!(m.waning_expired_earned, 10);
        let (_, c) = run_room(Room::Coincident675, SEED);
        assert_eq!(c.fed, 59);
        assert_eq!(c.burns, 15);
        assert_eq!(c.checks, 91);
        assert_eq!(c.waning_events, 631);
        assert_eq!(c.waning_learners, 11);
        assert_eq!(c.waning_acquittals, 143);
        assert_eq!(c.waning_expired_taught, 11);
        assert_eq!(c.waning_expired_earned, 9);
    }

    #[test]
    fn face_totals_pinned() {
        // Measured at planting, pinned as found. The face
        // does NOT cliff on this stage: staggered-face burns
        // 10, the step and edge price entire, sharing their
        // headline field for field (fed 54, burns 10, checks
        // 101, events 700, split 366/334, nights identical)
        // and differing only in acquittals (172, between the
        // step's 169 and the edge's 175) and the taught
        // census 32 (35 at the step, 21 at the edge, 11 at
        // the wide offset -- the census resolves inside the
        // interval while the price does not move).
        // coincident-face follows the taught stagger down
        // entire again (fed 54, burns 10, checks 101, events
        // 700, acquittals 172), differing only in the earned
        // census (10 against 15). mirrored-face reads
        // waning-750's headline for the sixth time in its
        // family (fed 55, burns 12, checks 97, events 671
        // exact, split 349/322 exact), its earned census now
        // 11 (16 / 11 / 11 / 9 / 10 / 11 across waning /
        // +375 / +450 / +750 / +600 / +525). The mirror gap
        // at the face is 2 here (12 against 10), as at the
        // step and the edge -- in the cave it read -5.
        let (_, s) = run_room(Room::StaggeredFace, SEED);
        assert_eq!(s.fed, 54);
        assert_eq!(s.burns, 10);
        assert_eq!(s.checks, 101);
        assert_eq!(s.waning_events, 700);
        assert_eq!(s.waning_learners, 11);
        assert_eq!(s.waning_acquittals, 172);
        assert_eq!(s.waning_expired_taught, 32);
        assert_eq!(s.waning_expired_earned, 15);
        let (_, m) = run_room(Room::MirroredFace, SEED);
        assert_eq!(m.fed, 55);
        assert_eq!(m.burns, 12);
        assert_eq!(m.checks, 97);
        assert_eq!(m.waning_events, 671);
        assert_eq!(m.waning_learners, 11);
        assert_eq!(m.waning_acquittals, 162);
        assert_eq!(m.waning_expired_taught, 62);
        assert_eq!(m.waning_expired_earned, 11);
        let (_, c) = run_room(Room::CoincidentFace, SEED);
        assert_eq!(c.fed, 54);
        assert_eq!(c.burns, 10);
        assert_eq!(c.checks, 101);
        assert_eq!(c.waning_events, 700);
        assert_eq!(c.waning_learners, 11);
        assert_eq!(c.waning_acquittals, 172);
        assert_eq!(c.waning_expired_taught, 32);
        assert_eq!(c.waning_expired_earned, 10);
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
