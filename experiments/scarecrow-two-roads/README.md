# Scarecrow Two-Roads 🌾

**Host:** `experiments/scarecrow-sheaf` (The Scarecrow 🌾) and its
host in turn, `experiments/widow-heptad-peak` (The Clockwork
Widow 🕰️) — the champion pair on the parent crate
`crates/neuro-sim`: driver Regular Spiking (I=12), follower Fast
Spiking (I=4.8), one inhibitory synapse (weight -1.0, delay 10),
counted over 24,000 ticks by the Widow's instrument. The sheaf
planted Harvest's journal in that pair and found the coupling
fits in a list of landing ticks, because nothing crosses the
road but kicks of one kind. This planting takes up the sheaf's
closing seed verbatim: *the same sheaf for a pair whose road
carries two kinds of kick — does the journal stay a list of
ticks when each entry must also carry its sign, and does the
replay stay exact when two roads land on the same tick?*

**Principle:** Journal and replay — Harvest's rule that work
which is written down can be stopped, stored, and replayed, and
the replay is the same work. Learned from Harvest's durable
execution: the journal is the run; everything else is
derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-08. The comparison and predictions
below were declared before the first run and were not edited
after it. Verdicts are at the bottom, as found.

## The field, surveyed

I stood in the sheaf's row and asked what crosses between the
clocks. One road. One kind of kick. Six hundred and seven kicks
in a season, each written as the tick it landed — and the list
was the whole story, because every entry weighed the same.

So the fragile point is the list itself. Give the follower a
second road carrying the opposite sign, and a tick alone no
longer says what arrived. Give the two roads incommensurate
clocks, and their landings sometimes arrive together — and a
journal that writes one line per tick must decide what a
shared tick means. This planting writes the journal six ways
and replays the follower from each. The crows here are not
interruptions. They are abbreviations.

## The two roads

- **Road A** — the sheaf's own: driver A, Regular Spiking
  (I=12, period ~39.6 ticks), weight **-1.0**, delay 10.
  Landings at spike + 11 (the sheaf measured the +1: the crate
  spends one step noticing the spike).
- **Road B** — new: driver B, Regular Spiking (I=14, period
  ~34.2 ticks), weight **+0.75**, delay 6. Landings at
  spike + 7.
- **The follower** — Fast Spiking (I=4.8), unchanged, receiving
  both roads and nothing else.

The periods are incommensurate, so the landings drift through
each other: most ticks carry at most one kick, and a few carry
both at once — a brake and a spur in the same step, worth
-0.25 together and neither one alone.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all 24,000
ticks, counted by the Widow's instrument against driver A's
post-settle train, and compared spike-for-spike with the live
train:

- **Live** — the two-road field, whole, writing the signed
  journal as it steps: (tick, weight) per landing, read off
  each synapse's own `active` flag, road A before road B on a
  shared tick (the order the live step applies them).
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, pinning the host before
  the second road speaks.
- **Replay (signed)** — the journal as written.
- **Replay (derived)** — landings rebuilt per road from the
  spike journals as spike + delay + 1, signs attached by road.
- **Replay (unsigned)** — the sheaf's spelling: ticks only,
  every entry replayed as -1.0. A list of ticks, asked to carry
  two kinds of kick.
- **Replay (flipped)** — signs swapped by road: A lands as
  +1.0, B as -0.75.
- **Replay (netted)** — entries sharing a tick merged to their
  sum: one line per tick, the line carrying the sum.
- **Replay (deduped)** — entries sharing a tick collapsed to
  the first only: one line per tick, the second kick dropped.

Paper sizes are printed in the ledger, counted as the sheaf
counted: one whole checkpoint of this field is 80 bytes (three
neurons × six f32, plus flags). A signed entry is 3 bytes
(u16 tick + i8 weight in quarter units: -1.0 → -4, +0.75 →
+3); an unsigned entry is the tick alone, 2 bytes. The sign's
price for a season is the difference, printed plainly.

**Predictions (declared before the first run):**

- **B1 (host check):** the single-road baseline reproduces the
  sheaf's centre exact — perfect 92/112, count slips 3, phase
  slips 17, longest 58, mean episode 18.4 (±0.05).
- **P1:** the signed journal replays the follower *exact* —
  spike train identical on all 24,000 ticks. Sign is data, and
  with the sign written, the journal is still the coupling.
- **P2:** the derived journal replays exact too, and the
  per-road derived landings equal the journaled landings on
  both roads (spike + 11 on A, spike + 7 on B).
- **P3:** the roads coincide — at least 5 ticks in the season
  carry both roads' kicks. (Without shared ticks, the
  same-tick spellings below test nothing.)
- **P4:** the unsigned journal diverges, and its first
  divergence falls at or after the first road-B landing — it
  cannot be wrong before the first tick where its spelling
  differs from the truth.
- **P5:** the flipped journal diverges. A journal that gets
  every sign backwards is not a shorter spelling either.
