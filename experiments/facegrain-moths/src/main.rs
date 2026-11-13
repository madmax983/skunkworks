//! # Facegrain Moths 🪔
//!
//! Brought by The Moth Keeper. Parent: `crates/resonance-audio`
//! (`PhysicsGrid`, a 2D FDTD wave solver), via `cliffface-moths`.
//!
//! `cliffface-moths` stood on the cliff's face at +525 and found
//! it early and sheer: staggered-face burned 16, the
//! unstaggered waning-750 price entire, coincident-face burned
//! 16 beside it, and mirrored-face held 11 -- the earned price
//! flat across +375 / +450 / +525, its single step somewhere
//! between +525 and +600. The shelf that ran flat from +375
//! through the claim-step (+450, burns 9) falls the whole seven
//! burns somewhere in the 75 ticks between the claim-step and
//! +525, and no room has yet stood inside that interval.
//! Cliffface's closing seed names the rooms planted here
//! verbatim: *the face at finer grain -- the stagger at +475
//! and at +500: the fall lives somewhere in the 75 ticks
//! between the claim-step (burns 9) and +525 (burns 16); does
//! any offset in there read an intermediate price (10..=15),
//! or is the fall whole at every grain, and does the coincident
//! face stand at the same tick as the taught face or merely in
//! the same interval?*
//!
//! The change, exactly six rooms: caution is booked by source
//! exactly as in `cliffface-moths`, every unit carries its gain
//! tick on the dawn-crossing global clock, both pools expire
//! at lifetime 750, and the one offset is set to +475 ticks
//! (25 ticks past the claim-step, one third into the interval)
//! or to +500 ticks (50 ticks past, two thirds in):
//!
//! - **staggered-475** -- taught units expire at gain + 750 +
//!   475; earned units at gain + 750.
//! - **mirrored-475** -- earned units expire at gain + 750 +
//!   475; taught units at gain + 750. The mirror, at +475.
//! - **coincident-475** (control) -- BOTH deadlines offset by
//!   +475 ticks; the deadlines stay coincident at the longest
//!   deadline (1975), between the step control's (1950) and
//!   the face control's (2025); only duration changes.
//! - **staggered-500**, **mirrored-500**, **coincident-500** --
//!   the same three rooms at +500 (longest deadline 2000).
//!
//! The packets, the re-judgement, the aversion, and the
//! acquittal rule are the evidence room's, verbatim. Thirty-
//! eight twin rooms, same seed: the thirty-two parents,
//! unchanged, plus the six grain rooms.
//!
//! Predictions, declared before the first run:
//! P1: the channel works at both grains -- in each grain room,
//! packets are witnessed in both kinds (irregular > 0,
//! steady > 0) and grain-first learners number > 0.
//! P2 (the offsets are real): each grain room expires BOTH
//! pools (> 0 taught and > 0 earned) and no dawn tax fires in
//! it. The offset values are asserted by test (475 on the
//! waiting pool, 0 on the other, 475/475 in its control;
//! likewise 500).
//! P3 (the fall is whole at the finest grain yet): staggered
//! burns land in 14..=16 at BOTH +475 and +500 -- the face
//! stands inside the first 25 ticks past the claim-step, and
//! no offset inside the interval reads an intermediate price
//! (10..=15). A landing at 9..=11 at either grain says the
//! shelf holds past that grain; a landing at 12..=13 says the
//! fall has a landing this planting denies; either cracks P3.
//! P4 (the earned price is still flat): mirrored burns read
//! exactly 11 at BOTH grains -- the earned schedule's single
//! step lives between +525 and +600, east of both rooms. A
//! mirrored landing of 12 at either grain moves that step
//! west and cracks P4.
//! P5 (the coincident face is the same address): coincident
//! burns land in 14..=16 at BOTH grains -- the coincident
//! line's cliff stands in the same first 25 ticks as the
//! taught cliff, not merely in the same 75-tick interval. A
//! landing at 12..=13 at either grain separates the two
//! faces' addresses and cracks P5.
//! P6: no grain room starves the channel or the table --
//! waning events >= 300 and fed >= 43 in each (the evidence
//! room's table).
//! P7 (the two grains agree): staggered-475 burns equal
//! staggered-500 burns AND coincident-475 burns equal
//! coincident-500 burns -- once the face is passed, the
//! interval's interior is one price, and the 25 ticks between
//! the grains move only bookkeeping. Any disagreement cracks
//! P7 and gives the interval an interior structure.
//!
//! Every verdict is reported in the README as found.

use anyhow::Result;
use std::collections::VecDeque;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use resonance_audio::{Material, PhysicsGrid};

/// Room width in cells.
const WIDTH: usize = 56;
/// Room height in cells.
const HEIGHT: usize = 28;
/// The honest lamp's position (west wall).
const HONEST_POS: (usize, usize) = (9, 14);
/// The decoy lamp's position (east wall).
const DECOY_POS: (usize, usize) = (46, 14);
/// The honest lamp's emission period, in ticks. Perfectly regular.
const HONEST_PERIOD: u64 = 16;
/// The honest lamp's pluck strength — modest wattage.
const HONEST_STRENGTH: f32 = 0.9;
/// The decoy's alternating emission intervals. Their mean equals
/// the honest period; their *disagreement* is the signature.
const DECOY_SHORT: u64 = 6;
/// See [`DECOY_SHORT`].
const DECOY_LONG: u64 = 26;
/// The decoy's pluck strength — the brighter claim.
const DECOY_STRENGTH: f32 = 2.2;
/// Moths in the room each night.
const MOTHS: usize = 24;
/// Nights observed. The population carries its burns — and its
/// caution — from night to night; only positions reset.
const NIGHTS: usize = 3;
/// Ticks per night.
const NIGHT_TICKS: u64 = 1500;
/// Distance from a lamp centre at which the keeper applies the
/// lamp's outcome (feed or burn). The moths never see this number.
const ARRIVE_RADIUS: f64 = 2.4;
/// Smoothed brightness at a moth's cell that says "a lamp is loud
/// here" — loud enough to be worth a cautious moth's stopped time.
const CHECK_TRIGGER: f32 = 0.10;
/// How long a checking moth stands still and listens, at most.
const CHECK_TICKS: u64 = 96;
/// Peak-interval coefficient of variation above which a flicker is
/// judged irregular — a decoy's signature.
const FLICKER_CV: f64 = 0.35;
/// Brightness peak threshold for flicker timing (on the moth's
/// smoothed, saturation-clipped perception).
const PEAK_THRESH: f32 = 0.16;
/// The moth sensor's saturation point: brightness above this is
/// clipped, so the decoy's extra wattage buys it no extra *detail* —
/// only extra pull on moths too far away to be saturated.
const SENSOR_SATURATION: f32 = 1.0;
/// Health a burn costs.
const BURN_DAMAGE: i32 = 35;
/// Health a feeding restores.
const FEED_RESTORE: i32 = 25;
/// Ticks a refused moth flees and will not re-approach.
const AVERSION_TICKS: u64 = 150;
/// Ticks after a check before the same moth will stop to check again.
const CHECK_COOLDOWN: u64 = 220;
/// Distance (cells) from a burn at which a moth can witness it:
/// see the flash and the knockback, earn caution, pay no health.
/// Beyond this radius a burn is private. Witnessing teaches
/// *that* a claim can burn, never *which* lamp did it.
const WITNESS_RADIUS: f64 = 12.0;
/// Distance (cells) from a refusal at which a moth can witness it:
/// see a neighbour stop, judge, and flee. Chain witnessing teaches
/// *that* a claim was refused nearby, never which lamp it was.
const CHAIN_RADIUS: f64 = 12.0;

/// Coefficient of variation of a peak-interval sample: the
/// flicker judge every moth uses, first-hand or by evidence.
fn cv_of(iv: &[u64]) -> f64 {
    if iv.len() < 2 { return 0.0; }
    let mean = iv.iter().sum::<u64>() as f64 / iv.len() as f64;
    if mean <= 0.0 { return 0.0; }
    let var = iv.iter().map(|&v| (v as f64 - mean).powi(2)).sum::<f64>() / iv.len() as f64;
    var.sqrt() / mean
}

/// Tuition system under test.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tuition {
    /// keeper-moths rule: caution only from your own burns.
    Solo,
    /// Caution also from witnessed burns within radius.
    Witness,
    /// Caution also from witnessed burns AND witnessed refusals.
    Chain,
    /// Caution also from witnessed refusals, with the recoil
    /// directed away from the refused lamp (the refuser's
    /// gradient direction at refusal), not the refuser's place.
    Placed,
    /// Evidence packets from every completed check: witnesses
    /// re-judge the transmitted peak intervals themselves.
    Evidence,
    /// Evidence packets, throttled: a checker transmits only its
    /// first completed check each night; later checks are
    /// withheld from the room.
    Throttled,
    /// Evidence packets, unshoved: every completed check transmits
    /// its peak intervals exactly as in the evidence room, and
    /// witnesses re-judge them -- but an irregular verdict earns
    /// caution WITHOUT the 150-tick aversion. Caution as
    /// information, not as a recoil.
    Unshoved,
    /// Evidence packets, shoved: every completed check transmits
    /// its peak intervals exactly as in the evidence room, and
    /// witnesses re-judge them -- but an irregular verdict earns
    /// the 150-tick aversion WITHOUT caution. The shove travels;
    /// the lesson does not. No checker is recruited, and a steady
    /// verdict earns nothing (no acquittal channel: no hearsay
    /// caution exists in this room to acquit).
    Shoved,
    /// Evidence packets, fading-taught: witnesses re-judge
    /// exactly as in the evidence room and taught caution is
    /// booked in its own pool -- and at each dawn after the
    /// first, the taught pool expires to zero. Earned caution
    /// (own burns, witnessed burns) persists.
    FadingTaught,
    /// Evidence packets, fading-earned: the mirror tax -- at
    /// each dawn the earned pool expires to zero and taught
    /// caution persists.
    FadingEarned,
    /// Evidence packets, fading-both: at each dawn both pools
    /// expire; only scars and health carry across the dawn.
    FadingBoth,
    /// Evidence packets, waning-1500: caution booked by source
    /// as in the fading rooms, but every unit expires exactly
    /// 1500 ticks after it was gained, on a clock that runs
    /// across dawns. No dawn tax.
    Waning1500,
    /// Evidence packets, waning-750: unit lifetime 750 ticks.
    Waning750,
    /// Evidence packets, waning-375: unit lifetime 375 ticks.
    Waning375,
    /// Evidence packets, waning-150: unit lifetime 150 ticks.
    Waning150,
    /// Evidence packets, pooled-taught: unit lifetime 750 ticks
    /// for the TAUGHT pool only; earned units never expire.
    PooledTaught,
    /// Evidence packets, pooled-earned: unit lifetime 750 ticks
    /// for the EARNED pool only; taught units never expire.
    PooledEarned,
    /// Evidence packets, staggered: both pools at lifetime 750,
    /// the TAUGHT pool's expiry deadline offset by +375 ticks.
    Staggered,
    /// Evidence packets, staggered-both (control): both pools at
    /// lifetime 750, BOTH deadlines offset by +375 ticks, so the
    /// deadlines stay coincident; only duration changes.
    StaggeredBoth,
    /// Evidence packets, mirrored: both pools at lifetime 750,
    /// the EARNED pool's expiry deadline offset by +375 ticks --
    /// the mirror image of the staggered room.
    Mirrored,
    /// Evidence packets, staggered-wide: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +750
    /// ticks -- the stagger widened past a whole journey leg.
    StaggeredWide,
    /// Evidence packets, mirrored-wide: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +750
    /// ticks -- the mirror stagger at the wide offset.
    MirroredWide,
    /// Evidence packets, coincident-wide (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +750 ticks, so
    /// the deadlines stay coincident; only duration changes.
    CoincidentWide,
    /// Evidence packets, staggered-step: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +450
    /// ticks -- the stagger at exactly the claim-step.
    StaggeredStep,
    /// Evidence packets, mirrored-step: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +450
    /// ticks -- the mirror stagger at the claim-step.
    MirroredStep,
    /// Evidence packets, coincident-step (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +450 ticks, so
    /// the deadlines stay coincident; only duration changes.
    CoincidentStep,
    /// Evidence packets, staggered-edge: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +600
    /// ticks -- the stagger midway between the claim-step and
    /// the wide offset, on the shelf's edge.
    StaggeredEdge,
    /// Evidence packets, mirrored-edge: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +600
    /// ticks -- the mirror stagger at the shelf's edge.
    MirroredEdge,
    /// Evidence packets, coincident-edge (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +600 ticks, so
    /// the deadlines stay coincident; only duration changes.
    CoincidentEdge,
    /// Evidence packets, staggered-face: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +525
    /// ticks -- the stagger midway between the claim-step and
    /// the edge, standing on the cliff's face.
    StaggeredFace,
    /// Evidence packets, mirrored-face: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +525
    /// ticks -- the mirror stagger at the cliff's face.
    MirroredFace,
    /// Evidence packets, coincident-face (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +525 ticks, so
    /// the deadlines stay coincident; only duration changes.
    CoincidentFace,
    /// Evidence packets, staggered-475: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +475
    /// ticks -- 25 ticks past the claim-step, one third into
    /// the interval between the claim-step and the face.
    Staggered475,
    /// Evidence packets, mirrored-475: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +475
    /// ticks -- the mirror stagger at the finer grain.
    Mirrored475,
    /// Evidence packets, coincident-475 (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +475 ticks, so
    /// the deadlines stay coincident; only duration changes.
    Coincident475,
    /// Evidence packets, staggered-500: both pools at lifetime
    /// 750, the TAUGHT pool's expiry deadline offset by +500
    /// ticks -- 50 ticks past the claim-step, two thirds into
    /// the interval between the claim-step and the face.
    Staggered500,
    /// Evidence packets, mirrored-500: both pools at lifetime
    /// 750, the EARNED pool's expiry deadline offset by +500
    /// ticks -- the mirror stagger at the finer grain.
    Mirrored500,
    /// Evidence packets, coincident-500 (control): both pools
    /// at lifetime 750, BOTH deadlines offset by +500 ticks, so
    /// the deadlines stay coincident; only duration changes.
    Coincident500,
}

