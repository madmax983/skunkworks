# Firstedge Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/secondedge-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-18. Survival ledger below — reported as found.

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
  750), **`pooled-moths`** (the danger lives in neither pool alone,
  only in both pools waning on the same journey),
  **`staggered-moths`** (taught deadline offset +375 beside a
  coincident control: staggered burns **9**, coincident **13**,
  waning-750 **16**), **`mirrored-moths`** (the offset moved to
  the earned pool: mirrored burns **11** — the mirror gap exactly
  2), **`wide-stagger-moths`** (+750: all three rooms **12** — the
  arrangement dissolved), **`claimstep-moths`** (+450 reproduces
  +375 field for field — the optimum is a **shelf**, not a peak),
  **`shelfedge-moths`** (+600: staggered **16** — the shelf ends
  in a cliff somewhere in (450, 600]), **`cliffface-moths`**
  (+525: the whole seven-burn fall inside the first 75 ticks past
  the claim-step), **`facegrain-moths`** (+475 and +500: at
  25-tick grain the cliff is a stair of two steps),
  **`stairgrain-moths`** (+462 and +487: the taught line's first
  step stands east, the coincident line's west),
  **`halfgrain-moths`** (+456, +468 and +481: every halved edge
  stands in its **eastern half** — and the second step is not
  shared: the two lines cross it in different halves of
  (475, 487], their treads overlapping at exactly one measured
  address, +475), **`quartergrain-moths`** (+459, +471, +478 and
  +484: one edge east, two edges west, and the fourth — the
  taught line's first step — **dissolved into a split at +471**:
  staggered-471 burns **10**, neither the shelf's 9 nor the
  tread's 14, a price between two prices, read by journeys that
  part at the same address; its mirror gap read **+1** there, a
  value the biography had never minted), **`eighthgrain-moths`**
  (+469, +470, +472 and +473: the split occupies **exactly one
  address** — staggered-470 reads the shelf entire, staggered-472
  the tread entire — and the taught line's first step closed at
  integer grain), **`secondedge-moths`** (+476, +477, +482 and
  +483: both **second** edges are clean steps standing in the
  **middle thirds** of their intervals — the taught second in
  (482, 483], the coincident second in (476, 477] — no split at
  either, no intermediate price at any of the four addresses).
- **`secondedge-moths`** named the succession taken up verbatim
  here, in its next seeds: *the coincident line's first edge,
  priced at its interiors — the coincident first edge stands in
  (459, 462], its interior addresses +460 and +461 unpriced: do
  they read the shelf entire (13) and the tread entire (14),
  standing that edge in (460, 461] like its second, or does the
  coincident line — which has never split — keep its own
  surprise somewhere?*

## The Novel Trait

Every grain so far halved an interval, priced a point's
neighbours, or closed a second edge. This planting prices the
**interior addresses of the coincident line's first-edge
interval itself**: the evidence room's packets, re-judgement,
aversion and acquittal rule are kept verbatim, caution is booked
by source exactly as `secondedge-moths` does, both pools expire
at lifetime **750**, and the one offset is set to **+460** or
**+461** (the two interior addresses of the coincident first
edge's interval (459, 462]) — in three rooms at each address:

- **Staggered-460 / -461** — taught units expire at gain + 750 +
  offset; earned units at gain + 750. Controls inside the taught
  shelf (9 at both +459 and +462).
- **Mirrored-460 / -461** — earned units expire at gain + 750 +
  offset; taught units at gain + 750. The mirror, at the first
  edge.
- **Coincident-460 / -461 (the controls that are the question)**
  — *both* pools expire at gain + 750 + offset. The deadlines
  stay coincident with each other; only the duration is longer
  (longest deadlines 1960 / 1961).

Ninety-five twin rooms, same seed: the eighty-nine parents,
unchanged (tests pin their ledgers exactly — including
staggered-471 fed 50 / burns 10, the split itself), plus the six
first-edge rooms. At +460/+461 the staggered rooms sit inside
the taught shelf and the mirrored rooms inside the earned flat,
and serve as controls while the coincident rooms price their
own first edge. The moths know only what the field delivers and
what it cost them; offsets are the keeper's bookkeeping, never
the moth's.

## Predictions, declared before the first run

- **P1:** the channel works at both first-edge grains — both
  verdict kinds and firstedge-first learners > 0 in each new
  room.
- **P2 (the offsets are real):** both new rooms at each grain
  expire both pools (> 0 taught and > 0 earned) and no dawn
  tax fires in either. Asserted by test, together with the
  offset values themselves (taught 460 / 461 in the staggered
  rooms; earned the same in the mirrors; both in the controls).
- **P3 (the coincident first edge is not in its western
  third):** coincident-460 burns read exactly 13 — the shelf
  entire at the western interior address. A reading of 14
  moves the edge into (459, 460]; any other reading names a
  surprise there and cracks P3 either way.
- **P4 (the coincident first edge stands in its middle third,
  like its second):** coincident-461 burns read exactly 14 —
  the tread entire at the eastern interior address. Together
  P3 and P4 stand the coincident first edge in (460, 461],
  the middle third of its interval, twin to the coincident
  second edge's (476, 477]. A reading of 13 at +461 moves the
  edge into (461, 462]; any other reading cracks P4.
- **P5 (the control interiors are flat):** staggered-460 and
  staggered-461 burns read exactly 9 (the taught shelf is flat
  here — the taught line's own first step lives far east, at
  (470, 472] with its split at +471), and mirrored burns read
  exactly 11 at BOTH grains (the earned schedule's single step
  still lives between +525 and +600).
- **P6 (no intermediate prices at the coincident first edge):**
  every staggered room here reads 9, every coincident room
  reads 13 or 14, and every mirrored room reads 11. Any other
  price — 10, 12, 15, 16 — names an intermediate price at the
  coincident first edge, a thing the coincident line has never
  read at any edge.
- **P7:** no new room starves the channel or the table —
  waning events ≥ 300 and fed ≥ 43 in each (the evidence
  room's table).

## The Survival Ledger (headless, as found)

24 moths, 3 nights × 1500 ticks, same seed. No deaths in five
of the six new rooms (24/24 at every dawn); coincident-461
carries one death on night 2 (23 at dawn thereafter) — inherited
entire from its parent headline, coincident-462's, which carries
the same death on the same night.

| room | fed | burns | checks | events | learners | acquittals | expired taught | expired earned |
|---|---|---|---|---|---|---|---|---|
| waning-750 (both pools, coincident) | 48 | **16** | 103 | 502 | 17 | 105 | 58 | 27 |
| staggered-460 (taught +460) | 50 | **9** | 116 | 611 | 18 | 146 | 54 | 15 |
| mirrored-460 (earned +460) | 46 | **11** | 129 | 662 | 18 | 162 | 99 | 12 |
| coincident-460 (both +460) | 45 | **13** | 128 | 836 | 18 | 183 | 55 | 19 |
| staggered-461 (taught +461) | 50 | **9** | 116 | 611 | 18 | 146 | 54 | 15 |
| mirrored-461 (earned +461) | 46 | **11** | 129 | 662 | 18 | 162 | 99 | 12 |
| coincident-461 (both +461) | 48 | **14** | 121 | 585 | 16 | 141 | 50 | 18 |

Per-night, the two rooms with the question in them:
coincident-460 fed 19/9/17, burns 5/4/4; coincident-461 fed
19/13/16, burns 5/5/4. The equivalences are headline-entire, all
six of them: staggered-460 and staggered-461 *are* the shelf
rooms (staggered-459's headline: fed 50, checks 116, events 611,
acquittals 146, expired 54 / 15); mirrored-460 and mirrored-461
*are* the mirrored headline (fed 46, checks 129, events 662,
acquittals 162, expired 99 / 12); coincident-460 *is*
coincident-459's headline entire (fed 45, checks 128, events
836, acquittals 183, expired 55 / 19); coincident-461 *is*
coincident-462's headline entire (fed 48, checks 121, events
585, acquittals 141, expired 50 / 18). Six new rooms, four old
headlines among them — the two coincident interiors minted no
new headline either: each is a parent entire, and the edge
stands between them.

### Verdicts

- **P1 HELD** — every new room witnesses both verdict kinds
  (irregular 190–269, steady 387–570 across the six) and every
  new room has firstedge-first learners (16–18).
- **P2 HELD, exactly** — the six rooms expire taught/earned
  54/15, 99/12, 55/19, 54/15, 99/12, 50/18; no dawn tax in
  any; the offset values are asserted by test.
- **P3 HELD, exactly** — coincident-460 burns **13**, the shelf
  entire, coincident-459's headline entire. The coincident
  first edge does not stand in (459, 460].
- **P4 HELD, exactly** — coincident-461 burns **14**,
  coincident-462's headline entire. Together P3 and P4 stand
  the coincident first edge in **(460, 461]**, the middle
  third of its interval — twin to the coincident second
  edge's (476, 477], as the seed hoped.
- **P5 HELD, exactly** — staggered reads **9** at both grains
  (the taught shelf's interior is flat across the whole
  coincident interval), mirrored reads **11** at both grains.
  The mirror gap reads **+2** at +460 and +461 both (11
  against the shelf's 9, the gap of the shelf, as at +375,
  +450, +456 and +459).
- **P6 HELD, exactly — no intermediate prices at the
  coincident first edge.** Every staggered room reads 9, every
  coincident room reads 13 or 14, every mirrored room reads
  11. The coincident line keeps its record: across both its
  edges, priced now at every interior address of both
  intervals, it has never split and never read a price between
  prices.
- **P7 HELD** — events 585–836 ≥ 300; fed 45–50 ≥ 43 in each.

The coincident first-edge interval is now **closed at integer
grain**: every integer address inside (459, 462] is priced, and
the edge occupies exactly the one sub-interval its interior
readings leave open — (460, 461]. With it, **both coincident
edges are closed**, and the coincident line's biography is
complete at integer grain on its western half: shelf 13 at
+375 / +450 / +456 / +459 / +460, tread 14 at +461 / +462 /
+468 / +469 / +470 / +471 / +472 / +473 / +475 / +476 — a tread
**ten** addresses wide with no split anywhere inside it — then
the bottom 16 from +477. The taught schedule gains two more
shelf addresses (9 at +460 and +461; its shelf now stands ten
addresses wide before its split at +471). The ordering across
the two-pool family gains no new rung: **the coincident line's
first edge, like its second, minted nothing — it only told us
where it stands, and it stands in the middle, again.**

**The law:** a line that has never split earns the presumption
of steps — but the presumption is priced, not free. The
coincident line has now been priced at every interior address
of both its edge intervals and read a parent entire at every
one: both its steps stand in the middle thirds of their
intervals, (460, 461] and (476, 477], sixteen ticks apart in
address and identical in shape. Where the taught line spent
its one surprise — a split at a single address — the coincident
line spent none. Before fearing a split on a line with no split
in its biography, price its interiors and expect a step in the
middle; keep the fear for the line that has split before.

## Falsification condition

This planting is wrong if, on another seed or cave, the
first-edge rooms separate from these readings (coincident-460
≠ 13 or coincident-461 ≠ 14 or staggered ≠ 9 or mirrored ≠ 11
at either grain here) — that would make the middle-third
standing a property of this seed's journeys, not of the
addresses — or if a room at +471 on another seed reads 9 or 14
entire, which would un-split the landing quartergrain reported
and reopen the first step this family believes closed.

## Next seeds

- The coincident line's fall from the bottom — coincident
  reads 16 from +477 through +600 (face and edge) and 12 at
  +750 (wide): the bottom ends somewhere in (600, 750]. At
  +675, midway, does the coincident line read the bottom
  entire (16) or the wide room entire (12) — and does this
  edge, the line's third, also stand in a middle, or does the
  line keep its first surprise for its last step?
- The shelf's western edge — a stagger below +375 (+150,
  +225): does the price rise toward coincidence as the offset
  shrinks inside the leg, or is the shelf flat all the way
  down to the coincident rooms?
- The witness room under a dawn tax still stands (does its
  earned-only protection, burns 8, survive expiry of the
  earned pool?).
- A lifetime priced in claims directly (units expiring after
  N of their owner's claims, not ticks) still stands.

*Every lamp is a claim. Check the claim — and when a line has
never split, price its interiors anyway: the middle is where
its steps stand, and now both of this line's steps have been
caught standing there.*
