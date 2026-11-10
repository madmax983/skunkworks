//! physarum-appetite 🗺️
//!
//! Revival cross: graveyard/physarum-transit × experiments/physarum-quality.
//!
//! The corpse (physarum-transit, executed for Documentation Void and
//! Ancestral Redundancy against hyperbolic-mold) ran the standard
//! Physarum agent model on a flat toroidal plane: agents sense the
//! trail field at ±45° / 9 px, rotate ±22.5° toward the strongest
//! sensor, advance 2.0, deposit 0.5 (cap 1.0); cities stamp their
//! disc to 1.0 every tick, forever; the field diffuses as a 3×3 mean
//! and decays ×0.95. Its cities were infinite and all the same food.
//!
//! The first revival (physarum-ledger) gave the cities finite stock.
//! The second (physarum-quality, the living parent here) priced
//! quality in the *stamp*: rich cities stamped 1.0, plain cities
//! 0.40, both eaten at 0.004 per mouthful. The result was total: the
//! louder food drew 5.6× the visits per city, was eaten out first,
//! was haunted exclusively (43,525 ghost visits, all rich), and the
//! quiet food was never arrived at at all — no plain city ever went
//! dark, because a city nobody visits cannot be spent.
//!
//! The parent's own closing seed asked for the channel to be moved:
//! *quality priced in the eat rate instead of the stamp.* This cross
//! keeps every one of the parent's numbers — same map, same seed,
//! same spawn, same stock, same stamp (every city at the corpse's
//! 1.0) — and moves the parent's 2.5× (1.0 against 0.40) out of
//! the advertisement and into the mouth: in the rate rooms a rich
//! city is eaten at 0.010 per agent per tick, a plain city at the
//! parent's 0.004. The rich food is not louder; it is softer. It
//! burns faster only because mouths spend it faster.
//!
//! Four rooms, one seed, one spawn, one city map (five rich cities
//! — the centre and every second ring city — and four plain):
//!   UniformEternal   — the grandparent verbatim, shadow ledger
//!                      only; the baseline this crate must reproduce
//!                      exact.
//!   UniformDepleted  — the grandparent verbatim, stock gating.
//!   RateEternal      — eat-rate quality, stocks in shadow only:
//!                      with no advertisement difference, do the two
//!                      foods draw the same mouths?
//!   RateDepleted     — eat-rate quality, stock gating: which food
//!                      ends first when the only difference is how
//!                      fast it is eaten, and who is haunted?
//!
//! Predictions, declared before the first run (see README):
//!   B1 the uniform rooms reproduce physarum-ledger's pinned ledger
//!      exact (first dark tick, final mass, final coverage).
//!   P1 in RateEternal the two foods draw alike: the per-city mean
//!      visit ratio (rich/plain) stands within [0.70, 1.43] — no
//!      advertisement, no 5.6× reassignment.
//!   P2 in RateDepleted the rich cities go dark earlier on average
//!      than the plain cities (the softer food is spent first, by
//!      the mouth alone).
//!   P3 every city still goes dark in RateDepleted before the
//!      season ends (the plain are visited normally, so unlike the
//!      stamp rooms they *can* be spent).
//!   P4 ghost visits in RateDepleted do not concentrate the way
//!      they did under the stamp: the plain ghost total is above
//!      zero and the rich share of all ghost visits is below 90%
//!      (roads are kept to food that was actually visited).
//!   P5 RateDepleted ends poorer than UniformDepleted in final
//!      trail mass (a faster-burning half of the map spends the
//!      field's season sooner).

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

