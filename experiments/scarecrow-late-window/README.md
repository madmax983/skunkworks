# Scarecrow Late-Window 🌾

**Host:** `experiments/scarecrow-after-window` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-ten-window`, `scarecrow-hundred-window`,
`scarecrow-short-window`,
`scarecrow-windowed-silence`, `scarecrow-spurless`,
`scarecrow-whole-kicks`, `scarecrow-two-roads`, and
`scarecrow-sheaf` — the champion pair on the parent
crate `crates/neuro-sim`: driver A, Regular Spiking
(I=12), and driver B, Regular Spiking (I=14), feeding
one Fast Spiking follower (I=4.8) over two roads of
opposite sign (A: weight −1.0, delay 10, landings at
spike + 11; B: weight +0.75, delay 6, landings at
spike + 7), counted over 24,000 ticks by the Widow's
instrument. Five windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), and 1,000 after the
tender tick (2,000–3,000). Every one of them parted
the train and none of them healed — the partings
stood to ticks 23,992–23,998 — and the row's law is
*lose nothing*. After-window's law, nineteenth under
my name: a parting's opening cannot be computed from
the kick that caused it; the row's three measured
lags behind a first taken kick — 32, 83, 130 — are
waits for the follower's next tender place, not
distances. This planting takes up after-window's
closing seed verbatim in substance: *a late window
(10,000–19,000): a window in the season's second
half still parts the train under the law — but does
it stand to the season's end when so little season
remains, or does a short remaining season forgive
what a long one could not; and if it parts, is its
lag behind its first taken kick a fourth number on
the row's list of waits (32, 83, 130), or does the
list gain a pattern late?*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-15. The comparison and
predictions below were declared before the first
run and were not edited after it. Verdicts are at
the bottom, as found.

## The field, surveyed

I stood in after-window's row and counted the
landings before I planted. That is Phase 1, and I
did it in the open: road B lands **292** times
before tick 10,000, exactly **264** times inside
[10,000, 19,000) — first at **10,000** itself, the
window's own first tick, last at **18,998**, spaced
34–35 ticks apart — and **146** times at/after
19,000.

Two hundred and sixty-four landings. The long
window's dose, at the season's far end, in a window
that opens at the season's midpoint and closes
5,000 ticks before its end — the shortest remainder
any window in this row has left standing. Every
window before this one had at least 14,000 ticks of
season after it in which to stand open. This one
has 5,000. If a parting needs room to stand, this
is where the row finds out.

The seed asks two questions, and they are priced
differently. The first — *does it stand to the end
when so little season remains* — cannot be priced
by the wreck's total: with 5,000 ticks after the
window, a total in the family's band (1,519–1,596)
is out of arithmetic reach whatever the law does.
The honest price is the rate: the five earlier
windows left **69–70 disagreed ticks per 1,000
ticks** of season after their windows (979 in
14,000; 1,515 in 22,000; 1,457 in 21,000). A
forgiving remainder shows in the rate, not the
tally. The second — *the lag* — is priced against
the list itself: 32, 83, 130. A pattern late would
repeat one of them, opening this parting at 10,032,
10,083, or 10,130. A fourth wait opens it anywhere
else. One address deserves a fencepost of its own:
10,032 stands three ticks *before* the window's
second landing (10,035) — a surfacing there is a
one-kick ignition, the ten window's own shape,
arriving at the long window's lag.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before
the window [0, 10,000), inside it [10,000, 19,000),
after it [19,000, 24,000), and in the tails at/after
20,000, 22,000, and 23,000. The **rejoin tick** is
the first tick at or after 19,000 from which the
two trains agree to the season's end; if
disagreements run to the last tick it reads 24,000
and no rejoin was observed; if the trains never
disagree it reads 19,000.

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
- **Replay (windowed-silenced)** — road B written
  0.0 only on ticks in [10,000, 19,000); every line
  kept, exact outside the window. The seed's
  spelling. It touches exactly 264 entries, the
  first at 10,000.
- **Replay (windowed-dropped)** — road B's entries
  inside the window removed outright; exact
  outside it.

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
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2:** the windowed journal is exact before the
  window: no disagreed tick stands before tick
  10,000.
- **P3 (causality):** no disagreed tick stands
  before the first silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **10,000**. Grounds: the spelling touches
  nothing upstream of 10,000; the replay machinery
  is exact (P1, and every window before this one).
  Here the fence and the window's edge are the same
  tick — the first landing stands on the window's
  own first tick — so P2 and P3 state one fact from
  two sides; both are kept, as the row has always
  kept both.
- **P4 (ignition, under the row's law):** the
  window ignites, and the parting does **not**
  heal even in the short remainder: disagreed ticks
  stand at/after tick 23,000, and the last
  disagreed tick stands at/after **23,900** — where
  every earlier window's parting stood (23,992–
  23,998). Grounds: *lose nothing* has now held at
  five addresses, early and late; nothing in the
  law mentions how much season is left. If this
  prediction fails — if a parting this late heals,
  or closes early — then the remainder's length
  prices the standing after all, and the law gains
  the boundary after-window went looking for and
  did not find at 2,000.
- **P5 (the seed's question, answered for the
  fourth wait):** the parting opens, and its lag
  behind the first taken kick (10,000) is **none of
  32, 83, 130** — the list of waits gains a fourth
  number, and no forecast address (10,032, 10,083,
  10,130) names the surfacing. Grounds:
  after-window's law — the lag is a wait for a
  tender place, a property of the follower's state,
  and three measured waits have never repeated. If
  this prediction fails — if the lag *is* one of
  the three — then the waits are not a scatter:
  something in the machinery keeps a small set of
  distances, and the row must go back and ask what
  the three numbers share.
- **P6:** the windowed-dropped journal replays to
  the windowed-silenced train *exact* — spurless'
  law in window form: a 0.0 entry is no entry,
  inside a window as anywhere.
- **P7:** the windowed animal is *not* the
  baseline animal inside the window: at least one
  tick inside [10,000, 19,000) disagrees with the
  baseline train. Same diet, different body — it
  entered the silence in the truth's state.
- **P8:** writing nothing inside a window costs
  full paper: the windowed-silenced book counts
  the signed book's 3,927 bytes; the
  windowed-dropped book saves exactly **792
  bytes** — 264 landings at 3 bytes each, counted
  in the survey before planting and asserted in
  the flesh by test.
- **P9 (the wreck's rate):** the disagreed ticks
  after the window stand in **300..=400** — 60 to
  80 per 1,000 ticks over the 5,000 remaining,
  against the family's measured 69–70 — and the
  windowed cell's perfect count stands in
  **70..=90** (the burned seasons read 73–86, the
  after window's 86 the crown). Grounds: the rate
  is the one property of the wreck no window has
  yet moved; a rate below the band is the short
  remainder forgiving in the only coin it can pay,
  and a rate above it means a late parting burns
  *faster*, not shorter.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before 10,000 parts
from the truth before 10,000 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P4 fails, *lose
nothing* has a boundary after all, and it stands
in the season's second half: the law holds while
the season is long and lapses when it runs short,
and every earlier statement of it must be re-read
with that boundary drawn in. If P5 fails, the
waits repeat, and a surfacing *can* be forecast
from a short list — the tender-place law falls
back to a distances law with three entries, and
the row owes the list an explanation.

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
- **The window, counted:** road B lands 292 times
  before tick 10,000, exactly **264** times inside
  the window — first at **10,000**, the window's own
  first tick, last at **18,998**, as the survey
  counted — and 146 times at/after 19,000. Two
  hundred and sixty-four landings. Seven hundred
  and ninety-two bytes of silence, written as
  nothing.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact.** No disagreed tick stands
  before 10,000. A spelling exact before the window
  is exact before the window.
- **P3 HELD, entire.** No disagreed tick stands
  before 10,000, the first silenced landing. The
  fence and the window's edge are the same tick at
  this address, and the fence held.
- **P4 HELD — the short remainder forgives
  nothing.** The window ignited in the season's
  second half and the parting stood to the end:
  **940** disagreed ticks in all, **71** of them
  at/after 23,000, the last at **23,992** — the long
  window's, the short window's, and the after
  window's own last tick — and the rejoin tick
  reads 23,993. Five thousand ticks of remainder
  held the parting exactly as twenty-two thousand
  did. *Lose nothing* has no boundary in the
  season's length; none was found at 2,000 and none
  stands at 10,000.
- **P5 HELD — the lag is a fourth wait, and it is
  five ticks.** The parting opens at tick
  **10,005**, a lag of **5** behind the first taken
  kick — none of 32, 83, 130, and nearer than any
  forecast the row could have made: the shortest
  forecast address, 10,032, stands twenty-seven
  ticks *after* the surfacing. The parting did not
  wait through four landings, as the after window's
  did, or even for the second: it surfaced before
  the window's second landing (10,035) arrived,
  thirty ticks early — a one-kick ignition, the ten
  window's shape at a new address, priced at five
  ticks. The list of waits now reads **32, 83, 130,
  5**: no order, no floor, no pattern — four waits
  that share nothing but the follower they waited
  in. A tender place can stand at the gate itself.
- **P6 HELD, exact.** The windowed-dropped
  journal — 1,045 entries where the silenced book
  keeps 1,309 — replays to the windowed-silenced
  train spike for spike. A 0.0 entry is no entry,
  in late-window form as in every form before it.
- **P7 HELD, beyond argument.** Inside the window
  the windowed train disagrees with the baseline on
  **609** ticks — in nine thousand ticks the third
  animal and the baseline animal are different
  animals on one tick in fifteen.
- **P8 HELD, exact.** The windowed-silenced book
  counts the full **3,927 bytes**; the
  windowed-dropped book counts 3,135 — a saving of
  exactly **792 bytes**, 264 landings at 3 bytes
  each, as surveyed. Silence in the second half
  costs full paper too. Absence saves seven hundred
  and ninety-two bytes and loses the season.
- **P9 HELD — the rate is the family's own.** The
  disagreed ticks after the window read **333** in
  5,000 ticks — **66.6 per 1,000**, inside the
  declared 60–80 band and a breath below the
  family's measured 69–70 — and the windowed
  cell's perfect count reads **83/112**, the
  truth's own number entire: count slips 12, phase
  slips 17, both the truth's, with episodes 8
  against 9 and longest **26** against the truth's
  16, mean 10.4. The instrument reads this burned
  season as the truth at headline depth. The row
  has been told before, and tells itself again: a
  score is not the counted thing.
- **The counts nearly heal; the train does not —
  at headline depth this time.** Follower spikes
  total **855** against the truth's 857. By region,
  windowed against live: before the window 357 =
  357; inside it 320 against 321; after it 178
  against 179. Inside the window the two trains
  fire within one spike of each other and disagree
  on **607 ticks** doing it. Count the train, not
  the tally.
  12/12 tests (the late-window season pinned as
  found; the spellings' boundaries, the
  264-landing survey, and the causality fence
  asserted exact).

**The law, twentieth under the Scarecrow's
name:** the season's length is not among the law's
terms. A window in the second half, with five
thousand ticks of season left after it — a quarter
of what the short window had, a fifth of the
season — parts the train at the family's own rate
(66.6 disagreed ticks per 1,000, against 69–70
everywhere else) and stands to the same last tick
as every window before it (23,992). Nothing about
*lose nothing* is priced in remaining ticks. And
the waits gain their fourth number and lose their
last pattern: 32, 83, 130 — and now **5**, a
surfacing five ticks behind the kick that caused
it, before the silence's second landing had even
arrived. There is no shortest wait the row can
promise and no longest it can rule out; a tender
place can stand at the gate, in the first state
the silence touches. So the corollary hardens:
when a line is lost, do not ask arithmetic where
the season will part, and do not ask the calendar
how much of it will burn. Stand in the field and
watch. The surfacing is found, never forecast —
and the burning is the season's whole remainder,
however much remainder there is.

**Next seeds:** one kick at the gate, isolated —
a ten-tick window at [10,000, 10,010) silences
exactly one landing, the one at 10,000: this run
read the parting at 10,005 with 264 kicks taken,
but the surfacing stood before the second kick
arrived, so the dose was one — does the one-kick
window alone part at the same 10,005 and stand to
the same end, or does the standing need the
silence that followed? The tender place named in
advance still stands, and sharper now: the
surfacing at 10,005 stood five ticks after the
kick — is there a pattern in the truth's own train
at 10,000–10,005 that names it, readable without
the difference, or does a five-tick wait leave
nothing to read? The tick-64 question in window
form still stands, from whole-kicks: a journal
coarse only inside one chosen window — does any
parting wait at the window's edge, or only where
the difference first changes a spike? And a new
one, priced by this run: a window with no
remainder at all — silence road B on
[19,000, 24,000), to the season's own edge: the
wreck is then all inside the window, with no
after-region to price a rate in — does the
parting still stand open at the last tick, and
what does the instrument call a season that ends
mid-parting?

