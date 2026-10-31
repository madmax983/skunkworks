# Chain Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/witness-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-10-31. Survival ledger below — the finding is a falsification, kept as found.

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
- **`witness-moths`** (the immediate parent) changed one rule: a
  burn is a local event — a moth within 12 cells of a burn earns
  caution and a recoil aversion without paying health. Its ledger:
  solo burns 12, witness burns 8, 13 witness events, 6 moths whose
  first caution was witnessed. Its README left the seed this
  planting takes up, verbatim: *chain witnessing — can a witness's
  refusal teach a third moth that never saw a burn at all?*

## The Novel Trait

Three twin rooms, same seed, one rule added per room:

- **Solo** — the keeper-moths rule: caution only from your own burns.
- **Witness** — the witness-moths rule, unchanged (a test pins its
  ledger exactly: fed 43, burns 8).
- **Chain** — the witness rule, plus: a **refusal is also a local
  event**. When a moth stops, judges a flicker irregular, and
  flees, every moth within **12 cells** of the refusing moth earns
  the same package a burn-witness earns — caution +1 and a recoil
  aversion (150 ticks) — without paying health, and whether or
  not it has ever seen a burn.

A chain learner learns **that a neighbour judged a claim false,
somewhere near here**. It does not learn which lamp was refused,
where either lamp is, or why the neighbour fled. Crucially, unlike
a burn, a refusal carries no true location: the burn happens *at*
the decoy, so a burn-witness's recoil points away from real
danger; the refusal happens wherever the checker happened to
stand, so a chain-witness's recoil points away from a *moth*.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *caution transmitted through a witnessed refusal
reduces the population's total burns below the witness room's* —
is falsified if any of these hold:

- no refusal is ever witnessed (chain events = 0);
- no moth's first caution ever comes from a witnessed refusal;
- chain-regime total burns are not below witness total burns.

The first two held; **the third falsified the claim** (see the
ledger). All three are pinned by unit tests where they can be
(events > 0, learners > 0); the burns comparison is reported,
not asserted — a keeper does not edit the ledger to save a claim.

## Tuning Notes (struck variants, kept honestly)

- **Chain, caution only (no recoil):** chain events 689, chain-first
  learners 18, refusals 68 — and burns still 13, feeding down to
  38. Removing the mis-pointed recoil did not save the chain; the
  cascade of checks grew instead.
- **Chain, depth-limited + no recoil** (only refusals by
  burn-grounded moths teach; chain-only refusals are hearsay and
  do not propagate): chain events 350, learners 18, burns **16**
  — the worst room of the three variants. Gossip rationed is
  still gossip.
- **Radius, seed, lamps:** untouched from the parents throughout.
  The shipped room is the direct, unrestricted chain — the
  question as the parent's README asked it.

## Usage

```bash
cargo run -p chain-moths -- --headless
cargo test -p chain-moths
```

## The Survival Ledger

Three twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Night | Fed | Burns | Checks | Refusals | Witness events | Chain events | New chain learners | Alive |
|--------|-------|-----|-------|--------|----------|----------|----------|----------|-------|
| solo    | 1 | 14 | 7 | 19 | 3  | 0 | 0   | 0  | 24 |
| solo    | 2 | 14 | 1 | 24 | 4  | 0 | 0   | 0  | 24 |
| solo    | 3 | 17 | 4 | 30 | 8  | 0 | 0   | 0  | 24 |
| witness | 1 | 13 | 3 | 18 | 3  | 4 | 0   | 0  | 24 |
| witness | 2 | 15 | 1 | 33 | 6  | 1 | 0   | 0  | 24 |
| witness | 3 | 15 | 4 | 33 | 10 | 8 | 0   | 0  | 24 |
| chain   | 1 | 19 | 5 | 36 | 11 | 6 | 58  | 12 | 24 |
| chain   | 2 | 8  | 3 | 88 | 29 | 5 | 188 | 6  | 24 |
| chain   | 3 | 15 | 5 | 84 | 27 | 11| 249 | 0  | 24 |

Totals: solo fed 45, burns **12**, refusals 15 — witness fed 43,
burns **8**, refusals 19, witness events 13 — chain fed 42,
burns **13**, refusals **67**, chain events **495**, and **18 of
24 moths whose first caution was a witnessed refusal** — a lesson
at third hand, from a moth that may itself have learned at second
hand, about a lamp neither of them could name.

The chain *teaches* — that half is not in doubt: 18 first-cautions
is three times the witness room's 6, and by night 2 every moth in
the chain room is cautious. The chain does not *protect*: burns
13, above even the solo room's 12. The mechanism is in the
parents' own tuning notes — in `witness-moths`, caution alone
(without the recoil) also burned more, 13 vs 12; what saved the
witness room was that a burn's lesson arrives bolted to a true
place. A refusal's lesson arrives bolted to a bystander's place.
Spread without location, caution only multiplies checking — 208
checks against the witness room's 84 — and every extra check near
the decoy is another flicker judgment the cave's interference can
fool. Second-hand caution is a head start. Third-hand caution is
a rumour, and the moths paid tuition on it anyway.

*Every lamp is a claim. Check the claim — and ask who told the
moth standing next to you, and what they actually saw.*
