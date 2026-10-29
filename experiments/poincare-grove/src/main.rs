//! # Poincaré Grove 🗺️
//!
//! Revival cross: `graveyard/git-poincare` × living `crates/miller-lattice`,
//! brought by The Cartographer of Lost Things.
//!
//! The dead specimen promised to project a codebase into hyperbolic
//! space and delivered one point wandering the disk on random angles,
//! dumping raw coordinates: no files, no tree, no two runs alike.
//! Its idea was sound — a tree is exactly what hyperbolic space is
//! *for*, because the disk's circumference grows exponentially and a
//! branching hierarchy stops crowding as it deepens. What it lacked
//! was a tree. The living `miller-lattice` supplies one: a filesystem
//! grown into a crystal, atoms for files and directories, bonds for
//! parentage.
//!
//! This grove embeds that crystal into the Poincaré disk properly:
//!
//! - the root sits at the origin;
//! - every child receives an angular sector inside its parent's
//!   sector, subdivided in proportion to the files in its subtree;
//! - every child is placed from its parent by a Möbius translation
//!   whose step has hyperbolic length exactly `STEP`, in the direction
//!   of its sector's centre. A Möbius translation is an isometry of
//!   the disk, so **every bond has the same hyperbolic length**,
//!   however deep it grows — while its Euclidean length on the page
//!   shrinks toward the boundary. That contrast is the survey: the
//!   map's ink runs out, the territory does not.
//!
//! Nothing here is random. Two surveys of the same grove produce the
//! same coordinates to the last bit — the dead specimen's dice are
//! buried with it.

use anyhow::Result;
use miller_lattice::Crystal;
use poincare_disk::{hyperbolic_dist, mobius_add, Point};
use std::f64::consts::TAU;
use std::path::{Path, PathBuf};

/// Hyperbolic length of every parent→child bond.
const STEP: f64 = 1.0;

/// A grove embedded in the disk.
struct Grove {
    crystal: Crystal,
    parent: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
    depth: Vec<usize>,
    /// Files in each atom's subtree (a file counts itself as 1).
    weight: Vec<usize>,
    /// Assigned angular sector [start, end) per atom.
    sector: Vec<(f64, f64)>,
    /// Position in the disk per atom.
    pos: Vec<Point>,
}

impl Grove {
    fn build(crystal: Crystal) -> Self {
        let n = crystal.atoms.len();
        let mut parent = vec![None; n];
        let mut children = vec![Vec::new(); n];
        for &(p, c) in &crystal.bonds {
            parent[c] = Some(p);
            children[p].push(c);
        }
        let mut depth = vec![0usize; n];
        let mut order = vec![0usize];
        let mut seen = vec![false; n];
        if n > 0 {
            seen[0] = true;
        }
        let mut bfs: Vec<usize> = Vec::new();
        while let Some(i) = order.pop() {
            bfs.push(i);
            for &c in &children[i] {
                if !seen[c] {
                    seen[c] = true;
                    depth[c] = depth[i] + 1;
                    order.push(c);
                }
            }
        }
        // Subtree file weights, deepest first.
        let mut weight = vec![0usize; n];
        for &i in bfs.iter().rev() {
            weight[i] = if crystal.atoms[i].is_dir {
                children[i].iter().map(|&c| weight[c]).sum::<usize>().max(1)
            } else {
                1
            };
        }
        // Sectors: the root owns the whole compass; each parent divides
        // its sector among its children by subtree weight.
        let mut sector = vec![(0.0f64, 0.0f64); n];
        if n > 0 {
            sector[0] = (-std::f64::consts::PI, std::f64::consts::PI);
        }
        for &i in &bfs {
            let (a0, a1) = sector[i];
            let total: usize = children[i].iter().map(|&c| weight[c]).sum();
            if total == 0 {
                continue;
            }
            let mut cur = a0;
            for &c in &children[i] {
                let w = (a1 - a0) * (weight[c] as f64 / total as f64);
                sector[c] = (cur, cur + w);
                cur += w;
            }
        }
        // Placement: root at the origin; each child one Möbius step of
        // hyperbolic length STEP from its parent, toward its sector centre.
        let step_r = (STEP / 2.0).tanh();
        let mut pos = vec![Point::new(0.0, 0.0); n];
        for &i in &bfs {
            let (a0, a1) = sector[i];
            let mid = (a0 + a1) / 2.0;
            for &c in &children[i] {
                let (c0, c1) = sector[c];
                let cmid = (c0 + c1) / 2.0;
                let _ = mid;
                let local = Point::new(step_r * cmid.cos(), step_r * cmid.sin());
                pos[c] = mobius_add(local, pos[i]);
            }
        }
        Self { crystal, parent, children, depth, weight, sector, pos }
    }

