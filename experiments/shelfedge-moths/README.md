# Shelfedge Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/claimstep-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-11. Survival ledger below — reported as found.

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
  mirror gap exactly 2), **`wide-stagger-moths`** (the offset
  widened to +750: staggered-wide **12** = mirrored-wide **12**
  = coincident-wide **12** — the arrangement dissolved; the
  mirror gap closed 2 → 0, the waiting apart fell 4 → 0),
  **`claimstep-moths`** (the offset at exactly the claim-step,
  +450: staggered-step **9** and mirrored-step **11** reproduce
  their +375 parents field for field — the optimum is a **shelf**,
  not a peak, running at least from +375 through the claim-step;
  the whole turn lives somewhere in the 300 ticks between the
  step and the wide offset).
- **`claimstep-moths`** named the succession taken up verbatim
  here, in its next seeds: *the stagger at +600 — midway between
  the claim-step and the wide offset: where on the shelf's edge
  does the price turn, and does the mirror gap close before,
  with, or after it?*

## The Novel Trait

Every staggered room so far offset a deadline by +375 ticks —
inside a journey leg — by +450, at the claim-step itself, or by
+750, past one. This planting keeps the evidence room's packets,
re-judgement, aversion and acquittal rule verbatim, books caution
by source exactly as `claimstep-moths` does, expires **both**
pools at lifetime **750**, and sets the one offset to **+600
ticks** — midway between the claim-step and the wide offset, on
the shelf's edge — in three rooms:

- **Staggered-edge** — taught units expire at gain + 750 + 600;
  earned units at gain + 750.
- **Mirrored-edge** — earned units expire at gain + 750 + 600;
  taught units at gain + 750. The mirror, at the edge.
- **Coincident-edge (the control)** — *both* pools expire at
  gain + 750 + 600. The deadlines stay coincident with each
  other; only the duration is longer, and the longest deadline
  (2100) sits between the step control's (1950) and the wide
  control's (2250).

Twenty-nine twin rooms, same seed: the twenty-six parents,
unchanged (tests pin their ledgers exactly — including
staggered-step fed 50 / burns 9, mirrored-step fed 46 / burns
11, coincident-step fed 45 / burns 13, the wide rooms at
12 / 12 / 12, waning-750 fed 48 / burns 16), plus the three
edge rooms. The moths know only what the field delivers and
what it cost them; offsets are the keeper's bookkeeping, never
the moth's.

## Predictions, declared before the first run

- **P1:** the channel works at the edge — both verdict kinds and
  edge-first learners > 0 in each new room.
- **P2 (the offset is real):** both new rooms expire both pools
  (> 0 taught and > 0 earned) and no dawn tax fires in either.
  Asserted by test, together with the offset values themselves
  (taught 600 / earned 0 in staggered-edge; 0 / 600 in the
  mirror; 600 / 600 in the control).
- **P3 (the turn):** staggered-edge burns land in 10..=12 — the
  shelf has ended by the midpoint and the price is turning
  toward the +750 dissolution (12) without passing it. A landing
  at 9 says the shelf runs past the midpoint entire; a landing
  past 12 says the turn overshoots dissolution; either cracks
  P3.
- **P4 (the gap closes WITH the turn):** |mirrored-edge −
  staggered-edge| ≤ 1 — the mirror gap, 2 at +375 and still 2
  at +450, 0 at +750, has begun to close by the midpoint, with
  the price turn and not after it. A gap still at 2 beside
  turned prices says the gap closes after the turn; a gap at 0
  beside an unturned price says it closed before; either
  ordering is reported as found.
- **P5 (the coincident line barely moves):** coincident-edge
  burns land in 12..=13, as the coincident arrangements did at
  +375 (13), +450 (13) and +750 (12).
- **P6:** no edge room starves the channel or the table —
  waning events ≥ 300 and fed ≥ 43 in each (the evidence room's
  table).

## The Survival Ledger (headless, as found)

24 moths, 3 nights × 1500 ticks, same seed. No deaths in any
edge room (24/24 at every dawn).

| room | fed | burns | checks | events | learners | acquittals | expired taught | expired earned |
|---|---|---|---|---|---|---|---|---|
| waning-750 (both pools, coincident) | 48 | **16** | 103 | 502 | 17 | 105 | 58 | 27 |
| staggered (taught +375) | 50 | **9** | 116 | 611 | 18 | 145 | 55 | 15 |
| staggered-both (both +375, coincident) | 45 | **13** | 128 | 836 | 18 | 183 | 58 | 19 |
| mirrored (earned +375) | 46 | **11** | 129 | 662 | 18 | 162 | 99 | 12 |
| staggered-wide (taught +750) | 54 | **12** | 110 | 644 | 18 | 145 | 31 | 20 |
| mirrored-wide (earned +750) | 46 | **12** | 125 | 779 | 17 | 166 | 89 | 17 |
| coincident-wide (both +750, coincident) | 43 | **12** | 151 | 943 | 18 | 222 | 42 | 20 |
| staggered-step (taught +450) | 50 | **9** | 116 | 611 | 18 | 145 | 55 | 15 |
| mirrored-step (earned +450) | 46 | **11** | 129 | 662 | 18 | 162 | 99 | 12 |
| coincident-step (both +450, coincident) | 45 | **13** | 128 | 836 | 18 | 183 | 55 | 19 |
| staggered-edge (taught +600) | 56 | **16** | 119 | 656 | 18 | 159 | 46 | 18 |
| mirrored-edge (earned +600) | 46 | **12** | 124 | 778 | 17 | 165 | 89 | 19 |
| coincident-edge (both +600, coincident) | 46 | **16** | 140 | 800 | 18 | 178 | 60 | 20 |

