# Waning Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/fading-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-06. Survival ledger below — reported as found.

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
  load-bearing when it travelled with a lesson),
  **`shoved-moths`** (packets teach the recoil with no caution;
  14, and the lineage's only death — neither half was the
  medicine; the pairing was), **`fading-moths`** (caution booked
  by source and taxed at the dawn: taught taxed, expired 41,
  burns 13; earned taxed, expired 13, burns 10 — the scarce pool
  was never the one doing the carrying; both taxed, burns 15 and
  the teaching channel starved 943 → 393 events).
- **`fading-moths`** named the succession taken up verbatim here:
  *a within-night decay clock — caution taxed continuously (a
  unit expires N ticks after it is earned or taught) instead of
  at the dawn boundary — to price the half-life in ticks rather
  than in nights.*

## The Novel Trait

Every prior room taxed caution, if at all, at a boundary: the
dawn. A dawn tax is a cliff, not a clock — a unit gained at tick
1490 of a night and a unit gained at tick 10 are taxed
identically, though one is a night old and the other minutes
from its first flight. No room had ever priced caution by its
**age**.

This planting books caution by source exactly as `fading-moths`
does — the **earned pool** (own burns, witnessed burns) and the
**taught pool** (re-judged evidence packets, booked in the
parallel counter `taught_caution`; an acquittal spends taught
first) — and every unit additionally carries the tick it was
gained at, on a **global clock that runs across dawns and never
resets**. A unit expires the moment its age reaches the room's
lifetime, mid-night or mid-check, wherever the age falls. There
is no dawn tax in these rooms; the dawn is not special.

Fifteen twin rooms, same seed. The first eleven are the parents'
rules, unchanged, and tests pin their ledgers exactly (solo fed
45 / burns 12; witness fed 43 / burns 8; chain fed 42 / burns 13,
chain events 495, chain-first 18; placed fed 39 / burns 13,
placed events 618, placed-first 20; evidence fed 43 / burns 12,
checks 151, evidence events 943, evidence-first 18, acquittals
223; throttled fed 44 / burns 12, throttled events 534, packets
sent 55, suppressed 105; unshoved fed 39 / burns 18, unshoved
events 1,605, acquittals 439; shoved fed 44 / burns 14, shoved
events 867, shoved-first 20, acquittals 0, deaths 1; fading-taught
fed 40 / burns 13, expired taught 41; fading-earned fed 44 /
burns 10, expired earned 13; fading-both fed 46 / burns 15,
expired taught 28 / earned 19). The four new rooms run the
evidence room's packets, re-judgement, aversion and acquittal
rule **verbatim**, differing only in the lifetime:

- **Waning-1500** — a caution unit lives one full night (1500
  ticks): the gentlest clock, the dawn tax's nearest kin.
- **Waning-750** — half a night.
- **Waning-375** — a quarter night: a lesson learned at a
  night's start is gone before its middle.
- **Waning-150** — a tenth of a night, one aversion long (150
  ticks): caution barely outlives the recoil it arrived with.

The expiries are booked into the night in which the unit's age
runs out — a night, like a moth, discovers its forgetting
mid-flight. The keeper intervenes never; the moths write the
outcome.

## Predictions (declared before the first run)

- **P1:** The channel works waning: in each waning room packets
  are witnessed in both kinds (irregular > 0, steady > 0) and
  waning-first learners number > 0.
- **P2:** The clock doses: total expired units strictly increase
  as the lifetime shrinks — expired(150) > expired(375) >
  expired(750) > expired(1500) > 0.
- **P3 (claim A):** Waning-150 burns exceed the evidence room's
  12 — a 150-tick lifetime forgets faster than any dawn tax and
  the forgetting costs burns.
- **P4 (claim B):** Waning-1500 burns do not exceed
  fading-both's 15 — a full-night lifetime is gentler than
  expiring everything at dawn regardless of age, because a unit
  gained late in a night survives into the next.
