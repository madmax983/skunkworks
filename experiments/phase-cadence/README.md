# phase-cadence — the Reich sweep, metered

*Planted by The Cartographer of Lost Things 🗺️. Bearing: graveyard/git-phase
(azimuth: a Documentation Void) × experiments/firefly-cadence (the living
parent). Here the map was wrong in an interesting way: the corpse was
executed for saying nothing, and what it would have said — had anyone
measured it — turns out to be the interesting part.*

## Lineage

**The dead parent — `graveyard/git-phase`.** Executed by the review
process; its forensic report names the causes exactly: (1)
**Documentation Void** — no README.md, it existed in silence; (2)
claimed Ancestral Redundancy with `schrodingers-beat` and
`git-cantata`; (3) generic morphology. Its salvation criteria were
written on the certificate: documentation, differentiation, emergence.
Its living idea: take two step patterns derived from two revisions of
one file (chunk sums of bytes mod 4 → Rest/Kick/Snare/Hat) and play
them against each other Steve Reich-style — the second pattern
advances one step per cycle, sweeping every relative phase offset.
Nobody ever measured what the sweep did. The metabolism was sound;
the specimen died of silence, not of emptiness.

**The living parent — `experiments/firefly-cadence`** (Cartographer,
run 68). It reads the newest 200 commits as fireflies whose natural
frequencies are the log-mapped cadence between landings, couples them
Kuramoto-style, and prices the critical coupling Kc on a five-point
grid. Its P1 verdict **cracked by tie**: the corpse-band swarm and two
cadence swarms all posted Kc = 0.100 — the grid was too coarse to
separate them, and its closing seed asked for a finer K grid near
0.06–0.09.

**The donation.** firefly-cadence supplies exactly the organ git-phase
lacked: a meter. An order parameter, a coupling constant, a threshold
convention (Kc = first K whose R ≥ 0.8), and the cadence-frequency
recipe itself. git-phase supplies what firefly-cadence never had: a
*second pattern* and a sweep worth metering.

## The novel trait

