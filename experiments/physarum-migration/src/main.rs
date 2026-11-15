//! physarum-migration 🗺️
//!
//! The Cartographer's twenty-third survey: the migrating voice,
//! dated - ghost visits per city per 300-tick slice across the
//! past window, so the haunting's change of tenure has a date on
//! it, not just a before and an after.
//!
//! `physarum-pastseason` (the Cartographer's twenty-second
//! survey) extended the season to 9,600 ticks and found the
//! haunting's composition unsettled: city 8, the last city to
//! fall, collected 7,346 ghost visits inside the past window
//! (ticks 7,200..9,600) - more than twice its gated lifetime's
//! 3,097 - and then, in the final 300 ticks, received none at
//! all, while loud city 1, the disk's first fall (tick 591),
//! ended the season as its loudest voice at 383. Pastseason's
//! closing seed, taken up verbatim: *the migrating voice, dated:
//! city 8's 7,346 past-window ghost visits all arrived before
//! tick 9,300 and its final-300 count is 0: a finer instrument
//! (ghost visits per city per 300-tick slice across the past
//! window) could date the migration - did city 8's collection
//! fade gradually or break at a tick, and did city 1's apron
//! rise as city 8's fell, or only after?*
//!
//! This cross carries pastseason's machinery whole - Mobius
//! movement and sensing, diffusion on the projection with the
//! void drinking at the rim, the mold's boundary rule, the
//! opposed channels stone for stone, the same seed, the same
//! nine cities, the same fifteen rooms at the same five lengths -
//! and adds exactly one instrument: in every room long enough to
//! reach it, cumulative per-city ghost visits are snapshotted at
//! ticks 7,200, 7,500, ..., 9,600, and the eight differences are
//! the past window's slices. The slices are exact by construction
//! (they sum to the window counts the parent priced by
//! difference, and the eighth slice is the final-300 ledger
//! itself), so the migration's shape - fade or break, overlap or
//! succession - is read off a ledger, not an impression.
//!
//! Predictions, declared before the first run (see README):
//!   B1 the flat 1,200 room reproduces physarum-opposed's pinned
//!      ledger exact. B2 the curved rooms reproduce their parents'
//!      pinned ledgers exact at all four earlier lengths, and the
//!      9,600 rooms reproduce physarum-pastseason's (past depleted
//!      ghost 37,282 at 38.9% loud, city 8 window ghost 7,346,
//!      final-300 total 771 as city 1 383 / city 5 373 / city 6 15).
//!   P1 the instrument is exact: per city, the eight slices sum
//!      to the past window's ghost count, the slice total is
//!      9,976, and slice 7 equals the final-300 ledger per city.
//!   P2 city 8's collection breaks rather than fades: its largest
//!      single slice holds at least 30% of its 7,346 window total.
//!   P3 the break is late and total: city 8's slice 6 (ticks
//!      9,000..9,300) is positive and its slice 7 is exactly 0.
//!   P4 succession overlaps the fall: city 1's largest slice is
//!      slice 6 or slice 7 - the first-fallen's collection is a
//!      late-window phenomenon, not an even tenure.
//!   P5 city 8 holds the early window: city 8 is the argmax city
//!      in slice 0 (ticks 7,200..7,500).
//!   P6 the tenure passes loud: in slice 7 the two largest counts
//!      are city 1's and city 5's (both loud), as the final-300
//!      ledger already shows - the slices' new evidence is the
//!      path, not the endpoint.
//!
//! The falsification condition is in the README: a flat city-8
//! profile (no slice above 20% of its window total) would make
//! the migration a fade the endpoint concealed; city 1 rising
//! only in slice 7, after city 8 is already silent, would make
//! it a succession with a gap, not a passing of the tenure.

use std::f32::consts::PI;

// ---- the corpse's constants, verbatim (flat ground) ----
const SENSOR_ANGLE: f32 = PI / 4.0; // 45 degrees
const SENSOR_DIST: f32 = 9.0;
const ROTATION_ANGLE: f32 = PI / 8.0; // 22.5 degrees
const AGENT_SPEED: f32 = 2.0;
const DEPOSIT: f32 = 0.5;
const DECAY: f32 = 0.95;
const CITY_RADIUS: isize = 3;

// ---- the parent's bookkeeping, planted here ----
const STOCK0: f32 = 25.0; // opening stock per city
const EAT: f32 = 0.004; // stock per feeding agent per tick
const FEED_RADIUS2: f32 = 12.25; // 3.5^2, the stamped disc's apron

// ---- the opposed channels, the parent's numbers verbatim ----
const LOUD_STAMP: f32 = 1.0; // quality's rich stamp, the corpse's own
const QUIET_STAMP: f32 = 0.40; // quality's plain stamp
const EAT_LOUD: f32 = EAT; // slow mouth: the uniform rate 0.004
const EAT_QUIET: f32 = 0.010; // quick mouth: appetite's rich rate

// ---- the mold's ground: the Poincaré disk ----
// Sensor and step lengths are hyperbolic (Möbius) distances, scaled
// from the mold's own 0.05 / 0.01 to this lineage's grid: at the disk's
// centre a Möbius step of δ moves δ in chart coordinates, and the flat
// ground's sensor/step are 9 and 2 cells on a 96-wide chart whose
// half-width is 48 cells — 0.1875 and 0.0417 chart units. The values
// below (0.15 / 0.035) keep the curved sensor a little shorter than
// the flat one, because a hyperbolic step of fixed length reaches
// *less* chart distance everywhere off-centre; that shortfall is the
// curvature under test, not a calibration to hide it.
const SENSOR_DIST_H: f32 = 0.15;
const MOVE_H: f32 = 0.035;
const DISK_R2_MAX: f32 = 0.98; // the mold's boundary rule, verbatim
const SPAWN_R2: f32 = 0.81; // spawn inside radius 0.9
const CITY_SCALE: f32 = 0.85; // fractional addresses -> disk coords
const FEED_RADIUS2_DISK: f32 = 0.005_314; // (3.5/48)^2 in chart units
const STAMP_RADIUS_DISK: f32 = 0.0625; // 3/48 in chart units

// ---- the season ----
const W: usize = 96;
const H: usize = 64;
const AGENTS: usize = 1200;
const TICKS: usize = 1200;
const LONG_TICKS: usize = 2400;
const FAR_TICKS: usize = 4800;
const LAST_TICKS: usize = 7200;
const PAST_TICKS: usize = 9600;
const SEED: u64 = 0x5059_5341_5255_4D80; // "PHYSARUM" + 0

