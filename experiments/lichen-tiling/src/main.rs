//! # Lichen Tiling 🗺️
//!
//! Brought by The Cartographer of Lost Things 🗺️. A revival cross:
//! graveyard/hyperbolic-lichen × crates/poincare-disk.
//!
//! The archived hyperbolic-lichen (548 LOC, macroquad, edition 2021)
//! was a complete lichen symbiosis living on a {4,5} Poincaré tiling:
//! Algae cells turn water into energy, Fungus cells turn energy into
//! water, neighbours trade 0.5 of what the other makes across every
//! shared edge, and a cell rich in both (> 20, > 20) sprouts into
//! empty neighbours. It was executed on 2026-02-22 for Documentation
//! Void / Zombie Status alone — no README, a GUI window as its only
//! mouth — and it carried a second, quieter flaw nobody charged it
//! with: its tiling nodes were registered only where a *player*
//! walked (register_node was called from the exploration render
//! path), so the lichen could only ever grow on ground someone had
//! already visited. Its metabolism was never counted, and most of
//! its world never existed.
//!
//! The living poincare-disk donates the organ the corpse lacked:
//! the tiling itself, whole. `TilingConsts::new_4_5()` and
//! `neighbor_transform_a` — the corpse's own imports — generate the
//! full {4,5} ball by breadth-first Möbius stepping, every cell
//! registered before the first tick, no player required. On that
//! ground the corpse's update rule runs verbatim (see `update`:
//! base metabolism −0.05/−0.05, photosynthesis +1.0 energy for
//! −0.2 water, the Fungus inverse, the 0.5 trades, clamps 0..100,
//! sprouting at p = 0.05 into empty neighbours, sprouts born at
//! 5/5). The control is the flat world the lichen never had: a
//! square von-Neumann grid of near-equal cell count, seeded by the
//! same hash rule, run under the identical rule and tick count.
//!
//! The question the ledger answers: what does hyperbolic ground do
//! to a symbiosis? A {4,5} ball is mostly frontier — ring sizes
//! grow geometrically, so most cells sit one step from unclaimed
//! ground — while a square grid is mostly interior. If growth is
//! priced by interface, the two grounds should colonise at very
//! different rates under one identical metabolism, while the trade
//! itself, being purely local, should cost the same per mixed edge
//! on either ground.
//!
//! Adaptations, named: the corpse drew sprout rolls and initial
//! types from thread_rng / a path hash; here a SplitMix64 stream
//! with a fixed seed and the corpse's own hash thresholds
//! (r < 0.2 Algae, r > 0.8 Fungus, else Empty) make every number
//! reproducible. The corpse's cells also carried an `age` field
//! its update never incremented; it is not carried here. The
//! corpse's world grew as explored; both grounds here are built
//! complete before tick 0, and each cell's ring (graph distance
//! from the origin cell) is recorded at build time.
//!
//! Predictions, declared before the first run and never edited:
//! - P1: the tiling colonises more empty cells than the grid over
//!   400 ticks (the frontier-rich ground pays its interface).
//! - P2: both symbionts persist on both grounds at tick 400
//!   (the symbiosis itself is ground-independent).
//! - P3: the colonised frontier reaches a greater ring on the
//!   tiling than on the grid.
//! - P4: trade volume per mixed (Algae–Fungus) edge per tick
//!   agrees between the two grounds within ±25% (the rule is
//!   local; the ground changes the traffic, not the price).
//!
//! Falsification condition: if the grid colonises as much as or
//! more than the tiling (P1 fails), the frontier-pricing account
//! of this symbiosis is wrong and the growth is priced by
//! something the two grounds share — most likely the raw count of
//! rich cells, not the shape of their boundary.
//!
//! `cargo run -p lichen-tiling -- --headless` prints both grounds'
//! ledgers. There is no TUI; --headless is handled before anything
//! else regardless.

use anyhow::Result;
use poincare_disk::{hyperbolic_dist, mobius_add, neighbor_transform_a, Point, TilingConsts};

// -----------------------------------------------------------------
// Deterministic RNG (SplitMix64), as elsewhere in the garden.
// -----------------------------------------------------------------
struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }
}

fn hash_frac(i: u64) -> f32 {
    // The corpse's register_node hash, kept: seed multiplied by the
    // Knuth constant, top 32 bits read as a fraction.
    let seed = (i.wrapping_add(1)).wrapping_mul(6364136223846793005);
    ((seed >> 32) as u32) as f32 / u32::MAX as f32
}

