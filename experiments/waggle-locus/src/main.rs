//! waggle-locus — a revival cross: graveyard/waggle-dance × crates/locus.
//!
//! The corpse (443 LOC, macroquad) staged honeybee quorum sensing:
//! scouts find food sites, return to the hive, and dance; observers
//! watch a dance and are recruited to its site; the hive converges on
//! the best site with no leader. It was executed for Ecosystem
//! Maladaptation / Skeletal Implementation — its headless GUI panic
//! was never resolved inside the grace period — and two deeper flaws
//! its certificate never named are visible in its own source:
//!
//! - The dance was a label, not a channel. `DanceInfo` carried
//!   `target_index` and foragers flew to the site's true position;
//!   the dance's angle field sat `#[allow(dead_code)]`, never read
//!   by anyone. No observer ever decoded anything.
//! - The map was wrong about its own dance. The README says dance
//!   duration is proportional to distance; the code sets
//!   `dance_duration = 100.0 * memory_quality`. Distance — the one
//!   thing a real waggle dance exists to carry — was encoded nowhere.
//!
//! The living `locus` crate supplies the organ whose absence killed
//! it: geometry without a window. The corpse reached for macroquad's
//! `Vec2` and got a GUI as a side effect; `locus::Vec2` is the same
//! geometry with no display attached, which is exactly what a
//! headless bypass is made of.
//!
//! The revival runs the corpse's hive three times on identical
//! streams, once per channel, so the dance itself is priced:
//! - Label:   the corpse's own channel — a dance hands over the
//!            site's identity and true position; recruits fly
//!            straight there. Perfect, and a cheat.
//! - Vector:  the dance carries only what a real dance carries —
//!            a waggle angle (direction) and a waggle-run length
//!            (distance, 10 units per tick of run). Observers decode
//!            both through noise, fly to the decoded point, and
//!            search. They may find the site, a different site, or
//!            nothing.
//! - Quality: the dance advertises quality but no vector at all;
//!            recruits fly out on a random bearing and search.
//!
//! Streams are purpose-split SplitMix64 (scout, recruit, dance
//! noise, search), identically seeded across channels — divergence
//! between the three ledgers is the channel's doing.

use anyhow::Result;
use locus::Vec2;
use std::f64::consts::TAU;

const BEES: usize = 60;
const TICKS: u32 = 6000;
const SPEED: f64 = 3.0;
const SIGHT: f64 = 22.0;
const FIND_RADIUS: f64 = 12.0;
const SEARCH_TICKS: u32 = 120;
const QUORUM: usize = BEES / 2;
const ANGLE_SIGMA: f64 = 0.085;
const RUN_SIGMA: f64 = 0.07;
const SEED: u64 = 0xBE35_1E00_0000_0001_u64;

/// Deterministic SplitMix64 — one stream per purpose.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }
    fn gauss(&mut self) -> f64 {
        // Box–Muller.
        let u1 = (self.unit()).max(1e-12);
        let u2 = self.unit();
        (-2.0 * u1.ln()).sqrt() * (TAU * u2).cos()
    }
    fn pick(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Channel {
    Label,
    Vector,
    Quality,
}

impl Channel {
    fn name(self) -> &'static str {
        match self {
            Channel::Label => "label",
            Channel::Vector => "vector",
            Channel::Quality => "quality-only",
        }
    }
}

#[derive(Clone, Copy)]
struct Site {
    angle: f64,
    dist: f64,
    quality: f64,
    pos: Vec2,
}

fn sites() -> [Site; 3] {
    let mk = |angle: f64, dist: f64, quality: f64| Site {
        angle,
        dist,
        quality,
        pos: Vec2::new(angle.cos() * dist, angle.sin() * dist),
    };
    [
        mk(-0.55, 60.0, 0.45),  // A: near, poor
        mk(0.95, 115.0, 0.70),  // B: mid, middling
        mk(2.45, 175.0, 0.95),  // C: far, rich — the site a hive should choose
    ]
}

/// The dance's distance code: waggle-run length in ticks, 10 units
/// of distance per tick of run. (The corpse encoded quality here by
/// mistake; quality is carried by dance length / recruitment, below.)
fn run_ticks_for(dist: f64) -> f64 {
    dist / 10.0
}

/// Total dance length in ticks — how long the dancer keeps
/// repeating the run. Carries quality, as the corpse's did.
fn dance_ticks_for(quality: f64) -> u32 {
    10 + (26.0 * quality).round() as u32
}

/// Encode a site into a dance: (waggle angle, waggle-run ticks).
fn encode(site: &Site) -> (f64, f64) {
    (site.angle, run_ticks_for(site.dist))
}

