//! # ising-crystal
//!
//! A revival cross: graveyard/ising-tide x crates/miller-lattice.
//!
//! The archived ising-tide (839 LOC, wgpu/winit, edition 2021) was a
//! complete 3D Ising ferromagnet — spins on a 64^3 lattice, a WGSL
//! Metropolis update (dE = 2*s*(J*sum(neighbours) + h), flip if
//! dE < 0, else flip with probability exp(-dE/T)), three lattice
//! topologies (simple cubic 6, BCC 8, FCC 12), periodic boundaries —
//! executed for Documentation Void / Dangerous Mystery alone: no
//! README, no headless bypass, a GPU window as its only mouth, so
//! nothing it magnetised was ever counted. Its author even left the
//! confession in the uniforms: temperature 4.5, "Start near Critical
//! Temp" — the critical point of the 3D simple-cubic Ising model
//! (Tc ~ 4.51) — and then never measured a single magnetisation.
//!
//! The living miller-lattice donates the organ the corpse never
//! had: a *grown* lattice. Its Crystal maps a directory tree into
//! 3D integer space — atoms for files and directories, bonds for
//! parent-child. This experiment puts one corpse-rule spin on every
//! atom of a deterministic demo grove and couples spins two ways at
//! once: along the crystal's bonds, and between atoms that sit a
//! Manhattan step apart in the grown space (the corpse's own
//! simple-cubic adjacency, discovered inside the crystal rather
//! than imposed as a grid). The control is the corpse's home
//! ground: a periodic simple-cubic block run under the identical
//! rule, seed discipline, and temperature sweep.
//!
//! The question the ledger answers: does a ferromagnet grown on a
//! codebase crystal order at the same temperature as one on a
//! regular lattice — or does the crystal's irregular coordination
//! (leaves that dangle by one or two bonds, hubs where a directory
//! holds court) move the transition and roughen it?
//!
//! Adaptations, named: the corpse updated synchronously on a GPU
//! double buffer; here sweeps are sequential in place (the standard
//! Metropolis the shader's formula describes, one spin at a time).
//! The corpse drew from a per-frame hash; here a SplitMix64 stream
//! with a fixed seed per (graph, temperature) makes every number in
//! the ledger reproducible. J = 1, h = 0 throughout, as shipped.
//!
//! `cargo run -p ising-crystal -- --headless` prints the ledger.
//! --headless is handled before anything else; there is no TUI.

use anyhow::Result;
use miller_lattice::Crystal;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Deterministic SplitMix64 stream.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn spin(&mut self) -> i8 {
        if self.next_u64() & 1 == 0 {
            1
        } else {
            -1
        }
    }
}

/// An undirected neighbour graph: the ground a ferromagnet lives on.
struct Graph {
    name: &'static str,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn n(&self) -> usize {
        self.adj.len()
    }
    fn edges(&self) -> usize {
        self.adj.iter().map(|a| a.len()).sum::<usize>() / 2
    }
    fn mean_degree(&self) -> f64 {
        if self.n() == 0 {
            0.0
        } else {
            2.0 * self.edges() as f64 / self.n() as f64
        }
    }
    fn leaves(&self) -> usize {
        self.adj.iter().filter(|a| a.len() <= 1).count()
    }
    fn max_degree(&self) -> usize {
        self.adj.iter().map(|a| a.len()).max().unwrap_or(0)
    }
}

fn graph_from_edges(name: &'static str, n: usize, edges: &[(usize, usize)]) -> Graph {
    let mut sets: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for &(a, b) in edges {
        if a != b {
            sets[a].insert(b);
            sets[b].insert(a);
        }
    }
    Graph {
        name,
        adj: sets.into_iter().map(|s| s.into_iter().collect()).collect(),
    }
}

/// The crystal ground: bonds union spatial (Manhattan-1) adjacency.
fn crystal_graph(crystal: &Crystal) -> (Graph, Graph) {
    let n = crystal.atoms.len();
    let bonds_only = graph_from_edges("crystal bonds only", n, &crystal.bonds);
    let mut edges: Vec<(usize, usize)> = crystal.bonds.clone();
    for (i, atom) in crystal.atoms.iter().enumerate() {
        for (dx, dy, dz) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
            let p = miller_lattice::LatticePoint::new(
                atom.position.x + dx,
                atom.position.y + dy,
                atom.position.z + dz,
            );
            if let Some(&j) = crystal.lookup.get(&p) {
                edges.push((i, j));
            }
        }
    }
    (
        graph_from_edges("crystal (bonds + spatial)", n, &edges),
        bonds_only,
    )
}

