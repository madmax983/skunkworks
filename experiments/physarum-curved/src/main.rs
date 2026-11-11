//! physarum-curved 🗺️
//!
//! Revival cross: graveyard/hyperbolic-mold × experiments/physarum-opposed.
//!
//! The corpse at the centre of this cross is not physarum-transit this
//! time but its hyperbolic descendant. `hyperbolic-mold` (physarum-transit
//! × hyperbolic-browser) put the slime on a Poincaré disk: agents as
//! complex points in the unit disk, movement and sensing by Möbius
//! addition, pheromones diffusing on the projected 2D map. It was
//! executed for **Skeletal Implementation / Terminal Genericism /
//! Identity Crisis** — 216 lines, a macroquad window, unseeded `rand`,
//! no stocks, no foods, no ledger: the disk was a rendering, not a
//! country. It died, in the transit certificate's own irony, as the
//! "fitter form" that had condemned its flat parent — and then never
//! measured anything the flat parent had not.
//!
//! The living parent, `physarum-opposed`, supplies the exact missing
//! organs: finite stock, two foods priced in opposed channels (loud-slow:
//! stamp 1.0, eaten 0.004; quiet-quick: stamp 0.40, eaten 0.010), a
//! seeded SplitMix stream, and a pinned ledger. Its law: the stamp sets
//! the exhaustion order (loud mean dark 268.2 against the slow mouth's
//! 2.5× headwind), the mouth sets the exceptions (three quiet cities
//! spent late; one, the least visited, immortal at 4.28 unspent), the
//! haunting 77.9% loud. Its closing seed, taken up verbatim: *the same
//! two foods on the hyperbolic descendant's own ground — does curvature
//! tax the stamp channel, the mouth channel, or both?*
//!
//! This cross keeps the parent's flat rooms verbatim as controls (they
//! must reproduce the parent's ledger exact) and replants the opposed
//! rooms on the mold's own ground: the unit disk, Möbius movement and
//! sensing, diffusion on the projection, the mold's boundary rule
//! (a step that would leave |z|² < 0.98 turns the agent around).
//! The nine cities stand at the parent's fractional addresses read as
//! disk coordinates scaled by 0.85, classes unchanged stone for stone.
//!
//! Six rooms, one seed, one city map, two grounds:
//!   UniformEternal / UniformDepleted   — flat, the grandparent verbatim.
//!   OpposedEternal / OpposedDepleted   — flat, the parent verbatim.
//!   CurvedOpposedEternal               — opposed channels on the disk,
//!                                        stocks in shadow only.
//!   CurvedOpposedDepleted              — opposed channels on the disk,
//!                                        stock gating: who ends first
//!                                        where the roads curve?
//!
//! Predictions, declared before the first run (see README):
//!   B1 the flat uniform rooms reproduce physarum-ledger's pinned
//!      ledger exact. B2 the flat opposed rooms reproduce
//!      physarum-opposed's pinned ledger exact.
//!   P1 curvature taxes the stamp: the curved eternal per-city visit
//!      ratio (loud/quiet) falls strictly below the flat 5.58, while
//!      staying above 1.5.
//!   P2 the order survives: in CurvedOpposedDepleted the loud class's
//!      mean first-dark tick stays below the quiet class's effective
//!      mean (never-dark counted as 1200).
//!   P3 immortality survives: at least one quiet city never goes dark
//!      in CurvedOpposedDepleted.
//!   P4 the void drinks: CurvedOpposedDepleted's final trail mass is
//!      below the flat room's 2237.22.
//!   P5 the centre is untaxed: city 0 (loud, at the disk's centre)
//!      first-dark in CurvedOpposedDepleted lands within ±40% of its
//!      flat tick 195.
//!   P6 the haunting spreads: the loud share of ghost visits in
//!      CurvedOpposedDepleted falls below the flat 77.9%.

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Regime {
    UniformEternal,
    UniformDepleted,
    OpposedEternal,
    OpposedDepleted,
    CurvedOpposedEternal,
    CurvedOpposedDepleted,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::UniformEternal => "uniform-eternal",
            Regime::UniformDepleted => "uniform-depleted",
            Regime::OpposedEternal => "opposed-eternal",
            Regime::OpposedDepleted => "opposed-depleted",
            Regime::CurvedOpposedEternal => "curved-opposed-eternal",
            Regime::CurvedOpposedDepleted => "curved-opposed-depleted",
        }
    }
    fn gates(self) -> bool {
        matches!(
            self,
            Regime::UniformDepleted | Regime::OpposedDepleted | Regime::CurvedOpposedDepleted
        )
    }
    fn opposed(self) -> bool {
        matches!(
            self,
            Regime::OpposedEternal
                | Regime::OpposedDepleted
                | Regime::CurvedOpposedEternal
                | Regime::CurvedOpposedDepleted
        )
    }
    fn curved(self) -> bool {
        matches!(self, Regime::CurvedOpposedEternal | Regime::CurvedOpposedDepleted)
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
}

