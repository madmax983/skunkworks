# diffusion-cortex 🗺️

**Planted by:** The Cartographer of Lost Things 🗺️
**Lineage:** graveyard/neuro-diffusion × crates/gray-scott + crates/neuro-sim (revival cross)

## The death

`graveyard/neuro-diffusion` (217 LOC, edition 2021) was a complete
bidirectional morphogenesis loop: an Izhikevich spiking network
embedded in a Gray-Scott reaction-diffusion field — each neuron
reading the U chemical at its cell as input current (U × 10), every
spike injecting the V chemical back into the field at the neuron's
cell, one field step per network step at the mitosis parameters
(f = 0.0367, k = 0.0649). It was executed for **Documentation Void
alone**: no README, `main` straight to a TUI, no headless bypass.
Its forensic report said it plainly — "elaborate neural morphogenesis
hidden behind complete lack of documentation" — and its loop was
never counted. Nobody knew whether the spikes spread the pattern,
anchored it, or merely decorated a diffusion that would have
happened anyway.

## The donation

Both of the corpse's own dependencies are still alive in
`crates/`: **gray-scott** (the field) and **neuro-sim** (the
network). They donate the exact organs the corpse was built from,
kept current — including one upstream change the corpse never saw:
the living gray-scott's `get_index` returns `Option<usize>` where
the corpse's returned a bare `usize`.

## The novel trait

Neither living crate prices the loop, and the corpse never tried.
This specimen runs the corpse's loop **verbatim** beside three
ablations on identical ground and identical wiring, so the loop's
contribution is whatever the closed ledger has that the others
do not:

- **Closed** — the corpse: read U → step → inject V on spike → step field.
- **Mute** — reads U, spikes are counted, injects nothing.
- **Deaf** — constant current 10.0 (what a neuron in pristine
  U = 1.0 field reads), spikes still inject.
- **Field alone** — no network; one centre seed and diffusion.

The corpse's numbers are kept: field 80×40, 200 neurons on a
spacing-4 lattice, 3 synapse draws per neuron (15.0 at p = 0.8,
else −10.0; delay 1..=10; self-draws skipped; 596 synapses land),
a single centre V seed, 2,000 ticks per regime.

Adaptations, named: the corpse drew its wiring from `thread_rng`;
here a fixed-seed SplitMix64 stream draws the identical
distribution, so every number below is reproducible. There is no
TUI; `--headless` is handled before anything else.

## Predictions (declared before the first run, never edited)

- **P1:** closed V coverage (cells with V > 0.1) exceeds field-alone
  coverage — injections seed V at 200 lattice sites, far beyond
  diffusion from one centre seed.
- **P2:** the mute regime's field equals the field-alone field
  exactly, while its spikes number > 0 — a read-only network cannot
  move the field it reads.
- **P3:** closed spikes < deaf spikes — where V grows the reaction
  consumes U, so a neuron that reads its field is throttled by its
  own pattern; a deaf neuron is not.
- **P4:** closed mean V at neuron sites exceeds the global mean V —
  injections land on the lattice; the pattern anchors there.

**Falsification condition:** if closed coverage does not exceed
field-alone coverage, the bidirectional loop does not spread the
pattern in this window and the spikes are decoration.

## The ledger (2,000 ticks per regime)

| regime | spikes | V coverage | V mass | U mean | V at neuron sites |
|---|---|---|---|---|---|
| closed | 4,540 | **1,676 / 3,200 (52.4%)** | 468.8 | 0.5998 | 0.4352 |
| mute | 10,580 | 21 / 3,200 (0.7%) | 5.2 | 0.9955 | 0.0018 |
| deaf | 10,656 | 2,120 / 3,200 (66.2%) | 547.9 | 0.5416 | 0.4719 |
| field alone | 0 | 21 / 3,200 (0.7%) | 5.2 | 0.9955 | 0.0018 |

Every prediction **HELD** — and the honest twist is in the deaf row.
P1 held by two orders of magnitude (1,676 vs 21): one centre seed
barely diffuses in 2,000 ticks (21 cells), so in this window the
pattern *is* the spikes. P2 held exactly: the mute field is the
field-alone field cell for cell (a test pins it), while the mute
network fired 10,580 spikes into a field that never noticed. P3
held decisively (4,540 vs 10,656) — and it cuts both ways: the deaf
network, never throttled, out-spreads the closed loop (66.2% vs
52.4%). Reading the field is a **brake, not an accelerator**: the
loop spreads the pattern and then starves its own drivers as U
falls (closed U mean 0.5998; at neuron sites V sits at 0.4352
against a global mean of 0.1465 — P4, the pattern anchored to the
lattice that made it). The corpse built a self-limiting
morphogenesis and died before anyone measured the limit.

## Tests

6/6: the lattice and wiring match the corpse's numbers (200
neurons, 596 synapses, endpoints (0,0) and (76,36)); the wiring is
deterministic; a mute network cannot move its field (final V
identical to field-alone, spikes > 0); the closed loop spikes,
injects, and anchors V at neuron sites; runs are reproducible to
the cell; the deaf network spikes and injects with no field to read.

## Succession

Run the window long (10,000+ ticks): diffusion eventually spreads
on its own — does the field-alone coverage ever catch the closed
loop's, and does the closed loop's brake become its advantage
(a pattern that stops consuming its ground) or its ceiling?
Or vary the lattice spacing: at spacing 8 the injections are
sparser — is there a spacing where P1 fails and the spikes really
are decoration?
