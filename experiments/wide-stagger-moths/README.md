# Wide-Stagger Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/mirrored-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-10. Survival ledger below — reported as found.

## Lineage

- **`resonance-audio`** provides the room and the medium: the same
  56×28 stone-walled `PhysicsGrid` as `keeper-moths`, both lamps
  speaking as `pluck()`s into one shared wave field.
- **`keeper-moths`** provides the moths, the lamps, and the flicker
  check entire: a moth perceives only the smoothed,
  saturation-clipped brightness and local gradient at its own cell;
  a burned moth earns caution; a cautious moth stops — up to 96
  ticks, exposed — times the peak intervals at its cell, and
  refuses a claim whose interval coefficient of variation exceeds
  0.35.
- The succession since: **`witness-moths`**, **`chain-moths`**,
  **`placed-moths`**, **`evidence-moths`** (checks transmit their
  peak intervals for re-judgement), **`throttled-moths`**,
  **`unshoved-moths`**, **`shoved-moths`**, **`fading-moths`**
  (caution booked by source — the **earned pool**: own burns and
  witnessed burns; the **taught pool**: re-judged evidence packets,
  spent first by acquittals), **`waning-moths`** (caution priced
  by age; the dangerous lifetime is the middle one — burns 16 at
  750), **`pooled-moths`** (waning-750 split by pool: the danger
  lives in neither pool alone, only in both pools waning on the
  same journey), **`staggered-moths`** (both pools at 750, the
  taught pool's deadline offset +375 beside a coincident control
  at the same longest deadline: staggered burns **9**,
  staggered-both **13**, waning-750 **16** — duration priced at
  3 burns, the stagger at 4 more), **`mirrored-moths`** (the
  offset moved to the earned pool: mirrored burns **11** — the
  ordering completed strict, staggered 9 < mirrored 11 <
  coincident-late 13 < coincident-750 16; a stagger symmetric in
  kind and asymmetric in price, the mirror gap exactly 2).
- **`mirrored-moths`** named the succession taken up verbatim
  here, in its next seeds: *a stagger wider than the claim-step
  (~450+ ticks) in this cave still stands — does the mirror gap
  widen, hold at 2, or close when the offset outlasts a whole
  journey leg?*

## The Novel Trait

Every staggered room so far offset a deadline by +375 ticks —
inside a journey leg, well inside the ~450-tick claim-step's
neighbourhood. This planting keeps the evidence room's packets,
re-judgement, aversion and acquittal rule verbatim, books
caution by source exactly as `mirrored-moths` does, expires
**both** pools at lifetime **750**, and widens the one offset
to **+750 ticks** — a full lifetime, wider than the claim-step,
longer than any single journey leg in this cave — in three
rooms:

- **Staggered-wide** — taught units expire at gain + 750 + 750;
  earned units at gain + 750.
- **Mirrored-wide** — earned units expire at gain + 750 + 750;
  taught units at gain + 750. The mirror, at the wide offset.
- **Coincident-wide** (control) — both deadlines offset +750,
  coincident at the longest deadline (2250); only duration
  changes.

Twenty-three twin rooms, same seed: the twenty parents,
unchanged (tests pin their ledgers exactly — including mirrored
fed 46 / burns 11, expired 99 taught / 12 earned; staggered fed
50 / burns 9; staggered-both fed 45 / burns 13; waning-750 fed
48 / burns 16), plus the three wide rooms. The moths know only
what the field delivers and what it cost them; offsets are the
keeper's bookkeeping, never the moth's.

## Predictions, declared before the first run

- **P1:** the channel works wide — in each wide room, both
  verdict kinds and waning-first learners > 0.
- **P2 (the offset is real):** each wide room expires both
  pools (> 0 taught and > 0 earned) and no dawn tax fires in
  it. Asserted by test, together with the offset values
  themselves (taught 750 / earned 0 in staggered-wide; taught
  0 / earned 750 in mirrored-wide; 750 / 750 in
  coincident-wide).
- **P3 (the mirror gap at the wide offset):** the gap holds in
  kind — |mirrored-wide − staggered-wide| ≤ 4, the coincidence
  gap at +375. A gap past 4 makes which pool waits the major
  term at wide offsets, and P3 cracks. Whether the exact gap
  widens past 2, holds at 2, or closes below 2 is reported as
  found.
- **P4 (any wide stagger beats wide coincidence):** both wide
  staggers burn less than coincident-wide — the coincident
  control at the same longest deadline.
- **P5:** no wide room starves the channel — waning events
  ≥ 300 in each.
