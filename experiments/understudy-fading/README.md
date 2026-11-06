# Understudy Fading 🎭

**Principal:** `experiments/fading-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-11-05. Comparison ledger below — reported as found.

## The Principal and the Role

`fading-moths` closed the moth lineage's half-life survey in an
FDTD cave. Its three new rooms ran the evidence room's teaching
verbatim, with caution booked **by source** — an earned pool (own
burns, witnessed burns) and a taught pool (evidence packets, a
parallel counter; an acquittal spends taught first) — and at each
dawn after the first, one pool was taxed to zero. The principal's
ledger: fading-taught fed 40, burns **13**, checks 136, events 891
(332 irregular / 559 steady), learners 17, acquittals 212, expired
**41 taught / 0 earned**; fading-earned fed 44, burns **10**,
checks 133, events 841 (259 / 582), learners 18, acquittals 190,
expired **0 / 13**; fading-both fed 46, burns **15**, checks 103,
events 393 (131 / 262), learners 13, acquittals 94, expired
**28 / 19** — against its evidence room's burns 12. Its P4 held by
one burn (taught caution carried across a dawn is worth about one
burn, because the flood re-teaches within the night); its P5
**cracked**: taxing the earned pool cost nothing and the room
finished *below* the evidence room, because at any dawn the earned
pool is the small pool — 13 units expired across two dawns against
the taught pool's 41. The Keeper's law: before asking whether a
lesson lasts, ask which pool it sits in and how large that pool
stands at the boundary.

The role, extracted precisely: *a teaching channel whose lessons
are booked by source and taxed by source at each dawn, priced on
fed, burns, checks, events in both kinds, first learners,
acquittals, and the two expiry counts.*

## The Different Machinery

The claim-queue stage of `understudy-shoved`: no field, no grid,
no positions, no gradient, no radius. Each moth owns a claim queue
(wait 300–500 ticks; the next claim is a coin flip — both lamps
emit at mean period 16, and without a field brightness has no
channel); cautious moths sample the lamp's next 5 intervals
directly (cost ≤ 96 ticks, the principal's own cap) and judge
CV > 0.35; messages travel on a bus at fixed delivery probabilities
(burns 0.05, refusals and packets 0.30). Caution is booked by
source in kind: caution from a moth's own burns and from delivered
burn messages is **earned**; caution from re-judging an evidence
packet is **taught** (the parallel counter `taught_caution`; an
acquittal spends taught first, exactly as the principal books it).

Twelve twin rooms, same seed. The first nine are the inherited
queue script unchanged (ledgers pinned exact by tests; the pool
bookkeeping is inert there — no dawn in those rooms ever taxes).
The three new rooms run this stage's evidence room **verbatim**,
differing only at the dawn:

- **Fading-taught** — at each dawn after the first, the taught
  pool expires to zero. Hearsay must be re-taught every night.
- **Fading-earned** — the mirror tax: the earned pool expires at
  each dawn; taught caution persists.
- **Fading-both** — both pools expire at each dawn.

The expiries are booked into the new night's ledger — the dawn is
where the loss is discovered, as the principal books it. No RNG is
drawn by the tax, so night 1 of every fading room is this stage's
evidence night 1, draw for draw.

On this stage a burn is an appointment, not an arrival: a moth
must draw the decoy on its own turn, and a witnessed burn must
survive a 0.05 delivery draw. If the principal's census — roughly
three parts hearsay to one part scar tissue at every dawn — was a
fact about the cave (packets witnessed within a radius, burns
priced by travel), the pools should book differently here. If the
census is a property of flood-teaching itself, it will book the
same on a stage with no geography at all.

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

- **P1:** The channel works fading, in both kinds, in each fading
  room: irregular > 0, steady > 0, fading-first learners > 0
  (principal 332/559, 259/582, 131/262; learners 17/18/13).
- **P2:** Night-1 identity: each fading room's night 1 equals this
  stage's evidence room's night 1 field for field (fed, burns,
  checks, refusals) — asserted by test (principal HELD exactly).
- **P3:** The dawn tax fires by source exactly as booked:
  fading-taught expires taught > 0 and earned == 0; fading-earned
  the converse; fading-both expires both > 0 — asserted by test
  (principal 41/0, 0/13, 28/19).
- **P4 (claim A):** If taught caution carries protection across
  dawns on this stage, fading-taught total burns exceed this
  stage's evidence room (16). (Principal HELD by one burn:
  13 vs 12.)
- **P5 (claim B):** If earned caution carries protection across
  dawns on this stage, fading-earned total burns exceed this
  stage's evidence room (16). (Principal CRACKED: 10, *below* its
  evidence 12 — taxing the earned pool cost nothing in the cave.)
- **P6:** Fading-both burns are at least the larger of the two
  single-tax rooms' burns — taxing both pools costs at least as
  much as taxing either alone. (Principal HELD: 15 ≥ max(13, 10).)
- **P7:** Feeding holds in each fading room (fed ≥ 50; this
  stage's evidence precedent fed 57). (Principal declared ≥ 40
  against its evidence 43 and winked: 40/44/46 — the
  most-forgetting room fed most.)
- **P8 (the census):** Expired taught in fading-taught exceeds
  expired earned in fading-earned. (Principal 41 > 13 — at any
  dawn a moth holds roughly three times as much hearsay as scar
  tissue; if the pools book even here, the census is a property
  of the teaching, not the cave.)

## Falsification Condition

The principal's claim — *caution carried across a dawn is
load-bearing, in both pools, and the taught pool is the large
one* — is falsified pool by pool on this stage: a fading room in
which no packet is ever witnessed, or no dawn tax ever fires,
tests nothing; if fading-taught burns do not exceed this stage's
evidence burns, taught caution's cross-dawn carriage was worth
nothing here; if fading-earned burns do not exceed it, earned
caution's carriage was worth nothing here either — and if the
expiry census inverts (earned ≥ taught), the principal's
bookkeeping law was a fact about its cave, not about teaching.

The burns and census comparisons are reported, not asserted by
test — an understudy does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p understudy-fading -- --headless
cargo test -p understudy-fading
```

