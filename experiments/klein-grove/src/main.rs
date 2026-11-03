//! # klein-grove
//!
//! A revival cross: graveyard/klein-files x crates/miller-lattice.
//!
//! The archived klein-files (979 LOC, wgpu/winit, edition 2021) laid a
//! filesystem over a Klein bottle: a scanner walked a directory and
//! assigned each entry a (u, v) on the figure-8 immersion, and a GPU
//! state rendered the bottle with the files on it. Its mathematics was
//! sound — its own tests proved the gluing identity
//! P(0, v) = P(2PI, 2PI - v) — and that identity is exactly what got it
//! pardoned once, when a "Logical Decay / Terminal Error" (an explicit
//! zero-multiplication) broke the gluing and a later fix restored it.
//! It was executed anyway, for the ordinary far-corner cause: Ecosystem
//! Maladaptation / Execution Void — a winit event loop as its only
//! mouth, no `--headless` bypass, so nothing it laid on the bottle was
//! ever counted by anyone who could not open its window.
//!
//! The living miller-lattice donates the organ the corpse's scanner
//! never had: a *tree*. The corpse placed files by traversal index
//! alone (u and v both proportional to i/N), so a directory and its
//! children could land antipodal and the bottle's one strange property
//! — non-orientability — was decoration. Here the crystal's grown
//! grove (the same deterministic demo grove poincare-grove and
//! ising-crystal survey: 25 atoms, 24 bonds) is placed by structure:
//! each atom's u is the centre of its inherited sector (children
//! subdivide their parent's sector), each atom's v is its depth rung,
//! v = 2PI * depth / (max_depth + 1). Bonds then run locally on the
//! bottle, and the bottle's strangeness becomes measurable in the
//! grove's own terms.
//!
//! Three ledgers, all printed by `--headless`:
//!
//! A. The gluing, re-proved in f64: max |P(0, v) - P(2PI, 2PI - v)|
//!    over a v grid (the corpse's pardoned identity), beside the naive
//!    tear, mean |P(0, v) - P(2PI, v)| — what the seam would cost if
//!    the flip were not there.
//! B. The seam frame: at the seam the immersion's own partial
//!    derivatives are compared across the identification. Because
//!    P(2PI, v) = P(0, 2PI - v) identically, differentiating says the
//!    u-frame must agree across the seam while the v-frame must
//!    *reverse*: dP/du dot > 0, dP/dv dot < 0, both normalised. A
//!    torus control (same tube shape, both gluings unflipped) must
//!    show both dots positive. This is the flip, measured from the
//!    corpse's formula, not asserted from its label.
//! C. The grove census: bond chords on the bottle vs the same
//!    (u, v) placement read on the torus, and the unmarked
//!    adjacencies — non-bond atom pairs that are close under the
//!    bottle's distance (which may use the flipped representative
//!    (u, 2PI - v)) but far under the chart's distance (the torus
//!    metric on the same coordinates). A pair can only qualify by
//!    meeting across the flip, which on this placement means
//!    v_i + v_j = 2PI: a shallow atom holding hands with a deep one
//!    (depths summing to max_depth + 1) that the flat map keeps a
//!    whole chart apart.
//!
//! Falsification condition: if ledger A's gluing deviation exceeds
//! 1e-6, or ledger B's v-frame dot at the seam is above -0.9 at any
//! sampled v, the surface as implemented is not the non-orientable
//! bottle the corpse described, and this revival's claim fails on
//! its own instrument. If ledger C finds zero unmarked adjacencies,
//! the flip is real but the grove never uses it — the bottle would
//! be decoration again, and that too is reported, not hidden.
//!
//! Adaptations, named: the corpse's immersion ran in f32 inside a
//! wgpu renderer; here it is f64 and headless, the renderer left in
//! the grave. The corpse's scanner used walkdir order over a live
//! directory; here miller-lattice grows the deterministic demo
//! grove, so every number below reproduces exactly.
//!
//! `cargo run -p klein-grove -- --headless` prints the ledger.
//! --headless is handled before anything else; there is no TUI.

use anyhow::Result;
use miller_lattice::Crystal;
use std::f64::consts::TAU;
use std::path::{Path, PathBuf};

/// The corpse's major radius (`a` in graveyard/klein-files/src/math.rs).
const A: f64 = 3.0;