impl Tuition {
    /// The within-night decay clock: the lifetime, in ticks, of
    /// a caution unit in a waning room; `None` everywhere else.
    fn waning_lifetime(self) -> Option<u64> {
        match self {
            Tuition::Waning1500 => Some(1500),
            Tuition::Waning750 => Some(750),
            Tuition::Waning375 => Some(375),
            Tuition::Waning150 => Some(150),
            Tuition::PooledTaught => Some(750),
            Tuition::PooledEarned => Some(750),
            Tuition::Staggered => Some(750),
            Tuition::StaggeredBoth => Some(750),
            Tuition::Mirrored => Some(750),
            Tuition::StaggeredWide => Some(750),
            Tuition::MirroredWide => Some(750),
            Tuition::CoincidentWide => Some(750),
            Tuition::StaggeredStep => Some(750),
            Tuition::MirroredStep => Some(750),
            Tuition::CoincidentStep => Some(750),
            Tuition::StaggeredEdge => Some(750),
            Tuition::MirroredEdge => Some(750),
            Tuition::CoincidentEdge => Some(750),
            Tuition::StaggeredFace => Some(750),
            Tuition::MirroredFace => Some(750),
            Tuition::CoincidentFace => Some(750),
            Tuition::Staggered475 => Some(750),
            Tuition::Mirrored475 => Some(750),
            Tuition::Coincident475 => Some(750),
            Tuition::Staggered500 => Some(750),
            Tuition::Mirrored500 => Some(750),
            Tuition::Coincident500 => Some(750),
            _ => None,
        }
    }

    fn is_waning(self) -> bool {
        self.waning_lifetime().is_some()
    }

    /// Whether the TAUGHT pool expires by age in this room:
    /// every waning room, plus pooled-taught; never
    /// pooled-earned (its taught units persist forever).
    fn expires_taught(self) -> bool {
        self.is_waning() && self != Tuition::PooledEarned
    }

    /// Whether the EARNED pool expires by age in this room:
    /// every waning room, plus pooled-earned; never
    /// pooled-taught (its earned units persist forever).
    fn expires_earned(self) -> bool {
        self.is_waning() && self != Tuition::PooledTaught
    }

    /// The taught pool's expiry-deadline offset, in ticks:
    /// +375 in both staggered rooms, +750 in staggered-wide and
    /// in the coincident-wide control, +450 in staggered-step
    /// and in the coincident-step control, +600 in staggered-
    /// edge and in the coincident-edge control, +525 in
    /// staggered-face and in the coincident-face control, +475
    /// in staggered-475 and in the coincident-475 control, +500
    /// in staggered-500 and in the coincident-500 control, zero
    /// everywhere else (the mirrored rooms' taught pools wait
    /// for nothing).
    fn taught_offset(self) -> u64 {
        match self {
            Tuition::Staggered | Tuition::StaggeredBoth => 375,
            Tuition::StaggeredWide | Tuition::CoincidentWide => 750,
            Tuition::StaggeredStep | Tuition::CoincidentStep => 450,
            Tuition::StaggeredEdge | Tuition::CoincidentEdge => 600,
            Tuition::StaggeredFace | Tuition::CoincidentFace => 525,
            Tuition::Staggered475 | Tuition::Coincident475 => 475,
            Tuition::Staggered500 | Tuition::Coincident500 => 500,
            _ => 0,
        }
    }

    /// The earned pool's expiry-deadline offset, in ticks:
    /// +375 in the staggered-both control and in the mirrored
    /// room, +750 in mirrored-wide and in the coincident-wide
    /// control, +450 in mirrored-step and in the coincident-
    /// step control, +600 in mirrored-edge and in the
    /// coincident-edge control, +525 in mirrored-face and in
    /// the coincident-face control, +475 in mirrored-475 and in
    /// the coincident-475 control, +500 in mirrored-500 and in
    /// the coincident-500 control, zero everywhere else.
    fn earned_offset(self) -> u64 {
        match self {
            Tuition::StaggeredBoth | Tuition::Mirrored => 375,
            Tuition::MirroredWide | Tuition::CoincidentWide => 750,
            Tuition::MirroredStep | Tuition::CoincidentStep => 450,
            Tuition::MirroredEdge | Tuition::CoincidentEdge => 600,
            Tuition::MirroredFace | Tuition::CoincidentFace => 525,
            Tuition::Mirrored475 | Tuition::Coincident475 => 475,
            Tuition::Mirrored500 | Tuition::Coincident500 => 500,
            _ => 0,
        }
    }
}

/// What the keeper (never the moth) knows happened in a night.
#[derive(Debug, Default, Clone, Copy)]
struct NightLedger {
    fed: usize,
    decoy_arrivals: usize,
    burns: usize,
    deaths: usize,
    checks: usize,
    refusals: usize,
    steady_verdicts: usize,
    alive_end: usize,
    witness_events: usize,
    new_witness_learners: usize,
    chain_events: usize,
    new_chain_learners: usize,
    placed_events: usize,
    new_placed_learners: usize,
    evidence_events: usize,
    new_evidence_learners: usize,
    evidence_acquittals: usize,
    evidence_irregular: usize,
    evidence_steady: usize,
    throttled_events: usize,
    new_throttled_learners: usize,
    throttled_acquittals: usize,
    throttled_irregular: usize,
    throttled_steady: usize,
    throttled_packets_sent: usize,
    throttled_suppressed: usize,
    unshoved_events: usize,
    new_unshoved_learners: usize,
    unshoved_acquittals: usize,
    unshoved_irregular: usize,
    unshoved_steady: usize,
    shoved_events: usize,
    new_shoved_first: usize,
    shoved_acquittals: usize,
    shoved_irregular: usize,
    shoved_steady: usize,
    fading_events: usize,
    new_fading_learners: usize,
    fading_acquittals: usize,
    fading_irregular: usize,
    fading_steady: usize,
    expired_taught: usize,
    expired_earned: usize,
    waning_events: usize,
    new_waning_learners: usize,
    waning_acquittals: usize,
    waning_irregular: usize,
    waning_steady: usize,
    waning_expired_taught: usize,
    waning_expired_earned: usize,
}

#[derive(Clone, Debug)]
struct Moth {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    health: i32,
    /// Burns survived. Caution is *earned*, never granted.
    caution: u32,
    personal_burns: u32,
    witnessed_burns: u32,
    /// True once this moth's first caution came from witnessing,
    /// before any burn of its own.
    witness_first: bool,
    witnessed_refusals: u32,
    /// True once this moth's first caution came from a witnessed
    /// refusal, before any burn or burn-witness of its own.
    chain_first: bool,
    witnessed_placed: u32,
    /// True once this moth's first caution came from a placed
    /// (direction-carrying) witnessed refusal.
    placed_first: bool,
    witnessed_evidence: u32,
    /// True once this moth's first caution came from re-judging
    /// a transmitted evidence packet as irregular.
    evidence_first: bool,
    witnessed_throttled: u32,
    /// True once this moth's first caution came from re-judging
    /// a transmitted throttled packet as irregular.
    throttled_first: bool,
    witnessed_unshoved: u32,
    /// True once this moth's first caution came from re-judging
    /// a transmitted unshoved packet as irregular.
    unshoved_first: bool,
    witnessed_shoved: u32,
    /// True once this moth's first taught shove came from
    /// re-judging a transmitted shoved packet as irregular,
    /// before any burn of its own. (A shove is not a caution;
    /// the flag only records that the push arrived first.)
    shoved_first: bool,
    witnessed_fading: u32,
    /// True once this moth's first caution came from re-judging
    /// a transmitted fading packet as irregular.
    fading_first: bool,
    /// The taught pool: caution currently held that came from
    /// re-judging evidence packets (fading rooms only; always
    /// zero elsewhere). Earned caution is `caution` minus this.
    taught_caution: u32,
    witnessed_waning: u32,
    /// True once this moth's first caution came from re-judging
    /// a transmitted waning packet as irregular.
    waning_first: bool,
    /// Gain ticks (on the room's global, dawn-crossing clock)
    /// of the earned caution units this moth currently holds,
    /// oldest first (waning rooms only; always empty elsewhere).
    /// Invariant in a waning room: `caution` equals
    /// `earned_units.len() + taught_units.len()`.
    earned_units: VecDeque<u64>,
    /// Gain ticks of the taught caution units currently held,
    /// oldest first (waning rooms only).
    taught_units: VecDeque<u64>,
    /// True once this moth has transmitted a packet this night.
    transmitted_this_night: bool,
    /// Smoothed brightness perceived at its own cell (saturating).
    glow: f32,
    prev_glow: f32,
    /// Ticks since the last perceived brightness peak.
    since_peak: u64,
    peak_intervals: Vec<u64>,
    /// Some(ticks remaining) while standing still, listening.
    checking: Option<u64>,
    check_cooldown: u64,
    aversion: u64,
    settled: bool,
    dead: bool,
}

impl Moth {
    fn new(rng: &mut StdRng) -> Self {
        // Spawn in the middle dark, equidistant-ish from both lamps.
        let x = 28.0 + rng.gen_range(-6.0..6.0);
        let y = 14.0 + rng.gen_range(-6.0..6.0);
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            health: 100,
            caution: 0,
            personal_burns: 0,
            witnessed_burns: 0,
            witness_first: false,
            witnessed_refusals: 0,
            chain_first: false,
            witnessed_placed: 0,
            placed_first: false,
            witnessed_evidence: 0,
            evidence_first: false,
            witnessed_throttled: 0,
            throttled_first: false,
            witnessed_unshoved: 0,
            unshoved_first: false,
            witnessed_shoved: 0,
            shoved_first: false,
            witnessed_fading: 0,
            fading_first: false,
            taught_caution: 0,
            witnessed_waning: 0,
            waning_first: false,
            earned_units: VecDeque::new(),
            taught_units: VecDeque::new(),
            transmitted_this_night: false,
            glow: 0.0,
            prev_glow: 0.0,
            since_peak: 0,
            peak_intervals: Vec::new(),
            checking: None,
            check_cooldown: 0,
            aversion: 0,
            settled: false,
            dead: false,
        }
    }
}

struct Room {
    tuition: Tuition,
    grid: PhysicsGrid,
    moths: Vec<Moth>,
    tick: u64,
    /// The dawn-crossing clock: ticks since the room opened,
    /// never reset at a dawn. Caution units are aged on this
    /// clock in waning rooms (and it ticks, unread, elsewhere).
    global_tick: u64,
    nights_done: u32,
    next_decoy_in: u64,
    decoy_short_next: bool,
    rng: StdRng,
}

impl Room {
    fn new(seed: u64, tuition: Tuition) -> Self {
        let mut grid = PhysicsGrid::new(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            grid.set_material(x, 0, Material::Wall);
            grid.set_material(x, HEIGHT - 1, Material::Wall);
        }
        for y in 0..HEIGHT {
            grid.set_material(0, y, Material::Wall);
            grid.set_material(WIDTH - 1, y, Material::Wall);
        }
        let mut rng = StdRng::seed_from_u64(seed);
        let moths = (0..MOTHS).map(|_| Moth::new(&mut rng)).collect();
        Self {
            tuition,
            grid,
            moths,
            tick: 0,
            global_tick: 0,
            nights_done: 0,
            next_decoy_in: 3,
            decoy_short_next: true,
            rng,
        }
    }

    fn cell_of(m: &Moth) -> (usize, usize) {
        (
            (m.x.round() as i64).clamp(1, WIDTH as i64 - 2) as usize,
            (m.y.round() as i64).clamp(1, HEIGHT as i64 - 2) as usize,
        )
    }

    fn brightness_at(&self, x: usize, y: usize) -> f32 {
        self.grid.get(x, y).abs().min(SENSOR_SATURATION)
    }