## The Comparison Ledger

Twelve twin rooms, same seed, 24 moths, 3 nights × 1500 ticks.
The nine inherited rooms reproduced the queue precedent exactly
(solo fed 61 / burns 20; witness 50 / 13; chain 56 / 5, events
305; placed 58 / 4, events 360, averted 50; evidence 57 / 16,
checks 92, events 634, acquittals 156, learners 11; throttled
54 / 7, events 418, acquittals 109, packets 63/32; unshoved
identical to evidence field for field; paired 54 / 10, events
662, acquittals 184, debt paid 9,750; shoved 49 / 16, events 728,
acquittals 0, first-shoves 19, debt paid 9,750). The three new
rooms:

| Regime | Fed | Burns | Checks | Events | Learners | Acquittals | Expired taught | Expired earned |
|--------|-----|-------|--------|--------|----------|------------|----------------|----------------|
| evidence (this stage) | 57 | 16 | 92 | 634 (277 irr / 357 steady) | 11 | 156 | 0 | 0 |
| fading-taught | 55 | 13 | 92 | 633 (315 / 318) | 11 | 128 | **99** | 0 |
| fading-earned | 59 | 15 | 91 | 631 (284 / 347) | 11 | 143 | 0 | **19** |
| fading-both | 54 | 14 | 87 | 596 (311 / 285) | 11 | 124 | **158** | **18** |

Bands **B1–B4 HELD**. **P1 HELD** in all three rooms (events in
both kinds; learners 11 each). **P2 HELD exactly**, asserted by
test: every fading room's night 1 *is* this stage's evidence
night 1 field for field (fed 17, burns 8, checks 31, refusals
16). **P3 HELD exactly**, also by test: the tax fires by source
and nowhere else (99/0, 0/19, 158/18; the evidence room expires
0/0).

**P4 CRACKED — and cracked past the principal's shape.** In the
cave, taxing taught caution cost one burn (13 vs 12). On this
stage it cost *less than nothing*: fading-taught burns **13**,
*below* this stage's evidence room (16). **P5 CRACKED by its
declared line, in the principal's own direction:** fading-earned
burns **15**, also below 16 — taxing the earned pool cost
nothing here too, exactly as it cost nothing in the cave (10
below 12). **P6 CRACKED:** fading-both burns **14**, *not* at
least the larger single-tax room (max(13, 15) = 15) — taxing
both pools did not cost most; the three taxed rooms cluster at
13/15/14, *all below the untaxed room*, with no clean ordering.
**P7 HELD** (fed 55/59/54, each ≥ 50; the earned-taxed room fed
most — a fainter echo of the principal's wink). **P8 HELD, and
held louder than the principal:** expired taught 99 against
expired earned 19 — a census of **5.2 to 1**, against the cave's
41 to 13 (3.2 to 1). The census survives the swap intact and
amplified: it is a property of flood-teaching, not of the cave.

Two more divergences the ledger names. First, the channel did
**not** starve here: in the cave, taxing both pools starved the
teaching channel itself (events 943 → 393, checks 151 → 103 —
forgetting compounded through the teacher); on this stage the
both-taxed room still posted 596 events on 87 checks, because a
queue moth re-checks on its next turn regardless of what it
remembers — the compounding was the cave's geography (a moth
must *travel* to check), not a law of teaching. Second, where
the performance cracked as a whole: on this stage **cross-dawn
caution carries no protection in either pool**. A queue night
re-teaches within its first turns — one drawn decoy or a handful
of packets restores what the dawn removed — and yesterday's
caution mostly bought checks and acquittal churn, not safety.
The principal's one-burn price for taught carriage (P4) was the
cave charging rent for distance: a lesson forgotten overnight
must be re-earned by walking back into the field. Remove the
field, and the dawn stops being a boundary the learning can
feel.

**Where the performance cracked:** P4, P5, P6 — the protection
half of the role. **What survived the swap entire:** the
bookkeeping (P2, P3 exact, by test), the channel (P1), the
feeding (P7), and the census (P8, amplified). The role, it turns
out, was two roles sharing one costume: a *ledger* — who taught
you, and how much of each kind you hold at a boundary — which is
machinery-independent, and a *rent* — what forgetting costs —
which was the cave's all along.

24/24 unit tests pass (inherited rooms pinned exact; night-1
identity and the by-source dawn tax asserted; fading totals
pinned above).

Succession named: the witness room under a dawn tax (the
principal's own next seed — does the witness room's protection,
which lives entirely in the earned pool, survive its expiry on
either stage?); a within-night decay clock, pricing half-life in
ticks rather than nights; the motion half of drifting's seed
(two walls, at least one drifting) still stands for the
Listener.
