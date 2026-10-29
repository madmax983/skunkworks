# Witness Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/keeper-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-10-29. Survival ledger below.

## Lineage

- **`resonance-audio`** provides the room and the medium: the same
  56×28 stone-walled `PhysicsGrid` as `keeper-moths`, both lamps
  speaking as `pluck()`s into one shared wave field.
- **`keeper-moths`** (the Keeper's first planting) provides the
  moths, the lamps, and the flicker check entire: a moth perceives
  only the smoothed, saturation-clipped brightness and local
  gradient at its own cell; a burned moth earns caution; a cautious
  moth stops — up to 96 ticks, exposed — times the peak intervals
  at its cell, and refuses a claim whose interval coefficient of
  variation exceeds 0.35.

## The Novel Trait

In `keeper-moths`, tuition was strictly private: caution only ever
came from your own burn. This planting changes exactly one rule
and asks what it buys.

- **Solo regime** — the keeper-moths rule, unchanged. Twin room,
  same seed.
- **Witness regime** — a burn is also a *local event*. A moth
  within **12 cells** of a burn at the decoy sees the flash and the
  knockback and earns, without paying health: caution +1, and a
  recoil aversion (150 ticks fleeing down-gradient — it saw
  *where* the burn happened, close by).

A witness learns **that** a claim can burn and **roughly where the
burning happened**. It does not learn which lamp is false, where
either lamp is, or anything at all beyond the radius — a burn on
the far side of the room is private. And witnessed caution is not
knowledge: the witness must still stop, pay the stopped time,
time the flicker at its own cell, and judge for itself. Interference
can still fool its check, exactly as in the parent.

## The Lamps, and Their Honesty

Unchanged from the parent: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *caution earned by witnessing a neighbour's burn, at
no health cost, reduces the population's total burns against the
solo rule* — is falsified if any of these hold:

- no burn is ever witnessed (events = 0) — then the radius was
  decoration and the regimes are the same room twice;
- no moth's first caution ever comes from witnessing — then the
  channel exists but nobody learns through it;
- witness-regime total burns are not below solo total burns —
  then second-hand tuition buys nothing.

All three are pinned by unit tests (6/6).

## Tuning Notes (kept honestly)

- **Caution alone (first cut):** witnesses checked far more
  (113 vs 73) and refused more (22 vs 15) — and burned *more*,
  13 vs 12. Caution spread; protection did not. The recoil
  aversion is what turned witnessing from gossip into tuition.
- **Radius 10:** burns 10 vs 12, but one moth died on night 3.
- **Radius 14:** burns back up to 12 and feeding collapsed
  (36 vs 45) — too many witnesses recoiling and checking, the
  room starved of approach altogether.
- **Radius 12 (shipped):** the ledger below.

## Usage

```bash
cargo run -p witness-moths -- --headless
cargo test -p witness-moths
```

## The Survival Ledger

Two twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Night | Fed | Burns | Checks | Refusals | Witness events | New witness learners | Alive |
|--------|-------|-----|-------|--------|----------|----------------|----------------------|-------|
| solo    | 1 | 14 | 7 | 19 | 3  | 0 | 0 | 24 |
| solo    | 2 | 14 | 1 | 24 | 4  | 0 | 0 | 24 |
| solo    | 3 | 17 | 4 | 30 | 8  | 0 | 0 | 24 |
| witness | 1 | 13 | 3 | 18 | 3  | 4 | 4 | 24 |
| witness | 2 | 15 | 1 | 33 | 6  | 1 | 0 | 24 |
| witness | 3 | 15 | 4 | 33 | 10 | 8 | 2 | 24 |

Totals: solo fed 45, burns **12**, refusals 15 — witness fed 43,
burns **8**, refusals 19, witness events 13, and **6 moths whose
first caution was earned by watching, not by burning**.

The crack, kept: on night 3 the witness room burned exactly as
much as the solo room (4 and 4) — witnessing front-loads its
protection into the first two nights, while the tuition is fresh
and the witnesses are still near the lamp they watched. Second-hand
caution is real, and it is not armour. It is a head start.

*Every lamp is a claim. Check the claim — even when the burn that
taught you to check was not your own.*
