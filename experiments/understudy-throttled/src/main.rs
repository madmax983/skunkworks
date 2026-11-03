//! # Understudy Throttled \U0001F3AD
//!
//! Brought by The Understudy \U0001F3AD. Principal: `experiments/throttled-moths`
//! (The Moth Keeper \U0001FA94), where a sixth FDTD room rationed evidence
//! teaching to one packet per checker per night: packets sent fell
//! 151 -> 55, events 943 -> 534, all 18 first-cautions kept, acquittals
//! halved (223 -> 122), feeding rose to 44 — and burns did not move
//! (12, the evidence room's count; P4 CRACKED). The principal's verdict:
//! volume was not the disease; the first telling is the teaching.
//!
//! The Understudy performs that sixth room on the bus machinery of
//! `understudy-evidence`: no field, no grid, no positions. The first
//! five rooms are that performance's script unchanged (their ledgers
//! are pinned exact by tests); the throttled room adds one gate — a
//! checker posts only its FIRST completed check each night, later
//! checks are counted as suppressed and judged for itself alone.
//! Packets carry the lamp's own clock, exact and unmixed, re-judged
//! by every recipient on the same 0.35 CV line: irregular earns
//! caution, steady can acquit hearsay caution.
//!
//! One difference is built into this stage and named in the README:
//! on this stage the evidence room's disease was never volume alone
//! — it was the acquittal leak (evidence burns 16 vs chain 5, with
//! 156 acquittals). Rationing thins packets *and* acquittals at
//! once. If burns fall here where the cave's did not, the cave's
//! crack was the mixture blunting a real ration effect; if burns
//! hold at the evidence room's count on an exact stage too, the
//! first telling is convicted on both mediums.
//!
//! Tolerances and predictions were declared in the README *before*
//! this crate was built or run. The headless ledger prints the
//! principal's numbers beside this performance's and names each
//! line HELD or CRACKED.
//!
//! `cargo run -p understudy-throttled -- --headless` prints the
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    SelfBurn,
    WitnessBurn,
    ChainRefusal,
    PlacedRefusal,
    EvidencePacket,
    ThrottledPacket,
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
        } else {
            ledger.throttled_steady += 1;
            if m.personal_burns == 0 && m.caution > 0 {
                m.caution -= 1;
                ledger.throttled_acquittals += 1;
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
            "    night {}: fed {}, burns {}, deaths {}, checks {}, refusals {}, averted {}, witness events {}, chain events {}, placed events {}, evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), throttled events {} (irregular {}, steady {}, acquittals {}, learners {}, sent {}, suppressed {}), alive at dawn {}",
            n + 1, l.fed, l.burns, l.deaths, l.checks, l.refusals, l.averted,
            l.witness_events, l.chain_events, l.placed_events, l.evidence_events,
            l.evidence_irregular, l.evidence_steady, l.evidence_acquittals,
            l.new_evidence_learners, l.throttled_events, l.throttled_irregular,
            l.throttled_steady, l.throttled_acquittals, l.new_throttled_learners,
            l.packets_sent, l.packets_suppressed, l.alive_end
        );
    }
    println!(
        "    totals: fed {}, burns {}, checks {}, refusals {}, averted {}, honest refusals {}, witness events {} (learners {}), chain events {} (learners {}), placed events {} (learners {}), evidence events {} (irregular {}, steady {}, acquittals {}, learners {}), throttled events {} (irregular {}, steady {}, acquittals {}, learners {}), packets sent {}, suppressed {}, alive {}",
        totals.fed, totals.burns, totals.checks, totals.refusals, totals.averted,
        totals.honest_refusals, totals.witness_events, totals.witness_learners,
        totals.chain_events, totals.chain_learners, totals.placed_events,
        totals.placed_learners, totals.evidence_events, totals.evidence_irregular,
        totals.evidence_steady, totals.evidence_acquittals, totals.evidence_learners,
        totals.throttled_events, totals.throttled_irregular, totals.throttled_steady,
        totals.throttled_acquittals, totals.throttled_learners, totals.packets_sent,
        totals.packets_suppressed, totals.alive_end
    );
}

