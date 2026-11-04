//! sandpile-grove — a revival cross: graveyard/sandpile-scheduler ×
//! living crates/miller-lattice.
//!
//! The dead specimen (315 LOC, macroquad + rayon) ran the Abelian
//! sandpile on a flat 512x512 grid and called the avalanches a load
//! balancer. It was executed for Terminal Genericism / Skeletal
//! Implementation: its README promised the sand would route "around
//! obstacles (if implemented)" — it never was — and its own Reaper
//! report named the salvation condition in so many words: implement
//! the routing, or integrate a more complex network topology.
//!
//! The living miller-lattice donates exactly that topology: the same
//! deterministic demo grove the disk, the magnet, and the bottle
//! have already surveyed (25 atoms, 24 bonds, 18 files / 7 dirs,
//! max depth 4), in two readings — bonds only (a tree), and bonds
//! plus Manhattan-1 spatial adjacency read off the crystal's own
//! lookup (the ising-crystal reading). A flat 5x5 grid (25 nodes)
//! is the corpse's ground shrunk to the grove's size, as control.
//!
//! The corpse's rule, generalised to a graph and made deterministic
//! (named adaptations): a node topples when its load reaches
//! degree + 1; a topple sends one grain along every edge and
//! processes exactly one (the corpse's stochastic processing,
//! charged per topple instead of per tick). Grains are injected one
//! at a time at the root (grove) or centre (grid), each followed by
//! full relaxation; an avalanche is the topplings one grain causes.
//! The blocked regimes remove the spatial graph's busiest hub: on
//! the tree its far side is unreachable by construction; on the
//! spatial graph the far side can still be reached — if it can.

use anyhow::Result;
use miller_lattice::Crystal;
use std::path::{Path, PathBuf};

const GRAINS: u64 = 2_000;

// ---------------------------------------------------------------- graph

#[derive(Clone, Debug)]
struct Graph {
    names: Vec<String>,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn from_edges(n: usize, names: Vec<String>, edges: &[(usize, usize)]) -> Self {
        let mut adj = vec![Vec::new(); n];
        for &(a, b) in edges {
            if a != b {
                if !adj[a].contains(&b) {
                    adj[a].push(b);
                }
                if !adj[b].contains(&a) {
                    adj[b].push(a);
                }
            }
        }
        for a in &mut adj {
            a.sort_unstable();
        }
        Graph { names, adj }
    }
    fn n(&self) -> usize {
        self.adj.len()
    }
    fn edges(&self) -> usize {
        self.adj.iter().map(|a| a.len()).sum::<usize>() / 2
    }
    fn degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }
    fn without_node(&self, banned: usize) -> (Graph, Vec<Option<usize>>) {
        // Returns the graph with `banned` removed and the old->new map.
        let mut map: Vec<Option<usize>> = vec![None; self.n()];
        let mut names = Vec::new();
        for v in 0..self.n() {
            if v != banned {
                map[v] = Some(names.len());
                names.push(self.names[v].clone());
            }
        }
        let mut edges = Vec::new();
        for v in 0..self.n() {
            if let Some(nv) = map[v] {
                for &w in &self.adj[v] {
                    if v < w {
                        if let Some(nw) = map[w] {
                            edges.push((nv, nw));
                        }
                    }
                }
            }
        }
        (Graph::from_edges(names.len(), names, &edges), map)
    }
}

// ---------------------------------------------------------------- pile

#[derive(Clone, Debug, Default)]
struct Ledger {
    injected: u64,
    processed: u64,
    topplings: u64,
    avalanches: u64,
    max_avalanche: u64,
    big_avalanches: u64, // size >= 17
    buckets: [u64; 5],   // 0, 1..=4, 5..=16, 17..=64, 65+
    final_chips: Vec<u64>,
}