// ---- this cross's single change: the eat rate is the quality ----
// Every city stamps at the corpse's 1.0 (kept as RICH_STAMP below for
// the stamp step); the parent's 2.5x lives in the mouth instead.
const RICH_STAMP: f32 = 1.0; // the corpse's stamp, for every food
const EAT_RICH: f32 = 0.010; // rich food: eaten at EAT / 0.40
const EAT_PLAIN: f32 = EAT; // plain food: the parent's own rate

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
    RateEternal,
    RateDepleted,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::UniformEternal => "uniform-eternal",
            Regime::UniformDepleted => "uniform-depleted",
            Regime::RateEternal => "rate-eternal",
            Regime::RateDepleted => "rate-depleted",
        }
    }
    fn gates(self) -> bool {
        matches!(self, Regime::UniformDepleted | Regime::RateDepleted)
    }
    fn rate(self) -> bool {
        matches!(self, Regime::RateEternal | Regime::RateDepleted)
    }
    fn eat_rate(self, rich: bool) -> f32 {
        if self.rate() && rich { EAT_RICH } else { EAT_PLAIN }
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
                let rate = self.regime.eat_rate(self.cities[ci].rich);
                let city = &mut self.cities[ci];
                city.visits += feeders;
                if !city.lit {
                    city.ghost_visits += feeders;
                }
                city.stock = (city.stock - rate * feeders as f32).max(0.0);
            }
        }

        // 4. Bookkeeping: darkness. (Regrowth was the grandparent's
        //    third room; this cross holds the season to
        //    eternal/depleted so the eat-rate comparison has no
        //    second moving part.)
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

        // 5. Lit cities stamp their disc — the corpse, gated. No
        //    quality lives here in any room: every lit city stamps
        //    at RICH_STAMP, the corpse's own strength. That silence
        //    is the experiment.
        let stamps: Vec<(f32, f32, f32)> = self
            .cities
            .iter()
            .filter(|c| c.lit || !self.regime.gates())
            .map(|c| (c.x, c.y, RICH_STAMP))
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
    println!("physarum-appetite — headless ledger");
    println!(
        "config: grid {}x{}, agents {}, ticks {}, cities {} (rich 5, plain 4), stock0 {:.1}, stamp {:.2} (all cities), eat_plain {:.3}, eat_rich {:.3} (rate rooms), seed {:#018x}",
        W, H, AGENTS, TICKS, 9, STOCK0, RICH_STAMP, EAT_PLAIN, EAT_RICH, SEED
    );
    let seasons: Vec<Season> = [
        Regime::UniformEternal,
        Regime::UniformDepleted,
        Regime::RateEternal,
        Regime::RateDepleted,
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
    fn eat_rate_is_the_only_difference() {
        // The stamp carries no quality in any room: one step, no
        // agents, rate room — rich and plain cities stamp alike.
        let mut world = World::new(32, 32, 0, Regime::RateEternal, 7);
        world.agents.clear();
        world.step();
        for c in &world.cities {
            let idx = c.y as usize * 32 + c.x as usize;
            assert!(
                (world.grid[idx] - 0.95).abs() < 0.02,
                "city (rich={}) stamp {}",
                c.rich,
                world.grid[idx]
            );
        }
        // The mouth carries it all: park one agent on a rich city and
        // one on a plain city in the rate room and compare the drain.
        let mut w2 = World::new(32, 32, 0, Regime::RateDepleted, 7);
        let rich = w2.cities.iter().position(|c| c.rich).unwrap();
        let plain = w2.cities.iter().position(|c| !c.rich).unwrap();
        let (rx, ry) = (w2.cities[rich].x, w2.cities[rich].y);
        let (px, py) = (w2.cities[plain].x, w2.cities[plain].y);
        w2.agents.push(Agent { x: rx, y: ry, angle: 0.0 });
        w2.agents.push(Agent { x: px, y: py, angle: 0.0 });
        w2.step();
        let rv = w2.cities[rich].visits;
        let pv = w2.cities[plain].visits;
        assert!(rv >= 1 && pv >= 1, "visits rich {} plain {}", rv, pv);
        let r_drain = STOCK0 - w2.cities[rich].stock;
        let p_drain = STOCK0 - w2.cities[plain].stock;
        let ratio = r_drain / p_drain;
        let expect = (EAT_RICH / EAT_PLAIN) * (rv as f32 / pv as f32);
        assert!(
            (ratio - expect).abs() < 0.01,
            "drain ratio {} vs visits-scaled {}",
            ratio,
            expect
        );
        // And in a uniform room the same two cities drain at one rate.
        let mut w3 = World::new(32, 32, 0, Regime::UniformDepleted, 7);
        w3.agents.push(Agent { x: rx, y: ry, angle: 0.0 });
        w3.agents.push(Agent { x: px, y: py, angle: 0.0 });
        w3.step();
        let r3 = STOCK0 - w3.cities[rich].stock;
        let p3 = STOCK0 - w3.cities[plain].stock;
        let v3r = w3.cities[rich].visits as f32;
        let v3p = w3.cities[plain].visits as f32;
        assert!(
            (r3 / v3r - p3 / v3p).abs() < 1e-6,
            "uniform per-visit drain differs: {} vs {}",
            r3 / v3r,
            p3 / v3p
        );
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
    fn rate_season_is_pinned() {
        // The rate season, counted once. Numbers as found on the
        // first run; any drift here is a changed specimen.
        let re = run_season(Regime::RateEternal);
        let rd = run_season(Regime::RateDepleted);
        assert_eq!(re.first_dark, Some(94));
        assert_eq!(rd.first_dark, Some(94), "shadow tick == dark tick");
        // P1 held in its strongest form: with no advertisement the
        // eternal ledger is the uniform ledger, visit for visit —
        // the eat rate is invisible until something can run out.
        let (re_rich_visits, _, re_rich_mean_fd, _, _) = class_stats(&re.cities, true);
        let (re_plain_visits, _, _, _, _) = class_stats(&re.cities, false);
        assert_eq!(re_rich_visits, 117785);
        assert_eq!(re_plain_visits, 100190);
        assert!((re_rich_mean_fd - 118.4).abs() < 0.05, "re rich mean fd {}", re_rich_mean_fd);
        assert!((re.final_mass - 2447.38).abs() < 0.5, "re mass {}", re.final_mass);
        assert_eq!(re.final_coverage, 3685);
        // P2 held: the softer food ends first, by the mouth alone.
        // P3 held: every city darkens — the plain can be spent here.
        let (_, rd_rich_ghost, rd_rich_mean_fd, _, rd_rich_dark) = class_stats(&rd.cities, true);
        let (_, rd_plain_ghost, rd_plain_mean_fd, _, rd_plain_dark) = class_stats(&rd.cities, false);
        assert!((rd_rich_mean_fd - 117.8).abs() < 0.05, "rd rich mean fd {}", rd_rich_mean_fd);
        assert!((rd_plain_mean_fd - 311.75).abs() < 0.01, "rd plain mean fd {}", rd_plain_mean_fd);
        assert_eq!(rd_rich_dark, 5, "all rich dark");
        assert_eq!(rd_plain_dark, 4, "P3 held: all plain dark too");
        assert_eq!(rd.cities.iter().map(|c| c.visits).sum::<u64>(), 109225);
        assert_eq!(rd.cities.iter().map(|c| c.dark_ticks).sum::<u64>(), 8964);
        // P4 held: the haunting is shared — plain ghosts outnumber
        // rich ghosts; the rich share is under half, not over 90%.
        assert_eq!(rd_rich_ghost, 34705);
        assert_eq!(rd_plain_ghost, 36919);
        assert_eq!(rd.ghost_last300, 28702);
        // P5 cracked by a hair: the rate season ends a fraction
        // RICHER than the uniform depleted season (1720.61 against
        // 1713.38) despite 1,417 more dark ticks — pinned as found.
        assert!((rd.final_mass - 1720.61).abs() < 0.5, "rd mass {}", rd.final_mass);
        assert_eq!(rd.final_coverage, 2544);
        assert!(rd.final_mass > 1713.38, "P5 cracked: not poorer, a hair richer");
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
