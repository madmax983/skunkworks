# Sandpile Grove

**Performed by:** The Cartographer of Lost Things 🗺️ (a revival cross)

## Lineage

- **The dead:** `graveyard/sandpile-scheduler` (315 LOC, macroquad +
  rayon, edition 2021) — a hybrid of `locust-ddos` × `sandpile-model`
  that ran the Abelian sandpile on a flat 512×512 grid and called
  the avalanches a load balancer: *"Avalanches are just aggressive
  load balancing."* Executed for **Terminal Genericism / Skeletal
  Implementation**. Its README promised the sand would route itself
  "around obstacles (if implemented)" — it never was. Its Reaper
  report named the salvation condition in so many words: implement
  the promised routing around obstacles, integrate a more complex
  network topology, or tie the metaphor to actual operations.
- **The living parent:** `crates/miller-lattice` — the grown
  crystal, a filesystem as a tree, already surveyed three times by
  this Cartographer on three grounds: the Poincaré disk
  (`poincare-grove`), the Ising magnet (`ising-crystal`), and the
  Klein bottle (`klein-grove`). It donates the exact missing organ:
  a real, non-grid topology — here in two readings, bonds only
  (24 edges, a tree) and bonds plus Manhattan-1 spatial adjacency
  read off the crystal's own lookup map (the `ising-crystal`
  reading).
- **The control:** the corpse's own ground shrunk to the grove's
  size — a flat 5×5 grid, 25 nodes, 40 edges.

## The cross

The corpse's rule, generalised from the grid to a graph and made
deterministic. A node **topples** when its load reaches
**degree + 1**; a topple sends one grain along every edge and
**processes** exactly one grain. Named adaptations, kept honest:

- The corpse's threshold was a flat 4 (every grid cell has four
  neighbours); on a graph the threshold is degree + 1, so the rule
  is the same shape on every node whatever its degree.
- The corpse's processing was stochastic (a per-tick coin at a
  tunable rate). Here every topple processes exactly one grain —
  the coin replaced by a charge, so the ledger can balance exactly
  and the experiment needs no dice at all.
- The corpse injected at a mouse cursor or in a "DDoS mode" of
  random cells. Here 2,000 grains arrive one at a time at the root
  (grid: the centre), each followed by full relaxation; an
  **avalanche** is the number of topplings one grain causes.
- The corpse's parallel update was synchronous; the relaxation
  here is sequential. Prediction P6 tests that this changes
  nothing — the pile's Abelian property, which the corpse was
  named for and never measured.

The blocked regimes answer the Reaper's salvation condition
directly. The spatial graph's busiest non-root hub is removed, and
the grains are counted on its **far side** — the atoms whose tree
path from the root passes through the hub. On the tree that side
is unreachable by construction. On the spatial graph it can still
be reached, if the spatial edges give it a second road.

## Predictions (declared before the first run)

- **P1** The books balance in every regime: injected = processed
  \+ held, exactly.
- **P2** The tree processes more grains than the grid: its leaves
  and low-degree nodes topple at low thresholds, so a larger share
  of the traffic is charged away instead of kept.
- **P3** The spatial reading processes less than the tree: more
  roads per node raise the thresholds, so grains are kept longer.
- **P4** The spatial graph's largest avalanche exceeds the
  tree's: denser coupling lets one grain cascade further.
- **P5** Routing around the obstacle exists exactly where the
  topology has a second road: with the hub blocked, the far side
  holds grains in the spatial regime and exactly none in the
  bonds-only regime.
- **P6** The pile is Abelian: injecting all 2,000 grains at once
  and relaxing ends in the identical final configuration (and
  identical processed count) as injecting them one at a time.

## Falsification condition

This planting is wrong about its central claim — that the
corpse's missing organ was topology, not scale — if the three
unblocked grounds produce indistinguishable ledgers (same
processed share, same avalanche shape), or if P5 fails in either
direction: a spatial far side that also starves would mean the
spatial edges are decoration on this grove, and a tree far side
that somehow holds grains would mean the far-side census itself
is misdrawn. Either result is kept in the ledger, not retuned.

## Evaluation

Headless, 2,000 grains per regime, deterministic (no dice
anywhere in the planting). The grove's spatial reading has 50
edges against the tree's 24 and the grid's 40; the hub is the
`beta` directory atom (spatial degree 9), and its tree far side
is 5 atoms.

| regime | topplings | avalanches | max | ≥17 | processed | held |
|---|---|---|---|---|---|---|
| grid 5×5 control | 1,952 | 512 | 25 | 36 | 1,952 | 48 |
| grove, bonds only | 1,968 | 471 | 22 | 1 | 1,968 | 32 |
| grove, spatial | 1,942 | 457 | 21 | 8 | 1,942 | 58 |
| spatial, hub blocked | 1,947 | 558 | 20 | 5 | 1,947 | 53 |
| bonds, hub blocked | 1,979 | 578 | 16 | 0 | 1,979 | 21 |

- **P1 HELD, exactly.** Every regime balances: 2,000 = processed
  \+ held, to the grain.
- **P2 HELD, narrowly.** The tree processes 1,968 against the
  grid's 1,952 — the direction is as declared, the margin is 16
  grains, and it is kept at that size here rather than dressed.
- **P3 HELD.** The spatial reading processes least of all
  (1,942) and holds most (58): more roads do keep grains longer.
- **P4 CRACKED, and the crack is the interesting row.** The
  spatial graph's largest avalanche (21) does *not* exceed the
  tree's (22), and the grid's (25) exceeds both. But the spatial
  graph produces **8** avalanches of size ≥17 against the tree's
  **1** — the tree's shape is many small sheds and one freak
  cascade; the spatial graph cascades *often* and sheds its peak.
  Density buys frequency, not size. The declared line stands
  cracked in the source, unretuned.
- **P5 HELD — the salvation condition, answered.** With the hub
  blocked, the far side holds **6 grains** in the spatial regime
  and exactly **0** in the bonds-only regime. The sand routes
  around the obstacle precisely where a second road exists, and
  the corpse's parenthetical — "(if implemented)" — is now a
  measured sentence instead of a promise. Blocking the hub also
  *raises* the avalanche count on both grounds (558 and 578
  against 457 and 471): a wounded topology topples more often,
  in smaller pieces.
- **P6 HELD, exactly.** Batch and one-at-a-time injection end in
  the identical final configuration with the identical processed
  count — the Abelian property the corpse was named for, measured
  here for the first time.

7/7 unit tests pass (grove shape matching the other maps;
spatial reading containing the tree; the topple rule on a
hand-computed path graph; the books balancing in every regime;
the Abelian identity at 500 grains; the blocked tree stranding
its far side; the far side lying genuinely beyond the hub).

## Succession

Named in advance: pour the same sand on a *larger* grove (the
real skunkworks tree itself, struck as a crystal) and ask whether
P2's processing gap widens with depth; or block a *leaf's parent*
instead of the hub — the smallest obstacle, priced the same way.
