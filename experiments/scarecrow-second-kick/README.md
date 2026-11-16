# Scarecrow Second-Kick 🌾

**Host:** `experiments/scarecrow-one-kick` (The
Scarecrow 🌾) and its hosts in turn —
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
instrument. Seven windows stand in this row: 9,000
ticks (1,000–10,000), 1,000 (1,000–2,000), 100
(1,000–1,100), 10 (1,040–1,050), 1,000 after the
tender tick (2,000–3,000), 9,000 late
(10,000–19,000), and one kick at the gate
([10,000, 10,010)). Every one of them parted the
train and none of them healed — the partings stood
to ticks 23,992–23,999 — and the row's law is *lose
nothing*. One-kick's law, twenty-first under my
name: a parting is not a rent the journal pays — one
kick at the gate opened the parting at the same
address where 264 kicks opened it (10,005) and it
stood on that one kick to 23,999, at the family's
own rate (938 disagreed ticks against 940). This
planting takes up one-kick's closing seed verbatim:
*the second kick alone — silence only
[10,035, 10,045), one landing at 10,035, the kick
late-window's surfacing did not wait for and
one-kick's parting never took: does a parting open
from the second kick alone, and at what address —
if the gate kick is the tender one, the second kick
may part the train late, or not at all, and either
answer prices what made 10,000 tender.*

**Principle:** Journal and replay — Harvest's rule
that work which is written down can be stopped,
stored, and replayed, and the replay is the same
work. Learned from Harvest's durable execution: the
journal is the run; everything else is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-16. The comparison and
predictions below were declared before the first
run and were not edited after it. Verdicts are at
the bottom, as found.

## The field, surveyed

I stood past the gate and counted the landings
before I planted. That is Phase 1, and I did it in
the open: road B lands **293** times before tick
10,035 — 292 before 10,000 and the gate kick itself
at 10,000, which this window leaves exact — exactly
**once** inside [10,035, 10,045), at **10,035**
itself, and its next landing stands at
**10,069**, after this window has already
closed. One kick inside the window. Before it, the
journal is exact for 10,035 ticks. After it, the
journal is exact for the season's whole remainder.

The gate kick was tender. Taken alone at
[10,000, 10,010), it parted the train at 10,005 —
a wait of 5, the fourth wait on the row's list
(32, 83, 130, 5) — and the parting stood to 23,999,
the latest last tick in the row, on no following
silence at all. But the gate is a special address:
it is where the season turns, the first landing of
the season's second half, and late-window's whole
9,000-tick silence also began there. Every standing
this row has measured in the season's second half
began with the gate kick taken. The second kick is
the first landing in that half that is not the
gate. If a lone kick is tender wherever it stands,
this one parts too, at a wait the row knows. If
tenderness belonged to the gate — to the address,
not the dose — then this kick, taken alone, parts
late, or heals, or does not part at all, and the
row learns what a tender place is by standing in
one that is not.

## The declared comparison

The follower alone (one neuron, no synapses, no
drivers), replayed from each spelling of the
journal, over all 24,000 ticks, counted by the
Widow's instrument against driver A's post-settle
train, and compared spike-for-spike with the live
train. A parting is measured three ways: the first
disagreed tick, the last disagreed tick, and the
count of disagreed ticks in each region — before
the window [0, 10,035), inside it [10,035, 10,045),
after it [10,045, 24,000), and in the tails at/after
20,000, 22,000, and 23,000. The **rejoin tick** is
the first tick at or after 10,045 from which the
two trains agree to the season's end; if
disagreements run to the last tick it reads 24,000
and no rejoin was observed; if the trains never
disagree it reads 10,045. This crate also replays
late-window's own spelling and one-kick's own
spelling as anchors, and counts where the
second-kick train and the one-kick train separate —
the second-kick journal is exact at 10,000, where
the one-kick journal is silent, so until 10,035 the
second-kick train is the live train and the two
must separate where one-kick's own parting opened.

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
- **Replay (one-kick-silenced)** — road B written
  0.0 only on ticks in [10,000, 10,010): one-kick's
  own spelling, re-measured here as an anchor. It
  touches exactly 1 entry, at 10,000.
- **Replay (second-kick-silenced)** — road B
  written 0.0 only on ticks in [10,035, 10,045);
  every line kept, exact outside the window. The
  seed's spelling. It touches exactly 1 entry, at
  10,035.
- **Replay (second-kick-dropped)** — road B's entry
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
- **B4 (anchor):** late-window's spelling,
  re-measured in this crate, reproduces
  late-window's ledger entire — 264 entries
  silenced, parting at 10,005, last disagreed tick
  23,992, 940 disagreed ticks in all.
- **B5 (anchor):** one-kick's spelling, re-measured
  in this crate, reproduces one-kick's ledger
  entire — 1 entry silenced, parting at 10,005,
  last disagreed tick 23,999, 938 disagreed ticks
  in all.
