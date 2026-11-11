# physarum-opposed 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/physarum-transit × experiments/physarum-appetite (revival cross)
**Coordinates:** graveyard/physarum-transit → experiments/physarum-appetite: back
to the flat corner of the map a fourth time, to the grave that has been raised
three times — for its missing books, for its identical foods, and for a quality
priced in the wrong organ. Bearing between them: the second raising priced a
food's quality in its stamp and the network reassigned itself; the third priced
the same quality in its mouth and the network changed nothing it did, only how
soon it was done. This cross sets the two priced channels against each other on
the same cities and asks which one the exhaustion order obeys when they
disagree.

## The death

`graveyard/physarum-transit` (edition 2021, macroquad + rayon) was a complete
Physarum transport simulation on a flat toroidal plane: 5,000 agents sensing
the trail field at ±45° / 9 px, rotating ±22.5° toward the strongest sensor,
advancing at speed 2.0, depositing 0.5 (capped 1.0); 15 cities, each stamping
its radius-3 disc to 1.0 every tick, forever; the field diffusing as a 3×3
mean and decaying ×0.95 per tick.

It was executed for **Documentation Void** (no README at all) and **Ancestral
Redundancy**: its certificate held that `hyperbolic-mold` implements the same
logic on a Poincaré disk and carries its genetic memory in fitter form. Its
salvation conditions were written on the certificate: differentiate with a
new biological mechanism the descendant does not have, document, and
demonstrate emergent behaviour the hyperbolic variant does not capture.

The first raising (`physarum-ledger`) answered with finite stock. The second
(`physarum-quality`) priced quality in the **stamp** — rich cities at 1.0,
plain at 0.40, both eaten at 0.004 — and the result was total: the louder food
drew 5.6× the visits per city, was eaten out first, was haunted exclusively
(43,525 ghost visits, all rich), and the quiet food was never arrived at. The
third (`physarum-appetite`) moved the same 2.5× into the **mouth** — every
stamp at 1.0, rich eaten at 0.010, plain at 0.004 — and the result inverted:
the visits were the uniform visits visit for visit, the softer food ended
first by the mouth alone, every city was spent, and the haunting was shared
(48.5% rich). The parent's own closing seed named the unasked question
verbatim: *both channels at once, opposed — which channel sets the exhaustion
order when they disagree?*

## The donation

`experiments/physarum-appetite` supplies the exact missing organ: **both
channels, separately measured on the same map** — quality's stamp ledger
(1.0 against 0.40) and its own eat-rate ledger (0.010 against 0.004), each
priced alone against the same uniform baseline, same map, same seed, same
spawn, same stock. This cross keeps every one of the lineage's numbers and
changes exactly one thing: it runs both channels at once, with opposite
signs.

## The novel trait

The two foods are crossed in the organs. The five cities that were rich in
both parents (the centre and every second ring city) are now **loud-slow**:
they stamp at **1.0** (quality's rich stamp, the corpse's own strength) and
are eaten at **0.004** per agent per tick (appetite's plain rate — the slow
mouth). The four that were plain are **quiet-quick**: they stamp at **0.40**
(quality's plain stamp) and are eaten at **0.010** (appetite's rich rate —
the quick mouth). A food's total drain is visits × rate; the stamp sets the
visits and the mouth sets the rate, and here they point in opposite
directions, 2.5× each. Whichever ledger the season's exhaustion order
resembles is the channel that governs it.

Four rooms, one seed, one spawn, one city map (96×64 torus, 1,200 agents,
1,200 ticks, nine hand-surveyed cities):

- **UniformEternal** — the grandparent verbatim: stocks fall in a *shadow
  ledger* but never gate the stamp. The baseline this crate must reproduce
  exact.
- **UniformDepleted** — the grandparent verbatim: the stock gates the stamp.
- **OpposedEternal** — opposed channels, stocks in shadow only: does the
  stamp still reassign the network when the mouth argues the other way?
- **OpposedDepleted** — opposed channels, stock gating: which food ends
  first when advertisement and appetite disagree, and who is haunted?

Named adaptations, inherited from the lineage: the corpse's 800×600 /
5,000-agent / 15-random-city stage stays at the ledger's size, cities placed
by hand so every room shares one map; steering drawn from one seeded
SplitMix64 stream in agent order; the corpse's window left in the grave
(`--headless` is the only mouth).

## Predictions, declared before the first run

- **B1:** the uniform rooms reproduce `physarum-ledger`'s pinned ledger
  exact — first dark tick 219 in both, Eternal final mass 2,447.38 /
  coverage 3,685, Depleted final mass 1,713.38 / coverage 2,529, ghost
  visits in the last 300 ticks 20,387.
- **P1:** in OpposedEternal the loud cities draw more visits per city than
  the quiet cities, ratio above 2.0 — the stamp reassigns even with the
  mouth opposed (in an eternal room the mouth is invisible by appetite's
  own law, so this is the control the rest of the ledger stands on).
- **P2:** in OpposedDepleted the loud cities end first: all five loud
  cities go dark, and their mean first-dark tick is below the quiet
  class's effective mean, counting a never-dark city as tick 1,200 —
  exhaustion follows the stamp despite the loud food being 2.5× slower
  to eat, because drain is visits × rate and the stamp sets the visits.
- **P3:** at least one quiet city never goes dark in OpposedDepleted — a
  city barely visited cannot be spent even at the quick rate; being
  ignored remains the only immortality on offer in this lineage.
