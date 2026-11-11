//! physarum-opposed 🗺️
//!
//! Revival cross: graveyard/physarum-transit × experiments/physarum-appetite.
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
//! The second (physarum-quality) priced quality in the *stamp* alone
//! (rich 1.0, plain 0.40, both eaten at 0.004): the louder food drew
//! 5.6x the visits per city, was eaten out first, was haunted
//! exclusively, and the quiet food was never arrived at at all. The
//! third (physarum-appetite, the living parent here) priced the same
//! 2.5x in the *mouth* alone (every stamp 1.0; rich eaten at 0.010,
//! plain at 0.004): the visits were the uniform visits visit for
//! visit, the softer food ended first by the mouth alone, every city
//! was spent, and the haunting was shared in proportion to the
//! eating. Two channels, each priced alone, gave opposite ledgers.
//!
//! The parent's closing seed asked what happens when the channels
//! disagree: *both channels at once, opposed — a food loud in the
//! stamp and slow in the mouth against a food quiet and quick: which
//! channel sets the exhaustion order when they disagree?* This cross
//! keeps every one of the lineage's numbers — same map, same seed,
//! same spawn, same stock — and sets the two measured channels
//! against each other on the same cities. The five cities that were
//! rich in both parents (centre and every second ring city) are now
//! LOUD-SLOW: stamp 1.0 (quality's rich stamp), eaten at 0.004
//! (appetite's plain rate). The four that were plain are QUIET-QUICK:
//! stamp 0.40 (quality's plain stamp), eaten at 0.010 (appetite's
//! rich rate). One cross, two organs, opposite signs.
//!
//! Four rooms, one seed, one spawn, one city map:
//!   UniformEternal   — the grandparent verbatim, shadow ledger
//!                      only; the baseline this crate must reproduce
//!                      exact.
//!   UniformDepleted  — the grandparent verbatim, stock gating.
//!   OpposedEternal   — opposed channels, stocks in shadow only:
//!                      does the stamp still reassign the network
//!                      when the mouth argues the other way?
//!   OpposedDepleted  — opposed channels, stock gating: which food
//!                      ends first, and who is haunted?
//!
//! Predictions, declared before the first run (see README):
//!   B1 the uniform rooms reproduce physarum-ledger's pinned ledger
//!      exact (first dark tick, final mass, final coverage).
//!   P1 in OpposedEternal the loud cities draw more visits per city
//!      than the quiet cities, ratio above 2.0 — the stamp reassigns
//!      even with the mouth opposed (the mouth is invisible in an
//!      eternal room by appetite's own law).
//!   P2 in OpposedDepleted the loud cities end first: all five loud
//!      cities go dark, and their mean first-dark tick is below the
//!      quiet class's effective mean, counting a never-dark city as
//!      tick 1200 — exhaustion follows the stamp despite the loud
//!      food being 2.5x slower to eat, because drain is visits x
//!      rate and the stamp sets the visits.
//!   P3 at least one quiet city never goes dark in OpposedDepleted —
//!      a city barely visited cannot be spent even at the quick
//!      rate; being ignored remains the only immortality here.
//!   P4 ghost visits in OpposedDepleted concentrate on the loud
//!      cities: the loud share of all ghost visits exceeds 90%.
//!   P5 OpposedDepleted ends richer than UniformDepleted in final
//!      trail mass — the unspent quiet reserves keep stamping all
//!      season, as they did in the quality rooms.

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

// ---- this cross's change: the two channels, opposed ----
// The stamp carries quality's 2.5x (loud 1.0, quiet 0.40); the mouth
// carries appetite's 2.5x pointed the other way (loud eaten slowly
// at the uniform rate, quiet eaten quickly at EAT / 0.40).
const LOUD_STAMP: f32 = 1.0; // quality's rich stamp, the corpse's own
const QUIET_STAMP: f32 = 0.40; // quality's plain stamp
const EAT_LOUD: f32 = EAT; // slow mouth: the uniform rate 0.004
const EAT_QUIET: f32 = 0.010; // quick mouth: appetite's rich rate

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
    OpposedEternal,
    OpposedDepleted,
}