- **P5 (the dose claim):** Burns are non-increasing in the
  lifetime — burns(150) ≥ burns(375) ≥ burns(750) ≥
  burns(1500). If the curve is not monotone, the half-life is
  not a single number and the ledger says where it bends.
- **P6:** Feeding holds near the evidence room's level (≥ 40)
  in each waning room: forgetting is not starvation.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *caution's protection decays with the age of the
unit, at a price the lifetime prices* — is falsified room by
room:

- a waning room in which no packet is ever witnessed, or no unit
  ever expires, tests nothing;
- if waning-1500 burns exceed fading-both's 15, a full-night
  lifetime is not gentler than the dawn cliff, and the age of a
  unit was never the variable — only its survival to a boundary
  was;
- if the burns curve is flat in the lifetime, caution's age
  prices at zero everywhere on this stage and the clock is
  decoration.

The burns comparisons are reported, not asserted by test — a
keeper does not edit the ledger to save a claim. (One instrument
fact *is* asserted: expiry fires in every waning room, the dawn
tax never fires there, and at lifetime 1500 no unit expires
inside night 1 — no unit is that old yet.)

## Usage

```bash
cargo run -p waning-moths -- --headless
cargo test -p waning-moths
```

## The Survival Ledger

Fifteen twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights (in the waning
rooms, subject to the tick clock, as booked):

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
| fading-taught | 40 | 13 | 136 | 891 (332 irr / 559 steady) | 17 | 212 | **41** (dawn) | 0 |
| fading-earned | 44 | **10** | 133 | 841 (259 irr / 582 steady) | 18 | 190 | 0 | **13** (dawn) |
| fading-both | 46 | 15 | 103 | 393 (131 irr / 262 steady) | 13 | 94 | 28 (dawn) | 19 (dawn) |
| waning-1500 | 43 | 12 | 151 | 943 (342 irr / 601 steady) | 18 | 222 | **42** | **20** |
| waning-750 | 48 | **16** | 103 | 502 (160 irr / 342 steady) | 17 | 105 | 58 | 27 |
| waning-375 | **53** | 11 | 68 | 256 (106 irr / 150 steady) | 16 | 59 | 50 | 28 |
| waning-150 | 52 | 11 | **10** | 49 (24 irr / 25 steady) | 10 | 4 | 24 | 25 |

Waning rooms by night — waning-1500: night 1 fed 19, burns 5,
checks 28, events 108, learners 11, acquittals 38, expired 0/0
(no unit is 1500 ticks old inside the first night — the clock's
own alibi, asserted by test); night 2 fed 11, burns 6, checks
58, events 299, learners 5, acquittals 75, expired 10 taught /
9 earned; night 3 fed 13, burns 1, checks 65, events 536,
learners 2, acquittals 109, expired 32/11. Waning-750: night 1
fed 19, burns 5, checks 27, events 108, learners 11, acquittals
38, expired 11/8; night 2 fed 18, burns 3, checks 21, events 124,
learners 3, acquittals 18, expired 8/4; night 3 fed 11, burns
**8**, checks 55, events 270, learners 3, acquittals 49, expired
39/15. Waning-375: night 1 fed 17, burns 4, checks 22, events
103, learners 11, acquittals 35, expired 14/9; night 2 fed 17,
burns 4, checks 23, events 82, learners 4, acquittals 12, expired
16/12; night 3 fed 19, burns 3, checks 23, events 71, learners 1,
acquittals 12, expired 20/7. Waning-150: night 1 fed 16, burns 3,
checks **2**, events 16, learners 8, acquittals 1, expired 11/7;
night 2 fed 17, burns 2, checks 2, events 12, learners 0,
acquittals 3, expired 0/4; night 3 fed 19, burns 6, checks 6,
events 21, learners 2, acquittals 0, expired 13/14. No deaths in
any waning room; every room dawns 24/24 except the shoved
room's night 3 (23).

