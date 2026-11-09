//! physarum-quality 🗺️
//!
//! Revival cross: graveyard/physarum-transit × experiments/physarum-ledger.
//!
//! The corpse (physarum-transit, executed for Documentation Void and
//! Ancestral Redundancy against hyperbolic-mold) ran the standard
//! Physarum agent model on a flat toroidal plane: agents sense the
//! trail field at ±45° / 9 px, rotate ±22.5° toward the strongest
//! sensor, advance 2.0, deposit 0.5 (cap 1.0); cities stamp their
//! disc to 1.0 every tick, forever; the field diffuses as a 3×3 mean
//! and decays ×0.95. Its cities were infinite and — the charge this
//! cross adds — its cities were all the same food. Nothing in the
//! corpse, and nothing in the first revival (physarum-ledger, which
//! gave the cities a finite stock), ever priced a difference in
//! quality: every meal stamped at the same strength, so the network
//! was never asked what it does when one food is better than another
//! and both are spent at the same rate.
//!
//! The living parent, physarum-ledger, donates the books: a finite
//! stock per city (25.0), feeding that spends it (0.004 per agent in
//! the apron per tick), a stamp gated on the stock, and the shadow
//! ledger of its Eternal room. This cross keeps every one of the
//! parent's numbers and changes exactly one thing: the stamp's
//! strength is the food's quality. Rich cities stamp their disc to
//! 1.0, as the corpse stamped; plain cities stamp to 0.40. Same
//! stock, same price per mouthful, same map, same seed — the only
//! difference between the two foods is how loudly each advertises.
//!
//! Four rooms, one seed, one spawn, one city map (five rich cities
//! — the centre and every second ring city — and four plain):
//!   UniformEternal   — the parent verbatim, shadow ledger only;
//!                      the baseline this crate must reproduce exact.
//!   UniformDepleted  — the parent verbatim, stock gating the stamp.
//!   QualityEternal   — quality stamps, stocks in shadow only: does
//!                      the louder food draw more mouths when nothing
//!                      can run out?
//!   QualityDepleted  — quality stamps, stock gating: which food
//!                      does the network abandon first, and which
//!                      does it keep visiting after the end?
//!
//! Predictions, declared before the first run (see README):
//!   B1 the uniform rooms reproduce physarum-ledger's pinned ledger
//!      exact (first dark tick, final mass, final coverage).
//!   P1 in QualityEternal the rich cities draw more visits in total
//!      than the plain cities (the advertisement works).
//!   P2 in QualityDepleted the rich cities go dark earlier on
//!      average than the plain cities (the advertisement is spent
//!      first — a louder food is eaten out sooner).
//!   P3 every city still goes dark in QualityDepleted before the
//!      season ends (quality delays the plain, it does not save
//!      them).
//!   P4 ghost visits in QualityDepleted concentrate on the rich
//!      cities: rich ghost total above plain ghost total (the
//!      network keeps the roads to the good food longest).
//!   P5 QualityDepleted ends poorer than UniformDepleted in final
//!      trail mass (the plain stamps' weakness is a tax the whole
//!      field pays, not merely a local dimming).

use std::f32::consts::PI;

// ---- the corpse's constants, verbatim ----
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

// ---- this cross's single change: the stamp is the quality ----
const RICH_STAMP: f32 = 1.0; // the corpse's stamp, for rich food
const PLAIN_STAMP: f32 = 0.40; // the same food, advertised quietly

// ---- the season ----
const W: usize = 96;
const H: usize = 64;
const AGENTS: usize = 1200;
const TICKS: usize = 1200;
const SEED: u64 = 0x5059_5341_5255_4D80; // "PHYSARUM" + 0

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Regime {
    UniformEternal,
    UniformDepleted,
    QualityEternal,
    QualityDepleted,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::UniformEternal => "uniform-eternal",
            Regime::UniformDepleted => "uniform-depleted",
            Regime::QualityEternal => "quality-eternal",
            Regime::QualityDepleted => "quality-depleted",
        }
    }
    fn gates(self) -> bool {
        matches!(self, Regime::UniformDepleted | Regime::QualityDepleted)
    }
    fn quality(self) -> bool {
        matches!(self, Regime::QualityEternal | Regime::QualityDepleted)
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
    rich: bool,
    stock: f32,
    lit: bool,
    visits: u64,
    ghost_visits: u64,
    dark_ticks: u64,
    first_dark: Option<usize>,
}

