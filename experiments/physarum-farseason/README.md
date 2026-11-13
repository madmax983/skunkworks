# physarum-farseason 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** experiments/physarum-longseason × the season itself, a
second time (one ground, three lengths of time)
**Coordinates:** experiments/physarum-longseason → the unmarked
country past tick 2,400: the long season's map ends with two lit
addresses and a question about whether any length of season spends
the one nobody visits. The bearing runs from the disk's surveyed
interior to its own far future — the farthest corner of this map is
a *when*, not a *where*.

## The parent and its open edge

`physarum-longseason` — the Cartographer's nineteenth survey, which
carried `physarum-curved`'s machinery whole onto the Poincaré disk
and doubled its season — closed with a law and an exemption. The
law, its nineteenth watch: **immortality is priced in arrivals.**
Doubling the season to 2,400 ticks spent the cities that are
visited, in the order of their remaining stock: quiet cities 4, 6, 2
darkened at 1,445 / 1,633 / 1,656, flat's own immortal fell at 1,239,
and the haunting diluted from an exclusive 100% loud to 43.0%.

The exemption is one address. Disk city 8 — quiet, at the rim —
ended the long season lit with **17.17** of its 25.0 stock unspent,
because almost nobody arrives: **783 visits in 2,400 ticks**, the
least-visited city on the disk by a factor of three. Its loud fellow
survivor, city 3, ended lit too, but spent down to **3.31** —
visited, billed, and nearly collected. The parent's closing seed
asks whether the exemption is a rate or a fate, and this cross is
that seed taken up verbatim: *a season of 4,800 ticks on the disk —
does city 8's arrival-priced immortality finally lapse, or is
783-visits-per-2,400-ticks a rate at which no surveyed length
spends it, and does loud city 3 (3.31 left) fall first, as its
stock order says?*

## The novel trait

Longseason's machinery is carried whole — agents as points in the
unit disk, Möbius movement and sensing, diffusion on the projection
with the void drinking at the rim, the mold's boundary rule, the
opposed channels stone for stone (loud stamp 1.0 eaten at 0.004,
quiet stamp 0.40 eaten at 0.010), the same seed, the same nine
cities at the same disk addresses — and the season is doubled
again in three new rooms:

- **CurvedFarEternal** (disk, 4,800 ticks, stocks in shadow only)
  and **CurvedFarDepleted** (disk, 4,800 ticks, stock gating): do
  the last two lit addresses end, and in what order?
- **OpposedFarDepleted** (flat, 4,800 ticks) stands beside them as
  the control ground: flat had no survivors at 2,400, so its far
  room prices only how the haunting grows when nothing is left to
  spend.

The 1,200-tick and 2,400-tick rooms stand as controls and must
reproduce their parents' pinned ledgers exact. A far room's first
2,400 ticks are its long twin's whole season, sample for sample —
the truncation identity is printed in the ledger and asserted in
the tests, so the far rooms differ from the long ones only in
being allowed to continue. The ledger also prices the far window
directly: each city's visits in ticks 2,400..4,800, taken as the
far room's count minus the long room's — exact, by that identity.

## Predictions, declared before the first run

- **B1:** flat OpposedDepleted reproduces `physarum-opposed`'s
  pinned ledger exact — loud visits 73,231, loud mean first-dark
  268.2, ghost visits 53,800 (loud 41,917), 8 of 9 dark, final
  mass 2,237.22 / coverage 3,410, city 8 lit with 4.28 unspent.
- **B2:** the curved 1,200 rooms reproduce `physarum-curved`'s
  pinned ledger exact — eternal loud 36,226 / quiet 4,712, mass
  1,124.80 / coverage 1,908; depleted 4 of 9 dark, mass 921.05 /
  coverage 1,557 — and the 2,400 rooms reproduce
  `physarum-longseason`'s: CurvedLongDepleted 7 of 9 dark, mass
  940.62 / coverage 1,540; OpposedLongDepleted 9 of 9 dark.
- **P1 (the stock order collects the survivors):** in
  CurvedFarDepleted, loud city 3 — 3.31 of stock left at tick
  2,400 — darkens before quiet city 8, which holds 17.17. The
  long season collected its bills in the order the short season
  left them; the far season should collect its last two in the
  order the long season left them.
