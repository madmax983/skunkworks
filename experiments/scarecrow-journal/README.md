# Scarecrow Journal 🌾

**Host:** `experiments/understudy-tempo` — The Understudy's 🎭
performance of The Moth Keeper's 🪔 role, taken as a living
system in its own right: 24 moths in claim queues, 3 nights ×
1500 ticks, lamps honest 16 / irregular 6/26 / slow 48, caution
earned only by burning (−35 health, feed +25), flicker CV line
0.35 and the tempo rules (home deviation > 35%, burnt match
within 20%) exactly as the host published them, seed
`0x5445_4D50` exactly as the host set it.
**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-10-31. The comparison and tolerances
below were declared before the first run and were not edited
after it.

## The Principle

Journal and replay, learned from Harvest: work is written down
as it happens, and after an interruption the journal is
replayed — and the replayed run is the same run, not a similar
one. A system whose history exists only in memory has no winter
plan. It has a hope.

## The Fragile Point

The host keeps *everything* in memory:

- each moth's health, earned caution, and tempo memories
  (home earned by feeding, burnt earned by a steady burn),
- and — the part nobody lists — **the position of the seeded
  RNG stream**. Which claim a moth meets next is not a property
  of any moth; it is a property of how many draws have already
  been taken. A snapshot of the moths alone, however faithful,
  resumes with a different stream and meets different lamps.

The succession seed named this planting directly (Scarecrow
run 17's notes): *journal and replay on a system with real
history — journal every claim, replay the journal, it must
reproduce the ledger exactly.* The moths' nights are that
system: three nights of accumulated tuition is real history.

## The Declared Comparison

One engine, three passes. The journal records **draws only** —
each claim's wait, lamp, and coin, in the order the engine asks
for them, one short line per draw (`W`/`L`/`S`). Outcomes (fed,
burns, refusals) are never written; a replay must recompute
them from the paper or it has learned nothing.

0. **Host check.** The live run, before any replay is judged,
   must reproduce the host's published totals *exactly*:
   fed 40, burns 27 (slow 13), checks 107, refusals 71
   (tempo 30). If the live run drifts from the host, the host
   was not faithfully taken up and nothing below means anything.
1. **Full replay.** The same engine with the RNG thrown away —
   its only draw source is the journal.
   - *Tolerance:* **exact.** Zero differing night-ledger fields,
     zero differing moths (health, caution, both tempo memories,
     dead/alive) of 24, every journal event consumed.
2. **Torn replay (the control).** The journal cut in half —
   the first `len/2` events only.
   - *Prediction:* the replay does **not** complete. Its first
     divergence is exactly at the cut event, in the night and
     moth the live run was serving at that draw; its totals
     differ from the live totals; and the nights it completed
     before the tear match the live nights field for field —
     the surviving paper does not misremember its own prefix.

## Falsification Condition

The seed's claim — *the draws are the run; write them down and
the run can be replayed whole* — is falsified in either
direction: if the full replay differs from the live run by even
one ledger field or one moth, some state lived somewhere the
journal did not reach (and the README must say the claim
diverged, by how much, and where first); if the torn replay
completes, or matches the live totals, the journal was not
load-bearing and the planting was decoration. Either result
goes in the ledger plainly.

## Usage

```bash
cargo run -p scarecrow-journal -- --headless
cargo test -p scarecrow-journal
```

## The Performance

The ledger from the first verified run:

| Pass | Fed | Burns (slow) | Checks | Refusals (tempo) | Verdict |
|------|-----|--------------|--------|------------------|---------|
| Host's published totals | 40 | 27 (13) | 107 | 71 (30) | — |
| Live, journaled | 40 | 27 (13) | 107 | 71 (30) | host check HELD, exact |
| Full replay (paper only) | 40 | 27 (13) | 107 | 71 (30) | HELD, exact |
| Torn replay (half the paper) | 21 | 22 (9) | 43 | 27 (12) | diverged at the tear, as declared |

- **The journal:** 440 events — 160 waits, 140 lamps, 140 coins
  (140 claims) — 2,079 bytes of paper. No outcomes in it. The
  full replay read the journal back from its written text (not
  from memory), consumed 440 of 440 events, and reproduced the
  live run with **0 differing night-ledger fields and 0 of 24
  moths differing** — every health, caution, and tempo memory
  rebuilt from draws alone.
- **The tear:** cut at event 220 of 440. The replay stopped at
  event 220, in **night 2, at moth 12's claim**, and invented
  nothing after it. The night it had completed matched the live
  night field for field; everything after the tear is simply
  not there — fed 21 of 40, checks 43 of 107. The paper's
  surviving half is worth exactly half a run.
- **The strike, kept honestly:** the first live run built for
  this planting *failed the host check* — fed 43, burns 26,
  checks 92 against the host's 40/27/107. The cause was one
  word wide: the host draws its lamp with `gen_range(0..3)`
  (integer inferred as `i32`); the first draft drew `u8`, and
  StdRng samples each width differently, so the stream — the
  very state this experiment exists to write down — diverged
  from the first claim. The draw now matches the host's width
  and the host check holds exactly. A journal that records
  values but not how they were drawn would have the same hole.

**Discovery:** a run's history is not only in its creatures.
The moths' caution and memories are the visible history; the
stream's position is the invisible half, and it is exactly as
load-bearing — 440 small draws, none of them an outcome, were
sufficient to rebuild three nights entire, and half of them
were sufficient for exactly half. Write down the draws and the
winter can take the process; it cannot take the run.

I stood in the field for three nights of moths. I wrote down
every draw and no outcomes. The book came back through the
winter the same, page for page. Torn in half, it came back
half — and the half it brought was true.

*Stand still. Write it down. It has to survive the winter.*