fn relax(adj: &[Vec<usize>], chips: &mut [u64], processed: &mut u64) -> u64 {
    // Sequential Abelian relaxation; returns topplings performed.
    let n = chips.len();
    let mut topplings = 0u64;
    let mut stack: Vec<usize> = (0..n).filter(|&v| chips[v] > adj[v].len() as u64).collect();
    let mut in_stack = vec![false; n];
    for &v in &stack {
        in_stack[v] = true;
    }
    while let Some(v) = stack.pop() {
        in_stack[v] = false;
        let thr = adj[v].len() as u64 + 1;
        while chips[v] >= thr {
            chips[v] -= thr;
            *processed += 1;
            topplings += 1;
            for k in 0..adj[v].len() {
                let w = adj[v][k];
                chips[w] += 1;
                if chips[w] >= adj[w].len() as u64 + 1 && !in_stack[w] {
                    stack.push(w);
                    in_stack[w] = true;
                }
            }
            if chips[v] >= thr && !in_stack[v] {
                // v may need to topple again; loop continues anyway.
            }
        }
    }
    topplings
}

fn run_sequential(g: &Graph, source: usize, grains: u64) -> Ledger {
    let mut chips = vec![0u64; g.n()];
    let mut led = Ledger {
        final_chips: Vec::new(),
        ..Default::default()
    };
    for _ in 0..grains {
        chips[source] += 1;
        led.injected += 1;
        let t = relax(&g.adj, &mut chips, &mut led.processed);
        led.topplings += t;
        if t > 0 {
            led.avalanches += 1;
        }
        if t > led.max_avalanche {
            led.max_avalanche = t;
        }
        if t >= 17 {
            led.big_avalanches += 1;
        }
        let b = match t {
            0 => 0,
            1..=4 => 1,
            5..=16 => 2,
            17..=64 => 3,
            _ => 4,
        };
        led.buckets[b] += 1;
    }
    led.final_chips = chips;
    led
}

fn run_batch(g: &Graph, source: usize, grains: u64) -> Ledger {
    let mut chips = vec![0u64; g.n()];
    chips[source] += grains;
    let mut led = Ledger {
        injected: grains,
        ..Default::default()
    };
    led.topplings = relax(&g.adj, &mut chips, &mut led.processed);
    led.final_chips = chips;
    led
}

fn accounting_holds(led: &Ledger) -> bool {
    led.injected == led.processed + led.final_chips.iter().sum::<u64>()
}

// ---------------------------------------------------------------- grove