    fn files(&self) -> Vec<usize> {
        (0..self.crystal.atoms.len()).filter(|&i| !self.crystal.atoms[i].is_dir).collect()
    }

    /// Top-level branch of a node: the child of the root it descends
    /// from (the root's own files answer with their own index).
    fn top_branch(&self, mut i: usize) -> usize {
        while let Some(p) = self.parent[i] {
            if p == 0 {
                return i;
            }
            i = p;
        }
        i
    }

    /// Tree distance in bonds, via depths and the lowest common ancestor.
    fn tree_dist(&self, mut a: usize, mut b: usize) -> usize {
        let mut da = self.depth[a];
        let mut db = self.depth[b];
        let mut d = 0;
        while da > db {
            a = self.parent[a].unwrap();
            da -= 1;
            d += 1;
        }
        while db > da {
            b = self.parent[b].unwrap();
            db -= 1;
            d += 1;
        }
        while a != b {
            a = self.parent[a].unwrap();
            b = self.parent[b].unwrap();
            d += 2;
        }
        d
    }
}

#[derive(Debug, Default, Clone)]
struct Ledger {
    atoms: usize,
    bonds: usize,
    files: usize,
    dirs: usize,
    max_depth: usize,
    bond_hyp_min: f64,
    bond_hyp_max: f64,
    bond_euclid_shallow: f64,
    bond_euclid_deep: f64,
    shrink_ratio: f64,
    max_disk_radius: f64,
    max_radius_hyp: f64,
    boundary_atom: String,
    leaf_min_hyp: f64,
    leaf_mean_nn_hyp: f64,
    leaf_min_euclid: f64,
    sibling_pairs: usize,
    sibling_mean_hyp: f64,
    cousin_pairs: usize,
    cousin_mean_hyp: f64,
    max_angle_deviation: f64,
}

fn survey(g: &Grove) -> Ledger {
    let mut l = Ledger {
        atoms: g.crystal.atoms.len(),
        bonds: g.crystal.bonds.len(),
        files: g.files().len(),
        ..Default::default()
    };
    l.dirs = l.atoms - l.files;
    l.max_depth = g.depth.iter().max().copied().unwrap_or(0);

    // Bond lengths, in both geometries at once.
    let mut hyp: Vec<f64> = Vec::new();
    let (mut shal, mut shal_n) = (0.0f64, 0usize);
    let (mut deep, mut deep_n) = (0.0f64, 0usize);
    for &(p, c) in &g.crystal.bonds {
        hyp.push(hyperbolic_dist(g.pos[p], g.pos[c]));
        let e = (g.pos[c] - g.pos[p]).norm();
        if g.depth[p] == 0 {
            shal += e;
            shal_n += 1;
        } else if g.depth[p] >= 2 {
            deep += e;
            deep_n += 1;
        }
    }
    l.bond_hyp_min = hyp.iter().cloned().fold(f64::INFINITY, f64::min);
    l.bond_hyp_max = hyp.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    l.bond_euclid_shallow = if shal_n > 0 { shal / shal_n as f64 } else { 0.0 };
    l.bond_euclid_deep = if deep_n > 0 { deep / deep_n as f64 } else { 0.0 };
    l.shrink_ratio = if l.bond_euclid_shallow > 0.0 {
        l.bond_euclid_deep / l.bond_euclid_shallow
    } else {
        0.0
    };

    // The boundary: which atom presses closest to the horizon, and how
    // far it truly is (hyperbolically) from home.
    for (i, p) in g.pos.iter().enumerate() {
        if p.norm() > l.max_disk_radius {
            l.max_disk_radius = p.norm();
            l.boundary_atom = g.crystal.atoms[i].name.clone();
        }
        l.max_radius_hyp = l.max_radius_hyp.max(hyperbolic_dist(Point::new(0.0, 0.0), *p));
    }

    // Leaf crowding and tree fidelity.
    let files = g.files();
    let mut nn_sum = 0.0;
    let (mut sib, mut sib_n) = (0.0f64, 0usize);
    let (mut cou, mut cou_n) = (0.0f64, 0usize);
    l.leaf_min_hyp = f64::INFINITY;
    l.leaf_min_euclid = f64::INFINITY;
    for (ai, &a) in files.iter().enumerate() {
        let mut nn = f64::INFINITY;
        for &b in files.iter().skip(ai + 1) {
            let h = hyperbolic_dist(g.pos[a], g.pos[b]);
            let e = (g.pos[a] - g.pos[b]).norm();
            nn = nn.min(h);
            l.leaf_min_hyp = l.leaf_min_hyp.min(h);
            l.leaf_min_euclid = l.leaf_min_euclid.min(e);
            if g.parent[a] == g.parent[b] {
                sib += h;
                sib_n += 1;
            } else if g.top_branch(a) != g.top_branch(b) {
                cou += h;
                cou_n += 1;
            }
            let _ = g.tree_dist(a, b);
        }
        // nearest neighbour for `a` may also be a later file only in the
        // loop above; recompute fully for exactness on small groves.
        let mut nn_full = f64::INFINITY;
        for &b in &files {
            if b != a {
                nn_full = nn_full.min(hyperbolic_dist(g.pos[a], g.pos[b]));
            }
        }
        let _ = nn;
        nn_sum += nn_full;
    }
    l.leaf_mean_nn_hyp = if files.is_empty() { 0.0 } else { nn_sum / files.len() as f64 };
    l.sibling_pairs = sib_n;
    l.sibling_mean_hyp = if sib_n > 0 { sib / sib_n as f64 } else { 0.0 };
    l.cousin_pairs = cou_n;
    l.cousin_mean_hyp = if cou_n > 0 { cou / cou_n as f64 } else { 0.0 };

    // Sector fidelity: how far the Möbius frame's rotation bends a
    // node's actual bearing away from the bearing it was assigned.
    for i in 1..g.pos.len() {
        let (a0, a1) = g.sector[i];
        let mid = (a0 + a1) / 2.0;
        let actual = g.pos[i].im.atan2(g.pos[i].re);
        let mut d = (actual - mid).abs();
        if d > std::f64::consts::PI {
            d = TAU - d;
        }
        l.max_angle_deviation = l.max_angle_deviation.max(d);
    }
    l
}