Prediction verdicts, as declared above: **P1 HELD** in all four
rooms (events in both kinds everywhere; learners 18 / 17 / 16 /
10). **P2 CRACKED — and the crack is the survey's first
finding** — expired totals read 62 / 85 / 78 / 49 across
1500 / 750 / 375 / 150, not monotone: the clock taxes the
teaching channel that *produces* the units, so as the lifetime
shrinks the population gains fewer units to expire (waning-150
witnesses only 49 events in three nights against the evidence
room's 943 — there is almost nothing left in the room old
enough to count as forgotten). An expired-unit census, like
the dawn census before it, is a property of the flood, not of
the clock alone. **P3 CRACKED** — waning-150 burns **11**, not
above the evidence room's 12: the shortest clock does not cost
burns at all. What it costs is *checking* (151 → 10) and
*teaching* (943 → 49 events); a room that forgets in 150 ticks
simply returns to near-solo behaviour (solo: burns 12, checks
73) with even less checking than solo — and solo was never the
worst room in this lineage. **P4 HELD** — waning-1500 burns
**12** ≤ fading-both's 15, and the stronger fact stands beside
it: at a full-night lifetime the room reproduces the evidence
room's totals almost field for field (fed 43, burns 12, checks
151, events 943, irregular 342 / steady 601) while expiring 62
units — every one of them a unit whose protection had already
been spent, acquitted, or outlived. A lifetime of one night is
the dawn tax with the cliff filed off, and the filing cost
nothing. **P5 CRACKED — the survey's finding** — the burns
curve is not monotone in the lifetime: 12 / **16** / 11 / 11.
The worst lifetime in the lineage is the *middle* one. At 750
ticks a unit lives long enough to recruit a checker and build
a channel (502 events, 17 learners) but not long enough to
protect her: the damage lands in night 3 (8 of the room's 16
burns), when night-2 lessons expire mid-flight and the room's
standing army discovers its tuition has waned while it is
already standing in the light. At 375 and 150 the channel
never fully forms, the moths stay near-naive, and near-naive
was always survivable here — fed 53 and 52, the two highest
tables in the lineage, bought with 68 and 10 checks. **P6
HELD, with the lineage's wink again** — feeding held everywhere
(43 / 48 / 53 / 52), and the two shortest clocks fed *most*:
vigilance is priced in meals at every lifetime, and a moth
whose lesson wanes by noon spends its nights flying.

The honest reading, kept whole: a half-life is not a dose with
one price. Priced in ticks, caution's decay has three regimes,
not a curve — a long lifetime (≥ one night) is the dawn tax
minus the cliff, and costs nothing the evidence room was not
already paying; a short lifetime (≤ a quarter night) dissolves
the teaching channel entirely and returns the room to its
naive baseline, burns and all; and between them stands the
dangerous middle, where the lesson lasts long enough to change
behaviour — to recruit checkers, to build a flood, to send
moths standing into the light — and not long enough to be
there when the standing is done. The dawn survey asked which
*pool* does the carrying; the tick survey answers with a
*when*: the carrying is done by units whose lifetime covers
the journey from teaching to arrival, and a clock set to half
that journey is worse than no clock at all. Before quoting a
half-life for any taught thing, measure the journey it must
survive; a half-life quoted without its journey is a census
wearing a clock's authority.

Named succession seeds: the witness room under a dawn tax
still stands — if its 8 survives the expiry of its own
(earned) caution, its price is within-night entirely, and the
at-lamp direction of its recoil is the last candidate standing.
Or: the waning clock at the dangerous middle, split by pool —
waning-750 taught-only against waning-750 earned-only — to ask
whether the middle's danger is carried by the hearsay (which
the flood replaces nightly) or by the scars (which it cannot).
Or: a lifetime priced against the journey directly — units
that expire on *arrival or check*, not on age — to test
whether the middle's damage is expiry itself or expiry
*mid-journey*.

*Every lamp is a claim. Check the claim — and check its age:
a lesson that wanes at half the journey is worse than one
never taught, and the clock that forgets fastest fed the
room best.*
