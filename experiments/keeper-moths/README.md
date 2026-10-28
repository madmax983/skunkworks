# Keeper Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid)
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-10-28. Survival ledger below.

## Lineage

- **`resonance-audio`** provides the room and the medium: a 56×28
  stone-walled `PhysicsGrid` in which both lamps speak as `pluck()`s
  and every claim travels as a real wave — reflecting, interfering,
  decaying. It is the same cave the Night Listener's `vespertilio`
  hunts in; here the moths are not the prey. They are the audience,
  and the lamps are lying to some of them.

## The Lamps, and Their Honesty

- **The honest lamp** (west, (9, 14)): plucks every **16 ticks** at
  strength **0.9**, perfectly periodic. A moth that reaches it feeds
  (+25 health) and settles for the night. Its claim is true and its
  flicker is its signature: steady.
- **The decoy lamp** (east, (46, 14)): plucks at strength **2.2** —
  the brighter claim — but on alternating intervals of **6 and 26
  ticks** (the *same mean period*, 16, as the honest lamp, so no
  moth can tell them apart by tempo alone, only by regularity). A
  moth that reaches it burns (−35 health) and is knocked back into
  the dark.

Both lamps' mean periods agree. Their wattages disagree. Their
regularity disagrees. Only one of those three facts is safe to
follow, and it is not the brightness.

## The Discernment Mechanic

A moth perceives only what the field delivers at its own cell: a
smoothed, saturation-clipped brightness (the sensor clips at 1.0, so
the decoy's extra wattage buys pull at range but no extra detail up
close) and the local gradient of |pressure| across its neighbouring
cells. It is never told where a lamp is, or what kind it is.

- **Naive moths rush.** Caution = 0: no checking, straight up the
  gradient of the loudest claim.
- **Burns are tuition.** A burned moth's caution rises. Thereafter,
  when its local brightness passes 0.10, it *stops* — up to 96
  ticks, exposed, going nowhere (the cost of verification) — and
  times the intervals between brightness peaks at its own cell.
- **The verdict is arithmetic.** If the peak intervals' coefficient
  of variation exceeds 0.35, the flicker is irregular: the moth
  refuses, flees down-gradient, and will not re-approach for 150
  ticks. If the intervals agree, it commits and flies in.
- **Interference is the honest flaw.** Both lamps share one field;
  where their waves interleave, a decoy's intervals can momentarily
  *look* regular, and a checking moth can be fooled fair and square.
  The ledger keeps those burns. The Keeper does not tidy them away.

## Falsification Condition

The claim — *moths learn to discern the decoy by its flicker, at
the cost of stopped time, and the population burns less for it* —
is falsified if any of these hold:

- the decoy fools no one on night 1 (no burns while all moths are
  naive) — then the decoy was never attractive, only labelled;
- no moth ever checks, or no claim is ever refused — then burns
  teach nothing and caution is decoration;
- night 3 burns as much as night 1 — then the checking costs time
  and buys nothing.

All three are pinned by unit tests.

## Usage

```bash
# Headless (handled before anything else; prints the survival ledger)
cargo run -p keeper-moths -- --headless

# Tests
cargo test -p keeper-moths
```

## The Survival Ledger

Three nights, 24 moths, 1500 ticks a night; health, caution and
scars carry across nights, only positions reset:

| Night | Fed | Burns | Checks | Refusals | Steady verdicts | Alive at dawn |
|-------|-----|-------|--------|----------|-----------------|---------------|
| 1     | 14  | 7     | 19     | 3        | 16              | 24            |
| 2     | 14  | 1     | 24     | 4        | 20              | 24            |
| 3     | 17  | 4     | 30     | 8        | 20              | 24            |

They went to the bright one first. They always do. By the third
night the refusals had nearly tripled and the burns had nearly
halved — and the decoy still claimed four, the ones whose checks
the interference fooled. Nobody died. Burns are tuition, not rent.

*Every lamp is a claim. Check the claim.*
