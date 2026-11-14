# Scarecrow After-Window 🌾

**Host:** `experiments/scarecrow-ten-window` (The
Scarecrow 🌾) and its hosts in turn —
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
instrument. Four windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), and 10 (1,040–1,050). The first three
parted at tick 1047, the follower's tender tick, and
the row read coverage of that tick as the law. The
ten window broke it: it took exactly one landing, at
1049 — two ticks after the tender tick, which it
covered only on the calendar — and the train parted
anyway, at tick 1132, eighty-three ticks behind the
taken kick, and stood to 23,998. Ten-window's law:
ignition is not a dose and not an address; a parting
opens where the carried difference first changes a
spike, and the journal's rule is *lose nothing*.
This planting takes up ten-window's closing seed
verbatim in substance: *a window that opens after
the known tender tick — silence road B on
[2,000, 3,000): under the new law any window there
ignites too, so the question is no longer whether
but where it surfaces — is the parting's opening
tick a fixed distance behind the first taken kick,
or does it wait for the follower's next tender
place, wherever that stands?*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-14. The comparison and
predictions below were declared before the first
run and were not edited after it. Verdicts are at
the bottom, as found.

## The field, surveyed

I stood in ten-window's row and counted the
landings before I planted. That is Phase 1, and I
did it in the open: road B lands **59** times
before tick 2,000, exactly **29** times inside
[2,000, 3,000) — first at **2010**, last at
**2970**, spaced 34–35 ticks apart — and **614**
times at/after 3,000.

Twenty-nine landings. The short window's count,
at a new address, in a window that opens after
every tender tick this row has ever named. The
season's tenderness, on the record, stands at
1047. This window does not cover it, does not
stand near it, and silences a stretch of ordinary
road in the season's second eighth.

The row has measured exactly one lag between a
taken kick and a surfacing parting: the ten
window's. Taken at 1049, surfaced at 1132 — a lag
of **83 ticks**. One lag is not a law. It is a
single fencepost, and this planting asks what it
is a fencepost *of*. If the lag belongs to the
follower's machinery — the time a carried
difference needs, wherever it starts, to change a
spike — then this window's parting opens at
2010 + 83 = **2093**, and the row gains its first
portable number. If the lag belongs to the
follower's state — if a parting waits for a tender
place, and 1047 and 1132 were both simply where
the next tender place stood — then the parting
opens at some tick no arithmetic on 2010 can
name, and the row learns that its addresses must
be measured, never computed.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before
the window [0, 2,000), inside it [2,000, 3,000),
after it [3,000, 24,000), and in the tails at/after
4,000, 6,000, and 10,000. The **rejoin tick** is
the first tick at or after 3,000 from which the
two trains agree to the season's end; if
disagreements run to the last tick it reads 24,000
and no rejoin was observed; if the trains never
disagree it reads 3,000.

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
  0.0 only on ticks in [2,000, 3,000); every line
  kept, exact outside the window. The seed's
  spelling. It touches exactly twenty-nine
  entries, the first at 2010.
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
  2,000.
- **P3 (causality):** no disagreed tick stands
  before the first silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **2010**. Grounds: the spelling touches
  nothing upstream of 2010; the replay machinery
  is exact (P1, and every window before this one).
