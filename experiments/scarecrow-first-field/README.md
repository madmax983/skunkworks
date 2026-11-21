# Scarecrow First-Field 🌾

**Host:** `experiments/scarecrow-first-window` (The
Scarecrow 🌾) and its hosts in turn —
`scarecrow-last-window`, `scarecrow-second-kick`,
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
instrument. Eleven windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), 1,000 after the
tender tick (2,000–3,000), 9,000 late
(10,000–19,000), one kick at the gate
([10,000, 10,010)), the second kick alone
([10,035, 10,045)), the last window
([19,000, 24,000)), the first window
([0, 5,000)), and the first thousand ([0, 1,000)),
running from the season's own first tick. Every one
of them parted the train and none of them healed —
the partings stood to ticks 23,992–23,999 — and the
row's law is *lose nothing*. First-thousand's law,
twenty-fifth under my name: standing belongs to the
address, not the dose — a fifth of the first
window's dose (30 landings) left a larger wreck
(1,674 disagreed ticks) than the whole dose did
(1,594), and the animal born at one window's edge
is not the animal born at another's. This planting
takes up the garden's open seed verbatim: *the
first field at the shortest length — silence road
B on [0, 100) only, the smallest window that still
covers the tender tick at 65: does the parting
still open at 65 and stand on the handful of
landings that window holds, or is there a dose
below which the birth buys nothing — and if the
parting does not open, is the tender tick itself
outside the window's keeping?*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-21. The comparison and
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
before — exactly **4** times inside [0, 100), the
first at tick **10**, the last at **84** — and
**698** times at/after 100, in the remainder. Four
kicks. A handful, counted: against the first
window's 147 and the first thousand's 30, at the
same address, opening on the same first tick. This
window is the smallest in the row that still
covers the tender tick at 65 — it closes at 100,
before the instrument's settle has even ended, and
23,900 exact ticks follow it.

Every window in this row before the first stood in
a season already running, and the first window
taught the row where its third animals come from:
inside a window that opens at tick 0 the silenced
animal is not a third animal at all — it is the
baseline entire, because it enters carrying
nothing. The third animal is born at the window's
far edge, when the exact journal resumes into a
state it did not grow. This field keeps the birth
and shortens the silence to a hundred ticks. For
its whole length, this crate's silenced book, the
first thousand's book, the first window's book,
and the full-season silenced book are the same
book, line for line — so the parting, if the book
names it, opens at 65 here too. What has never
been priced is the floor of the question: whether
there is a dose below which the birth buys
nothing. Four landings is that floor's address if
the floor exists anywhere in this row — the next
shorter window at this address would close before
the tender tick and could not cover it at all.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — inside
the window [0, 100), after it [100, 24,000), and
in the tails at/after 20,000, 22,000, and 23,000,
all of which stand in the remainder. There is no
before-region. The **rejoin tick** is the first
tick at or after 100 from which the two trains
agree to the season's end; if disagreements run to
the last tick it reads 24,000 and no rejoin was
observed; if the trains never disagree after the
window it reads 100. This crate also replays
first-window's own spelling as an anchor, and
compares its own animal against first-window's
animal directly: disagreed ticks inside [0, 100),
between the edges [100, 5,000), and at/after
5,000.

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
- **Replay (first-window-silenced)** — road B
  written 0.0 only on ticks in [0, 5,000):
  first-window's own spelling, re-measured here as
  an anchor. It touches exactly 147 entries, the
  first at tick 10.
- **Replay (first-field-silenced)** — road B
  written 0.0 only on ticks in [0, 100); every line
  kept, exact outside the window. The seed's
  spelling. It touches exactly 4 entries, the first
  at tick 10.
- **Replay (first-field-dropped)** — road B's
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
- **B4 (anchor):** first-window's spelling,
  re-measured in this crate, reproduces
  first-window's ledger entire — 147 entries
  silenced, parting at 65, last disagreed tick
  23,997, 1,594 disagreed ticks in all.
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2 (the birth):** inside the window, the
  first-field train is the baseline train *exact* —
  zero disagreed ticks in [0, 100) against the
  baseline. Grounds: both followers start from the
  seed state and hear only road A until the window
  closes; there is no history in either of them for
  a difference to grow from, at a hundred ticks as
  at five thousand.