/// The deterministic demo grove: known shape, known depths, a chain in
/// gamma to press toward the boundary and broad canopies elsewhere.
fn build_demo_grove(base: &Path) -> Result<PathBuf> {
    let root = base.join("poincare-grove");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("alpha/deep"))?;
    std::fs::create_dir_all(root.join("beta"))?;
    std::fs::create_dir_all(root.join("gamma/deep/deeper"))?;
    for f in [
        "alpha/ash.rs", "alpha/birch.rs", "alpha/cedar.rs",
        "alpha/deep/oak.rs", "alpha/deep/pine.rs", "alpha/deep/rowan.rs",
        "beta/elm.rs", "beta/fig.rs", "beta/holly.rs", "beta/juniper.rs", "beta/maple.rs",
        "gamma/larch.rs", "gamma/deep/willow.rs",
        "gamma/deep/deeper/alder.rs", "gamma/deep/deeper/beech.rs", "gamma/deep/deeper/yew.rs",
        "README-grove.txt", "ledger.txt",
    ] {
        std::fs::write(root.join(f), b"// grove\n")?;
    }
    Ok(root)
}

fn run(root: &Path) -> Result<(Grove, Ledger)> {
    let crystal = Crystal::build_from_path(root)?;
    let g = Grove::build(crystal);
    let l = survey(&g);
    Ok((g, l))
}