// -----------------------------------------------------------------
// Grounds: a graph of cells with a ring per cell.
// -----------------------------------------------------------------
struct Ground {
    name: &'static str,
    adj: Vec<Vec<usize>>,
    ring: Vec<u32>,
}

fn tiling_ground(rings: u32) -> (Ground, Vec<Point>, f64) {
    let consts = TilingConsts::new_4_5();
    let step = neighbor_transform_a(0, &consts);
    let step_len = hyperbolic_dist(Point::new(0.0, 0.0), step);
    let key = |p: Point| ((p.re * 1e6).round() as i64, (p.im * 1e6).round() as i64);
    let mut pos: Vec<Point> = vec![Point::new(0.0, 0.0)];
    let mut ring: Vec<u32> = vec![0];
    let mut seen = std::collections::HashMap::new();
    seen.insert(key(pos[0]), 0usize);
    let mut frontier = vec![0usize];
    for r in 1..=rings {
        let mut next = Vec::new();
        for &i in &frontier {
            for d in 0..4 {
                let a = neighbor_transform_a(d, &consts);
                let cand = mobius_add(pos[i], a);
                if cand.norm() >= 1.0 {
                    continue;
                }
                let k = key(cand);
                if !seen.contains_key(&k) {
                    seen.insert(k, pos.len());
                    pos.push(cand);
                    ring.push(r);
                    next.push(pos.len() - 1);
                }
            }
        }
        frontier = next;
    }
    let n = pos.len();
    let mut adj = vec![Vec::new(); n];
    for i in 0..n {
        for j in (i + 1)..n {
            if hyperbolic_dist(pos[i], pos[j]) < step_len * 1.05 {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }
    (Ground { name: "tiling {4,5}", adj, ring }, pos, step_len)
}

fn grid_ground(side: usize) -> Ground {
    let n = side * side;
    let mut adj = vec![Vec::new(); n];
    let idx = |x: usize, y: usize| y * side + x;
    for y in 0..side {
        for x in 0..side {
            if x + 1 < side {
                let (a, b) = (idx(x, y), idx(x + 1, y));
                adj[a].push(b);
                adj[b].push(a);
            }
            if y + 1 < side {
                let (a, b) = (idx(x, y), idx(x, y + 1));
                adj[a].push(b);
                adj[b].push(a);
            }
        }
    }
    // Ring = graph distance from the centre cell (BFS).
    let centre = idx(side / 2, side / 2);
    let mut ring = vec![u32::MAX; n];
    ring[centre] = 0;
    let mut q = std::collections::VecDeque::from([centre]);
    while let Some(i) = q.pop_front() {
        for &j in &adj[i] {
            if ring[j] == u32::MAX {
                ring[j] = ring[i] + 1;
                q.push_back(j);
            }
        }
    }
    Ground { name: "grid square", adj, ring }
}

// -----------------------------------------------------------------
// The corpse's symbiosis, verbatim in its numbers.
// -----------------------------------------------------------------
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Empty,
    Algae,
    Fungus,
}

#[derive(Clone, Copy)]
struct Cell {
    kind: Kind,
    energy: f32,
    water: f32,
}

struct Sim {
    cells: Vec<Cell>,
    colonized: u64,
    trade: f64,
}

fn seed_cells(adj_len: usize) -> Vec<Cell> {
    (0..adj_len)
        .map(|i| {
            let r = hash_frac(i as u64);
            let kind = if r < 0.2 {
                Kind::Algae
            } else if r > 0.8 {
                Kind::Fungus
            } else {
                Kind::Empty
            };
            Cell { kind, energy: 10.0, water: 10.0 }
        })
        .collect()
}

impl Sim {
    fn new(n: usize) -> Self {
        Sim { cells: seed_cells(n), colonized: 0, trade: 0.0 }
    }

    fn update(&mut self, adj: &[Vec<usize>], rng: &mut Rng, sprout_p: f32) {
        let n = self.cells.len();
        let mut de = vec![0.0f32; n];
        let mut dw = vec![0.0f32; n];
        for i in 0..n {
            let cell = self.cells[i];
            let mut e = -0.05f32;
            let mut w = -0.05f32;
            match cell.kind {
                Kind::Algae => {
                    if cell.water > 1.0 {
                        e += 1.0;
                        w -= 0.2;
                    }
                    for &nb in &adj[i] {
                        if self.cells[nb].kind == Kind::Fungus {
                            e -= 0.5;
                            de[nb] += 0.5;
                            self.trade += 0.5;
                        }
                    }
                }
                Kind::Fungus => {
                    if cell.energy > 1.0 {
                        w += 1.0;
                        e -= 0.2;
                    }
                    for &nb in &adj[i] {
                        if self.cells[nb].kind == Kind::Algae {
                            w -= 0.5;
                            dw[nb] += 0.5;
                            self.trade += 0.5;
                        }
                    }
                }
                Kind::Empty => {}
            }
            de[i] += e;
            dw[i] += w;
        }
        for i in 0..n {
            self.cells[i].energy = (self.cells[i].energy + de[i]).clamp(0.0, 100.0);
            self.cells[i].water = (self.cells[i].water + dw[i]).clamp(0.0, 100.0);
        }
        // Colonization: a cell rich in both sprouts into empty
        // neighbours; sprouts are collected, then applied in order.
        let mut sprouts: Vec<(usize, Kind)> = Vec::new();
        for i in 0..n {
            let cell = self.cells[i];
            if cell.kind != Kind::Empty && cell.energy > 20.0 && cell.water > 20.0 {
                for &nb in &adj[i] {
                    if self.cells[nb].kind == Kind::Empty && rng.next_f32() < sprout_p {
                        sprouts.push((nb, cell.kind));
                    }
                }
            }
        }
        for (nb, kind) in sprouts {
            if self.cells[nb].kind == Kind::Empty {
                self.cells[nb] = Cell { kind, energy: 5.0, water: 5.0 };
                self.colonized += 1;
            }
        }
    }
}

// -----------------------------------------------------------------
// Ledger
// -----------------------------------------------------------------
struct Ledger {
    cells: usize,
    edges: usize,
    mean_degree: f64,
    mixed_edges: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
    colonized: u64,
    trade: f64,
    mixed_edge_ticks: u64,
    max_ring: u32,
    outer_ring_share: f64,
    frontier_ring: u32,
    mean_energy: f64,
    mean_water: f64,
}

fn run_ground(g: &Ground, ticks: u32) -> Ledger {
    let n = g.adj.len();
    let edges: usize = g.adj.iter().map(|a| a.len()).sum::<usize>() / 2;
    let mut sim = Sim::new(n);
    let count = |cells: &[Cell], k: Kind| cells.iter().filter(|c| c.kind == k).count();
    let start = (
        count(&sim.cells, Kind::Algae),
        count(&sim.cells, Kind::Fungus),
        count(&sim.cells, Kind::Empty),
    );
    let was_empty: Vec<bool> = sim.cells.iter().map(|c| c.kind == Kind::Empty).collect();
    let mixed_edges = (0..n)
        .flat_map(|i| g.adj[i].iter().map(move |&j| (i, j)))
        .filter(|&(i, j)| {
            i < j
                && ((sim.cells[i].kind == Kind::Algae && sim.cells[j].kind == Kind::Fungus)
                    || (sim.cells[i].kind == Kind::Fungus && sim.cells[j].kind == Kind::Algae))
        })
        .count();
    let mut rng = Rng(0x4C49_4348_454E);
    let mut mixed_edge_ticks: u64 = 0;
    for _ in 0..ticks {
        mixed_edge_ticks += (0..n)
            .flat_map(|i| g.adj[i].iter().map(move |&j| (i, j)))
            .filter(|&(i, j)| {
                i < j
                    && ((sim.cells[i].kind == Kind::Algae && sim.cells[j].kind == Kind::Fungus)
                        || (sim.cells[i].kind == Kind::Fungus && sim.cells[j].kind == Kind::Algae))
            })
            .count() as u64;
        sim.update(&g.adj, &mut rng, 0.05);
    }
    let max_ring = *g.ring.iter().max().unwrap_or(&0);
    let outer_ring_share = g.ring.iter().filter(|&&r| r == max_ring).count() as f64 / n as f64;
    let end = (
        count(&sim.cells, Kind::Algae),
        count(&sim.cells, Kind::Fungus),
        count(&sim.cells, Kind::Empty),
    );
    let frontier_ring = (0..n)
        .filter(|&i| was_empty[i] && sim.cells[i].kind != Kind::Empty)
        .map(|i| g.ring[i])
        .max()
        .unwrap_or(0);
    let typed: Vec<&Cell> = sim.cells.iter().filter(|c| c.kind != Kind::Empty).collect();
    let (mean_energy, mean_water) = if typed.is_empty() {
        (0.0, 0.0)
    } else {
        (
            typed.iter().map(|c| c.energy as f64).sum::<f64>() / typed.len() as f64,
            typed.iter().map(|c| c.water as f64).sum::<f64>() / typed.len() as f64,
        )
    };
    Ledger {
        cells: n,
        edges,
        mean_degree: 2.0 * edges as f64 / n as f64,
        mixed_edges,
        start,
        end,
        colonized: sim.colonized,
        trade: sim.trade,
        mixed_edge_ticks,
        max_ring,
        outer_ring_share,
        frontier_ring,
        mean_energy,
        mean_water,
    }
}

fn print_ledger(l: &Ledger, ticks: u32) {
    println!(
        "  algae {} -> {}, fungus {} -> {}, empty {} -> {}",
        l.start.0, l.end.0, l.start.1, l.end.1, l.start.2, l.end.2
    );
    println!("  colonized (empty -> typed conversions): {}", l.colonized);
    println!("  trade volume (0.5 transfers, both directions): {:.1}", l.trade);
    println!(
        "  mixed-edge-ticks over the run: {} (mixed edges at seed: {})",
        l.mixed_edge_ticks, l.mixed_edges
    );
    println!(
        "  trade per mixed-edge-tick: {:.3}",
        if l.mixed_edge_ticks > 0 { l.trade / l.mixed_edge_ticks as f64 } else { 0.0 }
    );
    println!(
        "  colonised frontier ring: {} of max ring {} (max-ring cells are {:.1}% of the ground)",
        l.frontier_ring,
        l.max_ring,
        l.outer_ring_share * 100.0
    );
    println!(
        "  typed-cell means at tick {}: energy {:.2}, water {:.2}",
        ticks, l.mean_energy, l.mean_water
    );
}

fn run_headless() {
    let ticks = 400;
    let (tiling, _pos, step_len) = tiling_ground(5);
    // Grid sized to sit beside the tiling's cell count.
    let side = ((tiling.adj.len() as f64).sqrt().round() as usize).max(2);
    let grid = grid_ground(side);
    println!("lichen-tiling headless: the hyperbolic-lichen symbiosis, counted at last");
    println!("  corpse rule verbatim; {} ticks per ground; sprout p = 0.05; seed 0x4C494348454E", ticks);
    println!(
        "  tiling ground: {} cells, neighbour hyperbolic step {:.4}",
        tiling.adj.len(),
        step_len
    );
    let lt = run_ground(&tiling, ticks);
    println!(
        "{}: {} cells, {} edges, mean degree {:.2}, mixed edges {}",
        tiling.name, lt.cells, lt.edges, lt.mean_degree, lt.mixed_edges
    );
    print_ledger(&lt, ticks);
    let lg = run_ground(&grid, ticks);
    println!(
        "{} ({}x{}): {} cells, {} edges, mean degree {:.2}, mixed edges {}",
        grid.name, side, side, lg.cells, lg.edges, lg.mean_degree, lg.mixed_edges
    );
    print_ledger(&lg, ticks);
    let p1 = lt.colonized > lg.colonized;
    let p2 = lt.end.0 > 0 && lt.end.1 > 0 && lg.end.0 > 0 && lg.end.1 > 0;
    let p3 = lt.frontier_ring >= lg.frontier_ring;
    let (tt, tg) = (
        if lt.mixed_edge_ticks > 0 { lt.trade / lt.mixed_edge_ticks as f64 } else { 0.0 },
        if lg.mixed_edge_ticks > 0 { lg.trade / lg.mixed_edge_ticks as f64 } else { 0.0 },
    );
    let p4 = if tt > 0.0 && tg > 0.0 {
        (tt - tg).abs() / tt.max(tg) <= 0.25
    } else {
        false
    };
    println!("  P1 tiling colonises more ({} > {}): {}", lt.colonized, lg.colonized, if p1 { "HELD" } else { "CRACKED" });
    println!(
        "  P2 both symbionts persist on both grounds (tiling {}/{}, grid {}/{}): {}",
        lt.end.0, lt.end.1, lg.end.0, lg.end.1,
        if p2 { "HELD" } else { "CRACKED" }
    );
    println!("  P3 tiling frontier ring >= grid ({} >= {}): {}", lt.frontier_ring, lg.frontier_ring, if p3 { "HELD" } else { "CRACKED" });
    println!(
        "  P4 trade per mixed-edge-tick within 25% ({:.3} vs {:.3}): {}",
        tt, tg,
        if p4 { "HELD" } else { "CRACKED" }
    );
}

fn main() -> Result<()> {
    // --headless is handled before anything else; there is no TUI.
    run_headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiling_step_is_one_hyperbolic_length() {
        let consts = TilingConsts::new_4_5();
        let origin = Point::new(0.0, 0.0);
        let d0 = hyperbolic_dist(origin, neighbor_transform_a(0, &consts));
        for d in 1..4 {
            let dd = hyperbolic_dist(origin, neighbor_transform_a(d, &consts));
            assert!((dd - d0).abs() < 1e-9, "dir {d}: {dd} vs {d0}");
        }
        assert!(d0 > 0.5 && d0 < 1.5);
    }

    #[test]
    fn tiling_ground_is_connected_and_bounded_degree() {
        let (g, _p, _s) = tiling_ground(5);
        assert!(g.adj.len() > 100, "cells {}", g.adj.len());
        assert!(g.adj.iter().all(|a| a.len() <= 4));
        // BFS rings are consistent: every cell beyond ring 0 has a
        // neighbour one ring closer to the origin.
        for i in 0..g.adj.len() {
            if g.ring[i] > 0 {
                assert!(
                    g.adj[i].iter().any(|&j| g.ring[j] + 1 == g.ring[i]),
                    "cell {i} ring {} has no parent step",
                    g.ring[i]
                );
            }
        }
        // Connected: BFS over adjacency from 0 reaches every cell.
        let mut seen = vec![false; g.adj.len()];
        let mut stack = vec![0usize];
        seen[0] = true;
        while let Some(i) = stack.pop() {
            for &j in &g.adj[i] {
                if !seen[j] {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn corpse_trade_moves_half_units_both_ways() {
        // One Algae beside one Fungus, both stocked: after one tick
        // the algae has paid 0.5 energy and received 0.5 water, the
        // fungus the mirror — on top of their own metabolisms.
        let adj = vec![vec![1], vec![0]];
        let mut sim = Sim {
            cells: vec![
                Cell { kind: Kind::Algae, energy: 10.0, water: 10.0 },
                Cell { kind: Kind::Fungus, energy: 10.0, water: 10.0 },
            ],
            colonized: 0,
            trade: 0.0,
        };
        let mut rng = Rng(7);
        sim.update(&adj, &mut rng, 0.0);
        // Algae: -0.05 base, +1.0 photo, -0.5 trade out, +0.5 water in.
        assert!((sim.cells[0].energy - (10.0 - 0.05 + 1.0 - 0.5)).abs() < 1e-6);
        assert!((sim.cells[0].water - (10.0 - 0.05 - 0.2 + 0.5)).abs() < 1e-6);
        // Fungus: -0.05 base, +1.0 water made, -0.5 trade out, +0.5 energy in.
        assert!((sim.cells[1].water - (10.0 - 0.05 + 1.0 - 0.5)).abs() < 1e-6);
        assert!((sim.cells[1].energy - (10.0 - 0.05 - 0.2 + 0.5)).abs() < 1e-6);
        assert!((sim.trade - 1.0).abs() < 1e-9);
    }

    #[test]
    fn rich_cell_colonises_empty_neighbour() {
        let adj = vec![vec![1], vec![0]];
        let mut sim = Sim {
            cells: vec![
                Cell { kind: Kind::Algae, energy: 50.0, water: 50.0 },
                Cell { kind: Kind::Empty, energy: 10.0, water: 10.0 },
            ],
            colonized: 0,
            trade: 0.0,
        };
        let mut rng = Rng(11);
        sim.update(&adj, &mut rng, 1.0);
        assert_eq!(sim.cells[1].kind, Kind::Algae);
        assert_eq!(sim.colonized, 1);
    }

    #[test]
    fn seeding_matches_corpse_thresholds_in_kind() {
        let cells = seed_cells(1000);
        let a = cells.iter().filter(|c| c.kind == Kind::Algae).count();
        let f = cells.iter().filter(|c| c.kind == Kind::Fungus).count();
        assert!(a > 100 && a < 300, "algae {a}");
        assert!(f > 100 && f < 300, "fungus {f}");
    }

    #[test]
    fn headless_run_is_deterministic_and_colonises() {
        let (g, _p, _s) = tiling_ground(5);
        let a = run_ground(&g, 400);
        let b = run_ground(&g, 400);
        assert_eq!(a.end, b.end);
        assert_eq!(a.colonized, b.colonized);
        assert!(a.colonized > 0, "the lichen must grow somewhere");
        assert_eq!(a.start.0 + a.start.1 + a.start.2, a.cells);
        assert_eq!(a.end.0 + a.end.1 + a.end.2, a.cells);
    }
}
