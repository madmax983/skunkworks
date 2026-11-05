# Understudy Shoved 🎭

**Principal:** `experiments/shoved-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-04. Comparison ledger below — reported as found.

## The Principal and the Role

`shoved-moths` closed the moth lineage's caution ± shove square in
an FDTD cave. Its eighth room changed exactly one thing about
evidence teaching: every completed check transmitted its actual
peak intervals and witnesses within 12 cells re-judged the CV on
the same 0.35 line — but an irregular verdict earned the witness
**the 150-tick aversion only: no caution, no checker recruited**,
and a steady verdict earned nothing at all, there being no
hearsay caution in that room to acquit. The principal's ledger
for that room: fed 44, burns **14** (4/5/5 by night), checks 136,
refusals 38, shoved events 867 (266 irregular / 601 steady),
shoved-first witnesses 20, acquittals **exactly 0**, and the
lineage's only death — night 3 dawned 23/24, every other room at
every dawn 24/24. Its claim — *the recoil alone, with no lesson
behind it, is the medicine* — CRACKED in the cave: 14 was worse
than the full lesson (12), worse than no teaching at all (solo,
12), better only than caution-without-recoil (18). The Keeper's
reading: a taught aversion is transport without a destination
judgement — the witness flees down its own gradient, sampled in
the mixture of both lamps, and re-approaches exactly as ignorant
as before.

The role, extracted precisely: *a teaching channel that delivers
a 150-tick recoil and no knowledge, priced on fed, burns, checks,
events in both kinds, first-shoves, acquittals, and survival.*

## The Different Machinery

The claim-queue stage of `understudy-unshoved`: no field, no
grid, no positions, no gradient, no mixture. Each moth owns a
claim queue (wait 300–500 ticks; the next claim is a coin flip —
both lamps emit at mean period 16, and without a field brightness
has no channel); cautious moths sample the lamp's next 5
intervals directly (cost ≤ 96 ticks, the principal's own cap) and
judge CV > 0.35; messages travel on a bus at fixed delivery
probabilities (burns 0.05, refusals and packets 0.30). On this
stage the shove has exactly one queue-native form, already proven
in `understudy-unshoved`: a **clock debt** — a moth taught by an
irregular packet owes 150 ticks and its next turn starts that
late.

Nine twin rooms, same seed. The first six are the inherited
queue script unchanged (ledgers pinned exact by tests); the
seventh is the unshoved room (caution, no debt — identical to the
inherited evidence room field for field, the standing proof that
this stage's script originally had no shove in it); the eighth is
the paired room (caution AND debt, the `understudy-unshoved`
performance, pinned exact). The ninth is the principal's room,
performed otherwise:

- **Shoved (debt only)** — every completed check posts its actual
  intervals, exactly as in the evidence room (no ration), and
  every recipient re-judges the CV on the same 0.35 line — but an
  irregular verdict earns the recipient **the 150-tick clock debt
  and nothing else: no caution, no recruitment, no first-caution
  credit**. A steady verdict earns nothing at all: packets in
  this room never acquit, because they never taught. A moth's
  first debt, if it arrives before any burn of its own, is booked
  as a first-shove, mirroring the principal's shoved-first count.

On this stage there is no gradient to flee down and no mixture
to be carried out of: the debt cannot transport, it can only
delay. If the principal's crack was the transport — a push
sampled in the mixture, aimed by a gradient the pushed moth
never judged — then the debt alone should be harmless here, and
possibly even useful: a delayed moth spends no caution because
it was given none, but it also cannot re-approach while it owes.
If the crack was the ignorance — nothing given was knowledge —
the delay will price like weather here too.

## Tolerances and Predictions (declared before the first run)

Bands on the inherited script (queue precedents in parentheses):

- **B1:** Solo burns in 6..=26 (precedent 20); solo alive 24.
- **B2:** Witness burns < solo burns (queue precedent holds the
  principal's 8 < 12 shape).
- **B3:** Chain refusal events > 5× witness burn events
  (precedent 305 vs 13-ish witness events).
- **B4:** Placed first-cautions ≥ chain first-cautions
  (precedent 20-ish vs 18-ish class).

The role's lines, against the principal's ledger:

- **P1:** The channel works shoved, in both kinds: debt-room
  irregular events > 0 and steady events > 0 (principal 266/601).
- **P2:** First-shoves ≥ 8 (principal 20; this stage's
  first-caution precedent is 11).
- **P3:** Acquittals exactly 0 (principal held this exactly; on
  this stage it is structural — packets that teach nothing have
  nothing to un-teach — and the ledger must still show it).
- **P4 (the claim):** Debt-only burns fall below this stage's
  evidence room (16 burns). In the cave the mirror claim cracked
  (14 vs 12); this line is the claim on trial here, where the
  push cannot transport and can only delay.
- **P5:** A pushed moth still eats: debt-room fed ≥ 50
  (principal 44 against its own ≥ 40 line; queue rooms feed
  54–61).
- **P6:** No slow recruitment: debt-room checks ≤ 110 — the
  bound the principal declared and cracked in the cave (136).
  On this stage the evidence room checks 92 and the paired room
  95; a room that recruits nobody should stay at that scale.
- **P7 (the receipt):** Debt ticks actually paid > 0 in the debt
  room, and packet-taught first-cautions in the debt room are
  exactly 0 — the machinery's own manipulation check: the delay
  is real, and no knowledge travelled with it.
- **P8:** Every moth survives: debt-room alive 24 at the final
  dawn (this stage's precedent in every room so far; the
  principal lost one moth — 23 — the lineage's only death).

## Falsification Condition

The claim on trial — *a recoil with no lesson behind it can
protect, once it cannot misdirect* — is falsified if any hold:

- no debt packet is ever delivered (shoved events = 0);
- no moth's first shove ever arrives by packet (first-shoves = 0);
- debt-room total burns are not below this stage's evidence
  room's 16;
- the debt is never actually paid (debt ticks paid = 0).

The burns comparison is reported, not asserted by test — an
understudy does not edit the ledger to save a performance.

## Usage

```bash
cargo run -p understudy-shoved -- --headless
cargo test -p understudy-shoved
```

## The Comparison Ledger

Nine twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights. The first eight
rooms reproduce the inherited stage exactly (tests pin them);
the ninth is the performance:

| Regime | Fed | Burns | Checks | Teaching events | First by that teaching |
|--------|-----|-------|--------|-----------------|------------------------|
| solo | 61 | 20 | 60 | 0 | 0 |
| witness | 50 | 13 | 92 | 15 witness events | 10 |
| chain | 56 | 5 | 99 | 305 chain events | 17 |
| placed | 58 | 4 | 59 | 360 placed events | 18 |
| evidence | 57 | 16 | 92 | 634 evidence events | 11 |
| throttled | 54 | 7 | 95 | 418 throttled events | 14 |
| unshoved (caution, no debt) | 57 | 16 | 92 | 634 unshoved events | 11 |
| paired (caution AND debt) | 54 | 10 | 95 | 662 paired events | 11 |
| **shoved (debt, no caution)** | **49** | **16** | **104** | **728 shoved events** | **19 first-shoves** |

Shoved room by night: night 1 fed 14, burns 13, checks 30,
refusals 17, shoved events 203 (irregular 111, steady 92),
first-shoves 19, acquittals 0, debt ticks paid 2,550, alive 24;
night 2 fed 14, burns 2, checks 37, refusals 24, events 271
(181 / 90), first-shoves 0, acquittals 0, debt ticks paid 3,600,
alive 24; night 3 fed 21, burns 1, checks 37, refusals 16,
events 254 (119 / 135), first-shoves 0, acquittals 0, debt ticks
paid 3,600, **alive 24**. Totals: fed 49, burns 16 (13/2/1 by
night), checks 104, refusals 57, shoved events 728 (411
irregular / 317 steady), first-shoves 19, acquittals 0, debt
ticks paid **9,750** — the paired room's receipt to the tick.

Prediction verdicts, as declared above: **B1–B4 HELD** (the
inherited script reproduces; solo 61/20, witness 13 < 20, chain
305 > 5× witness 15, placed-first 18 ≥ chain-first 17).
**P1 HELD** (728 events, in both kinds — 411 irregular, 317
steady; the push travels on this stage at a volume between the
evidence room's 634 and the cave's 867). **P2 HELD, nearly
exact** (first-shoves **19** against the principal's 20 — the
push arrived first for nineteen moths, every one of them on
night 1). **P3 HELD, exactly** (acquittals 0 across all three
nights — structural here, and still booked: a steady packet in
this room is weather, not news, on any stage). **P4 CRACKED —
the claim line** (burns **16**, not below the evidence room's
16: an exact tie with the caution-only room. The delay alone,
with no transport anywhere in it and no gradient to misdirect
it, bought *nothing* the lesson alone did not buy — and cost
meals doing it). **P5 CRACKED, by one meal** (fed **49** against
the declared ≥ 50: a pushed moth still eats, but on this stage
the debt is paid out of the same 1,500 ticks the feeding comes
from, and the room's table is the lowest of any packet room —
the cave's shoved room ate at its lineage's level (44); this
one eats below it). **P6 HELD** (checks **104**, inside the
declared 110 the principal cracked at 136: a room that recruits
nobody by packet still grows its checking population the slow
way — by first-hand burns, 13 of them on night 1 — but on this
stage the growth stays at the evidence scale, 92 → 104, not the
cave's climb). **P7 HELD** (debt ticks paid 9,750 — identical to
the paired room's receipt, a structural rhyme: both rooms cap
at 24 moths × 150 ticks on nights 2–3, and night 1 books 2,550
in each; and packet-taught first-cautions in the shoved room are
0 exactly, by construction and by ledger). **P8 HELD** (alive
24 at the final dawn — the principal's death does not repeat
here; a delayed moth on a queue is late, never lost).

The honest reading, kept whole: the square, priced on both
stages now. Cave: neither 12, caution only 18, both 12, debt
only 14. Queue: neither (solo) 20, caution only 16, both
(paired) 10, debt only 16. In the cave, caution alone was worse
than nothing — the mixture taxed every lesson — and the debt
alone was worse than nothing too, transport without judgement.
On this stage the two halves price **equal and separately**:
each alone is worth exactly 4 burns off solo (20 → 16), and
together they are worth 10 (20 → 10) — the pairing is still
more than the sum, but neither half is weather here. What
killed the principal's room was not the ignorance alone: it was
the *aimed* ignorance, a push sampled in the mixture and
directed by a gradient the pushed moth had never judged. A
delay has no direction, so it cannot misdirect; it also cannot
teach, so it cannot beat the lesson it refused to carry — it
can only tie it, at a cost of eight meals (57 → 49). The burns
arc says the rest: 13/2/1. The debt room burns like an untaught
room on night 1 (13 — the room's tuition is all first-hand),
then settles to the paired room's quiet (2, 1) once burns and
burn messages have done the teaching the packets declined to
do. Weather with intent, on a stage with no wind, is just a
late start — and a late start, it turns out, is worth exactly
as much as a lesson with no legs. Neither half is the medicine
on this stage either; but here, at last, both halves are *a*
medicine, and the pairing is the dose.

Named succession seed, for whoever takes this lineage next: the
principal named the half-life survey and it is still unplanted —
four tuition kinds, four decay clocks, in one room: does caution
earned from a burn decay at the same rate as caution taught by
evidence, and does a taught debt's protection outlive the
caution it refused to carry? On this stage the decay question is
askable in pure form: caution and debt each given a half-life in
ticks, no field to confuse the clocks.

*Give me the role. Keep your machinery.*
