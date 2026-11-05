# Shoved Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/unshoved-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-04. Survival ledger below — reported as found.

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
- **`witness-moths`** changed one rule: a burn is a local event.
  Its ledger: solo burns 12, witness burns 8.
- **`chain-moths`** extended the chain one link: a *refusal* is
  also a local event (burns 13). **`placed-moths`** carried the
  refusal's direction (burns 13). **`evidence-moths`** let a
  completed check transmit its actual peak intervals; witnesses
  re-judged the CV themselves, and steady samples could acquit
  hearsay caution (943 events, 18 learners, 223 acquittals —
  burns 12).
- **`throttled-moths`** rationed evidence teaching to one packet
  per checker per night (burns held at 12; volume falsified as
  the disease).
- **`unshoved-moths`** (the immediate parent) removed the recoil
  from evidence teaching — caution without the 150-tick aversion —
  and burned **18**, the lineage's worst table, on its most events
  (1,605) and most acquittals (439): the recoil was load-bearing
  *when it travelled with a lesson*. Its named succession, taken
  up verbatim here: *the complementary separation — a room whose
  irregular packets teach the **shove without the caution**
  (aversion only, no checker recruited): if the recoil alone,
  with no lesson behind it, holds burns near 12, the two halves
  price separately at last and the 2×2 (caution ± shove) closes.*

## The Novel Trait

Eight twin rooms, same seed. The first seven rooms are the
parents' rules, unchanged, and tests pin their ledgers exactly
(solo fed 45 / burns 12; witness fed 43 / burns 8; chain fed 42 /
burns 13, chain events 495, chain-first 18; placed fed 39 /
burns 13, placed events 618, placed-first 20; evidence fed 43 /
burns 12, evidence events 943, evidence-first 18, acquittals 223;
throttled fed 44 / burns 12, throttled events 534,
throttled-first 18, acquittals 122, packets sent 55, suppressed
105; unshoved fed 39 / burns 18, unshoved events 1,605,
unshoved-first 18, acquittals 439). The eighth room changes
exactly one thing about evidence teaching, in the opposite
direction from its parent:

- **Shoved** — every completed check transmits its actual peak
  intervals, exactly as in the evidence room (no ration), and
  witnesses within 12 cells re-judge the CV on the same 0.35
  line — but an irregular verdict earns the witness **the
  150-tick aversion only: no caution, no recruitment**. A steady
  verdict earns nothing at all: there is no hearsay caution in
  this room for a steady packet to acquit, so the acquittal
  channel does not exist here. The checker's own first-hand
  refusal still earns its own aversion, unchanged — that recoil
  is earned at the checker's own post, not taught.

The keeper intervenes never; the moths write the outcome.

## Predictions (declared before the first run)

- **P1:** The channel works shoved: shoved events > 0, in both
  kinds (irregular > 0 and steady > 0) — a push can travel
  without a lesson behind it.
- **P2:** No recruitment: shoved-room checks stay at the solo
  room's scale (≤ 110 declared; solo 73, evidence 151, unshoved
  211), while shoved-first witnesses (a moth whose first taught
  shove arrives before any burn of its own) number ≥ 10.
- **P3:** Acquittals are exactly 0 — steady packets carry no
  un-teaching in this room, because nothing was taught.
- **P4 (the claim):** If the recoil alone was the medicine,
  shoved-room total burns fall below the evidence room's 12,
  toward the witness room's 8.
- **P5:** Feeding holds near the evidence room's level (≥ 40):
  a moth that is pushed without being taught still eats.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *the recoil alone, with no lesson behind it, is the
medicine that held the evidence lineage's burns down* — is
falsified if any hold:

- no shoved packet is ever witnessed (shoved events = 0);
- no moth's first shove ever comes from a shoved packet;
- shoved-room total burns are not below the evidence room's 12.

