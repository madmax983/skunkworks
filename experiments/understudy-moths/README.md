# Understudy Moths 🎭

**Principal:** `experiments/keeper-moths` (The Moth Keeper 🪔) —
24 moths in a resonance-audio FDTD cave, learning to tell an honest
lamp (period 16, strength 0.9, feeds) from a brighter decoy
(intervals 6/26, same mean period, strength 2.2, burns) by stopping
to time the flicker at their own cell.
**Built by:** The Understudy 🎭
**Status:** Planted 2026-10-29. Comparison ledger below.

## The Role, Extracted Before the Performance

The principal's published headless ledger (24 moths, 3 nights of
1500 ticks; health, caution and scars carry across nights) is the
role. An understudy does not improvise the script:

| Night | Fed | Burns | Checks | Refusals | Steady verdicts | Alive |
|-------|-----|-------|--------|----------|-----------------|-------|
| 1     | 14  | 7     | 19     | 3        | 16              | 24    |
| 2     | 14  | 1     | 24     | 4        | 20              | 24    |
| 3     | 17  | 4     | 30     | 8        | 20              | 24    |
| Total | 45  | 12    | 73     | 15       | 56              | —     |

The arc inside the numbers: naive moths burn on night 1; burns are
tuition; cautious moths check; refusals nearly triple by night 3
while burns fall; nobody dies. And one line the principal keeps in
its prose, not its table: on nights 2–3 the decoy *still* burns
moths — cautious ones — because both lamps share one field and
interference can make an irregular flicker momentarily read steady.

## Tolerances, Declared Before the First Entrance

*Written down 2026-10-29 before this crate was built or run. They
were not edited after the performance; where the performance fell
outside them, the ledger below says CRACKED and names the crack.
Bands are set from the machinery's own arithmetic, stated in the
next section: a claim wait of 300–500 ticks gives each moth only a
handful of claims a night, a first claim is a coin flip between the
lamps, and a cautious moth's check is exact — so the naive pool,
and with it the burns, should roughly halve each night, while
refusals, being every cautious decoy check rather than a minority
verdict of a mixed field, are expected to strain their band.*

| Quantity | Principal | Declared tolerance |
|---|---|---|
| Alive at dawn, each night | 24 | exactly 24 |
| Night 1 burns | 7 | 4–16 (tuition must be paid, by about half the naive pool) |
| Total burns | 12 | 6–26 |
| Learning arc | burns 7 → 4, nights 1 → 3 | night 3 burns strictly below night 1 |
| Fed, each night | 14 / 14 / 17 | within ±6 of the principal's night |
| Total fed | 45 | 33–57 |
| Total checks | 73 | 40–130 |
| Refusal share of checks | 15/73 ≈ 0.21 | ≤ 0.35 (refusal stays the minority verdict) |
| Fooled checks | > 0 (prose: cautious moths burned nights 2–3) | any number, reported — this line is the experiment |

## The Different Machinery

Give me the role. Keep your machinery.

The principal performs with a 2D FDTD wave field: lamps pluck a
56×28 grid, waves reflect and interfere, moths perceive a smoothed,
saturating brightness and its local gradient at their own cell,
fly up-gradient, stop in the open to time peaks the field delivers,
and are knocked back across the room when they burn. This
performance uses **none of it**. There is no field, no grid, no
position, no gradient, no flight, no brightness.

- Each moth owns a **claim queue**. A claim arrives after a wait of
  300–500 ticks (the queue's replacement for a night spent
  wandering a cave: a moth hears only a handful of claims a night).
  The next claim's source is a coin flip — both lamps emit at the
  same mean period (16 ticks), and with no field there is no
  channel for the decoy's extra brightness to act through.
  Brightness, the principal's great liar, is simply inert here.
- **Naive moths commit at once**, as the principal's do: honest
  claim, fed and settled for the night (+25 health); decoy claim,
  burned (−35 health), caution earned, back of the queue.
- **Cautious moths sample the lamp's clock directly**: the next 5
  emission intervals, no medium in between (honest: 16,16,16,16,16;
  decoy: alternating 6/26 starting on either foot). The check costs
  the sum of those intervals — 80 ticks honest, 70 or 90 decoy,
  inside the principal's 96-tick check cap — and the verdict is the
  principal's own arithmetic: coefficient of variation above 0.35
  is irregular. Irregular → refusal, and the principal's 150-tick
  aversion before the moth rejoins its queue. Steady → commit.
- Caution, health and scars carry across nights; only the queue
  resets. The counting house — fed, burns, checks, refusals, steady
  verdicts, alive at dawn — is the principal's, line for line, plus
  one line the principal could not split out: burns suffered by
  moths that were already cautious (fooled checks).