// The curved season, pinned as found on the first shipped run
// (test-only constants; see the test below).
#[cfg(test)]
const CURVED_ETERNAL_LOUD_VISITS: u64 = 36226;
#[cfg(test)]
const CURVED_ETERNAL_QUIET_VISITS: u64 = 4712;
#[cfg(test)]
const CURVED_ETERNAL_MASS: f32 = 1124.80;
#[cfg(test)]
const CURVED_ETERNAL_COVERAGE: usize = 1908;
#[cfg(test)]
const CURVED_DEPLETED_DARK: usize = 4;
#[cfg(test)]
const CURVED_DEPLETED_MASS: f32 = 921.05;
#[cfg(test)]
const CURVED_DEPLETED_COVERAGE: usize = 1557;
#[cfg(test)]
const LONG_DARK: usize = 7;
#[cfg(test)]
const LONG_MASS: f32 = 940.62;
#[cfg(test)]
const LONG_COVERAGE: usize = 1540;
#[cfg(test)]
const FLAT_LONG_DARK: usize = 9;
#[cfg(test)]
const FAR_DARK: usize = 8;
#[cfg(test)]
const FAR_MASS: f32 = 452.64;
#[cfg(test)]
const FAR_COVERAGE: usize = 733;
#[cfg(test)]
const FLAT_FAR_DARK: usize = 9;
#[cfg(test)]
const LAST_DARK: usize = 9;
#[cfg(test)]
const LAST_MASS: f32 = 509.72;
#[cfg(test)]
const LAST_COVERAGE: usize = 831;
#[cfg(test)]
const FLAT_LAST_DARK: usize = 9;
#[cfg(test)]
const PAST_DARK: usize = 9;
#[cfg(test)]
const PAST_MASS: f32 = 657.11;
#[cfg(test)]
const PAST_COVERAGE: usize = 1099;
#[cfg(test)]
const FLAT_PAST_DARK: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Regime {
    OpposedDepleted,
    OpposedLongDepleted,
    OpposedFarDepleted,
    CurvedOpposedEternal,
    CurvedOpposedDepleted,
    CurvedLongEternal,
    CurvedLongDepleted,
    CurvedFarEternal,
    CurvedFarDepleted,
    OpposedLastDepleted,
    CurvedLastEternal,
    CurvedLastDepleted,
    OpposedPastDepleted,
    CurvedPastEternal,
    CurvedPastDepleted,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::OpposedDepleted => "opposed-depleted",
            Regime::OpposedLongDepleted => "opposed-long-depleted",
            Regime::OpposedFarDepleted => "opposed-far-depleted",
            Regime::CurvedOpposedEternal => "curved-opposed-eternal",
            Regime::CurvedOpposedDepleted => "curved-opposed-depleted",
            Regime::CurvedLongEternal => "curved-long-eternal",
            Regime::CurvedLongDepleted => "curved-long-depleted",
            Regime::CurvedFarEternal => "curved-far-eternal",
            Regime::CurvedFarDepleted => "curved-far-depleted",
            Regime::OpposedLastDepleted => "opposed-last-depleted",
            Regime::CurvedLastEternal => "curved-last-eternal",
            Regime::CurvedLastDepleted => "curved-last-depleted",
            Regime::OpposedPastDepleted => "opposed-past-depleted",
            Regime::CurvedPastEternal => "curved-past-eternal",
            Regime::CurvedPastDepleted => "curved-past-depleted",
        }
    }
    fn gates(self) -> bool {
        matches!(
            self,
            Regime::OpposedDepleted
                | Regime::OpposedLongDepleted
                | Regime::OpposedFarDepleted
                | Regime::CurvedOpposedDepleted
                | Regime::CurvedLongDepleted
                | Regime::CurvedFarDepleted
                | Regime::OpposedLastDepleted
                | Regime::CurvedLastDepleted
                | Regime::OpposedPastDepleted
                | Regime::CurvedPastDepleted
        )
    }
    fn opposed(self) -> bool {
        true // every room in this cross carries the opposed channels
    }
    fn curved(self) -> bool {
        matches!(
            self,
            Regime::CurvedOpposedEternal
                | Regime::CurvedOpposedDepleted
                | Regime::CurvedLongEternal
                | Regime::CurvedLongDepleted
                | Regime::CurvedFarEternal
                | Regime::CurvedFarDepleted
                | Regime::CurvedLastEternal
                | Regime::CurvedLastDepleted
                | Regime::CurvedPastEternal
                | Regime::CurvedPastDepleted
        )
    }
    fn ticks(self) -> usize {
        match self {
            Regime::OpposedPastDepleted
            | Regime::CurvedPastEternal
            | Regime::CurvedPastDepleted => PAST_TICKS,
            Regime::OpposedLastDepleted
            | Regime::CurvedLastEternal
            | Regime::CurvedLastDepleted => LAST_TICKS,
            Regime::OpposedFarDepleted
            | Regime::CurvedFarEternal
            | Regime::CurvedFarDepleted => FAR_TICKS,
            Regime::OpposedLongDepleted
            | Regime::CurvedLongEternal
            | Regime::CurvedLongDepleted => LONG_TICKS,
            _ => TICKS,
        }
    }
    fn eat_rate(self, loud: bool) -> f32 {
        if self.opposed() && !loud { EAT_QUIET } else { EAT_LOUD }
    }
    fn stamp(self, loud: bool) -> f32 {
        if self.opposed() && !loud { QUIET_STAMP } else { LOUD_STAMP }
    }
}

/// Deterministic SplitMix64 stream — one per purpose, as the
/// Cartographer's revivals keep them.
#[derive(Clone)]
struct SplitMix {
    state: u64,
}

