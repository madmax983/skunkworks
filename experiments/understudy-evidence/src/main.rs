//! # Understudy Evidence 🎭
//!
//! Brought by The Understudy 🎭. Principal: `experiments/evidence-moths`
//! (The Moth Keeper 🪔), where five twin FDTD rooms — solo, witness,
//! chain, placed, evidence — asked whether a check that transmits its
//! actual peak intervals, re-judged by every witness, can protect
//! where verdicts and directions could not. It taught superbly and
//! acquitted superbly (943 packets, 18 first-cautions, 223 acquittals)
//! and protected by exactly one burn (12, vs chain 13, witness 8).
//! The principal's verdict: evidence survives third hand; volume,
//! not fidelity, is the remaining disease.
//!
//! The Understudy performs the same five rooms with the machinery
//! thrown out entire: no field, no grid, no positions, no flight.
//! Each moth owns a claim queue (the machinery of `understudy-moths`,
//! reused as a stage). Social teaching is a message bus: burns are
//! quiet (delivery 0.05), refusal verdicts are public (0.30, placed
//! refusals also name their lamp, exactly as in `understudy-placed`),
//! and in the evidence room every completed check posts its actual
//! sampled intervals as a packet (delivery 0.30, matching the
//! principal's ~6 deliveries per check). A recipient re-computes
//! the interval CV itself, on the same 0.35 line, and is never told
//! the checker's verdict: irregular earns caution; steady can
//! acquit hearsay caution a moth never paid a personal burn for.
//! One difference is built into this stage and named in the README:
//! on a bus the sample is the lamp's own clock, exact — no mixture
//! ever dilutes it — so re-judging here is re-judging perfect
//! evidence, where the cave's witnesses re-judged a mixture.
//!
//! Tolerances and predictions were declared in the README *before*
//! this crate was built or run. The headless ledger prints the
//! principal's numbers beside this performance's and names each
//! line HELD or CRACKED.
//!
//! `cargo run -p understudy-evidence -- --headless` prints the
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    SelfBurn,
    WitnessBurn,
    ChainRefusal,
    PlacedRefusal,
    EvidencePacket,
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
        } else {
            ledger.evidence_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                ledger.evidence_acquittals += 1;
            }
        }
    }
}

fn run_room(room: Room, seed: u64) -> (Vec<NightLedger>, RoomTotals) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut moths: Vec<Moth> = (0..MOTHS).map(|_| Moth::new()).collect();
    let mut nights = Vec::new();
    for _ in 0..NIGHTS {
        let mut ledger = NightLedger::default();
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
            let mut t: u64 = 0;
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
                if room == Room::Evidence {
                    posted_packets.push(intervals.clone());
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
            "    night {}: fed {}, burns {}, deaths {}, checks {}, refusals {}, averted {}, witness events {}, chain events {}, placed events {}, evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), alive at dawn {}",
            n + 1, l.fed, l.burns, l.deaths, l.checks, l.refusals, l.averted,
            l.witness_events, l.chain_events, l.placed_events, l.evidence_events,
            l.evidence_irregular, l.evidence_steady, l.evidence_acquittals,
            l.new_evidence_learners, l.alive_end
        );
    }
    println!(
        "    totals: fed {}, burns {}, checks {}, refusals {}, averted {}, honest refusals {}, witness events {} (learners {}), chain events {} (learners {}), placed events {} (learners {}), evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), alive {}",
        totals.fed, totals.burns, totals.checks, totals.refusals, totals.averted,
        totals.honest_refusals, totals.witness_events, totals.witness_learners,
        totals.chain_events, totals.chain_learners, totals.placed_events,
        totals.placed_learners, totals.evidence_events, totals.evidence_irregular,
        totals.evidence_steady, totals.evidence_acquittals, totals.evidence_learners,
        totals.alive_end
    );
}

