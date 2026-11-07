//! scarecrow-intentions — The Scarecrow 🌾 plants Harvest's
//! interruption equivalence in `waggle-locus` (The Cartographer
//! of Lost Things 🗺️): a hive whose bees carry their work in
//! transit.
//!
//! The host hive's state looks small on paper: sixty bees. But a
//! bee is not only where it stands. At any tick some bees are
//! mid-dance, carrying a site and a timer that is the dance's
//! remaining length. Some are mid-flight, carrying a decoded
//! destination, the site they claimed, and whether they fly
//! direct. Some are mid-search, carrying a destination and the
//! ticks they have left to look. Those intentions are state —
//! they exist in no snapshot of positions, and a winter that
//! takes the hive mid-season will meet them first.
//!
//! This planting runs the host's honest channel (Vector: the
//! dance carries angle and waggle-run only, decoded through
//! noise) uninterrupted, then stops it at the season's midpoint
//! (tick 3,000 of 6,000) and resumes it three ways:
//!
//! - Whole:        bees entire (positions *and* intentions),
//!                 all four purpose streams at their positions,
//!                 the ledger, the tick. The seam the principle
//!                 promises is invisible.
//! - Intentions forgotten: positions and last-foraged kept,
//!                 every bee set back to Observing — dancers
//!                 stopped mid-dance, flyers' destinations and
//!                 claims dropped, searchers' timers gone. The
//!                 streams and the ledger are kept whole, so the
//!                 only thing the winter took is what the bees
//!                 were *doing*.
//! - Streams forgotten: bees entire, ledger and tick kept, but
//!                 the four purpose streams rewound to their
//!                 seeds — the scarecrow-journal lesson in a new
//!                 field: a run's draws are state too.
//!
//! The host's machinery, constants, streams, and update order
//! are copied faithfully from `waggle-locus`; the hive is
//! refactored only enough to step one tick at a time and to be
//! written down. Tests pin this crate's uninterrupted Vector
//! ledger to the host's published one.

use anyhow::Result;
use locus::Vec2;
use std::f64::consts::TAU;

const BEES: usize = 60;
const TICKS: u32 = 6000;
const CHECKPOINT_TICK: u32 = 3000;
const SPEED: f64 = 3.0;
const SIGHT: f64 = 22.0;
const FIND_RADIUS: f64 = 12.0;
const SEARCH_TICKS: u32 = 120;
const QUORUM: usize = BEES / 2;
const ANGLE_SIGMA: f64 = 0.085;
const RUN_SIGMA: f64 = 0.07;
const SEED: u64 = 0xBE35_1E00_0000_0001_u64;

/// Packed size of one bee written whole: pos (16) + state (1) +
/// heading (8) + range_left (8) + found (1) + dance_site (1) +
/// timer (4) + dest (16) + claimed (1) + direct (1) +
/// last_foraged (1) = 58 bytes.
const BEE_WHOLE_BYTES: usize = 58;
/// Packed size of one bee in the intentions-forgotten writing:
/// pos (16) + last_foraged (1) = 17 bytes. Everything else is
/// reset, so nothing else is written.
const BEE_BARE_BYTES: usize = 17;
/// Streams (4 × u64) + tick (u32) + ledger (forages 24 +
/// committed 24 + never 8 + recruits 8 + searches 8 + successes
/// 8 + dances 8 + miss_sum 24 + miss_n 24 + quorum 5 = 141).
const PAPER_TAIL_BYTES: usize = 32 + 4 + 141;

/// Deterministic SplitMix64 — one stream per purpose, the
/// host's own design, copied unchanged.
#[derive(Clone)]
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
        mk(-0.55, 60.0, 0.45),
        mk(0.95, 115.0, 0.70),
        mk(2.45, 175.0, 0.95),
    ]
}

fn run_ticks_for(dist: f64) -> f64 {
    dist / 10.0
}

fn dance_ticks_for(quality: f64) -> u32 {
    10 + (26.0 * quality).round() as u32
}

fn encode(site: &Site) -> (f64, f64) {
    (site.angle, run_ticks_for(site.dist))
}

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