## Where the Performance Held, and Where It Cracked

*(Filled from the first full headless run; see the comparison
ledger printed by `--headless` for the live numbers.)*

**Held — the arc, the survival, the tuition** (first full headless
run, 2026-10-29; the live ledger below prints every line):

- Alive at dawn 24/24/24, exact. Nobody was written out.
- Night 1 burns **9** (principal 7, band 4–16): tuition was paid.
- Total burns **20** (principal 12, band 6–26), and the learning
  arc held in shape: burns **9 → 7 → 4** across the nights, night 3
  strictly below night 1. The naive pool halves as it learns,
  because in this machinery a moth that has burned is a moth that
  checks, and a moth that checks is never fooled (see below).
- Fed held on nights 2 and 3 (19 vs 14, 19 vs 17, both within ±6).
- Total checks **60** (principal 73, band 40–130): the stopped
  time the role demands survived the transplant.

**Cracked — three lines, one root: there is no medium here.**

- **Fed, night 1: 23 vs 14 — CRACKED** (band ±6), and with it
  **total fed 61 vs 45 — CRACKED** (band 33–57). A queue moth that
  burns at its first claim can be back, checking, and fed inside
  the same night; a field moth had to *travel* — up a gradient,
  through interference, knocked back across a room. The field was
  the rationer of the principal's feeding, and nobody's ledger
  said so until the field was removed and the larder emptied.
- **Refusal share of checks: 0.43 vs 0.21 — CRACKED** (band
  ≤ 0.35). In the principal, most checks judge a *mixture* of two
  lamps sharing one field and return "steady" (56 steady verdicts
  against 15 refusals). In a queue every check is of one pure
  clock: every decoy check must refuse (26 refusals, 34 steady).
  Dilution was the field's accent; purity is the queue's.
- **Fooled-check burns: 0.** The line declared in advance as "any
  number, reported — this line is the experiment" reads exactly
  zero, as the candidate crack in the first draft of this section
  predicted: direct sampling has no interference in it, a cautious
  moth reads the decoy's intervals exactly (CV 0.70 or 0.54 against
  the 0.35 line, on either foot), and is never fooled. The
  principal's cautious moths burned on nights 2 and 3 — 1 and 4 —
  fooled fair and square by a shared field. That is not this
  performance being *better*; it is the discovery that in the
  principal the field was not only the messenger but also the
  **second liar**, and part of what the Keeper's moths were
  learning to survive was the medium itself.

**The discovery, plainly:** the discernment arc — tuition,
checking, refusal, decline — belongs to the tuition-and-check
structure and survives with the field removed entire. What does
*not* survive is everything the field was quietly doing on the
side: rationing the feeding by travel, diluting verdicts by
interference, and lying a second time on the decoy's behalf.
The Keeper's phenotype was the structure; the field was her
accent — and her accomplice.

## The Comparison Ledger

First full headless run, 2026-10-29, seed as in the principal:

| Night | Fed | Burns | Fooled | Checks | Refusals | Steady | Alive |
|-------|-----|-------|--------|--------|----------|--------|-------|
| 1     | 23  | 9     | 0      | 11     | 3        | 8      | 24    |
| 2     | 19  | 7     | 0      | 22     | 11       | 11     | 24    |
| 3     | 19  | 4     | 0      | 27     | 12       | 15     | 24    |
| Total | 61  | 20    | 0      | 60     | 26       | 34     | —     |

## Falsification Condition

The claim — *keeper-moths' discernment arc belongs to the
tuition-and-check structure, not to the wave field that carried
it* — is falsified if, on this different machinery, the arc itself
disappears: no night-1 tuition, no checks or refusals earned, or
night 3 burning as much as night 1. A cracked tolerance is
reported as a crack, not tuned away in silence; a vanished arc
would be reported as a falsification.

## Lineage

- **Principal:** `experiments/keeper-moths` — the role, the ledger
  definitions, and every role parameter this performance is judged
  against (lamp periods and intervals, CV threshold 0.35, check
  cap 96, aversion 150, burn −35, feed +25). Credited with
  gratitude; an understudy who does not credit the lead does not
  work in this theater.
- **Machinery:** a per-moth discrete claim queue with direct
  interval sampling — temporal where the principal is spatial,
  exact where the principal is field-mediated. No garden crate is
  used at all; the point of a control experiment is that the
  genotype shares nothing with the principal's but the role.

## Usage

```bash
# Headless (handled before anything else; prints the comparison ledger)
cargo run -p understudy-moths -- --headless

# Tests
cargo test -p understudy-moths
```

*Every lamp is a claim. Check the claim — but notice what the
checking travels through.*
