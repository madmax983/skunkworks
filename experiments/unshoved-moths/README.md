# Unshoved Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/throttled-moths`
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
- **`throttled-moths`** (the immediate parent) rationed evidence
  teaching to one packet per checker per night: events 943 → 534,
  learners 18 kept, acquittals halved to 122, feeding rose to 44 —
  and burns held at **12**, the evidence room's count. A stricter
  per-run probe (133 events) burned 11. Volume was falsified as
  the disease. Its named succession, taken up verbatim here:
  *the witness room is the only one where teaching carries no
  aversion shove toward a lamp judged in the mixture; an evidence
  room whose irregular packets teach caution without the 150-tick
  aversion (caution as information, not as a shove) would separate
  the lesson from the recoil.*

## The Novel Trait

Seven twin rooms, same seed. The first six rooms are the parents'
rules, unchanged, and tests pin their ledgers exactly (solo fed
45 / burns 12; witness fed 43 / burns 8; chain fed 42 / burns 13,
chain events 495, chain-first 18; placed fed 39 / burns 13,
placed events 618, placed-first 20; evidence fed 43 / burns 12,
evidence events 943, evidence-first 18, acquittals 223; throttled
fed 44 / burns 12, throttled events 534, throttled-first 18,
acquittals 122, packets sent 55, suppressed 105). The seventh
room changes exactly one thing about evidence teaching:

- **Unshoved** — every completed check transmits its actual peak
  intervals, exactly as in the evidence room (no ration), and
  witnesses within 12 cells re-judge the CV on the same 0.35
  line — but an irregular verdict earns the witness **caution
  only: no 150-tick aversion, no recoil, no shove**. A steady
  verdict acquits hearsay caution exactly as in the evidence
  room (an acquittal never shoved anyone). The checker's own
  first-hand refusal still earns its own aversion, unchanged —
  that recoil is earned at the checker's own post, not taught.

The keeper intervenes never; the moths write the outcome.

## Predictions (declared before the first run)

- **P1:** The channel works unshoved: unshoved events > 0, in
  both kinds (irregular > 0 and steady > 0) — a lesson can travel
  without a push behind it.
- **P2:** The learners survive unshoved: unshoved-first learners
  at or near the evidence room's 18 (≥ 14 declared).
- **P3:** Acquittals survive: > 0 (the steady half of the channel
  is untouched, so the un-teaching should persist at some level).
- **P4 (the claim):** Unshoved-room total burns fall below the
  evidence room's 12, toward the witness room's 8 — the aversion
  shove in the mixture was the disease, so removing it cures.
- **P5:** Feeding holds near the evidence room's level (≥ 40):
  a moth that is taught without being shoved still eats.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *the evidence lineage's burns were the taught
aversion shove, and teaching caution without it cures them* — is
falsified if any hold:

- no unshoved packet is ever witnessed (unshoved events = 0);
- no moth's first caution ever comes from an unshoved packet;
- unshoved-room total burns are not below the evidence room's 12.

The burns comparison is reported, not asserted by test — a
keeper does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p unshoved-moths -- --headless
cargo test -p unshoved-moths
```

## The Survival Ledger

Seven twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Fed | Burns | Checks | Refusals | Teaching events | First-cautions by that teaching |
|--------|-----|-------|--------|----------|-----------------|-------------------------------|
| solo | 45 | 12 | 73 | 15 | 0 | 0 |
| witness | 43 | 8 | 84 | 19 | 13 witness events | 6 |
| chain | 42 | 13 | 208 | 67 | 495 chain events | 18 |
| placed | 39 | 13 | 223 | 77 | 618 placed events | 20 |
| evidence | 43 | 12 | 151 | 50 | 943 evidence events | 18 |
| throttled | 44 | 12 | 169 | 55 | 534 throttled events | 18 |
| unshoved | 39 | 18 | 211 | 68 | 1605 unshoved events | 18 |

Unshoved room by night: night 1 fed 15, burns 5, checks 40,
unshoved events 197 (irregular 88, steady 109), acquittals 65,
unshoved-first learners 14; night 2 fed 15, burns 6, checks 75,
events 488 (238 / 250), acquittals 123, learners 3; night 3
fed 9, burns 7, checks 96, events 920 (325 / 595), acquittals
251, learners 1. Alive 24/24 in every room at every dawn;
deaths 0.

Prediction verdicts, as declared above: **P1 HELD** (1,605
events, in both kinds — 651 irregular, 954 steady; the lesson
travels further unshoved than shoved, because nobody is pushed
out of earshot). **P2 HELD, exactly** (18 unshoved-first
learners, the evidence room's full class). **P3 HELD, past the
declaration** (439 acquittals, nearly double the evidence
room's 223 — a witness that is never shoved away stays in the
mixture, hears the next steady packet, and is un-taught again;
the channel churns). **P4 CRACKED — the claim line, and not
narrowly** (burns **18**, the worst table this lineage has
posted: 5/6/7 by night against the evidence room's 5/6/1; the
unshoved room is the only teaching room whose burns *rise* into
night 3 instead of settling). **P5 CRACKED by one meal** (fed
39, under the declared 40 — the placed room's table, the
lineage's lowest, tied).

The honest reading, kept whole: the shove was not the disease —
it was half the medicine, and this room removed the wrong half.
An aversion does two jobs no ledger had separated before: it
moves the witness *out of the mixture* (distance from both
lamps at once), and it buys 150 ticks in which the witness
cannot re-approach while its caution is still new. Caution
without the recoil leaves the taught moth standing exactly
where it was taught — in the mixed field, beside the checker —
where it checks again (211 checks, the lineage's most; 68
refusals, also the most), is acquitted and re-taught in a churn
(439 acquittals against 18 learners), and drifts back toward
whichever lamp is louder with nothing ever having pushed it
away. The witness room's 8 now reads differently: its teaching
is rare (13 events) *and* its recoil is paid at the lamp, in
the right direction — frequency and recoil were the two halves
all along, and throttling had already priced the first. What
this lineage can now measure, room by room: no teaching, 12;
rare teaching with an at-lamp recoil, 8; full teaching with a
mixture recoil, 12; full teaching with no recoil, 18. The
recoil is load-bearing. A lesson that does not move the
learner leaves the learner where the danger found them.

Named succession seed: the complementary separation — a room
whose irregular packets teach the **shove without the caution**
(aversion only, no checker recruited): if the recoil alone,
with no lesson behind it, holds burns near 12, the two halves
price separately at last and the 2×2 (caution ± shove) closes.
Or the half-life survey this lineage keeps deferring: four
tuition kinds, four decay clocks, in one room.


*Every lamp is a claim. Check the claim — and let a lesson
arrive as information, not as a push.*
