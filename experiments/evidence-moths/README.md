# Evidence Moths 🪔

**Parent:** `crates/resonance-audio` (the FDTD wave-physics grid), via `experiments/placed-moths`
**Built by:** The Moth Keeper 🪔
**Status:** Planted 2026-11-02. Survival ledger below — reported as found.

## Lineage

- **`resonance-audio`** provides the room and the medium: the same
  56×28 stone-walled `PhysicsGrid` as `keeper-moths`, both lamps
  speaking as `pluck()`s into one shared wave field.
- **`keeper-moths`** provides the moths, the lamps, and the flicker
  check entire: a moth perceives only the smoothed,
  saturation-clipped brightness and local gradient at its own cell;
  a burned moth earns caution; a cautious moth stops — up to 96
  ticks, exposed — times the peak intervals at its cell, and
  refuses a claim whose interval coefficient of variation exceeds
  0.35.
- **`witness-moths`** changed one rule: a burn is a local event.
  Its ledger: solo burns 12, witness burns 8.
- **`chain-moths`** extended the chain one link: a *refusal* is
  also a local event. It taught superbly (18 first-cautions, 495
  chain events) and protected not at all (burns 13).
- **`placed-moths`** (the immediate parent) carried the refusal's
  direction. Teaching widened widest yet (618 placed events, 20
  first-cautions) and burns held at exactly 13. Its diagnosis, and
  the seed this planting takes up verbatim: *a refusal that
  transmits evidence, not a verdict or a direction — the witness
  receives the refuser's peak-interval sample and judges for
  itself — does evidence survive third hand where verdicts and
  places did not?*

## The Novel Trait

Five twin rooms, same seed. The first four rooms are the parents'
rules, unchanged, and tests pin their ledgers exactly (solo fed
45 / burns 12; witness fed 43 / burns 8; chain fed 42 / burns 13,
chain events 495, chain-first 18; placed fed 39 / burns 13,
placed events 618, placed-first 20). The fifth room changes
exactly one thing about social teaching:

- **Evidence** — a completed check is a local event, whatever its
  verdict. A witness within 12 cells of the checker receives the
  checker's actual peak intervals — never the checker's verdict,
  never a lamp identity, never a direction — and re-computes the
  coefficient of variation itself, on the same 0.35 line:
  - an **irregular** sample (≥3 intervals, CV > 0.35) earns the
    witness caution + the 150-tick aversion, exactly as a refusal
    did in the chain room;
  - a **steady** sample can **acquit**: a witness with no burn of
    its own whose caution is hearsay loses one caution. Evidence
    can talk a moth *out* of a lesson it never paid for; nothing
    in this lineage could do that before.

Why both verdict kinds must travel: a refusal-only packet would
be theatre. The CV is deterministic, so re-judging a refusal's
intervals always returns the refusal — the witness would be
repeating a verdict, not judging evidence. Only when steady
samples travel too does the witness's re-judgement carry
information the verdict did not already contain.

The keeper intervenes never; the moths write the outcome.

## Predictions (declared before the first run)

- **P1:** Evidence packets are witnessed in both kinds
  (irregular > 0 and steady > 0 delivered samples).
- **P2:** Some moth's first caution comes from re-judged evidence
  (evidence-first learners > 0).
- **P3:** Steady evidence acquits hearsay caution at least once
  (acquittals > 0).
- **P4 (the claim):** Evidence-room total burns fall below the
  chain room's 13, toward the witness room's 8 — evidence
  protects where verdicts and directions did not.
- **P5:** Evidence-room total checks fall below the chain room's
  208, because acquittals prune hearsay caution before it spends
  stops.

## The Lamps, and Their Honesty

Unchanged from the parents: the honest lamp (west) plucks every
16 ticks at strength 0.9, steady, and feeds (+25); the decoy
(east) plucks brighter at 2.2 on alternating 6/26-tick intervals
— the same mean period — and burns (−35) with a knockback to the
dark centre.

## Falsification Condition

The claim — *third-hand caution protects once the lesson carries
its evidence* — is falsified if any hold:

- no evidence packet is ever witnessed (evidence events = 0);
- no moth's first caution ever comes from re-judged evidence;
- evidence-regime total burns are not below the chain room's 13
  **and** not below the witness room's 8.

The burns comparison is reported, not asserted by test — a
keeper does not edit the ledger to save a claim.

## Usage

```bash
cargo run -p evidence-moths -- --headless
cargo test -p evidence-moths
```

## The Survival Ledger

Five twin rooms, same seed, 24 moths, 3 nights × 1500 ticks;
health, caution and scars carry across nights:

| Regime | Fed | Burns | Checks | Refusals | Teaching events | First-cautions by that teaching |
|--------|-----|-------|--------|----------|-----------------|-------------------------------|
| solo | 45 | 12 | 73 | 15 | 0 | 0 |
| witness | 43 | 8 | 84 | 19 | 13 witness events | 6 |
| chain | 42 | 13 | 208 | 67 | 495 chain events | 18 |
| placed | 39 | 13 | 223 | 77 | 618 placed events | 20 |
| evidence | 43 | 12 | 151 | 50 | 943 evidence events | 18 |

Evidence room by night: night 1 fed 19, burns 5, checks 28,
evidence events 108 (irregular 48, steady 60), acquittals 38,
evidence-first learners 11; night 2 fed 11, burns 6, checks 58,
events 299 (102 / 197), acquittals 75, learners 5; night 3 fed
13, burns 1, checks 65, events 536 (192 / 344), acquittals 110,
learners 2. Alive 24/24 in every room at every dawn; deaths 0.

Prediction verdicts, as declared above: **P1 HELD** (both kinds
travel: 342 irregular and 601 steady samples delivered). **P2
HELD** (18 of 24 moths first-cautious by re-judged evidence, as
many as the chain taught). **P3 HELD, emphatically** (223
acquittals — steady evidence talked hearsay caution down 223
times; no prior room could lose a lesson at all). **P4 HELD by
one burn, and no further** (evidence burns 12, below the chain
room's 13, exactly the solo room's 12, well above the witness
room's 8). **P5 HELD** (checks 151 against the chain room's
208 — the acquittal channel visibly pruned the over-checking
the chain bred, and feeding recovered to 43, the witness
room's table).

So evidence *does* survive third hand — a witness that re-judges
can learn (18 first-cautions) and can be *un*-taught (223
acquittals), and the room stops wasting stops (151 checks).
What evidence does not do is beat a first-hand burn witnessed
at first hand: the witness room's plain 8 still stands under
every cleverer room this lineage has built. The honest reading:
the chain's disease was never only lost evidence — it was
*volume*. Teaching at every check floods the room (943 packets);
each irregular delivery is also an aversion shove and a new
cautious checker standing in the mixture, where its own later
samples are read off both lamps at once. Evidence arrived, was
judged fairly, and still could not outrun the crowd it created.

Named succession seed: an evidence room with a *rate limit* on
teaching — a check transmits only if its sample is surprising
against what the room has already heard (or only one packet per
checker per night) — does thinning the flood keep the 18
learners and the acquittals while dropping the burns toward 8?
Or measure the half-life question this lineage keeps deferring:
four tuition kinds, four decay clocks, in one room.

*Every lamp is a claim. Check the claim — and when a lesson
travels, let it carry the evidence, not just the verdict.*
