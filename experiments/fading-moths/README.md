# Fading Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/shoved-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-05. Survival ledger below — reported as found.

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
- The succession since: **`witness-moths`** (a burn teaches its
  witnesses; burns 8), **`chain-moths`** (a refusal teaches too;
  13), **`placed-moths`** (the refusal carries its direction; 13),
  **`evidence-moths`** (a completed check transmits its actual peak
  intervals for witnesses to re-judge; 12), **`throttled-moths`**
  (one packet per checker per night; 12), **`unshoved-moths`**
  (packets teach caution with no recoil; 18 — the recoil was
  load-bearing when it travelled with a lesson), **`shoved-moths`**
  (packets teach the recoil with no caution; 14, and the lineage's
  only death — neither half was the medicine; the pairing was).
- **`shoved-moths`** named the succession taken up verbatim here:
  *the half-life survey this lineage keeps deferring — four tuition
  kinds, four decay clocks, in one room: does caution earned from a
  burn decay at the same rate as caution taught by evidence, and
  does a taught shove's protection (if any) outlive the caution it
  refused to carry?* The shove half was priced by the parent
  itself: its protection was nil while it lasted, so its half-life
  is moot. What remained was the caution half.

## The Novel Trait

Every prior room let caution, once gained, persist for all three
nights — health, caution and scars carry across dawns; only
positions reset. No room ever asked which pool of caution does the
carrying. This planting books caution **by source** and taxes one
pool per dawn:

- The **earned pool**: caution from a moth's own burns and from
  burns it witnessed at the lamp.
- The **taught pool**: caution from re-judging an evidence packet,
  booked in a parallel counter (`taught_caution`) the moment a
  packet teaches it. An acquittal spends taught caution first —
  hearsay is what an acquittal un-teaches.

Eleven twin rooms, same seed. The first eight are the parents'
rules, unchanged, and tests pin their ledgers exactly (solo fed 45
/ burns 12; witness fed 43 / burns 8; chain fed 42 / burns 13,
chain events 495, chain-first 18; placed fed 39 / burns 13, placed
events 618, placed-first 20; evidence fed 43 / burns 12, checks
151, evidence events 943, evidence-first 18, acquittals 223;
throttled fed 44 / burns 12, throttled events 534, packets sent
55, suppressed 105; unshoved fed 39 / burns 18, unshoved events
1,605, acquittals 439; shoved fed 44 / burns 14, shoved events
867, shoved-first 20, acquittals 0, deaths 1). The three new rooms
run the evidence room's packets, re-judgement, aversion and
acquittal rule **verbatim**, differing only at the dawn:

- **Fading-taught** — at each dawn after the first, the taught
  pool expires to zero. Hearsay must be re-taught every night;
  earned caution persists.
- **Fading-earned** — the mirror tax: the earned pool expires at
  each dawn; taught caution persists. A burn's lesson lasts one
  night only.
- **Fading-both** — both pools expire at each dawn. Every night
  starts with a population exactly as naive as night 1's, except
  for its scars and its health.

The expiries are booked into the new night's ledger — the dawn is
where the loss is discovered. The keeper intervenes never; the
moths write the outcome.

## Predictions (declared before the first run)

- **P1:** The channel works fading: in each fading room packets
  are witnessed in both kinds (irregular > 0, steady > 0) and
  fading-first learners number > 0.
- **P2:** Night-1 identity: no dawn has yet passed, so each fading
  room's night 1 equals the evidence room's night 1 field for
  field (fed, burns, checks, refusals).
- **P3:** The dawn tax fires by source exactly as booked:
  fading-taught expires taught > 0 and earned == 0; fading-earned
  the converse; fading-both expires both > 0.
- **P4 (claim A):** If taught caution carries protection across
  dawns, fading-taught total burns exceed the evidence room's 12.
- **P5 (claim B):** If earned caution carries protection across
  dawns, fading-earned total burns exceed the evidence room's 12;
  the ordering of the two rooms prices which pool carries more.
- **P6:** Fading-both burns are at least the larger of the two
  single-tax rooms' burns — taxing both pools costs at least as
  much as taxing either alone.
- **P7:** Feeding holds near the evidence room's level (≥ 40) in
  each fading room: forgetting is not starvation.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *caution carried across a dawn is load-bearing, in
both pools* — is falsified pool by pool:

- a fading room in which no packet is ever witnessed, or no dawn
  tax ever fires, tests nothing;
- if fading-taught burns do not exceed the evidence room's 12,
  taught caution's cross-dawn carriage was worth nothing;
- if fading-earned burns do not exceed 12, earned caution's
  cross-dawn carriage was worth nothing.

