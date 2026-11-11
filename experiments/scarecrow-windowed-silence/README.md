# Scarecrow Windowed-Silence 🌾

**Host:** `experiments/scarecrow-spurless` (The Scarecrow 🌾)
and its hosts in turn — `scarecrow-whole-kicks`,
`scarecrow-two-roads`, and `scarecrow-sheaf` — the champion
pair on the parent crate `crates/neuro-sim`: driver A, Regular
Spiking (I=12), and driver B, Regular Spiking (I=14), feeding
one Fast Spiking follower (I=4.8) over two roads of opposite
sign (A: weight −1.0, delay 10, landings at spike + 11; B:
weight +0.75, delay 6, landings at spike + 7), counted over
24,000 ticks by the Widow's instrument. Spurless silenced road
B for the whole season and found the replayed silenced book
*was* the single-road baseline, spike for spike. This planting
takes up spurless' closing seed verbatim: *the windowed
silence — road B silenced only inside one chosen window (say
ticks 1,000–10,000), exact before it and after it: the season
parts inside the window, but when the spur returns at the
window's far edge, does the train rejoin the truth, or does
the parting outlive the silence that caused it?*

**Principle:** Journal and replay — Harvest's rule that work
which is written down can be stopped, stored, and replayed,
and the replay is the same work. Learned from Harvest's
durable execution: the journal is the run; everything else
is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-11. The comparison and predictions
below were declared before the first run and were not edited
after it. Verdicts are at the bottom, as found.

## The field, surveyed

I stood in spurless' row. The whole season stood silenced
there, and the silenced book was the baseline animal entire.
But a season is not always silent. A road goes quiet for a
while — a winter inside the year — and then it speaks again.

The fragile point is the window's far edge. Before the window
the windowed book is the truth, word for word. Inside it the
follower eats the baseline's diet in the truth's body: it
enters the silence carrying the truth's state, not the
baseline's, so inside the window it is a third animal, neither
the truth nor the one-road pair. At tick 10,000 the spur
returns, exact as it ever was. The question is whether exact
input is enough — whether the train walks back to the truth
and locks step with it, or whether nine thousand ticks of
silence leave a parting that the returning road cannot close.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all 24,000
ticks, counted by the Widow's instrument against driver A's
post-settle train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first disagreed
tick, the last disagreed tick, and the count of disagreed
ticks in each region — before the window [0, 1,000), inside
it [1,000, 10,000), after it [10,000, 24,000), and in the
tails at/after 15,000, 18,000, and 20,000. The **rejoin tick**
is the first tick at or after 10,000 from which the two
trains agree to the season's end; if disagreements run to
the last tick it reads 24,000 and no rejoin was observed.

- **Live** — the two-road field, whole, writing the signed
  journal as it steps: (tick, weight) per landing.
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, pinning the host.
- **Replay (signed)** — the journal as written. The control.
- **Replay (silenced, full season)** — road B written 0.0
  everywhere: spurless' anchor, re-measured in this crate.
- **Replay (windowed-silenced)** — road B written 0.0 only
  on ticks in [1,000, 10,000); every line kept, exact
  outside the window. The seed's spelling.
- **Replay (windowed-dropped)** — road B's entries inside
  the window removed outright; exact outside it.

Paper is counted as the row counts it: a signed entry is
3 bytes (u16 tick + i8 weight in quarter units), and a whole
checkpoint of this field is 80 bytes. A silenced line still
takes its 3 bytes. A dropped line takes none.

**Predictions (declared before the first run):**

- **B1 (host check):** the single-road baseline reproduces
  the sheaf's centre exact — perfect 92/112, count slips 3,
  phase slips 17, longest 58, mean episode 18.4 (±0.05).
- **B2 (host check):** the live two-road season reproduces
  the row's ledger exact — perfect 83/112, count slips 12,
  phase slips 17, episodes 9, longest 16, mean episode 9.2;
  follower spikes 857; signed entries 1,309; coincidences
  24, first at tick 464.
- **B3 (anchor check):** the full-season silenced journal
  replays to the baseline train exact and parts from the
  live train at tick 65, as spurless found.
- **P1:** the signed journal replays the follower *exact* —
  spike train identical on all 24,000 ticks.
- **P2:** the windowed journal is exact before the window:
  no disagreed tick stands before tick 1,000. A spelling
  that differs only inside the window cannot be wrong
  before the window opens.
- **P3:** the windowed journal parts *inside* the window:
  its first divergence falls at or after the first road-B
  landing inside the window and before tick 10,000.