fn build_demo_grove(base: &Path) -> Result<PathBuf> {
    let root = base.join("sandpile-grove");
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

struct GroveGraphs {
    bonds: Graph,
    spatial: Graph,
    hub: usize,
    hub_name: String,
    far_side: Vec<usize>, // spatial indices cut off from root in the tree when hub is removed
    atoms: usize,
    files: usize,
    dirs: usize,
    max_depth: usize,
}

fn grove_graphs(crystal: &Crystal) -> GroveGraphs {
    let names: Vec<String> = crystal.atoms.iter().map(|a| a.name.clone()).collect();
    let n = crystal.atoms.len();
    let bonds = Graph::from_edges(n, names.clone(), &crystal.bonds);
    // Spatial adjacency: Manhattan-1 neighbours in the crystal lookup.
    let mut edges: Vec<(usize, usize)> = crystal.bonds.clone();
    for (i, atom) in crystal.atoms.iter().enumerate() {
        let p = atom.position;
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let q = miller_lattice::LatticePoint::new(p.x + dx, p.y + dy, p.z + dz);
            if let Some(&j) = crystal.lookup.get(&q) {
                if i < j {
                    edges.push((i, j));
                }
            }
        }
    }
    let spatial = Graph::from_edges(n, names, &edges);
    // Hub: busiest non-root node of the spatial graph.
    let hub = (1..n)
        .max_by_key(|&v| spatial.degree(v))
        .expect("grove has more than one atom");
    // Far side: nodes reachable from hub in the tree without passing
    // the root side — i.e. the component of the bonds graph after
    // removing hub, that does not contain the root (atom 0), plus
    // hub's subtree beyond its parent edge. Take the component of
    // the tree minus hub that contains hub's deepest neighbour:
    // simpler and exact: all nodes whose tree path from root passes
    // through hub, excluding hub itself.
    let mut parent = vec![usize::MAX; n];
    let mut order = vec![0usize];
    parent[0] = 0;
    let mut k = 0;
    while k < order.len() {
        let v = order[k];
        k += 1;
        for &w in &bonds.adj[v] {
            if parent[w] == usize::MAX {
                parent[w] = v;
                order.push(w);
            }
        }
    }
    assert_eq!(order.len(), n, "the grove tree must be connected");
    let mut far_side: Vec<usize> = (0..n)
        .filter(|&v| {
            if v == hub {
                return false;
            }
            let mut x = v;
            loop {
                if x == hub {
                    return true;
                }
                if x == 0 {
                    return false;
                }
                x = parent[x];
            }
        })
        .collect();
    far_side.sort_unstable();
    // Depths from the same BFS.
    let mut depth = vec![0usize; n];
    for &v in &order {
        for &w in &bonds.adj[v] {
            if parent[w] == v {
                depth[w] = depth[v] + 1;
            }
        }
    }
    GroveGraphs {
        hub_name: spatial.names[hub].clone(),
        bonds,
        spatial,
        hub,
        far_side,
        atoms: n,
        files: crystal.atoms.iter().filter(|a| !a.is_dir).count(),
        dirs: crystal.atoms.iter().filter(|a| a.is_dir).count(),
        max_depth: *depth.iter().max().unwrap_or(&0),
    }
}

fn grid_graph(side: usize) -> (Graph, usize) {
    let n = side * side;
    let names: Vec<String> = (0..n).map(|i| format!("cell-{i}")).collect();
    let mut edges = Vec::new();
    for y in 0..side {
        for x in 0..side {
            let i = y * side + x;
            if x + 1 < side {
                edges.push((i, i + 1));
            }
            if y + 1 < side {
                edges.push((i, i + side));
            }
        }
    }
    (Graph::from_edges(n, names, &edges), (side / 2) * side + side / 2)
}

// ---------------------------------------------------------------- survey

struct Survey {
    grid: Ledger,
    bonds: Ledger,
    spatial: Ledger,
    spatial_blocked: Ledger,
    bonds_blocked: Ledger,
    spatial_batch: Ledger,
    far_chips_spatial_blocked: u64,
    far_chips_bonds_blocked: u64,
    all_accounting: bool,
}

fn survey(gg: &GroveGraphs) -> Survey {
    let (grid, centre) = grid_graph(5);
    let grid_led = run_sequential(&grid, centre, GRAINS);
    let bonds_led = run_sequential(&gg.bonds, 0, GRAINS);
    let spatial_led = run_sequential(&gg.spatial, 0, GRAINS);
    let batch_led = run_batch(&gg.spatial, 0, GRAINS);

    let (spatial_b, map_s) = gg.spatial.without_node(gg.hub);
    let root_s = map_s[0].expect("root survives");
    let spatial_blocked = run_sequential(&spatial_b, root_s, GRAINS);
    let far_s: u64 = gg
        .far_side
        .iter()
        .filter_map(|&v| map_s[v].map(|nv| spatial_blocked.final_chips[nv]))
        .sum();

    let (bonds_b, map_b) = gg.bonds.without_node(gg.hub);
    let root_b = map_b[0].expect("root survives");
    let bonds_blocked = run_sequential(&bonds_b, root_b, GRAINS);
    let far_b: u64 = gg
        .far_side
        .iter()
        .filter_map(|&v| map_b[v].map(|nv| bonds_blocked.final_chips[nv]))
        .sum();

    let all_accounting = [
        &grid_led,
        &bonds_led,
        &spatial_led,
        &spatial_blocked,
        &bonds_blocked,
        &batch_led,
    ]
    .iter()
    .all(|l| accounting_holds(l));

    Survey {
        grid: grid_led,
        bonds: bonds_led,
        spatial: spatial_led,
        spatial_blocked,
        bonds_blocked,
        spatial_batch: batch_led,
        far_chips_spatial_blocked: far_s,
        far_chips_bonds_blocked: far_b,
        all_accounting,
    }
}