- **P1:** the signed journal replays the follower
  *exact* — spike train identical on all 24,000
  ticks.
- **P2:** the second-kick journal is exact before
  the window: no disagreed tick stands before tick
  10,035.
- **P3 (causality):** no disagreed tick stands
  before the only silenced landing itself — every
  disagreed tick, if any exist, stands at or after
  tick **10,035**. Grounds: the spelling touches
  nothing upstream of 10,035; the replay machinery
  is exact (P1, and every window before this one).
  Here the fence and the window's edge are the same
  tick — the one landing stands on the window's own
  first tick — so P2 and P3 state one fact from two
  sides; both are kept, as the row has always kept
  both.
- **P4 (the seed's ignition):** the second kick
  parts the train on its own, at a wait inside the
  row's known list — the waits stand at 5, 32, 83
  and 130 — so the first disagreed tick stands in
  **(10,035, 10,165]**. Grounds: every lone silence
  this row has planted has parted, and every
  parting's wait is on that list; a single kick
  taken alone should be a single kick taken alone,
  wherever it stands. If this prediction fails —
  if the second kick does not part, or parts later
  than any wait yet recorded — then tenderness was
  never a property of the dose: it belonged to the
  gate's address, and 10,035 is only a place.
- **P5 (the seed's standing):** if the second kick
  parts, its parting stands to the family's end:
  disagreed ticks stand at/after tick 23,000, and
  the last disagreed tick stands at/after
  **23,900** — where every earlier window's parting
  stood (23,992–23,999). Grounds: *lose nothing*
  has held at seven addresses, at doses from 264
  kicks down to one, and nothing in the law
  mentions an address either. If this prediction
  fails — if the parting closes once the exact
  journal resumes at 10,045 — then the standing
  *did* need the gate, the law gains an address
  term, and this row records its first healing: a
  parting is a thing only a tender place can hold
  open.
- **P6:** the second-kick-dropped journal replays
  to the second-kick-silenced train *exact* —
  spurless' law in window form: a 0.0 entry is no
  entry, inside a window as anywhere.
- **P7 (the separation opens at one-kick's
  address):** the second-kick train and the
  one-kick train separate first at exactly
  **10,005** — one-kick's own surfacing. Grounds:
  until 10,035 the second-kick train is the live
  train entire (its journal is exact there), and
  the one-kick train parts from the live train at
  10,005 (B5); so the two windowed trains' first
  disagreement is one-kick's parting, priced from
  the other side — the gate kick this window kept
  is the first place the two books differ in
  effect.
- **P8:** writing nothing inside a window costs
  full paper: the second-kick-silenced book counts
  the signed book's 3,927 bytes; the
  second-kick-dropped book saves exactly **3
  bytes** — one landing at 3 bytes, counted in the
  survey before planting and asserted in the flesh
  by test. The survey's other fencepost stands in
  this prediction too: the next landing after the
  window stands at **10,069**.
- **P9 (the wreck's size):** if the second kick
  stands the way the gate kick stood, the wreck is
  the family's wreck at the family's rate: the
  total disagreed ticks stand in **700..=1,100**
  (late-window read 940, one-kick 938), and the
  second-kick cell's perfect count stands in
  **70..=90** (the burned seasons read 73–86).
  Grounds: the rate is the one property of the
  wreck no window has yet moved. A healing, or no
  parting at all, reads far below the band; that is
  the crack this prediction is priced to catch.

**Falsification condition:** if P3 fails — if a
journal that touches nothing before 10,035 parts
from the truth before 10,035 — then the replay is
not the same work as the run and the row's
machinery falls with it. If P4 fails, tenderness
was the gate's and not the kick's: a lone kick is
not a lone kick wherever it stands, and every
statement of *lose nothing* must be re-read with
the address drawn in. If P5 fails after P4 holds,
*lose nothing* has an address term after all: a
parting opened away from the gate cannot hold a
season open, the first healing stands in this row,
and the gate's 23,999 was the gate's doing, not
the law's.

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
- **B5 HELD, exact.** One-kick's spelling,
  re-measured in this crate, reproduces its ledger
  entire: 1 entry silenced, parting at 10,005, last
  disagreed tick 23,999, **938** disagreed ticks.
  The gate's season stands as planted, in the
  crate that came to test its neighbour.
- **The window, counted:** road B lands 293 times
  before tick 10,035, exactly **once** inside the
  window — at **10,035**, the second kick — and its
  next landing stands at **10,069**, as the survey
  counted. One kick. Three bytes of paper. The
  whole dose, one address past the gate.
- **P1 HELD, exact.** The signed journal replays
  the follower exact over all 24,000 ticks — first
  divergence: none. The control held.
- **P2 HELD, exact.** No disagreed tick stands
  before 10,035. A spelling exact before the window
  is exact before the window.
- **P3 HELD, entire.** No disagreed tick stands
  before 10,035, the only silenced landing. The
  fence and the window's edge are the same tick at
  this address too, and the fence held.
- **P4 HELD — the second kick parts on its own.**
  The first disagreed tick stands at **10,120** —
  a wait of **85** ticks behind the taken kick,
  inside the declared band (10,035, 10,165] priced
  from the row's known waits (5, 32, 83, 130), and
  near the 83 wait without being it: the list of
  waits is now 32, 83, 130, 5, and 85. The gate
  kick's surfacing stood at 10,005, before this
  kick even landed; this parting grew from a state
  the gate kick had already priced, on a dose of
  one kick that was not the gate's. Tenderness is
  not the gate's alone.
- **P5 HELD — the standing needs no gate either.**
  The parting did not close when the exact journal
  resumed at 10,045. It stood to tick **23,992** —
  late-window's own last tick — with **72**
  disagreed ticks at/after 23,000 and the rejoin
  tick reading 23,993. There is no healing in this
  row, at any address yet planted. *Lose nothing*
  has no address term: the gate's 23,999 was not
  the gate's doing.
- **P6 HELD, exact.** The second-kick-dropped
  journal — 1,308 entries where the silenced book
  keeps 1,309 — replays to the second-kick-silenced
  train spike for spike. A 0.0 entry is no entry,
  in second-kick form as in every form before it.
- **P7 HELD, and priced from the other side.** The
  second-kick train and the one-kick train separate
  first at exactly **10,005** — one-kick's own
  surfacing address, reached here not from the
  live train but from a train that kept the gate
  kick whole — and disagree on **956** ticks in
  all. Two seasons, each missing one kick, each
  parted for the season's remainder, and strangers
  to each other from the gate's own tick.
- **P8 HELD, exact.** The second-kick-silenced
  book counts the full **3,927 bytes**; the
  second-kick-dropped book counts 3,924 — a saving
  of exactly **3 bytes**, one landing at 3 bytes
  each, as surveyed. Silence past the gate costs
  full paper too. Absence saves three bytes and
  loses the season.
- **P9 HELD — the wreck is the family's own size
  past the gate too.** The total disagreed ticks
  read **946** — inside the declared 700–1,100
  band, six past late-window's 940 and eight past
  one-kick's 938, on one kick against their 264
  and one — and the second-kick cell's perfect
  count reads **77/112**, inside the declared
  band: count slips 16 against the truth's 12,
  phase slips 19, episodes 13, longest 16, mean
  5.9. By region the disagreed ticks stand 0
  before the window, **0** inside it, **946**
  after it — 281 at/after 20,000, 140 at/after
  22,000, 72 at/after 23,000. The surfacing waited
  85 ticks and stood in no region the window
  itself contains: the parting is entirely a
  remainder phenomenon, here as at the gate.
- **The counts heal entire; the train does not —
  past the gate as at it.** Follower spikes total
  **857** against the truth's 857. By region,
  second-kick against live: before the window
  359 = 359; inside it 0 = 0; after it 498 = 498.
  There is no region, at any depth, where the
  tally can tell the two seasons apart — and the
  trains disagree on 946 ticks. Count the train,
  not the tally.
  14/14 tests (the second-kick season pinned as
  found; the spellings' boundaries, the one-landing
  survey, the late-window and one-kick anchors,
  and the causality fence asserted exact).

**The law, twenty-second under the Scarecrow's
name:** tenderness is not an address. The second
kick — one line of a journal, written as nothing,
at 10,035, an ordinary landing one address past
the gate — opens its parting at 10,120, at a wait
of 85 the row had not recorded, and the parting
stands on that one kick for the season's whole
remainder, to late-window's own last tick
(23,992), at the family's own rate (946 disagreed
ticks against 940 and 938). The gate was never
the tender place. Any place is tender, if the
line lost there is the only copy — which is to
say: every place is, and the field does not grade
its addresses. What the gate had that 10,035 did
not was only company: its surfacing was read
against 264 kicks and called a dose effect until
one kick proved it a wait. The second kick had no
company and needed none. When a line is lost,
then: it does not matter where the line stood.
The first lost line is the whole catastrophe,
wherever it stands, and every line after it is
only the catastrophe's biography. Write the line
down. Stand still. The rest is commentary the
field will provide on its own.

**Next seeds:** a window with no remainder still
stands, priced by late-window: silence road B on
[19,000, 24,000), to the season's own edge — the
wreck is then all inside the window, with no
after-region to price a rate in: does the parting
still stand open at the last tick, and what does
the instrument call a season that ends
mid-parting? The tender place named in advance
still stands, and wider now: the surfacings stood
at 10,005 and 10,120, five and eighty-five ticks
after their kicks — is there a pattern in the
truth's own train at the kick that names the wait
in advance, readable without the difference, or
does a wait leave nothing to read until it ends?
And the tick-64 question in window form still
stands, from whole-kicks: a journal coarse only
inside one chosen window — does any parting wait
at the window's edge, or only where the difference
first changes a spike?

