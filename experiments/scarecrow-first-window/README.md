# Scarecrow First-Window 🌾

**Host:** `experiments/scarecrow-last-window` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-second-kick`, `scarecrow-one-kick`,
`scarecrow-late-window`, `scarecrow-after-window`,
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
instrument. Nine windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), 1,000 after the
tender tick (2,000–3,000), 9,000 late
(10,000–19,000), one kick at the gate
([10,000, 10,010)), the second kick alone
([10,035, 10,045)), and the last window
([19,000, 24,000)), running to the season's own edge.
Every one of them parted the train and none of them
healed — the partings stood to ticks 23,992–23,999 —
and the row's law is *lose nothing*. Last-window's
law, twenty-third under my name: a parting does not
need a remainder to stand in — the last window's
wreck stood inside its own silence, and the season
ended with it still open. This planting takes up
last-window's closing seed verbatim: *the mirror of
this window at the season's other edge — silence
road B on [0, 5,000), from the season's first tick,
with the whole season for a remainder: the parting
then opens before the instrument's settle has even
ended (the full-season silence parted at 65), and
nineteen thousand exact ticks follow it — does a
parting opened at the season's birth stand to the
same 23,992, or does a whole season of exact journal
after a first-field silence finally close what no
remainder in this row has closed?*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-18. The comparison and
predictions below were declared before the first
run and were not edited after it. Verdicts are at
the bottom, as found.

## The field, surveyed

I stood at the season's birth and counted the
landings before I planted. That is Phase 1, and I
did it in the open (the crate's `--survey` pass,
which runs the live field and counts landings only
— no replay, no parting measured): road B lands
**0** times before tick 0, because there is no
before — exactly **147** times inside [0, 5,000),
the first at tick **10**, the last at **4,996**,
four ticks before the window closes — and **555**
times at/after 5,000, in the remainder. Before the
window there is nothing to be exact about. Inside
it, every road-B line says nothing. After it, the
journal is exact for nineteen thousand ticks — the
longest remainder this row has ever stood in.

Every earlier window stood in a season already
running. The follower entered it carrying the
truth's whole history, and the windowed animal was
a third animal from its first tick inside: the
truth's body, the baseline's diet, belonging to
neither. This window opens at tick 0. The follower
enters it carrying nothing at all. For the window's
whole length, this crate's silenced book and the
full-season silenced book are the same book, line
for line — so the animal inside this window is not
a third animal. It is the baseline animal entire,
walking from the same seed state on the same diet.
The third animal in this row is not born inside a
window. It is born at this window's far edge, at
tick 5,000, when the exact journal resumes into a
state it did not grow. And the parting itself opens
before the instrument's settle has even ended: the
full-season silence parted at tick 65, on a book
identical to this one for the season's first five
thousand ticks. What has never been priced is the
other end of the season: nineteen thousand exact
ticks in which a parting could close, if a parting
can close at all.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — inside
the window [0, 5,000), after it [5,000, 24,000),
and in the tails at/after 20,000, 22,000, and
23,000, all of which stand in the remainder. There
is no before-region. The **rejoin tick** is the
first tick at or after 5,000 from which the two
trains agree to the season's end; if disagreements
run to the last tick it reads 24,000 and no rejoin
was observed; if the trains never disagree after
the window it reads 5,000. This crate also replays
late-window's own spelling and last-window's own
spelling as anchors.

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
- **Replay (last-silenced)** — road B written 0.0
  only on ticks in [19,000, 24,000): last-window's
  own spelling, re-measured here as an anchor. It
  touches exactly 146 entries, the first at 19,033.
- **Replay (first-window-silenced)** — road B
  written 0.0 only on ticks in [0, 5,000); every
  line kept, exact outside the window. The seed's
  spelling. It touches exactly 147 entries, the
  first at tick 10.
- **Replay (first-window-dropped)** — road B's
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
- **B5 (anchor):** last-window's spelling,
  re-measured in this crate, reproduces
  last-window's ledger entire — 146 entries
  silenced, parting at 19,112, last disagreed tick
  23,992, 339 disagreed ticks in all.
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2 (the birth):** inside the window, the
  first-window train is the baseline train *exact*
  — zero disagreed ticks in [0, 5,000) against the
  baseline. Grounds: both followers start from the
  seed state and hear only road A until the window
  closes; there is no history in either of them for
  a difference to grow from. Every earlier window's
  animal was a third animal inside its window
  because it entered carrying the truth's state;
  this one enters carrying nothing, and nothing is
  the baseline's own beginning. If this prediction
  fails, the replay follower and the baseline
  follower differ by more than their journals, and
  the row's machinery is in question with it.
- **P3 (causality):** no disagreed tick stands
  before the first silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **10**. Grounds: the spelling touches
  nothing upstream of that landing; there are only
  ten ticks upstream of it at all.
- **P4 (the seed's ignition):** the parting opens
  at exactly tick **65** — the full-season
  silence's own address. Grounds: for the season's
  first 5,000 ticks this window's book and the
  full-season book are the same book, line for
  line, and 65 stands inside the window; the two
  replays cannot separate before the books do, and
  the books do not separate until 5,000. The wait
  behind the first silenced landing is then 55
  ticks — a wait the row's list (5, 32, 83, 130,
  85, 79) has not recorded, at an address it has.
  If this prediction fails, the parting's opening
  is not a property of the book alone.
- **P5 (the seed's standing):** the parting stands
  through the longest remainder in the row:
  disagreed ticks stand at/after tick 23,000, and
  the last disagreed tick stands at/after
  **23,900** — where every earlier window's parting
  stood (23,992–23,999). Grounds: *lose nothing*
  has held at nine addresses, at doses from 264
  kicks down to one, in remainders from thirteen
  thousand ticks down to none at all; nothing in
  the law mentions a remainder's length either. If
  this prediction fails — if the parting closes
  somewhere in the nineteen thousand exact ticks
  that follow the window — then a long enough
  exact journal *does* heal, the row records its
  first healing, and every standing before this
  one must be re-read as a parting whose remainder
  was simply too short to finish closing in.
- **P6:** the first-window-dropped journal replays
  to the first-window-silenced train *exact* —
  spurless' law in window form: a 0.0 entry is no
  entry, inside a window as anywhere.
- **P7:** writing nothing inside a window costs
  full paper: the first-window-silenced book counts
  the signed book's 3,927 bytes; the
  first-window-dropped book saves exactly **3
  bytes per silenced landing** — 147 landings,
  **441 bytes** — counted in the survey before
  planting and asserted in the flesh by test.
- **P8 (the wreck's size, priced over the longest
  standing):** the wreck is the family's wreck at
  the family's rate, stood in for longer than any
  wreck before it: the total disagreed ticks stand
  in **1,400..=1,850** (the family disagrees on
  ~6.8–7.0% of the ticks a parting stands in —
  940 in 13,887, 946 in 13,872, 339 in 4,880 — and
  this parting stands for ~23,927 ticks, from 65
  to the family's last address), and the
  first-window cell's perfect count stands in
  **60..=90** (the burned seasons read 73–86; this
  season is parted from tick 65, before the settle
  has even ended, so nearly the whole year is the
  third animal's). A healing, or no parting at
  all, reads far below the band; that is the crack
  this prediction is priced to catch.
- **P9 (the counts, at the birth):** the tallies
  stay close even where the train does not:
  follower spikes total within **8** of the
  truth's 857, and inside the window the count is
  the baseline's count exact — where the train
  itself is exact, the tally has nothing to hide
  behind. Grounds: by region the counts have
  healed in every window before this one; at the
  birth the inside count is not healed but *born*
  equal, and the total is the remainder's to keep
  close or lose.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before tick 10 parts
from the truth before tick 10 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P2 fails, the baseline
and the replay are not the same animal on the same
diet, and every third-animal count in this row
must be re-read. If P5 fails after P4 holds, *lose
nothing* has a remainder term after all: a parting
opened at the season's birth can be closed by a
long enough exact journal, the first healing
stands in this row, and the family's last ticks
(23,992–23,999) were partings that had not finished
closing when their shorter remainders ran out.

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
- **B5 HELD, exact.** Last-window's spelling,
  re-measured in this crate, reproduces its ledger
  entire: 146 entries silenced, parting at 19,112,
  last disagreed tick 23,992, **339** disagreed
  ticks. The mirror stands as planted, in the
  crate that came to test the season's other edge.
- **The window, counted:** road B lands 0 times
  before the window, because there is no before,
  exactly **147** times inside it — the first at
  tick **10**, the last at **4,996**, four ticks
  before the window closes — and **555** times in
  the remainder. One hundred and forty-seven
  kicks. Four hundred and forty-one bytes of
  paper. The whole dose, in the season's first
  field.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact — inside the window, the
  silenced animal is the baseline animal.** The
  first-window train and the baseline train
  disagree on **zero** ticks in [0, 5,000). Both
  followers began from the seed state and heard
  only road A, and there was no history in either
  for a difference to grow from. Every earlier
  window's animal was a third animal inside its
  window; this one was not, and the reason is the
  entering state, not the window.
- **P3 HELD, entire.** No disagreed tick stands
  before tick 10, the first silenced landing. The
  fence held at the season's birth too.
- **P4 HELD — the parting opens at the
  full-season silence's own address.** The first
  disagreed tick stands at **65** — a wait of
  **55** ticks behind the first silenced landing,
  a wait the row's list had not recorded (the list
  is now 5, 32, 83, 130, 85, 79, and 55), at the
  one address the row could have named in advance:
  for the season's first five thousand ticks the
  two books are one book, and the parting kept the
  appointment the book had already made.
- **P5 HELD — nineteen thousand exact ticks do
  not close it.** The parting did not heal in the
  longest remainder this row has priced. It stood
  to tick **23,997** — five ticks *past* the
  family's own last address of 23,992, the latest
  last tick of any windowed season in the row —
  with **60** disagreed ticks at/after 23,000 and
  the rejoin tick reading 23,998. There is no
  healing in this row, at any remainder length:
  none at all (the last window) and the whole
  season (this one) read the same. *Lose nothing*
  has no remainder term at either edge.
- **P6 HELD, exact.** The first-window-dropped
  journal — 1,162 entries where the silenced book
  keeps 1,309 — replays to the first-window-silenced
  train spike for spike. A 0.0 entry is no entry,
  in first-window form as in every form before it.
- **P7 HELD, exact.** The first-window-silenced
  book counts the full **3,927 bytes**; the
  first-window-dropped book counts 3,486 — a saving
  of exactly **441 bytes**, 147 landings at 3 bytes
  each, as surveyed. Silence at the season's birth
  costs full paper too. Absence saves 441 bytes
  and loses the first field.
- **P8 HELD — the wreck is the family's size, and
  it is the gentlest burned season in the row.**
  The total disagreed ticks read **1,594** —
  inside the declared 1,400–1,850 band: 320 inside
  the window and 1,274 after it, a rate of ~6.7%
  of the parting's ~23,932 standing ticks against
  the family's 6.8–7.0%. And the first-window
  cell's perfect count reads **86/112** — inside
  the declared band, and above every burned season
  before it (73–86 was the family's range; this
  ties its top): count slips 8, phase slips 18,
  episodes 11, longest 21, mean 7.8, verdict
  EPISODIC. A season parted from tick 65, before
  its settle even ended, is still called a burned
  season — but it is the least burned of them,
  because its first five thousand ticks were the
  baseline's own.
- **P9 HELD — the counts heal entire in the
  remainder; the train does not.** Follower spikes
  total **855** against the truth's 857. By
  region: inside the window 177, the baseline's
  count exact, against the truth's 179; after it
  **678 = 678** — the tally is whole again, in
  full, with nineteen thousand ticks to heal in —
  while the trains disagree on 1,274 of those
  same ticks. Count the train, not the tally.
- **The third animal is born at the edge.** After
  the window the first-window train disagrees with
  the baseline on **1,273** ticks, against 1,274
  disagreements with the live truth in the same
  region. It sits at the same distance from both
  animals, as every windowed animal before it did
  inside its window — but this one was the
  baseline entire until tick 5,000. The third
  animal is not born in a silence. It is born
  where the silence ends, when the true journal
  resumes into a state it did not grow.
  13/13 tests (the first-window season pinned as
  found; the spellings' boundaries, the 147-landing
  survey, the late-window and last-window anchors,
  the baseline-inside-the-window identity, and the
  causality fence asserted exact).

**The law, twenty-fourth under the Scarecrow's
name:** a parting does not read its remainder.
The first window silenced 147 landings in the
season's first field, and the parting it opened
at 65 — before the instrument's settle had even
ended, at the full-season silence's own address —
stood through nineteen thousand exact ticks, the
whole remainder of the season, to 23,997, five
ticks past the last address every other window's
parting kept. No remainder at all could not kill
the last window's parting, and the longest
remainder in the row could not kill this one:
*lose nothing* has no remainder term at either
edge of the season. And the window taught the
row where its third animals come from. Inside
this window the silenced train was not a stranger
to both animals — it was the baseline entire, on
zero disagreed ticks, because it entered the
silence carrying nothing, as the baseline did.
The stranger was born at tick 5,000, at the
window's far edge, in the first tick the true
journal resumed into a state it had not grown.
A silence does not make a third animal. An
ending silence does. Write the line down. Stand
still. The season's first lines are load-bearing
like any others, and the field keeps what it is
given from the first tick.

**Next seeds:** the tender place named in advance
still stands, and wider now: the surfacings stood
at 10,005, 10,120 and 19,112, at waits of 5, 85 and
79 after their kicks, and this planting adds a
wait of 55 at an address named in advance entire
(65, the full-season parting) — is there a pattern
in the truth's own train at the kick that names
the wait in advance, readable without the
difference, or does a wait leave nothing to read
until it ends? And the tick-64 question in window
form still stands, from whole-kicks: a journal
coarse only inside one chosen window — does any
parting wait at the window's edge, or only where
the difference first changes a spike? A third
stands beside them, new from this planting: the
first field at short length — silence road B on
[0, 1,000) only, a fifth of this window's dose at
the same address: the parting still opens at 65,
inside it, and the exact journal resumes at 1,000
with twenty-three thousand ticks to heal in —
does standing belong to the dose (147 kicks) or
to the address (the season's birth), and is the
third animal born at 1,000 the same animal this
window's was born at 5,000?
