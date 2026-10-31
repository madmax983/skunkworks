# Understudy Damped 🎭

**Principal:** `experiments/damped-moths` (The Moth Keeper 🪔)
**Performed by:** The Understudy 🎭
**Status:** Planted 2026-10-31. The principal's two rooms — a bell
and a quiet room — performed on claim queues, where a "room" is
not a place but a property of the delivery.

## The Principal's Role, Extracted

`damped-moths` ran one cast of 24 moths, three nights of 1,500
ticks, three lamps (honest period **16**, strength 0.9, feeds;
irregular decoy **6/26**, strength 2.2, burns; slow decoy period
**48**, strength 1.4, perfectly steady, burns), two discernment
channels (flicker CV, then tempo memory), in two FDTD rooms that
differed only in appetite:

| Room | Fed | Burns (slow) | Checks | Refusals (tempo) | Alive |
|------|-----|--------------|--------|------------------|-------|
| Bell (damping 0.999) | 12/9/5 = 26 | 39 (21) | 195 | 121 (3) | 23/23/21 |
| Damped (Void frame) | 11/7/6 = 24 | 34 (16) | 165 | 39 (0) | 23/21/21 |

Ring-down of one pluck: bell **5,480** ticks, damped **1,881**.
The damped room was safer — but not by the sense it was built
for. Its tempo refusals were **zero**: the check-sample supply
collapsed (13 full samples in three nights; not one check near
the slow lamp gathered two onset intervals), because a lone
lamp's smoothed wavefront rises at most ~0.03–0.06 a tick, at or
under the moths' own onset threshold. The principal's own
verdict, kept whole: *the onset ear was never listening to
lamps; it was listening to interference — the one thing the
damped room honestly removes.* And its README names this stage
in advance: *"The Understudy's queues carry tempo exactly
because a queue delivers the interval itself; a wave medium,
quiet or loud, delivers something else."*

## The Understudy's Machinery

No field, no grid, no positions, no amplitudes — so nothing can
attenuate, and nothing can ring. Each moth owns a **claim
queue**. The lamps keep global clocks (honest onsets every 16
ticks; irregular alternating 6/26; slow every 48). A cautious
moth — caution earned only by burning, as in the principal —
stops and samples the next intervals **as its queue delivers
them**, up to the principal's 160-tick tempo cap, and judges
them by the principal's own arithmetic, unchanged: CV above
0.35 is irregular; a steady mean deviating from a home tempo by
>35%, or matching a burnt tempo within 20%, is refused on tempo
grounds. Tuition −35, feeding +25, aversion 150, claim waits
300–500 at equal lamp odds — the script is the principal's;
only the delivery differs, and the delivery comes in two rooms:

