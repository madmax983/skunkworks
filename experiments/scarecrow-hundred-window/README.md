# Scarecrow Hundred-Window 🌾

**Host:** `experiments/scarecrow-short-window` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-windowed-silence`, `scarecrow-spurless`,
`scarecrow-whole-kicks`, `scarecrow-two-roads`, and
`scarecrow-sheaf` — the champion pair on the parent
crate `crates/neuro-sim`: driver A, Regular Spiking
(I=12), and driver B, Regular Spiking (I=14), feeding
one Fast Spiking follower (I=4.8) over two roads of
opposite sign (A: weight −1.0, delay 10, landings at
spike + 11; B: weight +0.75, delay 6, landings at
spike + 7), counted over 24,000 ticks by the Widow's
instrument. Windowed-silence silenced road B for 9,000
ticks (1,000–10,000); short-window for 1,000 ticks
(1,000–2,000). Both parted at tick 1047 and neither
rejoined: the last disagreed tick stood at 23,992 in
both, and the shorter silence left the larger wreck
(1,515 disagreed ticks after the 1,000-tick window
against 979 after the 9,000-tick one). This planting
takes up short-window's closing seed verbatim: *a
window of 100 ticks (1,000–1,100) — the parting ignited
at 1047, inside even that: does a silence ten times
shorter again still ignite, and is there any length at
which a covered tender tick is forgiven?*

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

I stood in short-window's row. A thousand ticks of
silence parted the train at 1047 and 22,000 exact ticks
after it did not heal it. The law that row wrote says a
parting is not priced by the length of its silence —
only by whether the silence covers the follower's
tender tick. This seed tests the law at its smallest
grain: a hundred ticks, a handful of road-B landings,
the tender tick covered all the same.

The fragile point is still the window's far edge, now
at tick 1,100. Inside a hundred ticks the follower
misses perhaps three landings — against 29 in the short
window and 262 in the long one. It enters the silence
in the truth's state and eats the baseline's diet for
one hundred ticks, then the exact road returns with
22,900 ticks of season left. If any covered silence can
be forgiven, this is the kindest ground forgiveness
will ever get in this row: the fewest kicks ever taken,
and nearly the whole season to heal in. If the law
holds, none of that matters. The tender tick is
covered. The parting ignites. The season burns.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all
24,000 ticks, counted by the Widow's instrument against
driver A's post-settle train, and compared spike-for-spike
with the live train. A parting is measured three ways: the
first disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before the
window [0, 1,000), inside it [1,000, 1,100), after it
[1,100, 24,000), and in the tails at/after 4,000, 6,000,
and 10,000. The **rejoin tick** is the first tick at or
after 1,100 from which the two trains agree to the
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
  on ticks in [1,000, 1,100); every line kept, exact
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
- **P3:** the windowed journal parts *inside* the hundred
  window: its first divergence falls at or after the first
  road-B landing inside the window and before tick 1,100.
  Grounds: both earlier windows opened at 1,000 and parted
  at 1047, 32 ticks after the first silenced landing at
  1015; this window silences that same landing and covers
  that same tender tick.
- **P4 (the seed's question, answered by the law):** the
  parting does **not** heal. Disagreed ticks stand at/after
  tick 10,000 and the last disagreed tick stands at or
  after 23,000. Grounds: short-window's law — a parting is
  priced by the coverage of the tender tick, not by the
  length of the silence; a 1,000-tick silence and a
  9,000-tick silence both stood to tick 23,992, and the
  shorter left more wreck, not less. A hundred ticks
  covers 1047 exactly as nine thousand did. If this
  prediction fails — if the train rejoins — then length
  matters after all, somewhere below 1,000 ticks, and the
  law gets its first edge.
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
  inside the hundred window number between 1 and 6 (702
  landings spread over 24,000 ticks put about 3 inside a
  100-tick window).
- **P8:** the hundred-window season stays near the truth
  it mostly is: its perfect count stands within three of
  the live season's 83 (80–86) and its longest episode is
  at least the live season's 16. Grounds: the instrument
  scored the 1,000-tick hybrid a stranger (perfect 82,
  longest 14, *below* the truth's 16); a hybrid exact for
  all but a hundred ticks is declared here on the same
  terms, so the comparison stands or falls at the same
  fence.

**Falsification condition:** if P2 fails — if a journal
exact before the window parts from the truth before the
window opens — then the replay is not the same work as
the run and the row's machinery falls with it. If P4
fails, then a covered tender tick *can* be forgiven at
some length, the healing lives between 100 and 1,000
ticks, and short-window's law was a two-point line drawn
through a curve. If P5 fails, an entry's presence carries
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
- **The window, counted:** road B lands 30 times before
  tick 1,000, **3** times inside the hundred window (first
  at 1015 — inside the declared 1–6 band), and 669 times
  at/after 1,100. Three landings. Nine bytes of silence,
  written as nothing.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD, exact.** No disagreed tick stands before
  1,000. A spelling exact before the window is exact
  before the window.
- **P3 HELD.** The hundred-window train parts at tick
  **1047** — inside the window, 32 ticks after the first
  silenced landing at 1015. It is the long window's
  parting tick and the short window's parting tick. Three
  windows, three lengths, one tick: the tender place does
  not move.
- **P4 HELD — the law stands at its smallest grain.** The
  train does **not** rejoin the truth. Disagreed ticks:
  **4** inside the hundred window, **1,583** after it
  closes — against the short window's 1,515 after and the
  long window's 979 — 1,389 at/after 4,000, 1,259 at/after
  6,000, **979 at/after 10,000** (the long window's own
  after-count, read at a different fence), and the last
  disagreed tick stands at **23,992**, the same last tick
  as both earlier windows, eight ticks from the season's
  end. Total disagreed: 1,587, between the short window's
  1,572 and the long window's 1,596. Three silenced
  landings did what 262 did. A covered tender tick is not
  forgiven at a hundred ticks, and the wreck is not
  smaller for the silence being smaller — it is, if
  anything, the same wreck, re-dealt.
- **P5 HELD, exact.** The windowed-dropped journal —
  1,306 entries where the silenced book keeps 1,309 —
  replays to the windowed-silenced train spike for spike.
  A 0.0 entry is no entry, in hundred-window form as in
  every form before it.
- **P6 HELD — and sharper still at this grain.** Inside
  the window the windowed train disagrees with the
  baseline on **6** ticks — *more* than its 4
  disagreements with the live truth in the same window.
  In a hundred-tick window the third animal again stands
  farther from the animal whose diet it eats than from
  the animal whose body it wears.
- **P7 HELD, exact.** The windowed-silenced book counts
  the full **3,927 bytes**; the windowed-dropped book
  counts 3,918 — a saving of exactly **9 bytes**, 3 per
  silenced landing, and the 3 landings inside the window
  stood inside the declared 1–6 band. Silence in a
  hundred window costs full paper. Absence saves nine
  bytes and loses the season.
- **P8 CRACKED on the perfect column — the widest crack
  in the row.** The hundred-window season reads perfect
  **73/112** — *below* both longer windows' 82, ten below
  the truth's 83 — while its longest episode is **16**,
  the truth's own number entire (the short window fell to
  14; the long window doubled to 32), its count slips 15,
  phase slips 24, episodes 8, mean 9.1. The instrument
  does not score this hybrid near home, nor between the
  other hybrids: it scores the *shortest* silence the
  *strangest*. The less of the season the silence
  touches, the worse the season reads — the exact
  opposite of a dose response, in the instrument's own
  hand.
- **The counts nearly heal; the train does not.**
  Follower spikes total **856** against the truth's 857 —
  one spike short, where the short window matched the
  count entire. By region, windowed against live: before
  the window 36 = 36; inside it 3 = 3; after it 817
  against 818. By every count this season *is* the
  truth's season, one spike notwithstanding. Spike for
  spike it disagrees on 1,587 ticks and parts to the
  last week of the year. The row's law stands with its
  sharpest edge yet: count the train, not the tally.
  10/10 tests (the hundred-window season pinned as
  found; the spellings' boundaries and the anchor
  asserted exact).

**The law, seventeenth under the Scarecrow's name:**
ignition is not a dose. Three windows of 9,000, 1,000,
and 100 ticks — 262, 29, and 3 landings silenced — part
at the same tick (1047), stand to the same last tick
(23,992), and leave the same wreck (1,596, 1,572, 1,587
disagreed ticks): a silence is not measured in ticks or
in landings but in whether it covers the tender tick,
and coverage is whole at every length yet planted. So
the healing question is closed in length: if a covered
tender tick can be forgiven anywhere, it is not at a
shorter length — it is at a *different address*, a
silence that stands somewhere the follower is not
tender. And the instrument's bill runs backwards: the
season exact for 23,900 of its 24,000 ticks reads
perfect 73, stranger than seasons silenced nine and
ninety times longer — never grade a parted season by
how much of it was exact.

**Next seeds:** a window that opens *after* this season's
known tender tick — silence road B on [2,000, 3,000):
does a parting ignite at a later tender place, or was
1047 the one tender place the early season had? A late
window (10,000–19,000) still stands beside it: does a
silence in the season's second half part to the end too,
or does position matter as much as coverage? A window
of ten ticks (1,040–1,050) — the smallest silence that
still covers 1047, silencing perhaps a single landing:
is one kick the whole ignition, or does the tender tick
need company? And the tick-64 question in window form
still stands, from whole-kicks: a journal coarse only
inside one chosen window — does any parting wait at the
window's edge, or only at a tender tick?
