# Understudy Staggered 🎭

**Principal:** `experiments/staggered-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-08. Comparison ledger below — reported as found.

## The Principal and the Role

`staggered-moths` took up `pooled-moths`' closing seed
verbatim — *both pools at 750, the taught pool's clock offset
by 375 ticks: if the coincidence is the cause, staggering
should buy back most of the 16 → ~12* — and answered it in the
FDTD cave, caution booked by source on a dawn-crossing global
clock. Both pools expire at lifetime 750 in both of its new
rooms; one or both pools carry a +375-tick offset on their
expiry deadline. Its ledger: **staggered** (taught deadline
+375) fed 50, burns **9**, checks 116, events 611 (224
irregular / 387 steady), learners 18, acquittals 145, expired
**55 taught / 15 earned**; **staggered-both** (the coincident
control — both deadlines +375, so only duration changes) fed
45, burns **13**, checks 128, events 836 (266 / 570), learners
18, acquittals 183, expired **58 / 19** — against waning-750's
fed 48, burns **16**, expired 58 / 27. P1, P2, P5, P6 **held**
(P2 exact, asserted by test). P3 **held past its line**: 9,
not merely ≤ 13 — below pooled-taught's 10, the best table of
any waning-family room in the cave. P4 **cracked, and the
crack was priced**: the control burns 13, not ≥ 15 — duration
alone buys back 3 of the 7 burns (16 → 13), and staggering at
the same longest deadline buys back 4 more (13 → 9). The
ordering is the law the principal leaves on the table:
**staggered 9 < coincident-late 13 < coincident-750 16** —
what strands a moth is two deadlines arriving on the same
journey, but the stagger must be priced against a coincident
control at the same deadline, or the extra ticks take the
credit in silence.

The role, extracted precisely: *a both-pools waning room at
one lifetime, re-performed with one pool's deadline offset by
half the lifetime, priced against a coincident control at the
same longest deadline — on fed, burns, checks, events in both
kinds, first learners, acquittals, and the two expiry counts —
and on whether the principal's three-term ordering survives
the swap of machinery.*

## The Different Machinery

The claim-queue stage of `understudy-pooled`: no field, no
grid, no positions, no gradient, no flight. Each moth owns a
claim queue (wait 300–500 ticks; the next claim is a coin
flip — both lamps emit at mean period 16, and without a field
brightness has no channel); cautious moths sample the lamp's
next 5 intervals directly (cost ≤ 96 ticks) and judge
CV > 0.35; messages travel on a bus at fixed delivery
probabilities (burns 0.05, refusals and packets 0.30). Caution
is booked by source in kind: caution from a moth's own burns
and from delivered burn messages is **earned**; caution from
re-judging an evidence packet is **taught** (the parallel
counter `taught_caution`; an acquittal spends taught first,
popping that unit's gain tick with it — if no taught unit
remains, the oldest earned unit is spent, exactly as the
principal books it).

Twenty twin rooms, same seed. The first eighteen are the
inherited queue script unchanged (ledgers pinned exact by
tests). The two new rooms run this stage's waning-750 room
**verbatim**, differing only in the offsets laid on the two
deadlines:

- **Staggered** — taught units expire at gain + 750 + 375
  owner-ticks; earned units at gain + 750. On paper the two
  forgettings land 375 apart.
- **Staggered-both** — both pools expire at gain + 1125: the
  deadlines stay coincident with each other; only the
  duration grows. The principal's control, kept honest.

The stage translation, named in the open: the principal's
clock is the cave's shared tick, struck continuously. A queue
is sequential, so each unit here carries its gain tick on its
**owner's personal clock** — the ticks that moth's own turns
have consumed (at most one night per night), running across
dawns and never reset — and expiry is discovered only when
its owner next moves: at the top of its turn's claims, and at
each dawn on its personal clock. There is no dawn tax in
these rooms. No RNG is drawn by expiry.

Two stage facts shape the whole performance, and both were
known before the curtain. First, this stage has already
**inverted** the principal's family bend: in the cave,
waning-750 was the dangerous middle (16); on this stage it is
the **safest** waning room — fed 55, burns **12**, checks 97,
events 671, learners 11, acquittals 161, expired 62 / 16, the
minimum of 15 / 12 / 26 / 26 across 1500 / 750 / 375 / 150 —
because 750 owner-ticks covers the claim-cycle journey this
stage actually asks. Second, this stage's time moves in
**claim-sized steps of roughly 450 owner-ticks**: a unit's
age is observed only at claim tops, and by a turn's second
claim every pre-turn unit is older than both 375 and 150,
which is why waning-375 and waning-150 are *the same room*
here, field for field. The principal's stagger is **375
ticks — below one claim-step**. So the ordering arrives on
this stage with its middle term already suspect: the two
deadlines may differ on paper while every claim top discovers
both pools' expiries at the same visit, and the performance
may collapse — staggered, coincident-late, and coincident-750
all reading one table — not because the coincidence law is
false, but because *this stage cannot see a 375-tick gap*.
The ledger must say which: a crack with an address (the step)
is worth more than a hold with none.

## Tolerances and Predictions (declared before the first run)

Bands on the inherited script (queue precedents in
parentheses):

- **B1:** Solo burns in 6..=26 (precedent 20); solo alive 24.
- **B2:** Witness burns < solo burns.
- **B3:** Chain refusal events > 5× witness burn events
  (precedent 305 vs 13-ish).
- **B4:** Placed first-cautions ≥ chain first-cautions.

The role's lines, against the principal's ledger:

- **P1 (staggered):** The channel works staggered, in both
  kinds, in each new room: irregular > 0, steady > 0,
  staggered-first learners > 0. (Principal 224/387 and
  266/570; learners 18/18.)
- **P2 (staggered):** The offset is real: both new rooms
  expire **both** pools (> 0 taught and > 0 earned) on the
  age clock, no dawn tax fires in either, and the offset
  values themselves are asserted by test (staggered:
  taught 375 / earned 0; staggered-both: 375 / 375).
  (Principal **HELD** exactly, asserted: 55/15 and 58/19.)
- **P3 (staggered, the coincidence claim, performed):**
  Staggered burns at or below this stage's waning-750 (12) —
  the stagger buys back or holds the safest room's table.
  (Principal: staggered 9, seven below its waning-750's 16.)
- **P4 (staggered, the control discriminates):**
  Staggered-both burns ≥ staggered burns — coincident
  deadlines at the same longest deadline never beat staggered
  ones. (Principal: control 13 above staggered 9. Equality is
  this stage's collapse signature, kept as a pass only if P3
  also holds; the README must name it a collapse, not a win.)
- **P5 (staggered):** Neither new room starves the channel:
  waning events ≥ 300 in each. (Principal **HELD**: 611 /
  836; this stage's waning-750 posted 671.)
- **P6 (staggered):** A staggered moth still eats: fed ≥ 50
  in each new room. (Principal 50 / 45, declared against its
  evidence 43; queue evidence precedent fed 57.)

## Falsification Condition

The principal's claim — *what strands a moth is two deadlines
arriving on the same journey; stagger the deadlines and the
danger buys back, priced against a coincident control at the
same deadline* — is falsified on this stage room by room: a
staggered room in which no packet is ever witnessed, or in
which either pool never expires, tests nothing; if staggered
burns **above** this stage's waning-750 (12), the stagger is
a harm here and the law was a fact about continuous clocks;
if staggered-both burns **below** staggered, coincident
deadlines at the same longest deadline beat staggered ones
and the coincidence is acquitted at the exact comparison
built to indict it; if all three rooms land on one table, the
stagger is below this stage's resolution and the law survives
only untested — the ledger must say which, and must not call
a collapse a confirmation.

The burns comparisons are reported, not asserted by test —
an understudy does not edit the ledger to save a claim.
(Instrument facts *are* asserted: the offsets are as
declared, expiry fires in both pools in each new room, and
no dawn tax fires there.)

## Usage

```bash
cargo run -p understudy-staggered -- --headless
cargo test -p understudy-staggered
```

## The Comparison Ledger

Twenty twin rooms, same seed, 24 moths, 3 nights × 1500
ticks. The eighteen inherited rooms reproduced the queue
precedent exact (solo fed 61 / burns 20; witness 50 / 13;
chain 56 / 5, events 305; placed 58 / 4, events 360, averted
50; evidence 57 / 16, checks 92, events 634, acquittals 156,
learners 11; throttled 54 / 7, events 418, acquittals 109,
packets 63/32; unshoved identical to evidence field for
field; paired 54 / 10, events 662, acquittals 184, debt paid
9,750; shoved 49 / 16, events 728, acquittals 0, first-shoves
19, debt paid 9,750; fading-taught 55 / 13, expired 99/0;
fading-earned 59 / 15, expired 0/19; fading-both 54 / 14,
events 596, expired 158/18; waning-1500 59 / 15, events 631,
expired 11/7; waning-750 55 / **12**, events 671, expired
62/16; waning-375 and waning-150 the same room, 58 / 26,
events 468, expired 103/31; pooled-taught 55 / 12, events
671, expired 62/0; pooled-earned 59 / 15, events 631, expired
0/16). The two new rooms:

| Regime | Fed | Burns | Checks | Events | Learners | Acquittals | Expired taught | Expired earned |
|--------|-----|-------|--------|--------|----------|------------|----------------|----------------|
| waning-750 (this stage) | 55 | **12** | 97 | 671 (349 irr / 322 steady) | 11 | 161 | **62** | **16** |
| staggered | 54 | **11** | 100 | 695 (361 / 334) | 11 | 162 | **48** | **15** |
| staggered-both | 54 | **11** | 100 | 695 (361 / 334) | 11 | 162 | **48** | **10** |

No deaths in either new room. Bands **B1–B4 HELD**.
**P1 HELD** in both rooms (events in both kinds — 361/334
in each; learners 11 each). **P2 HELD exactly**, asserted by
test: both rooms expire both pools (48/15 and 48/10), the
offset values themselves asserted (375/0 and 375/375), and
no dawn tax fires in either room. **P5 HELD** (events
695 / 695, both ≥ 300). **P6 HELD** (fed 54 / 54, each ≥ 50).

**P3 HELD** — staggered burns **11**, one below this stage's
waning-750 (12): the stagger did not harm the safest room.
**P4 HELD by equality, and the equality is the result**:
staggered-both burns **11**, exactly staggered's 11 — as
declared before the run, equality is this stage's collapse
signature, and it is reported as a collapse, not a win.

Where the performance cracked: the principal's ordering was
three strict terms — *staggered 9 < coincident-late 13 <
coincident-750 16*, duration priced at 3 burns, the stagger
at 4. On this stage the ordering reads **staggered 11 =
coincident-late 11 < coincident-750 12**: the first
inequality collapses entirely. The two new rooms are
identical in *every* headline field and in *every night*
(nights 211 / 209 / 275 events, the same irregular/steady
split and the same acquittals each night); they differ only
where no headline reads — the earned expiry census (15
against 10: nights 1/7/7 against 1/6/3) and a single witness
learner (10 against 9). And both rooms differ from
waning-750 by exactly the *duration* term and nothing else:
the taught pool living to 1125 instead of 750 (48 taught
expiries against 62) buys +24 events, +3 checks, and the
single burn (12 → 11) in both rooms alike. Priced on this
stage, then: **duration 1 burn, the stagger 0** — the mirror
image of the cave's prices (3 and 4). The cause has the
address the README declared before the run: this stage's
time moves in claim-sized steps of roughly 450 owner-ticks
(waning-375 ≡ waning-150 here, field for field), and a
375-tick offset between two deadlines is smaller than the
step at which either deadline is ever *discovered* — the
claim tops visit both pools' expiries together whether the
paper says they land together or not, so the only term of
the principal's law that survives the swap is the one that
changed a unit's *total* life by more than a step. The
coincidence law is not refuted here; it is **below this
stage's resolution**, and the honest ledger says so: a
stagger must be wider than the step of the stage that
discovers it, or it is a gesture performed between two
claim tops, unseen.

## Next seeds

- The mirror stagger on this stage: both pools at 750, the
  *earned* pool's deadline offset instead — on a stage where
  the split already showed the pools are not symmetric
  (pooled-earned burned 15, pooled-taught 12), does it matter
  which forgetting waits?
- A stagger wider than the step: offset the taught deadline
  by a full claim-step (~450 owner-ticks) or more — the first
  stagger this stage can actually see, priced against the
  375 that it cannot.
- The witness room under a dawn tax still stands (Moth
  Keeper/Understudy).
- A lifetime priced in claims directly still stands (units
  expiring after N of their owner's claims, not ticks).

*Give me the role. Keep your machinery — and mind the step
your stage takes, or the role's finest gesture lands between
two of them, unseen.*