impl SplitMix {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// A point in the disk, as a pair (re, im) — the mold's Complex<f64>
/// kept in f32 so the curved rooms share the lineage's arithmetic.
type Z = (f32, f32);

fn z_add(a: Z, b: Z) -> Z {
    (a.0 + b.0, a.1 + b.1)
}

fn z_mul(a: Z, b: Z) -> Z {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn z_conj(a: Z) -> Z {
    (a.0, -a.1)
}

fn z_norm2(a: Z) -> f32 {
    a.0 * a.0 + a.1 * a.1
}

/// Möbius addition a ⊕ b = (a + b) / (1 + conj(a)·b) — the mold's
/// geodesic translation, the disk's only road.
fn mobius_add(a: Z, b: Z) -> Z {
    let num = z_add(a, b);
    let den = z_add((1.0, 0.0), z_mul(z_conj(a), b));
    let d2 = z_norm2(den);
    let q = z_mul(num, z_conj(den));
    (q.0 / d2, q.1 / d2)
}

fn z_polar(r: f32, ang: f32) -> Z {
    (r * ang.cos(), r * ang.sin())
}

/// The chart: disk point -> grid cell (the mold's projection).
fn chart_cell(z: Z, w: usize, h: usize) -> Option<(usize, usize)> {
    if z_norm2(z) >= 1.0 {
        return None;
    }
    let x = ((z.0 + 1.0) * 0.5 * w as f32) as usize;
    let y = ((z.1 + 1.0) * 0.5 * h as f32) as usize;
    if x < w && y < h { Some((x, y)) } else { None }
}

/// The chart read backwards: a grid cell's centre as a disk point.
fn cell_centre(x: usize, y: usize, w: usize, h: usize) -> Z {
    (
        (x as f32 + 0.5) / w as f32 * 2.0 - 1.0,
        (y as f32 + 0.5) / h as f32 * 2.0 - 1.0,
    )
}

#[derive(Clone, Copy, Debug)]
struct Agent {
    x: f32,
    y: f32,
    angle: f32,
}

#[derive(Clone, Debug)]
struct City {
    x: f32,
    y: f32,
    loud: bool,
    stock: f32,
    lit: bool,
    visits: u64,
    ghost_visits: u64,
    dark_ticks: u64,
    first_dark: Option<usize>,
}

impl City {
    fn new(x: f32, y: f32, loud: bool) -> Self {
        Self {
            x,
            y,
            loud,
            stock: STOCK0,
            lit: true,
            visits: 0,
            ghost_visits: 0,
            dark_ticks: 0,
            first_dark: None,
        }
    }
}

fn city_map(w: usize, h: usize) -> Vec<(f32, f32, bool)> {
    // Nine cities, hand-surveyed: a ring of eight and a centre —
    // the lineage's map verbatim (flat ground).
    let (wf, hf) = (w as f32, h as f32);
    vec![
        (wf * 0.50, hf * 0.50, true),
        (wf * 0.17, hf * 0.25, true),
        (wf * 0.50, hf * 0.16, false),
        (wf * 0.83, hf * 0.25, true),
        (wf * 0.92, hf * 0.62, false),
        (wf * 0.75, hf * 0.84, true),
        (wf * 0.50, hf * 0.88, false),
        (wf * 0.17, hf * 0.78, true),
        (wf * 0.08, hf * 0.50, false),
    ]
}

fn city_map_disk() -> Vec<(f32, f32, bool)> {
    // The same nine addresses read as disk coordinates: the flat
    // map's fractional position (fx, fy) becomes the disk point
    // ((2fx-1)·s, (2fy-1)·s) with s = CITY_SCALE, so the ring stands
    // inside the disk (outermost |z| = 0.743, city 4) and the centre
    // city stands at the origin, untaxed by construction. Classes
    // unchanged stone for stone.
    city_map(2, 2)
        .into_iter()
        .map(|(fx, fy, loud)| {
            (
                (fx - 1.0) * CITY_SCALE,
                (fy - 1.0) * CITY_SCALE,
                loud,
            )
        })
        .collect()
}

fn sense_on(grid: &[f32], w: usize, h: usize, a: &Agent, offset: f32) -> f32 {
    let ang = a.angle + offset;
    let sx = a.x + ang.cos() * SENSOR_DIST;
    let sy = a.y + ang.sin() * SENSOR_DIST;
    let x = (sx.round() as isize).rem_euclid(w as isize) as usize;
    let y = (sy.round() as isize).rem_euclid(h as isize) as usize;
    grid[y * w + x]
}

fn sense_disk(grid: &[f32], w: usize, h: usize, a: &Agent, offset: f32) -> f32 {
    // The mold's sensing: the sensor's position is the agent's
    // position Möbius-translated by the sensor vector. Outside the
    // disk the chart reads 0 — the void has no smell.
    let s = mobius_add((a.x, a.y), z_polar(SENSOR_DIST_H, a.angle + offset));
    match chart_cell(s, w, h) {
        Some((x, y)) => grid[y * w + x],
        None => 0.0,
    }
}

struct World {
    w: usize,
    h: usize,
    grid: Vec<f32>,
    agents: Vec<Agent>,
    cities: Vec<City>,
    steer: SplitMix,
    regime: Regime,
    tick: usize,
}

impl World {
    fn new(w: usize, h: usize, n_agents: usize, regime: Regime, seed: u64) -> Self {
        let mut spawn = SplitMix::new(seed);
        let mut agents = Vec::with_capacity(n_agents);
        if regime.curved() {
            // Spawn uniform in the disk by rejection from the same
            // seeded stream (draws in agent order; a rejected pair is
            // consumed, never redrawn from a second stream).
            for _ in 0..n_agents {
                loop {
                    let u = spawn.next_f32() * 2.0 - 1.0;
                    let v = spawn.next_f32() * 2.0 - 1.0;
                    if u * u + v * v < SPAWN_R2 {
                        agents.push(Agent {
                            x: u,
                            y: v,
                            angle: spawn.next_f32() * 2.0 * PI,
                        });
                        break;
                    }
                }
            }
        } else {
            for _ in 0..n_agents {
                agents.push(Agent {
                    x: spawn.next_f32() * w as f32,
                    y: spawn.next_f32() * h as f32,
                    angle: spawn.next_f32() * 2.0 * PI,
                });
            }
        }
        let cities = if regime.curved() {
            city_map_disk()
        } else {
            city_map(w, h)
        }
        .into_iter()
        .map(|(x, y, loud)| City::new(x, y, loud))
        .collect();
        Self {
            w,
            h,
            grid: vec![0.0; w * h],
            agents,
            cities,
            steer: SplitMix::new(seed ^ 0x57EE_12),
            regime,
            tick: 0,
        }
    }

    fn sense(&self, a: &Agent, offset: f32) -> f32 {
        if self.regime.curved() {
            sense_disk(&self.grid, self.w, self.h, a, offset)
        } else {
            sense_on(&self.grid, self.w, self.h, a, offset)
        }
    }

    fn in_disk(&self, x: usize, y: usize) -> bool {
        z_norm2(cell_centre(x, y, self.w, self.h)) < 1.0
    }

    fn step(&mut self) {
        let curved = self.regime.curved();
        // 1. Agents sense and move. All sensing reads the field as
        //    the tick found it. Flat: the corpse's steering verbatim.
        //    Curved: the same steering, but the roads are Möbius
        //    translations and the mold's boundary rule turns an agent
        //    around rather than letting it leave the disk.
        let mut moved: Vec<Agent> = Vec::with_capacity(self.agents.len());
        for a in &self.agents {
            let mut a = *a;
            let left = self.sense(&a, -SENSOR_ANGLE);
            let center = self.sense(&a, 0.0);
            let right = self.sense(&a, SENSOR_ANGLE);
            // The corpse drew its random steer every update.
            let random_steer = self.steer.next_f32();
            if center > left && center > right {
                // keep direction
            } else if center < left && center < right {
                if random_steer < 0.5 {
                    a.angle -= ROTATION_ANGLE;
                } else {
                    a.angle += ROTATION_ANGLE;
                }
            } else if left < right {
                a.angle += ROTATION_ANGLE;
            } else if right < left {
                a.angle -= ROTATION_ANGLE;
            }
            if curved {
                let nz = mobius_add((a.x, a.y), z_polar(MOVE_H, a.angle));
                if z_norm2(nz) < DISK_R2_MAX {
                    a.x = nz.0;
                    a.y = nz.1;
                } else {
                    a.angle += PI; // the mold's boundary rule, verbatim
                }
            } else {
                a.x += a.angle.cos() * AGENT_SPEED;
                a.y += a.angle.sin() * AGENT_SPEED;
                if a.x < 0.0 {
                    a.x += self.w as f32;
                }
                if a.x >= self.w as f32 {
                    a.x -= self.w as f32;
                }
                if a.y < 0.0 {
                    a.y += self.h as f32;
                }
                if a.y >= self.h as f32 {
                    a.y -= self.h as f32;
                }
            }
            moved.push(a);
        }
        self.agents = moved;

        // 2. Agents deposit — the corpse's 0.5, capped at 1.0. On the
        //    disk the deposit lands on the projection, as the mold had it.
        for i in 0..self.agents.len() {
            let a = self.agents[i];
            if curved {
                if let Some((x, y)) = chart_cell((a.x, a.y), self.w, self.h) {
                    let idx = y * self.w + x;
                    self.grid[idx] = (self.grid[idx] + DEPOSIT).min(1.0);
                }
            } else {
                let x = a.x as usize;
                let y = a.y as usize;
                if x < self.w && y < self.h {
                    let idx = y * self.w + x;
                    self.grid[idx] = (self.grid[idx] + DEPOSIT).min(1.0);
                }
            }
        }

        // 3. Feeding: agents in a city's apron eat from its stock.
        //    The stock falls in every regime; only gating differs.
        //    Flat aprons wrap on the torus; disk aprons are chart
        //    distances (the projection is where the mouth lives).
        for ci in 0..self.cities.len() {
            let (cx, cy) = (self.cities[ci].x, self.cities[ci].y);
            let mut feeders = 0u64;
            for a in &self.agents {
                let d2 = if curved {
                    let dx = a.x - cx;
                    let dy = a.y - cy;
                    dx * dx + dy * dy
                } else {
                    let mut dx = (a.x - cx).abs();
                    if dx > self.w as f32 / 2.0 {
                        dx = self.w as f32 - dx;
                    }
                    let mut dy = (a.y - cy).abs();
                    if dy > self.h as f32 / 2.0 {
                        dy = self.h as f32 - dy;
                    }
                    dx * dx + dy * dy
                };
                let in_apron = if curved {
                    d2 <= FEED_RADIUS2_DISK
                } else {
                    d2 <= FEED_RADIUS2
                };
                if in_apron {
                    feeders += 1;
                }
            }
            if feeders > 0 {
                let rate = self.regime.eat_rate(self.cities[ci].loud);
                let city = &mut self.cities[ci];
                city.visits += feeders;
                if !city.lit {
                    city.ghost_visits += feeders;
                }
                city.stock = (city.stock - rate * feeders as f32).max(0.0);
            }
        }

        // 4. Bookkeeping: darkness.
        for city in &mut self.cities {
            if self.regime.gates() {
                if city.lit && city.stock <= 0.0 {
                    city.lit = false;
                    if city.first_dark.is_none() {
                        city.first_dark = Some(self.tick);
                    }
                }
                if !city.lit {
                    city.dark_ticks += 1;
                }
            } else if city.first_dark.is_none() && city.stock <= 0.0 {
                // Eternal's shadow ledger: record, never gate.
                city.first_dark = Some(self.tick);
            }
        }

        // 5. Lit cities stamp their disc — gated. Flat: the corpse's
        //    disc on the torus. Curved: every chart cell whose centre
        //    lies within the stamp radius of the city's disk point.
        if curved {
            let stamps: Vec<(f32, f32, f32)> = self
                .cities
                .iter()
                .filter(|c| c.lit || !self.regime.gates())
                .map(|c| (c.x, c.y, self.regime.stamp(c.loud)))
                .collect();
            for (cx, cy, v) in stamps {
                for y in 0..self.h {
                    for x in 0..self.w {
                        let c = cell_centre(x, y, self.w, self.h);
                        let dx = c.0 - cx;
                        let dy = c.1 - cy;
                        if dx * dx + dy * dy <= STAMP_RADIUS_DISK * STAMP_RADIUS_DISK {
                            self.grid[y * self.w + x] = v;
                        }
                    }
                }
            }
        } else {
            let stamps: Vec<(f32, f32, f32)> = self
                .cities
                .iter()
                .filter(|c| c.lit || !self.regime.gates())
                .map(|c| (c.x, c.y, self.regime.stamp(c.loud)))
                .collect();
            for (cx, cy, v) in stamps {
                let (cxi, cyi) = (cx as isize, cy as isize);
                for dy in -CITY_RADIUS..=CITY_RADIUS {
                    for dx in -CITY_RADIUS..=CITY_RADIUS {
                        if dx * dx + dy * dy <= CITY_RADIUS * CITY_RADIUS {
                            let nx = (cxi + dx).rem_euclid(self.w as isize) as usize;
                            let ny = (cyi + dy).rem_euclid(self.h as isize) as usize;
                            self.grid[ny * self.w + nx] = v;
                        }
                    }
                }
            }
        }

        // 6. Diffuse (3×3 mean) and decay ×0.95. Flat: the corpse,
        //    verbatim, on the torus. Curved: the mold's hybrid — the
        //    diffusion lives on the projection; cells outside the
        //    disk are void and read 0 in every neighbourhood, so the
        //    rim bleeds field into the void each tick.
        let old = self.grid.clone();
        let (w, h) = (self.w, self.h);
        for y in 0..h {
            for x in 0..w {
                if curved && !self.in_disk(x, y) {
                    self.grid[y * w + x] = 0.0;
                    continue;
                }
                let mut sum = 0.0f32;
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        if curved {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                                continue; // void neighbour reads 0
                            }
                            let (ux, uy) = (nx as usize, ny as usize);
                            if !self.in_disk(ux, uy) {
                                continue; // void neighbour reads 0
                            }
                            sum += old[uy * w + ux];
                        } else {
                            let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                            sum += old[ny * w + nx];
                        }
                    }
                }
                self.grid[y * w + x] = (sum / 9.0) * DECAY;
            }
        }