/// The corpse's figure-8 Klein immersion, verbatim in f64:
/// r = a + cos(u/2) sin v - sin(u/2) sin 2v;
/// P = (r cos u, r sin u, sin(u/2) sin v + cos(u/2) sin 2v).
fn klein(u: f64, v: f64) -> [f64; 3] {
    let r = A + (u / 2.0).cos() * v.sin() - (u / 2.0).sin() * (2.0 * v).sin();
    [
        r * u.cos(),
        r * u.sin(),
        (u / 2.0).sin() * v.sin() + (u / 2.0).cos() * (2.0 * v).sin(),
    ]
}

/// The orientable control: the same tube read as a torus —
/// P = ((a + cos v) cos u, (a + cos v) sin u, sin v), both gluings
/// unflipped: P(2PI, v) = P(0, v), P(u, 2PI) = P(u, 0).
fn torus(u: f64, v: f64) -> [f64; 3] {
    [
        (A + v.cos()) * u.cos(),
        (A + v.cos()) * u.sin(),
        v.sin(),
    ]
}

fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn norm(a: [f64; 3]) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn unit(a: [f64; 3]) -> [f64; 3] {
    let n = norm(a);
    [a[0] / n, a[1] / n, a[2] / n]
}

/// Partial derivatives by central differences (h small against the
/// immersion's curvature; the seam comparisons below are normalised
/// dots, insensitive to the step at this size).
fn partials(p: fn(f64, f64) -> [f64; 3], u: f64, v: f64) -> ([f64; 3], [f64; 3]) {
    let h = 1e-5;
    let du = unit(sub(p(u + h, v), p(u - h, v)));
    let dv = unit(sub(p(u, v + h), p(u, v - h)));
    (du, dv)
}

// ---------------------------------------------------------------- A: gluing

struct Gluing {
    max_dev: f64,
    naive_mean: f64,
    naive_max: f64,
}

fn gluing_ledger() -> Gluing {
    let n = 128;
    let mut max_dev = 0.0f64;
    let mut naive_sum = 0.0f64;
    let mut naive_max = 0.0f64;
    for i in 0..n {
        let v = TAU * (i as f64) / (n as f64);
        let dev = norm(sub(klein(0.0, v), klein(TAU, TAU - v)));
        max_dev = max_dev.max(dev);
        let tear = norm(sub(klein(0.0, v), klein(TAU, v)));
        naive_sum += tear;
        naive_max = naive_max.max(tear);
    }
    Gluing {
        max_dev,
        naive_mean: naive_sum / (n as f64),
        naive_max,
    }
}

// ------------------------------------------------------- B: the seam frame

struct SeamRow {
    v: f64,
    klein_du_dot: f64,
    klein_dv_dot: f64,
    torus_du_dot: f64,
    torus_dv_dot: f64,
}

fn seam_ledger() -> Vec<SeamRow> {
    [0.7, 1.9, 3.4, 5.1]
        .iter()
        .map(|&v| {
            // Klein: the point at (TAU, v) IS the point at (0, TAU - v);
            // compare the frame there against the frame at (0, TAU - v).
            let (k_du_far, k_dv_far) = partials(klein, TAU, v);
            let (k_du_near, k_dv_near) = partials(klein, 0.0, TAU - v);
            // Torus: the point at (TAU, v) is the point at (0, v).
            let (t_du_far, t_dv_far) = partials(torus, TAU, v);
            let (t_du_near, t_dv_near) = partials(torus, 0.0, v);
            SeamRow {
                v,
                klein_du_dot: dot(k_du_far, k_du_near),
                klein_dv_dot: dot(k_dv_far, k_dv_near),
                torus_du_dot: dot(t_du_far, t_du_near),
                torus_dv_dot: dot(t_dv_far, t_dv_near),
            }
        })
        .collect()
}

// ---------------------------------------------------------- C: the grove

struct Placed {
    u: f64,
    v: f64,
    depth: usize,
}

struct Grove {
    placed: Vec<Placed>,
    bonds: Vec<(usize, usize)>,
    files: usize,
    dirs: usize,
    max_depth: usize,
}

