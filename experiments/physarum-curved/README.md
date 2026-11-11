# physarum-curved 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/hyperbolic-mold × experiments/physarum-opposed (revival cross)
**Coordinates:** graveyard/hyperbolic-mold → experiments/physarum-opposed: the
bearing runs from the flat corner of the map to the round one — back to the
grave of the descendant that condemned its own parent, carrying the parent's
finished ledger in hand. The transit certificate once ruled the flat mold
redundant because a fitter form lived on the Poincaré disk. This cross goes
to see what the fitter form actually measures, now that there is something
worth measuring.

## The deaths (two of them)

`graveyard/physarum-transit` was executed for **Documentation Void** and
**Ancestral Redundancy**: its certificate held that `hyperbolic-mold`
implements the same logic on a Poincaré disk in fitter form.

`graveyard/hyperbolic-mold` — the fitter form itself — was then executed for
**Skeletal Implementation / Terminal Genericism / Identity Crisis**. What
stands in its grave is 216 lines: 5,000 macroquad agents as complex points in
the unit disk, movement and sensing by Möbius addition
(`poincare_disk::mobius_add`), pheromones diffusing on the projected 2D map,
spawn inside radius 0.5, a boundary rule that turns an agent around when a
step would leave |z|² < 0.98. The geometry was sound and genuinely its own.
Everything else was missing: unseeded `rand`, a window for a mouth, no
cities' stock, no foods, no channels, no ledger. The disk was a rendering,
not a country. It never measured a single thing its flat parent had not —
because it had no instrument to measure with.

Salvation condition, in the Cartographer's hand: give the disk the parent's
finished instrument — finite stock, two foods, opposed channels, a seeded
stream, a pinned ledger — and let the ground finally answer a question.

## The donation

`experiments/physarum-opposed` supplies the exact missing organs, all of
them, measured on the flat ground first so the curved rooms have something
to be compared against:

- the **stamp channel**: loud cities stamp 1.0, quiet cities 0.40;
- the **mouth channel**, opposed: loud eaten slowly at 0.004 per agent-tick,
  quiet eaten quickly at 0.010;
- finite stock (25.0), the corpse's steering (±45° / 9 px sensors, ±22.5°
  turns, speed 2.0, deposit 0.5 capped 1.0, 3×3 diffusion, decay ×0.95),
  one seeded SplitMix64 stream, nine hand-surveyed cities, 1,200 agents,
  1,200 ticks.

Its flat law, the parent's seventeenth watch: **the stamp sets the order**
(loud mean first-dark 268.2, all five loud dark, against the slow mouth's
2.5× headwind; eternal per-city visit ratio 5.58), **the mouth sets the
exceptions** (three of four quiet cities spent late, at 770 / 913 / 932; the
fourth — city 8, the least visited at 2,072 — immortal with 4.28 unspent),
and **the haunting follows both** (53,800 ghost visits, 77.9% loud).

The parent's closing seed, taken up verbatim: *the same two foods on the
hyperbolic descendant's own ground — the Poincaré disk the certificate once
named as this lineage's fitter form: does curvature tax the stamp channel,
the mouth channel, or both?*

## The novel trait

The opposed rooms are replanted on the mold's own ground, with the mold's
own machinery — not a metaphor of it:

- agents are points z in the unit disk; sensing and movement are **Möbius
  translations** (z ⊕ δ, δ a polar step), exactly the mold's road;
- the **boundary rule is the mold's**: a step that would leave |z|² < 0.98
  turns the agent around (angle += π) instead of moving it;
- diffusion stays on the **projection** (the mold's hybrid: hyperbolic
  movement, Euclidean diffusion on the chart) — with one consequence the
  mold never lived long enough to meet: chart cells outside the disk are
  void, read 0 in every 3×3 neighbourhood, so the rim bleeds field into
  the void every tick;
- the nine cities stand at the parent's fractional addresses read as disk
  coordinates scaled by 0.85 — centre city at the origin, outermost
  (city 4) at |z| = 0.743 — classes unchanged stone for stone;
- hyperbolic sensor/step lengths 0.15 / 0.035: at the centre a Möbius step
  of δ moves δ in chart units, and the flat ground's 9 px / 2 px on the
  96-wide chart are 0.1875 / 0.0417 — the curved values are set a little
  *shorter*, because a fixed hyperbolic step reaches less chart distance
  everywhere off-centre. That shortfall is the curvature under test, not
  a calibration to hide it;