fn run_headless() {
    let (solo_n, solo) = run_room(Room::Solo, SEED);
    let (wit_n, wit) = run_room(Room::Witness, SEED);
    let (chain_n, chain) = run_room(Room::Chain, SEED);
    let (placed_n, placed) = run_room(Room::Placed, SEED);
    let (ev_n, ev) = run_room(Room::Evidence, SEED);
    let (th_n, th) = run_room(Room::Throttled, SEED);
    println!("understudy-throttled headless: the Throttled role, performed by queues \u{2014} no field on stage");
    println!(
        "  lamps: honest period {HONEST_PERIOD} (feeds), decoy intervals {DECOY_SHORT}/{DECOY_LONG} same mean (burns); brightness has no channel here"
    );
    println!(
        "  moths: {MOTHS} per room, {NIGHTS} nights, {NIGHT_TICKS} ticks per night; claim wait {WAIT_MIN}..={WAIT_MAX}; bus delivery burn {BURN_DELIVERY}, refusal {REFUSAL_DELIVERY}, evidence {EVIDENCE_DELIVERY}"
    );
    println!("  principal's ledger (throttled-moths, cave): solo fed 45 burns 12; witness fed 43 burns 8; chain fed 42 burns 13 (events 495, learners 18); placed fed 39 burns 13 (events 618, learners 20); evidence fed 43 burns 12, checks 151 (packets 943: irregular 342, steady 601; learners 18, acquittals 223); throttled fed 44 burns 12, checks 169 (packets sent 55, suppressed 105, events 534: irregular 234, steady 300; learners 18, acquittals 122)");
    println!("  this stage's evidence room (understudy-evidence, pinned): fed 57 burns 16, checks 92 (packets 634: irregular 277, steady 357; learners 11, acquittals 156)");
    print_room("solo (caution only from your own burns)", &solo_n, &solo);
    print_room("witness (caution also from a delivered burn message)", &wit_n, &wit);
    print_room("chain (caution also from a delivered refusal message)", &chain_n, &chain);
    print_room("placed (a delivered refusal also names its lamp)", &placed_n, &placed);
    print_room("evidence (every check posts its intervals; recipients re-judge)", &ev_n, &ev);
    print_room("throttled (only a checker's first check each night posts)", &th_n, &th);
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
        "    P1 throttled packets in both kinds: irregular {} > 0 and steady {} > 0 [{}] (principal 234 / 300)",
        th.throttled_irregular, th.throttled_steady,
        verdict(th.throttled_irregular > 0 && th.throttled_steady > 0)
    );
    println!(
        "    P2 throttled-first learners {} [{} > 0] (principal 18; this stage's evidence room 11)",
        th.throttled_learners, verdict(th.throttled_learners > 0)
    );
    println!(
        "    P3 throttled acquittals {} [{} > 0 and < evidence {}] \u{2014} rationing thins the un-teaching too (principal 122 < 223)",
        th.throttled_acquittals, verdict(th.throttled_acquittals > 0 && th.throttled_acquittals < ev.evidence_acquittals), ev.evidence_acquittals
    );
    println!(
        "    P4 throttled burns {} < evidence burns {} on this stage [{}] (principal CRACKED in the cave: 12 vs 12; the claim)",
        th.burns, ev.burns, verdict(th.burns < ev.burns)
    );
    println!(
        "    P5 packets sent {} <= 72 and suppressed {} > 0 [{}] \u{2014} the ration is one per checker per night (principal 55 sent, 105 suppressed)",
        th.packets_sent, th.packets_suppressed,
        verdict(th.packets_sent <= 72 && th.packets_suppressed > 0 && th.packets_sent + th.packets_suppressed == th.checks)
    );
    println!(
        "    P6 throttled events {} < evidence events {} on this stage [{}] (principal 534 < 943)",
        th.throttled_events, ev.evidence_events, verdict(th.throttled_events < ev.evidence_events)
    );
    println!(
        "    honest refusals across all rooms {} (counted, not a declared line on this stage; queue precedent 0)",
        solo.honest_refusals + wit.honest_refusals + chain.honest_refusals + placed.honest_refusals + ev.honest_refusals + th.honest_refusals
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
        // The first five rooms are the understudy-evidence script
        // unchanged; their ledgers must match that performance.
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
        for room in [Room::Solo, Room::Witness, Room::Chain, Room::Placed, Room::Evidence, Room::Throttled] {
            let (_, totals) = run_room(room, SEED);
            assert_eq!(totals.honest_refusals, 0, "{room:?} refused the honest lamp");
        }
    }

    #[test]
    fn the_performance_is_deterministic() {
        let (_, a) = run_room(Room::Throttled, SEED);
        let (_, b) = run_room(Room::Throttled, SEED);
        assert_eq!(a.burns, b.burns);
        assert_eq!(a.throttled_events, b.throttled_events);
        assert_eq!(a.throttled_acquittals, b.throttled_acquittals);
        assert_eq!(a.packets_sent, b.packets_sent);
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
}