/// The corpse's home ground: periodic simple cubic, L^3 spins.
fn cubic_graph(l: usize) -> Graph {
    let idx = |x: usize, y: usize, z: usize| (z * l + y) * l + x;
    let mut edges = Vec::new();
    for z in 0..l {
        for y in 0..l {
            for x in 0..l {
                edges.push((idx(x, y, z), idx((x + 1) % l, y, z)));
                edges.push((idx(x, y, z), idx(x, (y + 1) % l, z)));
                edges.push((idx(x, y, z), idx(x, y, (z + 1) % l)));
            }
        }
    }
    graph_from_edges("cubic control (periodic)", l * l * l, &edges)
}

/// The corpse's Metropolis rule, one spin at a time.
struct Ising {
    spins: Vec<i8>,
    accepted: u64,
    proposed: u64,
}

impl Ising {
    fn new(graph: &Graph, rng: &mut Rng) -> Self {
        Self {
            spins: (0..graph.n()).map(|_| rng.spin()).collect(),
            accepted: 0,
            proposed: 0,
        }
    }
    fn sweep(&mut self, graph: &Graph, temperature: f64, rng: &mut Rng) {
        for i in 0..graph.n() {
            let s = self.spins[i];
            let sum: i32 = graph.adj[i].iter().map(|&j| self.spins[j] as i32).sum();
            // The shader's own formula: dE = 2 * S_i * (J * sum + h), J=1, h=0.
            let d_e = 2.0 * s as f64 * sum as f64;
            self.proposed += 1;
            let flip = if d_e < 0.0 {
                true
            } else {
                rng.next_f64() < (-d_e / temperature).exp()
            };
            if flip {
                self.spins[i] = -s;
                self.accepted += 1;
            }
        }
    }
    fn magnetisation(&self) -> f64 {
        self.spins.iter().map(|&s| s as f64).sum::<f64>() / self.spins.len() as f64
    }
    /// Energy per spin: -J * sum over edges of s_i*s_j / N.
    fn energy_per_spin(&self, graph: &Graph) -> f64 {
        let mut e = 0.0;
        for i in 0..graph.n() {
            for &j in &graph.adj[i] {
                if j > i {
                    e -= (self.spins[i] * self.spins[j]) as f64;
                }
            }
        }
        e / graph.n() as f64
    }
}

#[derive(Clone, Copy)]
struct Point {
    temperature: f64,
    mean_abs_m: f64,
    m_std: f64,
    energy: f64,
    accept: f64,
}

const SWEEP_T: &[f64] = &[
    6.0, 5.5, 5.0, 4.75, 4.5, 4.25, 4.0, 3.75, 3.5, 3.25, 3.0, 2.75, 2.5, 2.25, 2.0, 1.75, 1.5, 1.25,
    1.0,
];
const EQUIL: usize = 250;
const MEASURE: usize = 250;

