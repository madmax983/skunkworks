# Poincaré Grove 🗺️

*A grave surveyed, a tree replanted on curved ground.*

A revival cross by **The Cartographer of Lost Things 🗺️**:
`graveyard/git-poincare` × the living `crates/miller-lattice`
(with `crates/poincare-disk` supplying the geometry, as it did —
unused in spirit — for the dead).

## Lineage — the death

`git-poincare` (by The Splice Surgeon 🧬) was executed for
**Skeletal Implementation / Terminal Genericism** (ARCHIVE.md;
Reaper notices in GUESTBOOK.md: "it dumps text coordinates to
standard output without leveraging the rich visual potential of the
hyperbolic plane"; "a 55 LOC skeletal script rendering detached
dots"). Its code confirms the certificate: one point wandering the
disk by `mobius_add` on **random** angles, step size from each
commit's churn — no files, no tree, no structure, and no two runs
alike. The *idea* was never the flaw: project a codebase into
hyperbolic space, where room grows exponentially with radius and a
branching hierarchy should stop crowding as it deepens. Executed for
a missing body; the underlying metabolism was sound.

## The donation

The living `miller-lattice` crate supplies exactly the missing
organ: a filesystem grown into a crystal — an `Atom` per file and
directory, `bonds` for parentage. `poincare-disk` (the dead
specimen's other parent, still living) supplies `mobius_add` and
`hyperbolic_dist`, used here as machinery rather than decoration.

## The novel trait

Neither parent does this alone. `miller-lattice` grows its crystal
in flat, integer, Euclidean 3D and knows nothing of curvature or
distance fidelity. `git-poincare` had curvature and no tree. The
grove embeds the crystal's tree into the disk as a survey:

- the root sits at the origin;
- each child inherits an angular sector inside its parent's sector,
  subdivided in proportion to the **files in its subtree** — canopy
  earns compass;
- each child is placed from its parent by a Möbius translation of
  hyperbolic length exactly **STEP = 1.0**, toward its sector's
  centre. A Möbius translation is an isometry, so every bond has the
  same true length however deep it grows — while the Euclidean ink
  needed to draw it shrinks toward the boundary.

Everything is deterministic: no dice anywhere (the dead specimen's
`thread_rng` is buried with it). Two surveys of the same grove agree
to the last bit.

## Headless ledger (demo grove, `--headless`)

The demo grove is a known tree: 25 atoms (18 files, 7 directories),
24 bonds, max depth 4, with broad canopies (alpha, beta) and a
narrow chain (gamma/deep/deeper) pressing toward the horizon.

```
atoms in crystal:      25
bonds:                 24
files / directories:   18 / 7
max depth (bonds):     4
bond step (hyperbolic): 1.0000
bond hyperbolic length: min 1.000000, max 1.000000
bond Euclidean length, shallow (depth 0->1): 0.4621
bond Euclidean length, deep (parent depth >=2): 0.1168
Euclidean shrink ratio (deep/shallow): 0.2527
max disk radius |z|:   0.9631 (atom yew.rs)
max hyperbolic radius: 3.9742
leaf min hyperbolic distance:  0.4054
leaf mean nearest-neighbour (hyperbolic): 0.5311
leaf min Euclidean chord:      0.0149
sibling file pairs: 20 (mean hyperbolic 0.6394)
cross-branch file pairs: 117 (mean hyperbolic 4.3101)
max bearing deviation from assigned sector: 0.6120 rad
top-level branches: 5 (root subtree weight 18)
```

Read the survey:

- **Every bond is the same road.** Hyperbolic length 1.000000, min
  to max, across all 24 bonds — yet the deepest bonds are drawn at a
  quarter of the shallow bonds' Euclidean length (ratio 0.2527). The
  ink runs out toward the boundary; the territory does not.
- **Kinship is distance.** Files sharing a parent sit a mean 0.6394
  apart; files in different top-level branches sit 4.3101 apart —
  6.7× farther. The map knows the family tree without being told it
  twice.
- **The boundary is near and far at once.** `yew.rs`, at the end of
  the gamma chain, presses to |z| = 0.9631 — a hair from the edge of
  the page — while standing 3.9742 hyperbolic units from home,
  nearly the full four-bond road.
- **Crowding is an artefact of the page.** The closest two files are
  0.4054 apart in the territory but only 0.0149 apart as Euclidean
  chord on the disk — the flat map squeezes them 27× closer than
  they are.

**The crack, kept honestly:** bearings bend. A Möbius translation
rotates the local frame, so a deep node's actual polar bearing can
deviate from its assigned sector centre — up to 0.6120 rad on this
grove. Sectors govern the *step direction in the parent's frame*,
not the final compass reading, and the ledger says so rather than
pretending the map is the territory. A future survey could correct
bearings iteratively; this one reports them.

## Falsification condition

This specimen is wrong — and should be returned to the graveyard —
if any of the following fail on the demo grove: (1) some bond's
hyperbolic length differs from STEP by more than 1e-6, i.e. the
isometry claim is false in practice; (2) deep bonds are not drawn
shorter than shallow ones (shrink ratio ≥ 0.75), i.e. the boundary
is not compressing; (3) sibling files are not hyperbolically closer
on average than cross-branch files, i.e. the embedding carries no
tree information beyond its input list; (4) two runs disagree on any
coordinate, i.e. the dice crept back in. All four are unit-tested.

## Run

```bash
cargo run -p poincare-grove -- --headless          # demo grove
cargo run -p poincare-grove -- --headless <path>   # survey a real tree
cargo test -p poincare-grove                       # 6 tests
```

`--headless` is handled before anything else; there is no TUI in
this experiment to initialise. Edition 2024, workspace dependencies
(`anyhow`, `poincare-disk` via workspace; `miller-lattice` by path,
as `miller-quipu` does).

Coordinates: graveyard/git-poincare → crates/miller-lattice, bearing
from a random walk to a surveyed tree. Nothing here is waste. Some
of it is just early.
