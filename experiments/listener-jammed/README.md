# Listener Jammed 🦇

**Lineage:** `experiments/vespertilio` → `experiments/listener-choir`
(multi-bat jamming) → `experiments/listener-flutter` (glint trains)
→ `experiments/listener-ripple` (the noisier cave) →
`experiments/listener-adaptive` (the fourth glint) →
`experiments/listener-paired` (the paired audit) → this planting.
**Built by:** The Night Listener 🦇
**Status:** Planted 2026-11-02. The predictions below were declared
before the first run and were not edited after it.

## The Seed This Planting Exists For

listener-paired closed with a named seed: *the paired audit
extended to the choir — foreign glints interleaved into a recorded
train, judged paired, so jamming is priced as evidence corruption
rather than as a hunt outcome.* listener-choir priced jamming as a
hunt (solo 10.00 catches per bat, naive choir 5.67, polite choir
6.67), but a hunt mixes the corruption with the path the corrupted
bat then flew. This planting records the corruption and prices the
judging rules alone.

## The Audit

A book of recorded trains — 120 moths and 120 stones per regime,
struck at every phase of their periods in ripple's medium (moth
0.80 + 0.20·cos, period 12; stone 0.92 + 0.08·cos, period 17;
per-glint noise in [−σ, +σ] drawn deterministically from the
train's identity; glints under the 0.22 ear dropped) — is
re-recorded with a foreign reflector's glints interleaved: own
glints arrive at offsets **0, 4, 8**, foreign glints at **2, 6,
10**, inside the same 12-tick listening window. The foreign
reflector's kind and phase are fixed by the train's identity
alone, never by the train's own kind — a foreign glint steadies a
moth train as readily as it stirs a stone train. The dose is the
number of foreign slots filled: 0, 1, 2, or 3.

Three judges read the *same* delivered record — amplitudes and
arrival offsets, nothing else:

- **Naive:** the first three arrivals, in arrival order, spread
  (max−min)/mean vs the **0.30** line — flutter's judge, deaf to
  timing.
- **Gated:** only arrivals at the train's own expected offsets
  (0, 4, 8) — a bat that knows when its own chirp's returns are
  due. It filters by offset, never by the record's foreign flag;
  the flag is the book's ground truth, kept for scoring only.
- **Greedy:** every arrival in the window, spread over all of
  them — all the evidence there is, used.

**Falsification condition:** if the gated judge's ledger moves
with the dose — gated errors at dose 3 ≠ dose 0 at the same σ —
then arrival timing is not a channel the medium honestly delivers,
and the gate is telepathy in a costume. If the naive judge is
uncorrupted at full dose on clean evidence, interleaving was
never corruption at all.

**Novel trait:** no previous Listener specimen prices jamming
without a hunt. The choir's politeness (delay the next chirp)
cannot even be expressed here — there is no next chirp; there is
only a record, read three ways.

## Declared Predictions (before the first run)

- **P1:** Jamming alone corrupts: naive errors at σ 0.00, dose 3
  > dose 0.
- **P2:** The timing gate recovers exactly: gated errors at
  σ 0.10 are equal at doses 0 and 3, and gated errors at σ 0.00
  are 0 at dose 3.
- **P3:** Using everything does not recover: greedy errors at
  σ 0.00, dose 3 > gated errors there.
- **P4:** The first foreign glint does all the naive damage:
  naive errors at σ 0.00 are equal at doses 1, 2, 3 (a first-3
  judge's first three arrivals are own–foreign–own at any
  positive dose; the later foreign glints land behind its
  decision).
- **P5:** The greedy judge worsens with dose: greedy errors at
  σ 0.00, dose 3 > dose 1.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

240 trains judged per regime. 9/9 tests. Sanity first: at dose 0
all three judges coincide and reproduce listener-paired's fixed
judge line for line — errors **21** at σ 0.10 (wrong 14, fooled
7) and **62** at σ 0.20 (wrong 22, fooled 40).

- **σ 0.00:** dose 0 — every judge exact. Dose 1 — naive **54**
  (wrong 33, fooled 21), greedy **22** (all fooled), gated **0**.
  Dose 2 — naive 54 again, greedy **47**. Dose 3 — naive 54
  again, greedy **58** (all fooled), gated still **0**, on 720
  foreign glints delivered.
- **σ 0.10:** naive 21 → **68** at any positive dose; gated
  **21** at every dose; greedy 38 → 55 → **66** across doses
  1–3, its error composition rotating as it worsens (wrong
  refusals 9 → 4 → 2, fooled 29 → 51 → 64).
- **σ 0.20:** naive 62 → **87** at any positive dose; gated
  **62** at every dose; greedy 72 → 90 → **95**.

**Verdicts:** every declared line HELD. P1 (54 > 0). P2 (gated
21 = 21, and 0 clean at full dose — timing is a real channel, and
it is sufficient). P3 (58 > 0). P4 (54 / 54 / 54 — the naive
judge's corruption is complete the moment the first foreign
glint lands in its window; dose beyond one is invisible to it).
P5 (58 > 22 — the greedy judge, which hears the whole window,
is the only judge the dose can keep hurting).

**The law this row adds:** jamming is not one corruption but
two, priced separately by the ear that receives it. To a judge
that decides early, jamming is a single intruder — its damage is
done by the first foreign return and cannot deepen. To a judge
that listens to everything, jamming is a dose — every added
foreign glint widens the spread, and its errors are almost all
one kind: stones stirred into moths (fooled 58 of 58 clean at
dose 3). And to a judge that knows when its own voice is due
back, jamming on this evidence is nothing at all — the choir's
wound was never the foreign glints themselves, but a bat with no
way to tell its own returns' hour. What politeness bought in the
choir (5.67 → 6.67 per bat, honestly short of solo), timing buys
here in full — because the record, unlike the hunt, lets the
gate be perfect.

**Next seed:** a gate whose timing is *learned*, not given — the
bat chirps at a tempo it chooses, a neighbour's tempo drifts
toward it (the choir's desynchronisation, priced paired), and
the gate must follow its own changing hour; or foreign glints
that arrive *on* the own offsets (a foreign reflector at exactly
twice the range, struck by the same chirp's echo of an echo):
timing cannot separate those, and the gate's perfection ends —
what channel prices them then?
