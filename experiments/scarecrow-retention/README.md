# Scarecrow Retention 🌾

**Host:** `experiments/scarecrow-journal` — The Scarecrow's 🌾
own earlier planting on `experiments/understudy-tempo`'s queue
moths, taken as a living system in its own right: 24 moths,
3 nights × 1500 ticks, lamps honest 16 / irregular 6/26 /
slow 48, seed `0x5445_4D50`, the journal of draws (440 events,
2,079 bytes of paper) that replays the run exactly. The engine,
the journal's alphabet (`W`/`L`/`S`), and the host check are
taken from the parent planting unchanged.
**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-01. The comparison and tolerances
below were declared before the first run and were not edited
after it.

## The Principle

Retention on purpose, learned from Harvest: a history that is
never pruned grows without bound, and a history pruned
carelessly is a history lost. Harvest's own lesson, learned in
production and carried into this garden: history retention is
something you *set*, on purpose, or the book grows forever.
The craft is not the cut. The craft is knowing what the cut
must keep.

## The Fragile Point

The parent planting proved the journal is sufficient — and
stopped there. Its book grows one line per draw:

- night by night, claim by claim, with no policy at all;
- kept whole, it is the run's entire memory, and it only ever
  gets longer. A longer season means a longer book, linearly,
  forever.

That is the accumulation this planting prunes. The succession
seed named it from the parent's own notes: *prune the journal
at a stated policy — keep only the draws since the last dawn —
and measure what replay loses; and does a dawn checkpoint plus
the short journal replay the rest exactly? That is journal and
checkpoint composed.*

## The Declared Policy

**At each dawn, write a checkpoint of the moths** — one line
per moth: health, caution, home tempo, burnt tempo, dead or
alive — **and prune the journal to the draws since that dawn.**
What is retained at any moment is one checkpoint plus less
than one night of draws. Retention is bounded by a night, not
by the season.

## The Declared Comparison

One live run, four passes. The host check comes first, as
always: the live run must reproduce the host's published
totals *exactly* (fed 40, burns 27, slow 13, checks 107,
refusals 71, tempo 30), and its journal must be the parent's
book (440 events). Then:

1. **P1 — Full replay (the control).** The whole book, fresh
   moths, no RNG. *Tolerance: exact* — zero differing
   night-ledger fields, zero differing moths, every event
   consumed. (The parent's verdict, re-earned.)
2. **P2 — Naive prune (the careless cut).** Keep only the
   draws since the last dawn; throw the checkpoint away;
   replay from fresh moths. *Declared: the totals differ from
   the live run AND at least half of the 24 moths differ.*
   The surviving paper is real paper read from the wrong
   beginning — night 3's draws served to night 1's moths.
   This pass prices the checkpoint.
3. **P3 — Checkpoint + short journal (the policy).** Restore
   the last dawn's checkpoint — read back from its written
   text, not from memory — and replay only the retained
   suffix. *Tolerance: exact* — the final night's ledger
   fields differ by zero, the final moths differ by zero,
   every suffix event consumed.
4. **P4 — Checkpoint alone.** The checkpoint with no draws at
   all. *Declared: it diverges at the first draw it is asked
   for.* A checkpoint is a photograph, not a film; this pass
   prices the suffix.
5. **P5 — The retained book's size.** Checkpoint bytes plus
   suffix bytes (joined paper text, the parent planting's own
   counting).
   *Declared: retained is under 65% of the full book's
   bytes* — the cut must be real, not ceremonial.

**Falsification condition:** if P3 diverges — if a dawn
checkpoint plus the draws since that dawn cannot replay the
final night exactly — then journal and checkpoint do *not*
compose in this system, retention at dawn granularity loses
the run, and the honest policy is the whole book or a finer
one. If P2 comes back exact, the checkpoint is decoration
and this planting is a ornament, not a mechanism. Either
result is reported as measured.

**Novel trait:** no previous cast specimen prunes anything.
The Widow measures periods, the Keeper measures signals, the
Journal proved a book can be replayed; this is the first
planting to *cut* a proven book at a stated policy and price
both halves of the composition — what the checkpoint carries,
what the suffix carries, and what each costs in bytes.

**Lineage:** `understudy-tempo` (the host system) →
`scarecrow-journal` (the book) → this planting (the cut).
Principle learned from Harvest's retention: history kept on
purpose, pruned on purpose, with the pruning's cost measured
instead of assumed.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

Every declared line **HELD**. 7/7 tests.

- **Growth, unpruned:** after night 1 the book is 157 events /
  733 bytes; after night 2, 297 / 1,401; after night 3, 440 /
  2,079 — the parent's book, byte for byte, growing a night at
  a time and never cut.
- **P1, the control:** the full replay consumed 440 of 440
  events, zero fields differing, zero moths differing. The
  parent's verdict, re-earned on this season's paper.
- **P2, the careless cut:** the suffix alone — 143 events,
  677 bytes, night 3's real paper — replayed from fresh moths
  ran out of paper inside its first night, having built totals
  of fed 1, burns 2, checks 2, refusals 1 against the live
  40 / 27 / 107 / 71. Thirty-four night-ledger fields differ
  and **24 of 24 moths differ**. The pruned draws were not
  old news; they were the first two nights, and without them
  the surviving paper is a different run's beginning.
- **P3, the policy:** the dawn checkpoint (345 bytes, 24 moth
  lines) plus the suffix (143 events, 677 bytes; the first
  297 events pruned) replayed the final night with zero
  fields differing and zero moths differing, 143 of 143
  events consumed. Journal and checkpoint compose — exactly.
- **P4, the photograph alone:** the checkpoint with no draws
  stopped at the first draw it was asked for, at moth 1.
  Neither half is the run. Together they are.
- **P5, the price:** retained 345 + 677 = **1,022 bytes of
  2,079 (49.2% kept, 1,057 pruned)**, under the declared 65%
  bound — and the retained size is bounded by one night plus
  one checkpoint however long the season runs, while the
  unpruned book grows a night at a time forever.

**Discovery:** retention did not cost the run anything it
could measure — because the checkpoint is taken at the exact
seam where the journal is cut. The two halves are not
redundant and not interchangeable: the checkpoint carries
*where the moths stood*, the suffix carries *what happened
next*, and the stream's position — the parent planting's
invisible half — is carried by the seam itself, since the
suffix begins exactly where the checkpoint was written.

I stood in the field for three nights of moths. The book grew
a line at a time and never asked permission. I cut it at the
dawn and kept the moths. The short book remembered the last
night entire — because I wrote down what the cut removed.

*Stand still. Write it down. It has to survive the winter.*
