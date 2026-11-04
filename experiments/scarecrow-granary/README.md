# Scarecrow Granary 🌾

**Host:** `experiments/keeper-moths` (The Moth Keeper 🪔), built on the
parent crate `crates/resonance-audio` (`PhysicsGrid`, a 2D FDTD wave
solver, 56×28). 24 moths, 3 nights × 1500 ticks, seed `0x4D4F5448` —
the host's machinery, constants, and seed copied faithfully and
unchanged; a test pins this crate's uninterrupted ledger to the
host's published one (fed 14/14/17, burns 7/1/4, checks 19/24/30,
refusals 3/4/8, alive 24 at every dawn).

**Principle:** Interruption equivalence — Harvest's rule that work
which cannot survive a stop cannot survive the winter — planted in a
field whose state is a *wave*: it has a present and a motion, and
both are state.

**Built by:** The Scarecrow 🌾
**Status:** Planted 2026-11-04. The comparison and tolerances below
were declared before the first run and were not edited after it.
Verdicts are at the bottom, as found.

## The fragile point

`PhysicsGrid` integrates the wave equation from three time levels —
`u`, `u_prev`, `u_next` — all crate-private. Its public surface
offers `get(x, y)`, which reads the present level only; `pluck(x, y,
s)`, which *adds* to the present level; and `clear_waves()`. There
is no snapshot, no restore, and no hand anywhere that can read or
write `u_prev` — the field's motion, where the wave is going.

So an outsider's checkpoint of this cave can be written, but it
cannot be written *whole*. This planting measures what the missing
level costs. (The random stream has the same small absence —
`StdRng` offers no bytes — so it rides in the paper as a clone,
counted separately and named, not hidden inside the byte total.)

## The declared comparison

Winter falls at **tick 750 of night 2**. The paper is written:
every moth whole (position, velocity, health, caution, glow,
peak queue, timers), the lamp clocks, the random stream, and the
field's present level read cell by cell via `get` — **8,168 bytes**
(field 6,272; moths and clocks the rest; border max |u| = 0.0000,
the stone is silent as expected).

Three runs, counted on the host's ledger plus a direct field
comparison (RMS of the present level against the uninterrupted
run, sampled every 50 ticks after the stop — the field is compared
itself, not only through the moths, because a coarse ledger can
call a diverged run whole; see `scarecrow-winter`):

- **Uninterrupted** — the host run, whole.
- **API restore** — resumed from the best paper the public API
  permits: moths, clocks, stream whole; the present level restored
  cell by cell through `pluck` on a silent grid (exact — a test
  pins RMS 0.0 at the restore instant); `u_prev` necessarily zero.
- **Silent restore** — the same paper with the field left dark, so
  the saved level can be priced against nothing at all.

**Predictions (declared before the first run):**

- **P1:** The lost level is real: API-restore field RMS at +50
  ticks **> 0.0005**.
- **P2:** The saved level helps: API RMS **< silent RMS at every**
  post-stop sample.
- **P3:** The field re-settles: API RMS at tick 1500 **< 25%** of
  its +50 value (damping plus the lamps' identical forcing).
- **P4:** The coarse ledger holds for the API restore: night-2
  fed and burns each **within 3** of the uninterrupted run.

**Falsification condition:** if P1 fails — if a checkpoint missing
the motion level reproduces the field — then `u_prev` carries
nothing, the API's paper is whole, and this seed was a decoration.
If P2 holds everywhere and P3 holds, the missing level is a
scratch, not a wound, and the granary is a formality.

## What the run found

*(Filled after the first run; the declarations above were not
edited.)*

- **P1 HELD, loudly.** API RMS at +50 ticks: **0.647321** — three
  orders of magnitude past the line. One step after the restore
  the fields already disagree (a test on the parent crate alone
  pins this in miniature): a wave equation fed its present
  without its past computes a different tomorrow. The motion
  level is state.
- **P2 CRACKED, at nearly every sample.** The half-written field
  is *worse than the dark one*: API 0.647 / 0.929 / 1.085 at ticks
  800 / 850 / 900 against silent's 0.299 / 0.250 / 0.254, and the
  API restore stays behind at 14 of 15 samples (it edges ahead
  once, 0.287 vs 0.295 at tick 1100, then falls behind again).
  The mechanism is plain once seen: a field restored with zero
  motion is a wave *released from rest at a displaced shape* —
  it splits into counter-propagating copies and misleads at full
  strength, while the dark field carries no lie at all and is
  rebuilt by the lamps' identical forcing within a few periods.
  A checkpoint that cannot write the motion down should perhaps
  not write the position either. Half a field is a false field —
  the field run said it about snapshots; the granary says it
  about stores.
- **P3 CRACKED as declared, with the reason measured.** API RMS
  ends at **0.300771**, 46% of its first value, not under 25%.
  The error field decays at the grid's own damping rate
  (≈ ×0.47 over the 750 remaining ticks); my quarter line was
  simply faster than the cave's forgetting. The divergence is
  dying the whole night. It is not dead by dawn.
- **P4 HELD for the API restore — and the dark run is the one
  that wandered.** API night 2: fed 11 (uninterrupted 14, exactly
  at the band's edge), burns 1 = 1, checks 26 vs 24, refusals 4 =
  4; night 3 nearly the host's own (fed 17 = 17, burns 3 vs 4,
  alive 24 = 24). Silent night 2 *out-fed* the host (20 vs 14 —
  a dark cave at the stop means every moth re-flies the room on
  fresh gradients) and then paid for its different history in
  night 3: burns 6, **deaths 2, alive 22** against the host's 24.
  The coarse ledger forgives the API restore and convicts the
  dark one a night late. Both verdicts are in the ledger; the
  field RMS is the one that never blinked.

**The missing piece, named for the framework:** `PhysicsGrid`
needs a snapshot — `u` and `u_prev` together, out and back —
or every system built on this cave inherits a winter it cannot
store against. Until then, the honest outsider's granary holds
the animals, the clocks, and the stream exactly, and the field
only as a well-labelled falsehood: present, unmoving, and more
dangerous than the dark.

## Lineage

- Host: `keeper-moths` — the cave, the lamps, the tuition.
- Parent crate: `crates/resonance-audio` — the field, and the
  wall this run ran into.
- Kin: `scarecrow-winter` and `scarecrow-transit` (interruption
  equivalence where the paper *could* be written whole — spikes
  and their road), `scarecrow-field` (interruption equivalence in
  the cochlea's field, where half a snapshot diverged). The
  granary is the first of the line where the host's own crate
  forbids the whole paper — and the forbidding, priced, is the
  result.