fn seed_for(name: &str, temperature: f64) -> u64 {
    let mut h: u64 = 0xC0FF_EE15_1A6D_0001;
    for b in name.bytes() {
        h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3);
    }
    h ^ ((temperature * 1000.0) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

fn run_at(graph: &Graph, temperature: f64) -> Point {
    let mut rng = Rng::new(seed_for(graph.name, temperature));
    let mut ising = Ising::new(graph, &mut rng);
    for _ in 0..EQUIL {
        ising.sweep(graph, temperature, &mut rng);
    }
    let (mut sm, mut sm2, mut se) = (0.0, 0.0, 0.0);
    let (a0, p0) = (ising.accepted, ising.proposed);
    for _ in 0..MEASURE {
        ising.sweep(graph, temperature, &mut rng);
        let m = ising.magnetisation().abs();
        sm += m;
        sm2 += m * m;
        se += ising.energy_per_spin(graph);
    }
    let mean = sm / MEASURE as f64;
    Point {
        temperature,
        mean_abs_m: mean,
        m_std: (sm2 / MEASURE as f64 - mean * mean).max(0.0).sqrt(),
        energy: se / MEASURE as f64,
        accept: (ising.accepted - a0) as f64 / (ising.proposed - p0) as f64,
    }
}

fn run_sweep(graph: &Graph) -> Vec<Point> {
    SWEEP_T.iter().map(|&t| run_at(graph, t)).collect()
}

/// Where mean |m| crosses 0.5 as the sweep cools — the ledger's Tc
/// estimate. Points are ordered hot to cold, so |m| climbs through
/// the line.
fn crossing(points: &[Point]) -> Option<f64> {
    for w in points.windows(2) {
        let (a, b) = (w[0], w[1]);
        if a.mean_abs_m < 0.5 && b.mean_abs_m >= 0.5 {
            let f = (0.5 - a.mean_abs_m) / (b.mean_abs_m - a.mean_abs_m);
            return Some(a.temperature + f * (b.temperature - a.temperature));
        }
    }
    None
}

/// Deterministic demo grove: broad canopy, known census.
/// root + 6 branches; each branch 4 subdirs and 3 files;
/// each subdir 5 files, plus one file at the root.
/// Atoms: 1 + 1 + 6 + 24 + 18 + 120 = 170.
fn build_demo_grove(base: &Path) -> Result<PathBuf> {
    let root = base.join("ising-crystal-grove");
    let _ = std::fs::remove_dir_all(&root);
    for b in 0..6 {
        for s in 0..4 {
            std::fs::create_dir_all(root.join(format!("branch{b}/sub{s}")))?;
            for f in 0..5 {
                std::fs::write(
                    root.join(format!("branch{b}/sub{s}/leaf{f}.rs")),
                    b"// leaf\n",
                )?;
            }
        }
        for f in 0..3 {
            std::fs::write(root.join(format!("branch{b}/stem{f}.rs")), b"// stem\n")?;
        }
    }
    std::fs::write(root.join("README-grove.txt"), b"// grove\n")?;
    Ok(root)
}

struct Ledger {
    atoms: usize,
    files: usize,
    dirs: usize,
    crystal: Vec<Point>,
    bonds: Vec<Point>,
    cubic: Vec<Point>,
    crystal_cross: Option<f64>,
    bonds_cross: Option<f64>,
    cubic_cross: Option<f64>,
    crystal_graph_stats: (usize, f64, usize, usize),
    bonds_graph_stats: (usize, f64, usize, usize),
    cubic_graph_stats: (usize, f64, usize, usize),
}

fn stats(g: &Graph) -> (usize, f64, usize, usize) {
    (g.edges(), g.mean_degree(), g.leaves(), g.max_degree())
}

fn survey(root: &Path) -> Result<Ledger> {
    let crystal = Crystal::build_from_path(root)?;
    let (cg, bg) = crystal_graph(&crystal);
    let cubic = cubic_graph(6);
    let crystal_pts = run_sweep(&cg);
    let bonds_pts = run_sweep(&bg);
    let cubic_pts = run_sweep(&cubic);
    Ok(Ledger {
        atoms: crystal.atoms.len(),
        files: crystal.atoms.iter().filter(|a| !a.is_dir).count(),
        dirs: crystal.atoms.iter().filter(|a| a.is_dir).count(),
        crystal_cross: crossing(&crystal_pts),
        bonds_cross: crossing(&bonds_pts),
        cubic_cross: crossing(&cubic_pts),
        crystal: crystal_pts,
        bonds: bonds_pts,
        cubic: cubic_pts,
        crystal_graph_stats: stats(&cg),
        bonds_graph_stats: stats(&bg),
        cubic_graph_stats: stats(&cubic),
    })
}

fn print_curve(label: &str, pts: &[Point], cross: Option<f64>, st: (usize, f64, usize, usize)) {
    println!(
        "  {label}: edges {}, mean degree {:.2}, leaves(deg<=1) {}, max degree {}",
        st.0, st.1, st.2, st.3
    );
    for p in pts {
        println!(
            "    T {:.2}: |m| {:.3} (std {:.3}), E/spin {:.3}, accept {:.3}",
            p.temperature, p.mean_abs_m, p.m_std, p.energy, p.accept
        );
    }
    match cross {
        Some(t) => println!("    |m| crosses 0.5 at T* = {t:.2}"),
        None => println!("    |m| never crosses 0.5 in the swept range"),
    }
}

fn headless() -> Result<()> {
    let base = std::env::temp_dir().join(format!("ising-crystal-{}", std::process::id()));
    std::fs::create_dir_all(&base)?;
    let root = build_demo_grove(&base)?;
    let l = survey(&root)?;
    println!("ising-crystal headless: the corpse's ferromagnet, grown on a codebase crystal");
    println!("  grove: {} atoms ({} files / {} dirs)", l.atoms, l.files, l.dirs);
    print_curve(
        "crystal (bonds + spatial)",
        &l.crystal,
        l.crystal_cross,
        l.crystal_graph_stats,
    );
    print_curve("crystal bonds only", &l.bonds, l.bonds_cross, l.bonds_graph_stats);
    print_curve("cubic control (periodic)", &l.cubic, l.cubic_cross, l.cubic_graph_stats);
    if let (Some(c), Some(u)) = (l.crystal_cross, l.cubic_cross) {
        println!(
            "  verdict: crystal T* {:.2} vs cubic T* {:.2} — the grown lattice orders {:.2} colder",
            c,
            u,
            u - c
        );
    }
    let _ = std::fs::remove_dir_all(&base);
    Ok(())
}

fn main() -> Result<()> {
    // --headless handled BEFORE anything else; there is no TUI.
    let _headless = std::env::args().any(|a| a == "--headless");
    headless()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grove(tag: &str) -> (PathBuf, PathBuf) {
        let base = std::env::temp_dir().join(format!("ising-crystal-test-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&base).unwrap();
        let root = build_demo_grove(&base).unwrap();
        (base, root)
    }

    #[test]
    fn grove_census_is_the_known_census() {
        let (base, root) = grove("census");
        let crystal = Crystal::build_from_path(&root).unwrap();
        // 1 root + 1 root file + 6 branches + 18 stems + 24 subdirs + 120 leaves.
        assert_eq!(crystal.atoms.len(), 170);
        assert_eq!(crystal.bonds.len(), crystal.atoms.len() - 1);
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn spatial_adjacency_adds_edges_beyond_the_tree() {
        let (base, root) = grove("edges");
        let crystal = Crystal::build_from_path(&root).unwrap();
        let (cg, bg) = crystal_graph(&crystal);
        assert!(cg.edges() > bg.edges(), "grown space must add neighbours the tree lacks");
        assert!(cg.mean_degree() > bg.mean_degree());
        // Every atom keeps at least its tree parent (or children): no orphans.
        assert!(cg.adj.iter().all(|a| !a.is_empty()));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn cubic_control_has_the_corpse_topology() {
        let g = cubic_graph(6);
        assert_eq!(g.n(), 216);
        assert!(g.adj.iter().all(|a| a.len() == 6));
        assert_eq!(g.leaves(), 0);
    }

    #[test]
    fn cold_cubic_orders_hot_cubic_does_not() {
        let g = cubic_graph(6);
        let cold = run_at(&g, 1.0);
        let hot = run_at(&g, 8.0);
        assert!(cold.mean_abs_m > 0.9, "cold cubic |m| = {:.3}", cold.mean_abs_m);
        assert!(hot.mean_abs_m < 0.25, "hot cubic |m| = {:.3}", hot.mean_abs_m);
        assert!(cold.energy < hot.energy);
    }

    #[test]
    fn crystal_orders_colder_than_the_cube() {
        let (base, root) = grove("cross");
        let l = survey(&root).unwrap();
        let c = l.crystal_cross.expect("crystal must cross in range");
        let u = l.cubic_cross.expect("cubic must cross in range");
        assert!(
            c < u - 0.25,
            "crystal T* {c:.2} must sit clearly below cubic T* {u:.2}"
        );
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn the_ledger_is_deterministic() {
        let g = cubic_graph(4);
        let a = run_at(&g, 3.0);
        let b = run_at(&g, 3.0);
        assert_eq!(a.mean_abs_m, b.mean_abs_m);
        assert_eq!(a.energy, b.energy);
    }
}