        self.tick += 1;
    }

    fn mass(&self) -> f32 {
        self.grid.iter().sum()
    }

    fn coverage(&self) -> usize {
        self.grid.iter().filter(|&&v| v > 0.2).count()
    }

    fn first_dark(&self) -> Option<usize> {
        self.cities.iter().filter_map(|c| c.first_dark).min()
    }
}

struct Season {
    regime: Regime,
    mass_series: Vec<f32>, // every 100 ticks, plus the final tick
    first_dark: Option<usize>,
    mass_at_first_dark: Option<f32>,
    final_mass: f32,
    final_coverage: usize,
    cities: Vec<City>,
    ghost_last300: u64,
    ghost_last300_cities: Vec<u64>, // per city, final 300 ticks only
    ghost_slices: Vec<Vec<u64>>, // per 300-tick slice from tick 7,200, per city (past rooms)
}

fn run_season(regime: Regime) -> Season {
    let ticks = regime.ticks();
    let mut world = World::new(W, H, AGENTS, regime, SEED);
    let mut mass_series = Vec::new();
    let mut mass_at_first_dark = None;
    let mut ghost_at_900 = 0u64;
    let mut ghost_cities_at_900: Vec<u64> = vec![0; world.cities.len()];
    let mut ghost_snaps: Vec<Vec<u64>> = Vec::new();
    for t in 0..ticks {
        world.step();
        if t + 1 >= 7200 && (t + 1) % 300 == 0 {
            ghost_snaps.push(world.cities.iter().map(|c| c.ghost_visits).collect());
        }
        if (t + 1) % 100 == 0 {
            mass_series.push(world.mass());
        }
        if mass_at_first_dark.is_none() {
            if let Some(fd) = world.first_dark() {
                if fd == t {
                    mass_at_first_dark = Some(world.mass());
                }
            }
        }
        if t + 1 == ticks - 300 {
            ghost_at_900 = world.cities.iter().map(|c| c.ghost_visits).sum();
            ghost_cities_at_900 = world.cities.iter().map(|c| c.ghost_visits).collect();
        }
    }
    let ghost_total: u64 = world.cities.iter().map(|c| c.ghost_visits).sum();
    let ghost_slices: Vec<Vec<u64>> = ghost_snaps
        .windows(2)
        .map(|w| w[1].iter().zip(w[0].iter()).map(|(a, b)| a - b).collect())
        .collect();
    Season {
        regime,
        mass_series,
        first_dark: world.first_dark(),
        mass_at_first_dark,
        final_mass: world.mass(),
        final_coverage: world.coverage(),
        cities: world.cities.clone(),
        ghost_last300: ghost_total - ghost_at_900,
        ghost_last300_cities: world
            .cities
            .iter()
            .zip(ghost_cities_at_900.iter())
            .map(|(c, g0)| c.ghost_visits - g0)
            .collect(),
        ghost_slices,
    }
}