impl Grove {
    fn build(crystal: Crystal) -> Self {
        let n = crystal.atoms.len();
        // Parent / children from the crystal's bonds, order preserved.
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut parent: Vec<Option<usize>> = vec![None; n];
        for &(p, c) in &crystal.bonds {
            children[p].push(c);
            parent[c] = Some(p);
        }
        let mut depth = vec![0usize; n];
        for i in 0..n {
            let mut d = 0;
            let mut cur = i;
            while let Some(p) = parent[cur] {
                d += 1;
                cur = p;
            }
            depth[i] = d;
        }
        let max_depth = depth.iter().copied().max().unwrap_or(0);
        // Sectors: root owns [0, TAU); children subdivide equally.
        let mut sector: Vec<(f64, f64)> = vec![(0.0, TAU); n];
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| depth[i]);
        for &i in &order {
            let (lo, hi) = sector[i];
            let k = children[i].len();
            for (j, &c) in children[i].iter().enumerate() {
                let w = (hi - lo) / (k as f64);
                sector[c] = (lo + w * (j as f64), lo + w * ((j + 1) as f64));
            }
        }
        let placed = (0..n)
            .map(|i| Placed {
                u: (sector[i].0 + sector[i].1) / 2.0,
                v: TAU * (depth[i] as f64) / ((max_depth + 1) as f64),
                depth: depth[i],
            })
            .collect();
        Grove {
            placed,
            bonds: crystal.bonds.clone(),
            files: crystal.atoms.iter().filter(|a| !a.is_dir).count(),
            dirs: crystal.atoms.iter().filter(|a| a.is_dir).count(),
            max_depth,
        }
    }
}

/// Wrapped distance on a circle of circumference TAU.
fn wrap_dist(a: f64, b: f64) -> f64 {
    let d = (a - b).abs() % TAU;
    d.min(TAU - d)
}

/// Chart (torus) distance between two placements.
fn chart_dist(p: &Placed, q: &Placed) -> f64 {
    let du = wrap_dist(p.u, q.u);
    let dv = wrap_dist(p.v, q.v);
    (du * du + dv * dv).sqrt()
}

/// Bottle distance: the chart distance to either of q's
/// representatives, (u_q, v_q) or (u_q, TAU - v_q).
fn bottle_dist(p: &Placed, q: &Placed) -> f64 {
    let du = wrap_dist(p.u, q.u);
    let dv_plain = wrap_dist(p.v, q.v);
    let dv_flip = wrap_dist(p.v, TAU - q.v);
    let dv = dv_plain.min(dv_flip);
    (du * du + dv * dv).sqrt()
}

#[allow(dead_code)] // unmarked_pairs is asserted on in the tests
struct Census {
    klein_bond_mean: f64,
    klein_bond_max: f64,
    torus_bond_mean: f64,
    torus_bond_max: f64,
    unmarked: usize,
    unmarked_cross_depth: usize,
    unmarked_pairs: Vec<(usize, usize)>,
}

const NEAR: f64 = 0.80;

fn census(g: &Grove) -> Census {
    let chord = |p: fn(f64, f64) -> [f64; 3], a: &Placed, b: &Placed| {
        norm(sub(p(a.u, a.v), p(b.u, b.v)))
    };
    let (mut ks, mut kmax) = (0.0f64, 0.0f64);
    let (mut ts, mut tmax) = (0.0f64, 0.0f64);
    for &(a, b) in &g.bonds {
        let kc = chord(klein, &g.placed[a], &g.placed[b]);
        let tc = chord(torus, &g.placed[a], &g.placed[b]);
        ks += kc;
        kmax = kmax.max(kc);
        ts += tc;
        tmax = tmax.max(tc);
    }
    let nb = g.bonds.len() as f64;
    let bond_set: std::collections::BTreeSet<(usize, usize)> = g
        .bonds
        .iter()
        .map(|&(a, b)| (a.min(b), a.max(b)))
        .collect();
    let mut pairs = Vec::new();
    let mut cross = 0usize;
    for i in 0..g.placed.len() {
        for j in (i + 1)..g.placed.len() {
            if bond_set.contains(&(i, j)) {
                continue;
            }
            let bd = bottle_dist(&g.placed[i], &g.placed[j]);
            let cd = chart_dist(&g.placed[i], &g.placed[j]);
            if bd < NEAR && cd >= NEAR {
                pairs.push((i, j));
                if g.placed[i].depth != g.placed[j].depth {
                    cross += 1;
                }
            }
        }
    }
    Census {
        klein_bond_mean: ks / nb,
        klein_bond_max: kmax,
        torus_bond_mean: ts / nb,
        torus_bond_max: tmax,
        unmarked: pairs.len(),
        unmarked_cross_depth: cross,
        unmarked_pairs: pairs,
    }
}