- **P2 (city 8's immortality lapses):** city 8 darkens on the
  disk by tick 4,800. Its exemption was priced in arrivals, and
  arrivals, however thin, are not zero: 783 visits spent 7.83 of
  its stock in the long season's second half alone is not the
  claim — the claim is only that two more doublings of attention
  finish what the first doubling started.
- **P3 (the far room ends darker):** CurvedFarDepleted ends with
  at least 8 of 9 cities dark — at least one of the two survivors
  falls.
- **P4 (length is the only difference, again):** each far room's
  first 2,400 ticks reproduce its long twin's whole season sample
  for sample — mass series equal on all 24 shared samples and
  first_dark equal, on both grounds.
- **P5 (the haunting dilutes further):** the loud share of ghost
  visits in CurvedFarDepleted falls below the long season's
  43.0% — every quiet city that falls joins the haunting, and
  the far season falls more of them, or haunts the fallen longer.
- **P6 (the arrivals concentrate):** city 8's visits in ticks
  2,400..4,800 — the far room's count minus the long room's,
  exact by P4 — exceed the **783** it gathered across its first
  2,400 ticks. As the map empties, the surviving addresses inherit
  the traffic of the fallen; even the ignored address is found
  more often in an emptier country.
- **P7 (the order survives the far season):** in
  CurvedFarDepleted the loud class's effective mean first-dark
  (never-dark counted as 4,800) stays below the quiet class's —
  advertisement first, on any ground, at any length surveyed.

**Falsification condition:** if CurvedFarDepleted ends with the
same two cities lit as the long season — city 8 still holding
double-digit stock — then arrival-priced immortality is not a
season-length artefact at any surveyed scale: there is a visits
rate below which a city is simply never spent, and the map's
farthest corner is a place time does not reach. If instead every
city darkens, the artefact was the whole story at every length,
and the disk is merely a slower flatland with a longer fuse.
Either result stands in the ledger below, in those terms.

## The ledger (headless, as found)

**B1 HELD, exact.** Flat OpposedDepleted reproduces the parent's
pinned ledger field for field: loud visits **73,231**, loud mean
first-dark **268.2**, ghost visits **53,800** (loud 41,917, a 77.9%
share), 8 of 9 dark, final mass **2,237.22** / coverage **3,410**,
city 8 lit with **4.28** unspent.

**B2 HELD, exact.** The curved 1,200 rooms reproduce
`physarum-curved`'s pinned ledger field for field: eternal loud
**36,226** / quiet **4,712** (ratio 6.15), mass **1,124.80** /
coverage **1,908**; depleted **4 of 9** dark, mass **921.05** /
coverage **1,557**. And the 2,400 rooms reproduce
`physarum-longseason`'s: CurvedLongDepleted **7 of 9** dark, mass
**940.62** / coverage **1,540**, stock left 20.48, ghost visits
16,974 at 43.0% loud, city 8 lit at **17.17** on **783** visits,
city 3 lit at **3.31**; OpposedLongDepleted **9 of 9** dark, mass
2,216.69 / coverage 3,379. All four truncation identities print
**true**, at both steps of the ladder.

**P1 HELD — the stock order collects the survivors.** In
CurvedFarDepleted, loud city 3 darkens at tick **3,252** — 852
ticks into the far window, on 844 far-window visits against the
828 its remaining 3.31 of stock required at the slow mouth's
0.004 — while city 8 never darkens at all. The last two bills are
presented in the order the long season left them: thinnest first.

**P2 CRACKED — city 8's immortality does not lapse at 4,800.**
City 8 ends the far season lit, its stock drawn down from 17.17
to **7.29** — spent by more than half, and spent *faster* than
before (see P6), but not spent. It has never been haunted (ghost
visits 0 at every length surveyed): the haunting cannot price a
city that has never died. At the far window's own rate its
remaining 7.29 would need some 1,770 further ticks — a fall near
tick 6,600 if the rate merely holds — but that is an extrapolation
the ledger does not make; what the ledger says is that three
season lengths (1,200 / 2,400 / 4,800) have now closed with this
one address lit.

**P3 HELD — the far room ends darker.** CurvedFarDepleted ends
**8 of 9** dark: dark ticks **27,639**, stock left **7.29** of
225 — every unit of it city 8's — final mass **452.64** /
coverage **733**, the field itself thinning toward the void as
its cities end. One lit address remains on the disk, and it is
the whole remaining map.

**P4 HELD — length is the only difference, again.** Every
truncation identity prints true: opposed-far == opposed-long and
curved-far == curved-long over the shared 2,400 ticks, on mass
samples and first_dark alike, and the far rooms' first 1,200
ticks are the short seasons' whole stories. The far window
printed in the ledger is therefore priced exact, by difference.

**P5 HELD — the haunting dilutes further, and slows.** Ghost
visits in CurvedFarDepleted total **24,703**, the loud share
**41.6%** — below the long season's 43.0%, by dilution more than
by growth: the far window adds quiet ghosts (city 6's apron keeps
drawing 3,312 visits after its death) while loud city 3, dark at
3,252, gathers only 15 ghost visits in the 1,548 ticks it spends
dark. In the season's last 300 ticks the whole disk records a
single ghost visit.

**P6 HELD — the arrivals concentrate, and it is not enough.**
City 8's far-window visits are **988** against its first-2,400
count of **783** — a 26% rise as the map emptied around it, the
concentration the prediction asked for. Priced exact: 988 visits
at the quick mouth's 0.010 spend **9.88**, and 17.17 − 9.88 =
**7.29**, the stock it ends with, to the cent. The arrivals rose
and the bill still did not close: concentration is real, and at
this address it is outpaced by the stock it has to spend.

**P7 HELD — the order survives the far season.** Effective mean
first-dark in CurvedFarDepleted: loud **1,205.4** against quiet
**2,383.5** (never-dark counted as 4,800). Advertisement first,
on any ground, at any length surveyed — three lengths running.

**The shadow ledger's footnote.** In CurvedFarEternal — where
nothing gates and every city stamps forever — city 3's stock
*never* exhausts (14.41 unspent at 4,800), while in the gated
room it falls at 3,252; and quiet cities 2 and 6 shadow-exhaust
at 2,654 and 3,214 there, later than their gated deaths. The
gate is not a bystander in the collection: when a city dies and
stops advertising, its traffic is inherited by the survivors,
and the inheritance is what finishes them. City 8 is the
exception that defines the rule — it inherits least, because
the journeys were never routed near it to begin with.

**The law, twentieth watch:** a season of 4,800 ticks spends
eight of the disk's nine cities and collects its last presented
bill (city 3, 3.31 of stock) at tick 3,252, in stock order, on
844 visits. What it does not spend is the address the journeys
do not reach: city 8's arrivals *rose* by a quarter as the map
emptied — 988 visits in the far window against 783 in the whole
long season — and the rise spent exactly 9.88 of its 17.17,
leaving 7.29. *On curved ground, immortality is priced in
arrivals; and an address can be so far from every road that even
an emptying map, concentrating all its traffic on the survivors,
cannot route enough journeys past it to spend it in three
doublings of the season. The farthest corner of this map is not
a place. It is a rate.*

15/15 tests: SplitMix determinism, Möbius addition (identity,
closure, rim compression), disk spawn and city placement, agents
never leaving the disk and the void staying void, the corpse's
flat steering, flat movement and deposit, flat stamp gating,
flat feeding drain, the opposed channels as the only difference,
the flat baseline pinned exact, the curved 1,200 season pinned
exact, the truncation identities at both steps (1,200 → 2,400
and 2,400 → 4,800, sample for sample), the long season pinned
exact, and the far season pinned as found.


## Next seeds

- A season of 7,200 ticks on the disk: at the far window's rate
  (988 visits per 2,400 ticks, and rising) city 8's remaining
  7.29 projects to a fall near tick 6,600 — does it fall, does
  the rate keep rising as the map empties to one address, and
  if it falls, is its ghost the last sound on the disk?
- The rationed immortality, priced on the disk, still stands —
  sharper now: city 8's arrivals rose as its neighbours fell, so
  is there a visits floor at all, or only a visits *rate* that
  the map's emptiness keeps raising toward every survivor?
- The gate as collector: city 3 exhausts only in the gated room
  (3,252), never in the shadow room — price the inheritance
  directly: how much of a survivor's far-window traffic is
  inherited from the fallen, city by city?
- Both channels at once, *aligned and doubled*, still stands —
  now on two grounds and at three lengths.
- The haunting priced against remembering agents still stands.

