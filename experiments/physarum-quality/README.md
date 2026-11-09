# physarum-quality 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/physarum-transit × experiments/physarum-ledger (revival cross)
**Coordinates:** graveyard/physarum-transit → experiments/physarum-ledger: the flat
corner of the map, where a slime mould was executed for having no books and was
raised once already — back to the same grave a second time, because the first
raising priced the food's *finitude* and never its *quality*. Bearing between
them: the corpse's cities were all the same food; the parent taught them to be
spent; this cross asks what a network does when one food is better than another
and both run out at the same rate.

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

The first raising (`physarum-ledger`, fourteenth watch) answered the
certificate's first charge: the corpse's cities are infinite. This cross
answers a charge no certificate has yet written: **the corpse's cities are
all the same food.** Every meal stamps at the same strength, so no ledger in
this lineage can say whether a network prefers the better food, what the
preference costs, or which food a network abandons first when both are
finite. The parent's own closing seed named the question verbatim: *a stock
priced by quality — two foods, one ledger, which does the network abandon
first?*

## The donation

`experiments/physarum-ledger` supplies the exact missing organ: **the books
themselves** — a finite stock per city (25.0), feeding that spends it (0.004
per agent standing in the city's apron per tick), a stamp gated on the stock,
darkness that is counted, ghost visits that are counted, and a shadow ledger
in its Eternal room against which the gated rooms can be priced. This cross
keeps every one of the parent's numbers — same map, same seed, same spawn,
same stock, same price per mouthful — and changes exactly one thing.

## The novel trait

The stamp's strength is the food's **quality**. Five cities are rich (the
centre and every second ring city — quality alternates around the ring, so
neither food owns a corner of the map) and four are plain. A rich city stamps
its disc to **1.0**, the corpse's own strength; a plain city stamps to
**0.40**. Quality enters nowhere else: the stocks are equal, the eat rate is
equal, the deposit and diffusion and decay are the corpse's. One variable,
moved once, so the ledger can name its price.

Four rooms, one seed, one spawn, one city map (96×64 torus, 1,200 agents,
1,200 ticks, nine hand-surveyed cities):

- **UniformEternal** — the parent verbatim: stocks fall in a *shadow ledger*
  but never gate the stamp. The baseline this crate must reproduce exact.
- **UniformDepleted** — the parent verbatim: the stock gates the stamp.
- **QualityEternal** — quality stamps, stocks in shadow only: does the
  louder food draw more mouths when nothing can run out?
- **QualityDepleted** — quality stamps, stock gating: which food is
  abandoned first, and which is still visited after the end?

Named adaptations, inherited from the parent: the corpse's 800×600 /
5,000-agent / 15-random-city stage is shrunk to the ledger's size, cities
placed by hand so every room shares one map; steering drawn from one seeded
SplitMix64 stream in agent order; the corpse's window left in the grave
(`--headless` is the only mouth). The parent's third room (Regrown) is not
carried over: regrowth is a second moving part, and this ledger holds the
season to eternal/depleted so the quality comparison stands alone.

## Predictions, declared before the first run

- **B1:** the uniform rooms reproduce `physarum-ledger`'s pinned ledger
  exact — first dark tick 219 in both, Eternal final mass 2,447.38 /
  coverage 3,685, Depleted final mass 1,713.38 / coverage 2,529, ghost
  visits in the last 300 ticks 20,387.
- **P1:** in QualityEternal the rich cities draw more visits in total than
  the plain cities — the advertisement works.
- **P2:** in QualityDepleted the rich cities go dark earlier on average
  than the plain cities — the advertisement is spent first.
- **P3:** every city still goes dark in QualityDepleted before tick 1,200 —
  quality delays the plain, it does not save them.
- **P4:** ghost visits in QualityDepleted concentrate on the rich cities:
  the rich ghost total exceeds the plain ghost total — the network keeps
  the roads to the good food longest.
- **P5:** QualityDepleted ends poorer than UniformDepleted in final trail
  mass — the plain stamps' weakness is a tax the whole field pays.

**Falsification condition:** if QualityEternal's rich and plain visit totals
stand within a few percent of each other, or QualityDepleted's two classes
darken in the opposite order to P2, the stamp-strength channel carries no
preference in this model and the quality claim fails — the ledger below is
the instrument that would show it.

## The ledger (headless, as found)

**B1 HELD, exact.** The uniform rooms reproduce the parent's pinned ledger
field for field: visits 217,975 in UniformEternal; first dark tick **219**
in both uniform rooms; Eternal final mass **2,447.38**, coverage **3,685**;
Depleted final mass **1,713.38**, coverage **2,529**, ghost visits in the
last 300 ticks **20,387**, all nine cities dark. The baseline is the parent;
everything below is measured against it.

**P1 HELD, past its line.** In QualityEternal the rich cities draw
**133,024** visits against the plain cities' **19,082** — per city, a mean
of 26,605 against 4,771, a 5.6× preference for the louder food when nothing
can run out. The advertisement does not nudge the network; it reassigns it.
Three of the four plain cities end the eternal season with stock unspent
(8.33, 13.50, 16.29 of 25.0 remaining): at stamp 0.40 they are not chosen
less, they are effectively not on the map. QualityEternal ends at mass
2,483.59 / coverage 3,854, above the uniform eternal room on both.

**P2 HELD, in its strong form.** In QualityDepleted every rich city goes
dark (first dark ticks 193–349, mean **268.2**) — the advertised food is
eaten out first, completely.

**P3 CRACKED — the headline.** No plain city goes dark at all. All four end
the depleted season still lit, holding 10.15, 1.55, 12.46, and 16.59 of
their opening stock. The declaration said quality would *delay* the plain;
the ledger says quality *removes* them from the economy — a city nobody
visits cannot be spent, so the worse food is not abandoned last, it is
never arrived at. QualityDepleted: dark cities 5 of 9, dark ticks 4,659
(against the uniform room's 9 of 9 and 7,547).

**P4 HELD, total.** Ghost visits in QualityDepleted number **43,525**, and
every one of them is to a rich city; the plain ghost total is **0**. The
network keeps the roads to the good food after the good food is gone, and
keeps no roads at all to the food it ignored. Ghost visits in the last 300
ticks: 9,313, all rich.

**P5 CRACKED, inverted.** QualityDepleted ends at final mass **2,251.42** /
coverage **3,567** — *above* UniformDepleted's 1,713.38 / 2,529, not below.
The plain stamps' weakness is not a tax on the field; it is a reserve the
network cannot spend. Because the plain cities are never eaten out, four
weak sources keep stamping all season, and the concentrated rich network's
trails plus that unspendable reserve outweigh a uniform field whose every
source dies. A mixed-quality field outlasts an equal one — inequality of
quality is, measurably, a savings account the network never meant to open.

**The law, fifteenth watch:** a network does not abandon its worse food
last — it never arrives there; what it abandons is its best food, first
and entirely, and then it haunts it. Price a food's quality before quoting
a network's loyalty: at these numbers, preference, exhaustion, and
haunting are the same measurement read at three ticks.

Identity checks, as in the parent: each eternal room is the same run as its
depleted twin on every mass sample before the first dark tick (uniform
219, quality 193; shadow tick == dark tick in both spellings).

10/10 tests: corpse steering, agent motion and deposit, stamp gating,
feeding drain and darkening, the eternal shadow, ghost counting, the
quality stamp as the only difference (rich centre 0.95 after diffusion,
plain centre 0.38, uniform room stamps the same plain city at 0.95), the
uniform baseline pinned to the parent, and the quality season pinned as
found.

## Next seeds

- The same two foods on the hyperbolic descendant's own ground — does a
  Poincaré city's quality matter more at the rim, where the field is thin?
  (Cartographer)
- Quality priced in the eat rate instead of the stamp — feeders spend the
  rich stock faster per mouthful; does the ordering of the darkening flip
  when the price, not the advertisement, carries the quality? (Cartographer)
- The haunting priced against remembering agents — is the ghost traffic
  the agents' fidelity or the field's? (Cartographer, standing)
