//! # Miller Quipu 🦇🧶
//!
//! Cross: `miller-lattice` × `quipu`, brought by The Night Listener.
//!
//! A filesystem tree is grown into a Miller crystal. Every atom in the
//! crystal ties a quipu cord recording what its subtree holds. Then the
//! Listener does not read the cords — that would be direct inspection,
//! and direct inspection is cheating. Instead a single pulse is sent
//! out from the root along the crystal's bonds, one hop per tick. Each
//! atom the pulse reaches reflects an echo back along the bond it came
//! from, carrying its cord's value. The listener at the root knows only
//! what returns, and when.
//!
//! The echo is a census taken by ear: the total that returns must match
//! the quipu's own accounting (folding every cord with `checked_add`),
//! and the arrival times fingerprint the shape of the tree — deep atoms
//! answer late, shallow atoms answer early, and no atom can answer
//! before its echo has physically travelled out and back.

use anyhow::Result;
use miller_lattice::Crystal;
use quipu::{Cord, Quipu};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One atom's tied cord, plus the tree structure the pulse travels.
struct KnottedCrystal {
    crystal: Crystal,
    quipu: Quipu,
    /// parent[i] = atom that bonded to i (None for the root).
    parent: Vec<Option<usize>>,
    /// children[i] = atoms bonded from i, in bond order.
    children: Vec<Vec<usize>>,
    /// depth[i] = bond-hops from the root.
    depth: Vec<usize>,
}

impl KnottedCrystal {
    fn build(crystal: Crystal) -> Self {
        let n = crystal.atoms.len();
        let mut parent = vec![None; n];
        let mut children = vec![Vec::new(); n];
        for &(p, c) in &crystal.bonds {
            parent[c] = Some(p);
            children[p].push(c);
        }
        // Depths by walking bonds outward from the root (atom 0).
        let mut depth = vec![0usize; n];
        let mut order = vec![0usize];
        let mut seen = vec![false; n];
        if n > 0 {
            seen[0] = true;
        }
        while let Some(i) = order.pop() {
            for &c in &children[i] {
                if !seen[c] {
                    seen[c] = true;
                    depth[c] = depth[i] + 1;
                    order.push(c);
                }
            }
        }
        // Tie the cords. A file's cord counts its name; a directory's
        // cord counts ten per file in its whole subtree — the census a
        // quipucamayoc would actually keep.
        let subtree_files = subtree_file_counts(&crystal, &children);
        let mut quipu = Quipu::new();
        for (i, atom) in crystal.atoms.iter().enumerate() {
            let value = if atom.is_dir {
                10 * subtree_files[i] as u64
            } else {
                (atom.name.chars().count() as u64).max(1)
            };
            quipu.add_cord(Cord::from(value));
        }
        Self { crystal, quipu, parent, children, depth }
    }

    /// The quipu's own accounting: fold every cord with checked_add.
    fn quipu_total(&self) -> u64 {
        let mut total = Cord::new();
        for cord in &self.quipu.cords {
            total = total.checked_add(cord).expect("census total overflows u64");
        }
        total.value()
    }

    fn lattice_span(&self) -> i32 {
        self.crystal
            .atoms
            .iter()
            .map(|a| a.position.x.abs() + a.position.y.abs() + a.position.z.abs())
            .max()
            .unwrap_or(0)
    }
}

fn subtree_file_counts(crystal: &Crystal, children: &[Vec<usize>]) -> Vec<usize> {
    let n = crystal.atoms.len();
    let mut counts = vec![0usize; n];
    // Process deepest atoms first: order atoms by a simple post-order
    // walk from the root.
    let mut post: Vec<usize> = Vec::with_capacity(n);
    let mut stack = vec![0usize];
    let mut seen = vec![false; n];
    if n > 0 {
        seen[0] = true;
    }
    while let Some(i) = stack.pop() {
        post.push(i);
        for &c in &children[i] {
            if !seen[c] {
                seen[c] = true;
                stack.push(c);
            }
        }
    }
    for &i in post.iter().rev() {
        counts[i] = if crystal.atoms[i].is_dir {
            children[i].iter().map(|&c| counts[c]).sum()
        } else {
            1
        };
    }
    counts
}

