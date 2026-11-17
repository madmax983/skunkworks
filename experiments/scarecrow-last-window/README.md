# Scarecrow Last-Window 🌾

**Host:** `experiments/scarecrow-second-kick` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-one-kick`, `scarecrow-late-window`,
`scarecrow-after-window`, `scarecrow-ten-window`,
`scarecrow-hundred-window`, `scarecrow-short-window`,
`scarecrow-windowed-silence`, `scarecrow-spurless`,
`scarecrow-whole-kicks`, `scarecrow-two-roads`, and
`scarecrow-sheaf` — the champion pair on the parent
crate `crates/neuro-sim`: driver A, Regular Spiking
(I=12), and driver B, Regular Spiking (I=14), feeding
one Fast Spiking follower (I=4.8) over two roads of
opposite sign (A: weight −1.0, delay 10, landings at
spike + 11; B: weight +0.75, delay 6, landings at
spike + 7), counted over 24,000 ticks by the Widow's
instrument. Eight windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), 1,000 after the
tender tick (2,000–3,000), 9,000 late
(10,000–19,000), one kick at the gate
([10,000, 10,010)), and the second kick alone
([10,035, 10,045)). Every one of them parted the
train and none of them healed — the partings stood
to ticks 23,992–23,999 — and the row's law is *lose
nothing*. Second-kick's law, twenty-second under my
name: tenderness is not an address — the second kick
parted at 10,120, at a wait of 85 the row had not
recorded, and stood to 23,992 on one kick alone.
This planting takes up second-kick's closing seed
verbatim: *a window with no remainder still stands,
priced by late-window: silence road B on
[19,000, 24,000), to the season's own edge — the
wreck is then all inside the window, with no
after-region to price a rate in: does the parting
still stand open at the last tick, and what does
the instrument call a season that ends mid-parting?*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-17. The comparison and
predictions below were declared before the first
run and were not edited after it. Verdicts are at
the bottom, as found.

## The field, surveyed

I stood in the season's last field and counted the
landings before I planted. That is Phase 1, and I
did it in the open (the crate's `--survey` pass,
which runs the live field and counts landings only
— no replay, no parting measured): road B lands
**556** times before tick 19,000, exactly
**146** times inside [19,000, 24,000) — the
first at **19,033**, the last at
**23,993** — and never after it, because there
is no after it: the window runs to tick 24,000, the
season's own edge. Before the window, the journal
is exact for 19,000 ticks. Inside it, every road-B
line says nothing. After it, there is nothing —
no exact journal resumes, no remainder in which a
parting could stand, close, or be priced by the
family's rate.

Every earlier window left a remainder, and the
row's standings were all read in one: late-window's
parting stood 4,992 ticks past its window's end;
the second kick's stood 13,947 past its own. The
rejoin tick — the first tick after a window from
which the two trains agree to the season's end —
was a real measurement there. Here it is not: the
window's end is the season's end, so the instrument
can only read 24,000, and that reading means a
different thing. It does not say *no rejoin was
observed in a long remainder*. It says *the season
ran out while the trains were still apart* — if
they are apart at all. And the Widow's cell, which
has named every burned season in this row
(73–86 perfect of 112, verdicts EPISODIC and
WALKING), is asked to name a season that is exact
for its first 19,000 ticks and parted only in its
last five thousand, with no far side: most of its
super-cycles are the truth's own, and its burned
ones stand at the end, where an episode cannot be
outlived.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before
the window [0, 19,000), inside it [19,000, 24,000),
and in the tails at/after 20,000, 22,000, and
23,000, all of which stand inside this window.
There is no after-region. The **rejoin tick** is
printed as the row defines it — the first tick at
or after the window's end from which the two trains
agree to the season's end — and reads 24,000 by
construction here; its meaning is carried by the
last disagreed tick instead. This crate also
replays late-window's own spelling and
second-kick's own spelling as anchors.

- **Live** — the two-road field, whole, writing
  the signed journal as it steps: (tick, weight)
  per landing.
- **Baseline** — the single-road pair exactly as
  the sheaf builds it, run whole in this crate,
  pinning the host.
- **Replay (signed)** — the journal as written.
  The control.
- **Replay (silenced, full season)** — road B
  written 0.0 everywhere: spurless' anchor,
  re-measured in this crate.
- **Replay (late-silenced)** — road B written 0.0
  on ticks in [10,000, 19,000): late-window's own
  spelling, re-measured here as an anchor. It
  touches exactly 264 entries, the first at 10,000.
- **Replay (second-kick-silenced)** — road B
  written 0.0 only on ticks in [10,035, 10,045):
  second-kick's own spelling, re-measured here as
  an anchor. It touches exactly 1 entry, at 10,035.
- **Replay (last-window-silenced)** — road B
  written 0.0 only on ticks in [19,000, 24,000);
  every line kept, exact outside the window. The
  seed's spelling. It touches exactly 146
  entries, the first at 19,033.
- **Replay (last-window-dropped)** — road B's
  entries inside the window removed outright;
  exact outside it.

Paper is counted as the row counts it: a signed
entry is 3 bytes (u16 tick + i8 weight in quarter
units), and a whole checkpoint of this field is 80
bytes. A silenced line still takes its 3 bytes. A
dropped line takes none.

**Predictions (declared before the first run):**

- **B1 (host check):** the single-road baseline
  reproduces the sheaf's centre exact — perfect
  92/112, count slips 3, phase slips 17, longest
  58, mean episode 18.4 (±0.05).
- **B2 (host check):** the live two-road season
  reproduces the row's ledger exact — perfect
  83/112, count slips 12, phase slips 17, episodes
  9, longest 16, mean episode 9.2; follower spikes
  857; signed entries 1,309; coincidences 24, first
  at tick 464.
- **B3 (anchor check):** the full-season silenced
  journal replays to the baseline train exact and
  parts from the live train at tick 65, as spurless
  found.
- **B4 (anchor):** late-window's spelling,
  re-measured in this crate, reproduces
  late-window's ledger entire — 264 entries
  silenced, parting at 10,005, last disagreed tick
  23,992, 940 disagreed ticks in all.
- **B5 (anchor):** second-kick's spelling,
  re-measured in this crate, reproduces
  second-kick's ledger entire — 1 entry silenced,
  parting at 10,120, last disagreed tick 23,992,
  946 disagreed ticks in all.
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2:** the last-window journal is exact before
  the window: no disagreed tick stands before tick
  19,000.
- **P3 (causality):** no disagreed tick stands
  before the first silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **19,033**. Grounds: the spelling
  touches nothing upstream of that landing; the
  replay machinery is exact (P1, and every window
  before this one).
- **P4 (the seed's ignition):** the parting opens
  inside the window, at a wait inside the row's
  known list — the waits stand at 5, 32, 83, 85
  and 130 — so the first disagreed tick stands in
  **(19,033, 19,163]**. Grounds:
  every silence this row has planted has parted,
  and every parting's wait is on that list; a
  window holding 146 landings, not one, should
  part at the first tender place it covers, as the
  9,000-tick and 1,000-tick windows both parted at
  1047 — the same tick, though their windows
  differed ninefold. If this prediction fails — if
  the parting opens later than any wait yet
  recorded — then the season's last field has a
  tenderness of its own the row has not priced.
- **P5 (the seed's standing):** the parting still
  stands open at the last tick: disagreed ticks
  stand at/after tick 23,000, and the last
  disagreed tick stands at/after **23,900** — where
  every earlier window's parting stood
  (23,992–23,999). Grounds: *lose nothing* has held
  at eight addresses, at doses from 264 kicks down
  to one, and nothing in the law mentions a
  remainder. If this prediction fails — if the
  parting closes inside the window, before the
  season ends — then a parting *can* die of old
  age inside a silence, the row records its first
  healing, and every standing before this one must
  be re-read as a parting that simply had not
  finished closing when the season did.
- **P6:** the last-window-dropped journal replays
  to the last-window-silenced train *exact* —
  spurless' law in window form: a 0.0 entry is no
  entry, inside a window as anywhere.
- **P7:** writing nothing inside a window costs
  full paper: the last-window-silenced book counts
  the signed book's 3,927 bytes; the
  last-window-dropped book saves exactly **3 bytes
  per silenced landing** — 146 landings,
  **438 bytes** — counted in the survey
  before planting and asserted in the flesh by
  test.
- **P8 (the wreck's size, with no remainder to
  price it in):** the wreck is the family's wreck
  at the family's rate, all of it inside the
  window: the total disagreed ticks stand in
  **100..=700** (the family's ~946 disagreed over
  the ~13,900 ticks a mid-season parting stands in,
  priced into this window's ~4,900 standing ticks),
  and the last-window cell's perfect count stands
  in **50..=90** (the burned seasons read 73–86;
  this season is exact for its first 19,000 ticks
  and burned only in its last ~24 super-cycles, so
  the instrument may crown it higher than any
  burned season before it — that, too, is an
  answer to what the instrument calls a season
  that ends mid-parting). A healing, or no parting
  at all, reads far below the band; that is the
  crack this prediction is priced to catch.
- **P9 (the counts, at the edge):** the tallies
  stay close even where the train does not:
  follower spikes total within **8** of the
  truth's 857, as windowed-silence's did (853
  against 857), and exact before the window, where
  the trains themselves are exact. Grounds: by
  region the counts have healed in every window
  before this one; at the edge there is no after
  region left for them to heal in, so the inside
  count is the whole testimony.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before 19,033
parts from the truth before 19,033 — then
the replay is not the same work as the run and the
row's machinery falls with it. If P5 fails, a
parting can close inside a silence: *lose nothing*
was never a law about partings, only about
remainders — every parting this row recorded was
still closing when its season ended, and the
family's last ticks (23,992–23,999) were not
standings but snapshots of a healing nobody waited
long enough to see. If P4 fails, the last field's
tenderness is its own, and the waits list gains an
address term at the season's edge.

## What the run found

*(Filled after the first run; the declarations
above were not edited.)*

- **B1 HELD, exact.** The baseline reproduces the
  sheaf's centre field for field (92/112, slips 3
  and 17, longest 58, mean 18.4), and pins the host
  before the window opens.
- **B2 HELD, exact.** The live season reproduces
  the row's ledger entire: perfect 83/112, slips
  12 and 17, episodes 9, longest 16, mean 9.2;
  follower spikes 857; signed entries 1,309;
  coincidences 24, first at 464.
- **B3 HELD, exact.** The full-season silenced
  anchor is the baseline train spike for spike and
  parts from the live train at tick 65, as spurless
  found. The anchors were measured, not remembered.
- **B4 HELD, exact.** Late-window's spelling,
  re-measured in this crate, reproduces its ledger
  entire: 264 entries silenced, parting at 10,005,
  last disagreed tick 23,992, **940** disagreed
  ticks. The row's cited season stands as cited.
- **B5 HELD, exact.** Second-kick's spelling,
  re-measured in this crate, reproduces its ledger
  entire: 1 entry silenced, parting at 10,120, last
  disagreed tick 23,992, **946** disagreed ticks.
  The gate's neighbour stands as planted, in the
  crate that came to test the season's edge.
- **The window, counted:** road B lands 556 times
  before tick 19,000, exactly **146** times inside
  the window — the first at **19,033**, the last at
  **23,993**, seven ticks from the season's edge —
  and never after it, as the survey counted. One
  hundred and forty-six kicks. Four hundred and
  thirty-eight bytes of paper. The whole dose, in
  the season's last field.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact.** No disagreed tick stands
  before 19,000. A spelling exact before the window
  is exact before the window.
- **P3 HELD, entire.** No disagreed tick stands
  before 19,033, the first silenced landing. The
  fence held at the season's edge too.
- **P4 HELD — the parting opens inside the window,
  at a wait the row had not recorded.** The first
  disagreed tick stands at **19,112** — a wait of
  **79** ticks behind the first silenced landing,
  inside the declared band (19,033, 19,163] priced
  from the row's known waits (5, 32, 83, 85, 130),
  and a wait that is none of them: the list of
  waits is now 5, 32, 83, 130, 85, and 79. The
  last field's tenderness is priced in the family's
  currency but struck in its own coin.
- **P5 HELD — the parting stands open at the last
  tick.** The parting did not close inside the
  window. It stood to tick **23,992** — the family
  address, late-window's and second-kick's own last
  tick — with **71** disagreed ticks at/after
  23,000, and the season ended with it still open:
  the rejoin tick reads 24,000, which here means
  the season ran out mid-parting, not that a long
  remainder was watched and no rejoin came. There
  is no healing in this row, and now there is no
  remainder in it either: *lose nothing* needed
  neither the gate nor the ground after the
  window. A parting stands inside its own silence
  as readily as past it.
- **P6 HELD, exact.** The last-window-dropped
  journal — 1,163 entries where the silenced book
  keeps 1,309 — replays to the last-window-silenced
  train spike for spike. A 0.0 entry is no entry,
  in last-window form as in every form before it.
- **P7 HELD, exact.** The last-window-silenced
  book counts the full **3,927 bytes**; the
  last-window-dropped book counts 3,489 — a saving
  of exactly **438 bytes**, 146 landings at 3 bytes
  each, as surveyed. Silence at the season's edge
  costs full paper too. Absence saves 438 bytes
  and loses the last field.
- **P8 HELD — the wreck is the family's size, and
  the instrument names the season a burned one.**
  The total disagreed ticks read **339** — inside
  the declared 100–700 band, all of them inside
  the window (275 at/after 20,000, 135 at/after
  22,000, 71 at/after 23,000), a rate of 339 in the
  parting's 4,888 standing ticks against the
  family's 946 in 13,872: the same wreck, priced
  without a remainder. And the last-window cell's
  perfect count reads **76/112** — inside the
  declared band, and inside the burned seasons'
  own band (73–86): count slips 14, phase slips
  22, episodes 10, longest 16, mean 7.6, verdict
  EPISODIC. That is the instrument's answer to the
  seed's question: a season that ends mid-parting
  is called exactly what every burned season in
  this row is called. The cell counts the whole
  year; it cannot see that the wreck stood in the
  last field, and it does not grade a parting by
  whether anything came after it.
- **P9 HELD — the counts stay close at the edge;
  the train does not.** Follower spikes total
  **856** against the truth's 857. By region:
  before the window 678 = 678; inside it 178
  against 179. There is no after region for the
  tally to heal in, and it nearly heals anyway —
  one spike in five thousand ticks — while the
  trains disagree on 339 ticks, all of them in the
  only region left. Count the train, not the
  tally.
- **The third animal, at the edge.** Inside the
  window the last-window train disagrees with the
  baseline on **351** ticks, against 339
  disagreements with the live truth in the same
  window. It sits at nearly the same distance from
  both animals, as windowed-silence's did (616
  against 617): the truth's body, the baseline's
  diet, belonging to neither, to the season's last
  tick.
  13/13 tests (the last-window season pinned as
  found; the spellings' boundaries, the 146-landing
  survey, the late-window and second-kick anchors,
  and the causality fence asserted exact).

**The law, twenty-third under the Scarecrow's
name:** a parting does not need a remainder to
stand in. The last window silenced 146 landings in
the season's final field, and the parting it
opened at 19,112 — at a wait of 79 the row had not
recorded — was still open at 23,992, the family's
own last tick, with every one of its 339 disagreed
ticks inside the window that caused it. Nothing
came after. Nothing had to. The standings this row
recorded before were not partings renting a
remainder; they were partings, and the remainder
was only where the row happened to be standing
when it read them. And the instrument, asked what
to call a season that ends mid-parting, answers
with its whole-season count: 76 of 112, EPISODIC —
a burned season, indistinguishable in its ledger
from the gate's and the second kick's. The cell
cannot tell where in a year the wreck stood. It
was never measuring the wreck's address. It was
measuring the year. Write the line down. Stand
still. The season ends where it ends, and a
parting that is open then is open.

**Next seeds:** the tender place named in advance
still stands, and wider now: the surfacings stood
at 10,005, 10,120 and 19,112, at waits of 5, 85 and
79 after their kicks — is there a pattern in the
truth's own train at the kick that names the wait
in advance, readable without the difference, or
does a wait leave nothing to read until it ends?
And the tick-64 question in window form still
stands, from whole-kicks: a journal coarse only
inside one chosen window — does any parting wait
at the window's edge, or only where the difference
first changes a spike? A third stands beside them,
new from this planting: the mirror of this window
at the season's other edge — silence road B on
[0, 5,000), from the season's first tick, with the
whole season for a remainder: the parting then
opens before the instrument's settle has even
ended (the full-season silence parted at 65), and
nineteen thousand exact ticks follow it — does a
parting opened at the season's birth stand to the
same 23,992, or does a whole season of exact
journal after a first-field silence finally close
what no remainder in this row has closed?