Per night — staggered-edge: fed 19/20/17, burns 5/4/7, expired
taught 0/21/25, earned 8/7/3. Mirrored-edge: fed 19/9/18,
burns 5/5/2, expired taught 11/22/56, earned 0/9/10.
Coincident-edge: fed 19/11/16, burns 5/6/5, expired taught
0/24/36, earned 0/9/11. (Night 1 matches across every staggered
room — 19 fed, 5 burns — the clocks have not yet had time to be
a difference.)

- **P1 HELD** — staggered-edge irregular 235 / steady 421,
  mirrored-edge 253 / 525, coincident-edge 267 / 533; 18 / 17 /
  18 first learners.
- **P2 HELD, exactly** — staggered-edge expires 46 taught / 18
  earned, the mirror 89 / 19, the control 60 / 20; no dawn tax
  in any; the offset values are asserted by test.
- **P3 CRACKED — and the crack is the finding.** Staggered-edge
  burns **16**, not 10..=12: the price does not turn toward the
  dissolution at the midpoint, it falls clean past it. Sixteen
  is waning-750's own price — the unstaggered room's — and it is
  coincident-edge's price too: at +600 a taught stagger buys
  exactly nothing, not even against its own control. The shelf
  does not slope down from the claim-step; somewhere between
  +450 and +600 it ends in a cliff, and the taught side lands at
  the bottom of it, at the full unstaggered price, 4 burns
  *below* the wide rooms' dissolution it was supposed to be
  approaching.
- **P4 CRACKED, in the direction nobody declared.** The mirror
  gap at +600 is **−4**: mirrored-edge burns **12** against
  staggered-edge's 16. The gap did not close toward 0 on its
  way from 2 (at +375, still 2 at +450) to 0 (at +750) — it
  swung through zero and out the other side, and at +750 it
  must swing back. Which pool waits, worth +2 burns inside the
  shelf, is worth −4 at the edge: at +600 the *earned* stagger
  is the only stagger left standing, holding exactly the
  dissolution price (12) while its taught twin pays the
  unstaggered price entire. The gap closes neither before nor
  with nor after the turn — the two pools do not share a turn:
  the taught price cliffs between +450 and +600, the earned
  price is already at its +750 reading by +600, and the gap's
  sign is just the difference of two unrelated schedules.
- **P5 CRACKED** — coincident-edge burns **16**, outside the
  declared 12..=13: the coincident line, so well-behaved at
  +375 (13), +450 (13) and +750 (12), jumps to the unstaggered
  price at the midpoint too. Even *duration alone*, with no
  stagger to dissolve, is charged 16 at +600 — the cliff is
  not a property of the stagger arrangement but of the longest
  deadline's neighbourhood itself: deadlines landing a
  lifetime-plus-600 from gain strand journeys that deadlines
  at +450 and at +750 both spare.
- **P6 HELD** — events 656 / 778 / 800 ≥ 300; fed 56 / 46 / 46
  ≥ 43 in each. Staggered-edge feeds the most moths of any
  two-pool room (56) while burning at the unstaggered price —
  the cliff costs burns, not dinners.

The thirteen-room ordering across the two-pool family is now:
**staggered 9 = staggered-step 9 < mirrored 11 =
mirrored-step 11 < staggered-wide 12 = mirrored-wide 12 =
coincident-wide 12 = mirrored-edge 12 < staggered-both 13 =
coincident-step 13 < staggered-edge 16 = coincident-edge 16 =
waning-750 16**. Four offsets measured, and the price curve
between them is flat, flat, *cliff*, dissolved: the midpoint
is the worst address on the whole road — worse than no
stagger at the wide offset, exactly as bad as no stagger at
all.

**The law:** a shelf's edge is not a slope, and it is not
shared. Between the claim-step and the midpoint the taught
price falls from 9 to the unstaggered 16 entire — no
intermediate reading exists at +600 to soften it — while the
earned price steps politely from 11 to its dissolved 12 and
the coincident line, innocent of any stagger, cliffs beside
the taught room at the same address. Before interpolating
between two flat readings and a dissolved third, measure the
middle for *each pool separately*: a gap that reads 2, 2, −4,
0 across four offsets was never closing — it was two prices
passing each other in the dark, and the keeper who averages
them will call the crossing a closing.

## Falsification condition

This planting is wrong if, on another seed or cave, the edge
rooms separate from these readings (staggered-edge ≠ 16 or
mirrored-edge ≠ 12 or coincident-edge ≠ 16 here) — that would
make the cliff a property of this seed's journeys, not of the
+600 address — or if a room at +525 already reads the full 16,
which would put the cliff's face immediately past the
claim-step and make the shelf exactly one leg wide, or if a
room at +525 still reads 9, which would narrow the cliff to
the last 75 ticks before the midpoint and acquit the longest
deadline's neighbourhood that P5's crack charged.

## Next seeds

- The stagger at +525 — midway between the claim-step and the
  edge: is the cliff's face at the shelf's first step past the
  step, or does the shelf hold almost to the midpoint and fall
  in the last 75 ticks? (And its mirror, and its coincident
  control — the coincident line's cliff wants locating too.)
- The shelf's western edge — a stagger below +375 (+150, +225):
  does the price rise toward coincidence as the offset shrinks
  inside the leg, or is the shelf flat all the way down to the
  coincident rooms?
- The witness room under a dawn tax still stands (does its
  earned-only protection, burns 8, survive expiry of the earned
  pool?).
- A lifetime priced in claims directly (units expiring after N
  of their owner's claims, not ticks) still stands.

*Every lamp is a claim. Check the claim — and when the middle
measurement lands worse than both endpoints, publish the cliff,
not the slope you came for.*