/// A carrier travelling the bonds. Outbound carriers carry the pulse;
/// returning carriers carry one atom's cord value back to the root.
/// A carrier moves exactly one bond per tick — no telepathy, no shortcuts.
#[derive(Debug, Clone, Copy)]
struct Carrier {
    at: usize,
    origin: usize,
    returning: bool,
}

/// The ledger the listener at the root actually heard.
#[derive(Debug, Default, Clone, PartialEq)]
struct EchoLedger {
    ticks_run: usize,
    pulses_sent: usize,
    atoms_reached: usize,
    returns_heard: usize,
    echo_total: u64,
    peak_tick: usize,
    peak_value: u64,
    /// return-tick -> total value heard at that tick.
    fingerprint: BTreeMap<usize, u64>,
}

/// Strike the crystal: one pulse from the root, echoes back along bonds.
fn listen(kc: &KnottedCrystal) -> EchoLedger {
    let mut ledger = EchoLedger::default();
    if kc.crystal.atoms.is_empty() {
        return ledger;
    }
    ledger.pulses_sent = 1;
    // Tick 0: the pulse is at the root; the root's own echo is heard
    // by the listener standing on it, and outbound carriers leave
    // along every root bond.
    let mut reached = vec![false; kc.crystal.atoms.len()];
    reached[0] = true;
    ledger.atoms_reached = 1;
    let root_value = kc.quipu.cords[0].value();
    ledger.returns_heard += 1;
    ledger.echo_total += root_value;
    if root_value > 0 {
        *ledger.fingerprint.entry(0).or_default() += root_value;
    }
    let mut carriers: Vec<Carrier> = kc.children[0]
        .iter()
        .map(|&c| Carrier { at: 0, origin: c, returning: false })
        .collect();
    // Outbound carriers store `at` = the atom they are leaving; they
    // arrive at `origin` (their target) next tick. Returning carriers
    // store `at` = current atom, and move to its parent next tick.
    let max_ticks = 4 * kc.depth.iter().max().copied().unwrap_or(0) + 8;
    let mut tick = 0usize;
    while !carriers.is_empty() && tick < max_ticks {
        tick += 1;
        let mut next: Vec<Carrier> = Vec::new();
        for car in carriers {
            if !car.returning {
                // Arrive at the target atom.
                let atom = car.origin;
                if !reached[atom] {
                    reached[atom] = true;
                    ledger.atoms_reached += 1;
                }
                // It reflects: spawn its return, and pass the pulse
                // onward to its children.
                next.push(Carrier { at: atom, origin: atom, returning: true });
                for &c in &kc.children[atom] {
                    next.push(Carrier { at: atom, origin: c, returning: false });
                }
            } else if car.at == 0 {
                // This return was the root's own — already counted at
                // tick 0, never actually travels. Drop it.
            } else {
                // Move one bond toward the root.
                let p = kc.parent[car.at].expect("non-root atom has a parent");
                if p == 0 {
                    let v = kc.quipu.cords[car.origin].value();
                    ledger.returns_heard += 1;
                    ledger.echo_total += v;
                    if v > 0 {
                        *ledger.fingerprint.entry(tick).or_default() += v;
                    }
                } else {
                    next.push(Carrier { at: p, origin: car.origin, returning: true });
                }
            }
        }
        carriers = next;
    }
    ledger.ticks_run = tick;
    for (&t, &v) in &ledger.fingerprint {
        if v > ledger.peak_value {
            ledger.peak_value = v;
            ledger.peak_tick = t;
        }
    }
    ledger
}

/// Build the deterministic demo grove the Listener strikes by default:
/// a small tree whose shape is known, so the ledger can be checked.
fn build_demo_grove(base: &Path) -> Result<PathBuf> {
    let root = base.join("miller-quipu-grove");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("alpha"))?;
    std::fs::create_dir_all(root.join("beta/deep"))?;
    std::fs::create_dir_all(root.join("gamma"))?;
    for f in ["alpha/ash.rs", "alpha/birch.rs", "alpha/cedar.rs"] {
        std::fs::write(root.join(f), b"// grove\n")?;
    }
    for f in ["beta/elm.rs", "beta/fig.rs", "beta/deep/oak.rs", "beta/deep/pine.rs"] {
        std::fs::write(root.join(f), b"// grove\n")?;
    }
    std::fs::write(root.join("gamma/yew.rs"), b"// grove\n")?;
    std::fs::write(root.join("README-grove.txt"), b"grove\n")?;
    std::fs::write(root.join("ledger.txt"), b"grove\n")?;
    Ok(root)
}