fn print_regime(label: &str, led: &Ledger) {
    let held: u64 = led.final_chips.iter().sum();
    let max_load = led.final_chips.iter().max().copied().unwrap_or(0);
    println!(
        "  {label}: topplings {}, avalanches {} (max {}, >=17: {}), processed {}, held {}, max final load {}",
        led.topplings, led.avalanches, led.max_avalanche, led.big_avalanches, led.processed, held,
        max_load
    );
    println!(
        "    avalanche buckets [0, 1-4, 5-16, 17-64, 65+]: {:?}",
        led.buckets
    );
}

fn print_ledger(gg: &GroveGraphs, s: &Survey) {
    println!("sandpile-grove headless: the corpse's sand, poured on the crystal's tree — avalanches counted, the blocked hub priced");
    println!(
        "  grove: {} atoms, {} bonds, {} files / {} dirs, max depth {}; spatial edges {} (bonds + Manhattan-1)",
        gg.atoms,
        gg.bonds.edges(),
        gg.files,
        gg.dirs,
        gg.max_depth,
        gg.spatial.edges()
    );
    println!(
        "  rule: topple at load >= degree + 1; a topple sends 1 grain along every edge and processes 1; {} grains, one at a time, at the root (grid: centre)",
        GRAINS
    );
    println!(
        "  hub: {} (spatial degree {}), its tree far side: {} atoms",
        gg.hub_name,
        gg.spatial.degree(gg.hub),
        gg.far_side.len()
    );
    print_regime("grid 5x5 control", &s.grid);
    print_regime("grove bonds only", &s.bonds);
    print_regime("grove spatial", &s.spatial);
    print_regime("grove spatial, hub blocked", &s.spatial_blocked);
    print_regime("grove bonds, hub blocked", &s.bonds_blocked);
    println!(
        "  far-side grains held with hub blocked: spatial {} vs bonds {}",
        s.far_chips_spatial_blocked, s.far_chips_bonds_blocked
    );
    println!(
        "  batch check (spatial, all {} grains at once): final configuration identical to one-at-a-time: {}",
        GRAINS,
        s.spatial_batch.final_chips == s.spatial.final_chips && s.spatial_batch.processed == s.spatial.processed
    );
    println!("  declared predictions, judged on the ledgers above:");
    println!(
        "    P1 the books balance in every regime (injected = processed + held): {}",
        if s.all_accounting { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the tree processes more than the grid (its leaves have nowhere to shed): processed {} vs {} — {}",
        s.bonds.processed,
        s.grid.processed,
        if s.bonds.processed > s.grid.processed { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the spatial reading processes less than the tree (more roads, longer keeping): {} vs {} — {}",
        s.spatial.processed,
        s.bonds.processed,
        if s.spatial.processed < s.bonds.processed { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the spatial graph's largest avalanche exceeds the tree's: {} vs {} — {}",
        s.spatial.max_avalanche,
        s.bonds.max_avalanche,
        if s.spatial.max_avalanche > s.bonds.max_avalanche { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 routing around the obstacle exists exactly where the topology has a second road (far side holds grains on spatial, none on the tree): {} vs {} — {}",
        s.far_chips_spatial_blocked,
        s.far_chips_bonds_blocked,
        if s.far_chips_spatial_blocked > 0 && s.far_chips_bonds_blocked == 0 {
            "HELD"
        } else {
            "CRACKED"
        }
    );
    println!(
        "    P6 the pile is Abelian (batch and one-at-a-time end in the same configuration): {}",
        if s.spatial_batch.final_chips == s.spatial.final_chips {
            "HELD"
        } else {
            "CRACKED"
        }
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--headless") {
        let base = std::env::temp_dir();
        let root = build_demo_grove(&base)?;
        let crystal = Crystal::build_from_path(&root)?;
        let gg = grove_graphs(&crystal);
        let s = survey(&gg);
        print_ledger(&gg, &s);
        return Ok(());
    }
    println!("sandpile-grove: run with --headless for the ledger. (The corpse's window stays in the grave; this planting counts instead of rendering.)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grove() -> GroveGraphs {
        let base = std::env::temp_dir().join(format!(
            "sandpile-grove-tests-{:?}",
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&base).unwrap();
        let root = build_demo_grove(&base).unwrap();
        let crystal = Crystal::build_from_path(&root).unwrap();
        grove_graphs(&crystal)
    }

    #[test]
    fn grove_shape_matches_the_other_maps() {
        let gg = grove();
        assert_eq!(gg.atoms, 25);
        assert_eq!(gg.bonds.edges(), 24);
        assert_eq!(gg.files, 18);
        assert_eq!(gg.dirs, 7);
        assert_eq!(gg.max_depth, 4);
    }

    #[test]
    fn spatial_reading_contains_the_tree() {
        let gg = grove();
        assert!(gg.spatial.edges() > gg.bonds.edges());
        for v in 0..gg.atoms {
            for &w in &gg.bonds.adj[v] {
                assert!(gg.spatial.adj[v].contains(&w));
            }
        }
    }

    #[test]
    fn topple_rule_on_a_path() {
        // Path 0-1-2: degrees 1,2,1, thresholds 2,3,2.
        let g = Graph::from_edges(
            3,
            vec!["a".into(), "b".into(), "c".into()],
            &[(0, 1), (1, 2)],
        );
        let mut chips = vec![0u64; 3];
        let mut processed = 0u64;
        chips[0] += 1;
        assert_eq!(relax(&g.adj, &mut chips, &mut processed), 0);
        chips[0] += 1;
        // Node 0 topples once: 1 grain to node 1, 1 processed.
        assert_eq!(relax(&g.adj, &mut chips, &mut processed), 1);
        assert_eq!(chips, vec![0, 1, 0]);
        assert_eq!(processed, 1);
    }

    #[test]
    fn books_balance_in_every_regime() {
        let gg = grove();
        let s = survey(&gg);
        assert!(s.all_accounting);
        for led in [
            &s.grid,
            &s.bonds,
            &s.spatial,
            &s.spatial_blocked,
            &s.bonds_blocked,
        ] {
            assert_eq!(led.injected, GRAINS);
            assert!(led.processed > 0);
            assert!(led.topplings > 0);
        }
    }

    #[test]
    fn pile_is_abelian() {
        let gg = grove();
        let seq = run_sequential(&gg.spatial, 0, 500);
        let batch = run_batch(&gg.spatial, 0, 500);
        assert_eq!(seq.final_chips, batch.final_chips);
        assert_eq!(seq.processed, batch.processed);
    }

    #[test]
    fn blocked_tree_strands_its_far_side() {
        let gg = grove();
        assert!(!gg.far_side.is_empty());
        let s = survey(&gg);
        assert_eq!(s.far_chips_bonds_blocked, 0);
    }

    #[test]
    fn far_side_is_beyond_the_hub() {
        let gg = grove();
        // Every far-side node's tree path from the root passes the hub,
        // and the hub itself is not on its own far side.
        assert!(!gg.far_side.contains(&gg.hub));
        assert!(!gg.far_side.contains(&0));
        assert!(gg.spatial.degree(gg.hub) >= 3);
    }
}
