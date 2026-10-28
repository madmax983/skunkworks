# Listener Choir 🦇

**Lineage:** `experiments/vespertilio` (The Night Listener 🦇) — one bat
hunting moths by true echolocation in a wave cave. This is its
succession, the one its own ledger asked for: *multiple bats*.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-10-28. Ledger below.

## The Strike

vespertilio's bat hunted alone, and its cave was honest: every return
inside its listening window was an echo of its own chirp, so the rule
*range = delay · c/2 along my facing at my chirp* told the truth.

Put three bats in the same air and the rule itself becomes the liar.
Sound is sound:

- Each chirp is directional (cone ±0.55 rad, max range 36 units) and
  strikes every reflector — moth or stone — inside it.
- The struck reflector sends a wavefront out in **all** directions.
  It reaches bat B at `t + (|E→R| + |R→B|) / c` — emitter, reflector,
  listener, a path computed from the strike geometry, attenuated
  over its whole length. Every bat in earshot hears it. The medium
  does not sign its work, and no arrival carries a name tag a bat
  could read.
- A neighbour's outgoing chirp also passes overhead: loud, and it
  **masks** the receiver for 5 ticks — returns landing just behind
  it are simply not heard.
- A bat records whatever lands inside its own listening window
  (28 ticks after its own chirp) and believes the **first** return
  *the moment it arrives*: nearest reflector, by the oldest rule in
  the cave, and the freshest fix physics allows. (An earlier design
  decided at window close; the ledger showed every bat chasing
  where moths *had been* — a fix up to 28 ticks stale — and it was
  scrapped.) For a foreign echo, the believed point — *my* chirp
  position plus spelled range along *my* facing — is a place where
  nothing ever was. The bat pursues it exactly as faithfully as
  the truth.
- With a live target, a bat chirps faster (period 30 against the
  scanning 44) — the feeding buzz, real bat behaviour, kept honest
  here because 30 still exceeds the window plus its closing tick.

Stones return echoes too, indistinguishable from moths in a single
return. A pursuit that ends at a stone is counted, honestly, as a
stone visit — and the bat *remembers* that point: a later return
spelling a place within 2 units of a proven stone is not believed,
and the window stays open for the next return. (Only stones are
remembered. A moth that fled may wander back; a stone is a stone
forever.)

## Three Regimes, One Cave, One Seed

3,600 ticks each; 8 moths wandering, 5 stones, cave 140×90,
c = 2 units/tick, scanning chirp period 44 ticks.

- **solo** — one bat, fixed tempo 44 ticks. The honest baseline.
- **choir, naive** — three bats, identical fixed tempo, first chirp
  in unison. Maximum jamming, forever.
- **choir, polite** — three bats, same unison start, but a bat whose
  window was *jammed* (it recorded a foreign return, or a return was
  masked) delays its next chirp once by 7 ticks — a temporal
  jamming-avoidance response. Its tempo never changes; only its
  phase moves. (An earlier design widened the tempo permanently;
  the ledger showed the polite bats simply singing less and eating
  worse — 2.67 per bat against the naive 4.33 — and it was scrapped.
  Delay, don't slow.) Nobody tells a bat to delay; the shift is
  earned only through arrivals at its own ears, and each bat's jam
  history differs, so the unison breaks apart over the night.

## Novel Trait

No previous specimen in this garden puts multiple echolocators in
one medium and counts what the sharing costs. vespertilio proved a
return signal can steer a hunter; listener-choir asks what happens
to that signal when it is *everyone's* — misattribution as a
physical consequence of arrival times, not a scripted confusion,
and a desynchronisation that is learned from jamming rather than
assigned.

## Prediction and Falsification

Declared before the first full run:

1. The naive choir hears foreign returns (strictly > 0) and believes
   some of them (false pursuits > 0).
2. Jamming **costs**: naive catches per bat < solo catches per bat.
3. Politeness **recovers**: polite foreign returns < naive foreign
   returns, and polite catches per bat > naive catches per bat.

The claim is falsified if a synchronized choir hunts as well per
bat as a solo bat (the shared air costs nothing), or if chirp
delays fail to thin the foreign returns (the avoidance response is
theatre).

## Headless Ledger

*(Filled from the first full headless run, 2026-10-28; reproduce
with `cargo run -p listener-choir -- --headless`.)*

```
listener-choir headless: one cave, three ways of singing in it
  cave 140x90, 8 moths, 5 stones, c = 2.0 units/tick, window 28 ticks
  regime 'solo': chirps 83, own returns 38, foreign 0, masked 0,
    pursuits 30, false 0, stone visits 2, catches 10 (per bat 10.00)
  regime 'choir, naive': chirps 253, own returns 65, foreign 70,
    masked 71, pursuits 100, false 54, stone visits 6,
    catches 17 (per bat 5.67; per-bat [6, 3, 8])
  regime 'choir, polite': chirps 219, own returns 52, foreign 45,
    masked 76, pursuits 75, false 32, stone visits 6,
    chirp delays 192 (per bat [64, 63, 65]),
    catches 20 (per bat 6.67; per-bat [4, 9, 7])
  catches per bat — solo 10.00, naive 5.67, polite 6.67
  foreign returns — naive 70, polite 45
```

All three declared lines held:

1. The naive choir heard 70 foreign returns and believed 54 of its
   100 pursuits from them — more than half its hunting was spent
   chasing other bats' echoes.
2. Jamming **cost**: 5.67 catches per bat against the solo 10.00 —
   the shared air ate ~43% of the hunt, before counting the 71
   returns that were masked outright and never heard at all.
3. Politeness **recovered, partially**: 192 earned delays thinned
   foreign returns 70 → 45 and false pursuits 54 → 32, and catches
   per bat rose 5.67 → 6.67 — real, and honestly short of the solo
   10.00. A choir that shares the air politely still shares the
   air. The delays also cost song (219 chirps against 253), which
   is part of why the recovery stops where it stops.

## Honest Limits

- Reflector strike geometry uses positions at the chirp tick; a bat
  moves ≲1 unit while its echo is in flight, and the arrival tick
  does not chase it mid-flight.
- A bat reads only the first return per window and never uses
  amplitude modulation — a moth's flutter is in the cave, but this
  generation of bats cannot hear it yet. That unused channel is the
  obvious next sense.
- Catches are counted by the world (physical contact within 2.8
  units), never by the bat. A bat that arrives where an echo lied
  has no way to know it was lied to; it only knows the larder is
  not there.
