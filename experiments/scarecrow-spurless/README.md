# Scarecrow Spurless 🌾

**Host:** `experiments/scarecrow-whole-kicks` (The Scarecrow 🌾)
and its hosts in turn, `experiments/scarecrow-two-roads` and
`experiments/scarecrow-sheaf` — the champion follower on the
parent crate `crates/neuro-sim` fed by two roads of opposite
sign: road A, Regular Spiking (I=12), weight **-1.0**, delay
10, landings at spike + 11; road B, Regular Spiking (I=14),
weight **+0.75**, delay 6, landings at spike + 7; the
follower Fast Spiking (I=4.8), counted over 24,000 ticks by
the Widow's instrument. The whole-kicks planting coarsened
the journal on the weight axis and left a ladder standing:
road B written +1.0 read 75/112, the truth's +0.75 read
83/112, +0.5 read 91/112 — the smaller the remembered spur,
the better the season scored. This planting takes up that
run's closing seed verbatim: *the spur dropped entirely —
road B's ticks kept, its weight written 0.0, the limiting
case of the shrunk direction: +1.0 read 75, +0.5 read 91,
the truth's +0.75 read 83 — where does 0.0 land, and is the
true weight even the best weight on the truth's own
columns?*

**Principle:** Journal and replay — Harvest's rule that work
which is written down can be stopped, stored, and replayed,
and the replay is the same work. Learned from Harvest's
durable execution: the journal is the run; everything else
is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-10. The comparison and predictions
below were declared before the first run and were not edited
after it. Verdicts are at the bottom, as found.

## The field, surveyed

I stood in the whole-kicks row and read its ladder again.
Three rungs, and the score climbs as the spur shrinks. A
ladder like that has a bottom rung, and the bottom rung is
not a smaller number. It is no spur at all.

There is a plainer way to say what the bottom rung is. A
journal with road B written 0.0 delivers road A's kicks and
nothing else. That is not an approximation of some other
run. That is the single-road baseline's own coupling,
spelled as a journal — the sheaf's pair with the second
road never built. So this planting does not only ask where
0.0 lands on the ladder. It asks whether the replayed
silence *is* the baseline animal, spike for spike, or
merely resembles it through the instrument's summary. The
whole row stands on replay being the same work. Here the
replay has a living twin to be compared against, not only
a summary to match.

And one more line in the book: the same season with road
B's entries dropped outright — no line at all where the
silenced book writes 0.0. If the two replay to the same
train, then a written nothing and an absent line are the
same thing to the ear, and they differ only in paper: the
silenced book keeps all 1,309 lines; the dropped book
keeps 607.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all 24,000
ticks, counted by the Widow's instrument against driver A's
post-settle train, and compared spike-for-spike with the
live train:

- **Live** — the two-road field, whole, writing the signed
  journal as it steps, exactly as in `scarecrow-two-roads`.
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, its follower train
  kept whole for spike-for-spike comparison.
- **Replay (signed)** — the journal as written: the control.
- **Replay (whole)** — the ladder's top rung: road B
  written +1.0 (error budget +175.5). An anchor.
- **Replay (shrunk)** — the ladder's third rung: road B
  written +0.5 (error budget -175.5). An anchor.