fn run_headless() {
    let (solo_n, solo) = run_room(Room::Solo, SEED);
    let (wit_n, wit) = run_room(Room::Witness, SEED);
    let (chain_n, chain) = run_room(Room::Chain, SEED);
    let (placed_n, placed) = run_room(Room::Placed, SEED);
    let (ev_n, ev) = run_room(Room::Evidence, SEED);
    println!("understudy-evidence headless: the Evidence role, performed by queues \u{2014} no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX}; bus delivery burn {BURN_DELIVERY}, refusal {REFUSAL_DELIVERY}, evidence {EVIDENCE_DELIVERY}"
    );
    println!("  principal's ledger: solo fed 45 burns 12; witness fed 43 burns 8 (events 13, learners 6); chain fed 42 burns 13 (events 495, learners 18); placed fed 39 burns 13 (events 618, learners 20); evidence fed 43 burns 12, checks 151, refusals 50 (packets 943: irregular 342, steady 601; learners 18, acquittals 223)");
    print_room("solo (caution only from your own burns)", &solo_n, &solo);
    print_room("witness (caution also from a delivered burn message)", &wit_n, &wit);
    print_room("chain (caution also from a delivered refusal message)", &chain_n, &chain);
    print_room("placed (a delivered refusal also names its lamp)", &placed_n, &placed);
    print_room("evidence (every check posts its intervals; recipients re-judge)", &ev_n, &ev);
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
        "    P1 evidence packets in both kinds: irregular {} > 0 and steady {} > 0 [{}] (principal 342 / 601)",
        ev.evidence_irregular, ev.evidence_steady,
        verdict(ev.evidence_irregular > 0 && ev.evidence_steady > 0)
    );
    println!(
        "    P2 evidence-first learners {} [{} > 0] (principal 18)",
        ev.evidence_learners, verdict(ev.evidence_learners > 0)
    );
    println!(
        "    P3 evidence acquittals {} [{} > 0] \u{2014} a room on this stage can also lose a lesson (principal 223)",
        ev.evidence_acquittals, verdict(ev.evidence_acquittals > 0)
    );
    println!(
        "    P4 evidence burns {} < chain burns {} on this stage [{}] (principal HELD by one: 12 < 13; witness cave 8)",
        ev.burns, chain.burns, verdict(ev.burns < chain.burns)
    );
    println!(
        "    P5 evidence checks {} < chain checks {} on this stage [{}] (principal 151 < 208)",
        ev.checks, chain.checks, verdict(ev.checks < chain.checks)
    );
    println!(
        "    P6 evidence events {} > chain events {} on this stage [{}] (principal 943 > 495)",
        ev.evidence_events, chain.chain_events, verdict(ev.evidence_events > chain.chain_events)
    );
    println!(
        "    honest refusals across all rooms {} (counted, not a declared line on this stage; queue precedent 0)",
        solo.honest_refusals + wit.honest_refusals + chain.honest_refusals + placed.honest_refusals + ev.honest_refusals
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
        assert_eq!(solo.witness_events + solo.chain_events + solo.placed_events + solo.evidence_events, 0);
    }

    #[test]
    fn the_control_rooms_reproduce_the_queue_precedent() {
        // The first four rooms are the understudy-placed script
        // unchanged; their ledgers must match that performance.
        let (_, solo) = run_room(Room::Solo, SEED);
        assert_eq!(solo.burns, 20);
        assert_eq!(solo.fed, 61);
        let (_, chain) = run_room(Room::Chain, SEED);
        assert_eq!(chain.chain_events, 305);
        let (_, placed) = run_room(Room::Placed, SEED);
        assert_eq!(placed.placed_events, 360);
        assert_eq!(placed.averted, 50);
    }

    #[test]
    fn evidence_packets_travel_in_both_kinds() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert!(ev.evidence_events > 0, "no evidence packet was ever delivered: {ev:?}");
        assert!(ev.evidence_irregular > 0, "no irregular packet delivered: {ev:?}");
        assert!(ev.evidence_steady > 0, "no steady packet delivered: {ev:?}");
        assert_eq!(ev.evidence_irregular + ev.evidence_steady, ev.evidence_events);
    }

    #[test]
    fn evidence_teaches_a_first_caution() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert!(ev.evidence_learners > 0, "no moth's first caution was re-judged evidence: {ev:?}");
    }

    #[test]
    fn steady_evidence_can_acquit_hearsay_caution() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert!(ev.evidence_acquittals > 0, "no hearsay caution was ever acquitted: {ev:?}");
    }

    #[test]
    fn the_evidence_room_still_feeds_lives_and_burns() {
        let (_, ev) = run_room(Room::Evidence, SEED);
        assert!(ev.fed > 0, "evidence room never fed: {ev:?}");
        assert!(ev.burns > 0, "evidence room never burned: {ev:?}");
        assert_eq!(ev.alive_end, 24, "evidence room lost a moth: {ev:?}");
    }

    #[test]
    fn no_room_ever_refuses_the_honest_lamp() {
        for room in [Room::Solo, Room::Witness, Room::Chain, Room::Placed, Room::Evidence] {
            let (_, totals) = run_room(room, SEED);
            assert_eq!(totals.honest_refusals, 0, "{room:?} refused the honest lamp");
        }
    }

    #[test]
    fn the_performance_is_deterministic() {
        let (_, a) = run_room(Room::Evidence, SEED);
        let (_, b) = run_room(Room::Evidence, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.evidence_events, b.evidence_events);
        assert_eq!(a.evidence_acquittals, b.evidence_acquittals);
    }
}