fn class_stats(cities: &[City], loud: bool) -> (u64, u64, f32, i64, usize) {
    // (visits, ghost visits, mean first_dark, min first_dark, dark at end)
    let sel: Vec<&City> = cities.iter().filter(|c| c.loud == loud).collect();
    let visits: u64 = sel.iter().map(|c| c.visits).sum();
    let ghost: u64 = sel.iter().map(|c| c.ghost_visits).sum();
    let darks: Vec<usize> = sel.iter().filter_map(|c| c.first_dark).collect();
    let mean_fd = if darks.is_empty() {
        -1.0
    } else {
        darks.iter().sum::<usize>() as f32 / darks.len() as f32
    };
    let min_fd = darks.iter().min().map(|t| *t as i64).unwrap_or(-1);
    let dark_end = sel.iter().filter(|c| !c.lit).count();
    (visits, ghost, mean_fd, min_fd, dark_end)
}

fn effective_mean_fd(cities: &[City], loud: bool, ticks: usize) -> f32 {
    // Mean first-dark tick counting a never-dark city as the season
    // length - the parent's P2 yardstick, priced per room.
    let sel: Vec<&City> = cities.iter().filter(|c| c.loud == loud).collect();
    let sum: usize = sel
        .iter()
        .map(|c| c.first_dark.unwrap_or(ticks))
        .sum();
    sum as f32 / sel.len() as f32
}