/// Decode a measured dance back into a destination point.
fn decode(angle_meas: f64, run_meas: f64) -> Vec2 {
    let dist = (run_meas * 10.0).max(0.0);
    Vec2::new(angle_meas.cos() * dist, angle_meas.sin() * dist)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Observing,
    Scouting,
    Returning,
    Dancing,
    Flying,
    Searching,
}

#[derive(Clone)]
struct Bee {
    pos: Vec2,
    state: State,
    heading: f64,
    range_left: f64,
    found: Option<usize>,
    dance_site: usize,
    timer: u32,
    dest: Vec2,
    claimed: Option<usize>,
    direct: bool,
    last_foraged: Option<usize>,
}

impl Bee {
    fn new() -> Self {
        Self {
            pos: Vec2::zero(),
            state: State::Observing,
            heading: 0.0,
            range_left: 0.0,
            found: None,
            dance_site: 0,
            timer: 0,
            dest: Vec2::zero(),
            claimed: None,
            direct: false,
            last_foraged: None,
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Ledger {
    quorum_tick: Option<u32>,
    forages: [u64; 3],
    committed_end: [usize; 3],
    never_foraged: usize,
    recruits: u64,
    searches: u64,
    search_successes: u64,
    dances: u64,
    miss_sum: [f64; 3],
    miss_n: [u64; 3],
}

impl Ledger {
    fn mean_miss(&self, site: usize) -> f64 {
        if self.miss_n[site] == 0 {
            0.0
        } else {
            self.miss_sum[site] / self.miss_n[site] as f64
        }
    }
}

fn step_toward(bee: &mut Bee, target: Vec2) -> bool {
    let d = bee.pos.distance(target);
    if d <= SPEED {
        bee.pos = target;
        true
    } else {
        let dir = (target - bee.pos).normalize();
        bee.pos = bee.pos + dir * SPEED;
        false
    }
}

fn run(channel: Channel, ticks: u32) -> Ledger {
    let sites = sites();
    let mut scout_rng = SplitMix64(SEED ^ 0x5C07);
    let mut recruit_rng = SplitMix64(SEED ^ 0x2EC2);
    let mut noise_rng = SplitMix64(SEED ^ 0xDAAC);
    let mut search_rng = SplitMix64(SEED ^ 0x5EA2);

    let mut bees: Vec<Bee> = (0..BEES).map(|_| Bee::new()).collect();
    // The first fifteen bees scout on a fan of headings.
    for (i, bee) in bees.iter_mut().take(15).enumerate() {
        bee.state = State::Scouting;
        bee.heading = (i as f64) * TAU / 15.0 + scout_rng.range(-0.1, 0.1);
        bee.range_left = 340.0;
    }

    let mut led = Ledger::default();

    for tick in 0..ticks {
        // Active dances at the top of the tick (site indices).
        let dances: Vec<usize> = bees
            .iter()
            .filter(|b| b.state == State::Dancing)
            .map(|b| b.dance_site)
            .collect();

        for bi in 0..bees.len() {
            // Split borrows: pull the bee out by index each match arm.
            match bees[bi].state {
                State::Observing => {
                    if !dances.is_empty() && recruit_rng.unit() < 0.30 {
                        let s = dances[recruit_rng.pick(dances.len())];
                        if recruit_rng.unit() < 0.05 + 0.30 * sites[s].quality {
                            led.recruits += 1;
                            let (dest, direct) = match channel {
                                Channel::Label => (sites[s].pos, true),
                                Channel::Vector => {
                                    let (ang, run) = encode(&sites[s]);
                                    let ang_m = ang + noise_rng.gauss() * ANGLE_SIGMA;
                                    let run_m =
                                        run * (1.0 + noise_rng.gauss() * RUN_SIGMA);
                                    (decode(ang_m, run_m), false)
                                }
                                Channel::Quality => {
                                    let ang = noise_rng.range(0.0, TAU);
                                    let dist = noise_rng.range(40.0, 210.0);
                                    (Vec2::new(ang.cos() * dist, ang.sin() * dist), false)
                                }
                            };
                            let miss = dest.distance(sites[s].pos);
                            led.miss_sum[s] += miss;
                            led.miss_n[s] += 1;
                            let bee = &mut bees[bi];
                            bee.dest = dest;
                            bee.direct = direct;
                            bee.claimed = Some(s);
                            bee.state = State::Flying;
                        }
                    } else if recruit_rng.unit() < 0.004 {
                        let bee = &mut bees[bi];
                        bee.state = State::Scouting;
                        bee.heading = scout_rng.range(0.0, TAU);
                        bee.range_left = 340.0;
                    }
                }
                State::Scouting => {
                    let mut found = None;
                    {
                        let bee = &mut bees[bi];
                        bee.heading += scout_rng.gauss() * 0.05;
                        bee.pos = Vec2::new(
                            bee.pos.x + bee.heading.cos() * SPEED,
                            bee.pos.y + bee.heading.sin() * SPEED,
                        );
                        bee.range_left -= SPEED;
                        for (si, site) in sites.iter().enumerate() {
                            if bee.pos.distance(site.pos) <= SIGHT {
                                found = Some(si);
                                break;
                            }
                        }
                        if found.is_none() && bee.range_left <= 0.0 {
                            bee.state = State::Returning;
                            bee.found = None;
                        }
                    }
                    if let Some(si) = found {
                        let bee = &mut bees[bi];
                        bee.state = State::Returning;
                        bee.found = Some(si);
                    }
                }
                State::Returning => {
                    let arrived = step_toward(&mut bees[bi], Vec2::zero());
                    if arrived {
                        let bee = &mut bees[bi];
                        match bee.found {
                            Some(s) => {
                                bee.dance_site = s;
                                bee.timer = dance_ticks_for(sites[s].quality);
                                bee.state = State::Dancing;
                                led.dances += 1;
                            }
                            None => {
                                bee.state = State::Observing;
                            }
                        }
                    }
                }
                State::Dancing => {
                    let s = bees[bi].dance_site;
                    let bee = &mut bees[bi];
                    bee.timer = bee.timer.saturating_sub(1);
                    if bee.timer == 0 {
                        // The corpse's own post-dance coin, restored:
                        // the dancer re-visits its site with
                        // probability = quality, else goes back to
                        // watching other dances. (First tuning pass
                        // dropped this coin and the hive locked onto
                        // the nearest site in every channel — cycle
                        // frequency beat quality outright. Recorded
                        // in the README.)
                        if recruit_rng.unit() < sites[s].quality {
                            bee.dest = sites[s].pos;
                            bee.direct = true;
                            bee.claimed = Some(s);
                            bee.state = State::Flying;
                        } else {
                            bee.state = State::Observing;
                        }
                    }
                }
                State::Flying => {
                    if bees[bi].direct {
                        let claimed = bees[bi].claimed;
                        let arrived = {
                            let dest = bees[bi].dest;
                            step_toward(&mut bees[bi], dest)
                        };
                        // Direct flyers see the site from SIGHT away.
                        let near = bees[bi].pos.distance(bees[bi].dest) <= SIGHT;
                        if arrived || near {
                            if let Some(s) = claimed {
                                led.forages[s] += 1;
                                let bee = &mut bees[bi];
                                bee.last_foraged = Some(s);
                                bee.state = State::Returning;
                                bee.found = Some(s);
                            }
                        }
                    } else {
                        let dest = bees[bi].dest;
                        let arrived = step_toward(&mut bees[bi], dest);
                        if arrived {
                            let bee = &mut bees[bi];
                            bee.state = State::Searching;
                            bee.timer = SEARCH_TICKS;
                            led.searches += 1;
                        }
                    }
                }
                State::Searching => {
                    let mut found = None;
                    {
                        let bee = &mut bees[bi];
                        let ang = search_rng.range(0.0, TAU);
                        let mut step = Vec2::new(ang.cos(), ang.sin()) * (SPEED * 0.7);
                        // Stay near the decoded point: it is the
                        // bee's whole belief about where food is.
                        if bee.pos.distance(bee.dest) > 30.0 {
                            step = (bee.dest - bee.pos).normalize() * (SPEED * 0.7);
                        }
                        bee.pos = bee.pos + step;
                        bee.timer = bee.timer.saturating_sub(1);
                        for (si, site) in sites.iter().enumerate() {
                            if bee.pos.distance(site.pos) <= FIND_RADIUS {
                                found = Some(si);
                                break;
                            }
                        }
                        if found.is_none() && bee.timer == 0 {
                            bee.state = State::Returning;
                            bee.found = None;
                        }
                    }
                    if let Some(si) = found {
                        led.forages[si] += 1;
                        led.search_successes += 1;
                        let bee = &mut bees[bi];
                        bee.last_foraged = Some(si);
                        bee.state = State::Returning;
                        bee.found = Some(si);
                    }
                }
            }
        }

        if led.quorum_tick.is_none() {
            let committed_c = bees.iter().filter(|b| b.last_foraged == Some(2)).count();
            if committed_c >= QUORUM {
                led.quorum_tick = Some(tick);
            }
        }
    }

    for bee in &bees {
        match bee.last_foraged {
            Some(s) => led.committed_end[s] += 1,
            None => led.never_foraged += 1,
        }
    }
    led
}

fn print_ledger(channel: Channel, led: &Ledger) {
    println!("channel {}:", channel.name());
    match led.quorum_tick {
        Some(t) => println!(
            "  quorum tick ({} bees last-foraged far-rich C): {}",
            QUORUM, t
        ),
        None => println!(
            "  quorum tick ({} bees last-foraged far-rich C): none in {}",
            QUORUM, TICKS
        ),
    }
    println!(
        "  forages by site A/B/C: {} / {} / {}",
        led.forages[0], led.forages[1], led.forages[2]
    );
    println!(
        "  committed at end A/B/C/never: {} / {} / {} / {}",
        led.committed_end[0], led.committed_end[1], led.committed_end[2], led.never_foraged
    );
    println!(
        "  recruits launched: {}, searches: {}, search successes: {}, dances: {}",
        led.recruits, led.searches, led.search_successes, led.dances
    );
    println!(
        "  mean decode miss by claimed site A/B/C: {:.2} / {:.2} / {:.2}",
        led.mean_miss(0),
        led.mean_miss(1),
        led.mean_miss(2)
    );
}

fn main() -> Result<()> {
    // --headless is handled before anything else; this specimen has
    // no TUI at all (its corpse's window was the thing that killed
    // it), so every route runs the ledger.
    let _headless = std::env::args().any(|a| a == "--headless");
    println!(
        "waggle-locus headless: the dance as a channel ({} bees, {} ticks, 3 sites)",
        BEES, TICKS
    );
    println!("  sites: A near/poor (d=60, q=0.45), B mid (d=115, q=0.70), C far/rich (d=175, q=0.95)");
    for channel in [Channel::Label, Channel::Vector, Channel::Quality] {
        let led = run(channel, TICKS);
        print_ledger(channel, &led);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noiseless_decode_recovers_the_site_exactly() {
        for site in sites() {
            let (ang, run) = encode(&site);
            let back = decode(ang, run);
            assert!(back.distance(site.pos) < 1e-9, "site at {:?}", site.pos);
        }
    }

    #[test]
    fn the_run_carries_distance_and_the_dance_carries_quality() {
        // Distance code is monotonic in distance (the corpse's map
        // had quality here instead).
        assert!(run_ticks_for(60.0) < run_ticks_for(115.0));
        assert!(run_ticks_for(115.0) < run_ticks_for(175.0));
        // Dance length is monotonic in quality.
        assert!(dance_ticks_for(0.45) < dance_ticks_for(0.70));
        assert!(dance_ticks_for(0.70) < dance_ticks_for(0.95));
    }

    #[test]
    fn label_channel_never_misses() {
        let led = run(Channel::Label, 2000);
        for s in 0..3 {
            assert_eq!(led.mean_miss(s), 0.0);
        }
        assert!(led.recruits > 0, "label hive must recruit");
    }

    #[test]
    fn decode_miss_grows_with_distance() {
        let sites = sites();
        let mut rng = SplitMix64(0xABCDEF);
        let mut means = [0.0f64; 3];
        for (i, site) in sites.iter().enumerate() {
            let (ang, run) = encode(site);
            let mut sum = 0.0;
            for _ in 0..400 {
                let p = decode(
                    ang + rng.gauss() * ANGLE_SIGMA,
                    run * (1.0 + rng.gauss() * RUN_SIGMA),
                );
                sum += p.distance(site.pos);
            }
            means[i] = sum / 400.0;
        }
        assert!(
            means[2] > means[0] * 1.5,
            "far miss {} should exceed near miss {} by 1.5x",
            means[2],
            means[0]
        );
    }

    #[test]
    fn runs_are_deterministic() {
        let a = run(Channel::Vector, 1500);
        let b = run(Channel::Vector, 1500);
        assert_eq!(a.forages, b.forages);
        assert_eq!(a.quorum_tick, b.quorum_tick);
        assert_eq!(a.dances, b.dances);
    }

    #[test]
    fn vector_channel_beats_blind_search_on_miss() {
        let v = run(Channel::Vector, 2000);
        let q = run(Channel::Quality, 2000);
        let v_miss: f64 = (0..3).map(|s| v.mean_miss(s)).sum();
        let q_miss: f64 = (0..3).map(|s| q.mean_miss(s)).sum();
        assert!(v_miss < q_miss, "vector {} vs blind {}", v_miss, q_miss);
    }
}