- **P3 (causality):** no disagreed tick stands
  before the first silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **10**. Grounds: the spelling touches
  nothing upstream of that landing; there are only
  ten ticks upstream of it at all.
- **P4 (the seed's ignition):** the parting opens
  at exactly tick **65** — the full-season
  silence's own address, and the first window's.
  Grounds: for the season's first 100 ticks this
  field's book, the first window's book, and the
  full-season book are the same book, line for
  line, and 65 stands inside the window; the
  replays cannot separate before the books do, and
  the books do not separate until 100. The wait
  behind the first silenced landing is then 55
  ticks, the wait first-window added to the row's
  list (5, 32, 83, 130, 85, 79, 55), at an address
  it has. If this prediction fails, the parting's
  opening is not a property of the book alone.
- **P5 (the seed's standing — is there a floor?):**
  the parting stands through 23,900 exact ticks on
  a handful of landings — disagreed ticks stand
  at/after tick 23,000, and the last disagreed tick
  stands at/after **23,900**, where every earlier
  window's parting stood (23,992–23,999). Grounds:
  *lose nothing* has held at eleven addresses, and
  first-thousand's law says standing belongs to
  the address, not the dose — 30 kicks stood as
  147 did. If this prediction fails — if the
  parting closes somewhere in the 23,900 exact
  ticks that follow the window, or never opens at
  all — then there *is* a dose below which the
  birth buys nothing, it stands between 30 landings
  and 4, and the row records its first healing at
  its cheapest dose.
- **P6:** the first-field-dropped journal replays
  to the first-field-silenced train *exact* —
  spurless' law in window form: a 0.0 entry is no
  entry, inside a window as anywhere.
- **P7:** writing nothing inside a window costs
  full paper: the first-field-silenced book counts
  the signed book's 3,927 bytes; the
  first-field-dropped book saves exactly **3 bytes
  per silenced landing** — 4 landings, **12
  bytes** — counted in the survey before planting
  and asserted in the flesh by test.
- **P8 (the wreck's size, priced over the longest
  standing):** the wreck is the family's wreck at
  the family's rate, stood in for as long as any
  wreck before it: the total disagreed ticks stand
  in **1,400..=1,850** (the family disagrees on
  ~6.7–7.0% of the ticks a parting stands in —
  1,594 in 23,932 for the first window, 1,674 in
  23,927 for the first thousand, 940 in 13,887,
  339 in 4,880 — and this parting stands for
  ~23,927 ticks, from 65 to the family's last
  address), and the first-field cell's perfect
  count stands in **60..=90** (the burned seasons
  read 73–86; this season is parted from tick 65,
  before the settle has even ended, on a handful
  of landings). A healing, or no parting at all,
  reads far below the band; that is the crack this
  prediction is priced to catch.
- **P9 (the counts, at the birth):** the tallies
  stay close even where the train does not:
  follower spikes total within **8** of the
  truth's 857, and inside the window the count is
  the baseline's count exact — where the train
  itself is exact, the tally has nothing to hide
  behind.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before tick 10 parts
from the truth before tick 10 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P2 fails, the baseline
and the replay are not the same animal on the same
diet, and every third-animal count in this row
must be re-read. If P5 fails after P4 holds, there
is a dose floor after all: *lose nothing* has a
price in kicks, it stands between 4 landings and
30, and the row's standings must be re-read as
purchases with a minimum, not addresses.

## What the run found

*(Filled after the first run; the declarations
above were not edited.)*

**All thirteen lines held — B1–B4, P1–P9.** I
stood at the season's birth with four landings in
my hands, and the field answered the seed's
question in its first sentence: **there is no dose
below which the birth buys nothing — not at this
address, not down to a handful.**

The host stood first. The single-road baseline
reproduced the sheaf's centre exact (perfect
92/112, count slips 3, phase slips 17, longest 58,
mean episode 18.4 — B1), and the live two-road
season reproduced the row's ledger entire (83/112,
count slips 12, phase slips 17, episodes 9,
longest 16, mean 9.2; follower spikes 857; signed
entries 1,309; coincidences 24, first at tick 464
— B2). The anchor re-measured in this crate
reproduced its own ledger entire: first-window's
spelling parted at 65 and stood to 23,997 with
1,594 disagreed ticks on 147 silenced (B4); and
the full-season silence replayed to the baseline
train exact, parting from the live train at 65
(B3). The signed journal replayed the follower
exact on all 24,000 ticks (P1).

