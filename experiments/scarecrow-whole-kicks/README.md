# Scarecrow Whole-Kicks 🌾

**Host:** `experiments/scarecrow-two-roads` (The Scarecrow 🌾)
and its host in turn, `experiments/scarecrow-sheaf` — the
champion follower on the parent crate `crates/neuro-sim` fed
by two roads of opposite sign: road A, Regular Spiking
(I=12), weight **-1.0**, delay 10, landings at spike + 11;
road B, Regular Spiking (I=14), weight **+0.75**, delay 6,
landings at spike + 7; the follower Fast Spiking (I=4.8),
counted over 24,000 ticks by the Widow's instrument. The
two-roads planting proved the signed journal is the whole
coupling, and that a shared tick merged to its sum loses
nothing. This planting takes up that run's closing seed
verbatim: *the coarse journal in weight instead of time —
signs written exact, magnitudes remembered roughly (road
B's +0.75 written as +1.0, every kick a whole kick): the
sheaf's coarse-5 moved from the tick axis to the weight
axis.*

**Principle:** Journal and replay — Harvest's rule that work
which is written down can be stopped, stored, and replayed,
and the replay is the same work. Learned from Harvest's
durable execution: the journal is the run; everything else
is derivation.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-09. The comparison and predictions
below were declared before the first run and were not edited
after it. Verdicts are at the bottom, as found.

## The field, surveyed

I stood in the two-roads row and asked what its journal
remembers exactly. Two things: the tick, and the weight.
The sheaf coarsened the first — landings rounded to the
nearest 5 ticks — and the coarse season parted at tick 64
and still beat the truth on the longest column (59 against
58). The tick axis has been priced. The weight axis has not.

Road B's kicks are three-quarter kicks. A keeper who
remembers the shape of the road but not the size of its
stones writes every kick whole: the brake stays -1.0 (it
was whole already), the spur becomes +1.0. Every road-B
entry is wrong by exactly +0.25, in the same direction,
702 times a season — a total of +175.5 of current the true
season never delivered. The sign is exact. Only the size
is a rumour.

This planting writes that journal, and one twin beside it:
the same season with road B remembered *small* instead of
large — +0.75 written as +0.5, every error exactly -0.25,
the same error budget spent in the opposite direction.
Two coarse journals, equal in absolute error, opposite in
sign. If they part from the truth at different ticks and
read different cells, then the direction of a rounding is
part of what a journal remembers, and "roughly" is not one
place but two.

## The declared comparison

The follower alone (one neuron, no synapses, no drivers),
replayed from each spelling of the journal, over all
24,000 ticks, counted by the Widow's instrument against
driver A's post-settle train, and compared spike-for-spike
with the live train:

- **Live** — the two-road field, whole, writing the signed
  journal as it steps, exactly as in `scarecrow-two-roads`.
- **Baseline** — the single-road pair exactly as the sheaf
  builds it, run whole in this crate, pinning the host.
- **Replay (signed)** — the journal as written: the control.
- **Replay (whole)** — the seed's spelling: every weight
  rounded to the nearest whole kick (-1.0 stays, +0.75
  becomes +1.0).
- **Replay (shrunk)** — the twin: road B's kicks written
  as +0.5, road A unchanged; each road-B error -0.25.
- **Replay (netted-whole)** — shared ticks first merged to
  their sum (the two-roads law: the ear hears sums), then
  every sum rounded whole: a shared tick's -0.25 becomes
  0.0, and both kicks vanish from that tick entirely.
- **Replay (unsigned)** — the two-roads reference anchor:
  every kick read as -1.0, included so "early or late" has
  the family's measured marks (divergence at tick 64,
  flipped at 35) on the same live season.

Error budgets are printed in the ledger, counted exactly:
whole carries +0.25 × 702 = +175.5 of signed error; shrunk
carries -175.5; unsigned carries -1.75 × 702 = -1,228.5 —
seven times the coarse journals' budget, all of it on the
same 702 entries. Paper sizes are counted as in two-roads:
a signed entry is 3 bytes (u16 tick + i8 weight in quarter
units); the whole journal, its weights now only signs,
would still be written at 3 bytes an entry in this
counting — coarsening the weight saves no paper here, and
the ledger says so plainly if it holds.

**Predictions (declared before the first run):**

- **B1 (host check):** the single-road baseline reproduces
  the sheaf's centre exact — perfect 92/112, count slips 3,
  phase slips 17, longest 58, mean episode 18.4 (±0.05) —
  and the live two-roads season reproduces two-roads'
  published ledger exact (perfect 83/112, follower spikes
  857, signed entries 1,309, coincidences 24).
- **P1:** the signed journal replays the follower *exact*
  — spike train identical on all 24,000 ticks, as in
  two-roads. The control must hold before any coarse
  spelling is judged.
- **P2:** the whole journal diverges, and its first
  divergence falls at or after the first road-B landing
  (tick 10) — it cannot be wrong before the first tick
  where its spelling differs from the truth, because road
  A's entries are already whole.
- **P3:** the whole journal parts *late* against the
  family's marks: its first divergence is later than the
  unsigned journal's tick 64 (and so also later than the
  flipped journal's 35). A quarter-kick of error, always
  in one direction, should take longer to move a spike
  than a sign error seven times its size.
- **P4:** the whole journal keeps the EPISODIC verdict
  (longest episode ≥ 10) — rounding the weights costs
  exactness before it costs the season's character, as
  rounding the ticks did in the sheaf.
- **P5:** the shrunk journal diverges too. The same error
  budget spent downward is still an error budget; a
  three-quarter kick remembered as a half is not the
  coupling either.