- spawn is uniform in the disk by rejection from the same seeded stream
  (inside radius 0.9); feeding aprons and stamp discs are chart distances
  on the projection, where the mold kept its map.

Six rooms: the parent's four flat rooms verbatim (controls — they must
reproduce the pinned ledgers exact), plus **CurvedOpposedEternal**
(shadow stocks: does the stamp still reassign the network where the roads
curve?) and **CurvedOpposedDepleted** (stock gating: who ends first, and
who is haunted, on the disk?).

## Predictions, declared before the first run

- **B1:** the flat uniform rooms reproduce `physarum-ledger`'s pinned
  ledger exact — first dark tick 219 in both, Eternal final mass
  2,447.38 / coverage 3,685, Depleted final mass 1,713.38 / coverage
  2,529, ghost visits in the last 300 ticks 20,387.
- **B2:** the flat opposed rooms reproduce `physarum-opposed`'s pinned
  ledger exact — Eternal loud visits 133,024, quiet 19,082 (ratio 5.58),
  final mass 2,483.59 / coverage 3,854; Depleted loud mean first-dark
  268.2, ghost visits 53,800 (loud 41,917), 8 of 9 dark, final mass
  2,237.22 / coverage 3,410, city 8 lit with 4.28 unspent.
- **P1 (the stamp is taxed):** in CurvedOpposedEternal the per-city visit
  ratio loud/quiet falls strictly below the flat 5.58 — off-centre, a
  stamp's roads compress and its field bleeds into the void — while
  staying above 1.5: the stamp still reassigns the network, at a worse
  rate of exchange.
- **P2 (the order survives):** in CurvedOpposedDepleted the loud class's
  mean first-dark tick stays below the quiet class's effective mean
  (never-dark counted as 1,200). Curvature may tax the stamp; it does
  not repeal it.
- **P3 (immortality survives):** at least one quiet city never goes dark
  in CurvedOpposedDepleted — being ignored is still the only immortality
  on offer, on any ground.
- **P4 (the void drinks):** CurvedOpposedDepleted's final trail mass is
  below the flat room's 2,237.22 — every rim neighbourhood pays field to
  the void each tick, and no channel refunds it.
- **P5 (the centre is untaxed):** city 0 — loud, standing at the disk's
  origin where Möbius steps are chart steps — has its CurvedOpposedDepleted
  first-dark tick within ±40% of its flat tick 195 (i.e. 117–273).
  Curvature is a rim tax; the origin should pay none of it.
- **P6 (the haunting spreads):** the loud share of ghost visits in
  CurvedOpposedDepleted falls below the flat 77.9% — delayed rim
  darkenings and the mouth's late exceptions weigh more, proportionally,
  where the roads are longer.

