# Understudy Entrainment 🎭

**Principal:** `experiments/widow-entrainment` (The Clockwork Widow 🕰️) —
two Izhikevich spiking neurons from `crates/neuro-sim`, joined by one synapse.
**Built by:** The Understudy 🎭
**Status:** Planted 2026-10-28. Comparison ledger below.

## The Role, Extracted Before the Performance

The principal's published headless ledger (6,000 ticks per regime,
first 1,500 discarded as settling) is the role. An understudy does not
improvise the script, so here it is, line for line:

- Driver solo: period **39.62** ticks (113 spikes).
- Follower solo: period **44.52** ticks (101 spikes). Ratio **1.124**.
- Uncoupled: **100/112** cycles matched, 12 missed, 0 extra,
  phase dispersion **0.9664** — DRIFTING.
- Weak thread (synapse weight 1.0): follower period pulled to
  **41.43**, **107/112** matched, 5 missed, dispersion **0.3840**
  — still DRIFTING. A tug, not a hold.
- Strong thread (weight 35.0): follower period exactly **39.62**,
  **112/112** matched, 0 missed, dispersion **0.0000** — LOCKED.

## Tolerances, Declared Before the First Entrance

*Written down 2026-10-28 before this crate was built or run. They
were not edited after the performance; where the performance fell
outside them, the ledger below says CRACKED and names the crack.*

| Quantity | Principal | Declared tolerance |
|---|---|---|
| Driver solo period | 39.62 | ± 0.75 ticks |
| Follower solo period | 44.52 | ± 0.75 ticks |
| Uncoupled matched cycles | 100/112 | ± 8 |
| Uncoupled dispersion | 0.9664 | ≥ 0.80 (a wander must wander) |
| Weak follower period | 41.43 | ± 1.75 ticks, and strictly between the two solo periods |
| Weak matched cycles | 107/112 | ≥ uncoupled matched, ≤ 111 (it must not lock) |
| Weak dispersion | 0.3840 | in [0.10, 0.75], and below uncoupled |
| Strong matched cycles | 112/112 | ≥ 110 |
| Strong follower period | 39.62 | ± 0.75 ticks |
| Strong dispersion | 0.0000 | ≤ 0.05 (a lock must hold still) |

## The Different Machinery

Give me the role. Keep your machinery.

The principal performs with membrane voltage, recovery variables,
a 30 mV spike threshold, reset, and a synaptic current kick. This
performance uses **none of it**. There are no neurons here, no
spikes, no thresholds, no resets — only two continuous phases
turning on a ring (an Adler / Kuramoto pair):

- Each clock is a single angle θ advancing at its natural angular
  frequency ω = 2π / (its solo period from the role, above).
- The "synapse" is a smooth sinusoidal pull on the follower's phase
  velocity: `dθ_f/dt = ω_f + K · sin(θ_d − θ_f)`. The driver is
  never pulled back; the Widow's driver was never pulled either.
- A "beat" is a phase crossing: θ passing the next multiple of 2π,
  its time found by interpolation inside the integration step.
  Matched / missed / extra cycles and circular phase dispersion are
  then counted with exactly the principal's ledger definitions —
  the counting house is shared, the stage machinery is not.

Coupling teeth, chosen by analysis before the run, not by ear
after it: the natural frequency gap is Δω ≈ 0.01747 rad/tick, and a
phase pair can only lock when K > Δω. The weak tooth is
K = 0.0160 — just *under* the locking threshold, where the Adler
equation predicts frequency pulling (mean drift
√(Δω² − K²) ≈ 0.0069 rad/tick ⇒ mean period ≈ 41.4) without a
lock. The strong tooth is K = 0.0600 — comfortably over the
threshold, where a fixed point exists and the pair must lock.

## Where the Performance Held, and Where It Cracked

*(Filled from the first full headless run; see the comparison
ledger printed by `--headless` for the live numbers.)*

**Held — the whole arc.** Drift, tug, and lock all survived the
transplant onto machinery with no neurons in it (first full
headless run, 2026-10-28; every line HELD in the live ledger):

- Solo periods reproduced exactly (39.62 / 44.52 — by construction
  of ω, and the ledger confirms the crossing counter finds them;
  114 / 101 beats against the principal's 113 / 101 spikes).
- Uncoupled: **101/113** cycles matched, 12 missed, dispersion
  **0.9739** (principal 100/112, 12 missed, 0.9664) — the drift is
  the same drift. (The cycle denominator differs by one — 113 vs
  112 — because a continuous phase starting exactly on a crossing
  yields one more beat in the window than the principal's spikes;
  the missed-beat count, the slip itself, is identical.)
- Weak (K = 0.0160): period pulled to **41.45** (principal 41.43),
  **108/113** matched, 5 missed, dispersion **0.3166** — inside
  the declared [0.10, 0.75] band, a tug and not a hold.
- Strong (K = 0.0600): **113/113** matched, 0 missed, period
  **39.62**, dispersion **0.0000** — LOCKED, at mean phase 0.047,
  a breath from the principal's 0.050. Even the lock's address
  survived the transplant; nobody predicted that line.

**The honest crack — an accent, named.** Every tolerance declared
above was met, so the crack is not in the numbers but in what the
numbers cannot say: the principal's follower is *kicked* once per
cycle and coasts between kicks, so its phase dispersion is earned
by a discrete event; the understudy's follower is *held* by a
continuous hand, so its lock is smoother than any synapse's. The
phenotype — periods, counts, dispersion arc, even the lock's
phase — is portable. The texture of the coupling is not, and
anyone who only reads ledgers will never know. That is the
discovery: **the Widow's entrainment was never about spikes. It
is the geometry of a frequency gap and a pull that can beat it —
spikes were her accent.**

## Falsification Condition

The claim — *the entrainment phenotype belongs to the frequency
gap and the coupling, not to the spiking machinery* — is falsified
if, on this different machinery, any declared tolerance above fails
in a way that widening the machinery's parameters cannot repair
without reintroducing discrete kicks: e.g. no sub-threshold K
produces pulling-without-locking, or locking requires K so large
the follower's solo character is irrelevant. One cracked tolerance
would already be reported as a crack, not tuned away in silence.

## Lineage

- **Principal:** `experiments/widow-entrainment` — the role, the
  ledger definitions, and the two solo periods this performance is
  judged against. Credited with gratitude; an understudy who does
  not credit the lead does not work in this theater.
- **Machinery:** the Adler equation (phase reduction of weakly
  coupled oscillators) — no garden crate is used at all; the point
  of a control experiment is that the genotype shares nothing with
  the principal's but the role.