    fn run_night(&mut self) -> NightLedger {
        let mut ledger = NightLedger::default();
        // Dawn decay (fading rooms only): caution carried across
        // the dawn is taxed by source before the new night
        // begins. The expiries are booked into the new night's
        // ledger -- the dawn is where the loss is discovered.
        if self.nights_done > 0 {
            match self.tuition {
                Tuition::FadingTaught => {
                    for m in &mut self.moths {
                        ledger.expired_taught += m.taught_caution as usize;
                        m.caution = m.caution.saturating_sub(m.taught_caution);
                        m.taught_caution = 0;
                    }
                }
                Tuition::FadingEarned => {
                    for m in &mut self.moths {
                        let earned = m.caution.saturating_sub(m.taught_caution);
                        ledger.expired_earned += earned as usize;
                        m.caution = m.taught_caution;
                    }
                }
                Tuition::FadingBoth => {
                    for m in &mut self.moths {
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
        // Fresh dark: clear the field, respawn the living unsettled
        // at the centre. Caution, health and scars carry over.
        self.grid.clear_waves();
        self.tick = 0;
        self.next_decoy_in = 3;
        self.decoy_short_next = true;
        for m in &mut self.moths {
            if !m.dead {
                m.x = 28.0 + self.rng.gen_range(-6.0..6.0);
                m.y = 14.0 + self.rng.gen_range(-6.0..6.0);
                m.vx = 0.0;
                m.vy = 0.0;
                m.settled = false;
                m.checking = None;
                m.aversion = 0;
                m.check_cooldown = 0;
                m.transmitted_this_night = false;
                m.peak_intervals.clear();
                m.glow = 0.0;
                m.prev_glow = 0.0;
            }
        }

        for _ in 0..NIGHT_TICKS {
            self.tick += 1;
            self.global_tick += 1;
            // Waning: caution expires by age, not by dawn. Each
            // unit's queue is oldest-first, so expiry pops from
            // the front until the front unit is still young.
            // The loss is booked into the night in which the
            // unit's age runs out -- a night, like a moth,
            // discovers its forgetting mid-flight.
            if let Some(lifetime) = self.tuition.waning_lifetime() {
                for m in &mut self.moths {
                    while let Some(&g) = m.taught_units.front() {
                        if !self.tuition.expires_taught() { break; }
                        if self.global_tick >= g + lifetime + self.tuition.taught_offset() {
                            m.taught_units.pop_front();
                            m.taught_caution = m.taught_caution.saturating_sub(1);
                            m.caution = m.caution.saturating_sub(1);
                            ledger.waning_expired_taught += 1;
                        } else {
                            break;
                        }
                    }
                    while let Some(&g) = m.earned_units.front() {
                        if !self.tuition.expires_earned() { break; }
                        if self.global_tick >= g + lifetime + self.tuition.earned_offset() {
                            m.earned_units.pop_front();
                            m.caution = m.caution.saturating_sub(1);
                            ledger.waning_expired_earned += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            // --- The lamps speak into the field. ---
            if self.tick % HONEST_PERIOD == 0 {
                self.grid.pluck(HONEST_POS.0, HONEST_POS.1, HONEST_STRENGTH);
            }
            if self.tick >= self.next_decoy_in {
                self.grid.pluck(DECOY_POS.0, DECOY_POS.1, DECOY_STRENGTH);
                let interval = if self.decoy_short_next { DECOY_SHORT } else { DECOY_LONG };
                self.decoy_short_next = !self.decoy_short_next;
                self.next_decoy_in = self.tick + interval;
            }
            self.grid.step();

            // --- The moths perceive, decide, move. ---
            let mut outcomes: Vec<(usize, bool)> = Vec::new(); // (moth idx, honest?)
            let mut refusal_events: Vec<(usize, f64, f64)> = Vec::new(); // (moth idx, gradient x, gradient y at refusal)
            let mut check_packets: Vec<(usize, Vec<u64>)> = Vec::new(); // (checker idx, peak intervals it heard) — evidence room only
            for i in 0..self.moths.len() {
                if self.moths[i].dead || self.moths[i].settled {
                    continue;
                }
                // Perceive before taking the mutable borrow: the
                // field is read whole, the moth is written.
                let (cx, cy) = Self::cell_of(&self.moths[i]);
                let raw = self.brightness_at(cx, cy);
                let gx = self.brightness_at((cx + 1).min(WIDTH - 2), cy)
                    - self.brightness_at(cx.saturating_sub(1).max(1), cy);
                let gy = self.brightness_at(cx, (cy + 1).min(HEIGHT - 2))
                    - self.brightness_at(cx, cy.saturating_sub(1).max(1));
                let m = &mut self.moths[i];
                m.prev_glow = m.glow;
                m.glow = m.glow * 0.80 + raw * 0.20;

                // Peak timing, for any moth that is listening.
                m.since_peak += 1;
                if m.checking.is_some()
                    && m.prev_glow <= PEAK_THRESH
                    && m.glow > PEAK_THRESH
                    && m.since_peak > 3
                {
                    m.peak_intervals.push(m.since_peak);
                    m.since_peak = 0;
                }

                if m.check_cooldown > 0 {
                    m.check_cooldown -= 1;
                }

                // Checking: stand still, listen, then judge.
                if let Some(left) = m.checking {
                    if left <= 1 || m.peak_intervals.len() >= 5 {
                        if self.tuition == Tuition::Evidence || self.tuition == Tuition::Unshoved || self.tuition == Tuition::Shoved || matches!(self.tuition, Tuition::FadingTaught | Tuition::FadingEarned | Tuition::FadingBoth | Tuition::Waning1500 | Tuition::Waning750 | Tuition::Waning375 | Tuition::Waning150 | Tuition::PooledTaught | Tuition::PooledEarned | Tuition::Staggered | Tuition::StaggeredBoth | Tuition::Mirrored | Tuition::StaggeredWide | Tuition::MirroredWide | Tuition::CoincidentWide | Tuition::StaggeredStep | Tuition::MirroredStep | Tuition::CoincidentStep | Tuition::StaggeredEdge | Tuition::MirroredEdge | Tuition::CoincidentEdge | Tuition::StaggeredFace | Tuition::MirroredFace | Tuition::CoincidentFace | Tuition::Staggered475 | Tuition::Mirrored475 | Tuition::Coincident475 | Tuition::Staggered500 | Tuition::Mirrored500 | Tuition::Coincident500) {
                            check_packets.push((i, m.peak_intervals.clone()));
                        } else if self.tuition == Tuition::Throttled {
                            if !m.transmitted_this_night {
                                check_packets.push((i, m.peak_intervals.clone()));
                                m.transmitted_this_night = true;
                                ledger.throttled_packets_sent += 1;
                            } else {
                                ledger.throttled_suppressed += 1;
                            }
                        }
                        // Judge the flicker.
                        if m.peak_intervals.len() >= 3 {
                            let iv = &m.peak_intervals;
                            let mean = iv.iter().sum::<u64>() as f64 / iv.len() as f64;
                            let var = iv
                                .iter()
                                .map(|&v| (v as f64 - mean).powi(2))
                                .sum::<f64>()
                                / iv.len() as f64;
                            let cv = if mean > 0.0 { var.sqrt() / mean } else { 0.0 };
                            if cv > FLICKER_CV {
                                ledger.refusals += 1;
                                refusal_events.push((i, gx as f64, gy as f64));
                                m.aversion = AVERSION_TICKS;
                                // Flee down-gradient: reverse heading.
                                m.vx = -m.vx - 0.3;
                                m.vy = -m.vy;
                            } else {
                                ledger.steady_verdicts += 1;
                            }
                        } else {
                            ledger.steady_verdicts += 1;
                        }
                        m.checking = None;
                        m.check_cooldown = CHECK_COOLDOWN;
                        m.peak_intervals.clear();
                    } else {
                        m.checking = Some(left - 1);
                        continue; // standing still is the cost
                    }
                }

                // A cautious moth that hears a loud claim stops to check it.
                if m.caution > 0
                    && m.checking.is_none()
                    && m.check_cooldown == 0
                    && m.aversion == 0
                    && m.glow > CHECK_TRIGGER
                {
                    m.checking = Some(CHECK_TICKS);
                    m.peak_intervals.clear();
                    m.since_peak = 0;
                    ledger.checks += 1;
                    continue;
                }

                // Flight: steer up the local brightness gradient
                // (sampled above, before the borrow), with jitter —
                // moth flight, not missile guidance.
                let pull = if m.aversion > 0 { -1.0 } else { 1.0 };
                if m.aversion > 0 {
                    m.aversion -= 1;
                }
                let jx = self.rng.gen_range(-0.05..0.05);
                let jy = self.rng.gen_range(-0.05..0.05);
                m.vx = (m.vx + pull * gx as f64 * 1.4 + jx) * 0.88;
                m.vy = (m.vy + pull * gy as f64 * 1.4 + jy) * 0.88;
                let speed = (m.vx * m.vx + m.vy * m.vy).sqrt();
                let max_speed = 0.55;
                if speed > max_speed {
                    m.vx *= max_speed / speed;
                    m.vy *= max_speed / speed;
                }
                m.x = (m.x + m.vx).clamp(1.5, WIDTH as f64 - 2.5);
                m.y = (m.y + m.vy).clamp(1.5, HEIGHT as f64 - 2.5);

                // Arrival: the keeper, not the moth, applies it.
                let dh = ((m.x - HONEST_POS.0 as f64).powi(2)
                    + (m.y - HONEST_POS.1 as f64).powi(2))
                .sqrt();
                let dd = ((m.x - DECOY_POS.0 as f64).powi(2)
                    + (m.y - DECOY_POS.1 as f64).powi(2))
                .sqrt();
                if dh <= ARRIVE_RADIUS {
                    outcomes.push((i, true));
                } else if dd <= ARRIVE_RADIUS {
                    outcomes.push((i, false));
                }
            }

            let waning = self.tuition.is_waning();
            let gtick = self.global_tick;
            for (i, honest) in outcomes {
                let m = &mut self.moths[i];
                if honest {
                    ledger.fed += 1;
                    m.health = (m.health + FEED_RESTORE).min(100);
                    m.settled = true;
                } else {
                    ledger.decoy_arrivals += 1;
                    ledger.burns += 1;
                    // The burned moth's own account is settled in
                    // this scope; the borrow ends before witnesses
                    // are taught (they are other moths entirely).
                    {
                        let m = &mut self.moths[i];
                        m.health -= BURN_DAMAGE;
                        m.caution += 1;
                        if waning {
                            m.earned_units.push_back(gtick);
                        }
                        m.personal_burns += 1;
                        if m.health <= 0 {
                            m.dead = true;
                            ledger.deaths += 1;
                        } else {
                            // Knocked back into the dark, wiser.
                            m.x = 28.0 + self.rng.gen_range(-5.0..5.0);
                            m.y = 14.0 + self.rng.gen_range(-5.0..5.0);
                            m.vx = 0.0;
                            m.vy = 0.0;
                            m.check_cooldown = 0;
                        }
                    }
                    if self.tuition != Tuition::Solo {
                        // A burn is a local event. Collect the
                        // witnesses first (distance from the burn
                        // site, i.e. the decoy), then teach them:
                        // caution only, no lamp identity, no map.
                        let witnesses: Vec<usize> = self
                            .moths
                            .iter()
                            .enumerate()
                            .filter(|(j, w)| {
                                *j != i && !w.dead && !w.settled && {
                                    let d = ((w.x - DECOY_POS.0 as f64).powi(2)
                                        + (w.y - DECOY_POS.1 as f64).powi(2))
                                    .sqrt();
                                    d <= WITNESS_RADIUS
                                }
                            })
                            .map(|(j, _)| j)
                            .collect();
                        for j in witnesses {
                            let w = &mut self.moths[j];
                            ledger.witness_events += 1;
                            if w.personal_burns == 0 && !w.witness_first && !w.chain_first {
                                w.witness_first = true;
                                ledger.new_witness_learners += 1;
                            }
                            w.witnessed_burns += 1;
                            w.caution += 1;
                            if waning {
                                w.earned_units.push_back(gtick);
                            }
                            // A witness also recoils: it saw
                            // *where* the burn happened, close
                            // by, and flees down-gradient for a
                            // while — the same aversion a refused
                            // claim earns, paid in distance.
                            w.aversion = AVERSION_TICKS;
                        }
                    }
                }
            }
            // Chain link: a refusal is a local event in Chain and
            // Placed rooms. In Placed rooms the witness also
            // inherits the refusal's direction: the refuser's
            // gradient at refusal points toward the judged lamp,
            // so the witness recoils along its negative.
            if self.tuition == Tuition::Chain || self.tuition == Tuition::Placed {
                for &(ri, rgx, rgy) in &refusal_events {
                    let (rx, ry) = (self.moths[ri].x, self.moths[ri].y);
                    let glen = (rgx * rgx + rgy * rgy).sqrt();
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != ri && !w.dead && !w.settled && {
                            let d = ((w.x - rx).powi(2) + (w.y - ry).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        if self.tuition == Tuition::Placed {
                            ledger.placed_events += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.placed_first && !w.witness_first && !w.chain_first {
                                w.placed_first = true;
                                ledger.new_placed_learners += 1;
                            }
                            w.witnessed_placed += 1;
                            w.caution += 1;
                            w.aversion = AVERSION_TICKS;
                            // Direction away from the refused lamp:
                            // negative gradient at the refuser; if
                            // that gradient is ~zero, fall back to
                            // away from the refuser's position.
                            let dir = if glen > 1e-6 {
                                (-rgx / glen, -rgy / glen)
                            } else {
                                let (dx, dy) = (w.x - rx, w.y - ry);
                                let l = (dx * dx + dy * dy).sqrt();
                                if l > 1e-6 { (dx / l, dy / l) } else { (0.0, 0.0) }
                            };
                            // The direction is delivered as one
                            // shove at the teaching moment; the
                            // ordinary scalar aversion (down the
                            // witness's own gradient) does the
                            // rest, exactly as in the chain room.
                            // A sustained directed recoil was tried
                            // first and struck — see the README.
                            w.vx += dir.0 * 0.6;
                            w.vy += dir.1 * 0.6;
                        } else {
                            ledger.chain_events += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.chain_first && !w.witness_first {
                                w.chain_first = true;
                                ledger.new_chain_learners += 1;
                            }
                            w.witnessed_refusals += 1;
                            w.caution += 1;
                            w.aversion = AVERSION_TICKS;
                        }
                    }
                }
            }
            // Evidence link: a completed check is a local event
            // in the Evidence room. Witnesses receive the actual
            // peak intervals and re-judge them: irregular earns
            // caution; steady can acquit hearsay caution (a moth
            // with no personal burn whose caution > 0 loses one).
            // The checker's own verdict is never told to them.
            if self.tuition == Tuition::Evidence {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.evidence_events += 1;
                        w.witnessed_evidence += 1;
                        if irregular {
                            ledger.evidence_irregular += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.evidence_first && !w.witness_first && !w.chain_first && !w.placed_first {
                                w.evidence_first = true;
                                ledger.new_evidence_learners += 1;
                            }
                            w.caution += 1;
                            w.aversion = AVERSION_TICKS;
                        } else {
                            ledger.evidence_steady += 1;
                            if w.personal_burns == 0 && w.caution > 0 {
                                w.caution -= 1;
                                ledger.evidence_acquittals += 1;
                            }
                        }
                    }
                }
            }
            // Throttled link: same packets, same re-judgement, but
            // only the first check per checker per night was ever
            // sent (gated above). Witnesses re-judge exactly as in
            // the evidence room; the ledger counts are separate so
            // the two rooms can be compared packet for packet.
            if self.tuition == Tuition::Throttled {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.throttled_events += 1;
                        w.witnessed_throttled += 1;
                        if irregular {
                            ledger.throttled_irregular += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.throttled_first && !w.witness_first && !w.chain_first && !w.placed_first && !w.evidence_first {
                                w.throttled_first = true;
                                ledger.new_throttled_learners += 1;
                            }
                            w.caution += 1;
                            w.aversion = AVERSION_TICKS;
                        } else {
                            ledger.throttled_steady += 1;
                            if w.personal_burns == 0 && w.caution > 0 {
                                w.caution -= 1;
                                ledger.throttled_acquittals += 1;
                            }
                        }
                    }
                }
            }
            // Unshoved link: the evidence room's packets and
            // re-judgement, verbatim, with exactly one change --
            // an irregular verdict earns the witness caution and
            // NO aversion. The lesson arrives as information.
            // A steady verdict acquits hearsay caution exactly as
            // in the evidence room (an acquittal never shoved).
            if self.tuition == Tuition::Unshoved {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.unshoved_events += 1;
                        w.witnessed_unshoved += 1;
                        if irregular {
                            ledger.unshoved_irregular += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.unshoved_first && !w.witness_first && !w.chain_first && !w.placed_first && !w.evidence_first && !w.throttled_first {
                                w.unshoved_first = true;
                                ledger.new_unshoved_learners += 1;
                            }
                            w.caution += 1;
                            // NO aversion: that is the experiment.
                        } else {
                            ledger.unshoved_steady += 1;
                            if w.personal_burns == 0 && w.caution > 0 {
                                w.caution -= 1;
                                ledger.unshoved_acquittals += 1;
                            }
                        }
                    }
                }
            }
            // Shoved link: the evidence room's packets and
            // re-judgement, verbatim, with exactly the opposite
            // change from the unshoved room -- an irregular
            // verdict earns the witness the 150-tick aversion
            // and NO caution. The push travels; the lesson does
            // not: no witness is recruited as a checker by a
            // packet, and a steady verdict earns nothing at all
            // (there is no hearsay caution in this room for a
            // steady packet to acquit, so the acquittal count
            // stays at zero by construction and is reported,
            // not asserted, outside the tests).
            if self.tuition == Tuition::Shoved {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.shoved_events += 1;
                        w.witnessed_shoved += 1;
                        if irregular {
                            ledger.shoved_irregular += 1;
                            if w.personal_burns == 0 && !w.shoved_first {
                                w.shoved_first = true;
                                ledger.new_shoved_first += 1;
                            }
                            // The shove, and only the shove:
                            // NO caution is earned here.
                            w.aversion = AVERSION_TICKS;
                        } else {
                            ledger.shoved_steady += 1;
                            // No acquittal: nothing was taught,
                            // so there is nothing to un-teach.
                        }
                    }
                }
            }

            // Fading link: the evidence room's packets and
            // re-judgement, verbatim, with exactly one change --
            // taught caution is booked in its own pool
            // (taught_caution) so the dawn can tax each source
            // separately. An irregular verdict earns caution AND
            // taught_caution, plus the evidence room's aversion;
            // a steady verdict acquits hearsay caution, spending
            // the taught pool first.
            if matches!(self.tuition, Tuition::FadingTaught | Tuition::FadingEarned | Tuition::FadingBoth) {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.fading_events += 1;
                        w.witnessed_fading += 1;
                        if irregular {
                            ledger.fading_irregular += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.fading_first && !w.witness_first && !w.chain_first && !w.placed_first && !w.evidence_first {
                                w.fading_first = true;
                                ledger.new_fading_learners += 1;
                            }
                            w.caution += 1;
                            w.taught_caution += 1;
                            w.aversion = AVERSION_TICKS;
                        } else {
                            ledger.fading_steady += 1;
                            if w.personal_burns == 0 && w.caution > 0 {
                                w.caution -= 1;
                                if w.taught_caution > 0 {
                                    w.taught_caution -= 1;
                                }
                                ledger.fading_acquittals += 1;
                            }
                        }
                    }
                }
            }

            // Waning link: the evidence room's packets and
            // re-judgement, verbatim, with exactly one change --
            // taught caution is booked in its own pool AND every
            // unit, taught or earned, carries its gain tick on
            // the room's global clock, to be expired by age at
            // the top of some future tick (see run_night). An
            // acquittal spends the taught pool first, exactly
            // as in the fading rooms, and pops that unit's tick
            // with it; if no taught unit remains, the oldest
            // earned unit is spent.
            if self.tuition.is_waning() {
                for (ci, intervals) in &check_packets {
                    let (cx2, cy2) = (self.moths[*ci].x, self.moths[*ci].y);
                    let cv = cv_of(intervals);
                    let irregular = intervals.len() >= 3 && cv > FLICKER_CV;
                    let witnesses: Vec<usize> = self.moths.iter().enumerate()
                        .filter(|(j, w)| *j != *ci && !w.dead && !w.settled && {
                            let d = ((w.x - cx2).powi(2) + (w.y - cy2).powi(2)).sqrt();
                            d <= CHAIN_RADIUS
                        }).map(|(j, _)| j).collect();
                    for j in witnesses {
                        let w = &mut self.moths[j];
                        ledger.waning_events += 1;
                        w.witnessed_waning += 1;
                        if irregular {
                            ledger.waning_irregular += 1;
                            if w.personal_burns == 0 && w.witnessed_burns == 0 && !w.waning_first && !w.witness_first && !w.chain_first && !w.placed_first && !w.evidence_first {
                                w.waning_first = true;
                                ledger.new_waning_learners += 1;
                            }
                            w.caution += 1;
                            w.taught_caution += 1;
                            w.taught_units.push_back(self.global_tick);
                            w.aversion = AVERSION_TICKS;
                        } else {
                            ledger.waning_steady += 1;
                            if w.personal_burns == 0 && w.caution > 0 {
                                w.caution -= 1;
                                if w.taught_caution > 0 {
                                    w.taught_caution -= 1;
                                    w.taught_units.pop_front();
                                } else {
                                    w.earned_units.pop_front();
                                }
                                ledger.waning_acquittals += 1;
                            }
                        }
                    }
                }
            }
        }
        self.nights_done += 1;
        ledger.alive_end = self.moths.iter().filter(|m| !m.dead).count();
        ledger
    }
}