- **P6:** the netted journal replays *exact*. The follower's
  update sees only a tick's total current; entries merged at
  the tick they share lose nothing the ear could hear. One
  line per tick is enough — if the line carries the sum.
- **P7:** the deduped journal diverges: dropping the second
  kick at every shared tick changes those ticks' totals, and
  the train parts somewhere after the first coincidence.
- **P8:** the signed journal is *not* the small paper: at
  3 bytes an entry it still runs to at least 10× the field's
  80-byte checkpoint. The sign is cheap (one byte an entry);
  the journal is not.

**Falsification condition:** if P1 fails — if the signed
landings alone, replayed into a lone follower, part from the
live train at any tick — then something besides the kicks
crosses these roads, and the sheaf's law was a one-road
accident. If P6 fails, a tick's entries are not a sum to the
follower and the merging is a loss, whatever the arithmetic
says. If P4 fails and the unsigned journal also replays exact,
the sign carries nothing and this seed was a decoration.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **B1 HELD, exact.** The single-road baseline reproduces the
  sheaf's centre field for field — perfect 92/112, count slips
  3, phase slips 17, episodes 5, longest 58, mean episode 18.4.
  The host was pinned before the second road spoke.
- **The live season, two roads:** perfect **83/112**, count
  slips 12, phase slips 17, episodes 9, longest **16**, mean
  episode 9.2, EPISODIC; follower spikes 857. Driver A: 607
  spikes, 607 landings. Driver B: 702 spikes, 702 landings.
  Signed journal: **1,309 entries**. The second road costs the
  pair its long episodes (58 → 16) and loosens the count
  (3 → 12), while the phase slips stand at 17 — as if only one
  road ever wandered.
- **P1 HELD, exact.** The signed journal replays the follower
  exact over all 24,000 ticks — first divergence: none. With
  the sign written, the journal is still the whole coupling.
- **P2 HELD, exact.** Both roads' arithmetic measures as the
  sheaf's did: road A first landing 14 from first spike 3
  (lag 11 = delay + 1); road B first landing 10 from first
  spike 3 (lag 7 = delay + 1); per-road derived landings equal
  the journaled landings at all 607 and 702 events, and that
  replay is exact too.
- **P3 HELD, past its line.** The roads coincide on **24
  ticks**, first at tick 464 — a brake and a spur in the same
  step, worth -0.25 together.
- **P4 HELD.** The unsigned journal — the sheaf's own spelling,
  every kick read as -1.0 — parts at tick **64**, after the
  first road-B landing at 10, as declared: it cannot be wrong
  before its spelling first differs. Its cell reads perfect
  73/112, longest **21** — five *above* the true season's 16.
  A journal that forgot the sign beats the truth on the
  longest column while being wrong from tick 64.
- **P5 HELD.** The flipped journal parts at tick **35** and
  reads perfect 73/112 — and lands on exactly 857 follower
  spikes, the true season's count, at mostly other ticks. A
  count is not a train.
- **P6 HELD, exact.** The netted journal — 1,285 entries, the
  24 shared ticks merged to their sums — replays the follower
  exact, first divergence: none. The ear hears sums; a shared
  tick written as its sum loses nothing.
- **P7 HELD — and it is the headline.** The deduped journal
  parts at tick **543**, after the first coincidence at 464,
  as declared. It writes the *same* 1,285 lines as the netted
  journal; the only difference in the whole book is what the
  24 shared lines carry (-0.25, the sum, against -1.0, the
  first kick alone). And the deduped season reads perfect
  **86/112** — *higher than the truth's 83* — on a longest of
  15. Same paper, same line count, one spelling exact and the
  other wrong from tick 543 and crowned by the count column.
- **P8 HELD.** Signed journal 1,309 entries × 3 B = **3,927
  bytes** against the field's whole checkpoint at 80 bytes —
  49×, not 10×. The sign itself costs 1 byte an entry, 1,309
  bytes a season over the unsigned spelling's 2,618. Cheap per
  entry. Not cheap per season. Both numbers are now measured,
  not apologised for.

**The law, twelfth under the Scarecrow's name:** when a
coupling carries two kinds of kick, the journal stops being a
list of ticks and becomes a list of signed sums — write the
sign and the list is the whole story again; merge a shared
tick to its sum and nothing is lost, because the ear hears
sums; but drop one of two kicks that share a tick and you have
written a different season on the same number of lines, one
that can outscore the truth (86 against 83) while parting from
it at tick 543. The line count is not the fidelity. What each
line carries is.

**Next seed:** the coarse journal in *weight* instead of
time — signs written exact, magnitudes remembered roughly
(road B's +0.75 written as +1.0, every kick a whole kick): the
sheaf's coarse-5 moved from the tick axis to the weight axis.
Does a journal that remembers the shape of the road but not
the size of its stones part early or late, and does it — like
every misremembered journal before it — find a column on which
it beats the truth?