impl Regime {
    fn name(self) -> &'static str {
        match self {
            Regime::UniformEternal => "uniform-eternal",
            Regime::UniformDepleted => "uniform-depleted",
            Regime::OpposedEternal => "opposed-eternal",
            Regime::OpposedDepleted => "opposed-depleted",
        }
    }
    fn gates(self) -> bool {
        matches!(self, Regime::UniformDepleted | Regime::OpposedDepleted)
    }
    fn opposed(self) -> bool {
        matches!(self, Regime::OpposedEternal | Regime::OpposedDepleted)
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
    // the lineage's map verbatim. The classes alternate around the
    // ring so neither food owns a corner: the centre and every
    // second ring city are loud-slow (five — the cities that were
    // rich in both parents), the rest quiet-quick (four).
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
                let rate = self.regime.eat_rate(self.cities[ci].loud);
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

        // 5. Lit cities stamp their disc — the corpse, gated. In
        //    the opposed rooms the stamp carries quality's channel:
        //    loud cities at 1.0, quiet cities at 0.40. In the uniform
        //    rooms every city stamps at 1.0, the corpse's strength.
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

fn headless() {
    println!("physarum-opposed — headless ledger");
    println!(
        "config: grid {}x{}, agents {}, ticks {}, cities {} (loud-slow 5, quiet-quick 4), stock0 {:.1}, stamp_loud {:.2}, stamp_quiet {:.2} (opposed rooms), eat_loud {:.3}, eat_quiet {:.3} (opposed rooms), seed {:#018x}",
        W, H, AGENTS, TICKS, 9, STOCK0, LOUD_STAMP, QUIET_STAMP, EAT_LOUD, EAT_QUIET, SEED
    );
    let seasons: Vec<Season> = [
        Regime::UniformEternal,
        Regime::UniformDepleted,
        Regime::OpposedEternal,
        Regime::OpposedDepleted,
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
    fn opposed_channels_are_the_only_difference() {
        // The stamp carries quality's channel in the opposed rooms:
        // one step, no agents — a loud city stamps at the corpse's
        // strength, a quiet city at 0.40 (0.95 / 0.38 after the
        // corpse's diffusion and decay, as in physarum-quality).
        let mut world = World::new(32, 32, 0, Regime::OpposedEternal, 7);
        world.agents.clear();
        world.step();
        for c in &world.cities {
            let idx = c.y as usize * 32 + c.x as usize;
            let expect = if c.loud { 0.95 } else { 0.38 };
            assert!(
                (world.grid[idx] - expect).abs() < 0.02,
                "city (loud={}) stamp {} vs {}",
                c.loud,
                world.grid[idx],
                expect
            );
        }
        // And in a uniform room the same two cities stamp alike.
        let mut wu = World::new(32, 32, 0, Regime::UniformEternal, 7);
        wu.agents.clear();
        wu.step();
        for c in &wu.cities {
            let idx = c.y as usize * 32 + c.x as usize;
            assert!((wu.grid[idx] - 0.95).abs() < 0.02, "uniform stamp {}", wu.grid[idx]);
        }
        // The mouth carries appetite's channel, opposed: park one
        // agent on a loud city and one on a quiet city in the opposed
        // room and compare the per-visit drain.
        let mut w2 = World::new(32, 32, 0, Regime::OpposedDepleted, 7);
        let loud = w2.cities.iter().position(|c| c.loud).unwrap();
        let quiet = w2.cities.iter().position(|c| !c.loud).unwrap();
        let (lx, ly) = (w2.cities[loud].x, w2.cities[loud].y);
        let (qx, qy) = (w2.cities[quiet].x, w2.cities[quiet].y);
        w2.agents.push(Agent { x: lx, y: ly, angle: 0.0 });
        w2.agents.push(Agent { x: qx, y: qy, angle: 0.0 });
        w2.step();
        let lv = w2.cities[loud].visits;
        let qv = w2.cities[quiet].visits;
        assert!(lv >= 1 && qv >= 1, "visits loud {} quiet {}", lv, qv);
        let l_per = (STOCK0 - w2.cities[loud].stock) / lv as f32;
        let q_per = (STOCK0 - w2.cities[quiet].stock) / qv as f32;
        assert!((l_per - EAT_LOUD).abs() < 1e-6, "loud per-visit {}", l_per);
        assert!((q_per - EAT_QUIET).abs() < 1e-6, "quiet per-visit {}", q_per);
        assert!(q_per > l_per, "the quiet mouth is the quick one");
    }

    #[test]
    fn uniform_rooms_reproduce_the_parent_exact() {
        // B1: the grandparent physarum-ledger's pinned ledger,
        // reproduced field for field by this crate's uniform rooms.
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
    fn opposed_season_is_pinned() {
        // The opposed season, counted once. Numbers as found on the
        // first run; any drift here is a changed specimen.
        let oe = run_season(Regime::OpposedEternal);
        let od = run_season(Regime::OpposedDepleted);
        assert_eq!(oe.first_dark, Some(193));
        assert_eq!(od.first_dark, Some(193), "shadow tick == dark tick");
        // P1 held, exact: the eternal opposed ledger is the quality
        // ledger visit for visit — the stamp alone sets the eternal
        // network, the opposed mouth leaves no trace on it.
        let (oe_loud, _, oe_loud_mean, _, _) = class_stats(&oe.cities, true);
        let (oe_quiet, _, oe_quiet_mean, _, _) = class_stats(&oe.cities, false);
        assert_eq!(oe_loud, 133024);
        assert_eq!(oe_quiet, 19082);
        let ratio = (oe_loud as f32 / 5.0) / (oe_quiet as f32 / 4.0);
        assert!(ratio > 5.5 && ratio < 5.7, "per-city ratio {}", ratio);
        assert!((oe_loud_mean - 266.2).abs() < 0.05, "oe loud shadow mean {}", oe_loud_mean);
        assert!((oe_quiet_mean - 779.67).abs() < 0.05, "oe quiet shadow mean {}", oe_quiet_mean);
        assert!((oe.final_mass - 2483.59).abs() < 0.5, "oe mass {}", oe.final_mass);
        assert_eq!(oe.final_coverage, 3854);
        // P2 held: every loud city darkens, at the quality rooms'
        // own mean — the slow mouth did not delay them at all.
        let (_, od_loud_ghost, od_loud_mean, _, od_loud_dark) = class_stats(&od.cities, true);
        let (_, od_quiet_ghost, od_quiet_mean, _, od_quiet_dark) = class_stats(&od.cities, false);
        assert!((od_loud_mean - 268.2).abs() < 0.05, "od loud mean {}", od_loud_mean);
        assert_eq!(od_loud_dark, 5, "all loud dark");
        // P3 held, by one city: three quiet cities ARE spent late
        // (770, 913, 932) by the quick mouth — the stamp alone left
        // all four immortal in the quality rooms — and exactly one,
        // the least visited, is never spent.
        assert_eq!(od_quiet_dark, 3, "three quiet cities spent late");
        assert!((od_quiet_mean - 871.67).abs() < 0.05, "od quiet mean {}", od_quiet_mean);
        assert!(od.cities[8].first_dark.is_none(), "city 8 never darkens");
        assert!((od.cities[8].stock - 4.28).abs() < 0.05, "city 8 stock {}", od.cities[8].stock);
        let quiet_eff = (913.0 + 770.0 + 932.0 + 1200.0) / 4.0;
        assert!(od_loud_mean < quiet_eff, "P2: loud ends first");
        assert_eq!(od.cities.iter().map(|c| c.visits).sum::<u64>(), 94690);
        assert_eq!(od.cities.iter().map(|c| c.dark_ticks).sum::<u64>(), 5644);
        // P4 CRACKED: the haunting concentrates on the loud cities
        // but does not stay there — the late quiet deaths collect
        // their own ghosts (11,883), so the loud share is 77.9%,
        // not the declared 90% and not the stamp rooms' 100%.
        assert_eq!(od_loud_ghost, 41917);
        assert_eq!(od_quiet_ghost, 11883);
        let share = od_loud_ghost as f32 / (od_loud_ghost + od_quiet_ghost) as f32;
        assert!(share > 0.77 && share < 0.79, "loud ghost share {}", share);
        assert_eq!(od.ghost_last300, 15968);
        // P5 held: the opposed season ends richer than uniform —
        // though a little poorer than the quality season (2,251.42),
        // the late quiet deaths costing the field three weak stamps.
        assert!((od.final_mass - 2237.22).abs() < 0.5, "od mass {}", od.final_mass);
        assert_eq!(od.final_coverage, 3410);
        assert!(od.final_mass > 1713.38, "P5: richer than uniform depleted");
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
