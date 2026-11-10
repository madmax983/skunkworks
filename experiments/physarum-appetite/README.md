# physarum-appetite 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/physarum-transit × experiments/physarum-quality (revival cross)
**Coordinates:** graveyard/physarum-transit → experiments/physarum-quality: back
to the flat corner of the map a third time, to the grave that has been raised
twice — once for its missing books, once for its identical foods. Bearing
between them: the second raising priced a food's quality in how loudly it
advertised; this cross moves the same price into how fast it is eaten, and
asks which of the parent's dramatic results were the advertisement's and
which were the food's.

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
plain at 0.40, both eaten at 0.004 — and the result was total: the louder
food drew 5.6× the visits per city, was eaten out first, was haunted
exclusively (43,525 ghost visits, all rich), and the quiet food was never
arrived at: no plain city went dark, because a city nobody visits cannot be
spent. The parent's own closing seed named the unasked question verbatim:
*a stock priced in the eat rate instead of the stamp.*

## The donation

`experiments/physarum-quality` supplies the exact missing organ: **the
two-class map and the class ledger** — five rich cities (the centre and every
second ring city, so neither food owns a corner) and four plain, equal stocks
of 25.0, class-by-class tallies of visits, ghost visits, and darkening ticks
against which a channel can be priced. This cross keeps every one of the
parent's numbers — same map, same seed, same spawn, same stock — and changes
exactly one thing.

## The novel trait

Quality moves from the stamp to the mouth. Every city stamps at the corpse's
own **1.0**; the advertisement is silent. In the rate rooms a rich city is
eaten at **0.010** per agent per tick and a plain city at the parent's
**0.004** — the parent's own 2.5× (1.0 against 0.40), spent in the opposite
organ. The rich food is not louder; it is softer.

Four rooms, one seed, one spawn, one city map (96×64 torus, 1,200 agents,
1,200 ticks, nine hand-surveyed cities):

- **UniformEternal** — the grandparent verbatim: stocks fall in a *shadow
  ledger* but never gate the stamp. The baseline this crate must reproduce
  exact.
- **UniformDepleted** — the grandparent verbatim: the stock gates the stamp.
- **RateEternal** — eat-rate quality, stocks in shadow only: with no
  advertisement difference, do the two foods draw the same mouths?
- **RateDepleted** — eat-rate quality, stock gating: which food ends first
  when the only difference is how fast it is eaten, and who is haunted?

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
- **P1:** in RateEternal the two foods draw alike: the per-city mean visit
  ratio (rich/plain) stands within [0.70, 1.43] — no advertisement, no
  5.6× reassignment.
- **P2:** in RateDepleted the rich cities go dark earlier on average than
  the plain cities — the softer food is spent first, by the mouth alone.
- **P3:** every city still goes dark in RateDepleted before tick 1,200 —
  the plain are visited normally, so unlike the stamp rooms they *can* be
  spent.
- **P4:** ghost visits in RateDepleted do not concentrate the way they did
  under the stamp: the plain ghost total is above zero and the rich share
  of all ghost visits is below 90% — roads are kept to food that was
  actually visited.
- **P5:** RateDepleted ends poorer than UniformDepleted in final trail
  mass — a faster-burning half of the map spends the field's season
  sooner.

**Falsification condition:** if RateEternal's per-city visit ratio lands
anywhere near the stamp rooms' 5.6×, or RateDepleted's two classes darken
in the opposite order to P2, the eat-rate channel is not the quiet channel
this cross claims, and the ledger below is the instrument that would show
it.

## The ledger (headless, as found)

**B1 HELD, exact.** The uniform rooms reproduce the grandparent's pinned
ledger field for field: visits 217,975 in UniformEternal; first dark tick
**219** in both uniform rooms; Eternal final mass **2,447.38**, coverage
**3,685**; Depleted final mass **1,713.38**, coverage **2,529**, ghost
visits in the last 300 ticks **20,387**, all nine cities dark.

**P1 HELD, in its strongest form.** RateEternal's ledger is not merely
within the declared band — it is the uniform ledger, visit for visit:
rich **117,785**, plain **100,190**, per-city means 23,557 against 25,048,
a ratio of **0.94**. Final mass and coverage are the uniform eternal room's
own (2,447.38 / 3,685). An eat rate that gates nothing is *invisible*:
while nothing can run out, the mouth's price leaves no trace on where the
network goes. The only place it shows is the shadow ledger, where the rich
cities' would-be dark ticks fall at a mean of **118.4** against the plain
cities' 346.0 — the bill is written long before it is presented.

**P2 HELD.** In RateDepleted the rich cities go dark at a mean first-dark
tick of **117.8** (94–154) against the plain cities' **311.8** (213–396) —
the softer food is spent first, by the mouth alone, with no advertisement
to blame. First dark tick of the season: **94**, against the uniform rooms'
219.

**P3 HELD — the contrast with the stamp rooms is the finding.** All nine
cities go dark (dark ticks **8,964** against the uniform room's 7,547).
Under the stamp, the plain cities could not be spent because they were
never arrived at; under the mouth, they are visited normally — plain visits
**61,963**, the majority of the room's 109,225 — and so they end like
everything else. Being ignored was the only immortality on offer in this
lineage, and the eat rate does not offer it.

**P4 HELD, past its line.** Ghost visits total **71,624** — more haunting
than the uniform room's 52,206, because the food ends sooner — and they are
*shared*: rich **34,705**, plain **36,919**. The rich share is 48.5%, not
the stamp rooms' 100%. A network haunts what it ate, in proportion to
having eaten it; it was the advertisement, not the appetite, that made the
haunting exclusive. Ghost visits in the last 300 ticks: **28,702** (uniform:
20,387).

**P5 CRACKED, by a hair — and the hair is pinned.** RateDepleted ends at
final mass **1,720.61** / coverage **2,544**, a fraction *above*
UniformDepleted's 1,713.38 / 2,529, not below — despite 1,417 more dark
ticks. The declaration said a faster-burning map spends the season sooner;
the ledger says the season's mass is set by the roads, not the burn: once
every city is dark either way, the field decays on the corpse's clock from
much the same web, and the faster burn's only lasting signature is in the
dark-tick and ghost columns, not in the mass.

**The law, sixteenth watch:** price a quality in the advertisement and the
network reassigns itself — preference, exhaustion order, and an exclusive
haunting all follow the stamp. Price the same quality in the mouth and the
network changes nothing it does, only how soon it is done: the visits are
the uniform visits, the haunting is shared in proportion to the eating,
and every food, courted or not, is spent. *What a thing says decides where
the roads go; what a thing costs decides only when they empty.*

## Next seeds

- The same two foods on the hyperbolic descendant's own ground — the
  Poincaré disk the certificate once named as this lineage's fitter form:
  does curvature tax the stamp channel, the mouth channel, or both?
- Both channels at once, opposed: a food loud in the stamp and slow in the
  mouth against a food quiet and quick — which channel sets the
  exhaustion order when they disagree?
- The haunting priced against remembering agents still stands.
