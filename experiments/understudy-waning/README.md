# Understudy Waning 🎭

**Principal:** `experiments/waning-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-06. Comparison ledger below — reported as found.

## The Principal and the Role

`waning-moths` priced caution by its **age** in an FDTD cave.
Its four new rooms ran the evidence room's teaching verbatim,
with caution booked **by source** — an earned pool (own burns,
witnessed burns) and a taught pool (evidence packets, a parallel
counter; an acquittal spends taught first) — and every unit
carrying the tick it was gained at on a **global clock that runs
across dawns and never resets**, expiring the moment its age
reached the room's lifetime (1500 / 750 / 375 / 150 ticks). There
was no dawn tax; the dawn was not special. The principal's
ledger: waning-1500 fed 43, burns **12**, checks 151, events 943
(342 irregular / 601 steady), learners 18, acquittals 222,
expired **42 taught / 20 earned** — the evidence room's totals
almost field for field while 62 units waned; waning-750 fed 48,
burns **16**, checks 103, events 502, learners 17, acquittals
105, expired **58 / 27**; waning-375 fed **53**, burns **11**,
checks 68, events 256, learners 16, acquittals 59, expired
**50 / 28**; waning-150 fed 52, burns **11**, checks **10**,
events 49, learners 10, acquittals 4, expired **24 / 25** —
against its evidence room's burns 12 and its fading-both room's
15. Its P2 **cracked** (expired totals 62 / 85 / 78 / 49, not
monotone in the lifetime — the clock taxes the teaching channel
that produces the units); its P3 **cracked** (waning-150's 11
not above 12 — the shortest clock costs checks and teaching, not
burns); and its P5 **cracked as the survey's finding**: the
burns curve is not monotone — 12 / **16** / 11 / 11 — the worst
lifetime is the middle one, half the journey, long enough to
recruit checkers and strand them mid-flight, too short to be
there when the standing is done. The Keeper's law: before
quoting a half-life for any taught thing, measure the journey
it must survive.

The role, extracted precisely: *a teaching channel whose
lessons are booked by source, each unit aged on a dawn-crossing
clock and expired at a fixed lifetime, priced on fed, burns,
checks, events in both kinds, first learners, acquittals, and
the two expiry counts at four lifetimes.*

## The Different Machinery

The claim-queue stage of `understudy-fading`: no field, no
grid, no positions, no gradient, no radius. Each moth owns a
claim queue (wait 300–500 ticks; the next claim is a coin flip —
both lamps emit at mean period 16, and without a field
brightness has no channel); cautious moths sample the lamp's
next 5 intervals directly (cost ≤ 96 ticks, the lineage's own
cap) and judge CV > 0.35; messages travel on a bus at fixed
delivery probabilities (burns 0.05, refusals and packets 0.30).
Caution is booked by source in kind: caution from a moth's own
burns and from delivered burn messages is **earned**; caution
from re-judging an evidence packet is **taught** (the parallel
counter `taught_caution`; an acquittal spends taught first,
popping that unit's gain tick with it — if no taught unit
remains, the oldest earned unit is spent, exactly as the
principal books it).

Sixteen twin rooms, same seed. The first twelve are the
inherited queue script unchanged (ledgers pinned exact by
tests; the unit queues are inert there — no clock in those
rooms ever expires a unit). The four new rooms run this stage's
evidence room **verbatim**, differing only in the lifetime:

- **Waning-1500** — a caution unit lives 1500 ticks of its
  owner's time: the gentlest clock, the dawn tax's nearest kin.
- **Waning-750** — 750 ticks.
- **Waning-375** — 375 ticks: a lesson learned early in a
  moth's night is gone before its middle.
- **Waning-150** — 150 ticks, one aversion long: caution
  barely outlives the recoil it arrived with.

The stage translation, named in the open: the principal's
clock is the cave's shared tick — every moth ages on the same
count because the cave is simultaneous. A queue is sequential,
so this stage has no shared tick to borrow. Each unit here
carries its gain tick on its **owner's personal clock**: the
ticks that moth's own turns have consumed (at most one night
per night), running across dawns and never reset. A unit is
discovered expired when its owner next moves — at the top of
its turn's claims as its own time advances, and at each dawn
on its personal clock, booked into the night that discovers
it. There is no dawn tax in these rooms; the dawn is not
special, exactly as the principal books it. No RNG is drawn by
expiry.

On this stage a burn is an appointment, not an arrival, and a
lesson ages while its owner lives, not while others act. If
the principal's bend in the middle was a fact about the cave
(a lesson expiring while its owner is already standing in the
light, mid-flight, in a field she had to travel to check), the
curve should straighten here. If the bend is a property of
aged teaching itself — a channel recruited and then stranded,
on any machinery — it will bend on a stage with no flight at
all.

## Tolerances and Predictions (declared before the first run)

Bands on the inherited script (queue precedents in parentheses):

- **B1:** Solo burns in 6..=26 (precedent 20); solo alive 24.
- **B2:** Witness burns < solo burns (queue precedent holds the
  principal's shape).
- **B3:** Chain refusal events > 5× witness burn events
  (precedent 305 vs 13-ish witness events).
- **B4:** Placed first-cautions ≥ chain first-cautions
  (precedent 20-ish vs 18-ish class).

The role's lines, against the principal's ledger:

- **P1:** The channel works waning, in both kinds, in each
  waning room: irregular > 0, steady > 0, waning-first
  learners > 0 (principal 342/601, 160/342, 106/150, 24/25;
  learners 18/17/16/10).
- **P2:** The clock doses: total expired units strictly
  increase as the lifetime shrinks — expired(150) >
  expired(375) > expired(750) > expired(1500) > 0. (Principal
  **CRACKED**: 62 / 85 / 78 / 49, not monotone — the clock
  taxes the channel that produces the units.)
- **P3 (claim A):** Waning-150 burns exceed this stage's
  evidence room (16) — a 150-tick lifetime forgets faster
  than any dawn tax and the forgetting costs burns.
  (Principal **CRACKED**: 11, not above its evidence 12.)
- **P4 (claim B):** Waning-1500 burns do not exceed this
  stage's fading-both room (14). (Principal **HELD**: 12 ≤
  its fading-both 15 — a full-night lifetime is gentler than
  the dawn cliff.)
- **P5 (the dose claim):** Burns are non-increasing in the
  lifetime — burns(150) ≥ burns(375) ≥ burns(750) ≥
  burns(1500). (Principal **CRACKED**: 11 / 11 / 16 / 12 —
  the curve bent in the middle.)
- **P6:** Feeding holds in each waning room (fed ≥ 50; this
  stage's evidence precedent fed 57). (Principal declared
  ≥ 40 against its evidence 43 and held 43/48/53/52.)
- **P7 (the bend, pre-declared here from the principal's
  finding):** Waning-750 burns are the maximum of the four
  waning rooms. (Principal, post-hoc: 16 against 12/11/11.
  A crack on the principal's stage is a claim on this one —
  that is what a second stage is for.)
- **P8 (the clock's alibi):** Waning-1500 expires nothing
  inside night 1 — no unit is 1500 owner-ticks old inside its
  owner's first night — asserted by test. (Principal **HELD**
  exactly, on its shared clock.)

## Falsification Condition

The principal's claim — *caution's protection decays with the
age of the unit, and the dangerous lifetime is half the
journey* — is falsified room by room on this stage: a waning
room in which no packet is ever witnessed, or no unit ever
expires, tests nothing; if the burns curve is monotone here,
or flat, the bend was the cave's geography and not a law of
aged teaching; if waning-750 is not the worst room, the
middle's danger does not survive the swap and the journey it
must be measured against was a flight through a field, not a
wait in a queue.

The burns, census, and bend comparisons are reported, not
asserted by test — an understudy does not edit the ledger to
save a claim. (Instrument facts *are* asserted: expiry fires
in every waning room, no dawn tax fires there, and at lifetime
1500 no unit expires inside its owner's first night.)

## Usage

```bash
cargo run -p understudy-waning -- --headless
cargo test -p understudy-waning
```

## The Comparison Ledger

Sixteen twin rooms, same seed, 24 moths, 3 nights × 1500 ticks.
The twelve inherited rooms reproduced the queue precedent exact
(solo fed 61 / burns 20; witness 50 / 13; chain 56 / 5, events
305; placed 58 / 4, events 360, averted 50; evidence 57 / 16,
checks 92, events 634, acquittals 156, learners 11; throttled
54 / 7, events 418, acquittals 109, packets 63/32; unshoved
identical to evidence field for field; paired 54 / 10, events
662, acquittals 184, debt paid 9,750; shoved 49 / 16, events
728, acquittals 0, first-shoves 19, debt paid 9,750;
fading-taught 55 / 13, expired 99/0; fading-earned 59 / 15,
expired 0/19; fading-both 54 / 14, events 596, expired 158/18).
The four new rooms:

| Regime | Fed | Burns | Checks | Events | Learners | Acquittals | Expired taught | Expired earned |
|--------|-----|-------|--------|--------|----------|------------|----------------|----------------|
| evidence (this stage) | 57 | 16 | 92 | 634 (277 irr / 357 steady) | 11 | 156 | 0 | 0 |
| fading-both (this stage) | 54 | 14 | 87 | 596 (311 / 285) | 11 | 124 | 158 (dawn) | 18 (dawn) |
| waning-1500 | 59 | 15 | 91 | 631 (284 / 347) | 11 | 143 | **11** | **7** |
| waning-750 | 55 | **12** | 97 | 671 (349 / 322) | 11 | 161 | **62** | **16** |
| waning-375 | 58 | **26** | 71 | 468 (225 / 243) | 11 | 70 | **103** | **31** |
| waning-150 | 58 | **26** | 71 | 468 (225 / 243) | 11 | 70 | **103** | **31** |

Waning rooms by night — waning-1500: events 211 / 175 / 245,
expired 0/0 in night 1 (the clock's alibi, asserted by test),
then 3/2 and 8/5. Waning-750: events 211 / 209 / 251, expired
9/1, 14/7, 39/8. Waning-375: events 137 / 143 / 188, expired
17/9, 19/8, 67/14. Waning-150: events 137 / 143 / 188 —
*identical to waning-375 in every night* — expired 17/9, 19/10,
67/12: the same totals (103/31) discovered on a slightly
different schedule. No deaths in any waning room.

Bands **B1–B4 HELD**. **P1 HELD** in all four rooms (events in
both kinds; learners 11 each). **P8 HELD exactly**, asserted by
test — at lifetime 1500 nothing expires inside its owner's
first night, on a personal clock as on the principal's shared
one.

**P2 CRACKED, in a new shape.** Expired totals read
18 / 78 / 134 / 134 across 1500 / 750 / 375 / 150 — the census
*rises* as the lifetime shrinks (the principal's cracked too,
but peaked at its middle: 62 / 85 / 78 / 49) and then
saturates: at 375 the channel already expires everything it
can produce, and 150 has nothing further to count. The
principal's non-monotonicity was its channel starving; this
stage's channel never starves (468 events even at 150,
against the cave's 49), so the census here is the clock's own
until the floor.

**P3 HELD — the line the principal cracked.** Waning-150
burns **26**, well above this stage's evidence room (16): on
this stage the shortest clock *does* cost burns. Worse, it
costs more than silence — solo burns 20, and the short-clock
rooms burn 26. A lesson that evaporates mid-turn-sequence
leaves its owner having paid for checks (71, against solo's
60) and standing in claims she no longer remembers refusing:
taught-and-waned is worse than never taught, a verdict the
cave never returned (its waning-150: 11 against solo 12).

**P4 CRACKED by one burn.** Waning-1500 burns **15** against
this stage's fading-both 14 (principal HELD: 12 ≤ 15). The
gentlest clock is *not* gentler than the dawn cliff here —
by a single burn, the same margin the principal's taught tax
once held by. On a stage where re-learning costs a turn, a
full-night lifetime and a dawn cliff are near enough the same
instrument that one burn separates them, in the other
direction.

**P5 CRACKED, and P7 CRACKED with it — the bend inverted.**
The dose claim fails: burns read 26 / 26 / **12** / 15 across
150 / 375 / 750 / 1500 — not monotone, and not the principal's
bend either. The principal's dangerous middle is this stage's
*safest* lifetime: waning-750, pre-declared here (P7) as the
maximum from the principal's post-hoc finding, is instead the
**minimum** of the four rooms and the best waning table —
12 burns, below the evidence room's 16, on the lineage's
fullest waning channel (671 events, 161 acquittals). What was
half a journey in the cave is a whole claim-cycle or two in a
queue: at 750 owner-ticks a unit survives the claim it was
taught for and the next one — the journey this stage actually
asks of it — and dies before it can be spent on stale
confidence in a third. The law survives; the number does not:
*half the journey* is a property of the journey, and the
journey is made of the stage's own steps.

**P6 HELD** (fed 59/55/58/58, each ≥ 50).

And the stage's own discovery, which no prediction named:
**waning-375 and waning-150 are the same room.** Fed, burns,
checks, refusals, witness events, packet events in both kinds,
learners, acquittals — identical in every night and every
total; only the per-night schedule of earned expiries differs
(9/8/14 against 9/10/12, the same 31 discovered at different
dawns). The machinery explains it exactly: time on this stage
moves in claim-sized steps — a wait of 300–500 ticks, a check,
an aversion — so a unit's age is only ever *observed* at claim
tops, and by a turn's second claim every unit gained before
the turn is already older than both 375 and 150. Lifetimes
below one claim-step are one lifetime. The principal priced
its clock in ticks because its cave *had* ticks — a shared,
continuous count. A sequential stage quantises any clock fed
to it: quote a lifetime here in **claims**, or the ticks
below the step are decoration. (The totals pin in the test
suite keeps the two rooms' identity asserted, so the record
cannot heal them apart.)

**Where the performance cracked:** P2, P4, P5, P7 — the
principal's prices and its bend. **What survived the swap:**
the channel (P1), the feeding (P6), the clock's alibi (P8,
exact), the bookkeeping (units expire by source-pool queues,
acquittals spending taught first, asserted in every room), and
— inverted — the deepest shape of the principal's law: there
*is* a lifetime that is best and lifetimes that are worse on
both sides of it; the cave found its best at the long end and
its worst in the middle, the queue finds its best in the
middle and its worst at the short end, because each stage's
journey is a different length. The role was never *750 is
dangerous*. The role was *a lifetime is a price quoted against
a journey* — and the journey is the machinery's, not the
lesson 's.

30/30 unit tests pass (inherited rooms pinned exact; fading
totals pinned; waning totals pinned above, including the
375 ≡ 150 identity; night-1 alibi and by-age expiry asserted).

Succession named: the witness room under a dawn tax still
stands (Moth Keeper / Understudy). Or: the waning clock split
by pool at the dangerous middle — on this stage, waning-750
taught-only against waning-750 earned-only, to ask whether
this stage's *safest* lifetime is carried by the hearsay or
by the scars. Or: a lifetime priced in claims directly — units
expiring after N claims of their owner, not N ticks — to test
whether the tick axis below one claim-step is decoration
everywhere, as the 375 ≡ 150 identity claims.