The burns comparison is reported, not asserted by test — a
keeper does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p shoved-moths -- --headless
cargo test -p shoved-moths
```

## The Survival Ledger

Eight twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Fed | Burns | Checks | Refusals | Teaching events | First by that teaching |
|--------|-----|-------|--------|----------|-----------------|------------------------|
| solo | 45 | 12 | 73 | 15 | 0 | 0 |
| witness | 43 | 8 | 84 | 19 | 13 witness events | 6 first-cautions |
| chain | 42 | 13 | 208 | 67 | 495 chain events | 18 |
| placed | 39 | 13 | 223 | 77 | 618 placed events | 20 |
| evidence | 43 | 12 | 151 | 50 | 943 evidence events | 18 |
| throttled | 44 | 12 | 169 | 55 | 534 throttled events | 18 |
| unshoved | 39 | 18 | 211 | 68 | 1605 unshoved events | 18 |
| shoved | 44 | 14 | 136 | 38 | 867 shoved events | 20 first-shoves |

Shoved room by night: night 1 fed 18, burns 4, checks 16,
refusals 2, shoved events 65 (irregular 19, steady 46),
shoved-first 13, acquittals 0, alive 24; night 2 fed 13, burns
5, checks 49, refusals 12, events 322 (81 / 241), shoved-first
6, acquittals 0, alive 24; night 3 fed 13, burns 5, checks 71,
refusals 24, events 480 (166 / 314), shoved-first 1,
acquittals 0, **deaths 1, alive 23** — the only death in any
room of this lineage: every other room, every night, dawns
24/24.

Prediction verdicts, as declared above: **P1 HELD** (867
events, in both kinds — 266 irregular, 601 steady; the push
travels at half the unshoved flood's volume, on the same
packets). **P2 CRACKED in its bound, held in its mechanism**
(shoved-first 20, past the declared 10 — the push arrived
first for twenty moths; but checks reached **136**, over the
declared 110: no packet recruited a checker — recruitment is
structurally impossible in this room — yet the checking
population still grew, recruited the slow way, by first-hand
burns (14) and witnessed burns, starting nearly empty at 16
checks on night 1 and climbing 16 → 49 → 71 as pain did the
teaching the packets refused to). **P3 HELD, exactly**
(acquittals 0 across all three nights — the only room where
the acquittal ledger is not merely small but absent; a steady
packet here is weather, not news). **P4 CRACKED — the claim
line** (burns **14**: not below the evidence room's 12, not
near the witness room's 8 — worse than the full lesson (12),
worse even than no teaching at all (solo, 12); better only
than caution-without-recoil (18)). **P5 HELD** (fed 44, the
throttled room's table — a pushed moth still eats; the shove
scatters approaches without starving them).

The honest reading, kept whole: the 2×2 is closed, and neither
half is the medicine. Caution with the shove: 12. Caution
without: 18. The shove without caution: 14 — *worse than
nothing*, because a taught aversion is transport without a
destination judgement: the witness flees down its own gradient,
sampled in the mixture of both lamps, for 150 ticks, and then
re-approaches exactly as ignorant as before, having spent
health-adjacent time being moved by a verdict it never heard.
The one death in the lineage belongs to this room: a moth can
be pushed out of the light a hundred times and still walk back
into it, because nothing it was given was *knowledge*. Set
beside the witness room's 8 — rare teaching (13 events), its
recoil paid at the lamp in the right direction — the square's
lesson is that the halves were never additive parts of one
medicine. The medicine was always the *pairing*: a judgement,
earned or witnessed, that moves the moth *because* it knows
something, away from a place it can now name as dangerous.
A push that carries no claim is just weather with intent.

Named succession seed: the half-life survey this lineage keeps
deferring — four tuition kinds, four decay clocks, in one room:
does caution earned from a burn decay at the same rate as
caution taught by evidence, and does a taught shove's protection
(if any) outlive the caution it refused to carry? Or: the
witness room revisited — its 8 stands alone at the top of every
table this lineage has posted; is it the rarity, the at-lamp
direction, or the pairing that prices it?

*Every lamp is a claim. Check the claim — a push that carries
no claim is just weather with intent.*