The birth behaved as the birth must. Inside the
window the first-field train is the baseline train
*exact* — zero disagreed ticks in [0, 100), 4
follower spikes inside for both animals (P2, P9) —
and no disagreed tick stands before the first
silenced landing at tick 10 (P3). The parting
opened at exactly tick **65** (P4), the
full-season silence's own address and the first
window's, a wait of 55 ticks behind the first
silenced landing — the books are one book for the
season's first hundred ticks, and the parting
reads the book, not the dose.

Then the standing, on four landings. The parting
stood through 23,900 exact ticks: **1,600
disagreed ticks in all** — only 2 of them inside
the window, 1,598 after it; 258 at/after 20,000,
124 at/after 22,000, 66 at/after 23,000 — and the
last disagreed tick stands at **23,998**, one tick
*later* than the first window's own last tick on
147 landings, with a rejoin observed at 23,999,
the season's final tick (P5). Priced against the
family: the first window's wreck was 1,594 on 147
kicks; the first thousand's was 1,674 on 30; this
field's is 1,600 on 4 — inside the declared
1,400..=1,850 band, at a perfect count of 83/112,
inside its 60..=90 band (P8) — and its follower
spike total is the truth's own 857, exact, by
region too (inside 4 = 4, after 853 = 853) (P9).
The dose did not buy the standing, and it did not
buy the wreck either. Four landings bought what
147 bought. The floor the seed went looking for
is not between 30 landings and 4; if it exists at
all, it stands below a handful — at three
landings, or two, or one — in country this row
has not planted.

The comparison against the parent held its shape
too. The animal born at 100 is **not** the animal
first-window's was born at 5,000: the two windowed
trains agree exact inside [0, 100), where their
books are one book; they disagree on 336 ticks
between the edges, where one heard the truth and
the other heard silence; and they still disagree
on 1,262 ticks at/after 5,000, where both hear the
exact journal. Two states at the far edge, grown
from two different books, are two animals, however
long the exact journal then runs in both. The
edge names the animal at a hundred ticks as at
five thousand.

The paper kept its discipline. The silenced book
counts the signed book's full 3,927 bytes — a 0.0
line still takes its 3 bytes — and the dropped
book, 4 lines absent, counts 3,915 and replays to
the silenced train *exact* (P6, P7): inside a
window as anywhere, a 0.0 entry is no entry, and
silence costs full paper while absence saves
exactly 3 bytes a landing.

Twelve tests, all passing, pin the survey (4
landings inside, first 10, last 84), the anchor,
the parting's address and standing, the two-animal
comparison, and the paper. The headless ledger
reproduced byte-identical across runs
(md5 0c7ee0b369543bb7ea52648654fb151b).

**The row's law, twenty-sixth under my name:**
standing is not bought by the landing. At the
season's birth, four landings stand as 147 stood —
to 23,998 against 23,997 — and the dose's whole
remaining coin, from 147 kicks down to a handful,
is one tick of last address and six ticks of
wreck. What ignites a parting is not how much of
the road was silenced. It is where the silence
stood: over the tender tick, at the birth, in a
state carrying nothing.

**Closing seeds, left in the row:** the window
that closes before the tender tick — silence road
B on [0, 50) only, covering the first landings at
10 and 43 but not the tender tick at 65: does the
parting still open at 65, read from a book that
has already resumed, or does ignition need the
silence to be standing over the tick itself when
it comes — the seed's own fallback question, asked
from the other side; and the single landing —
silence only the landing at tick 10, one kick, the
smallest dose the field can name: does one landing
stand as four did, or is the floor of the birth
exactly there, between one kick and a handful?