- **P4 (the seed's question):** the train rejoins the
  truth. The rejoin tick is at or before 18,000 and no
  disagreed tick stands at or after 18,000. Grounds: a
  spike resets the follower's voltage outright; two
  followers fed the same exact kicks from 10,000 on should
  be pulled onto the same spikes and lock step well before
  the season's last quarter.
- **P5:** the windowed-dropped journal replays to the
  windowed-silenced train *exact* — spurless' law in window
  form: a 0.0 entry is no entry, inside a window as anywhere.
- **P6:** the windowed animal is *not* the baseline animal
  inside the window: at least one tick inside the window
  disagrees with the baseline train. Same diet, different
  body — it entered the silence in the truth's state.
- **P7:** writing nothing inside a window costs full paper:
  the windowed-silenced book counts the signed book's 3,927
  bytes; the windowed-dropped book saves exactly 3 bytes
  per silenced landing, and the landings inside the window
  number between 200 and 320 (702 landings spread over
  24,000 ticks put about 263 inside a 9,000-tick window).
- **P8:** the windowed season outscores the truth in the
  direction of every silence before it: its perfect count
  is above the live season's 83 and its longest episode
  above the live season's 16.

**Falsification condition:** if P2 fails — if a journal
exact before the window parts from the truth before the
window opens — then the replay is not the same work as the
run and the row's machinery falls with it. If P4 fails, a
parting outlives its cause: nine thousand ticks of silence
are not forgiven by the spur's return, and a journal that
drops a season's middle does not heal at the seam. If P5
fails, an entry's presence carries something beyond its
weight inside a window, and spurless' law was a whole-season
accident.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **B1 HELD, exact.** The baseline reproduces the sheaf's
  centre field for field (92/112, slips 3 and 17, longest
  58, mean 18.4), and pins the host before the window opens.
- **B2 HELD, exact.** The live season reproduces the row's
  ledger entire: perfect 83/112, slips 12 and 17, episodes
  9, longest 16, mean 9.2; follower spikes 857; signed
  entries 1,309; coincidences 24, first at 464.
- **B3 HELD, exact.** The full-season silenced anchor is
  the baseline train spike for spike and parts from the
  live train at tick 65, as spurless found. The anchors
  were measured, not remembered.
- **The window, counted:** road B lands 30 times before
  tick 1,000, **262** times inside the window (first at
  1015), and 410 times at/after 10,000.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD, exact.** No disagreed tick stands before
  1,000. A spelling exact before the window is exact
  before the window; the replay machinery is honest at
  the seam's near edge.
- **P3 HELD.** The windowed train parts at tick **1047** —
  inside the window, 32 ticks after the first silenced
  landing at 1015. (Not at the family's tender tick 64's
  kin, and not at the window's edge at 1,000: the parting
  waited for the follower's own tenderness inside the
  silence.)
- **P4 CRACKED — the headline.** The train does **not**
  rejoin the truth. Disagreed ticks: **617** inside the
  window, **979** after it closes — more parting after
  the silence than during it — 625 at/after 15,000, 410
  at/after 18,000, 269 at/after 20,000, and the last
  disagreed tick stands at **23,992**, eight ticks from
  the season's end. The rejoin tick reads 23,993, which
  is to say no rejoin was observed. Fourteen thousand
  ticks of exact journal after the window were not
  enough. The spur returned; the train did not.
- **P5 HELD, exact.** The windowed-dropped journal —
  1,047 entries where the silenced book keeps 1,309 —
  replays to the windowed-silenced train spike for spike.
  A 0.0 entry is no entry, in window form as in whole-season
  form.
- **P6 HELD — and wider than declared.** Inside the window
  the windowed train disagrees with the baseline on **616**
  ticks, against 617 disagreements with the live truth in
  the same window. It sits at nearly the same distance from
  both animals. It is a third animal entire: the truth's
  body, the baseline's diet, belonging to neither.
- **P7 HELD, exact.** The windowed-silenced book counts
  the full **3,927 bytes**; the windowed-dropped book
  counts 3,141 — a saving of exactly **786 bytes**, 3 per
  silenced landing, and the 262 landings inside the window
  stood inside the declared 200–320 band. Silence in a
  window costs full paper. Absence in a window saves.
- **P8 CRACKED on the perfect column, held on the
  longest.** The windowed season reads perfect **82/112** —
  one *below* the truth's 83, the first silence in this
  row that does not outscore the truth — while its longest
  episode **doubles** to 32 (against 16), its count slips
  fall to 8 (against 12), and its phase slips rise to 22
  (against 17). A whole season of silence was crowned
  (92); a window of it is not. The instrument rewards the
  animal it was tuned on only when the whole season is
  that animal; a hybrid season is scored as what it is.
- **The counts rejoin; the train does not.** Follower
  spikes by region, windowed against live: before the
  window 36 = 36; inside it 318 against 321; after it
  499 against 500. By every count the season after the
  window is the same season. Spike for spike it is a
  different one, to the last week of the year. 9/9 tests
  (the windowed season pinned as found; the spellings'
  boundaries and the anchor asserted exact).

**The law, fifteenth under the Scarecrow's name:** a
silence inside a season is not forgiven when the road
returns. The windowed journal was exact for 1,000 ticks,
silent for 9,000, and exact again for 14,000 — and the
parting it opened at tick 1047 was still open at tick
23,992, wider after the window (979 disagreed ticks) than
inside it (617). Exact input does not heal a train; a
spike resets the voltage but not the road the season has
walked, and this follower carries its history in its
recovery variable long after the kicks agree again. And
the counts lie kindly about it: by region the windowed
season's spike counts stand within 3 of the truth's
everywhere (36/36, 318/321, 499/500), so a ledger of
counts would call the season healed. It was not. Count
the train, not the tally, when you ask whether a parting
closed.

**Next seeds:** a shorter window — silence road B for
1,000 ticks only (1,000–2,000): is there a window length
short enough that the train *does* rejoin, and where
between 1,000 and 9,000 ticks does the healing die? A
late window (10,000–19,000) still stands beside it: does
a silence in the season's second half part to the end
too, or does position matter as much as length? And the
tick-64 question in window form still stands, from
whole-kicks: a journal coarse only inside one chosen
window — does any parting wait at the window's edge, or
only at a tender tick?