- **The clean queue** (the quiet room's understudy): the queue
  delivers the sampled lamp's own onsets, exactly. Nothing is
  added; nothing is lost; there is no amplitude to fall under a
  threshold.
- **The echo queue** (the bell's understudy): the queue is a
  party line. At every post, the delivered onset stream is the
  sampled lamp's own onsets **plus** one self-echo of each
  (+53 ticks — a reflection arriving late) **plus** a leak of
  every other lamp's onsets (+7 ticks). The moth cannot
  attribute an onset to a lamp; it hears the mixture, exactly
  as a moth in the bell hears the mixture. Keeper-side, each
  sample is bucketed by the lamp the moth *intended*.

The queue's ring-down analog is printed in the ledger:
deliveries per true onset at the honest post (clean 1.00 by
construction; the echo queue's figure is measured).

## Tolerances and Predictions — Declared Before the First Run

Comparison bands (totals across three nights, principal's
damped room is the role's quiet half, its bell the loud half):

- **B1.** Clean-queue feeding total in **18..=32** (principal
  damped 24, bell 26).
- **B2.** Clean-queue slow burns total in **8..=22** (principal
  damped 16).
- **B3.** Echo-queue refusals total **≥ 60** (principal bell
  121) **and** greater than the clean queue's refusals total
  (principal: 121 vs 39).
- **B4.** Night-3 survivors in **18..=24** in both queue rooms
  (principal 21 and 21).

Predictions — the performance exists to test these:

- **P1. The channel revives.** Clean-queue tempo refusals total
  **≥ 15** (principal damped: **0**). A queue delivers the
  interval itself; there is no wavefront to fall under an onset
  threshold, so the quiet room cannot starve this sense.
- **P2. Interference, not quiet, mutes tempo.** Echo-queue tempo
  refusals total **≤ 8** (principal bell: 3) while echo-queue
  slow burns total **≥** clean-queue slow burns.
- **P3. Learning shows in the clean room.** Clean-queue slow
  burns fall from night 1 to night 3 (night 3 < night 1) —
  the principal's damped room managed only 5 → 4 on flicker
  evidence alone.
- **P4. A queue cannot starve a check.** Starved samples
  (fewer than two intervals inside the 160-tick cap) total
  **exactly 0** in both queue rooms — against the principal's
  damped room, where the sample supply collapsed to 13 full
  samples in three nights. The queue's failure mode is
  corruption, never scarcity.

These lines were written before this crate was built or run
and are not edited afterward. The headless ledger prints the
principal's numbers beside this performance's and marks each
line HELD or CRACKED.

## Falsification Condition

The principal's closing claim — *a wave medium, quiet or loud,
delivers something else; a queue delivers the interval
itself* — is falsified if the clean queue also fails to carry
tempo (P1 cracks: tempo refusals near zero on exact delivery),
or if the echo queue's contamination does not reproduce the
bell's signature (P2 cracks: tempo judges fine through the
mixture). Either crack would mean the tempo channel's fate was
never the medium's doing at all.

## What the Ledger Found

Deliveries per true onset at the honest post: clean **1.00**,
echo **2.82** — the party line talks nearly three times for
every word the lamp says.

| Room | Night | Fed | Burns (slow) | Checks | Refusals (tempo) | Alive |
|------|-------|-----|--------------|--------|------------------|-------|
| Clean queue | 1 | 14 | 13 (10) | 20 | 15 (7) | 24 |
| Clean queue | 2 | 16 | 6 (5) | 32 | 20 (8) | 24 |
| Clean queue | 3 | 16 | 3 (1) | 37 | 23 (15) | 24 |
| Echo queue | 1 | 14 | 16 (10) | 25 | 19 (0) | 23 |
| Echo queue | 2 | 10 | 9 (7) | 39 | 31 (9) | 23 |
| Echo queue | 3 | 3 | 5 (3) | 46 | 43 (21) | 22 |

Sample evidence, keeper-bucketed by intended lamp: clean queue
89 samples ≥2 intervals, 69 full, heard means **16.00** near
honest and **48.00** near slow — the intervals themselves,
exact. Echo queue: 110 samples, *every one full*, heard means
**5.54** near honest and **5.95** near slow — evidence
everywhere, all of it forged, and far denser than the bell's
mush (23–27 at every lamp): a queue's interference does not
smear a tempo, it *replaces* it with the mixture's own fast,
regular-looking pulse.

**Bands:** B2 HELD (clean slow burns 16, the principal damped
room's exact figure), B3 HELD (echo refusals 93, above the
clean queue's 58 — the bell's refusal inflation survives the
swap), B4 HELD (alive 24 / 22). **B1 CRACKED:** clean feeding
totalled **46** against the damped room's 24 — as in this
theater's earlier queue roles, nothing rations arrivals on a
queue: no flight, no brightness, cheap exact checks, so the
cast feeds at nearly every claim it accepts. Arrival politics
belongs to the field, not the script.

**Predictions:** P1 HELD, decisively — clean-queue tempo
refusals **30** (7 → 8 → 15 as the memories spread) against the
principal damped room's **0**: on exact delivery the channel
the quiet room starved fires at the rate this theater's earlier
queue performance established. P3 HELD — clean slow burns fell
10 → 1, where the principal's damped room managed 5 → 4 on
flicker evidence alone. P4 HELD — starved samples **0** in both
rooms; the clean queue still gathered 69 full samples to the
principal's 13.

**P2 CRACKED, and the crack is the discovery.** The echo queue
was declared to mute tempo as the bell did (≤ 8 tempo
refusals). It produced **30** — as many as the clean queue —
but aimed at everything: the mixture's heard mean (~5.5–6.0)
deviates from a home tempo of 16 by over 60%, so a moth holding
a home tempo refuses *every* steady-looking sample on tempo
grounds, the honest lamp included, and echo feeding collapsed
14 → 10 → 3 as the memories spread. In the principal's bell
the same interference rarely reached the tempo judge at all —
the mixture's flicker was chaotic, the flicker check ate the
verdicts, and tempo refusals stayed at 3. On the party line the
mixture is *regular* (the honest lamp's leaked grid dominates
it), so samples pass the flicker line carrying a forged tempo,
and the tempo judge — which the bell starved of verdicts and
the damped room starved of samples — is here fed forgeries and
convicts the innocent with the guilty. P2's second half HELD
(echo slow burns 20 ≥ clean 16): the slow lamp still collected
its tuition, some of it through fooled checks (7 across the
run) whose steady-looking samples sounded like home.

So the principal's closing claim survives only in part, and
the falsification condition above is met on its second clause:
a clean queue does deliver the interval itself (P1) — but a
contaminated queue does not deliver "something else" the way a
wave medium does. The field *drowns* tempo and *starves* it;
the queue *forges* it. There are three ways a sense can fail,
and the medium chooses which.

## Usage

```bash
cargo run -p understudy-damped -- --headless
cargo test -p understudy-damped
```

*Give me the role. Keep your machinery — and mind what your
delivery adds to the interval, and what it quietly takes away.*
