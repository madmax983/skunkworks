# Placed Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/chain-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-01. Survival ledger below — the finding is a falsification, kept as found.

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
- **`witness-moths`** changed one rule: a burn is a local event —
  a moth within 12 cells of a burn earns caution and a recoil
  without paying health. Its ledger: solo burns 12, witness
  burns 8.
- **`chain-moths`** (the immediate parent) extended the chain one
  link: a *refusal* is also a local event. The chain taught
  superbly (18 of 24 moths first-cautious by hearsay, 495 chain
  events) and protected not at all (burns 13, above solo 12).
  Its diagnosis, and the seed this planting takes up verbatim:
  *a refusal that carries its place honestly — chain-witnesses
  recoil from the lamp the refuser was facing (gradient
  direction at refusal, not the refuser's position) — does
  place-true third-hand caution finally protect, or is depth
  itself the poison?*

## The Novel Trait

Four twin rooms, same seed. The first three rooms are the
parents' rules, unchanged, and tests pin their ledgers exactly
(solo fed 45 / burns 12; witness fed 43 / burns 8; chain fed 42 /
burns 13, chain events 495, chain-first learners 18). The fourth
room changes exactly one thing about the chain link:

- **Placed** — a witnessed refusal carries its **direction**. At
  the moment of refusal, the refuser's local brightness gradient
  is recorded; it points up-gradient, toward the claim it just
  judged. A witness within 12 cells earns the chain package
  (caution +1, recoil aversion 150 ticks) **plus one shove**:
  velocity += 0.6 × (unit vector *away* from the refused lamp,
  i.e. the negative of the refuser's gradient direction). If the
  refuser's gradient is ~zero, the shove falls back to *away
  from the refuser's position*.

The witness still learns no lamp identity, no map, no reason —
only a direction, honestly measured at the place the lesson was
learned. The keeper intervenes never; the moths write the outcome.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *third-hand caution protects once the refusal's
lesson carries its true direction* — is falsified if any hold:

- no placed refusal is ever witnessed (placed events = 0);
- no moth's first caution ever comes from a placed refusal;
- placed-regime total burns are not below the chain room's 13
  **and** not below the witness room's 8.

The first two held (618 events, 20 first-cautions). **The third
falsified the claim**: placed burns 13 — exactly the chain room's
count, at a slightly worse table (fed 39 against 42). Direction
was delivered honestly and changed *who* learned (20 placed-first
learners, the most of any room) without changing *what* happened
at the lamps. The burns comparison is reported, not asserted by
test — a keeper does not edit the ledger to save a claim.

## Tuning Notes (struck variant, kept honestly)

- **Sustained directed recoil** (the witness flees along the
  inherited direction for the full 150-tick aversion, overriding
  its own gradient): placed events 598, placed-first learners
  17 — and burns **27**, fed **28**, **3 deaths**, the worst room
  this lineage has ever kept. A direction held too long stops
  being a lesson and becomes an autopilot: at up to ~0.3 cells
  per tick it carries a moth clear across the room, into
  whichever lamp waits at the other end. The shipped rule is
  the impulse shove above; the sustained variant is struck, not
  forgotten.
- **Radius, seed, lamps, aversion length:** untouched from the
  parents throughout.

## Usage

```bash
cargo run -p placed-moths -- --headless
cargo test -p placed-moths
```

## The Survival Ledger

Four twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Night | Fed | Burns | Checks | Refusals | Placed events | New placed learners | Alive |
|--------|-------|-----|-------|--------|----------|----------|----------|-------|
| solo    | 1 | 14 | 7 | 19 | 3  | 0 | 0  | 24 |
| solo    | 2 | 14 | 1 | 24 | 4  | 0 | 0  | 24 |
| solo    | 3 | 17 | 4 | 30 | 8  | 0 | 0  | 24 |
| witness | 1 | 13 | 3 | 18 | 3  | 0 | 0  | 24 |
| witness | 2 | 15 | 1 | 33 | 6  | 0 | 0  | 24 |
| witness | 3 | 15 | 4 | 33 | 10 | 0 | 0  | 24 |
| chain   | 1 | 19 | 5 | 36 | 11 | 0 | 0  | 24 |
| chain   | 2 | 8  | 3 | 88 | 29 | 0 | 6  | 24 |
| chain   | 3 | 15 | 5 | 84 | 27 | 0 | 0  | 24 |
| placed  | 1 | 12 | 5 | 56 | 20 | 94  | 15 | 24 |
| placed  | 2 | 17 | 3 | 73 | 26 | 274 | 4  | 24 |
| placed  | 3 | 10 | 5 | 94 | 31 | 250 | 1  | 24 |

(chain-room new *chain* learners by night: 12 / 6 / 0, totalling
18; the placed columns count placed-room events only.)

Totals: solo fed 45, burns **12** — witness fed 43, burns **8** —
chain fed 42, burns **13**, refusals 67, chain events 495 —
placed fed 39, burns **13**, refusals **77**, checks **223**,
placed events **618**, and **20 of 24 moths whose first caution
was a placed refusal** — the widest teaching this lineage has
measured, bolted now to an honest direction, and the burns did
not move.

So the chain-moths diagnosis was only half the disease. A
refusal's gradient is not the lamp's address: in a reverberant
room the gradient at the refuser's cell points along the
*mixture* — the last wavefront to pass, honest or decoy or both
at once — so the "true place" a refusal can honestly carry is
already the cave's rumour, not the decoy's. And even a perfect
direction would arrive as a shove, not a judgment: the witness
still cannot tell a decoy refusal from an honest-lamp refusal
the interference provoked. Place was not the missing half of
the lesson. **Depth is the poison**: each hand the caution
passes through keeps the caution and loses the evidence, and no
annotation bolted onto the hearsay — not place, not direction —
restores evidence that was never transmitted.

*Every lamp is a claim. Check the claim — a lesson can carry
its place perfectly and still arrive empty.*