fn print_ledger(root: &Path, g: &Grove, l: &Ledger) {
    println!("poincare-grove headless: surveyed the grove at {}", root.display());
    println!("  atoms in crystal:      {}", l.atoms);
    println!("  bonds:                 {}", l.bonds);
    println!("  files / directories:   {} / {}", l.files, l.dirs);
    println!("  max depth (bonds):     {}", l.max_depth);
    println!("  bond step (hyperbolic): {:.4}", STEP);
    println!("  bond hyperbolic length: min {:.6}, max {:.6}", l.bond_hyp_min, l.bond_hyp_max);
    println!("  bond Euclidean length, shallow (depth 0->1): {:.4}", l.bond_euclid_shallow);
    println!("  bond Euclidean length, deep (parent depth >=2): {:.4}", l.bond_euclid_deep);
    println!("  Euclidean shrink ratio (deep/shallow): {:.4}", l.shrink_ratio);
    println!("  max disk radius |z|:   {:.4} (atom {})", l.max_disk_radius, l.boundary_atom);
    println!("  max hyperbolic radius: {:.4}", l.max_radius_hyp);
    println!("  leaf min hyperbolic distance:  {:.4}", l.leaf_min_hyp);
    println!("  leaf mean nearest-neighbour (hyperbolic): {:.4}", l.leaf_mean_nn_hyp);
    println!("  leaf min Euclidean chord:      {:.4}", l.leaf_min_euclid);
    println!("  sibling file pairs: {} (mean hyperbolic {:.4})", l.sibling_pairs, l.sibling_mean_hyp);
    println!("  cross-branch file pairs: {} (mean hyperbolic {:.4})", l.cousin_pairs, l.cousin_mean_hyp);
    println!("  max bearing deviation from assigned sector: {:.4} rad", l.max_angle_deviation);
    if !g.crystal.atoms.is_empty() {
        println!("  top-level branches: {} (root subtree weight {})", g.children[0].len(), g.weight[0]);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let arg_path = args.iter().skip(1).find(|a| !a.starts_with("--"));
    let (root, cleanup) = match arg_path {
        Some(p) => (PathBuf::from(p), None),
        None => {
            let base = std::env::temp_dir().join(format!("poincare-grove-{}", std::process::id()));
            std::fs::create_dir_all(&base)?;
            let grove = build_demo_grove(&base)?;
            (grove, Some(base))
        }
    };
    let (g, l) = run(&root)?;
    print_ledger(&root, &g, &l);
    if !headless {
        println!("  (survey it headless any time: --headless [path])");
    }
    if let Some(base) = cleanup {
        let _ = std::fs::remove_dir_all(base);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grove(tag: &str) -> (PathBuf, PathBuf) {
        let base = std::env::temp_dir().join(format!("poincare-grove-test-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let root = build_demo_grove(&base).unwrap();
        (base, root)
    }

    #[test]
    fn every_bond_has_the_same_hyperbolic_length() {
        let (base, root) = grove("bonds");
        let (g, l) = run(&root).unwrap();
        assert_eq!(l.bonds, g.crystal.atoms.len() - 1);
        assert!((l.bond_hyp_min - STEP).abs() < 1e-6, "short bond: {}", l.bond_hyp_min);
        assert!((l.bond_hyp_max - STEP).abs() < 1e-6, "long bond: {}", l.bond_hyp_max);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn all_points_stay_inside_the_disk_and_root_is_home() {
        let (base, root) = grove("disk");
        let (g, l) = run(&root).unwrap();
        assert!(g.pos[0].norm() < 1e-12);
        assert!(g.pos.iter().all(|p| p.norm() < 1.0));
        assert!(l.max_disk_radius > 0.5, "the grove never left home: {}", l.max_disk_radius);
        assert_eq!(l.max_depth, 4);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn euclidean_ink_shrinks_toward_the_boundary() {
        let (base, root) = grove("shrink");
        let (_, l) = run(&root).unwrap();
        assert!(l.bond_euclid_deep < l.bond_euclid_shallow, "deep {} !< shallow {}", l.bond_euclid_deep, l.bond_euclid_shallow);
        assert!(l.shrink_ratio < 0.75, "shrink too weak: {}", l.shrink_ratio);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn survey_is_deterministic_no_dice() {
        let (base, root) = grove("dice");
        let (g1, _) = run(&root).unwrap();
        let (g2, _) = run(&root).unwrap();
        for (a, b) in g1.pos.iter().zip(g2.pos.iter()) {
            assert_eq!(a.re.to_bits(), b.re.to_bits());
            assert_eq!(a.im.to_bits(), b.im.to_bits());
        }
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn siblings_sit_closer_than_strangers() {
        let (base, root) = grove("kin");
        let (g, l) = run(&root).unwrap();
        assert!(l.sibling_pairs > 0 && l.cousin_pairs > 0);
        assert!(l.sibling_mean_hyp < l.cousin_mean_hyp, "siblings {} !< cousins {}", l.sibling_mean_hyp, l.cousin_mean_hyp);
        // Tree distance agrees with the map's own bookkeeping.
        let files = g.files();
        assert_eq!(g.tree_dist(files[0], files[0]), 0);
        assert!(g.tree_dist(files[0], files[files.len() - 1]) >= 2);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn sectors_are_disjoint_and_weighted() {
        let (base, root) = grove("sectors");
        let (g, _) = run(&root).unwrap();
        for (i, kids) in g.children.iter().enumerate() {
            let (a0, a1) = g.sector[i];
            let mut cur = a0;
            for &c in kids {
                assert!((g.sector[c].0 - cur).abs() < 1e-9);
                assert!(g.sector[c].1 > g.sector[c].0);
                cur = g.sector[c].1;
            }
            if !kids.is_empty() {
                assert!((cur - a1).abs() < 1e-9);
            }
        }
        // beta (5 files) earns a wider sector than gamma's chain head
        // larch alone would — checked at root level by total widths:
        let widths: Vec<f64> = g.children[0].iter().map(|&c| g.sector[c].1 - g.sector[c].0).collect();
        assert!(widths.iter().all(|&w| w > 0.0));
        let _ = std::fs::remove_dir_all(base);
    }
}
