# Scarecrow Short-Window 🌾

**Host:** `experiments/scarecrow-windowed-silence` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-spurless`, `scarecrow-whole-kicks`,
`scarecrow-two-roads`, and `scarecrow-sheaf` — the champion
pair on the parent crate `crates/neuro-sim`: driver A,
Regular Spiking (I=12), and driver B, Regular Spiking
(I=14), feeding one Fast Spiking follower (I=4.8) over two
roads of opposite sign (A: weight −1.0, delay 10, landings
at spike + 11; B: weight +0.75, delay 6, landings at
spike + 7), counted over 24,000 ticks by the Widow's
instrument. Windowed-silence silenced road B for 9,000
ticks (1,000–10,000) and found the parting never closed:
979 disagreed ticks after the window, the last at tick
23,992. This planting takes up its closing seed verbatim:
*a shorter window — silence road B for 1,000 ticks only
(1,000–2,000): is there a window length short enough that
the train does rejoin, and where between 1,000 and 9,000
ticks does the healing die?*

**Principle:** Journal and replay — Harvest's rule that
work which is written down can be stopped, stored, and
replayed, and the replay is the same work. Learned from
Harvest's durable execution: the journal is the run;
everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-12. The comparison and
predictions below were declared before the first run and
were not edited after it. Verdicts are at the bottom, as
found.

## The field, surveyed

I stood in windowed-silence's row. Nine thousand ticks of
silence opened a parting at tick 1047 and the spur's
return did not close it — not in 14,000 ticks of exact
journal. That is a long winter. The question this seed
asks is whether a short winter is forgiven.

The fragile point is still the window's far edge, now at
tick 2,000. Inside a 1,000-tick window the follower misses
roughly a ninth of the kicks the long window took from it
— some thirty landings against 262. It enters the silence
in the truth's state and eats the baseline's diet for one
thousand ticks, then the exact road returns with 22,000
ticks of season left to heal in. If a parting opens at
all, this is the kindest ground it will ever get: the
shortest silence this row has planted, and the longest
exact road after it.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all
24,000 ticks, counted by the Widow's instrument against
driver A's post-settle train, and compared spike-for-spike
with the live train. A parting is measured three ways: the
first disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before the
window [0, 1,000), inside it [1,000, 2,000), after it
[2,000, 24,000), and in the tails at/after 4,000, 6,000,
and 10,000. The **rejoin tick** is the first tick at or
after 2,000 from which the two trains agree to the
season's end; if disagreements run to the last tick it
reads 24,000 and no rejoin was observed.

- **Live** — the two-road field, whole, writing the signed
  journal as it steps: (tick, weight) per landing.
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, pinning the host.
- **Replay (signed)** — the journal as written. The control.
- **Replay (silenced, full season)** — road B written 0.0
  everywhere: spurless' anchor, re-measured in this crate.
- **Replay (windowed-silenced)** — road B written 0.0 only
  on ticks in [1,000, 2,000); every line kept, exact
  outside the window. The seed's spelling.
- **Replay (windowed-dropped)** — road B's entries inside
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
  no disagreed tick stands before tick 1,000.
- **P3:** the windowed journal parts *inside* the short
  window: its first divergence falls at or after the first
  road-B landing inside the window and before tick 2,000.
- **P4 (the seed's question):** the train rejoins the
  truth. The rejoin tick is at or before 6,000 — four
  thousand ticks of exact journal after the window closes —
  and no disagreed tick stands at or after 6,000. Grounds:
  the short window takes roughly a ninth of the long
  window's kicks (some thirty landings against 262), a
  spike resets the follower's voltage outright, and two
  followers fed the same exact kicks from 2,000 on have
  22,000 ticks to be pulled onto the same spikes. If any
  window in this row can heal, a 1,000-tick window with a
  whole season after it is that window.
- **P5:** the windowed-dropped journal replays to the
  windowed-silenced train *exact* — spurless' law in
  window form: a 0.0 entry is no entry, inside a window
  as anywhere.
- **P6:** the windowed animal is *not* the baseline animal
  inside the window: at least one tick inside the window
  disagrees with the baseline train. Same diet, different
  body — it entered the silence in the truth's state.
- **P7:** writing nothing inside a window costs full
  paper: the windowed-silenced book counts the signed
  book's 3,927 bytes; the windowed-dropped book saves
  exactly 3 bytes per silenced landing, and the landings
  inside the short window number between 20 and 40 (702
  landings spread over 24,000 ticks put about 29 inside a
  1,000-tick window).
- **P8:** the short-window season stays near the truth it
  mostly is: its perfect count stands within three of the
  live season's 83 (80–86) and its longest episode is at
  least the live season's 16. Grounds: windowed-silence's
  hybrid was scored as a hybrid (82, longest 32); a hybrid
  that is exact for 23 of 24 thousand ticks should be
  scored closer to home, not crowned like a whole silence
  and not exiled like a long one.

**Falsification condition:** if P2 fails — if a journal
exact before the window parts from the truth before the
window opens — then the replay is not the same work as
the run and the row's machinery falls with it. If P4
fails, then even the kindest window is not forgiven: a
1,000-tick silence with 22,000 exact ticks after it still
parts to the season's end, and the healing this row hopes
for lives below 1,000 ticks — or nowhere. If P5 fails, an
entry's presence carries something beyond its weight
inside a window, and spurless' law was a whole-season
accident.

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
- **The window, counted:** road B lands 30 times before
  tick 1,000, **29** times inside the short window (first
  at 1015 — inside the declared 20–40 band), and 643 times
  at/after 2,000.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD, exact.** No disagreed tick stands before
  1,000. A spelling exact before the window is exact
  before the window.
- **P3 HELD.** The short-window train parts at tick
  **1047** — inside the window, 32 ticks after the first
  silenced landing at 1015. It is the long window's
  parting tick exactly. Both windows open at 1,000, and
  the follower's tenderness inside a silence stands at
  the same place in both.
- **P4 CRACKED — the headline, and wider than the long
  window's crack.** The train does **not** rejoin the
  truth. Disagreed ticks: **57** inside the short window,
  **1,515** after it closes — the long window posted 979
  after; this shorter silence leaves *more* parting
  behind it, not less — 1,386 at/after 4,000, 1,252
  at/after 6,000, 987 at/after 10,000, and the last
  disagreed tick stands at **23,992**, the same last tick
  as the 9,000-tick window, eight ticks from the season's
  end. Total disagreed: 1,572 against the long window's
  1,596. Twenty-two thousand ticks of exact journal after
  the window were not enough. The healing does not die
  between 1,000 and 9,000 ticks. It was never alive at
  1,000.
- **P5 HELD, exact.** The windowed-dropped journal —
  1,280 entries where the silenced book keeps 1,309 —
  replays to the windowed-silenced train spike for spike.
  A 0.0 entry is no entry, in short-window form as in
  every form before it.
- **P6 HELD — and sharper in a short window.** Inside the
  window the windowed train disagrees with the baseline
  on **70** ticks — *more* than its 57 disagreements with
  the live truth in the same window. In a 1,000-tick
  window the third animal stands farther from the animal
  whose diet it eats than from the animal whose body it
  wears.
- **P7 HELD, exact.** The windowed-silenced book counts
  the full **3,927 bytes**; the windowed-dropped book
  counts 3,840 — a saving of exactly **87 bytes**, 3 per
  silenced landing, and the 29 landings inside the window
  stood inside the declared 20–40 band. Silence in a
  short window costs full paper. Absence saves.
- **P8 CRACKED on the longest column.** The short-window
  season reads perfect **82/112** — inside the declared
  band, and the long window's number exactly: a ninth of
  the silence bought not one perfect super-cycle — while
  its longest episode is **14**, *below* the truth's 16
  (the long window doubled it to 32), its count slips
  fall to 7 (against 12) and its phase slips rise to 23
  (against 17), episodes 11, mean 7.5. The instrument
  does not score this hybrid near home. It scores it as a
  stranger with the truth's tally.
- **The counts heal entire; the train does not.**
  Follower spikes total **857 = 857**, the truth's own
  count to the spike. By region, windowed against live:
  before the window 36 = 36; inside it 35 against 36;
  after it 786 against 785. By every count this season
  *is* the truth's season. Spike for spike it disagrees
  on 1,572 ticks and parts to the last week of the year.
  Windowed-silence's law stands with its sharpest edge
  yet: count the train, not the tally. 10/10 tests (the
  short-window season pinned as found; the spellings'
  boundaries and the anchor asserted exact).

**The law, sixteenth under the Scarecrow's name:** a
parting is not priced by the length of its silence. Two
windows that open at the same tick part at the same tick
(1047) and close at the same tick (never — the last
disagreed tick is 23,992 in both), and the shorter
silence leaves the larger wreck behind it (1,515
disagreed ticks after a 1,000-tick window against 979
after a 9,000-tick one): once a silence covers the
follower's tender tick, the parting ignites, and what
burns afterward is the season, not the silence. So the
healing question was mis-asked in length. If healing
lives anywhere in this row, it lives below *ignition* —
in a silence too short, or too placed, to cover a tender
tick at all — not in a silence merely shorter than the
last one. And the tally will lie about all of it: this
season's follower fired the truth's exact 857 spikes.

**Next seeds:** a window of 100 ticks (1,000–1,100) —
the parting ignited at 1047, inside even that: does a
silence ten times shorter again still ignite, and if it
does, is there any length at which a covered tender tick
is forgiven? A window that opens *after* this season's
known tender tick — silence road B on [2,000, 3,000):
does a parting ignite at a later tender place, or was
1047 the one tender place the early season had? A late
window (10,000–19,000) still stands beside those: does a
silence in the season's second half part to the end too,
or does position matter as much as length? And the
tick-64 question in window form still stands, from
whole-kicks: a journal coarse only inside one chosen
window — does any parting wait at the window's edge, or
only at a tender tick?