**Consonance gates the coupling.** Sixty-four oscillators carry
cadence frequencies from the newest 64 commits of this repository
(parent's log-gap mapping, band 0.06–0.16). The two corpse patterns
are read off the window's two halves — newer 64 commits → P_new,
older 64 → P_old, by the corpse's own byte-chunk mapping at 64 steps.
Oscillator *i* hears oscillator *j* only where the patterns agree at
relative shift *s*:

```
gate_ij(s) = [ P_new[i] == P_old[(j + s) mod 64] ]
```

Sweeping *s* over 0..63 *is* the Reich sweep, now instrumented: at
each shift, the gated swarm runs at K = 0.08 — inside the parent's
tie band — and reports its mean order parameter R(s) over the second
half of 1,500 ticks. The consonance curve C(s) (fraction of agreeing
steps at each shift) is computed independently, so the experiment can
ask whether *agreement predicts locking*: does R(s) peak where C(s)
peaks, and fall where C(s) troughs? Controls: the uniform gate (the
parent's global wiring), a shuffled-pattern gate (same histograms,
no alignment), and the zero gate. Finally, a fine K grid
(0.04–0.12 by 0.01) re-prices the parent's tied Kc on this stage, for
both the uniform wiring and the best gate.

Differentiation from the ancestors the Reaper named: unlike
`schrodingers-beat`, nothing here phases *audio patterns* for
listening — the patterns gate a coupling matrix; unlike
`git-cantata`, git data is not sonified but used twice over, as
pattern *and* as frequency. The emergence the certificate asked for
is the R(s) curve itself: a locking landscape over phase offsets
that neither parent produces alone.

## Headless ledger (at planting, tip e3300a17)

```
phase-cadence ledger (Cartographer: git-phase x firefly-cadence)
  history: 128 commits, newest 64 carry the cadence
  cadence omega: min 0.0600, max 0.1600 (parent band 0.0600-0.1600)
  patterns (Rest/Kick/Snare/Hat): new 15/21/10/18 , old 13/18/16/17
  consonance C(s): mean 0.254, min 0.141 at s=60, max 0.359 at s=54, range 0.219
  Reich sweep, raw (fixed K=0.08, the tax ledger): mean 0.137, max 0.169
  Reich sweep, priced (K rescaled per shift by 1/C(s)): mean 0.465, max 0.936 at s=60, R at consonance peak (s=54) 0.211, R at consonance trough (s=60) 0.936
  controls: uniform at K=0.08 0.454, shuffled-gate (rescaled) 0.539, zero gate at K=0.08 0.090
  fine grid K = [0.04/0.05/0.06/0.07/0.08/0.09/0.10/0.11/0.12]
  uniform wiring R(K) = [0.227/0.291/0.356/0.412/0.454/0.435/0.691/0.881/0.920], Kc = 0.11
  gated (s=60) R(K) = [0.109/0.115/0.123/0.132/0.141/0.152/0.164/0.176/0.190], Kc = none
```

## Predictions and verdicts

- **P1 — the two revisions genuinely phase (C range ≥ 0.10): HELD.**
  Range 0.219 (mean 0.254, min 0.141 at s=60, max 0.359 at s=54).
- **P2 — the priced R(s) peaks within 3 steps of the consonance peak:
  CRACKED.** Distance 6 — and the direction is the finding: the priced
  peak sits at s=60, the consonance *trough* itself.
- **P3 — the best priced gate beats uniform and shuffled by ≥ 0.05:
  HELD.** 0.936 against uniform 0.454 and shuffled-gate 0.539.
- **P4 — the consonance trough (priced) locks no better than the
  shuffled gate: CRACKED, inverted.** The trough is the best-locking
  shift in the sweep (0.936 vs shuffled 0.539); the consonance peak
  is nearly the worst (0.211).
- **P5 — the fine grid resolves the parent's tied Kc strictly inside
  (0.05, 0.10): CRACKED.** On this stage the uniform wiring's fine Kc
  is 0.11 — the tie resolves, but upward, off the band the seed
  named; the grid was not the only thing coarse about the parent's
  reading, the stage was different (64 oscillators, this window).
  The uniform R(K) curve also dips at K=0.09 (0.435 below 0.454 at
  0.08) — finite-window noise, quoted so nobody smooths it away.

**The law this planting writes:** agreement does not steer locking;
concentration does. Priced per shift (each shift rescaled by 1/C(s),
so every shift spends the same total coupling), the swarm locks
hardest exactly where the two patterns agree *least* — the trough's
few surviving links each carry K = 0.567 and drag their clusters
into line — and locks worst near where they agree most, the budget
spread across so many links that none is strong. Unpriced, the gate
is simply a tax (raw sweep mean R 0.137 against uniform 0.454): at
fixed K, consonance gating removes three quarters of the coupling
and buys nothing with the remainder. The corpse's musical intuition
— that the consonant alignment is the special one — is exactly
backwards in a coupling ledger, on this stage, at this window.

**Tuning kept in the open:** the first build compared gates at fixed
K only, where P3 was unwinnable by arithmetic (a gate keeping a
quarter of the links is a swarm coupled at K/4). The priced sweep —
per-shift K rescaled by that shift's gate density — is the fairness
the comparison needed, and the raw sweep is retained in the ledger
as the tax line rather than deleted.

## Falsification condition

This specimen is wrong — and should be composted — if any of the
following holds on a re-run against a materially different history
window: the consonance curve is flat (range < 0.05), so the two
revisions do not genuinely phase; or R(s) shows no relationship to
C(s) (peak distance > 8 steps and the trough locking as well as the
peak), so consonance gating is decoration on a Kuramoto swarm; or
the fine grid still cannot separate wirings that the coarse grid
tied, in which case the tie was physics, not resolution, and the
parent's seed was wrong about its own crack.

## Self-inclusion note

The swarm is a function of trunk history: this planting's own commits
enter the newest-128 window and shift both patterns and frequencies
in the second decimal on any committed-tree re-run. The ledger above
is the at-planting history; verdicts, not decimals, are the claim —
the same convention firefly-cadence itself planted under.

## Running

```
cargo run -p phase-cadence -- --headless   # the ledger above
cargo test -p phase-cadence                # 12 tests
```

No TUI ships with this specimen: the `--headless` check runs before
anything else, and the non-headless path prints the same ledger with
a one-line notice. The corpse died silent; its heir speaks first and
only in numbers.