- **P6:** no wide room starves the table — fed ≥ 43 in each
  (the evidence room's table).

## The Survival Ledger (headless, as found)

24 moths, 3 nights × 1500 ticks, same seed. No deaths in the
wide rooms (24/24 at every dawn).

| room | fed | burns | checks | events | learners | acquittals | expired taught | expired earned |
|---|---|---|---|---|---|---|---|---|
| waning-750 (both pools, coincident) | 48 | **16** | 103 | 502 | 17 | 105 | 58 | 27 |
| staggered (taught +375) | 50 | **9** | 116 | 611 | 18 | 145 | 55 | 15 |
| staggered-both (both +375, coincident) | 45 | **13** | 128 | 836 | 18 | 183 | 58 | 19 |
| mirrored (earned +375) | 46 | **11** | 129 | 662 | 18 | 162 | 99 | 12 |
| staggered-wide (taught +750) | 54 | **12** | 110 | 644 | 18 | 145 | 31 | 20 |
| mirrored-wide (earned +750) | 46 | **12** | 125 | 779 | 17 | 166 | 89 | 17 |
| coincident-wide (both +750, coincident) | 43 | **12** | 151 | 943 | 18 | 222 | 42 | 20 |

Per night — staggered-wide: fed 19/20/15, burns 5/4/3,
expired taught 0/10/21, earned 8/7/5. Mirrored-wide: fed
19/9/18, burns 5/5/2, expired taught 11/22/56, earned 0/8/9.
Coincident-wide: fed 19/11/13, burns 5/6/1, expired taught
0/10/32, earned 0/9/11. (Night 1 matches across every staggered
room — 19 fed, 5 burns — the clocks have not yet had time to be
a difference.)

- **P1 HELD** — staggered-wide irregular 202 / steady 442,
  mirrored-wide 251 / 528, coincident-wide 342 / 601; 18 / 17 /
  18 first learners.
- **P2 HELD, exactly** — every wide room expires both pools
  (taught 31 / 89 / 42, earned 20 / 17 / 20); no dawn tax; the
  offset values are asserted by test.
- **P3 HELD, and the seed's question is answered: the gap
  CLOSED.** At +375 the mirror gap was 2 (mirrored 11 against
  staggered 9). At +750 it is **0** — staggered-wide and
  mirrored-wide both burn **12**. Which pool waits, worth 2
  burns inside a journey leg, is worth nothing past one.
- **P4 CRACKED, and the crack is the headline:** neither wide
  stagger beats its coincident control. Coincident-wide also
  burns **12** — at the wide offset the three deadline
  arrangements are indistinguishable in price: staggered-wide
  12 = mirrored-wide 12 = coincident-wide 12. The waiting
  apart, worth 4 burns against coincidence at +375 (9 against
  13), is worth **0** at +750. Widening also taxed the staggers
  themselves: staggered-wide's 12 is 3 worse than staggered's
  9, mirrored-wide's 12 is 1 worse than mirrored's 11 — while
  the coincident line barely moved (13 at +375, 12 at +750).
  The full ordering across the seven two-pool rooms is now:
  **staggered 9 < mirrored 11 < staggered-wide 12 =
  mirrored-wide 12 = coincident-wide 12 < staggered-both 13 <
  waning-750 16**. There is an optimum offset inside a journey
  leg, and past the leg the deadline arrangement dissolves
  into the longest deadline alone.
- **P5 HELD** — events 644 / 779 / 943 ≥ 300.
- **P6 HELD, on the line in the control** — fed 54 / 46 / 43 ≥
  43; coincident-wide feeds exactly at the evidence room's
  table and not one moth more.

**The law:** a stagger only exists while the offset fits
inside a journey. Inside a leg, waiting apart is worth 4 burns
and which forgetting waits is worth 2; past a leg, both prices
fall to zero together, and three different arrangements of the
same deadlines burn as one room. Before widening a wait, ask
whether anything still travels the distance it spans — a
stagger wider than the journey is just a longer deadline
wearing the stagger's coat.

## Falsification condition

This planting is wrong if, on another seed or cave, either wide
stagger burns at or above the coincident control at the same
longest deadline (staggered-wide or mirrored-wide ≥
coincident-wide) — that would make a stagger wider than a
journey leg no stagger at all — or if the wide mirror gap
exceeds the wide coincidence gap itself on a second seed,
which would promote which pool waits from a priced detail to
the whole story at wide offsets.

## Next seeds

- The witness room under a dawn tax still stands (does its
  earned-only protection, burns 8, survive expiry of the earned
  pool?).
- A lifetime priced in claims directly (units expiring after N
  of their owner's claims, not ticks) still stands.
- - The stagger at exactly the claim-step (+450) still stands
  — the optimum found at +375, the dissolution at +750: is the
  peak at the step itself, or does the price already turn
  before it? (And its mirror, for the gap's last reading
  before it closes.)

*Every lamp is a claim. Check the claim — and when you widen
the wait past the road itself, check whether the mirror still
answers at the same price.*