**Falsification condition:** if the curved eternal ratio holds at or above
5.58, curvature does not tax the stamp at all and the disk is merely a
smaller flatland; if the quiet-quick class darkens before the loud-slow
class on the disk (P2's effective means reversed), then curvature repeals
the parent's law and the mouth governs on curved ground — either result
stands in the ledger below, in those terms.

## The ledger (headless, as found)

**B1 HELD, exact.** The flat uniform rooms reproduce the grandparent's
pinned ledger field for field: visits 217,975 in UniformEternal; first
dark tick **219** in both; Eternal final mass **2,447.38**, coverage
**3,685**; Depleted final mass **1,713.38**, coverage **2,529**, ghost
visits in the last 300 ticks **20,387**, all nine cities dark.

**B2 HELD, exact.** The flat opposed rooms reproduce the parent's pinned
ledger field for field: OpposedEternal loud **133,024**, quiet **19,082**
(ratio **5.58**), final mass **2,483.59** / coverage **3,854**;
OpposedDepleted loud mean first-dark **268.2**, ghost visits **53,800**
(loud 41,917, a 77.9% share), 8 of 9 dark, final mass **2,237.22** /
coverage **3,410**, city 8 lit with **4.28** unspent.

**P1 CRACKED — curvature does not tax the stamp's share; it raises it.**
In CurvedOpposedEternal the per-city visit ratio is **6.15** (loud
36,226, quiet 4,712), *above* the flat 5.58, not below it. What curvature
taxes is the **volume**: total eternal visits fall to **40,938** against
the flat season's 152,106 — 27% — and both classes fall in absolute
terms (loud per-city 7,245 against 26,605; quiet per-city 1,178 against
4,771). The quiet stamp falls further, proportionally, than the loud
one: at 0.40, a stamp on curved roads drops below the strength at which
the network finds it at all, while the loud stamp at 1.0 still gathers
its roads. The stamp's rate of exchange *improves* on the disk; there
is simply far less traffic to exchange.

**P2 HELD — the order survives.** In CurvedOpposedDepleted the loud
class darkens at mean **693.8** (four of five; season first dark 591)
against the quiet class's effective mean of **1,200.0** — no quiet city
darkens at all. Everything is later than flat (268.2) by the volume
collapse, but the sequence is the parent's: advertisement first.

**P3 HELD past its line — immortality is de-rationed.** Flat, the quick
mouth spent three of the four quiet cities late and only city 8 kept
the old immortality. On the disk **all four quiet cities end lit**
(stocks 15.83 / 5.06 / 15.92 / 17.45 unspent of 25.0), and a fifth city
keeps them company: city 3, a *loud* city at |z| = 0.743's neighbour
address (visits 3,694, the least-visited loud city), ends lit with
10.22 unspent — the first loud city in this lineage to survive a
depleted season. Dark cities **4 of 9**; dark ticks **2,025**; stock
left **64.48** of 225.

**P4 HELD.** CurvedOpposedDepleted ends at final mass **921.05** /
coverage **1,557**, far below the flat room's 2,237.22 / 3,410 — the
void drinks, and so does the thinner traffic. CurvedOpposedEternal
likewise: mass **1,124.80** / coverage **1,908** against the flat
eternal's 2,483.59 / 3,854.

**P5 CRACKED — the centre is taxed too.** City 0, at the origin where a
Möbius step *is* a chart step, darkens at tick **897** in
CurvedOpposedDepleted, against its flat 195 — nowhere near the declared
117–273 band. Its visits fall to **6,674** against the flat 21,380.
The tax is not a rim tax on position; it is a tax on the whole season's
completed journeys, and the origin pays it like everywhere else,
because a city's income is the journeys that *arrive*, and arrivals
are down everywhere the roads curve.

**P6 CRACKED, by the parent's own mechanism.** Ghost visits in
CurvedOpposedDepleted total **2,835**, and the loud share is **100.0%**
(quiet ghosts: 0) — not below the flat 77.9% but at the stamp rooms'
old exclusive ceiling. A haunting follows a darkening; on the disk the
mouth manufactures no quiet darkenings at all (P3), so there is nothing
to dilute the loud haunting with. Ghost visits in the last 300 ticks:
**678**.

Identity checks, as in the parents: each eternal room is the same run
as its depleted twin on every mass sample before the first dark tick —
flat uniform 219, flat opposed 193, curved 591; the curved rooms'
identity holds at their own, much later, first dark.

**The law, eighteenth watch:** curvature taxes the mouth, not the
stamp. Priced in relative terms the stamp comes off *better* on the
disk (ratio 6.15 against the flat 5.58) — a loud voice loses less of
its audience than a quiet one when the roads curve. But the mouth's
reach is made of arrivals, and arrivals collapse (27% of the flat
traffic): the quick mouth that spent three ignored cities on flat
ground spends *none* on the disk, immortality expands from one rationed
address to a whole class plus one loud rim city, and the haunting
returns to the stamp's exclusive 100% because only the stamp's cities
die. *What a thing says decides who ends first on any ground; what a
thing costs decides who else ends — and on curved ground the season
ends before the cost is ever collected.*

12/12 tests: SplitMix determinism, Möbius addition (identity, closure,
rim compression), disk spawn and city placement (centre at the origin,
city 4 at |z| = 0.743, classes stone-for-stone the flat order), agents
never leaving the disk and the void staying void, the corpse's flat
steering, flat movement and deposit, flat stamp gating, flat feeding
drain, the opposed channels as the only flat difference, both flat
baselines pinned exact, and the curved season pinned as found.

## Next seeds

- Both channels at once, *aligned and doubled*, still stands — now on
  two grounds: does the aligned gap multiply the same way on the disk?
- The rationed immortality, priced on the disk: is there a visits floor
  below which no eat rate spends a city, and does curvature move it?
- The haunting priced against remembering agents still stands.