fn run_all(seed: u64, tuition: Tuition) -> Vec<NightLedger> {
    let mut room = Room::new(seed, tuition);
    (0..NIGHTS).map(|_| room.run_night()).collect()
}

const SEED: u64 = 0x4D4F_5448;

fn print_regime(name: &str, nights: &[NightLedger]) {
    println!("  regime '{name}':");
    for (n, l) in nights.iter().enumerate() {
        println!(
            "    night {}: fed {}, burns {}, deaths {}, checks {}, refusals {}, steady verdicts {}, witness events {}, new witness learners {}, chain events {}, new chain learners {}, placed events {}, new placed learners {}, evidence events {}, new evidence learners {}, evidence acquittals {}, evidence irregular {}, evidence steady {}, throttled events {}, new throttled learners {}, throttled acquittals {}, throttled irregular {}, throttled steady {}, throttled packets sent {}, throttled suppressed {}, unshoved events {}, new unshoved learners {}, unshoved acquittals {}, unshoved irregular {}, unshoved steady {}, shoved events {}, new shoved-first {}, shoved acquittals {}, shoved irregular {}, shoved steady {}, fading events {}, new fading learners {}, fading acquittals {}, fading irregular {}, fading steady {}, expired taught {}, expired earned {}, waning events {}, new waning learners {}, waning acquittals {}, waning irregular {}, waning steady {}, waning expired taught {}, waning expired earned {}, alive at dawn {}",
            n + 1, l.fed, l.burns, l.deaths, l.checks, l.refusals,
            l.steady_verdicts, l.witness_events, l.new_witness_learners, l.chain_events, l.new_chain_learners, l.placed_events, l.new_placed_learners, l.evidence_events, l.new_evidence_learners, l.evidence_acquittals, l.evidence_irregular, l.evidence_steady, l.throttled_events, l.new_throttled_learners, l.throttled_acquittals, l.throttled_irregular, l.throttled_steady, l.throttled_packets_sent, l.throttled_suppressed, l.unshoved_events, l.new_unshoved_learners, l.unshoved_acquittals, l.unshoved_irregular, l.unshoved_steady, l.shoved_events, l.new_shoved_first, l.shoved_acquittals, l.shoved_irregular, l.shoved_steady, l.fading_events, l.new_fading_learners, l.fading_acquittals, l.fading_irregular, l.fading_steady, l.expired_taught, l.expired_earned, l.waning_events, l.new_waning_learners, l.waning_acquittals, l.waning_irregular, l.waning_steady, l.waning_expired_taught, l.waning_expired_earned, l.alive_end
        );
    }
    let burns: usize = nights.iter().map(|l| l.burns).sum();
    let fed: usize = nights.iter().map(|l| l.fed).sum();
    let refusals: usize = nights.iter().map(|l| l.refusals).sum();
    println!("    totals: fed {fed}, burns {burns}, refusals {refusals}");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: this experiment
    // has no TUI, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let solo = run_all(SEED, Tuition::Solo);
    let witness = run_all(SEED, Tuition::Witness);
    let chain = run_all(SEED, Tuition::Chain);
    let placed = run_all(SEED, Tuition::Placed);
    let evidence = run_all(SEED, Tuition::Evidence);
    let throttled = run_all(SEED, Tuition::Throttled);
    let unshoved = run_all(SEED, Tuition::Unshoved);
    let shoved = run_all(SEED, Tuition::Shoved);
    let fading_taught = run_all(SEED, Tuition::FadingTaught);
    let fading_earned = run_all(SEED, Tuition::FadingEarned);
    let fading_both = run_all(SEED, Tuition::FadingBoth);
    let waning_1500 = run_all(SEED, Tuition::Waning1500);
    let waning_750 = run_all(SEED, Tuition::Waning750);
    let waning_375 = run_all(SEED, Tuition::Waning375);
    let waning_150 = run_all(SEED, Tuition::Waning150);
    let pooled_taught = run_all(SEED, Tuition::PooledTaught);
    let pooled_earned = run_all(SEED, Tuition::PooledEarned);
    let staggered = run_all(SEED, Tuition::Staggered);
    let staggered_both = run_all(SEED, Tuition::StaggeredBoth);
    let mirrored = run_all(SEED, Tuition::Mirrored);
    let staggered_wide = run_all(SEED, Tuition::StaggeredWide);
    let mirrored_wide = run_all(SEED, Tuition::MirroredWide);
    let coincident_wide = run_all(SEED, Tuition::CoincidentWide);
    let staggered_step = run_all(SEED, Tuition::StaggeredStep);
    let mirrored_step = run_all(SEED, Tuition::MirroredStep);
    let coincident_step = run_all(SEED, Tuition::CoincidentStep);
    let staggered_edge = run_all(SEED, Tuition::StaggeredEdge);
    let mirrored_edge = run_all(SEED, Tuition::MirroredEdge);
    let coincident_edge = run_all(SEED, Tuition::CoincidentEdge);
    let staggered_face = run_all(SEED, Tuition::StaggeredFace);
    let mirrored_face = run_all(SEED, Tuition::MirroredFace);
    let coincident_face = run_all(SEED, Tuition::CoincidentFace);
    let staggered_475 = run_all(SEED, Tuition::Staggered475);
    let mirrored_475 = run_all(SEED, Tuition::Mirrored475);
    let coincident_475 = run_all(SEED, Tuition::Coincident475);
    let staggered_500 = run_all(SEED, Tuition::Staggered500);
    let mirrored_500 = run_all(SEED, Tuition::Mirrored500);
    let coincident_500 = run_all(SEED, Tuition::Coincident500);
    println!("facegrain-moths headless: one cave, thirty-eight tuition systems");
    println!("  honest lamp: period {HONEST_PERIOD} ticks, strength {HONEST_STRENGTH} \u{2014} steady");
    println!("  decoy lamp: intervals {DECOY_SHORT}/{DECOY_LONG} ticks (same mean), strength {DECOY_STRENGTH} \u{2014} brighter, irregular");
    println!("  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; witness radius {WITNESS_RADIUS} cells; chain radius {CHAIN_RADIUS} cells");
    print_regime("solo (caution only from your own burns)", &solo);
    print_regime("witness (caution also from a neighbour's burn, within radius)", &witness);
    print_regime("chain (caution also from a neighbour's refusal, within radius)", &chain);
    print_regime("placed (chain, but the refusal carries its lamp direction)", &placed);
    print_regime("evidence (every check transmits its peak intervals; witnesses re-judge)", &evidence);
    print_regime("throttled (only each checker's first check per night transmits; witnesses re-judge)", &throttled);
    print_regime("unshoved (every check transmits its peak intervals; witnesses re-judge; irregular teaches caution with NO aversion)", &unshoved);
    print_regime("shoved (every check transmits its peak intervals; witnesses re-judge; irregular teaches the aversion with NO caution)", &shoved);
    print_regime("fading-taught (evidence teaching; taught caution expires at each dawn, earned persists)", &fading_taught);
    print_regime("fading-earned (evidence teaching; earned caution expires at each dawn, taught persists)", &fading_earned);
    print_regime("fading-both (evidence teaching; both caution pools expire at each dawn)", &fading_both);
    print_regime("waning-1500 (evidence teaching; every caution unit expires 1500 ticks after it is gained; no dawn tax)", &waning_1500);
    print_regime("waning-750 (evidence teaching; unit lifetime 750 ticks; no dawn tax)", &waning_750);
    print_regime("waning-375 (evidence teaching; unit lifetime 375 ticks; no dawn tax)", &waning_375);
    print_regime("waning-150 (evidence teaching; unit lifetime 150 ticks; no dawn tax)", &waning_150);
    print_regime("pooled-taught (evidence teaching; TAUGHT units expire 750 ticks after gain; earned units never expire; no dawn tax)", &pooled_taught);
    print_regime("pooled-earned (evidence teaching; EARNED units expire 750 ticks after gain; taught units never expire; no dawn tax)", &pooled_earned);
    print_regime("staggered (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +375 ticks; no dawn tax)", &staggered);
    print_regime("staggered-both (evidence teaching; both pools lifetime 750; BOTH deadlines offset +375 ticks, coincident control; no dawn tax)", &staggered_both);
    print_regime("mirrored (evidence teaching; both pools lifetime 750; EARNED deadline offset +375 ticks, the mirror stagger; no dawn tax)", &mirrored);
    print_regime("staggered-wide (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +750 ticks, wider than a journey leg; no dawn tax)", &staggered_wide);
    print_regime("mirrored-wide (evidence teaching; both pools lifetime 750; EARNED deadline offset +750 ticks, the wide mirror stagger; no dawn tax)", &mirrored_wide);
    print_regime("coincident-wide (evidence teaching; both pools lifetime 750; BOTH deadlines offset +750 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_wide);
    print_regime("staggered-step (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +450 ticks, exactly the claim-step; no dawn tax)", &staggered_step);
    print_regime("mirrored-step (evidence teaching; both pools lifetime 750; EARNED deadline offset +450 ticks, the mirror at the claim-step; no dawn tax)", &mirrored_step);
    print_regime("coincident-step (evidence teaching; both pools lifetime 750; BOTH deadlines offset +450 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_step);
    print_regime("staggered-edge (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +600 ticks, midway between the claim-step and the wide offset; no dawn tax)", &staggered_edge);
    print_regime("mirrored-edge (evidence teaching; both pools lifetime 750; EARNED deadline offset +600 ticks, the mirror at the shelf's edge; no dawn tax)", &mirrored_edge);
    print_regime("coincident-edge (evidence teaching; both pools lifetime 750; BOTH deadlines offset +600 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_edge);
    print_regime("staggered-face (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +525 ticks, midway between the claim-step and the edge; no dawn tax)", &staggered_face);
    print_regime("mirrored-face (evidence teaching; both pools lifetime 750; EARNED deadline offset +525 ticks, the mirror at the cliff's face; no dawn tax)", &mirrored_face);
    print_regime("coincident-face (evidence teaching; both pools lifetime 750; BOTH deadlines offset +525 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_face);
    print_regime("staggered-475 (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +475 ticks, 25 ticks past the claim-step inside the cliff interval; no dawn tax)", &staggered_475);
    print_regime("mirrored-475 (evidence teaching; both pools lifetime 750; EARNED deadline offset +475 ticks, the mirror at the finer grain; no dawn tax)", &mirrored_475);
    print_regime("coincident-475 (evidence teaching; both pools lifetime 750; BOTH deadlines offset +475 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_475);
    print_regime("staggered-500 (evidence teaching; both pools lifetime 750; TAUGHT deadline offset +500 ticks, 50 ticks past the claim-step inside the cliff interval; no dawn tax)", &staggered_500);
    print_regime("mirrored-500 (evidence teaching; both pools lifetime 750; EARNED deadline offset +500 ticks, the mirror at the finer grain; no dawn tax)", &mirrored_500);
    print_regime("coincident-500 (evidence teaching; both pools lifetime 750; BOTH deadlines offset +500 ticks, coincident control at the longest deadline; no dawn tax)", &coincident_500);
    let sb: usize = solo.iter().map(|l| l.burns).sum();
    let wb: usize = witness.iter().map(|l| l.burns).sum();
    let cb: usize = chain.iter().map(|l| l.burns).sum();
    let we: usize = witness.iter().map(|l| l.witness_events).sum();
    let wl: usize = witness.iter().map(|l| l.new_witness_learners).sum();
    let ce: usize = chain.iter().map(|l| l.chain_events).sum();
    let cl: usize = chain.iter().map(|l| l.new_chain_learners).sum();
    let cf: usize = chain.iter().map(|l| l.fed).sum();
    let pb: usize = placed.iter().map(|l| l.burns).sum();
    let pf: usize = placed.iter().map(|l| l.fed).sum();
    let pe: usize = placed.iter().map(|l| l.placed_events).sum();
    let pl: usize = placed.iter().map(|l| l.new_placed_learners).sum();
    let pc: usize = placed.iter().map(|l| l.checks).sum();
    let cc: usize = chain.iter().map(|l| l.checks).sum();
    let eb: usize = evidence.iter().map(|l| l.burns).sum();
    let ef: usize = evidence.iter().map(|l| l.fed).sum();
    let ee: usize = evidence.iter().map(|l| l.evidence_events).sum();
    let el: usize = evidence.iter().map(|l| l.new_evidence_learners).sum();
    let ea: usize = evidence.iter().map(|l| l.evidence_acquittals).sum();
    let ei: usize = evidence.iter().map(|l| l.evidence_irregular).sum();
    let es: usize = evidence.iter().map(|l| l.evidence_steady).sum();
    let ec: usize = evidence.iter().map(|l| l.checks).sum();
    let tb: usize = throttled.iter().map(|l| l.burns).sum();
    let tf: usize = throttled.iter().map(|l| l.fed).sum();
    let te: usize = throttled.iter().map(|l| l.throttled_events).sum();
    let tl: usize = throttled.iter().map(|l| l.new_throttled_learners).sum();
    let ta: usize = throttled.iter().map(|l| l.throttled_acquittals).sum();
    let ti: usize = throttled.iter().map(|l| l.throttled_irregular).sum();
    let ts: usize = throttled.iter().map(|l| l.throttled_steady).sum();
    let tc: usize = throttled.iter().map(|l| l.checks).sum();
    let tps: usize = throttled.iter().map(|l| l.throttled_packets_sent).sum();
    let tsup: usize = throttled.iter().map(|l| l.throttled_suppressed).sum();
    let ub: usize = unshoved.iter().map(|l| l.burns).sum();
    let uf: usize = unshoved.iter().map(|l| l.fed).sum();
    let ue: usize = unshoved.iter().map(|l| l.unshoved_events).sum();
    let ul: usize = unshoved.iter().map(|l| l.new_unshoved_learners).sum();
    let ua: usize = unshoved.iter().map(|l| l.unshoved_acquittals).sum();
    let ui: usize = unshoved.iter().map(|l| l.unshoved_irregular).sum();
    let us: usize = unshoved.iter().map(|l| l.unshoved_steady).sum();
    let uc: usize = unshoved.iter().map(|l| l.checks).sum();
    let hb: usize = shoved.iter().map(|l| l.burns).sum();
    let hf: usize = shoved.iter().map(|l| l.fed).sum();
    let he: usize = shoved.iter().map(|l| l.shoved_events).sum();
    let hl: usize = shoved.iter().map(|l| l.new_shoved_first).sum();
    let ha: usize = shoved.iter().map(|l| l.shoved_acquittals).sum();
    let hi: usize = shoved.iter().map(|l| l.shoved_irregular).sum();
    let hs: usize = shoved.iter().map(|l| l.shoved_steady).sum();
    let hc: usize = shoved.iter().map(|l| l.checks).sum();
    println!("  comparison: total burns solo {sb}, witness {wb}, chain {cb}, placed {pb}, evidence {eb}, throttled {tb}, unshoved {ub}, shoved {hb}; witness events (witness room) {we}; moths whose first caution was witnessed {wl}; chain events {ce}; moths whose first caution was a witnessed refusal {cl}; chain fed {cf}; placed events {pe}; moths whose first caution was a placed refusal {pl}; placed fed {pf}; checks chain {cc}, placed {pc}; evidence events {ee}, evidence-first learners {el}, evidence acquittals {ea}, evidence irregular {ei}, evidence steady {es}, evidence fed {ef}, evidence checks {ec}; throttled events {te}, throttled-first learners {tl}, throttled acquittals {ta}, throttled irregular {ti}, throttled steady {ts}, throttled fed {tf}, throttled checks {tc}, throttled packets sent {tps}, throttled suppressed {tsup}; unshoved events {ue}, unshoved-first learners {ul}, unshoved acquittals {ua}, unshoved irregular {ui}, unshoved steady {us}, unshoved fed {uf}, unshoved burns {ub}, unshoved checks {uc}; shoved events {he}, shoved-first {hl}, shoved acquittals {ha}, shoved irregular {hi}, shoved steady {hs}, shoved fed {hf}, shoved burns {hb}, shoved checks {hc}");
    let ftb: usize = fading_taught.iter().map(|l| l.burns).sum();
    let ftf: usize = fading_taught.iter().map(|l| l.fed).sum();
    let ftc: usize = fading_taught.iter().map(|l| l.checks).sum();
    let fte: usize = fading_taught.iter().map(|l| l.fading_events).sum();
    let ftl: usize = fading_taught.iter().map(|l| l.new_fading_learners).sum();
    let fta: usize = fading_taught.iter().map(|l| l.fading_acquittals).sum();
    let fxt: usize = fading_taught.iter().map(|l| l.expired_taught).sum();
    let fxe_t: usize = fading_taught.iter().map(|l| l.expired_earned).sum();
    let feb: usize = fading_earned.iter().map(|l| l.burns).sum();
    let fef: usize = fading_earned.iter().map(|l| l.fed).sum();
    let fec: usize = fading_earned.iter().map(|l| l.checks).sum();
    let fee: usize = fading_earned.iter().map(|l| l.fading_events).sum();
    let fel: usize = fading_earned.iter().map(|l| l.new_fading_learners).sum();
    let fea: usize = fading_earned.iter().map(|l| l.fading_acquittals).sum();
    let fxt_e: usize = fading_earned.iter().map(|l| l.expired_taught).sum();
    let fxe: usize = fading_earned.iter().map(|l| l.expired_earned).sum();
    let fbb: usize = fading_both.iter().map(|l| l.burns).sum();
    let fbf: usize = fading_both.iter().map(|l| l.fed).sum();
    let fbc: usize = fading_both.iter().map(|l| l.checks).sum();
    let fbe: usize = fading_both.iter().map(|l| l.fading_events).sum();
    let fbl: usize = fading_both.iter().map(|l| l.new_fading_learners).sum();
    let fba: usize = fading_both.iter().map(|l| l.fading_acquittals).sum();
    let fxt_b: usize = fading_both.iter().map(|l| l.expired_taught).sum();
    let fxe_b: usize = fading_both.iter().map(|l| l.expired_earned).sum();
    println!("  fading comparison: fading-taught burns {ftb}, fed {ftf}, checks {ftc}, fading events {fte}, fading-first learners {ftl}, acquittals {fta}, expired taught {fxt}, expired earned {fxe_t}; fading-earned burns {feb}, fed {fef}, checks {fec}, fading events {fee}, fading-first learners {fel}, acquittals {fea}, expired taught {fxt_e}, expired earned {fxe}; fading-both burns {fbb}, fed {fbf}, checks {fbc}, fading events {fbe}, fading-first learners {fbl}, acquittals {fba}, expired taught {fxt_b}, expired earned {fxe_b}; evidence burns {eb}, evidence checks {ec}");
    for (label, nights) in [("waning-1500", &waning_1500), ("waning-750", &waning_750), ("waning-375", &waning_375), ("waning-150", &waning_150), ("pooled-taught", &pooled_taught), ("pooled-earned", &pooled_earned), ("staggered", &staggered), ("staggered-both", &staggered_both), ("mirrored", &mirrored), ("staggered-wide", &staggered_wide), ("mirrored-wide", &mirrored_wide), ("coincident-wide", &coincident_wide), ("staggered-step", &staggered_step), ("mirrored-step", &mirrored_step), ("coincident-step", &coincident_step), ("staggered-edge", &staggered_edge), ("mirrored-edge", &mirrored_edge), ("coincident-edge", &coincident_edge), ("staggered-face", &staggered_face), ("mirrored-face", &mirrored_face), ("coincident-face", &coincident_face), ("staggered-475", &staggered_475), ("mirrored-475", &mirrored_475), ("coincident-475", &coincident_475), ("staggered-500", &staggered_500), ("mirrored-500", &mirrored_500), ("coincident-500", &coincident_500)] {
        let b: usize = nights.iter().map(|l| l.burns).sum();
        let f: usize = nights.iter().map(|l| l.fed).sum();
        let c: usize = nights.iter().map(|l| l.checks).sum();
        let ev: usize = nights.iter().map(|l| l.waning_events).sum();
        let ln: usize = nights.iter().map(|l| l.new_waning_learners).sum();
        let aq: usize = nights.iter().map(|l| l.waning_acquittals).sum();
        let ir: usize = nights.iter().map(|l| l.waning_irregular).sum();
        let st: usize = nights.iter().map(|l| l.waning_steady).sum();
        let xt: usize = nights.iter().map(|l| l.waning_expired_taught).sum();
        let xe: usize = nights.iter().map(|l| l.waning_expired_earned).sum();
        println!("  waning comparison: {label} burns {b}, fed {f}, checks {c}, waning events {ev}, waning-first learners {ln}, acquittals {aq}, irregular {ir}, steady {st}, expired taught {xt}, expired earned {xe}");
    }
    if !headless {
        println!("  (count it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_decoy_still_fools_the_naive() {
        let solo = run_all(SEED, Tuition::Solo);
        assert!(solo[0].burns > 0, "decoy fooled no one night 1 solo: {solo:?}");
    }

    #[test]
    fn solo_and_witness_controls_reproduce() {
        let s = run_all(SEED, Tuition::Solo);
        assert_eq!(s.iter().map(|l| l.burns).sum::<usize>(), 12);
        assert_eq!(s.iter().map(|l| l.fed).sum::<usize>(), 45);
        let w = run_all(SEED, Tuition::Witness);
        assert_eq!(w.iter().map(|l| l.burns).sum::<usize>(), 8);
        assert_eq!(w.iter().map(|l| l.fed).sum::<usize>(), 43);
    }

    #[test]
    fn chain_control_reproduces() {
        let c = run_all(SEED, Tuition::Chain);
        assert_eq!(c.iter().map(|l| l.burns).sum::<usize>(), 13);
        assert_eq!(c.iter().map(|l| l.fed).sum::<usize>(), 42);
        assert_eq!(c.iter().map(|l| l.chain_events).sum::<usize>(), 495);
        assert_eq!(c.iter().map(|l| l.new_chain_learners).sum::<usize>(), 18);
    }

    #[test]
    fn placed_refusals_are_witnessed_with_direction() {
        let p = run_all(SEED, Tuition::Placed);
        assert!(p.iter().map(|l| l.placed_events).sum::<usize>() > 0, "no placed refusal witnessed: {p:?}");
        assert!(p.iter().map(|l| l.new_placed_learners).sum::<usize>() > 0, "no moth first learned from a placed refusal: {p:?}");
    }

    #[test]
    fn placed_control_reproduces() {
        let p = run_all(SEED, Tuition::Placed);
        assert_eq!(p.iter().map(|l| l.burns).sum::<usize>(), 13);
        assert_eq!(p.iter().map(|l| l.fed).sum::<usize>(), 39);
        assert_eq!(p.iter().map(|l| l.placed_events).sum::<usize>(), 618);
        assert_eq!(p.iter().map(|l| l.new_placed_learners).sum::<usize>(), 20);
    }

    #[test]
    fn evidence_packets_are_transmitted_and_rejudged() {
        let e = run_all(SEED, Tuition::Evidence);
        assert!(e.iter().map(|l| l.evidence_events).sum::<usize>() > 0, "no evidence packet witnessed: {e:?}");
        let irr = e.iter().map(|l| l.evidence_irregular).sum::<usize>();
        let stdy = e.iter().map(|l| l.evidence_steady).sum::<usize>();
        assert!(irr > 0 && stdy > 0, "evidence must carry both verdict kinds: irregular {irr}, steady {stdy}");
    }

    #[test]
    fn evidence_room_still_feeds_and_lives() {
        let e = run_all(SEED, Tuition::Evidence);
        assert!(e[NIGHTS - 1].alive_end > 0);
        assert!(e.iter().map(|l| l.fed).sum::<usize>() > 0);
        assert!(e.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in evidence room");
    }

    #[test]
    fn placed_room_still_feeds_and_lives() {
        let p = run_all(SEED, Tuition::Placed);
        assert!(p[NIGHTS - 1].alive_end > 0);
        assert!(p.iter().map(|l| l.fed).sum::<usize>() > 0);
    }

    #[test]
    fn placed_room_diverges_from_chain_room() {
        // The direction must change *something* measurable; the
        // headless ledger, not this test, judges whether it helps.
        let c = run_all(SEED, Tuition::Chain);
        let p = run_all(SEED, Tuition::Placed);
        let cb = c.iter().map(|l| l.burns).sum::<usize>();
        let pb = p.iter().map(|l| l.burns).sum::<usize>();
        let cc = c.iter().map(|l| l.checks).sum::<usize>();
        let pc = p.iter().map(|l| l.checks).sum::<usize>();
        let cf = c.iter().map(|l| l.fed).sum::<usize>();
        let pf = p.iter().map(|l| l.fed).sum::<usize>();
        assert!(cb != pb || cc != pc || cf != pf, "placed identical to chain: burns {cb}/{pb} checks {cc}/{pc} fed {cf}/{pf}");
    }

    #[test]
    fn throttled_packets_are_transmitted_and_rejudged() {
        let t = run_all(SEED, Tuition::Throttled);
        assert!(t.iter().map(|l| l.throttled_events).sum::<usize>() > 0, "no throttled packet witnessed: {t:?}");
        let irr = t.iter().map(|l| l.throttled_irregular).sum::<usize>();
        let stdy = t.iter().map(|l| l.throttled_steady).sum::<usize>();
        assert!(irr > 0 && stdy > 0, "throttled evidence must carry both verdict kinds: irregular {irr}, steady {stdy}");
        assert!(t.iter().map(|l| l.throttled_packets_sent).sum::<usize>() <= MOTHS * NIGHTS, "throttle leaked: packets sent exceed one per moth per night");
        assert!(t.iter().map(|l| l.throttled_suppressed).sum::<usize>() > 0, "throttle suppressed nothing: {t:?}");
    }

    #[test]
    fn throttled_room_still_feeds_and_lives() {
        let t = run_all(SEED, Tuition::Throttled);
        assert!(t[NIGHTS - 1].alive_end > 0);
        assert!(t.iter().map(|l| l.fed).sum::<usize>() > 0);
        assert!(t.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in throttled room");
    }

    #[test]
    fn throttled_room_thins_the_flood() {
        let e = run_all(SEED, Tuition::Evidence);
        let t = run_all(SEED, Tuition::Throttled);
        let ee = e.iter().map(|l| l.evidence_events).sum::<usize>();
        let te = t.iter().map(|l| l.throttled_events).sum::<usize>();
        assert!(te < ee, "throttling did not thin the flood: throttled events {te} vs evidence {ee}");
    }

    #[test]
    fn throttled_control_reproduces() {
        let t = run_all(SEED, Tuition::Throttled);
        assert_eq!(t.iter().map(|l| l.burns).sum::<usize>(), 12);
        assert_eq!(t.iter().map(|l| l.fed).sum::<usize>(), 44);
        assert_eq!(t.iter().map(|l| l.throttled_events).sum::<usize>(), 534);
        assert_eq!(t.iter().map(|l| l.new_throttled_learners).sum::<usize>(), 18);
        assert_eq!(t.iter().map(|l| l.throttled_acquittals).sum::<usize>(), 122);
        assert_eq!(t.iter().map(|l| l.throttled_packets_sent).sum::<usize>(), 55);
        assert_eq!(t.iter().map(|l| l.throttled_suppressed).sum::<usize>(), 105);
    }
    #[test]
    fn unshoved_packets_are_transmitted_and_rejudged() {
        let u = run_all(SEED, Tuition::Unshoved);
        assert!(u.iter().map(|l| l.unshoved_events).sum::<usize>() > 0, "no unshoved packet witnessed: {u:?}");
        let irr = u.iter().map(|l| l.unshoved_irregular).sum::<usize>();
        let stdy = u.iter().map(|l| l.unshoved_steady).sum::<usize>();
        assert!(irr > 0 && stdy > 0, "unshoved evidence must carry both verdict kinds: irregular {irr}, steady {stdy}");
        assert!(u.iter().map(|l| l.new_unshoved_learners).sum::<usize>() > 0, "no moth first learned from an unshoved packet: {u:?}");
    }

    #[test]
    fn unshoved_room_still_feeds_and_lives() {
        let u = run_all(SEED, Tuition::Unshoved);
        assert!(u[NIGHTS - 1].alive_end > 0);
        assert!(u.iter().map(|l| l.fed).sum::<usize>() > 0);
        assert!(u.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in unshoved room");
    }

    #[test]
    fn unshoved_room_diverges_from_evidence_room() {
        // Removing the shove must change *something* measurable;
        // the headless ledger, not this test, judges whether it helps.
        let e = run_all(SEED, Tuition::Evidence);
        let u = run_all(SEED, Tuition::Unshoved);
        let eb = e.iter().map(|l| l.burns).sum::<usize>();
        let ub = u.iter().map(|l| l.burns).sum::<usize>();
        let ec = e.iter().map(|l| l.checks).sum::<usize>();
        let uc = u.iter().map(|l| l.checks).sum::<usize>();
        let ef = e.iter().map(|l| l.fed).sum::<usize>();
        let uf = u.iter().map(|l| l.fed).sum::<usize>();
        assert!(eb != ub || ec != uc || ef != uf, "unshoved identical to evidence: burns {eb}/{ub} checks {ec}/{uc} fed {ef}/{uf}");
    }
    #[test]
    fn unshoved_control_reproduces() {
        let u = run_all(SEED, Tuition::Unshoved);
        assert_eq!(u.iter().map(|l| l.burns).sum::<usize>(), 18);
        assert_eq!(u.iter().map(|l| l.fed).sum::<usize>(), 39);
        assert_eq!(u.iter().map(|l| l.unshoved_events).sum::<usize>(), 1605);
        assert_eq!(u.iter().map(|l| l.new_unshoved_learners).sum::<usize>(), 18);
        assert_eq!(u.iter().map(|l| l.unshoved_acquittals).sum::<usize>(), 439);
    }

    #[test]
    fn shoved_packets_are_transmitted_and_rejudged() {
        let h = run_all(SEED, Tuition::Shoved);
        assert!(h.iter().map(|l| l.shoved_events).sum::<usize>() > 0, "no shoved packet witnessed: {h:?}");
        let irr = h.iter().map(|l| l.shoved_irregular).sum::<usize>();
        let stdy = h.iter().map(|l| l.shoved_steady).sum::<usize>();
        assert!(irr > 0 && stdy > 0, "shoved evidence must carry both verdict kinds: irregular {irr}, steady {stdy}");
        assert!(h.iter().map(|l| l.new_shoved_first).sum::<usize>() > 0, "no moth's first shove was taught: {h:?}");
    }

    #[test]
    fn shoved_room_still_feeds_and_lives() {
        let h = run_all(SEED, Tuition::Shoved);
        assert!(h[NIGHTS - 1].alive_end > 0);
        assert!(h.iter().map(|l| l.fed).sum::<usize>() > 0);
        assert!(h.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in shoved room");
    }

    #[test]
    fn shoved_teaches_the_push_not_the_lesson() {
        // The separation itself, measured: a steady packet in the
        // shoved room acquits nobody (nothing was taught), and
        // with no checker recruited by any packet, the room's
        // checks stay far below the evidence room's, where every
        // taught moth goes on to check for itself.
        let h = run_all(SEED, Tuition::Shoved);
        assert_eq!(h.iter().map(|l| l.shoved_acquittals).sum::<usize>(), 0);
        let e = run_all(SEED, Tuition::Evidence);
        let hc = h.iter().map(|l| l.checks).sum::<usize>();
        let ec = e.iter().map(|l| l.checks).sum::<usize>();
        assert!(hc < ec, "shoved checks {hc} not below evidence checks {ec}: recruitment leaked");
    }

    #[test]
    fn shoved_control_reproduces() {
        // The measured ledger of the eighth room, pinned as
        // found -- including the lineage's first death (night 3).
        // The burns comparison against the claim (P4) is reported
        // in the README, not asserted here.
        let h = run_all(SEED, Tuition::Shoved);
        assert_eq!(h.iter().map(|l| l.burns).sum::<usize>(), 14);
        assert_eq!(h.iter().map(|l| l.fed).sum::<usize>(), 44);
        assert_eq!(h.iter().map(|l| l.checks).sum::<usize>(), 136);
        assert_eq!(h.iter().map(|l| l.shoved_events).sum::<usize>(), 867);
        assert_eq!(h.iter().map(|l| l.new_shoved_first).sum::<usize>(), 20);
        assert_eq!(h.iter().map(|l| l.shoved_acquittals).sum::<usize>(), 0);
        assert_eq!(h.iter().map(|l| l.shoved_irregular).sum::<usize>(), 266);
        assert_eq!(h.iter().map(|l| l.shoved_steady).sum::<usize>(), 601);
        assert_eq!(h.iter().map(|l| l.deaths).sum::<usize>(), 1);
        assert_eq!(h[NIGHTS - 1].alive_end, 23);
    }

    #[test]
    fn shoved_room_diverges_from_evidence_and_unshoved() {
        // Removing the lesson must change *something* measurable;
        // the headless ledger, not this test, judges whether the
        // push alone protects.
        let e = run_all(SEED, Tuition::Evidence);
        let u = run_all(SEED, Tuition::Unshoved);
        let h = run_all(SEED, Tuition::Shoved);
        let eb = e.iter().map(|l| l.burns).sum::<usize>();
        let ub = u.iter().map(|l| l.burns).sum::<usize>();
        let hb = h.iter().map(|l| l.burns).sum::<usize>();
        let ec = e.iter().map(|l| l.checks).sum::<usize>();
        let uc = u.iter().map(|l| l.checks).sum::<usize>();
        let hc = h.iter().map(|l| l.checks).sum::<usize>();
        let ef = e.iter().map(|l| l.fed).sum::<usize>();
        let uf = u.iter().map(|l| l.fed).sum::<usize>();
        let hf = h.iter().map(|l| l.fed).sum::<usize>();
        assert!(eb != hb || ec != hc || ef != hf, "shoved identical to evidence: burns {eb}/{hb} checks {ec}/{hc} fed {ef}/{hf}");
        assert!(ub != hb || uc != hc || uf != hf, "shoved identical to unshoved: burns {ub}/{hb} checks {uc}/{hc} fed {uf}/{hf}");
    }

    #[test]
    fn fading_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::FadingTaught, Tuition::FadingEarned, Tuition::FadingBoth] {
            let f = run_all(SEED, t);
            assert!(f.iter().map(|l| l.fading_events).sum::<usize>() > 0, "no fading packet witnessed in {t:?}: {f:?}");
            let irr = f.iter().map(|l| l.fading_irregular).sum::<usize>();
            let stdy = f.iter().map(|l| l.fading_steady).sum::<usize>() ;
            assert!(irr > 0 && stdy > 0, "fading evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(f.iter().map(|l| l.new_fading_learners).sum::<usize>() > 0, "no moth first learned from a fading packet in {t:?}: {f:?}");
        }
    }

    #[test]
    fn fading_night_one_is_evidence_night_one() {
        // No dawn has passed before night 1 ends, so the dawn
        // tax cannot have fired: each fading room's first night
        // is the evidence room's first night, field for field.
        let e = run_all(SEED, Tuition::Evidence);
        for t in [Tuition::FadingTaught, Tuition::FadingEarned, Tuition::FadingBoth] {
            let f = run_all(SEED, t);
            assert_eq!(f[0].fed, e[0].fed, "night 1 fed diverged in {t:?}");
            assert_eq!(f[0].burns, e[0].burns, "night 1 burns diverged in {t:?}");
            assert_eq!(f[0].checks, e[0].checks, "night 1 checks diverged in {t:?}");
            assert_eq!(f[0].refusals, e[0].refusals, "night 1 refusals diverged in {t:?}");
            assert_eq!(f[0].expired_taught + f[0].expired_earned, 0, "a dawn tax fired inside night 1 in {t:?}");
        }
    }

    #[test]
    fn dawn_decay_fires_by_source_as_booked() {
        let ft = run_all(SEED, Tuition::FadingTaught);
        assert!(ft.iter().map(|l| l.expired_taught).sum::<usize>() > 0, "taught pool never taxed in fading-taught: {ft:?}");
        assert_eq!(ft.iter().map(|l| l.expired_earned).sum::<usize>(), 0, "earned pool taxed in fading-taught");
        let fe = run_all(SEED, Tuition::FadingEarned);
        assert!(fe.iter().map(|l| l.expired_earned).sum::<usize>() > 0, "earned pool never taxed in fading-earned: {fe:?}");
        assert_eq!(fe.iter().map(|l| l.expired_taught).sum::<usize>(), 0, "taught pool taxed in fading-earned");
        let fb = run_all(SEED, Tuition::FadingBoth);
        assert!(fb.iter().map(|l| l.expired_taught).sum::<usize>() > 0, "taught pool never taxed in fading-both: {fb:?}");
        assert!(fb.iter().map(|l| l.expired_earned).sum::<usize>() > 0, "earned pool never taxed in fading-both: {fb:?}");
        // And the parents are never taxed at all.
        let e = run_all(SEED, Tuition::Evidence);
        assert_eq!(e.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0);
    }

    #[test]
    fn waning_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::Waning1500, Tuition::Waning750, Tuition::Waning375, Tuition::Waning150] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no waning packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "waning evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a waning packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn waning_clock_fires_and_doses() {
        // The instrument check: expiry fires in every waning
        // room, the dawn tax never fires there, and at lifetime
        // 1500 no unit can expire inside night 1 (no unit is
        // that old yet). The expired totals themselves are
        // reported in the README, not asserted: P2's declared
        // dose monotonicity CRACKED as measured (62 / 85 / 78 /
        // 49 -- the clock taxes the teaching channel that
        // produces the units, so the shortest clocks have
        // fewer units left to expire), and a keeper does not
        // edit the ledger to save a claim.
        for t in [Tuition::Waning1500, Tuition::Waning750, Tuition::Waning375, Tuition::Waning150] {
            let w = run_all(SEED, t);
            let exp: usize = w.iter().map(|l| l.waning_expired_taught + l.waning_expired_earned).sum();
            assert!(exp > 0, "no unit ever expired in {t:?}: {w:?}");
            // The dawn tax never fires in a waning room.
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in waning room {t:?}");
        }
        let w = run_all(SEED, Tuition::Waning1500);
        assert_eq!(w[0].waning_expired_taught + w[0].waning_expired_earned, 0, "a unit expired inside night 1 at lifetime 1500: {w:?}");
    }

    #[test]
    fn waning_rooms_still_feed_and_live() {
        for t in [Tuition::Waning1500, Tuition::Waning750, Tuition::Waning375, Tuition::Waning150] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "waning room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "waning room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in waning room {t:?}");
        }
    }

    #[test]
    fn fading_rooms_still_feed_and_live() {
        for t in [Tuition::FadingTaught, Tuition::FadingEarned, Tuition::FadingBoth] {
            let f = run_all(SEED, t);
            assert!(f[NIGHTS - 1].alive_end > 0, "fading room {t:?} died out");
            assert!(f.iter().map(|l| l.fed).sum::<usize>() > 0, "fading room {t:?} never fed");
            assert!(f.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in fading room {t:?}");
        }
    }

    #[test]
    fn pooled_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::PooledTaught, Tuition::PooledEarned] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no pooled packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "pooled evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a pooled packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn pooled_split_is_exact() {
        // P2, asserted: each pooled room expires its named pool
        // and ONLY its named pool, on the age clock; the dawn
        // tax never fires in either.
        let pt = run_all(SEED, Tuition::PooledTaught);
        assert!(pt.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in pooled-taught: {pt:?}");
        assert_eq!(pt.iter().map(|l| l.waning_expired_earned).sum::<usize>(), 0, "earned pool waned in pooled-taught");
        assert_eq!(pt.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in pooled-taught");
        let pe = run_all(SEED, Tuition::PooledEarned);
        assert!(pe.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in pooled-earned: {pe:?}");
        assert_eq!(pe.iter().map(|l| l.waning_expired_taught).sum::<usize>(), 0, "taught pool waned in pooled-earned");
        assert_eq!(pe.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in pooled-earned");
    }

    #[test]
    fn pooled_rooms_still_feed_and_live() {
        for t in [Tuition::PooledTaught, Tuition::PooledEarned] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "pooled room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "pooled room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in pooled room {t:?}");
        }
    }

    #[test]
    fn pooled_rooms_reproduce() {
        // Measured at planting, pinned as found. P3 CRACKED:
        // neither split carries waning-750's 16 -- the danger
        // is the two pools expiring together.
        let pt = run_all(SEED, Tuition::PooledTaught);
        assert_eq!(pt.iter().map(|l| l.burns).sum::<usize>(), 10);
        assert_eq!(pt.iter().map(|l| l.fed).sum::<usize>(), 43);
        assert_eq!(pt.iter().map(|l| l.checks).sum::<usize>(), 134);
        assert_eq!(pt.iter().map(|l| l.waning_events).sum::<usize>(), 860);
        assert_eq!(pt.iter().map(|l| l.new_waning_learners).sum::<usize>(), 18);
        assert_eq!(pt.iter().map(|l| l.waning_acquittals).sum::<usize>(), 210);
        assert_eq!(pt.iter().map(|l| l.waning_expired_taught).sum::<usize>(), 111);
        let pe = run_all(SEED, Tuition::PooledEarned);
        assert_eq!(pe.iter().map(|l| l.burns).sum::<usize>(), 12);
        assert_eq!(pe.iter().map(|l| l.fed).sum::<usize>(), 54);
        assert_eq!(pe.iter().map(|l| l.checks).sum::<usize>(), 110);
        assert_eq!(pe.iter().map(|l| l.waning_events).sum::<usize>(), 644);
        assert_eq!(pe.iter().map(|l| l.new_waning_learners).sum::<usize>(), 18);
        assert_eq!(pe.iter().map(|l| l.waning_acquittals).sum::<usize>(), 147);
        assert_eq!(pe.iter().map(|l| l.waning_expired_earned).sum::<usize>(), 20);
    }

    #[test]
    fn staggered_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::Staggered, Tuition::StaggeredBoth] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no staggered packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "staggered evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a staggered packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn staggered_offsets_are_as_declared() {
        assert_eq!(Tuition::Staggered.taught_offset(), 375);
        assert_eq!(Tuition::Staggered.earned_offset(), 0);
        assert_eq!(Tuition::StaggeredBoth.taught_offset(), 375);
        assert_eq!(Tuition::StaggeredBoth.earned_offset(), 375);
        assert_eq!(Tuition::Waning750.taught_offset(), 0);
        assert_eq!(Tuition::Waning750.earned_offset(), 0);
        assert_eq!(Tuition::Staggered.waning_lifetime(), Some(750));
        assert_eq!(Tuition::StaggeredBoth.waning_lifetime(), Some(750));
    }

    #[test]
    fn staggered_both_pools_expire_and_no_dawn_tax() {
        // P2, asserted: each staggered room expires both pools
        // on the age clock; the dawn tax never fires in either.
        for t in [Tuition::Staggered, Tuition::StaggeredBoth] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn staggered_rooms_still_feed_and_live() {
        for t in [Tuition::Staggered, Tuition::StaggeredBoth] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "staggered room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "staggered room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in staggered room {t:?}");
        }
    }

    #[test]
    fn staggered_rooms_reproduce() {
        // Measured at planting, pinned as found. P3 HELD past
        // its line (staggered 9, not merely <= 13); P4 CRACKED
        // in its strong form (control 13, not >= 15) -- the
        // ordering staggered 9 < control 13 < waning-750 16 is
        // the finding: duration buys back 3, staggering 4 more.
        let s = run_all(SEED, Tuition::Staggered);
        assert_eq!(s.iter().map(|l| l.burns).sum::<usize>(), 9);
        assert_eq!(s.iter().map(|l| l.fed).sum::<usize>(), 50);
        assert_eq!(s.iter().map(|l| l.checks).sum::<usize>(), 116);
        assert_eq!(s.iter().map(|l| l.waning_events).sum::<usize>(), 611);
        assert_eq!(s.iter().map(|l| l.new_waning_learners).sum::<usize>(), 18);
        assert_eq!(s.iter().map(|l| l.waning_acquittals).sum::<usize>(), 145);
        assert_eq!(s.iter().map(|l| l.waning_expired_taught).sum::<usize>(), 55);
        assert_eq!(s.iter().map(|l| l.waning_expired_earned).sum::<usize>(), 15);
        let b = run_all(SEED, Tuition::StaggeredBoth);
        assert_eq!(b.iter().map(|l| l.burns).sum::<usize>(), 13);
        assert_eq!(b.iter().map(|l| l.fed).sum::<usize>(), 45);
        assert_eq!(b.iter().map(|l| l.checks).sum::<usize>(), 128);
        assert_eq!(b.iter().map(|l| l.waning_events).sum::<usize>(), 836);
        assert_eq!(b.iter().map(|l| l.new_waning_learners).sum::<usize>(), 18);
        assert_eq!(b.iter().map(|l| l.waning_acquittals).sum::<usize>(), 183);
        assert_eq!(b.iter().map(|l| l.waning_expired_taught).sum::<usize>(), 58);
        assert_eq!(b.iter().map(|l| l.waning_expired_earned).sum::<usize>(), 19);
    }

    #[test]
    fn mirrored_packets_are_transmitted_and_rejudged() {
        let w = run_all(SEED, Tuition::Mirrored);
        assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no mirrored packet witnessed: {w:?}");
        let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
        let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
        assert!(irr > 0 && stdy > 0, "mirrored evidence must carry both verdict kinds: irregular {irr}, steady {stdy}");
        assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a mirrored packet: {w:?}");
    }

    #[test]
    fn mirrored_offsets_are_the_mirror_image() {
        // P2's instrument half, asserted: the offsets are the
        // staggered room's, reflected.
        assert_eq!(Tuition::Mirrored.taught_offset(), 0);
        assert_eq!(Tuition::Mirrored.earned_offset(), 375);
        assert_eq!(Tuition::Staggered.taught_offset(), 375);
        assert_eq!(Tuition::Staggered.earned_offset(), 0);
        assert_eq!(Tuition::Mirrored.waning_lifetime(), Some(750));
    }

    #[test]
    fn mirrored_both_pools_expire_and_no_dawn_tax() {
        // P2, asserted: the mirrored room expires both pools on
        // the age clock; the dawn tax never fires in it.
        let w = run_all(SEED, Tuition::Mirrored);
        assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in mirrored: {w:?}");
        assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in mirrored: {w:?}");
        assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in mirrored");
    }

    #[test]
    fn mirrored_room_reproduces() {
        // Measured at planting, pinned as found. All six
        // predictions HELD: P3 symmetry (11 within three of
        // staggered's 9), P4 (11 < the coincident control's 13).
        // The full ordering: staggered 9 < mirrored 11 <
        // staggered-both 13 < waning-750 16.
        let w = run_all(SEED, Tuition::Mirrored);
        assert_eq!(w.iter().map(|l| l.burns).sum::<usize>(), 11);
        assert_eq!(w.iter().map(|l| l.fed).sum::<usize>(), 46);
        assert_eq!(w.iter().map(|l| l.checks).sum::<usize>(), 129);
        assert_eq!(w.iter().map(|l| l.waning_events).sum::<usize>(), 662);
        assert_eq!(w.iter().map(|l| l.new_waning_learners).sum::<usize>(), 18);
        assert_eq!(w.iter().map(|l| l.waning_acquittals).sum::<usize>(), 162);
        assert_eq!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>(), 99);
        assert_eq!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>(), 12);
    }

    #[test]
    fn mirrored_room_still_feeds_and_lives() {
        let w = run_all(SEED, Tuition::Mirrored);
        assert!(w[NIGHTS - 1].alive_end > 0, "mirrored room died out");
        assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "mirrored room never fed");
        assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in mirrored room");
    }

    #[test]
    fn wide_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::StaggeredWide, Tuition::MirroredWide, Tuition::CoincidentWide] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no wide packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "wide evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a wide packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn wide_offsets_are_the_wide_staggers() {
        // P2's instrument half, asserted: +750 on the waiting
        // pool, the mirror image at the wide offset, and 750/750
        // in the coincident control.
        assert_eq!(Tuition::StaggeredWide.taught_offset(), 750);
        assert_eq!(Tuition::StaggeredWide.earned_offset(), 0);
        assert_eq!(Tuition::MirroredWide.taught_offset(), 0);
        assert_eq!(Tuition::MirroredWide.earned_offset(), 750);
        assert_eq!(Tuition::CoincidentWide.taught_offset(), 750);
        assert_eq!(Tuition::CoincidentWide.earned_offset(), 750);
        assert_eq!(Tuition::StaggeredWide.waning_lifetime(), Some(750));
        assert_eq!(Tuition::MirroredWide.waning_lifetime(), Some(750));
        assert_eq!(Tuition::CoincidentWide.waning_lifetime(), Some(750));
    }

    #[test]
    fn wide_rooms_expire_both_pools_and_no_dawn_tax() {
        // P2, asserted: every wide room expires both pools on
        // the age clock; the dawn tax never fires in them.
        for t in [Tuition::StaggeredWide, Tuition::MirroredWide, Tuition::CoincidentWide] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn wide_rooms_still_feed_and_live() {
        for t in [Tuition::StaggeredWide, Tuition::MirroredWide, Tuition::CoincidentWide] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "wide room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "wide room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in wide room {t:?}");
        }
    }

    #[test]
    fn step_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::StaggeredStep, Tuition::MirroredStep, Tuition::CoincidentStep] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no step packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "step evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a step packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn step_offsets_are_the_claimstep_staggers() {
        // P2's instrument half, asserted: +450 on the waiting
        // pool, the mirror image at the claim-step, and 450/450
        // in the coincident control.
        assert_eq!(Tuition::StaggeredStep.taught_offset(), 450);
        assert_eq!(Tuition::StaggeredStep.earned_offset(), 0);
        assert_eq!(Tuition::MirroredStep.taught_offset(), 0);
        assert_eq!(Tuition::MirroredStep.earned_offset(), 450);
        assert_eq!(Tuition::CoincidentStep.taught_offset(), 450);
        assert_eq!(Tuition::CoincidentStep.earned_offset(), 450);
        assert_eq!(Tuition::StaggeredStep.waning_lifetime(), Some(750));
        assert_eq!(Tuition::MirroredStep.waning_lifetime(), Some(750));
        assert_eq!(Tuition::CoincidentStep.waning_lifetime(), Some(750));
    }

    #[test]
    fn step_rooms_expire_both_pools_and_no_dawn_tax() {
        // P2, asserted: every step room expires both pools on
        // the age clock; the dawn tax never fires in them.
        for t in [Tuition::StaggeredStep, Tuition::MirroredStep, Tuition::CoincidentStep] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn step_rooms_still_feed_and_live() {
        for t in [Tuition::StaggeredStep, Tuition::MirroredStep, Tuition::CoincidentStep] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "step room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "step room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in step room {t:?}");
        }
    }

    #[test]
    fn edge_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::StaggeredEdge, Tuition::MirroredEdge, Tuition::CoincidentEdge] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no edge packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "edge evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from an edge packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn edge_offsets_are_the_midway_staggers() {
        // P2's instrument half, asserted: +600 on the waiting
        // pool, the mirror image at the shelf's edge, and 600/600
        // in the coincident control.
        assert_eq!(Tuition::StaggeredEdge.taught_offset(), 600);
        assert_eq!(Tuition::StaggeredEdge.earned_offset(), 0);
        assert_eq!(Tuition::MirroredEdge.taught_offset(), 0);
        assert_eq!(Tuition::MirroredEdge.earned_offset(), 600);
        assert_eq!(Tuition::CoincidentEdge.taught_offset(), 600);
        assert_eq!(Tuition::CoincidentEdge.earned_offset(), 600);
        assert_eq!(Tuition::StaggeredEdge.waning_lifetime(), Some(750));
        assert_eq!(Tuition::MirroredEdge.waning_lifetime(), Some(750));
        assert_eq!(Tuition::CoincidentEdge.waning_lifetime(), Some(750));
    }

    #[test]
    fn edge_rooms_expire_both_pools_and_no_dawn_tax() {
        // P2, asserted: every edge room expires both pools on
        // the age clock; the dawn tax never fires in them.
        for t in [Tuition::StaggeredEdge, Tuition::MirroredEdge, Tuition::CoincidentEdge] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn edge_rooms_still_feed_and_live() {
        for t in [Tuition::StaggeredEdge, Tuition::MirroredEdge, Tuition::CoincidentEdge] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "edge room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "edge room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in edge room {t:?}");
        }
    }

    #[test]
    fn face_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::StaggeredFace, Tuition::MirroredFace, Tuition::CoincidentFace] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no face packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "face evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a face packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn face_offsets_are_the_midway_staggers() {
        // P2's instrument half, asserted: +525 on the waiting
        // pool, the mirror image at the cliff's face, and
        // 525/525 in the coincident control.
        assert_eq!(Tuition::StaggeredFace.taught_offset(), 525);
        assert_eq!(Tuition::StaggeredFace.earned_offset(), 0);
        assert_eq!(Tuition::MirroredFace.taught_offset(), 0);
        assert_eq!(Tuition::MirroredFace.earned_offset(), 525);
        assert_eq!(Tuition::CoincidentFace.taught_offset(), 525);
        assert_eq!(Tuition::CoincidentFace.earned_offset(), 525);
        assert_eq!(Tuition::StaggeredFace.waning_lifetime(), Some(750));
        assert_eq!(Tuition::MirroredFace.waning_lifetime(), Some(750));
        assert_eq!(Tuition::CoincidentFace.waning_lifetime(), Some(750));
    }

    #[test]
    fn face_rooms_expire_both_pools_and_no_dawn_tax() {
        // P2, asserted: every face room expires both pools on
        // the age clock; the dawn tax never fires in them.
        for t in [Tuition::StaggeredFace, Tuition::MirroredFace, Tuition::CoincidentFace] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn face_rooms_still_feed_and_live() {
        for t in [Tuition::StaggeredFace, Tuition::MirroredFace, Tuition::CoincidentFace] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "face room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "face room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in face room {t:?}");
        }
    }

    #[test]
    fn grain_packets_are_transmitted_and_rejudged() {
        for t in [Tuition::Staggered475, Tuition::Mirrored475, Tuition::Coincident475, Tuition::Staggered500, Tuition::Mirrored500, Tuition::Coincident500] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_events).sum::<usize>() > 0, "no grain packet witnessed in {t:?}: {w:?}");
            let irr = w.iter().map(|l| l.waning_irregular).sum::<usize>();
            let stdy = w.iter().map(|l| l.waning_steady).sum::<usize>();
            assert!(irr > 0 && stdy > 0, "grain evidence must carry both verdict kinds in {t:?}: irregular {irr}, steady {stdy}");
            assert!(w.iter().map(|l| l.new_waning_learners).sum::<usize>() > 0, "no moth first learned from a grain packet in {t:?}: {w:?}");
        }
    }

    #[test]
    fn grain_offsets_are_the_finer_staggers() {
        // P2's instrument half, asserted: +475 / +500 on the
        // waiting pool, the mirror image at each grain, and
        // 475/475, 500/500 in the coincident controls.
        assert_eq!(Tuition::Staggered475.taught_offset(), 475);
        assert_eq!(Tuition::Staggered475.earned_offset(), 0);
        assert_eq!(Tuition::Mirrored475.taught_offset(), 0);
        assert_eq!(Tuition::Mirrored475.earned_offset(), 475);
        assert_eq!(Tuition::Coincident475.taught_offset(), 475);
        assert_eq!(Tuition::Coincident475.earned_offset(), 475);
        assert_eq!(Tuition::Staggered500.taught_offset(), 500);
        assert_eq!(Tuition::Staggered500.earned_offset(), 0);
        assert_eq!(Tuition::Mirrored500.taught_offset(), 0);
        assert_eq!(Tuition::Mirrored500.earned_offset(), 500);
        assert_eq!(Tuition::Coincident500.taught_offset(), 500);
        assert_eq!(Tuition::Coincident500.earned_offset(), 500);
        for t in [Tuition::Staggered475, Tuition::Mirrored475, Tuition::Coincident475, Tuition::Staggered500, Tuition::Mirrored500, Tuition::Coincident500] {
            assert_eq!(t.waning_lifetime(), Some(750));
        }
    }

    #[test]
    fn grain_rooms_expire_both_pools_and_no_dawn_tax() {
        // P2, asserted: every grain room expires both pools on
        // the age clock; the dawn tax never fires in them.
        for t in [Tuition::Staggered475, Tuition::Mirrored475, Tuition::Coincident475, Tuition::Staggered500, Tuition::Mirrored500, Tuition::Coincident500] {
            let w = run_all(SEED, t);
            assert!(w.iter().map(|l| l.waning_expired_taught).sum::<usize>() > 0, "taught pool never waned in {t:?}: {w:?}");
            assert!(w.iter().map(|l| l.waning_expired_earned).sum::<usize>() > 0, "earned pool never waned in {t:?}: {w:?}");
            assert_eq!(w.iter().map(|l| l.expired_taught + l.expired_earned).sum::<usize>(), 0, "a dawn tax fired in {t:?}");
        }
    }

    #[test]
    fn grain_rooms_still_feed_and_live() {
        for t in [Tuition::Staggered475, Tuition::Mirrored475, Tuition::Coincident475, Tuition::Staggered500, Tuition::Mirrored500, Tuition::Coincident500] {
            let w = run_all(SEED, t);
            assert!(w[NIGHTS - 1].alive_end > 0, "grain room {t:?} died out");
            assert!(w.iter().map(|l| l.fed).sum::<usize>() > 0, "grain room {t:?} never fed");
            assert!(w.iter().map(|l| l.burns).sum::<usize>() > 0, "decoy fooled no one in grain room {t:?}");
        }
    }

    #[test]
    fn grain_prices_are_the_plantings_question() {
        // The measured prices, pinned as found on the first run
        // (see README ledger): staggered and coincident at the
        // bottom of the cliff at both grains, mirrored flat.
        let s475 = run_all(SEED, Tuition::Staggered475);
        let s500 = run_all(SEED, Tuition::Staggered500);
        let m475 = run_all(SEED, Tuition::Mirrored475);
        let m500 = run_all(SEED, Tuition::Mirrored500);
        let c475 = run_all(SEED, Tuition::Coincident475);
        let c500 = run_all(SEED, Tuition::Coincident500);
        let burns = |w: &[NightLedger]| w.iter().map(|l| l.burns).sum::<usize>();
        assert_eq!(burns(&s475), 14, "staggered-475 price moved");
        assert_eq!(burns(&s500), 16, "staggered-500 price moved");
        assert_eq!(burns(&m475), 11, "mirrored-475 price moved");
        assert_eq!(burns(&m500), 11, "mirrored-500 price moved");
        assert_eq!(burns(&c475), 14, "coincident-475 price moved");
        assert_eq!(burns(&c500), 16, "coincident-500 price moved");
    }
}
