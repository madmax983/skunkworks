# Scarecrow Ten-Window 🌾

**Host:** `experiments/scarecrow-hundred-window` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-short-window`, `scarecrow-windowed-silence`,
`scarecrow-spurless`, `scarecrow-whole-kicks`,
`scarecrow-two-roads`, and `scarecrow-sheaf` — the
champion pair on the parent crate `crates/neuro-sim`:
driver A, Regular Spiking (I=12), and driver B, Regular
Spiking (I=14), feeding one Fast Spiking follower
(I=4.8) over two roads of opposite sign (A: weight
−1.0, delay 10, landings at spike + 11; B: weight
+0.75, delay 6, landings at spike + 7), counted over
24,000 ticks by the Widow's instrument. Three windows
stand in this row: 9,000 ticks (1,000–10,000), 1,000
(1,000–2,000), and 100 (1,000–1,100). All three parted
at tick 1047 and none rejoined — the last disagreed
tick stood at 23,992 in all three, and the hundred
window, silencing only 3 landings, left a wreck (1,587
disagreed ticks) between the other two (1,596 and
1,572). Hundred-window's law: ignition is not a dose;
a silence is measured by whether it covers the
follower's tender tick, and coverage is whole at every
length yet planted. This planting takes up
hundred-window's closing seed verbatim: *a window of
ten ticks (1,040–1,050) — the smallest silence that
still covers 1047, silencing perhaps a single landing:
is one kick the whole ignition, or does the tender
tick need company?*

**Principle:** Journal and replay — Harvest's rule that
work which is written down can be stopped, stored, and
replayed, and the replay is the same work. Learned from
Harvest's durable execution: the journal is the run;
everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-13. The comparison and
predictions below were declared before the first run and
were not edited after it. Verdicts are at the bottom, as
found.

## The field, surveyed

I stood in hundred-window's row and counted the
landings before I planted. That is Phase 1, and I did
it in the open: road B lands at **1015, 1049 and 1083**
inside the hundred window's span — the three kicks the
last planting took. Inside this seed's ten ticks,
[1,040, 1,050), road B lands exactly **once, at tick
1049**.

Two ticks after the tender tick.

So this window covers 1047 the way a fence covers a
field it stands beside. On the calendar, covered. In
the flesh, never reached: the only kick the silence
takes arrives downstream of the tenderness, when the
tender tick has already fired — or not fired — exactly
as the truth had it. The fragile point of every window
in this row was the tender tick inside it. This
window's fragile point is different: it asks whether
tenderness is a *place* at all, or whether any single
kick, taken alone at any address, is ignition enough.

If the row's law is whole, this is the quietest
planting in the row: no parting at 1047, and whatever
the missing kick at 1049 does, it does not burn the
season. If one kick is the whole ignition wherever it
stands, the law was never about coverage — it was
about loss, and any lost kick costs the year.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all
24,000 ticks, counted by the Widow's instrument against
driver A's post-settle train, and compared spike-for-spike
with the live train. A parting is measured three ways: the
first disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before the
window [0, 1,040), inside it [1,040, 1,050), after it
[1,050, 24,000), and in the tails at/after 4,000, 6,000,
and 10,000. The **rejoin tick** is the first tick at or
after 1,050 from which the two trains agree to the
season's end; if disagreements run to the last tick it
reads 24,000 and no rejoin was observed; if the trains
never disagree it reads 1,050.

- **Live** — the two-road field, whole, writing the signed
  journal as it steps: (tick, weight) per landing.
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, pinning the host.
- **Replay (signed)** — the journal as written. The control.
- **Replay (silenced, full season)** — road B written 0.0
  everywhere: spurless' anchor, re-measured in this crate.
- **Replay (windowed-silenced)** — road B written 0.0 only
  on ticks in [1,040, 1,050); every line kept, exact
  outside the window. The seed's spelling. It touches
  exactly one entry: the landing at 1049.
- **Replay (windowed-dropped)** — road B's entry inside
  the window removed outright; exact outside it.

Paper is counted as the row counts it: a signed entry is
3 bytes (u16 tick + i8 weight in quarter units), and a
whole checkpoint of this field is 80 bytes. A silenced
line still takes its 3 bytes. A dropped line takes none.

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
  no disagreed tick stands before tick 1,040.
- **P3 (causality):** no disagreed tick stands before the
  one silenced landing itself — every disagreed tick, if
  any exist, stands at or after tick **1049**, and the
  tender tick **1047 is not a disagreed tick**, though
  the window covers it on the calendar. Grounds: the
  spelling touches nothing upstream of 1049; the replay
  machinery is exact (P1, and every window before this
  one); tenderness cannot be reached by a silence that
  arrives after it.
- **P4 (the seed's question, answered by the law):** one
  kick is **not** the whole ignition. If a parting opens
  at all behind the missing landing, it **heals**: no
  disagreed tick stands at/after tick 10,000 and the last
  disagreed tick stands before 23,000. Grounds:
  hundred-window's law — ignition is coverage of the
  tender tick, and this silence covers no tender tick in
  the flesh; the kick at 1049 is an ordinary kick at an
  ordinary address, and the row has never yet seen an
  ordinary loss burn a season: the whole-kicks row's
  coarsenings parted at the family's tender places, and
  every window's parting opened at 1047, never at a
  silenced landing itself. If this prediction fails — if
  one kick at 1049 parts the train to the season's end —
  then coverage was never the law: *loss* was, and any
  journal that drops a single line anywhere is a burned
  season.
- **P5:** the windowed-dropped journal replays to the
  windowed-silenced train *exact* — spurless' law in
  window form: a 0.0 entry is no entry, inside a window
  as anywhere.
- **P6:** the windowed animal is *not* the baseline animal
  inside the window: at least one tick inside [1,040,
  1,050) disagrees with the baseline train. Same diet,
  different body — it entered the silence in the truth's
  state. (Declared knowing the window is ten ticks wide
  and the two trains disagree on only a few ticks in a
  hundred; this is the prediction most likely to crack,
  and a crack here is a measure of the window's smallness,
  not of the machinery.)
- **P7:** writing nothing inside a window costs full
  paper: the windowed-silenced book counts the signed
  book's 3,927 bytes; the windowed-dropped book saves
  exactly 3 bytes — one landing, the one at 1049, counted
  in the survey before planting and asserted in the flesh
  by test.
- **P8:** the ten-window season stays near the truth it
  mostly is: its perfect count stands within three of the
  live season's 83 (80–86) and its longest episode is at
  least the live season's 16. Grounds: the instrument
  scored the hundred-window hybrid a stranger (perfect
  73) *because* its parting stood to the season's end; a
  season whose parting heals — or never opens — is
  declared here on the same terms, and this prediction
  stands or falls with P4.

**Falsification condition:** if P3 fails — if a journal
that touches nothing before 1049 parts from the truth
before 1049 — then the replay is not the same work as
the run and the row's machinery falls with it. If P4
fails, then a covered tender tick was never the
requirement: one kick, anywhere, is the whole ignition,
and hundred-window's law is replaced by a harder one —
a journal must lose *nothing*, at any address, or lose
the season. If P5 fails, an entry's presence carries
something beyond its weight inside a window, and
spurless' law was a whole-season accident.

## What the run found

*(Filled after the first run; the declarations above were
not edited.)*

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
- **The window, counted:** road B lands 31 times before
  tick 1,040, exactly **1** time inside the ten window —
  at **1049**, as the survey counted — and 670 times
  at/after 1,050. One landing. Three bytes of silence,
  written as nothing.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD, exact.** No disagreed tick stands before
  1,040. A spelling exact before the window is exact
  before the window.
- **P3 HELD, entire.** No disagreed tick stands before
  1049, and 1047 is not one — stronger still, no disagreed
  tick stands *inside the window at all*: before 0, inside
  **0**, and the follower's counts inside the window read
  1 = 1. The silence passed over the tender tick and left
  no mark on it. Causality stands: a journal cannot part
  from the truth upstream of the only line it changed.
- **P4 CRACKED — the law of this row falls here.** One
  kick **is** the whole ignition. Behind the single
  missing landing at 1049 the train parts at tick
  **1132** — eighty-three ticks after the kick was taken,
  at no address this row has named tender — and it does
  **not** heal: **1,593** disagreed ticks, all of them
  after the window, **991** at/after 10,000, the last at
  **23,998**, six ticks from the season's end, the rejoin
  tick reading 23,999. Set the four windows side by side:
  9,000 ticks / 262 landings / wreck 1,596; 1,000 / 29 /
  1,572; 100 / 3 / 1,587; **10 / 1 / 1,593**. One landing
  did what 262 did. Coverage of the tender tick was never
  the requirement. Every earlier window removed the
  landing at 1015, and its consequence surfaced at the
  follower's next tender place, 1047 — so the row read
  the *surfacing* as the *cause*, and named tenderness
  the law. Remove a different kick and the parting
  surfaces at a different place (1132), with the same
  whole-season wreck. What ignites is not a covered tick.
  What ignites is a lost kick — any kick, anywhere, alone.
- **P5 HELD, exact.** The windowed-dropped journal —
  1,308 entries where the silenced book keeps 1,309 —
  replays to the windowed-silenced train spike for spike.
  A 0.0 entry is no entry, in ten-window form as in every
  form before it.
- **P6 HELD, by a single tick.** Inside the window the
  windowed train disagrees with the baseline on exactly
  **1** tick — the prediction declared most likely to
  crack stood by the smallest margin the row allows. In
  ten ticks the third animal and the baseline animal
  share nine; the window is nearly too small to hold
  them apart, and it held by one tick.
- **P7 HELD, exact.** The windowed-silenced book counts
  the full **3,927 bytes**; the windowed-dropped book
  counts 3,924 — a saving of exactly **3 bytes**, one
  landing, at 1049, as surveyed. Silence in a ten window
  costs full paper. Absence saves three bytes and loses
  the season.
- **P8 CRACKED on the perfect column, with P4.** The
  ten-window season reads perfect **76/112** — below the
  declared band's floor of 80, between the hundred
  window's 73 and the longer windows' 82 — while its
  longest episode is **16**, the truth's own number
  entire, its count slips 17, phase slips 19, episodes
  12, mean 6.3. The instrument's bill still runs
  backwards (the truth reads 83; seasons silenced ten,
  a hundred, a thousand and nine thousand ticks read
  76, 73, 82, 82 — no dose response in either direction),
  but the crack's cause is plainer here: the season reads
  a stranger because it *is* one, from tick 1132 to the
  year's last week.
- **The counts nearly heal; the train does not.**
  Follower spikes total **856** against the truth's 857 —
  one spike short, as in the hundred window. By region,
  windowed against live: before the window 37 = 37;
  inside it 1 = 1; after it 818 against 819. By every
  count this season *is* the truth's season, one spike
  notwithstanding. Spike for spike it disagrees on 1,593
  ticks and parts to the last week of the year. Count the
  train, not the tally.
  12/12 tests (the ten-window season pinned as found;
  the spellings' boundaries, the one-landing survey, and
  the causality fence asserted exact).

**The law, eighteenth under the Scarecrow's name:**
ignition is not a dose, and it is not an address either.
Four silences — 9,000, 1,000, 100, and 10 ticks; 262,
29, 3, and 1 landings taken — leave the same wreck
(1,596, 1,572, 1,587, 1,593 disagreed ticks) standing
to the same last week of the year. The tender tick was
never the cause; it was the place where one particular
lost kick's consequence first showed. A parting opens
where the carried difference first changes a spike —
1047 for the kick at 1015, 1132 for the kick at 1049 —
and from there it does not close. So the journal's rule
is harder than the row believed: it is not *cover
nothing tender*; it is *lose nothing*. One line, lost
anywhere, is a burned season — and the season will read
nearly healed in every count the whole way down.

**Next seeds:** a window that opens *after* this season's
known tender tick — silence road B on [2,000, 3,000):
under the new law any window there ignites too, so the
question is no longer whether but *where it surfaces* —
is the parting's opening tick a fixed distance behind
the first taken kick, or does it wait for the follower's
next tender place, wherever that stands? A late window
(10,000–19,000) still stands beside it: in the season's
second half, does one kick still burn the rest of the
year, or does a short remaining season forgive what a
long one could not? And the tick-64 question in window
form still stands, from whole-kicks: a journal coarse
only inside one chosen window — does any parting wait
at the window's edge, or only where the difference first
changes a spike?