/// The deterministic demo grove: the same shape poincare-grove and
/// ising-crystal survey, so the three maps of it can be compared.
fn build_demo_grove(base: &Path) -> Result<PathBuf> {
    let root = base.join("klein-grove");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("alpha/deep"))?;
    std::fs::create_dir_all(root.join("beta"))?;
    std::fs::create_dir_all(root.join("gamma/deep/deeper"))?;
    for f in [
        "alpha/ash.rs",
        "alpha/birch.rs",
        "alpha/cedar.rs",
        "alpha/deep/oak.rs",
        "alpha/deep/pine.rs",
        "alpha/deep/rowan.rs",
        "beta/elm.rs",
        "beta/fig.rs",
        "beta/holly.rs",
        "beta/juniper.rs",
        "beta/maple.rs",
        "gamma/larch.rs",
        "gamma/deep/willow.rs",
        "gamma/deep/deeper/alder.rs",
        "gamma/deep/deeper/beech.rs",
        "gamma/deep/deeper/yew.rs",
        "README-grove.txt",
        "ledger.txt",
    ] {
        std::fs::write(root.join(f), b"// grove\n")?;
    }
    Ok(root)
}

fn run(root: &Path) -> Result<(Grove, Gluing, Vec<SeamRow>, Census)> {
    let crystal = Crystal::build_from_path(root)?;
    let g = Grove::build(crystal);
    let c = census(&g);
    Ok((g, gluing_ledger(), seam_ledger(), c))
}