impl City {
    fn new(x: f32, y: f32, rich: bool) -> Self {
        Self {
            x,
            y,
            rich,
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
    // the parent's map verbatim. Quality alternates around the ring
    // so neither food owns a corner: the centre and every second
    // ring city are rich (five), the rest plain (four).
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

fn sense_on(grid: &[f32], w: usize, h: usize, a: &Agent, offset: f32) -> f32 {
    let ang = a.angle + offset;
    let sx = a.x + ang.cos() * SENSOR_DIST;
    let sy = a.y + ang.sin() * SENSOR_DIST;
    let x = (sx.round() as isize).rem_euclid(w as isize) as usize;
    let y = (sy.round() as isize).rem_euclid(h as isize) as usize;
    grid[y * w + x]
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
        for _ in 0..n_agents {
            agents.push(Agent {
                x: spawn.next_f32() * w as f32,
                y: spawn.next_f32() * h as f32,
                angle: spawn.next_f32() * 2.0 * PI,
            });
        }
        let cities = city_map(w, h)
            .into_iter()
            .map(|(x, y, rich)| City::new(x, y, rich))
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
        sense_on(&self.grid, self.w, self.h, a, offset)
    }

    fn step(&mut self) {
        // 1. Agents sense and move — the corpse's steering, verbatim.
        //    All sensing reads the field as the tick found it.
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
            moved.push(a);
        }
        self.agents = moved;

        // 2. Agents deposit — the corpse's 0.5, capped at 1.0.
        for i in 0..self.agents.len() {
            let a = self.agents[i];
            let x = a.x as usize;
            let y = a.y as usize;
            if x < self.w && y < self.h {
                let idx = y * self.w + x;
                self.grid[idx] = (self.grid[idx] + DEPOSIT).min(1.0);
            }
        }

        // 3. Feeding: agents in a city's apron eat from its stock.
        //    The stock falls in every regime; only gating differs.
        for ci in 0..self.cities.len() {
            let (cx, cy) = (self.cities[ci].x, self.cities[ci].y);
            let mut feeders = 0u64;
            for a in &self.agents {
                let mut dx = (a.x - cx).abs();
                if dx > self.w as f32 / 2.0 {
                    dx = self.w as f32 - dx;
                }
                let mut dy = (a.y - cy).abs();
                if dy > self.h as f32 / 2.0 {
                    dy = self.h as f32 - dy;
                }
                if dx * dx + dy * dy <= FEED_RADIUS2 {
                    feeders += 1;
                }
            }
            if feeders > 0 {
                let city = &mut self.cities[ci];
                city.visits += feeders;
                if !city.lit {
                    city.ghost_visits += feeders;
                }
                city.stock = (city.stock - EAT * feeders as f32).max(0.0);
            }
        }

        // 4. Bookkeeping: darkness. (Regrowth was the parent's third
        //    room; this cross holds the season to eternal/depleted so
        //    the quality comparison has no second moving part.)
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

        // 5. Lit cities stamp their disc — the corpse, gated, and
        //    priced by quality: rich at RICH_STAMP, plain at
        //    PLAIN_STAMP in the quality rooms; every city at
        //    RICH_STAMP in the uniform rooms (the parent verbatim).
        let stamps: Vec<(f32, f32, f32)> = self
            .cities
            .iter()
            .filter(|c| c.lit || !self.regime.gates())
            .map(|c| {
                let v = if self.regime.quality() && !c.rich {
                    PLAIN_STAMP
                } else {
                    RICH_STAMP
                };
                (c.x, c.y, v)
            })
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

        // 6. Diffuse (3×3 mean) and decay ×0.95 — the corpse, verbatim.
        let old = self.grid.clone();
        let (w, h) = (self.w, self.h);
        for y in 0..h {
            for x in 0..w {
                let mut sum = 0.0f32;
                for dy in -1isize..=1 {
                    for dx in -1isize..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        sum += old[ny * w + nx];
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

fn class_stats(cities: &[City], rich: bool) -> (u64, u64, f32, i64, usize) {
    // (visits, ghost visits, mean first_dark, min first_dark, dark at end)
    let sel: Vec<&City> = cities.iter().filter(|c| c.rich == rich).collect();
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

fn headless() {
    println!("physarum-quality — headless ledger");
    println!(
        "config: grid {}x{}, agents {}, ticks {}, cities {} (rich 5, plain 4), stock0 {:.1}, eat {:.3}, rich_stamp {:.2}, plain_stamp {:.2}, seed {:#018x}",
        W, H, AGENTS, TICKS, 9, STOCK0, EAT, RICH_STAMP, PLAIN_STAMP, SEED
    );
    let seasons: Vec<Season> = [
        Regime::UniformEternal,
        Regime::UniformDepleted,
        Regime::QualityEternal,
        Regime::QualityDepleted,
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
        for (label, rich) in [("rich", true), ("plain", false)] {
            let (v, g, mean_fd, min_fd, dark_end) = class_stats(&s.cities, rich);
            println!(
                "  class {}: visits {}, ghost {}, mean_first_dark {:.1}, min_first_dark {}, dark_end {}",
                label, v, g, mean_fd, min_fd, dark_end
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
                if c.rich { "rich" } else { "plain" },
                c.visits,
                c.ghost_visits,
                c.first_dark.map(|t| t as i64).unwrap_or(-1),
                c.dark_ticks,
                c.stock,
                c.lit
            );
        }
    }
    // The parent's identity, in both spellings (printed; a test
    // asserts the uniform one):
    for (e, d) in [(&seasons[0], &seasons[1]), (&seasons[2], &seasons[3])] {
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
    // No TUI in this revival: a window was half the corpse's death
    // certificate. Headless is the only mouth.
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
    fn steering_follows_the_corpse() {
        // A field loud at the centre sensor: keep direction.
        let mut world = World::new(32, 32, 0, Regime::UniformEternal, 7);
        let a = Agent { x: 16.0, y: 16.0, angle: 0.0 };
        world.grid[16 * 32 + 25] = 1.0; // centre sensor at (25,16)
        let before = a.angle;
        let l = world.sense(&a, -SENSOR_ANGLE);
        let c = world.sense(&a, 0.0);
        let r = world.sense(&a, SENSOR_ANGLE);
        assert!(c > l && c > r);
        assert_eq!(before, 0.0);
        // A field loud on the left: the corpse turns left (angle -= rot
        // when right < left).
        let mut world2 = World::new(32, 32, 0, Regime::UniformEternal, 7);
        world2.grid[(16 - 6) * 32 + 22] = 0.0; // placeholder, computed below
        let left_pos = {
            let ang = -SENSOR_ANGLE;
            let sx = (16.0 + ang.cos() * SENSOR_DIST).round() as usize;
            let sy = (16.0 + ang.sin() * SENSOR_DIST).round() as usize;
            (sx, sy)
        };
        world2.grid[left_pos.1 * 32 + left_pos.0] = 1.0;
        let l2 = world2.sense(&a, -SENSOR_ANGLE);
        let c2 = world2.sense(&a, 0.0);
        let r2 = world2.sense(&a, SENSOR_ANGLE);
        assert!(l2 > c2 && l2 > r2, "left {} centre {} right {}", l2, c2, r2);
    }

    #[test]
    fn agent_moves_and_deposits() {
        let mut world = World::new(32, 32, 0, Regime::UniformEternal, 7);
        world.agents.push(Agent { x: 10.0, y: 10.0, angle: 0.0 });
        world.cities.clear();
        world.step();
        let a = world.agents[0];
        assert!(a.x > 10.0, "agent advanced: {}", a.x);
        assert!(world.mass() > 0.0, "a deposit survived diffusion");
    }

    #[test]
    fn lit_city_stamps_dark_city_does_not() {
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
    fn feeding_drains_stock_and_darkens() {
        let mut world = World::new(32, 32, 0, Regime::UniformDepleted, 7);
        // Park ten agents on city 0's apron; empty the other cities'
        // aprons by moving those cities' stock high is unnecessary —
        // only city 0 has feeders.
        let (cx, cy) = (world.cities[0].x, world.cities[0].y);
        for _ in 0..10 {
            world.agents.push(Agent { x: cx, y: cy, angle: 0.0 });
        }
        // Agents move 2.0/tick and may wander off the apron; pin the
        // bookkeeping directly instead: one step's drain is measured
        // from the visits the step itself recorded.
        world.step();
        let city = &world.cities[0];
        assert!(city.visits >= 10, "visits {}", city.visits);
        let expected = STOCK0 - EAT * city.visits as f32;
        assert!((city.stock - expected).abs() < 1e-4, "stock {} vs {}", city.stock, expected);

        // A city whose stock is spent by feeding goes dark.
        let mut world2 = World::new(32, 32, 0, Regime::UniformDepleted, 7);
        world2.cities[0].stock = EAT; // one feeder-tick from empty
        let (cx2, cy2) = (world2.cities[0].x, world2.cities[0].y);
        world2.agents.push(Agent { x: cx2, y: cy2, angle: 0.0 });
        world2.step();
        assert!(!world2.cities[0].lit, "spent city went dark");
        assert!(world2.cities[0].first_dark.is_some());
    }

    #[test]
    fn eternal_shadow_records_but_never_gates() {
        let mut world = World::new(32, 32, 0, Regime::UniformEternal, 7);
        world.cities[0].stock = EAT;
        let (cx, cy) = (world.cities[0].x, world.cities[0].y);
        world.agents.push(Agent { x: cx, y: cy, angle: 0.0 });
        world.step();
        assert_eq!(world.cities[0].stock, 0.0);
        assert!(world.cities[0].first_dark.is_some(), "shadow recorded");
        assert!(world.cities[0].lit, "eternal never gates");
        let idx = cy as usize * 32 + cx as usize;
        assert!(world.grid[idx] > 0.9, "eternal still stamps at zero stock");
    }

    #[test]
    fn quality_stamp_is_the_only_difference() {
        // One step, no agents, quality room: the rich centre stamps
        // at the corpse's strength, a plain city at PLAIN_STAMP —
        // measured after the corpse's diffusion (x0.95 on a disc
        // whose 3x3 neighbourhood is uniformly stamped).
        let mut world = World::new(32, 32, 0, Regime::QualityEternal, 7);
        world.agents.clear();
        world.step();
        let rich = &world.cities[0];
        assert!(rich.rich);
        let idx = rich.y as usize * 32 + rich.x as usize;
        assert!((world.grid[idx] - 0.95).abs() < 0.02, "rich stamp {}", world.grid[idx]);
        let plain = world.cities.iter().find(|c| !c.rich).unwrap();
        let idx = plain.y as usize * 32 + plain.x as usize;
        let expect = PLAIN_STAMP * DECAY;
        assert!(
            (world.grid[idx] - expect).abs() < 0.02,
            "plain stamp {} vs {}",
            world.grid[idx],
            expect
        );
        // The uniform room stamps the same plain city at full
        // strength: quality is a room property, not a city property.
        let mut uni = World::new(32, 32, 0, Regime::UniformEternal, 7);
        uni.agents.clear();
        uni.step();
        let idx = plain.y as usize * 32 + plain.x as usize;
        assert!((uni.grid[idx] - 0.95).abs() < 0.02, "uniform stamp {}", uni.grid[idx]);
    }

    #[test]
    fn uniform_rooms_reproduce_the_parent_exact() {
        // B1: the parent physarum-ledger's pinned ledger, reproduced
        // field for field by this crate's uniform rooms.
        let e = run_season(Regime::UniformEternal);
        let d = run_season(Regime::UniformDepleted);
        assert_eq!(e.first_dark, Some(219));
        assert_eq!(d.first_dark, Some(219), "shadow tick == dark tick");
        assert!((e.final_mass - 2447.38).abs() < 0.5, "eternal mass {}", e.final_mass);
        assert_eq!(e.final_coverage, 3685);
        assert_eq!(e.cities.iter().map(|c| c.visits).sum::<u64>(), 217975);
        assert_eq!(d.cities.iter().filter(|c| !c.lit).count(), 9, "every city darkens");
        assert!((d.final_mass - 1713.38).abs() < 0.5, "depleted mass {}", d.final_mass);
        assert_eq!(d.final_coverage, 2529);
        assert_eq!(d.ghost_last300, 20387, "ghosts in the last 300 ticks");
        assert!(d.final_mass < d.mass_at_first_dark.unwrap(), "spent after the food");
    }

    #[test]
    fn quality_season_is_pinned() {
        // The quality season, counted once. Numbers as found on the
        // first run; any drift here is a changed specimen.
        let qe = run_season(Regime::QualityEternal);
        let qd = run_season(Regime::QualityDepleted);
        assert_eq!(qe.first_dark, Some(193));
        assert_eq!(qd.first_dark, Some(193), "shadow tick == dark tick");
        // P1: the advertisement works — rich visits dwarf plain.
        let (qe_rich_visits, _, _, _, _) = class_stats(&qe.cities, true);
        let (qe_plain_visits, _, _, _, _) = class_stats(&qe.cities, false);
        assert_eq!(qe_rich_visits, 133024);
        assert_eq!(qe_plain_visits, 19082);
        assert!((qe.final_mass - 2483.59).abs() < 0.5, "qe mass {}", qe.final_mass);
        assert_eq!(qe.final_coverage, 3854);
        // P2 held in its strong form; P3 cracked: in QualityDepleted
        // every rich city darkens and no plain city does.
        let (_, qd_rich_ghost, qd_rich_mean_fd, _, qd_rich_dark) = class_stats(&qd.cities, true);
        let (qd_plain_visits, qd_plain_ghost, _, _, qd_plain_dark) = class_stats(&qd.cities, false);
        assert!((qd_rich_mean_fd - 268.2).abs() < 0.05, "rich mean fd {}", qd_rich_mean_fd);
        assert_eq!(qd_rich_dark, 5, "all rich dark");
        assert_eq!(qd_plain_dark, 0, "P3 cracked: no plain city darkens");
        assert_eq!(qd_plain_visits, 14812);
        // P4 held total: every ghost visit is to a rich city.
        assert_eq!(qd_rich_ghost, 43525);
        assert_eq!(qd_plain_ghost, 0);
        assert_eq!(qd.ghost_last300, 9313);
        // P5 cracked, inverted: the quality season ends richer than
        // the uniform depleted season (2251.42 against 1713.38).
        assert!((qd.final_mass - 2251.42).abs() < 0.5, "qd mass {}", qd.final_mass);
        assert_eq!(qd.final_coverage, 3567);
        assert!(qd.final_mass > 1713.38, "the ignored food is a reserve");
    }

    #[test]
    fn ghost_visits_count_only_while_dark() {
        let mut world = World::new(32, 32, 0, Regime::UniformDepleted, 7);
        let (cx, cy) = (world.cities[0].x, world.cities[0].y);
        world.agents.push(Agent { x: cx, y: cy, angle: 0.0 });
        world.step();
        assert_eq!(world.cities[0].ghost_visits, 0, "lit city: no ghosts");
        world.cities[0].stock = 0.0;
        world.cities[0].lit = false;
        world.agents[0].x = cx;
        world.agents[0].y = cy;
        world.step();
        assert!(world.cities[0].ghost_visits >= 1, "dark city: ghosts counted");
    }
}
