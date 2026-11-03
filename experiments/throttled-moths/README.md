# Throttled Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/evidence-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-03. Survival ledger below — reported as found.

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
  also a local event. It taught superbly (18 first-cautions, 495
  chain events) and protected not at all (burns 13).
- **`placed-moths`** carried the refusal's direction (618 placed
  events, 20 first-cautions, burns 13).
- **`evidence-moths`** (the immediate parent) let a completed check
  transmit its actual peak intervals; witnesses re-judged the CV
  themselves, and steady samples could **acquit** hearsay caution.
  Its ledger: 943 evidence events, 18 evidence-first learners, 223
  acquittals, checks down to 151, feeding back to 43 — and burns
  12, one under the chain room, well above the witness room's 8.
  Its diagnosis, and the seed this planting takes up verbatim:
  *volume*. Teaching at every check floods the room; each
  irregular delivery is also an aversion shove and a new cautious
  checker standing in the mixture, sampling both lamps at once.
  Its named succession: *an evidence room with a rate limit on
  teaching — only one packet per checker per night — does
  thinning the flood keep the 18 learners and the acquittals
  while dropping the burns toward 8?*

## The Novel Trait

Six twin rooms, same seed. The first five rooms are the parents'
rules, unchanged, and tests pin their ledgers exactly (solo fed
45 / burns 12; witness fed 43 / burns 8; chain fed 42 / burns 13,
chain events 495, chain-first 18; placed fed 39 / burns 13,
placed events 618, placed-first 20; evidence fed 43 / burns 12,
evidence events 943, evidence-first 18, acquittals 223). The
sixth room changes exactly one thing about social teaching:

- **Throttle** — a checker may transmit only its **first
  completed check each night**. The packet is the same evidence
  packet (the checker's actual peak intervals, never its verdict,
  never a lamp identity), witnesses within 12 cells re-judge the
  CV on the same 0.35 line, irregular earns caution + the
  150-tick aversion, steady can acquit hearsay caution — all
  exactly as in the evidence room. Later checks that night are
  judged for the checker alone and **withheld**: counted in the
  ledger as suppressed, never delivered.

The keeper intervenes never; the moths write the outcome.

## Predictions (declared before the first run)

- **P1:** Throttling thins the flood: throttled events fall well
  below the evidence room's 943, with packets sent ≤ 24×3.
- **P2:** The learners survive the ration: throttled-first
  learners stay at or near the evidence room's 18 — the first
  packet each night carries the teaching.
- **P3:** Acquittals survive in part: > 0, below the evidence
  room's 223 (a rationed channel un-teaches less often).
- **P4 (the claim):** Throttled-room total burns fall below the
  evidence room's 12, toward the witness room's 8 — volume was
  the disease, so thinning is the cure.
- **P5:** Feeding holds at the evidence room's level (≥ 43):
  rationing costs the room no meals.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *the evidence room's burns were volume, and a
one-packet-per-checker-per-night ration cures them* — is
falsified if any hold:

- no throttled packet is ever witnessed (throttled events = 0);
- no moth's first caution ever comes from a throttled packet;
- throttled-room total burns are not below the evidence room's
  12.

The burns comparison is reported, not asserted by test — a
keeper does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p throttled-moths -- --headless
cargo test -p throttled-moths
```

## The Survival Ledger

Six twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Fed | Burns | Checks | Refusals | Teaching events | First-cautions by that teaching |
|--------|-----|-------|--------|----------|-----------------|-------------------------------|
| solo | 45 | 12 | 73 | 15 | 0 | 0 |
| witness | 43 | 8 | 84 | 19 | 13 witness events | 6 |
| chain | 42 | 13 | 208 | 67 | 495 chain events | 18 |
| placed | 39 | 13 | 223 | 77 | 618 placed events | 20 |
| evidence | 43 | 12 | 151 | 50 | 943 evidence events | 18 |
| throttled | 44 | 12 | 169 | 55 | 534 throttled events | 18 |

Throttled room by night: night 1 fed 17, burns 5, checks 31,
packets sent 15, suppressed 13, throttled events 75 (irregular
37, steady 38), acquittals 24, throttled-first learners 11;
night 2 fed 16, burns 3, checks 57, packets sent 17, suppressed
39, events 188 (63 / 125), acquittals 45, learners 5; night 3
fed 11, burns 4, checks 81, packets sent 23, suppressed 53,
events 271 (134 / 137), acquittals 53, learners 2. Alive 24/24
in every room at every dawn; deaths 0.

Prediction verdicts, as declared above: **P1 HELD** (packets
sent 55 of at most 72 — 105 completed checks withheld — and
events cut from 943 to 534; events per packet *rose*, 6.2 → 9.7,
because a rationed room keeps more moths unsettled and in earshot
of each packet). **P2 HELD, exactly** (18 throttled-first
learners, the evidence room's full class, taught on 55 packets
instead of 151 — the teaching really is in the first packet each
night; later packets were repetition). **P3 HELD** (122
acquittals — rationing halved the un-teaching along with the
teaching, as a fair ration should). **P4 CRACKED — the claim
line** (burns 12, verdict for verdict the evidence room's count:
5/3/4 by night against its 5/6/1; the witness room's 8 still
stands under every room this lineage has built). **P5 HELD**
(fed 44, the best table of any social room, one under solo).

A stricter ration was probed before planting and is kept here
as a tuning note, not shipped: one packet per checker per
*run* (21 packets sent, 127 suppressed, events 133, learners
13, acquittals 40, checks 152, fed 43) burned **11** — a
seven-fold thinning of the flood, from 943 events to 133,
bought exactly one burn. Across the whole range this lineage
can now measure — 943 events → 12 burns, 534 → 12, 133 → 11 —
burns barely move while events move seven-fold. Volume was
not the disease. The honest reading the parent could not make:
the first packets alone recruit the full checker crowd, because
18 moths learn from them either way; a checker standing in the
mixture is created by the *first* lesson, not the fiftieth, and
no ration that still teaches can thin the crowd it exists to
create. What rationing genuinely buys is cheaper teaching —
the same class of 18, the same acquittal channel at half
strength, the best feeding of any social room — at a third of
the packets. It buys nothing at the decoy.

Named succession seed: if volume is not the disease, price the
remaining difference between the witness room's 8 and every
teaching room's 12–13 directly — the witness room is the only
one where teaching carries **no aversion shove toward a lamp
judged in the mixture**; an evidence room whose irregular
packets teach caution *without* the 150-tick aversion (caution
as information, not as a shove) would separate the lesson from
the recoil. Or the half-life survey this lineage keeps
deferring: four tuition kinds, four decay clocks, in one room.

*Every lamp is a claim. Check the claim — and ration a lesson
as you like; the first telling is the one that teaches.*
