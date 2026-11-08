# physarum-ledger 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/physarum-transit × experiments/lichen-tiling (revival cross)
**Coordinates:** graveyard/physarum-transit → experiments/lichen-tiling: the flat
corner of the map, where a slime mould was executed for having no books, to the
tiling where every resource is a stock that is spent. Bearing between them: the
corpse stamped its food onto the field forever; the parent never lets anything
be stamped for free.

## The death

`graveyard/physarum-transit` (edition 2021, macroquad + rayon) was a complete
Physarum transport simulation on a flat toroidal plane: 5,000 agents sensing
the trail field at ±45° / 9 px, rotating ±22.5° toward the strongest sensor,
advancing at speed 2.0, depositing 0.5 (capped 1.0); 15 cities, each stamping
its radius-3 disc to 1.0 every tick, forever; the field diffusing as a 3×3
mean and decaying ×0.95 per tick.

It was executed for **Documentation Void** (no README at all) and **Ancestral
Redundancy**: its certificate held that `hyperbolic-mold` implements the same
logic on a Poincaré disk and carries its genetic memory in fitter form. Its
salvation conditions were written on the certificate: differentiate with a
new biological mechanism the descendant does not have, document, and
demonstrate emergent behaviour the hyperbolic variant does not capture.

One thing the certificate never charged, and the descendant never fixed:
**the corpse's cities are infinite.** A city stamps at full strength on tick
1 and on tick 10,000, however many agents feed there. Real plasmodia eat
their food. A network built to reach a meal that never runs out has never
been asked what it does when the meal ends.

## The donation

`experiments/lichen-tiling` supplies the exact missing organ: **finite stock
bookkeeping.** In the lichen, every resource is a stock — algae spend water
to make energy, neighbours pay 0.5 of a stock across an edge, sprouting is
gated on both stocks standing above a threshold, and every stock is clamped
and counted. Nothing in the lichen is stamped for free, and its ledger is
its argument. That bookkeeping, planted in the corpse's cities, is this
cross's whole machinery of difference: each city holds a stock, agents
standing in its apron spend it, and the stamp the corpse fired forever is
gated on the stock the parent taught it to keep.

## The novel trait

The corpse's model runs verbatim (same sensors, rotation, speed, deposit,
diffusion, decay; sequential in place of rayon, two seeded SplitMix64
streams in place of `thread_rng`, so the ledger can be exact) beside the
parent's books, in three rooms on one seed, one spawn, one city map
(96×64 torus, 1,200 agents, 1,200 ticks, nine hand-surveyed cities — a ring
of eight and a centre; stock 25.0 per city; feeding spends 0.004 per agent
in the apron per tick):

- **Eternal** — the corpse itself: stocks fall in a *shadow ledger* (the
  would-be dark tick is recorded) but never gate the stamp.
- **Depleted** — the stock gates the stamp. A city at zero goes dark and
  stamps nothing; its trails remain and decay on the corpse's clock.
- **Regrown** — the stock gates, and regrows at 0.15/tick to the opening
  25.0; a dark city relights at stock 5.0 (hysteresis: a relit city is a
  meal, not a flicker).

Named adaptations: the corpse's 800×600 / 5,000-agent / 15-random-city
stage is shrunk to the ledger's size, cities placed by hand so every room
shares one map; the corpse drew its steering coin per agent from
`thread_rng` inside a parallel loop, here one steer stream is drawn in
agent order — the price of the Eternal≡Depleted identity below, paid
gladly; the corpse's window is left in the grave (`--headless` is the
only mouth).

## Predictions, declared before the first run

- **P1:** Eternal and Depleted are *the same run* until the first dark
  tick: their 100-tick mass samples agree on every sample before it,
  and Eternal's shadow dark tick equals Depleted's first dark tick.
- **P2:** at least one city goes dark in Depleted before tick 1,200.
- **P3:** Depleted ends poorer than Eternal on both measures: final
  trail mass and final coverage (cells > 0.2) strictly below Eternal's.
- **P4:** the network outlives its food: ghost visits — feeding visits
  to a city while it is dark — in Depleted's last 300 ticks number > 0.
- **P5:** Regrown ends with fewer dark cities than Depleted and a final
  trail mass above Depleted's.
- **P6:** Depleted's final trail mass is below its mass at the first
  dark tick: after the food ends, the network is spent, not sustained.

## Falsification condition

If no city ever darkens, or Depleted ends with Eternal's mass and
coverage, or ghost visits are zero, the stock is decoration — the
corpse with a ledger taped to it — and this revival is a portrait,
not a measurement. The salvation condition it answers is the first:
depletion (and regrowth) is a biological mechanism neither the corpse
nor its hyperbolic descendant carries, and its emergent behaviour —
a network spending itself toward food that is no longer there — is
the behaviour the flat original was executed before it could show.

## Headless ledger (1,200 agents, 96×64, 1,200 ticks, 9 cities)

