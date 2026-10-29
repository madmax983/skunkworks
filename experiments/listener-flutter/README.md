# Listener Flutter 🦇

**Lineage:** `experiments/vespertilio` (The Night Listener 🦇) by way of
`experiments/listener-choir` — the choir's README closed by naming an
unused sense: *"a bat reads only the first return per window and never
uses amplitude modulation — a moth's flutter is in the cave, but this
generation of bats cannot hear it yet."* This is that sense, grown.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-10-29. Ledger below.

## The Strike

vespertilio's bat and the choir's bats all share one blindness: a
single return from a stone and a single return from a moth are the
same event — an arrival tick and an amplitude. Stones are learned
only by flying to them and finding nothing.

But a moth is not steady while it is being heard. A bat's chirp is
a short sweep, not an instant, and this experiment gives the return
the structure a sweep actually samples: a **glint train** — three
glints, four ticks apart, whose amplitudes sample the reflector's
surface at the strike and at +4 and +8 ticks after it.

- A **stone** presents the same surface at every sample: three
  glints, one amplitude (the path attenuation, and nothing else).
- A **moth** beats its wings with period 12 ticks, so the three
  samples land a third of a wingbeat apart and swing with
  multiplier `0.62 + 0.38·cos(phase + 2π·t/12)`. Three samples a
  third of a cycle apart always span at least half the swing —
  there is no phase at which a moth's train reads flat (proven in
  the tests over every phase and every strike tick in the cycle).

## Two Regimes, One Bat, One Cave, One Seed

3,600 ticks each; 8 moths wandering, 12 stones (the choir's five
plus seven more, so the stone channel is struck often enough for
the ledger to judge a sense whose whole claim is *not flying to
rock*), cave 140×90, c = 2 units/tick, listening window 28 ticks.

- **single-return** — the principal's rule, unchanged: believe the
  first glint the moment it arrives and pursue. A proven stone is
  remembered and not believed again — but the proof costs a flight.
- **flutter** — on the first return, the bat **holds still** and
  listens to the whole train. The third glint lands 8 ticks after
  the first; the moth moves ~1.2 units in that time, and that delay
  is the sense's price, paid on every attended return. The bat then
  classifies by the train's amplitude spread, `(max − min) / mean`:
  above **0.30**, flutter — pursue; at or below, steady — refuse
  the spelled point without spending the flight, and remember it
  as empty.

The perception rule is sacred, as always: the bat reads only
arrival ticks, amplitudes, its own chirp tick and facing, and its
own memory. It never reads a reflector's kind. Whether a refusal
was *right* is scored by the world, never by the bat.

## Novel Trait

No previous specimen in this garden uses the *internal structure*
of a return as evidence. vespertilio proved a return can steer a
hunter; listener-choir proved a shared medium can lie about whose
return it is; listener-flutter asks what a single return, heard
*completely* instead of *quickly*, knows about what made it —
discrimination before locomotion, bought with listening time.

## Prediction and Falsification

Declared from the design, before the first full run:

1. The single-return bat wastes flights on stones (stone visits > 0).
2. The flutter bat refuses stones at range (refusals correct > 0)
   and its stone visits fall below the single-return bat's.
3. A flat train can never cross the flutter line (fooled flutter = 0),
   and a moth's train can never read flat at full length — so any
   wrong refusal would have to come from the world, not the channel.
4. The listening delay does not starve the flutter bat: its catches
   stay within reach of the single-return bat's.

The claim is falsified if flutter bats still fly to stones as often
(the sense is theatre), or if the 8-tick listen costs more moths
than the saved stone flights buy back (the sense is a luxury).

## Headless Ledger

*(Filled from the first full headless run, 2026-10-29; reproduce
with `cargo run -p listener-flutter -- --headless`.)*

```
listener-flutter headless: one bat, one new sense — the glint train
  cave 140x90, 8 moths, 12 stones, c = 2.0 units/tick, window 28 ticks,
  wingbeat 12 ticks, glints 3 x 4 ticks apart, flutter line 0.30
  regime 'single-return': chirps 83, trains heard 66, glints 196,
    pursuits 48, stone pursuits 35, stone visits 10,
    refusals 0, catches 5
  regime 'flutter': chirps 71, trains heard 35, glints 105,
    pursuits 15, stone pursuits 0, stone visits 0,
    refusals 12 (correct 12, wrong 0), fooled flutter 0,
    catches 7
  stone visits — single 10, flutter 0
  catches — single 5, flutter 7
```

All four declared lines held:

1. The principal's rule spent **35 of its 48 pursuits on stone** and
   paid 10 full flights to rock before memory could help it. In a
   stone-heavy cave, the first-return rule is mostly a rock-finding
   rule.
2. The flutter bat refused 12 trains at range, **all 12 stones**,
   and never flew to one: stone pursuits 0, stone visits 0.
3. Fooled flutter **0** and wrong refusals **0** — at full train
   length the channel separates perfectly, exactly as the phase
   proof said it must. The honest limit is below, not here.
4. The 8-tick listen did not starve it — it fed *better*: catches
   7 against 5. The saved stone flights bought more hunting than
   the listening cost, at least in a cave this stony. The flutter
   bat also sang less (71 chirps against 83; holding still to
   listen stretches its cycle) and attended fewer trains (35
   against 66 heard) — it heard less, and knew more of what it
   heard.

## Honest Limits

- The separation is perfect *because the model is clean*: a stone's
  train is exactly flat and a moth's swing is deep (multiplier
  0.24–1.00). A shallower wingbeat, a rippled stone, or noise on
  the amplitudes would put trains on both sides of the 0.30 line,
  and the ledger would have errors to count instead of zeros.
  That noisier cave is the honest next strike.
- Train audibility is judged on the unmodulated base amplitude
  (the choir's EAR rule); a glint that dips low still arrives.
  Partial trains — and the classification errors they would
  cause — are therefore not modelled here.
- One bat, alone. Whether a choir can share the flutter channel
  without jamming it — foreign glints interleaving with a train
  being judged — is untested, and the choir's history says the
  shared air will have opinions.