fn run_season(regime: Regime) -> Season {
    let mut world = World::new(W, H, AGENTS, regime, SEED);
    let mut mass_series = Vec::new();
    let mut mass_at_first_dark = None;
    let mut ghost_at_900 = 0u64;
    for t in 0..TICKS {
        world.step();
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
        if t + 1 == 900 {
            ghost_at_900 = world.cities.iter().map(|c| c.ghost_visits).sum();
        }
    }
    let ghost_total: u64 = world.cities.iter().map(|c| c.ghost_visits).sum();
    Season {
        regime,
        mass_series,
        first_dark: world.first_dark(),
        mass_at_first_dark,
        final_mass: world.mass(),
        final_coverage: world.coverage(),
        cities: world.cities.clone(),
        ghost_last300: ghost_total - ghost_at_900,
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

fn effective_mean_fd(cities: &[City], loud: bool) -> f32 {
    // Mean first-dark tick counting a never-dark city as TICKS —
    // the parent's P2 yardstick.
    let sel: Vec<&City> = cities.iter().filter(|c| c.loud == loud).collect();
    let sum: usize = sel
        .iter()
        .map(|c| c.first_dark.unwrap_or(TICKS))
        .sum();
    sum as f32 / sel.len() as f32
}

fn headless() {
    println!("physarum-curved — headless ledger");
    println!(
        "config: grid {}x{}, agents {}, ticks {}, cities {} (loud-slow 5, quiet-quick 4), stock0 {:.1}, stamp_loud {:.2}, stamp_quiet {:.2} (opposed rooms), eat_loud {:.3}, eat_quiet {:.3} (opposed rooms), seed {:#018x}; curved rooms: poincare disk, sensor_h {:.3}, move_h {:.3}, boundary |z|^2 < {:.2}, city_scale {:.2}",
        W, H, AGENTS, TICKS, 9, STOCK0, LOUD_STAMP, QUIET_STAMP, EAT_LOUD, EAT_QUIET, SEED,
        SENSOR_DIST_H, MOVE_H, DISK_R2_MAX, CITY_SCALE
    );
    let seasons: Vec<Season> = [
        Regime::UniformEternal,
        Regime::UniformDepleted,
        Regime::OpposedEternal,
        Regime::OpposedDepleted,
        Regime::CurvedOpposedEternal,
        Regime::CurvedOpposedDepleted,
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
                label, v, g, mean_fd, effective_mean_fd(&s.cities, loud), min_fd, dark_end
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
            "  mass_series[100..1200]: {}",
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
    // asserts the flat uniform one):
    for (e, d) in [
        (&seasons[0], &seasons[1]),
        (&seasons[2], &seasons[3]),
        (&seasons[4], &seasons[5]),
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
        let mut world = World::new(32, 32, 0, Regime::UniformEternal, 7);
        let a = Agent { x: 16.0, y: 16.0, angle: 0.0 };
        world.grid[16 * 32 + 25] = 1.0; // centre sensor at (25,16)
        let l = world.sense(&a, -SENSOR_ANGLE);
        let c = world.sense(&a, 0.0);
        let r = world.sense(&a, SENSOR_ANGLE);
        assert!(c > l && c > r);
    }

    #[test]
    fn agent_moves_and_deposits_flat() {
        let mut world = World::new(32, 32, 0, Regime::UniformEternal, 7);
        world.agents.push(Agent { x: 10.0, y: 10.0, angle: 0.0 });
        world.cities.clear();
        world.step();
        let a = world.agents[0];
        assert!(a.x > 10.0, "agent advanced: {}", a.x);
        assert!(world.mass() > 0.0, "a deposit survived diffusion");
    }

    #[test]
    fn lit_city_stamps_dark_city_does_not_flat() {
        let mut lit_world = World::new(32, 32, 0, Regime::UniformDepleted, 7);
        lit_world.agents.clear();
        lit_world.step();
        let (cx, cy) = (lit_world.cities[0].x, lit_world.cities[0].y);
        let idx = cy as usize * 32 + cx as usize;
        assert!(lit_world.grid[idx] > 0.9, "lit city stamped: {}", lit_world.grid[idx]);

        let mut dark_world = World::new(32, 32, 0, Regime::UniformDepleted, 7);
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
        let mut world = World::new(32, 32, 0, Regime::UniformDepleted, 7);
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
        let mut world = World::new(W, H, 0, Regime::OpposedEternal, SEED);
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
    fn flat_uniform_baseline_reproduces_the_grandparent() {
        let s = run_season(Regime::UniformEternal);
        assert_eq!(s.first_dark, Some(219));
        assert!((s.final_mass - 2447.38).abs() < 0.5, "mass {}", s.final_mass);
        assert_eq!(s.final_coverage, 3685);
        let d = run_season(Regime::UniformDepleted);
        assert_eq!(d.first_dark, Some(219));
        assert!((d.final_mass - 1713.38).abs() < 0.5, "mass {}", d.final_mass);
        assert_eq!(d.final_coverage, 2529);
        assert_eq!(d.ghost_last300, 20387);
    }

    #[test]
    fn flat_opposed_baseline_reproduces_the_parent() {
        let s = run_season(Regime::OpposedEternal);
        let (lv, _, _, _, _) = class_stats(&s.cities, true);
        let (qv, _, _, _, _) = class_stats(&s.cities, false);
        assert_eq!(lv, 133024);
        assert_eq!(qv, 19082);
        assert!((s.final_mass - 2483.59).abs() < 0.5, "mass {}", s.final_mass);
        assert_eq!(s.final_coverage, 3854);
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