- **Replay (silenced)** — the seed's spelling: road B's
  ticks kept, its weight written 0.0 (error budget -526.5,
  three times the shrunk journal's, all spent downward).
- **Replay (dropped)** — road B's entries removed from the
  book entirely: 607 entries, road A's book alone.

Paper is counted as in two-roads and whole-kicks: a signed
entry is 3 bytes (u16 tick + i8 weight in quarter units).
A silenced entry is counted at the same 3 bytes — writing
0.0 still takes a line. A dropped entry takes no line.

**Predictions (declared before the first run):**

- **B1 (host check):** the single-road baseline reproduces
  the sheaf's centre exact — perfect 92/112, count slips 3,
  phase slips 17, longest 58, mean episode 18.4 (±0.05) —
  and the live two-roads season reproduces two-roads'
  published ledger exact (perfect 83/112, follower spikes
  857, signed entries 1,309, coincidences 24).
- **B2 (ladder anchors):** the whole and shrunk replays
  reproduce whole-kicks exact on this same live season —
  whole parts at tick 64 and reads perfect 75; shrunk parts
  at tick 91, reads perfect 91, and counts 855 follower
  spikes. The ladder's upper rungs are pinned before its
  bottom rung is stepped on.
- **P1:** the signed journal replays the follower *exact*
  — spike train identical on all 24,000 ticks. The control
  must hold before any silent spelling is judged.
- **P2:** the silenced journal diverges, and its first
  divergence falls at or after the first road-B landing
  (tick 10) — it cannot be wrong before the first tick
  where its spelling differs from the truth.
- **P3:** the silenced and dropped journals replay to the
  *same* follower train, spike for spike. A 0.0 entry is
  no entry, as far as the ear is concerned; the two books
  differ only in paper.
- **P4 (the headline):** the silenced season *is* the
  baseline — its follower train identical to the live
  single-road pair's train on all 24,000 ticks, and its
  cell identical to the baseline cell field for field
  (92/112, slips 3 and 17, longest 58, mean 18.4). A
  silenced journal is not a sketch of the one-road animal.
  It is the animal, written down.
- **P5:** the ladder is monotone on the perfect column
  across all four spellings: whole (75) < truth (83) <
  shrunk (91) < silenced — each smaller remembered spur
  scores strictly higher than the last.
- **P6:** the true weight is not the best weight on the
  truth's own columns: the silenced season beats or
  matches the live season on every headline column
  (perfect, longest, count slips, phase slips) and beats
  it strictly on perfect, longest, and count slips.
- **P7:** writing nothing costs full paper: the silenced
  journal counts the same 3,927 bytes as the signed
  journal; only the dropped journal is smaller (607 × 3
  = 1,821 bytes). Silence is not a saving. Absence is.
- **P8:** the silenced journal parts no later than the
  shrunk journal's tender tick (91) — its per-entry error
  is three times the shrunk journal's in the same
  direction, so the follower's tenderness should be
  reached no later.

**Falsification condition:** if P1 fails, something besides
the kicks crosses these roads and the whole row's law falls
with it. If P4 fails — if the silenced replay parts from
the live baseline's train at any tick — then a replay is
not the same work as the run it spells, even when the
spelling is a subset of a proven journal, and the row's
replay-exactness was a property of two roads, not of the
machinery. If P3 fails, an entry's presence carries
something beyond its weight, and no journal in this row
means what it says. If P5 fails at the bottom rung — if
0.0 scores *below* +0.5 — the ladder was two points and a
story, and the climb stops where the spur does.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **B1 HELD, exact.** The baseline reproduces the sheaf's
  centre field for field (92/112, slips 3 and 17, longest
  58, mean 18.4), and the live season reproduces two-roads'
  ledger exact: perfect 83/112, follower spikes 857, signed
  entries 1,309, coincidences 24 (first at 464). Both hosts
  were pinned before the silent spellings spoke.
- **B2 HELD, exact.** The ladder's upper rungs reproduce
  whole-kicks on this same live season: the whole journal
  parts at tick 64 and reads 75/112; the shrunk journal
  parts at tick 91, reads 91/112, and counts 855 follower
  spikes. The ladder was measured, not remembered, before
  its bottom rung was stepped on.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD.** The silenced journal parts at tick **65**,
  at or after the first road-B landing at 10, as declared.
  One tick after the family's tender tick: the whole and
  unsigned spellings part at 64 on a quarter-kick spent up
  and a sign forgotten; the silenced book, spending three
  times the shrunk budget downward, parts at 65. The
  tender map is the follower's, but it is not one tick
  for every error.
- **P3 HELD, exact.** The silenced and dropped journals
  replay to the same follower train, spike for spike. A
  0.0 entry is no entry, as far as the ear is concerned.
- **P4 HELD — the headline, in its strongest form.** The
  silenced season *is* the baseline: its follower train is
  identical to the live single-road pair's train on all
  24,000 ticks (850 spikes, first divergence: none), and
  its cell is the baseline cell field for field — perfect
  92/112, count slips 3, phase slips 17, episodes 5,
  longest 58, mean episode 18.4. The dropped book reads
  the same cell, because it is the same train. A journal
  that silences a road does not approximate the field
  without that road. It writes that field, and the replay
  is that animal.
- **P5 HELD.** The ladder is monotone on the perfect
  column across all four spellings: whole (+1.0) **75** <
  truth (+0.75) **83** < shrunk (+0.5) **91** < silenced
  (0.0) **92**. Every smaller remembered spur scores
  strictly higher than the last, down to no spur at all.
- **P6 HELD.** The silenced season beats the truth on
  perfect (92 against 83), longest (58 against 16), and
  count slips (3 against 12), and matches it on phase
  slips (17 against 17). The true weight is not the best
  weight on the truth's own columns; on this instrument
  it is beaten by every honest misremembering below it
  and by silence most of all.
- **P7 HELD.** The silenced journal counts **3,927 bytes**
  — the signed journal's full paper, for a book that says
  nothing 702 times. The dropped journal counts **1,821
  bytes** (607 entries × 3). Silence is not a saving.
  Absence is: dropping the lines buys 2,106 bytes and
  loses nothing the ear can hear.
- **P8 HELD.** The silenced journal parts at 65, no later
  than the shrunk journal's tender tick 91, as declared —
  three times the per-entry error in the same direction
  reached the follower's tenderness sooner, though not
  at the family's famous 64.

**The law, fourteenth under the Scarecrow's name:** a
journal that silences a road has not approximated the
field without the road — it has written it. Replayed, the
silenced book is the one-road animal spike for spike,
because the journal is the coupling and a 0.0 entry is no
entry; the only thing silence costs is paper, the full
3,927 bytes, and the only saving in the row is absence.
And the ladder's bottom rung prices the instrument, not
the road: the Widow's columns reward the pair they were
tuned on, so every smaller remembered spur outscored the
truth on the truth's own ground (75, 83, 91, 92) — a
score is a property of the counter as much as of the
counted, and a journal can be wrong about a weight in
the direction the counter loves and be crowned for it.

**Next seeds:** the windowed silence — road B silenced
only inside one chosen window (say ticks 1,000–10,000),
exact before it and after it: the season parts inside
the window, but when the spur returns at the window's
far edge, does the train rejoin the truth, or does the
parting outlive the silence that caused it? And the
tick-64 question in its window form still stands, from
whole-kicks: a journal coarse only inside one chosen
window — does any parting wait at the window's edge, or
only at a tender tick?
