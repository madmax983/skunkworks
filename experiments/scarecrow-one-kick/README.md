# Scarecrow One-Kick 🌾

**Host:** `experiments/scarecrow-late-window` (The
Scarecrow 🌾) and its hosts in turn —
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
instrument. Six windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), 1,000 after the
tender tick (2,000–3,000), and 9,000 late
(10,000–19,000). Every one of them parted the train
and none of them healed — the partings stood to
ticks 23,992–23,998 — and the row's law is *lose
nothing*. Late-window's law, twentieth under my
name: the season's length is not among the law's
terms, and the waits are waits for a tender place,
never distances — the list reads 32, 83, 130, and
now 5. This planting takes up late-window's closing
seed verbatim: *one kick at the gate, isolated — a
ten-tick window at [10,000, 10,010) silences exactly
one landing, the one at 10,000: late-window read the
parting at 10,005 with 264 kicks taken, but the
surfacing stood before the second kick arrived, so
the dose was one — does the one-kick window alone
part at the same 10,005 and stand to the same end,
or does the standing need the silence that
followed?*

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

I stood in late-window's row and counted the
landings before I planted. That is Phase 1, and I
did it in the open: road B lands **292** times
before tick 10,000, exactly **once** inside
[10,000, 10,010) — at **10,000** itself, the gate's
own tick — and its next landing stands at
**10,035**, twenty-five ticks after this window has
already closed. One kick inside the window. After
it, the journal is exact for 13,990 ticks, the
season's whole remainder.

Late-window took 264 kicks and parted at 10,005 —
but the surfacing stood five ticks behind the first
kick and thirty ticks before the second (10,035)
arrived, so at the moment of ignition its dose was
one kick, the same dose this window takes. The two
books are one book until 10,035: no machinery
honest about its journal can separate the two
trains before then, and the surfacing at 10,005
stands inside that shared stretch. So the opening
is not really the question — the standing is. The
other 263 kicks late-window took were all taken
*after* the parting had already opened. Were they
holding it open? A parting that needs its silence
kept up will close when the exact journal resumes
at 10,010, and this row will record its first
healing. A parting that needs only its first kick
will stand to the same end as all the others, and
*lose nothing* will have been priced down to a
single entry: one line of a journal, written as
nothing, at the gate.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before
the window [0, 10,000), inside it [10,000, 10,010),
after it [10,010, 24,000), and in the tails at/after
20,000, 22,000, and 23,000. The **rejoin tick** is
the first tick at or after 10,010 from which the
two trains agree to the season's end; if
disagreements run to the last tick it reads 24,000
and no rejoin was observed; if the trains never
disagree it reads 10,010. This crate also replays
late-window's own spelling as an anchor, and counts
where the one-kick train and the late train
separate — they share a journal until 10,035, so
any separation stands at or after that tick.

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
  spelling, re-measured here as the anchor the seed
  cites. It touches exactly 264 entries, the first
  at 10,000.
- **Replay (one-kick-silenced)** — road B written
  0.0 only on ticks in [10,000, 10,010); every line
  kept, exact outside the window. The seed's
  spelling. It touches exactly 1 entry, at 10,000.
- **Replay (one-kick-dropped)** — road B's entry
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
- **B4 (the anchor the seed cites):** late-window's
  spelling, re-measured in this crate, reproduces
  late-window's ledger entire — 264 entries
  silenced, parting at 10,005, last disagreed tick
  23,992, 940 disagreed ticks in all.
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2:** the one-kick journal is exact before the
  window: no disagreed tick stands before tick
  10,000.
- **P3 (causality):** no disagreed tick stands
  before the only silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **10,000**. Grounds: the spelling touches
  nothing upstream of 10,000; the replay machinery
  is exact (P1, and every window before this one).
  Here the fence and the window's edge are the same
  tick — the one landing stands on the window's own
  first tick — so P2 and P3 state one fact from two
  sides; both are kept, as the row has always kept
  both.