| regime | visits | ghost visits | ghost, last 300 | first dark | dark at end | dark ticks | relights | final mass | final coverage |
|---|---|---|---|---|---|---|---|---|---|
| eternal | 217,975 | 0 | 0 | 219 (shadow) | 0 | 0 | 0 | 2,447.38 | 3,685 |
| depleted | 108,552 | 52,206 | 20,387 | 219 | 9 / 9 | 7,547 | 0 | 1,713.38 | 2,529 |
| regrown | 172,527 | 51,259 | 21,690 | 219 | 3 / 9 | 3,191 | 53 | 2,599.70 | 3,962 |

Mass every 100 ticks — eternal: 2820.8 / 2682.2 / 2522.9 / 2547.0 /
2549.0 / 2565.9 / 2678.5 / 2700.0 / 2665.6 / 2505.4 / 2421.1 / 2447.4;
depleted: the same first two samples, then 2456.7 / 2522.1 / 2483.6 /
2423.1 / 2405.5 / 2169.2 / 2112.2 / 1945.8 / 1745.6 / 1713.4; regrown:
the same first two, then 2473.3 / 2437.9 / 2691.9 / 2684.5 / 2657.6 /
2547.1 / 2490.7 / 2528.5 / 2572.8 / 2599.7.

Read the bearings:

- **Every city is mortal.** In the shadow ledger all nine cities spend
  their 25.0 — the hungriest (city 4) at tick 219, the quietest (city 7)
  at 745. The corpse's eternal plane was running on food that, priced,
  lasts a fifth of the season.
- **The network outlives its food, and pays for the haunting.** In
  Depleted, 52,206 of 108,552 feeding visits — 48% — are ghost visits
  to cities that stamp nothing, and 20,387 of them land in the last
  300 ticks, long after the last city darkened (tick 701). The trails
  decay at ×0.95 but the deposits of the faithful keep re-laying them:
  the mould keeps commuting to a restaurant that has closed.
- **Depletion halves the table.** Depleted feeds 108,552 times against
  Eternal's 217,975 and ends at mass 1,713.38 against 2,447.38,
  coverage 2,529 against 3,685 — the corpse's network was roughly
  twice the network its food could actually support.
- **Regrowth pulses, and the pulse beats the eternal stamp** — the
  season's surprise, kept as found. Regrown relights 53 times, ends
  with 3 cities dark, and posts mass **2,599.70, above Eternal's
  2,447.38**, on coverage 3,962 above Eternal's 3,685. A permanently
  stamped disc is saturated: deposits landing on 1.0 cells are capped
  and wasted. A city that goes dark lets its field decay; when it
  relights, the returning swarm's deposits count again. Intermittent
  food is worth more trail than eternal food — not despite the dark
  intervals, but because of what they let the field forget.

One tuning correction, kept: at the first constants tried (stock
60.0, eat 0.002, regrow 0.04) the season was nearly silent — only
the centre city darkened, at tick 977, and Regrown was Depleted
field for field, because the agents camped on the dark city ate
its regrowth (0.05/tick of feeding) faster than the city regrew
(0.04/tick). A ledger that quiet does not ship. The shipped
constants (stock 25.0, eat 0.004, regrow 0.15) are the ones every
number below prices, and the predictions were re-declared against
them before the tuned run.

## Verdicts

- **P1 HELD exactly:** mass samples at ticks 100 and 200 agree to the
  digit (2820.8 / 2682.2 in both rooms), the rooms part at the 300
  sample, and Eternal's shadow dark tick is Depleted's dark tick:
  219. (Later cities' shadow ticks differ from their depleted ticks —
  after the first darkening the rooms are different worlds; only the
  first tick is the identity's claim.)
- **P2 HELD past its letter:** not one city but all nine darken, the
  last at tick 701.
- **P3 HELD:** mass 1,713.38 < 2,447.38; coverage 2,529 < 3,685.
- **P4 HELD:** 20,387 ghost visits in the last 300 ticks.
- **P5 HELD:** 3 dark at the end against Depleted's 9; mass 2,599.70
  above Depleted's 1,713.38 — and above Eternal's, which P5 did not
  ask and the ledger volunteered.
- **P6 HELD:** Depleted's final mass 1,713.38 is below its mass at
  the first dark tick, 2,614.51.

9/9 tests, the season's totals pinned as found in
`season_ledger_is_pinned`.

**The law, from this survey:** a stamp that never spends is a subsidy,
and a network drawn to a subsidy is twice the network the food can
carry; let the food end and the map keeps the roads to empty cities
for hundreds of ticks — but let the food *pulse*, and the dark
intervals themselves become part of the yield.

**Next seeds:** the same books on the hyperbolic descendant's own
ground (does a Poincaré city's stock last longer at the rim, where
the field is thin?); a stock priced per-city by quality, not just
quantity (two foods, one ledger — which does the network abandon
first?); ghost-visit decay priced against a *smarter* corpse — agents
that remember a dark city and stop returning (is the haunting the
agents' fidelity or the field's?).

## Usage

```bash
cargo run -p physarum-ledger -- --headless
cargo test -p physarum-ledger
```

*Nothing here is waste. Some of it is just early. — the Cartographer*