impl State {
    fn disc(self) -> u64 {
        match self {
            State::Observing => 0,
            State::Scouting => 1,
            State::Returning => 2,
            State::Dancing => 3,
            State::Flying => 4,
            State::Searching => 5,
        }
    }
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
    /// The bare writing: where the bee stood and what it last
    /// ate. Nothing it was doing survives.
    fn bare(&self) -> Self {
        Self {
            pos: self.pos,
            last_foraged: self.last_foraged,
            ..Bee::new()
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
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
    /// Count of ledger fields that differ from another ledger
    /// (each array counts per element; quorum counts once;
    /// miss means compared at 1e-9).
    fn fields_differing(&self, other: &Ledger) -> usize {
        let mut n = 0;
        if self.quorum_tick != other.quorum_tick {
            n += 1;
        }
        for s in 0..3 {
            if self.forages[s] != other.forages[s] {
                n += 1;
            }
            if self.committed_end[s] != other.committed_end[s] {
                n += 1;
            }
            if (self.mean_miss(s) - other.mean_miss(s)).abs() > 1e-9 {
                n += 1;
            }
        }
        if self.never_foraged != other.never_foraged {
            n += 1;
        }
        if self.recruits != other.recruits {
            n += 1;
        }
        if self.searches != other.searches {
            n += 1;
        }
        if self.search_successes != other.search_successes {
            n += 1;
        }
        if self.dances != other.dances {
            n += 1;
        }
        n
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

/// The hive, stepped one tick at a time — the host's `run`
/// loop with its state lifted into a struct so a run can be
/// written down at a tick boundary and taken up again.
struct Hive {
    channel: Channel,
    bees: Vec<Bee>,
    scout_rng: SplitMix64,
    recruit_rng: SplitMix64,
    noise_rng: SplitMix64,
    search_rng: SplitMix64,
    led: Ledger,
    tick: u32,
}

impl Hive {
    fn new(channel: Channel) -> Self {
        let mut scout_rng = SplitMix64(SEED ^ 0x5C07);
        let mut bees: Vec<Bee> = (0..BEES).map(|_| Bee::new()).collect();
        for (i, bee) in bees.iter_mut().take(15).enumerate() {
            bee.state = State::Scouting;
            bee.heading = (i as f64) * TAU / 15.0 + scout_rng.range(-0.1, 0.1);
            bee.range_left = 340.0;
        }
        Self {
            channel,
            bees,
            scout_rng,
            recruit_rng: SplitMix64(SEED ^ 0x2EC2),
            noise_rng: SplitMix64(SEED ^ 0xDAAC),
            search_rng: SplitMix64(SEED ^ 0x5EA2),
            led: Ledger::default(),
            tick: 0,
        }
    }

    fn step(&mut self) {
        let sites = sites();
        let channel = self.channel;
        let tick = self.tick;
        let dances: Vec<usize> = self
            .bees
            .iter()
            .filter(|b| b.state == State::Dancing)
            .map(|b| b.dance_site)
            .collect();

        for bi in 0..self.bees.len() {
            match self.bees[bi].state {
                State::Observing => {
                    if !dances.is_empty() && self.recruit_rng.unit() < 0.30 {
                        let s = dances[self.recruit_rng.pick(dances.len())];
                        if self.recruit_rng.unit() < 0.05 + 0.30 * sites[s].quality {
                            self.led.recruits += 1;
                            let (dest, direct) = match channel {
                                Channel::Label => (sites[s].pos, true),
                                Channel::Vector => {
                                    let (ang, run) = encode(&sites[s]);
                                    let ang_m = ang + self.noise_rng.gauss() * ANGLE_SIGMA;
                                    let run_m = run * (1.0 + self.noise_rng.gauss() * RUN_SIGMA);
                                    (decode(ang_m, run_m), false)
                                }
                                Channel::Quality => {
                                    let ang = self.noise_rng.range(0.0, TAU);
                                    let dist = self.noise_rng.range(40.0, 210.0);
                                    (Vec2::new(ang.cos() * dist, ang.sin() * dist), false)
                                }
                            };
                            let miss = dest.distance(sites[s].pos);
                            self.led.miss_sum[s] += miss;
                            self.led.miss_n[s] += 1;
                            let bee = &mut self.bees[bi];
                            bee.dest = dest;
                            bee.direct = direct;
                            bee.claimed = Some(s);
                            bee.state = State::Flying;
                        }
                    } else if self.recruit_rng.unit() < 0.004 {
                        let heading = self.scout_rng.range(0.0, TAU);
                        let bee = &mut self.bees[bi];
                        bee.state = State::Scouting;
                        bee.heading = heading;
                        bee.range_left = 340.0;
                    }
                }
                State::Scouting => {
                    let mut found = None;
                    {
                        let bee = &mut self.bees[bi];
                        bee.heading += self.scout_rng.gauss() * 0.05;
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
                        let bee = &mut self.bees[bi];
                        bee.state = State::Returning;
                        bee.found = Some(si);
                    }
                }
                State::Returning => {
                    let arrived = step_toward(&mut self.bees[bi], Vec2::zero());
                    if arrived {
                        let bee = &mut self.bees[bi];
                        match bee.found {
                            Some(s) => {
                                bee.dance_site = s;
                                bee.timer = dance_ticks_for(sites[s].quality);
                                bee.state = State::Dancing;
                                self.led.dances += 1;
                            }
                            None => {
                                bee.state = State::Observing;
                            }
                        }
                    }
                }
                State::Dancing => {
                    let s = self.bees[bi].dance_site;
                    let bee = &mut self.bees[bi];
                    bee.timer = bee.timer.saturating_sub(1);
                    if bee.timer == 0 {
                        if self.recruit_rng.unit() < sites[s].quality {
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
                    if self.bees[bi].direct {
                        let claimed = self.bees[bi].claimed;
                        let arrived = {
                            let dest = self.bees[bi].dest;
                            step_toward(&mut self.bees[bi], dest)
                        };
                        let near = self.bees[bi].pos.distance(self.bees[bi].dest) <= SIGHT;
                        if arrived || near {
                            if let Some(s) = claimed {
                                self.led.forages[s] += 1;
                                let bee = &mut self.bees[bi];
                                bee.last_foraged = Some(s);
                                bee.state = State::Returning;
                                bee.found = Some(s);
                            }
                        }
                    } else {
                        let dest = self.bees[bi].dest;
                        let arrived = step_toward(&mut self.bees[bi], dest);
                        if arrived {
                            let bee = &mut self.bees[bi];
                            bee.state = State::Searching;
                            bee.timer = SEARCH_TICKS;
                            self.led.searches += 1;
                        }
                    }
                }
                State::Searching => {
                    let mut found = None;
                    {
                        let bee = &mut self.bees[bi];
                        let ang = self.search_rng.range(0.0, TAU);
                        let mut step = Vec2::new(ang.cos(), ang.sin()) * (SPEED * 0.7);
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
                        self.led.forages[si] += 1;
                        self.led.search_successes += 1;
                        let bee = &mut self.bees[bi];
                        bee.last_foraged = Some(si);
                        bee.state = State::Returning;
                        bee.found = Some(si);
                    }
                }
            }
        }

        if self.led.quorum_tick.is_none() {
            let committed_c = self
                .bees
                .iter()
                .filter(|b| b.last_foraged == Some(2))
                .count();
            if committed_c >= QUORUM {
                self.led.quorum_tick = Some(tick);
            }
        }
        self.tick += 1;
    }

    fn finish(&mut self) {
        for bee in &self.bees {
            match bee.last_foraged {
                Some(s) => self.led.committed_end[s] += 1,
                None => self.led.never_foraged += 1,
            }
        }
    }

    fn run_to(&mut self, end: u32, checksums: &mut Vec<u64>) {
        while self.tick < end {
            self.step();
            checksums.push(hive_checksum(&self.bees));
        }
    }

    /// Write the hive down whole.
    fn snapshot(&self) -> Snapshot {
        Snapshot {
            bees: self.bees.clone(),
            streams: [
                self.scout_rng.0,
                self.recruit_rng.0,
                self.noise_rng.0,
                self.search_rng.0,
            ],
            led: self.led.clone(),
            tick: self.tick,
        }
    }

    fn restore(channel: Channel, snap: &Snapshot) -> Self {
        Self {
            channel,
            bees: snap.bees.clone(),
            scout_rng: SplitMix64(snap.streams[0]),
            recruit_rng: SplitMix64(snap.streams[1]),
            noise_rng: SplitMix64(snap.streams[2]),
            search_rng: SplitMix64(snap.streams[3]),
            led: snap.led.clone(),
            tick: snap.tick,
        }
    }
}

#[derive(Clone)]
struct Snapshot {
    bees: Vec<Bee>,
    streams: [u64; 4],
    led: Ledger,
    tick: u32,
}

impl Snapshot {
    fn whole_bytes(&self) -> usize {
        self.bees.len() * BEE_WHOLE_BYTES + PAPER_TAIL_BYTES
    }
    fn bare_bytes(&self) -> usize {
        self.bees.len() * BEE_BARE_BYTES + PAPER_TAIL_BYTES
    }
    /// The intentions-forgotten writing: same streams, same
    /// ledger, same tick — bare bees.
    fn intentions_forgotten(&self) -> Snapshot {
        Snapshot {
            bees: self.bees.iter().map(|b| b.bare()).collect(),
            streams: self.streams,
            led: self.led.clone(),
            tick: self.tick,
        }
    }
    /// The streams-forgotten writing: whole bees, whole ledger,
    /// streams rewound to their seeds. Same byte count as whole.
    fn streams_forgotten(&self) -> Snapshot {
        Snapshot {
            bees: self.bees.clone(),
            streams: [
                SEED ^ 0x5C07,
                SEED ^ 0x2EC2,
                SEED ^ 0xDAAC,
                SEED ^ 0x5EA2,
            ],
            led: self.led.clone(),
            tick: self.tick,
        }
    }
    fn in_transit_census(&self) -> [usize; 6] {
        let mut c = [0usize; 6];
        for b in &self.bees {
            c[b.state.disc() as usize] += 1;
        }
        c
    }
}

/// FNV-1a over every bee's position bits and state, in order.
fn hive_checksum(bees: &[Bee]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bees {
        for v in [b.pos.x.to_bits(), b.pos.y.to_bits(), b.state.disc()] {
            for byte in v.to_le_bytes() {
                h ^= byte as u64;
                h = h.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
    }
    h
}

fn first_divergence(a: &[u64], b: &[u64]) -> Option<usize> {
    a.iter().zip(b.iter()).position(|(x, y)| x != y)
}

fn bees_differing(a: &[Bee], b: &[Bee]) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|(x, y)| {
            x.pos.x.to_bits() != y.pos.x.to_bits()
                || x.pos.y.to_bits() != y.pos.y.to_bits()
                || x.state != y.state
                || x.last_foraged != y.last_foraged
        })
        .count()
}

fn run_plain(channel: Channel, ticks: u32) -> Ledger {
    let mut hive = Hive::new(channel);
    let mut sums = Vec::new();
    hive.run_to(ticks, &mut sums);
    hive.finish();
    hive.led
}

struct ResumeResult {
    led: Ledger,
    #[allow(dead_code)] // read by the tests' whole-resume check
    bees: Vec<Bee>,
    first_div: Option<usize>,
    fields_differing: usize,
    bees_differing: usize,
}

fn survey() -> (Ledger, Vec<Bee>, Snapshot, [ResumeResult; 3]) {
    // Live, uninterrupted, checksums per tick.
    let mut live = Hive::new(Channel::Vector);
    let mut live_sums: Vec<u64> = Vec::new();
    live.run_to(CHECKPOINT_TICK, &mut live_sums);
    let snap = live.snapshot();
    live.run_to(TICKS, &mut live_sums);
    let live_bees = live.bees.clone();
    live.finish();
    let live_led = live.led.clone();

    let mut results = Vec::new();
    for variant in [
        snap.clone(),
        snap.intentions_forgotten(),
        snap.streams_forgotten(),
    ] {
        let mut hive = Hive::restore(Channel::Vector, &variant);
        let mut sums: Vec<u64> = Vec::new();
        hive.run_to(TICKS, &mut sums);
        let div = first_divergence(&live_sums[CHECKPOINT_TICK as usize..], &sums)
            .map(|i| CHECKPOINT_TICK as usize + i);
        let bees = hive.bees.clone();
        hive.finish();
        let led = hive.led.clone();
        results.push(ResumeResult {
            fields_differing: led.fields_differing(&live_led),
            bees_differing: bees_differing(&bees, &live_bees),
            led,
            bees,
            first_div: div,
        });
    }
    (
        live_led,
        live_bees,
        snap,
        results.try_into().ok().unwrap(),
    )
}

fn print_ledger(tag: &str, led: &Ledger) {
    println!(
        "{}: quorum {:?}, forages A/B/C {}/{}/{}, committed A/B/C/never {}/{}/{}/{}, recruits {}, searches {}, successes {}, dances {}",
        tag,
        led.quorum_tick,
        led.forages[0],
        led.forages[1],
        led.forages[2],
        led.committed_end[0],
        led.committed_end[1],
        led.committed_end[2],
        led.never_foraged,
        led.recruits,
        led.searches,
        led.search_successes,
        led.dances
    );
    println!(
        "{}: mean decode miss A/B/C {:.2}/{:.2}/{:.2}",
        tag,
        led.mean_miss(0),
        led.mean_miss(1),
        led.mean_miss(2)
    );
}

fn headless() {
    println!("scarecrow-intentions headless ledger");
    println!(
        "host: waggle-locus vector channel ({} bees, {} ticks, sites A d=60 q=0.45 / B d=115 q=0.70 / C d=175 q=0.95)",
        BEES, TICKS
    );
    println!("principle: interruption equivalence (Harvest) — stop at tick {}, write the hive down, resume; the final ledger must be the uninterrupted run's, field for field", CHECKPOINT_TICK);

    // Host anchors on all three channels, plain runs.
    for channel in [Channel::Label, Channel::Vector, Channel::Quality] {
        let led = run_plain(channel, TICKS);
        print_ledger(&format!("host {}", channel.name()), &led);
    }

    let (live, _live_bees, snap, results) = survey();
    print_ledger("live vector", &live);
    let census = snap.in_transit_census();
    println!(
        "checkpoint at tick {}: bees by state observing/scouting/returning/dancing/flying/searching = {}/{}/{}/{}/{}/{} ({} carrying work in transit)",
        snap.tick,
        census[0], census[1], census[2], census[3], census[4], census[5],
        census[1] + census[2] + census[3] + census[4] + census[5]
    );
    println!(
        "checkpoint at tick {}: dances so far {}, recruits {}, searches {}, forages A/B/C {}/{}/{}",
        snap.tick, snap.led.dances, snap.led.recruits, snap.led.searches,
        snap.led.forages[0], snap.led.forages[1], snap.led.forages[2]
    );
    println!(
        "writing sizes: whole {} bytes ({} bees x {} + {} tail); intentions-forgotten {} bytes ({} bees x {} + {} tail); streams-forgotten {} bytes (same size as whole — the wrong writing is not the cheap one)",
        snap.whole_bytes(), snap.bees.len(), BEE_WHOLE_BYTES, PAPER_TAIL_BYTES,
        snap.bare_bytes(), snap.bees.len(), BEE_BARE_BYTES, PAPER_TAIL_BYTES,
        snap.whole_bytes()
    );

    let names = ["whole", "intentions-forgotten", "streams-forgotten"];
    for (name, r) in names.iter().zip(results.iter()) {
        print_ledger(&format!("resumed {}", name), &r.led);
        println!(
            "resumed {}: first divergent tick {:?}, ledger fields differing {}, bees differing at end {}/{}",
            name, r.first_div, r.fields_differing, r.bees_differing, BEES
        );
    }

    let b1 = live.quorum_tick == Some(1043)
        && live.forages == [446, 174, 2065]
        && live.committed_end == [0, 0, 60]
        && live.never_foraged == 0
        && live.searches == 495
        && live.search_successes == 421
        && live.dances == 2666;
    let whole = &results[0];
    let intentions = &results[1];
    let streams = &results[2];
    let p1 = whole.first_div.is_none() && whole.fields_differing == 0 && whole.bees_differing == 0;
    let p2 = intentions.fields_differing >= 3;
    let p3 = intentions.first_div == Some(CHECKPOINT_TICK as usize);
    let p4 = streams.first_div.is_some() && streams.fields_differing >= 1;
    let p5 = snap.bare_bytes() < snap.whole_bytes();
    println!(
        "verdicts: B1 host anchor (vector) exact: {}; P1 whole resume exact (no divergent tick, 0 fields, 0 bees): {}; P2 intentions-forgotten diverges in the ledger ({} fields >= 3): {}; P3 intentions-forgotten parts at the first resumed tick (div {:?} == {}): {}; P4 streams-forgotten diverges (div {:?}, {} fields): {}; P5 the bare writing is cheaper ({} < {} bytes) and wrong: {}",
        b1, p1, intentions.fields_differing, p2, intentions.first_div, CHECKPOINT_TICK, p3,
        streams.first_div, streams.fields_differing, p4,
        snap.bare_bytes(), snap.whole_bytes(), p5
    );
}

fn main() -> Result<()> {
    // --headless is handled BEFORE anything else; this specimen
    // has no TUI at all.
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("scarecrow-intentions: run with --headless for the ledger (no TUI; the field is listened to, not looked at).");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_anchor_vector_exact() {
        // waggle-locus's published vector ledger.
        let led = run_plain(Channel::Vector, TICKS);
        assert_eq!(led.quorum_tick, Some(1043));
        assert_eq!(led.forages, [446, 174, 2065]);
        assert_eq!(led.committed_end, [0, 0, 60]);
        assert_eq!(led.never_foraged, 0);
        assert_eq!(led.searches, 495);
        assert_eq!(led.search_successes, 421);
        assert_eq!(led.dances, 2666);
        assert!((led.mean_miss(0) - 5.83).abs() < 0.01, "miss A {}", led.mean_miss(0));
        assert!((led.mean_miss(1) - 11.26).abs() < 0.01, "miss B {}", led.mean_miss(1));
        assert!((led.mean_miss(2) - 15.61).abs() < 0.01, "miss C {}", led.mean_miss(2));
    }

    #[test]
    fn whole_checkpoint_resumes_exact() {
        let (live, live_bees, _snap, results) = survey();
        let whole = &results[0];
        assert_eq!(whole.first_div, None);
        assert_eq!(whole.fields_differing, 0);
        assert_eq!(whole.bees_differing, 0);
        assert_eq!(whole.led, live);
        assert_eq!(whole.bees.len(), live_bees.len());
    }

    #[test]
    fn intentions_forgotten_parts_at_once() {
        let (_live, _bees, _snap, results) = survey();
        let intentions = &results[1];
        assert_eq!(intentions.first_div, Some(CHECKPOINT_TICK as usize));
        assert!(intentions.fields_differing >= 3, "fields {}", intentions.fields_differing);
        assert!(intentions.bees_differing > 0);
    }

    #[test]
    fn streams_forgotten_diverges() {
        let (_live, _bees, _snap, results) = survey();
        let streams = &results[2];
        assert!(streams.first_div.is_some());
        assert!(streams.fields_differing >= 1, "fields {}", streams.fields_differing);
    }

    #[test]
    fn checkpoint_carries_in_transit_bees() {
        // At the midpoint the hive must actually be carrying
        // work in transit, or the survey prices nothing.
        let mut hive = Hive::new(Channel::Vector);
        let mut sums = Vec::new();
        hive.run_to(CHECKPOINT_TICK, &mut sums);
        let snap = hive.snapshot();
        let census = snap.in_transit_census();
        let in_transit: usize = census[1..].iter().sum();
        assert!(in_transit > 0, "census {:?}", census);
        assert!(snap.bare_bytes() < snap.whole_bytes());
    }

    #[test]
    fn bare_writing_keeps_place_and_drops_purpose() {
        let mut bee = Bee::new();
        bee.pos = Vec2::new(3.0, 4.0);
        bee.state = State::Dancing;
        bee.dance_site = 2;
        bee.timer = 17;
        bee.last_foraged = Some(1);
        let bare = bee.bare();
        assert_eq!(bare.pos, bee.pos);
        assert_eq!(bare.last_foraged, Some(1));
        assert_eq!(bare.state, State::Observing);
        assert_eq!(bare.timer, 0);
    }
}