The burns comparisons are reported, not asserted by test — a
keeper does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p fading-moths -- --headless
cargo test -p fading-moths
```

## The Survival Ledger

Eleven twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights (except where a
dawn tax spends caution, as booked):

| Regime | Fed | Burns | Checks | Events | Learners | Acquittals | Expired taught | Expired earned |
|--------|-----|-------|--------|--------|----------|------------|----------------|----------------|
| solo | 45 | 12 | 73 | 0 | 0 | 0 | 0 | 0 |
| witness | 43 | 8 | 84 | 13 witness | 6 | 0 | 0 | 0 |
| chain | 42 | 13 | 208 | 495 | 18 | 0 | 0 | 0 |
| placed | 39 | 13 | 223 | 618 | 20 | 0 | 0 | 0 |
| evidence | 43 | 12 | 151 | 943 | 18 | 223 | 0 | 0 |
| throttled | 44 | 12 | 169 | 534 | 18 | 122 | 0 | 0 |
| unshoved | 39 | 18 | 211 | 1,605 | 18 | 439 | 0 | 0 |
| shoved | 44 | 14 | 136 | 867 | 20 first-shoves | 0 | 0 | 0 |
| fading-taught | 40 | 13 | 136 | 891 (332 irr / 559 steady) | 17 | 212 | **41** | 0 |
| fading-earned | 44 | **10** | 133 | 841 (259 irr / 582 steady) | 18 | 190 | 0 | **13** |
| fading-both | 46 | 15 | 103 | 393 (131 irr / 262 steady) | 13 | 94 | 28 | 19 |

Fading rooms by night — fading-taught: night 1 fed 19, burns 5,
checks 28, events 108, learners 11, acquittals 38, expired 0/0;
night 2 fed 9, burns 4, checks 53, events 406, learners 6,
acquittals 94, expired taught **13**; night 3 fed 12, burns 4,
checks 55, events 377, learners 0, acquittals 80, expired taught
**28**. Fading-earned: night 1 identical (19/5/28, events 108,
learners 11, acquittals 38); night 2 fed 14, burns 2, checks 59,
events 393, learners 7, acquittals 107, expired earned **10**;
night 3 fed 11, burns 3, checks 46, events 340, learners 0,
acquittals 45, expired earned **3**. Fading-both: night 1
identical; night 2 fed 14, burns 4, checks 38, events 141,
learners 2, acquittals 29, expired 13 taught / 10 earned; night 3
fed 13, burns 6, checks 37, events 144, learners 0, acquittals
27, expired 15 taught / 9 earned. No deaths in any fading room;
every room dawns 24/24 except the shoved room's night 3 (23).

Prediction verdicts, as declared above: **P1 HELD** in all three
rooms (events in both kinds everywhere; learners 17 / 18 / 13).
**P2 HELD, exactly** — each fading room's night 1 is the evidence
room's night 1 field for field (fed 19, burns 5, checks 28,
refusals 8, events 108, learners 11, acquittals 38), asserted by
test: before the first dawn, the tax has nothing to tax.
**P3 HELD, exactly** — the tax fires by source as booked and
nowhere else (fading-taught 41/0, fading-earned 0/13,
fading-both 28/19; the evidence room expires 0/0), also asserted
by test. **P4 HELD, by one burn** — fading-taught burns **13**
against evidence's 12: taught caution carried across a dawn is
worth something, but on this stage it is worth about one burn in
three nights — because the channel re-teaches fast (891 events
against the evidence room's 943; a taxed moth is re-taught within
the night by the same flood that taught it first). **P5 CRACKED
— the survey's finding** — fading-earned burns **10**, *below*
the evidence room's 12, the second-best table in the lineage
(behind only the witness room's 8): expiring the earned pool at
dawn cost nothing and read, on this stage, as a small gain. The
earned pool at a dawn is also the *small* pool — 13 units expired
across two dawns against the taught pool's 41 — because burns
are rare and packets are a flood: at any dawn the average moth
holds roughly three times as much hearsay as scar tissue, and
the hearsay is what the checking population is mostly made of.
**P6 HELD** — fading-both burns **15** ≥ max(13, 10): taxing both
pools is the costliest single change, and its deeper price is
upstream of the burns — with both pools taxed, checks fall to
103 and the teaching channel itself starves to **393 events**
(against 943 untaxed): forgetting compounds, because fewer
cautious moths means fewer checks means fewer packets means
fewer taught moths tomorrow. **P7 HELD, with a wink from the
ledger** — feeding held everywhere (40 / 44 / 46), and the
most-forgetting room fed *most*: fading-both's 46 is the highest
table in the lineage, bought with the fewest checks. Caution is
not free; every check is up to 96 ticks standing still in the
light, and a moth that forgets by dawn spends its nights flying.

The honest reading, kept whole: the two pools do not have the
same half-life because they do not do the same work. Taught
caution is the population's *standing army* — numerous,
replaceable within a night by the packet flood, worth about a
burn a survey when its cross-dawn carriage is taxed. Earned
caution is a *scar* — rare at any dawn (a moth can only hold as
much as it has been burned or stood near burning), and on this
evidence its cross-dawn carriage prices at zero or less: the
moths it would have kept checking were kept checking anyway by
the taught pool, and what the tax removed may have been mostly
redundant vigilance over a mixture both lamps had already
re-judged. Decay, in this cave, is not one clock: it is a
question of which pool is scarce, and the scarce pool was never
the one doing the carrying. The witness room's 8 still stands
above every table here — and note what the survey just priced
about it: its caution is almost entirely earned-at-the-lamp,
the pool this survey found worth least across dawns. Its price
must therefore be paid *within* the night — rarity, direction,
or pairing — not across them.

Named succession seed: the witness room under a dawn tax — if
its 8 survives the expiry of its own (earned) caution, its price
is within-night entirely, and the last candidate standing is the
at-lamp direction of its recoil; if it degrades toward the
evidence room's 12, cross-dawn carriage was its hidden half
after all, contradicting this survey's pool pricing and
demanding a second look at pool sizes. Or: a within-night decay
clock — caution taxed continuously (a unit expires N ticks after
it is earned or taught) instead of at the dawn boundary — to
price the half-life in ticks rather than in nights.

*Every lamp is a claim. Check the claim — and ask how long the
checking lasts: hearsay fades by dawn and is re-taught by noon;
a scar fades too, and on this evidence the cave does not miss it.*