- **P4:** ghost visits in OpposedDepleted concentrate on the loud cities:
  the loud share of all ghost visits exceeds 90% — the haunting follows
  the stamp's roads, not the mouth's speed.
- **P5:** OpposedDepleted ends richer than UniformDepleted in final trail
  mass — the unspent quiet reserves keep stamping weakly all season, as
  they did in the quality rooms.

**Falsification condition:** if OpposedDepleted's quiet-quick cities darken
earlier than the loud-slow ones (by the effective mean of P2), or the ghost
visits split anywhere near appetite's shared 48.5%, the mouth — not the
stamp — governs exhaustion when the channels disagree, and the ledger below
is the instrument that would show it.

## The ledger (headless, as found)

**B1 HELD, exact.** The uniform rooms reproduce the grandparent's pinned
ledger field for field: visits 217,975 in UniformEternal; first dark tick
**219** in both uniform rooms; Eternal final mass **2,447.38**, coverage
**3,685**; Depleted final mass **1,713.38**, coverage **2,529**, ghost
visits in the last 300 ticks **20,387**, all nine cities dark.

**P1 HELD, exact — the control is perfect.** OpposedEternal's ledger is
`physarum-quality`'s eternal ledger visit for visit: loud **133,024**,
quiet **19,082**, per-city means 26,605 against 4,771, a ratio of **5.58**.
Final mass and coverage are the quality room's own (2,483.59 / 3,854). In an
eternal room the opposed mouth leaves no trace on where the network goes;
the only place it shows is the shadow ledger, where the quiet cities'
would-be dark ticks fall at a mean of **779.7** (504–1,118, one city never
spent even in shadow) against the loud cities' 266.2 — the quick mouth's
bill is written in shadow long before the gated room presents it.

**P2 HELD, at the parent's own number.** In OpposedDepleted every loud
city goes dark, at a mean first-dark tick of **268.2** (193–349) — the
quality rooms' rich mean to the decimal. The slow mouth did not delay the
loud food by a single tick: its drain is visits × rate, and at 5.6× the
visits, the 0.4× rate never stood a chance. The quiet class's effective
mean (never-dark counted as 1,200) is **953.8**. First dark tick of the
season: **193**, the stamp rooms' own.

**P3 HELD, by one city — and the exception is the finding.** Three of the
four quiet cities *do* go dark in OpposedDepleted, at ticks **770, 913,
and 932** — late, long after every loud city is spent. Under the stamp
alone (the quality rooms) no plain city ever darkened at all; the quick
mouth, given a whole season, spends even a city the network barely visits.
Exactly one quiet city keeps the old immortality: city 8, the least
visited on the map (2,072 visits all season), ends still lit with **4.28**
of its 25.0 unspent. Being ignored is still the only immortality here —
but at the quick rate it is rationed to the single most-ignored address,
not granted to a whole class. Dark cities 8 of 9; dark ticks **5,644**.

**P4 CRACKED — the haunting leaks.** Ghost visits total **53,800**, and
the loud share is **77.9%** (loud 41,917, quiet 11,883), not the declared
90% and not the stamp rooms' exclusive 100%. The mechanism is P3's: a
haunting follows a darkening, and the mouth manufactures three late
quiet darkenings the stamp never would, each collecting its own ghosts
(city 4 alone: 7,718). The opposed ledger stands between its parents —
the stamp concentrates the haunting, the mouth dilutes it. Ghost visits
in the last 300 ticks: **15,968**.

**P5 HELD.** OpposedDepleted ends at final mass **2,237.22** / coverage
**3,410**, well above UniformDepleted's 1,713.38 / 2,529 — and a little
below the quality season's 2,251.42 / 3,567. The gap between the two
depleted seasons is the price of P3's late deaths: the quality season
kept four weak stamps lit all 1,200 ticks; this season lost three of
them for its last 270–430 ticks.

**The law, seventeenth watch:** when the channels disagree, the stamp
sets the order and the mouth sets the exceptions. Advertisement decides
who ends first — entirely, at its own measured numbers, against a 2.5×
headwind in the mouth — because exhaustion is visits × rate and the
stamp owns the visits. But appetite decides who *else* ends at all:
given a season, a quick mouth spends cities the advertisement had
written off as immortal, and every late death it manufactures takes its
share of the haunting with it. *What a thing says decides where the
roads go and who is spent first; what a thing costs decides how far
down the ignored list the spending reaches before the season ends.*

Identity checks, as in the parents: each eternal room is the same run as
its depleted twin on every mass sample before the first dark tick
(uniform 219, opposed 193; shadow tick == dark tick in both spellings).

10/10 tests: corpse steering, agent motion and deposit, stamp gating,
feeding drain and darkening, the eternal shadow, ghost counting, the
opposed channels as the only difference (loud centre 0.95 and quiet
centre 0.38 after diffusion in the opposed room, both 0.95 in uniform;
per-visit drain 0.004 loud against 0.010 quiet), the uniform baseline
pinned to the grandparent, and the opposed season pinned as found.


## Next seeds

- The same two foods on the hyperbolic descendant's own ground — the
  Poincaré disk the certificate once named as this lineage's fitter form:
  does curvature tax the stamp channel, the mouth channel, or both?
- Both channels at once, *aligned and doubled* — loud-and-quick against
  quiet-and-slow at the full 2.5× in each organ: is the exhaustion gap
  the product of the two channels, or does one channel's ceiling cap it?
- The haunting priced against remembering agents still stands.