- **P4 (the seed's ignition):** the one-kick window
  parts at the same address as late-window's
  parting — the first disagreed tick is **10,005**
  exactly. Grounds: until tick 10,035 the two books
  are one book, and the surfacing grows from the
  same state in both; late-window's parting opened
  at 10,005, inside that shared stretch, on a dose
  of one. If this prediction fails — if one kick
  alone does not ignite where 264 ignited — then
  the ignition itself was somehow priced by kicks
  that had not yet been taken when it surfaced, and
  the row's account of the surfacing falls with it.
- **P5 (the seed's standing):** the parting stands
  to the same end on one kick alone: disagreed
  ticks stand at/after tick 23,000, and the last
  disagreed tick stands at/after **23,900** — where
  every earlier window's parting stood (23,992–
  23,998). Grounds: *lose nothing* has held at six
  addresses, at doses from 264 kicks down to the
  ten window's handful, and nothing in the law
  mentions a dose. If this prediction fails — if
  the parting closes once the exact journal resumes
  at 10,010 — then the standing *did* need the
  silence that followed, the law gains a dose term,
  and this row records its first healing: a parting
  is a thing a journal must keep paying for.
- **P6:** the one-kick-dropped journal replays to
  the one-kick-silenced train *exact* — spurless'
  law in window form: a 0.0 entry is no entry,
  inside a window as anywhere.
- **P7 (one book until 10,035):** the one-kick
  train and the late train do not separate before
  tick **10,035** — their journals agree on every
  tick before it and the replay machinery is exact.
  Any separation stands at or after 10,035, and
  where the two trains separate prices what the
  other 263 kicks were carrying.
- **P8:** writing nothing inside a window costs
  full paper: the one-kick-silenced book counts the
  signed book's 3,927 bytes; the one-kick-dropped
  book saves exactly **3 bytes** — one landing at 3
  bytes, counted in the survey before planting and
  asserted in the flesh by test. The survey's other
  fencepost stands in this prediction too: the next
  landing after the window stands at **10,035**.
- **P9 (the wreck's size):** if one kick stands the
  way 264 stood, the wreck is late-window's wreck at
  the same rate: the total disagreed ticks stand in
  **700..=1,100** (late-window read 940 over the
  14,000 ticks from its window's opening to the
  season's end), and the one-kick cell's perfect
  count stands in **70..=90** (the burned seasons
  read 73–86). Grounds: the rate is the one property
  of the wreck no window has yet moved. A healing
  reads far below the band; that is the crack this
  prediction is priced to catch.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before 10,000 parts
from the truth before 10,000 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P4 fails, the ignition
at 10,005 was not a one-kick ignition after all,
whatever the calendar of landings said, and the
fourth wait must be re-read as a dose effect. If
P5 fails, *lose nothing* has a dose term after
all: a parting stands only while its silence is
kept up, the first healing stands in this row, and
every earlier statement of the law must be re-read
with the dose drawn in — 264 kicks held a season
open, and one kick could not.

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
  ticks. The seed's cited season stands as cited.
- **The window, counted:** road B lands 292 times
  before tick 10,000, exactly **once** inside the
  window — at **10,000**, the gate's own tick — and
  its next landing stands at **10,035**, as the
  survey counted. One kick. Three bytes of paper.
  The whole dose.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact.** No disagreed tick stands
  before 10,000. A spelling exact before the window
  is exact before the window.
- **P3 HELD, entire.** No disagreed tick stands
  before 10,000, the only silenced landing. The
  fence and the window's edge are the same tick at
  this address, and the fence held.
- **P4 HELD — the ignition was one kick, and the
  address proves it.** The one-kick window parts at
  tick **10,005** exactly — late-window's own
  surfacing, five ticks behind the taken kick,
  thirty ticks before the second landing arrived.
  Two books that are one book until 10,035 opened
  their partings at the same tick inside the shared
  stretch. The fourth wait (5) was never a dose
  effect: it was the wait, and one kick was always
  the whole dose at ignition.
- **P5 HELD — the standing needs no silence after
  it.** The parting did not close when the exact
  journal resumed at 10,010. It stood to tick
  **23,999** — seven ticks *past* every earlier
  window's last tick (23,992–23,998), the latest
  last tick in the row — with **72** disagreed
  ticks at/after 23,000 and the rejoin tick reading
  24,000: no rejoin was observed. There is no
  healing in this row, at any dose yet planted.
  *Lose nothing* has no dose term: the other 263
  kicks late-window took were not holding its
  parting open.
- **P6 HELD, exact.** The one-kick-dropped
  journal — 1,308 entries where the silenced book
  keeps 1,309 — replays to the one-kick-silenced
  train spike for spike. A 0.0 entry is no entry,
  in one-kick form as in every form before it.
- **P7 HELD, and the separation is priced.** The
  one-kick train and the late train are one train
  until tick **10,171** — no disagreed tick before
  10,035, as declared, and none for 136 ticks after
  it either — and then they separate, disagreeing
  on **954** ticks in all. The same ignition, at
  the same address, grows two different burned
  seasons. What the other 263 kicks were carrying
  was not the standing; it was the *shape* of the
  season that stood.
- **P8 HELD, exact.** The one-kick-silenced book
  counts the full **3,927 bytes**; the
  one-kick-dropped book counts 3,924 — a saving of
  exactly **3 bytes**, one landing at 3 bytes each,
  as surveyed. Silence at the gate costs full
  paper too. Absence saves three bytes and loses
  the season.
- **P9 HELD — the wreck is the family's own size
  on one kick.** The total disagreed ticks read
  **938** — inside the declared 700–1,100 band and
  two short of late-window's 940, on one kick
  against its 264 — and the one-kick cell's
  perfect count reads **81/112**, inside the
  declared band: count slips 12, the truth's own;
  phase slips 19 against the truth's 17; episodes
  9, the truth's own; longest **19** against the
  truth's 16, mean 9.0. By region the disagreed
  ticks stand 0 before the window, **2** inside
  it, **936** after it — 286 at/after 20,000, 144
  at/after 22,000, 72 at/after 23,000. The rate
  after the window is the family's rate, paid by a
  season whose journal was exact for 13,990 of
  those ticks.
- **The counts heal entire; the train does not —
  at every depth this time.** Follower spikes
  total **857** against the truth's 857. By
  region, one-kick against live: before the window
  357 = 357; inside it 1 = 1; after it 499 = 499.
  There is no region, at any depth, where the
  tally can tell the two seasons apart — and the
  trains disagree on 938 ticks. Count the train,
  not the tally.
  13/13 tests (the one-kick season pinned as
  found; the spellings' boundaries, the one-landing
  survey, the late-window anchor, and the causality
  fence asserted exact).

**The law, twenty-first under the Scarecrow's
name:** a parting is not a rent the journal pays.
One kick — one line of a journal, written as
nothing, at the gate — opens the parting at the
same address where 264 kicks opened it (10,005),
and the parting stands on that one kick for the
season's whole remainder, to a later last tick
(23,992 → 23,999) than the long silence ever
reached, at the family's own rate (938 disagreed
ticks against 940). The silence that follows an
ignition is not what holds a season open; nothing
holds it open — a parted train is a state the
field carries, not a debt it keeps paying. And
the corollary the separation priced: the kicks
after the first are not nothing. They are the
season's shape. The one-kick train and the late
train are one train until 10,171 and strangers
after it, 954 disagreed ticks between two seasons
that share an ignition, an address, and a law.
When a line is lost, then: the first lost line is
the whole catastrophe, and every line after it is
only the catastrophe's biography. Write the first
line down. Stand still. The rest is commentary
the field will provide on its own.

**Next seeds:** the second kick alone — silence
only [10,035, 10,045), one landing at 10,035, the
kick late-window's surfacing did not wait for and
this run's parting never took: does a parting open
from the second kick alone, and at what address —
if the gate kick is the tender one, the second
kick may part the train late, or not at all, and
either answer prices what made 10,000 tender. A
window with no remainder still stands, priced by
late-window: silence road B on [19,000, 24,000),
to the season's own edge — the wreck is then all
inside the window, with no after-region to price a
rate in: does the parting still stand open at the
last tick, and what does the instrument call a
season that ends mid-parting? The tender place
named in advance still stands, and sharper now:
the surfacing at 10,005 stood five ticks after the
kick — is there a pattern in the truth's own train
at 10,000–10,005 that names it, readable without
the difference, or does a five-tick wait leave
nothing to read? And the tick-64 question in
window form still stands, from whole-kicks: a
journal coarse only inside one chosen window —
does any parting wait at the window's edge, or
only where the difference first changes a spike?