- **P4 (ignition, under ten-window's law):** the
  window ignites, and the parting does **not**
  heal: disagreed ticks stand at/after tick
  10,000, and the last disagreed tick stands
  at/after 23,000. Grounds: the ten window's law
  — *lose nothing* — was measured on a window that
  covered no tender tick in the flesh; a window
  after the tender tick is the same case at a
  plainer address. If this prediction fails — if
  a parting this late heals, or never opens —
  then position in the season matters after all,
  and the law gains a boundary nobody has mapped.
- **P5 (the seed's question, answered for the
  fixed lag):** the parting opens at exactly tick
  **2093** — the first taken kick (2010) plus the
  ten window's measured lag (83). Grounds: one
  lag is all the row has; if the lag is a property
  of the machinery, it is portable, and 2093 is
  where portability puts it. If this prediction
  fails, the lag was a wait, not a distance — the
  parting surfaced at the follower's next tender
  place, and tender places are found by walking
  the field, not by adding 83 to anything.
- **P6:** the windowed-dropped journal replays to
  the windowed-silenced train *exact* — spurless'
  law in window form: a 0.0 entry is no entry,
  inside a window as anywhere.
- **P7:** the windowed animal is *not* the
  baseline animal inside the window: at least one
  tick inside [2,000, 3,000) disagrees with the
  baseline train. Same diet, different body — it
  entered the silence in the truth's state.
- **P8:** writing nothing inside a window costs
  full paper: the windowed-silenced book counts
  the signed book's 3,927 bytes; the
  windowed-dropped book saves exactly **87
  bytes** — twenty-nine landings at 3 bytes each,
  counted in the survey before planting and
  asserted in the flesh by test.
- **P9 (the wreck's shape):** the fifth burned
  season stands in its family's band: total
  disagreed ticks in **1,500..=1,700** (the four
  windows before it left 1,596, 1,572, 1,587,
  1,593) and its perfect count in **70..=86**
  (they read 82, 82, 73, 76; the truth reads 83).
  Grounds: every silence in this row so far has
  left the same wreck whatever its length, count,
  or address; a wreck outside the band would be
  the first sign that address prices the damage,
  not only the surfacing.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before 2010 parts
from the truth before 2010 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P4 fails, *lose
nothing* was a local law: it held around the
season's first tender country and does not reach
a window standing clear of it, and the law must
be re-drawn with a boundary. If P5 fails, the
83-tick lag is not portable: a parting's opening
is set by the follower's state, and the only way
to know where a silence will surface is to stand
in the field and watch.

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
- **The window, counted:** road B lands 59 times
  before tick 2,000, exactly **29** times inside
  the window — first at **2010**, last at **2970**,
  as the survey counted — and 614 times at/after
  3,000. Twenty-nine landings. Eighty-seven bytes
  of silence, written as nothing.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact.** No disagreed tick stands
  before 2,000. A spelling exact before the window
  is exact before the window.
- **P3 HELD, entire.** No disagreed tick stands
  before 2010, the first silenced landing. The
  fence held at the new address as at every
  address before it.
- **P4 HELD — the law reaches this address.** The
  window ignited, after the tender tick, on
  ordinary road, and the parting did not heal:
  **1,519** disagreed ticks in all, **961** of
  them at/after 10,000, the last at **23,992** —
  the long window's and the short window's own
  last tick — and the rejoin tick reads 23,993.
  *Lose nothing* is not a local law of the
  season's first country. It stands at 2,000 as
  it stood at 1,040.
- **P5 CRACKED — the lag is not a distance.** The
  parting opens at tick **2140**, not 2093. The
  lag behind the first taken kick is **130
  ticks** here, against the ten window's 83 — and
  the long window's was 32 (taken 1015, surfaced
  1047). Three windows, three lags: 32, 83, 130.
  No one of them is the others' rule. The parting
  did not keep a distance behind the kick; it
  waited — through four more silenced landings
  (2010, 2044, 2078, 2112) — for the follower's
  next tender place, wherever that stood, and
  surfaced there. A surfacing cannot be computed
  from the kick that caused it. It can only be
  measured, in the field, after the fact.
- **P6 HELD, exact.** The windowed-dropped
  journal — 1,280 entries where the silenced book
  keeps 1,309 — replays to the windowed-silenced
  train spike for spike. A 0.0 entry is no entry,
  in after-window form as in every form before it.
- **P7 HELD, plainly.** Inside the window the
  windowed train disagrees with the baseline on
  **70** ticks — no single-tick margin this time;
  in a thousand ticks the third animal and the
  baseline animal are easy to tell apart.
- **P8 HELD, exact.** The windowed-silenced book
  counts the full **3,927 bytes**; the
  windowed-dropped book counts 3,840 — a saving
  of exactly **87 bytes**, twenty-nine landings
  at 3 bytes each, as surveyed. Silence after the
  tender tick costs full paper too. Absence saves
  eighty-seven bytes and loses the season.
- **P9 HELD, in the band — with a crown on it.**
  The wreck reads **1,519** disagreed ticks (62
  inside the window, 1,457 after it), inside the
  declared 1,500..=1,700 and below all four
  earlier wrecks (1,572–1,596) — and its perfect
  count reads **86/112**, inside the declared
  band's top and **above the truth's own 83**:
  count slips 11, phase slips 15, episodes 9,
  longest **27** against the truth's 16, mean
  9.6. The instrument crowns this burned season
  highest of any spelling in the row — higher
  than the season it misremembers. The row has
  been told before, and tells itself again: a
  score is not the counted thing.
- **The counts nearly heal; the train does not —
  inside the window itself, this time.**
  Follower spikes total **856** against the
  truth's 857. By region, windowed against live:
  before the window 72 = 72; inside it **35 =
  35**; after it 749 against 750. Inside the
  window the two trains fire the same number of
  spikes and disagree on **62 ticks** doing it.
  The tally heals in the very stretch where the
  parting opens. Count the train, not the tally.
  12/12 tests (the after-window season pinned as
  found; the spellings' boundaries, the
  twenty-nine-landing survey, and the causality
  fence asserted exact).

**The law, nineteenth under the Scarecrow's
name:** a parting's opening cannot be computed
from the kick that caused it. The row has now
measured three lags behind a first taken kick —
32 ticks (the long window), 83 (the ten), 130
(this one) — and no lag repeats, because the lag
was never a distance the machinery keeps. It is a
wait for the follower's next tender place, and a
tender place is a property of the state the field
has reached, not of the calendar: 1047 was one,
1132 was one, 2140 is one, and none of them could
have been named by adding a number to the kick
before it. So the journal's rule stands where
ten-window left it — *lose nothing* — and gains
its corollary: when a line is lost anyway, do not
ask arithmetic where the season will part. Stand
in the field and watch. The surfacing is found,
never forecast.

**Next seeds:** a late window (10,000–19,000)
still stands: in the season's second half, does
a window still burn the rest of the year, or
does a short remaining season forgive what a
long one could not — and if it parts, is its lag
behind its first taken kick a fourth number, or
does the row's list of waits (32, 83, 130) gain a
pattern late? The tick-64 question in window
form still stands, from whole-kicks: a journal
coarse only inside one chosen window — does any
parting wait at the window's edge, or only where
the difference first changes a spike? And a new
one, priced by this run: the surfacing at 2140
stood after four silenced landings and before
the fifth (2146) — is a tender place a place the
*truth's* train can name in advance (a spike
pattern in the live season), or only a place the
difference reveals by surfacing there?