fn headless() {
    println!("physarum-migration \u{2014} headless ledger");
    println!(
        "config: grid {}x{}, agents {}, ticks {} (long rooms {}, far rooms {}, last rooms {}, past rooms {}), cities {} (loud-slow 5, quiet-quick 4), stock0 {:.1}, stamp_loud {:.2}, stamp_quiet {:.2}, eat_loud {:.3}, eat_quiet {:.3}, seed {:#018x}; curved rooms: poincare disk, sensor_h {:.3}, move_h {:.3}, boundary |z|^2 < {:.2}, city_scale {:.2}",
        W, H, AGENTS, TICKS, LONG_TICKS, FAR_TICKS, LAST_TICKS, PAST_TICKS, 9, STOCK0, LOUD_STAMP, QUIET_STAMP, EAT_LOUD, EAT_QUIET, SEED,
        SENSOR_DIST_H, MOVE_H, DISK_R2_MAX, CITY_SCALE
    );
    let seasons: Vec<Season> = [
        Regime::OpposedDepleted,
        Regime::OpposedLongDepleted,
        Regime::CurvedOpposedEternal,
        Regime::CurvedOpposedDepleted,
        Regime::CurvedLongEternal,
        Regime::CurvedLongDepleted,
        Regime::OpposedFarDepleted,
        Regime::CurvedFarEternal,
        Regime::CurvedFarDepleted,
        Regime::OpposedLastDepleted,
        Regime::CurvedLastEternal,
        Regime::CurvedLastDepleted,
        Regime::OpposedPastDepleted,
        Regime::CurvedPastEternal,
        Regime::CurvedPastDepleted,
    ]
    .into_iter()
    .map(run_season)
    .collect();
    for s in &seasons {
        let visits: u64 = s.cities.iter().map(|c| c.visits).sum();
        let ghost: u64 = s.cities.iter().map(|c| c.ghost_visits).sum();
        let dark_ticks: u64 = s.cities.iter().map(|c| c.dark_ticks).sum();
        let stock_left: f32 = s.cities.iter().map(|c| c.stock).sum();
        println!(
            "regime {}: visits {}, ghost {}, ghost_last300 {}, first_dark {}, mass_at_first_dark {}, dark_cities_end {}, dark_ticks {}, stock_left {:.2}, final_mass {:.2}, final_coverage {}",
            s.regime.name(),
            visits,
            ghost,
            s.ghost_last300,
            s.first_dark.map(|t| t as i64).unwrap_or(-1),
            s.mass_at_first_dark
                .map(|m| format!("{:.2}", m))
                .unwrap_or_else(|| "none".to_string()),
            s.cities.iter().filter(|c| !c.lit && s.regime.gates()).count(),
            dark_ticks,
            stock_left,
            s.final_mass,
            s.final_coverage
        );
        for (label, loud) in [("loud", true), ("quiet", false)] {
            let (v, g, mean_fd, min_fd, dark_end) = class_stats(&s.cities, loud);
            println!(
                "  class {}: visits {}, ghost {}, mean_first_dark {:.1}, eff_mean_first_dark {:.1}, min_first_dark {}, dark_end {}",
                label, v, g, mean_fd, effective_mean_fd(&s.cities, loud, s.regime.ticks()), min_fd, dark_end
            );
        }
        if s.regime.opposed() {
            let (lv, _, _, _, _) = class_stats(&s.cities, true);
            let (qv, _, _, _, _) = class_stats(&s.cities, false);
            let ratio = (lv as f32 / 5.0) / (qv as f32 / 4.0);
            let ghost_total = ghost as f32;
            let loud_ghost = class_stats(&s.cities, true).1 as f32;
            println!(
                "  per-city visit ratio loud/quiet: {:.2}; loud ghost share: {}",
                ratio,
                if ghost_total > 0.0 {
                    format!("{:.1}%", 100.0 * loud_ghost / ghost_total)
                } else {
                    "n/a".to_string()
                }
            );
        }
        println!(
            "  mass_series[100..{}]: {}", s.regime.ticks(),
            s.mass_series
                .iter()
                .map(|m| format!("{:.1}", m))
                .collect::<Vec<_>>()
                .join(" ")
        );
        for (i, c) in s.cities.iter().enumerate() {
            println!(
                "  city {} ({}): visits {}, ghost {}, first_dark {}, dark_ticks {}, stock {:.2}, lit {}",
                i,
                if c.loud { "loud" } else { "quiet" },
                c.visits,
                c.ghost_visits,
                c.first_dark.map(|t| t as i64).unwrap_or(-1),
                c.dark_ticks,
                c.stock,
                c.lit
            );
        }
    }
    // The lineage's identity, in both spellings (printed; a test
    // asserts the truncation one):
    for (e, d) in [
        (&seasons[2], &seasons[3]),
        (&seasons[4], &seasons[5]),
        (&seasons[7], &seasons[8]),
    ] {
        let shared = match d.first_dark {
            Some(fd) => {
                let n = (fd + 1) / 100;
                let upto = n.min(e.mass_series.len()).min(d.mass_series.len());
                (0..upto).all(|i| e.mass_series[i] == d.mass_series[i])
            }
            None => e.mass_series == d.mass_series,
        };
        println!(
            "identity {}=={} on mass samples before first_dark: {}",
            e.regime.name(),
            d.regime.name(),
            shared
        );
    }
    // Truncation identities: a long room's first 1,200 ticks are the
    // short room's whole season, sample for sample - the long rooms
    // differ only in being allowed to continue.
    for (short, long) in [
        (&seasons[0], &seasons[1]),
        (&seasons[3], &seasons[5]),
        (&seasons[1], &seasons[6]),
        (&seasons[5], &seasons[8]),
    ] {
        let upto = short.mass_series.len().min(long.mass_series.len());
        let same = (0..upto).all(|i| short.mass_series[i] == long.mass_series[i])
            && short.first_dark == long.first_dark;
        println!(
            "truncation {}=={}[0..1200]: {}",
            long.regime.name(),
            short.regime.name(),
            same
        );
    }
    for (far, last) in [
        (&seasons[6], &seasons[9]),
        (&seasons[8], &seasons[11]),
    ] {
        let upto = far.mass_series.len().min(last.mass_series.len());
        let same = (0..upto).all(|i| far.mass_series[i] == last.mass_series[i])
            && far.first_dark == last.first_dark;
        println!(
            "truncation {}=={}[0..4800]: {}",
            last.regime.name(),
            far.regime.name(),
            same
        );
    }
    for (e, d) in [(&seasons[10], &seasons[11])] {
        let shared = match d.first_dark {
            Some(fd) => {
                let n = (fd + 1) / 100;
                let upto = n.min(e.mass_series.len()).min(d.mass_series.len());
                (0..upto).all(|i| e.mass_series[i] == d.mass_series[i])
            }
            None => e.mass_series == d.mass_series,
        };
        println!(
            "identity {}=={} on mass samples before first_dark: {}",
            e.regime.name(),
            d.regime.name(),
            shared
        );
    }
    for (last, past) in [
        (&seasons[9], &seasons[12]),
        (&seasons[11], &seasons[14]),
    ] {
        let upto = last.mass_series.len().min(past.mass_series.len());
        let same = (0..upto).all(|i| last.mass_series[i] == past.mass_series[i])
            && last.first_dark == past.first_dark;
        println!(
            "truncation {}=={}[0..7200]: {}",
            past.regime.name(),
            last.regime.name(),
            same
        );
    }
    for (e, d) in [(&seasons[13], &seasons[14])] {
        let shared = match d.first_dark {
            Some(fd) => {
                let n = (fd + 1) / 100;
                let upto = n.min(e.mass_series.len()).min(d.mass_series.len());
                (0..upto).all(|i| e.mass_series[i] == d.mass_series[i])
            }
            None => e.mass_series == d.mass_series,
        };
        println!(
            "identity {}=={} on mass samples before first_dark: {}",
            e.regime.name(),
            d.regime.name(),
            shared
        );
    }
    // The far window, priced by difference (exact by the truncation
    // identities above): what ticks 2,400..4,800 did to each city.
    let long_d = &seasons[5];
    let far_d = &seasons[8];
    for (i, (fc, lc)) in far_d.cities.iter().zip(long_d.cities.iter()).enumerate() {
        println!(
            "  far-window city {} ({}): visits_2400_4800 {}, visits_0_2400 {}, first_dark {}, stock_at_4800 {:.2}, lit {}",
            i,
            if fc.loud { "loud" } else { "quiet" },
            fc.visits - lc.visits,
            lc.visits,
            fc.first_dark.map(|t| t as i64).unwrap_or(-1),
            fc.stock,
            fc.lit
        );
    }
    // The last window, priced by difference (exact by the truncation
    // identities above): what ticks 4,800..7,200 did to each city.
    let far_d2 = &seasons[8];
    let last_d = &seasons[11];
    for (i, (lc, fc)) in last_d.cities.iter().zip(far_d2.cities.iter()).enumerate() {
        println!(
            "  last-window city {} ({}): visits_4800_7200 {}, visits_0_4800 {}, first_dark {}, stock_at_7200 {:.2}, lit {}",
            i,
            if lc.loud { "loud" } else { "quiet" },
            lc.visits - fc.visits,
            fc.visits,
            lc.first_dark.map(|t| t as i64).unwrap_or(-1),
            lc.stock,
            lc.lit
        );
    }
    // The last sound, priced city by city: ghost visits in the
    // final 300 ticks of the last depleted room.
    for (i, g) in last_d.ghost_last300_cities.iter().enumerate() {
        println!(
            "  last300 ghost city {} ({}): {}",
            i,
            if last_d.cities[i].loud { "loud" } else { "quiet" },
            g
        );
    }
    // The shadow footnote at the last length: city 8's gated fall
    // against its shadow exhaustion in the last eternal room.
    let last_e = &seasons[10];
    println!(
        "  city 8 at the last length: gated first_dark {}, shadow first_dark {}, shadow stock {:.2}, shadow visits {}",
        last_d.cities[8].first_dark.map(|t| t as i64).unwrap_or(-1),
        last_e.cities[8].first_dark.map(|t| t as i64).unwrap_or(-1),
        last_e.cities[8].stock,
        last_e.cities[8].visits
    );
    // The past window, priced by difference (exact by the truncation
    // identities above): what ticks 7,200..9,600 did to each city.
    // Every visit in this window is a ghost visit on the disk: the
    // last city fell at tick 5,204, a full season before it opened.
    let last_d2 = &seasons[11];
    let past_d = &seasons[14];
    for (i, (pc, lc)) in past_d.cities.iter().zip(last_d2.cities.iter()).enumerate() {
        println!(
            "  past-window city {} ({}): visits_7200_9600 {}, ghost_7200_9600 {}, visits_0_7200 {}, first_dark {}, stock_at_9600 {:.2}, lit {}",
            i,
            if pc.loud { "loud" } else { "quiet" },
            pc.visits - lc.visits,
            pc.ghost_visits - lc.ghost_visits,
            lc.visits,
            pc.first_dark.map(|t| t as i64).unwrap_or(-1),
            pc.stock,
            pc.lit
        );
    }
    // The duet, priced at both ends of the past season: ghost visits
    // in the final 300 ticks of the last room (ticks 6,900..7,200)
    // against the final 300 ticks of the past room (9,300..9,600).
    for (i, (g_last, g_past)) in last_d2
        .ghost_last300_cities
        .iter()
        .zip(past_d.ghost_last300_cities.iter())
        .enumerate()
    {
        println!(
            "  duet ghost city {} ({}): final300_at_7200 {}, final300_at_9600 {}",
            i,
            if past_d.cities[i].loud { "loud" } else { "quiet" },
            g_last,
            g_past
        );
    }
    // The shadow floor at the past length: city 8's gated fall
    // against its shadow exhaustion in the past eternal room.
    let past_e = &seasons[13];
    println!(
        "  city 8 at the past length: gated first_dark {}, shadow first_dark {}, shadow stock {:.2}, shadow visits {}",
        past_d.cities[8].first_dark.map(|t| t as i64).unwrap_or(-1),
        past_e.cities[8].first_dark.map(|t| t as i64).unwrap_or(-1),
        past_e.cities[8].stock,
        past_e.cities[8].visits
    );
    // The migration, dated: ghost visits per city per 300-tick
    // slice across the past window (ticks 7,200..9,600), read in
    // CurvedPastDepleted - the instrument this survey adds.
    println!("  migration slices (CurvedPastDepleted, ghost visits per 300 ticks):");
    for (k, slice) in past_d.ghost_slices.iter().enumerate() {
        let lo = 7200 + 300 * k;
        let total: u64 = slice.iter().sum();
        let top = slice
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .map(|(i, _)| i)
            .unwrap_or(0);
        println!(
            "  slice {} [{}..{}): total {}, argmax city {}, city8 {}, city1 {}, city5 {}, city6 {}",
            k, lo, lo + 300, total, top, slice[8], slice[1], slice[5], slice[6]
        );
        for (i, g) in slice.iter().enumerate() {
            println!(
                "  slice{} ghost city {} ({}): {}",
                k,
                i,
                if past_d.cities[i].loud { "loud" } else { "quiet" },
                g
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        headless();
        return;
    }
    // No TUI in this revival: a window was half of both corpses'
    // death certificates. Headless is the only mouth.
    headless();
}

#[cfg(test)]
mod tests {
    const MIGRATION_C8: [u64; 8] = [378, 1469, 1654, 1650, 1966, 212, 17, 0];
    const MIGRATION_C1: [u64; 8] = [0, 0, 0, 0, 333, 0, 0, 383];
    const MIGRATION_C5: [u64; 8] = [541, 125, 92, 299, 8, 193, 172, 373];
    use super::*;

    #[test]
    fn splitmix_is_deterministic() {
        let mut a = SplitMix::new(42);
        let mut b = SplitMix::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
        let mut c = SplitMix::new(42);
        for _ in 0..1000 {
            let v = c.next_f32();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn mobius_addition_is_the_disks_road() {
        // Identity: 0 ⊕ b = b, a ⊕ 0 = a.
        let b = (0.3f32, -0.4f32);
        let r = mobius_add((0.0, 0.0), b);
        assert!((r.0 - b.0).abs() < 1e-6 && (r.1 - b.1).abs() < 1e-6);
        let a = (-0.2f32, 0.55f32);
        let r = mobius_add(a, (0.0, 0.0));
        assert!((r.0 - a.0).abs() < 1e-6 && (r.1 - a.1).abs() < 1e-6);
        // Closure: the disk is closed under ⊕ for points inside it.
        let mut rng = SplitMix::new(7);
        for _ in 0..1000 {
            let p = (rng.next_f32() * 1.8 - 0.9, rng.next_f32() * 1.8 - 0.9);
            let q = (rng.next_f32() * 0.4 - 0.2, rng.next_f32() * 0.4 - 0.2);
            if z_norm2(p) < 0.98 && z_norm2(q) < 1.0 {
                assert!(z_norm2(mobius_add(p, q)) < 1.0);
            }
        }
        // Compression: the same step travels less chart distance
        // near the rim than at the centre — the curvature itself.
        let step = z_polar(MOVE_H, 0.0);
        let centre = mobius_add((0.0, 0.0), step);
        let rim = mobius_add((0.85, 0.0), step);
        let d_centre = (centre.0).abs();
        let d_rim = (rim.0 - 0.85).abs();
        assert!(d_rim < d_centre, "rim step {} !< centre step {}", d_rim, d_centre);
    }

    #[test]
    fn disk_spawn_and_cities_live_inside() {
        let world = World::new(W, H, AGENTS, Regime::CurvedOpposedDepleted, SEED);
        assert!(world.agents.iter().all(|a| z_norm2((a.x, a.y)) < SPAWN_R2));
        assert!(world.cities.iter().all(|c| z_norm2((c.x, c.y)) < 1.0));
        // City 0 stands at the origin; the outermost city is city 4.
        assert!(z_norm2((world.cities[0].x, world.cities[0].y)) < 1e-9);
        let r4 = z_norm2((world.cities[4].x, world.cities[4].y)).sqrt();
        assert!((r4 - 0.743).abs() < 0.01, "city 4 radius {}", r4);
        // Classes unchanged: five loud, four quiet, same order as flat.
        let flat = World::new(W, H, AGENTS, Regime::OpposedDepleted, SEED);
        for (dc, fc) in world.cities.iter().zip(flat.cities.iter()) {
            assert_eq!(dc.loud, fc.loud);
        }
    }

    #[test]
    fn disk_agents_never_leave_the_disk() {
        let mut world = World::new(48, 32, 200, Regime::CurvedOpposedEternal, 7);
        for _ in 0..200 {
            world.step();
            assert!(
                world.agents.iter().all(|a| z_norm2((a.x, a.y)) < DISK_R2_MAX),
                "an agent escaped the disk"
            );
        }
        // And the void stays void: no field outside the disk.
        for y in 0..world.h {
            for x in 0..world.w {
                if !world.in_disk(x, y) {
                    assert_eq!(world.grid[y * world.w + x], 0.0);
                }
            }
        }
    }

    #[test]
    fn steering_follows_the_corpse_flat() {
        let mut world = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        let a = Agent { x: 16.0, y: 16.0, angle: 0.0 };
        world.grid[16 * 32 + 25] = 1.0; // centre sensor at (25,16)
        let l = world.sense(&a, -SENSOR_ANGLE);
        let c = world.sense(&a, 0.0);
        let r = world.sense(&a, SENSOR_ANGLE);
        assert!(c > l && c > r);
    }

    #[test]
    fn agent_moves_and_deposits_flat() {
        let mut world = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        world.agents.push(Agent { x: 10.0, y: 10.0, angle: 0.0 });
        world.cities.clear();
        world.step();
        let a = world.agents[0];
        assert!(a.x > 10.0, "agent advanced: {}", a.x);
        assert!(world.mass() > 0.0, "a deposit survived diffusion");
    }

    #[test]
    fn lit_city_stamps_dark_city_does_not_flat() {
        let mut lit_world = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        lit_world.agents.clear();
        lit_world.step();
        let (cx, cy) = (lit_world.cities[0].x, lit_world.cities[0].y);
        let idx = cy as usize * 32 + cx as usize;
        assert!(lit_world.grid[idx] > 0.9, "lit city stamped: {}", lit_world.grid[idx]);

        let mut dark_world = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        dark_world.agents.clear();
        for c in &mut dark_world.cities {
            c.stock = 0.0;
            c.lit = false;
        }
        dark_world.step();
        assert_eq!(dark_world.mass(), 0.0, "no lit city, no agents: field stays dark");
    }

    #[test]
    fn feeding_drains_stock_and_darkens_flat() {
        let mut world = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        let (cx, cy) = (world.cities[0].x, world.cities[0].y);
        for _ in 0..10 {
            world.agents.push(Agent { x: cx, y: cy, angle: 0.0 });
        }
        world.step();
        let city = &world.cities[0];
        assert!(city.visits >= 10, "visits {}", city.visits);
        let expected = STOCK0 - EAT * city.visits as f32;
        assert!((city.stock - expected).abs() < 1e-4, "stock {} vs {}", city.stock, expected);
    }

    #[test]
    fn opposed_channels_are_the_only_difference_flat() {
        // Same seed, same spawn: in an opposed room the loud centre
        // stamps 1.0 and the quiet centre 0.40 before diffusion mixes
        // them; measured through the parent's own probe — after one
        // tick with no agents, loud centre 0.95, quiet centre 0.38.
        let mut world = World::new(W, H, 0, Regime::OpposedDepleted, SEED);
        world.agents.clear();
        world.step();
        let (lx, ly) = (world.cities[0].x as usize, world.cities[0].y as usize);
        let (qx, qy) = (world.cities[2].x as usize, world.cities[2].y as usize);
        assert!((world.grid[ly * W + lx] - 0.95).abs() < 0.02, "loud centre {}", world.grid[ly * W + lx]);
        assert!((world.grid[qy * W + qx] - 0.38).abs() < 0.02, "quiet centre {}", world.grid[qy * W + qx]);
        // Per-visit drain: loud at EAT_LOUD, quiet at EAT_QUIET.
        assert_eq!(Regime::OpposedDepleted.eat_rate(true), 0.004);
        assert_eq!(Regime::OpposedDepleted.eat_rate(false), 0.010);
        assert_eq!(Regime::CurvedOpposedDepleted.eat_rate(true), 0.004);
        assert_eq!(Regime::CurvedOpposedDepleted.eat_rate(false), 0.010);
    }

    #[test]
    fn flat_opposed_baseline_reproduces_the_parent() {
        let d = run_season(Regime::OpposedDepleted);
        let (dlv, dg, dmean, _, dde) = class_stats(&d.cities, true);
        let (_, qg, _, _, qde) = class_stats(&d.cities, false);
        assert_eq!(dlv, 73231);
        assert!((dmean - 268.2).abs() < 0.05, "loud mean dark {}", dmean);
        assert_eq!(dg + qg, 53800);
        assert_eq!(dg, 41917);
        assert_eq!(dde + qde, 8);
        assert!((d.final_mass - 2237.22).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, 3410);
        // City 8: the parent's immortal, lit with 4.28 unspent.
        assert!(d.cities[8].lit);
        assert!((d.cities[8].stock - 4.28).abs() < 0.01);
    }

    #[test]
    fn long_rooms_truncate_to_the_short_seasons() {
        // A long room's first 1,200 ticks are the short room's whole
        // season, sample for sample: length is the only difference.
        let short = run_season(Regime::CurvedOpposedDepleted);
        let long = run_season(Regime::CurvedLongDepleted);
        assert_eq!(short.first_dark, long.first_dark);
        assert_eq!(short.mass_series.len(), 12);
        assert_eq!(long.mass_series.len(), 24);
        for i in 0..12 {
            assert_eq!(short.mass_series[i], long.mass_series[i], "sample {}", i);
        }
    }

    #[test]
    fn long_season_pinned_as_found() {
        // Filled in from the first shipped headless run; the numbers
        // are the finding, not a target.
        let d = run_season(Regime::CurvedLongDepleted);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), LONG_DARK);
        assert!((d.final_mass - LONG_MASS).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, LONG_COVERAGE);
        let f = run_season(Regime::OpposedLongDepleted);
        assert_eq!(f.cities.iter().filter(|c| !c.lit).count(), FLAT_LONG_DARK);
    }

    #[test]
    fn far_rooms_truncate_to_the_long_seasons() {
        // A far room's first 2,400 ticks are the long room's whole
        // season, sample for sample: length is the only difference.
        let long = run_season(Regime::CurvedLongDepleted);
        let far = run_season(Regime::CurvedFarDepleted);
        assert_eq!(long.first_dark, far.first_dark);
        assert_eq!(long.mass_series.len(), 24);
        assert_eq!(far.mass_series.len(), 48);
        for i in 0..24 {
            assert_eq!(long.mass_series[i], far.mass_series[i], "sample {}", i);
        }
    }

    #[test]
    fn far_season_pinned_as_found() {
        // Filled in from the first shipped headless run; the numbers
        // are the finding, not a target.
        let d = run_season(Regime::CurvedFarDepleted);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), FAR_DARK);
        assert!((d.final_mass - FAR_MASS).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, FAR_COVERAGE);
        let f = run_season(Regime::OpposedFarDepleted);
        assert_eq!(f.cities.iter().filter(|c| !c.lit).count(), FLAT_FAR_DARK);
    }

    #[test]
    fn last_rooms_truncate_to_the_far_seasons() {
        // A last room's first 4,800 ticks are the far room's whole
        // season, sample for sample: length is the only difference.
        let far = run_season(Regime::CurvedFarDepleted);
        let last = run_season(Regime::CurvedLastDepleted);
        assert_eq!(far.first_dark, last.first_dark);
        assert_eq!(far.mass_series.len(), 48);
        assert_eq!(last.mass_series.len(), 72);
        for i in 0..48 {
            assert_eq!(far.mass_series[i], last.mass_series[i], "sample {}", i);
        }
    }

    #[test]
    fn last_season_pinned_as_found() {
        // Filled in from the first shipped headless run; the numbers
        // are the finding, not a target.
        let d = run_season(Regime::CurvedLastDepleted);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), LAST_DARK);
        assert!((d.final_mass - LAST_MASS).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, LAST_COVERAGE);
        let f = run_season(Regime::OpposedLastDepleted);
        assert_eq!(f.cities.iter().filter(|c| !c.lit).count(), FLAT_LAST_DARK);
    }

    #[test]
    fn past_rooms_truncate_to_the_last_seasons() {
        // A past room's first 7,200 ticks are the last room's whole
        // season, sample for sample: length is the only difference.
        let last = run_season(Regime::CurvedLastDepleted);
        let past = run_season(Regime::CurvedPastDepleted);
        assert_eq!(last.first_dark, past.first_dark);
        assert_eq!(last.mass_series.len(), 72);
        assert_eq!(past.mass_series.len(), 96);
        for i in 0..72 {
            assert_eq!(last.mass_series[i], past.mass_series[i], "sample {}", i);
        }
    }

    #[test]
    fn migration_slices_sum_to_the_window() {
        // The slice instrument is exact: per city, the eight slices
        // sum to the past window's ghost count (past minus last),
        // and the last slice is the final-300 ledger itself.
        let past = run_season(Regime::CurvedPastDepleted);
        let last = run_season(Regime::CurvedLastDepleted);
        assert_eq!(past.ghost_slices.len(), 8);
        for i in 0..9 {
            let sum: u64 = past.ghost_slices.iter().map(|sl| sl[i]).sum();
            assert_eq!(sum, past.cities[i].ghost_visits - last.cities[i].ghost_visits);
            assert_eq!(past.ghost_slices[7][i], past.ghost_last300_cities[i]);
        }
        let total: u64 = past.ghost_slices.iter().flat_map(|sl| sl.iter()).sum();
        assert_eq!(total, 9976);
    }

    #[test]
    fn migration_slices_pinned_as_found() {
        // Filled in from the first shipped headless run.
        let past = run_season(Regime::CurvedPastDepleted);
        let c8: Vec<u64> = past.ghost_slices.iter().map(|sl| sl[8]).collect();
        let c1: Vec<u64> = past.ghost_slices.iter().map(|sl| sl[1]).collect();
        let c5: Vec<u64> = past.ghost_slices.iter().map(|sl| sl[5]).collect();
        assert_eq!(c8, MIGRATION_C8);
        assert_eq!(c1, MIGRATION_C1);
        assert_eq!(c5, MIGRATION_C5);
    }

    #[test]
    fn past_season_pinned_as_found() {
        // Filled in from the first shipped headless run; the numbers
        // are the finding, not a target.
        let d = run_season(Regime::CurvedPastDepleted);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), PAST_DARK);
        assert!((d.final_mass - PAST_MASS).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, PAST_COVERAGE);
        let f = run_season(Regime::OpposedPastDepleted);
        assert_eq!(f.cities.iter().filter(|c| !c.lit).count(), FLAT_PAST_DARK);
    }

    #[test]
    fn curved_season_pinned_as_found() {
        // Filled in from the first shipped headless run; the numbers
        // are the finding, not a target.
        let e = run_season(Regime::CurvedOpposedEternal);
        let (lv, _, _, _, _) = class_stats(&e.cities, true);
        let (qv, _, _, _, _) = class_stats(&e.cities, false);
        assert_eq!(lv, CURVED_ETERNAL_LOUD_VISITS);
        assert_eq!(qv, CURVED_ETERNAL_QUIET_VISITS);
        assert!((e.final_mass - CURVED_ETERNAL_MASS).abs() < 0.5, "mass {}", e.final_mass);
        assert_eq!(e.final_coverage, CURVED_ETERNAL_COVERAGE);
        let d = run_season(Regime::CurvedOpposedDepleted);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), CURVED_DEPLETED_DARK);
        assert!((d.final_mass - CURVED_DEPLETED_MASS).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, CURVED_DEPLETED_COVERAGE);
    }
}