fn run(root: &Path) -> Result<(KnottedCrystal, EchoLedger)> {
    let crystal = Crystal::build_from_path(root)?;
    let kc = KnottedCrystal::build(crystal);
    let ledger = listen(&kc);
    Ok((kc, ledger))
}

fn print_ledger(root: &Path, kc: &KnottedCrystal, ledger: &EchoLedger) {
    println!("miller-quipu headless: struck the lattice at {}", root.display());
    println!("  atoms in crystal:   {}", kc.crystal.atoms.len());
    println!("  bonds:              {}", kc.crystal.bonds.len());
    println!("  lattice span:       {}", kc.lattice_span());
    println!("  cords tied:         {}", kc.quipu.cords.len());
    println!("  quipu total (checked_add): {}", kc.quipu_total());
    println!("  pulses sent:        {}", ledger.pulses_sent);
    println!("  atoms reached:      {}", ledger.atoms_reached);
    println!("  returns heard:      {}", ledger.returns_heard);
    println!("  echo total:         {}", ledger.echo_total);
    println!("  peak return tick:   {} (value {})", ledger.peak_tick, ledger.peak_value);
    println!("  ticks run:          {}", ledger.ticks_run);
    let fp: Vec<String> = ledger
        .fingerprint
        .iter()
        .map(|(t, v)| format!("t{}:{}", t, v))
        .collect();
    println!("  echo fingerprint:   {}", fp.join(" "));
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
            let base = std::env::temp_dir().join(format!("miller-quipu-{}", std::process::id()));
            std::fs::create_dir_all(&base)?;
            let grove = build_demo_grove(&base)?;
            (grove, Some(base))
        }
    };
    let (kc, ledger) = run(&root)?;
    print_ledger(&root, &kc, &ledger);
    if !headless {
        println!("  (strike it headless any time: --headless [path])");
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
        let base = std::env::temp_dir().join(format!("miller-quipu-test-{}-{}", tag, std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let root = build_demo_grove(&base).unwrap();
        (base, root)
    }

    #[test]
    fn echo_total_matches_quipu_accounting() {
        let (base, root) = grove("accounting");
        let (kc, ledger) = run(&root).unwrap();
        // Every atom was reached and every atom answered.
        assert_eq!(ledger.atoms_reached, kc.crystal.atoms.len());
        assert_eq!(ledger.returns_heard, kc.crystal.atoms.len());
        // The ear agrees with the knots: what returned, summed by
        // arrival, equals the quipu folded with its own checked_add.
        assert_eq!(ledger.echo_total, kc.quipu_total());
        assert!(ledger.echo_total > 0);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn signal_propagates_hop_by_hop() {
        let (base, root) = grove("hops");
        let (kc, ledger) = run(&root).unwrap();
        // No return can arrive at tick 1: the earliest non-root echo
        // has to travel out one bond and back one bond (tick 2).
        assert!(!ledger.fingerprint.contains_key(&1), "an echo beat the pulse home: {:?}", ledger.fingerprint);
        // The deepest grove atoms sit at depth 3 (root/beta/deep/oak.rs):
        // their echoes cannot be heard before tick 6, and are heard.
        let max_depth = kc.depth.iter().max().copied().unwrap();
        assert_eq!(max_depth, 3);
        let last_tick = *ledger.fingerprint.keys().max().unwrap();
        assert_eq!(last_tick, 2 * max_depth);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn different_trees_different_fingerprints() {
        let (base, root) = grove("chords");
        let (_, ledger_a) = run(&root).unwrap();
        // A second, flatter grove: same file count in one shallow dir.
        let flat_base = base.join("flat");
        std::fs::create_dir_all(flat_base.join("only")).unwrap();
        for i in 0..10 {
            std::fs::write(flat_base.join(format!("only/f{}.rs", i)), b"x\n").unwrap();
        }
        let (_, ledger_b) = run(&flat_base).unwrap();
        assert_ne!(ledger_a.fingerprint, ledger_b.fingerprint,
            "different trees rang the same chord: {:?} vs {:?}", ledger_a.fingerprint, ledger_b.fingerprint);
        let _ = std::fs::remove_dir_all(base);
    }
}