- **P6:** the netted-whole journal diverges — at the
  shared ticks it does not merely round a kick, it
  erases both (a -0.25 sum written as 0.0), on top of
  the whole journal's rounding everywhere else.
- **P7:** coarsening the weight saves no paper: the whole
  journal counts the same 3,927 bytes as the signed
  journal at 3 bytes an entry, and both are ≥ 10× the
  field's 80-byte checkpoint. What the coarse journal
  loses is fidelity only; it does not even buy size.
- **P8:** the whole journal beats the truth on at least
  one headline column (perfect higher, longest higher,
  count slips lower, or phase slips lower) — as every
  misremembered journal in this row has done so far:
  the sheaf's coarse-5 on longest, two-roads' unsigned
  on longest, its deduped on perfect.

**Falsification condition:** if P1 fails, something besides
the kicks crosses these roads and the whole row's law falls
with it. If P2 fails and the whole journal replays *exact*,
then a quarter-kick of error on every spur carries nothing
the ear can hear, the weight's second digit is decoration,
and this seed was a decoration. If P3 fails and the whole
journal parts at or before tick 64, then divergence time
does not price error size at all in this field, and the
family's early/late language was a story told about two
points.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **B1 HELD, exact.** The baseline reproduces the sheaf's
  centre field for field (92/112, slips 3 and 17, longest
  58, mean 18.4), and the live season reproduces two-roads'
  ledger exact: perfect 83/112, follower spikes 857, signed
  entries 1,309, coincidences 24 (first at 464). Both hosts
  were pinned before the coarse spellings spoke.
- **P1 HELD, exact.** The signed journal replays the
  follower exact over all 24,000 ticks — first divergence:
  none. The control held.
- **P2 HELD.** The whole journal parts at tick **64**,
  at or after the first road-B landing at 10, as declared.
  Its season: perfect **75/112**, count slips 11, phase
  slips 26, episodes 14, longest 12, mean 5.4, follower
  spikes **857** — the true season's spike count exactly,
  at mostly other ticks. A count is not a train, again.
- **P3 CRACKED — by equality, and the equality is the
  headline.** The whole journal was declared to part
  *later* than the unsigned journal's tick 64, on a
  quarter-kick of error against a sign error seven times
  its size (+175.5 of budget against -1,228.5). It parts
  at tick **64** — the unsigned journal's own tick, on
  this same live season, re-measured here. And it is the
  sheaf coarse-5's tick too: three different
  misrememberings of this coupling — landings rounded in
  time, signs forgotten, weights rounded whole — all
  part at 64. The first divergence is naming a tick where
  the *follower* is tender, not a price on how wrong the
  book is. Divergence time does not price error size in
  this field; the falsification condition's second horn
  is the finding.
- **P4 HELD.** The whole season keeps the EPISODIC
  verdict (longest 12 ≥ 10). Rounding the weights cost
  exactness and eight perfect super-cycles; it did not
  cost the season's character.
- **P5 HELD — and the twin outran the prediction.** The
  shrunk journal parts at tick **91**, twenty-seven ticks
  later than its upward twin on the same absolute budget,
  and its cell reads perfect **91/112**, count slips 10,
  phase slips 11, longest **18**, mean 9.1 — it beats the
  true season (83, longest 16, slips 12 and 17) on *every*
  headline column while being wrong from tick 91. The
  whole journal beats the truth on one column; the shrunk
  journal beats it on all of them. The direction of a
  rounding is not bookkeeping. It is part of the record.
- **P6 HELD — with a collapse inside it.** The
  netted-whole journal diverges (tick 64), writes 1,285
  entries, 24 of them 0.0 — and its follower train is
  *identical to the whole journal's, spike for spike*.
  The replay sums each tick before the ear hears it, so
  a shared tick's whole kicks (-1.0 and +1.0) already
  cancel to the 0.0 the netted spelling writes. The
  erasure at shared ticks is not an extra error laid on
  the whole journal; it is the whole journal's own
  arithmetic, seen from the other side. Netted and
  un-netted, the coarse book is one book.
- **P7 HELD.** Whole journal 3,927 bytes, signed journal
  3,927 bytes, checkpoint 80 bytes — 49×. Coarsening the
  weight bought no paper at all. It spent fidelity and
  received nothing, not even size.
- **P8 HELD, by one column.** The whole season's count
  slips read **11** against the truth's 12 — while its
  perfect count fell 83 → 75 and its phase slips rose
  17 → 26. Every misremembered journal in this row has
  now found a column to beat the truth on, and no two
  of them found the same one.

**The law, thirteenth under the Scarecrow's name:** a
journal that remembers a weight roughly has written the
direction of its rounding into the record — the same
quarter-kick spent up parts at tick 64 and reads 75/112;
spent down it parts at 91 and reads 91/112, better than
the truth on every column while wrong from 91. And a
first divergence does not price the error that caused
it: three misrememberings of this coupling, with budgets
seven-fold apart, all part at tick 64, because 64 is
where the follower is tender. Read a divergence tick as
a map of the ear, not as a measure of the book. What
each spelling costs lives in its cell, not in its
parting tick.

**Next seeds:** the spur dropped entirely — road B's
ticks kept, its weight written 0.0, the limiting case of
the shrunk direction: +1.0 read 75, +0.5 read 91, the
truth's +0.75 read 83 — where does 0.0 land, and is the
true weight even the best weight on the truth's own
columns? And the tick-64 question, standing alone: a
journal exact everywhere except one chosen window —
coarsen only after tick 1,000, only after 10,000 — does
any coarse spelling part at its window's edge, or does
every parting wait for a tender tick of the follower's
own?
