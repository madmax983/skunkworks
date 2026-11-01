# lichen-tiling 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/hyperbolic-lichen × crates/poincare-disk (revival cross)

## The death

`graveyard/hyperbolic-lichen` (548 LOC, macroquad, edition 2021) was a
complete lichen symbiosis living on a {4,5} Poincaré tiling: Algae cells
turn water into energy, Fungus cells turn energy into water, neighbours
trade 0.5 of what the other makes across every shared edge, and a cell
rich in both (energy > 20, water > 20) sprouts into empty neighbours at
p = 0.05. It was executed on 2026-02-22 for **Documentation Void /
Zombie Status alone** — no README, a GUI window as its only mouth, its
metabolism never counted.

It carried a second, quieter flaw nobody charged it with: its tiling
nodes were registered only where a *player* walked (`register_node` was
called from the exploration path), so the lichen could only ever grow
on ground someone had already visited. Most of its world never existed.

## The donation

The living `crates/poincare-disk` supplies the exact missing organ:
the tiling itself, whole. `TilingConsts::new_4_5()` and
`neighbor_transform_a` — the corpse's own imports — generate the full
{4,5} ball by breadth-first Möbius stepping (neighbour hyperbolic step
1.0613), every cell registered before tick 0, no player required.
Adjacency is read off the geometry: two cells are neighbours iff their
hyperbolic distance is within 5% of the step length.

## The novel trait

The corpse's update rule runs **verbatim** (base metabolism
−0.05/−0.05; Algae: +1.0 energy for −0.2 water if water > 1, and 0.5
energy paid to each Fungus neighbour; Fungus the mirror; clamps
0..100; sprouting as above, sprouts born at 5/5) on two grounds at
once:

- **The tiling ball:** 257 cells, 336 edges, mean degree 2.61 —
  and 62.3% of its cells sit in the outermost ring. A finite
  hyperbolic ball is mostly frontier.
- **The flat control:** a 16×16 von-Neumann grid, 256 cells, 480
  edges, mean degree 3.75, outer ring 23.4% of cells — the ground
  the lichen never had.

Both grounds are seeded by the corpse's own hash rule (r < 0.2 Algae,
r > 0.8 Fungus, else Empty; both grounds seed 51 Algae / 51 Fungus)
and run 400 ticks under one deterministic SplitMix64 stream.

Adaptations, named: the corpse drew sprout rolls from `thread_rng`;
here the stream is fixed-seeded and reproducible. The corpse's cells
carried an `age` field its update never incremented; it is not
carried here.

## Predictions (declared before the first run, never edited)

- **P1:** the tiling colonises more empty cells than the grid
  (the frontier-rich ground pays its interface).
- **P2:** both symbionts persist on both grounds at tick 400.
- **P3:** the colonised frontier reaches a greater ring on the
  tiling than on the grid.
- **P4:** trade volume per mixed (Algae–Fungus) edge per tick
  agrees between grounds within ±25% (the rule is local; the
  ground changes the traffic, not the price).

## The ledger (headless, 400 ticks per ground)

| | tiling {4,5} | grid 16×16 |
|---|---|---|
| cells / edges / mean degree | 257 / 336 / 2.61 | 256 / 480 / 3.75 |
| seeded Algae / Fungus / Empty | 51 / 51 / 155 | 51 / 51 / 154 |
| final Algae / Fungus / Empty | 90 / 105 / 62 | 123 / 129 / 4 |
| colonised (conversions) | **93** | **150** |
| trade volume | 21,011.0 | 44,135.0 |
| mixed-edge-ticks | 21,011 | 44,135 |
| trade per mixed-edge-tick | **1.000** | **1.000** |
| colonised frontier ring | 5 of max 5 | 15 of max 16 |
| typed means (energy / water) | 45.83 / 47.36 | 69.24 / 69.48 |

- **P1 CRACKED.** The frontier-rich ground colonised *less* (93 vs
  150). The falsification condition below therefore fires.
- **P2 HELD.** Both symbionts persist on both grounds (90/105 and
  123/129).
- **P3 CRACKED as declared** (ring 5 vs 15) — with the surveyor's
  caveat kept beside it: the tiling's frontier reached 5 of its 5
  rings (every ring it has), the grid's 15 of 16. Raw ring counts
  compare grounds of different depth; the share is 100% vs 94%.
- **P4 HELD, exactly.** Trade volume *equals* mixed-edge-ticks on
  both grounds, to the unit: each mixed edge trades exactly 1.0
  per tick (0.5 each way, unconditionally), on curved ground and
  flat alike. The ground changed how many mixed edges ever exist
  (21,011 vs 44,135 edge-ticks) and never the price of one.

**The finding:** colonisation here is not priced by frontier. The
tiling is 62.3% frontier and colonised less, because a finite
hyperbolic ball's frontier cells are *degree-starved within the
ball* (mean degree 2.61 vs 3.75): fewer neighbours means fewer
trade partners, lower income (typed means 45.8/47.4 vs 69.2/69.5),
fewer rich cells, fewer sprouts. Growth is priced by degree and
the income degree buys — the map's edge is not free interface; it
is where coordination goes to be poor.

## Falsification condition

Declared in advance: if the grid colonises as much as or more than
the tiling (P1 fails), the frontier-pricing account of this
symbiosis is wrong and growth is priced by something the grounds
share. **It fired.** The shared currency the ledger names is
degree: trade income per cell, and with it the count of cells rich
enough to sprout. A future planting that gives the ball's frontier
its missing outward neighbours (a ghost ring that trades but is
not counted) should, under this account, close most of the 93 vs
150 gap; if it does not, degree is not the currency either.

## Run it

```
cargo test -p lichen-tiling
cargo run -p lichen-tiling -- --headless
```

`--headless` is handled before anything else; there is no TUI.
6/6 tests: the tiling step is one hyperbolic length in all four
directions; the ball is connected with degree ≤ 4 and consistent
rings; the corpse's trade moves exactly 0.5 both ways on top of
each partner's own metabolism; a rich cell colonises an empty
neighbour; the seed hash reproduces the corpse's proportions;
the headless run is deterministic and colonises.

*Coordinates: graveyard/hyperbolic-lichen → crates/poincare-disk.
Bearing: the dead walked a tiling one visitor at a time; the living
crate draws the whole map before the first cell wakes. Here the map
was wrong — the frontier was the poorest ground on it, not the
richest. Nothing here is waste. Some of it is just early.*