fn print_ledger(g: &Grove, gl: &Gluing, seam: &[SeamRow], c: &Census) {
    println!("klein-grove headless: a grown grove laid on the corpse's bottle, the seam measured, the flip counted in the grove's own pairs");
    println!(
        "  grove: {} atoms, {} bonds, {} files / {} dirs, max depth {}",
        g.placed.len(),
        g.bonds.len(),
        g.files,
        g.dirs,
        g.max_depth
    );
    println!("  placement: u = inherited sector centre, v = TAU * depth / (max_depth + 1); bottle a = {A:.1} (the corpse's)");
    println!("  A gluing (128 v samples): max |P(0,v) - P(2PI, 2PI-v)| = {:.3e}", gl.max_dev);
    println!(
        "  A naive tear: mean |P(0,v) - P(2PI, v)| = {:.4}, max {:.4} (what the seam costs without the flip)",
        gl.naive_mean, gl.naive_max
    );
    for r in seam {
        println!(
            "  B seam frame at v = {:.1}: klein du dot {:+.4}, dv dot {:+.4}; torus du dot {:+.4}, dv dot {:+.4}",
            r.v, r.klein_du_dot, r.klein_dv_dot, r.torus_du_dot, r.torus_dv_dot
        );
    }
    println!(
        "  C bond chords: klein mean {:.4} max {:.4}; torus mean {:.4} max {:.4} (same placement)",
        c.klein_bond_mean, c.klein_bond_max, c.torus_bond_mean, c.torus_bond_max
    );
    println!(
        "  C unmarked adjacencies (bottle < {NEAR:.2} rad, chart >= {NEAR:.2} rad, non-bond): {} pairs, {} cross-depth",
        c.unmarked, c.unmarked_cross_depth
    );
    println!("  declared predictions, judged on the ledgers above:");
    println!(
        "    P1 the gluing holds in f64 (max dev < 1e-6): {:.3e} — {}",
        gl.max_dev,
        if gl.max_dev < 1e-6 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the naive tear is real (mean > 1.0): {:.4} — {}",
        gl.naive_mean,
        if gl.naive_mean > 1.0 { "HELD" } else { "CRACKED" }
    );
    let dv_worst = seam.iter().map(|r| r.klein_dv_dot).fold(f64::MIN, f64::max);
    let du_worst = seam.iter().map(|r| r.klein_du_dot).fold(1.0f64, f64::min);
    println!(
        "    P3 the v-frame reverses at the seam (klein dv dot < -0.9 at every sampled v; worst {:+.4}) and the u-frame does not (du dot > 0.9; worst {:+.4}): {}",
        dv_worst,
        du_worst,
        if dv_worst < -0.9 && du_worst > 0.9 { "HELD" } else { "CRACKED" }
    );
    let t_worst = seam
        .iter()
        .map(|r| r.torus_du_dot.min(r.torus_dv_dot))
        .fold(1.0f64, f64::min);
    println!(
        "    P4 the torus control keeps both frames (worst dot > 0.9): {:+.4} — {}",
        t_worst,
        if t_worst > 0.9 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 the grove uses the flip (unmarked adjacencies >= 1, at least one cross-depth): {} pairs, {} cross-depth — {}",
        c.unmarked,
        c.unmarked_cross_depth,
        if c.unmarked >= 1 && c.unmarked_cross_depth >= 1 { "HELD" } else { "CRACKED" }
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        let base = std::env::temp_dir();
        let root = build_demo_grove(&base)?;
        let (g, gl, seam, c) = run(&root)?;
        print_ledger(&g, &gl, &seam, &c);
        return Ok(());
    }
    println!("klein-grove: run with --headless for the ledger. (The corpse's window stays in the grave; this planting counts instead of rendering.)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grove() -> Grove {
        // Distinct base per test thread: build_demo_grove removes and
        // recreates its directory, and tests run in parallel.
        let base = std::env::temp_dir().join(format!(
            "klein-grove-tests-{:?}",
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&base).unwrap();
        let root = build_demo_grove(&base).unwrap();
        let crystal = Crystal::build_from_path(&root).unwrap();
        Grove::build(crystal)
    }

    #[test]
    fn gluing_identity_holds() {
        let gl = gluing_ledger();
        assert!(gl.max_dev < 1e-9, "gluing dev {}", gl.max_dev);
    }

    #[test]
    fn naive_tear_is_real() {
        let gl = gluing_ledger();
        assert!(gl.naive_mean > 1.0, "naive mean {}", gl.naive_mean);
    }

    #[test]
    fn v_frame_reverses_at_the_seam() {
        for r in seam_ledger() {
            assert!(r.klein_dv_dot < -0.99, "v={} dv dot {}", r.v, r.klein_dv_dot);
            assert!(r.klein_du_dot > 0.99, "v={} du dot {}", r.v, r.klein_du_dot);
        }
    }

    #[test]
    fn torus_control_keeps_both_frames() {
        for r in seam_ledger() {
            assert!(r.torus_du_dot > 0.99, "v={} du {}", r.v, r.torus_du_dot);
            assert!(r.torus_dv_dot > 0.99, "v={} dv {}", r.v, r.torus_dv_dot);
        }
    }

    #[test]
    fn grove_shape_matches_the_other_maps() {
        let g = grove();
        assert_eq!(g.placed.len(), 25);
        assert_eq!(g.bonds.len(), 24);
        assert_eq!(g.files, 18);
        assert_eq!(g.dirs, 7);
        assert_eq!(g.max_depth, 4);
    }

    #[test]
    fn grove_uses_the_flip() {
        let g = grove();
        let c = census(&g);
        assert!(c.unmarked >= 1, "no unmarked adjacencies");
        assert!(c.unmarked_cross_depth >= 1, "no cross-depth pair");
        // Every unmarked pair must meet across the flip: v_i + v_j = TAU
        // (depths summing to max_depth + 1) — the flip is the only door.
        for &(i, j) in &c.unmarked_pairs {
            assert_eq!(g.placed[i].depth + g.placed[j].depth, g.max_depth + 1);
        }
    }

    #[test]
    fn bond_chords_are_finite_and_bounded() {
        let g = grove();
        let c = census(&g);
        assert!(c.klein_bond_mean > 0.0 && c.klein_bond_mean.is_finite());
        // No chord on a bottle of major radius 3 (tube reach <= 2)
        // can exceed the immersion's diameter, 2 * (A + 2).
        assert!(c.klein_bond_max <= 2.0 * (A + 2.0));
        assert!(c.torus_bond_max <= 2.0 * (A + 1.0));
    }
}
