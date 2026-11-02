//! # Diffusion Cortex 🗺️
//!
//! Brought by The Cartographer of Lost Things 🗺️. A revival cross:
//! graveyard/neuro-diffusion × crates/gray-scott + crates/neuro-sim.
//!
//! The archived neuro-diffusion (217 LOC, edition 2021) was a complete
//! bidirectional morphogenesis loop: an Izhikevich network embedded in
//! a Gray-Scott reaction-diffusion field, each neuron reading the U
//! chemical at its cell as input current (U × 10), and every spike
//! injecting the V chemical back into the field at the neuron's cell.
//! The continuous field drove the discrete spikes; the discrete spikes
//! seeded the continuous pattern. It was executed for Documentation
//! Void alone — no README, main straight to a TUI, no headless bypass —
//! "elaborate neural morphogenesis hidden behind complete lack of
//! documentation", in its forensic report's own words. Its loop was
//! never counted: nobody knows whether the spikes spread the pattern,
//! anchor it, or merely decorate a diffusion that would have happened
//! anyway.
//!
//! The living garden donates what the corpse lacked, and both donors
//! are the corpse's own dependencies, still alive: `crates/gray-scott`
//! (the field) and `crates/neuro-sim` (the network). What neither
//! living crate does alone, and the corpse never did at all, is
//! *price the loop*: this specimen runs the corpse's loop verbatim
//! beside three ablations on identical ground and identical wiring —
//! a mute network (reads U, spikes counted, injects nothing), a deaf
//! network (constant current 10.0, the value a neuron in pristine
//! U = 1.0 field reads, spikes still inject), and the field alone
//! (no network at all). The loop's contribution is whatever the
//! closed ledger has that the ablations do not.
//!
//! The corpse's numbers, verbatim: field 80×40, neurons on a spacing-4
//! lattice (20 × 10 = 200), 3 synapse draws per neuron (weight 15.0
//! with p = 0.8, else −10.0; delay 1..=10; self-draws skipped), one
//! centre seed of V (add_chemical at (40, 20), 1.0), Gray-Scott
//! mitosis parameters f = 0.0367, k = 0.0649, dt = 1.0, one field step
//! per network step, in the corpse's order: read U → step network →
//! inject V at spiking cells → step field.
//!
//! Adaptations, named: the corpse drew its wiring from `thread_rng`;
//! here a fixed-seed SplitMix64 stream draws the identical distribution
//! so every number is reproducible. The corpse's `get_index` returned
//! a bare `usize`; the living gray-scott returns `Option<usize>` — the
//! call sites unwrap the in-bounds case. There is no TUI here;
//! `--headless` is handled before anything else regardless.
//!
//! Predictions, declared before the first run and never edited:
//! - P1: the closed loop's final V coverage (cells with V > 0.1)
//!   exceeds the field-alone coverage — spike injections seed V at
//!   200 lattice sites, far beyond what diffusion from one centre
//!   seed reaches in the same ticks.
//! - P2: the mute regime's field ledger equals the field-alone field
//!   ledger exactly, while its spike count is greater than zero —
//!   a read-only network cannot move the field it reads.
//! - P3: the closed loop spikes less than the deaf network — where V
//!   grows, the reaction consumes U, so a neuron that reads its field
//!   is throttled by its own pattern; a deaf neuron is not.
//! - P4: in the closed loop, mean V at neuron sites exceeds the
//!   field's global mean V — injections land on the lattice, so the
//!   pattern is anchored where the neurons are.
//!
//! Falsification condition: if the closed coverage does not exceed
//! the field-alone coverage (P1 fails), the bidirectional loop does
//! not spread the pattern in this window and the spikes are
//! decoration on a diffusion that ignores them.
//!
//! `cargo run -p diffusion-cortex -- --headless` prints all four
//! regimes' ledgers.

use anyhow::Result;
use gray_scott::GrayScott;
use neuro_sim::Network;

// -----------------------------------------------------------------
// Deterministic RNG (SplitMix64), as elsewhere in the garden.
// -----------------------------------------------------------------
struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
    fn frac(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

const W: usize = 80;
const H: usize = 40;
const SPACING: usize = 4;
const FEED: f32 = 0.0367; // the corpse's mitosis feed
const KILL: f32 = 0.0649; // the corpse's mitosis kill
const TICKS: usize = 2000;
const SEED: u64 = 0xD1FF_0510;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Regime {
    /// The corpse verbatim: read U, step, inject on spike, step field.
    Closed,
    /// Reads U and spikes, injects nothing.
    Mute,
    /// Constant current 10.0, spikes inject.
    Deaf,
    /// No network at all.
    FieldAlone,
}

struct Ledger {
    regime: Regime,
    neurons: usize,
    synapses: usize,
    spikes: u64,
    v_coverage: usize,
    v_mass: f64,
    v_max: f32,
    u_mean: f64,
    v_mean: f64,
    v_at_neurons: f64,
    final_v: Vec<f32>,
}

/// Build the corpse's wiring on a fixed-seed stream: neurons on the
/// spacing-4 lattice, 3 draws each, 80% excitatory 15.0 / 20%
/// inhibitory −10.0, delay 1..=10, self-draws skipped.
fn build_network() -> (Network, Vec<(usize, usize)>) {
    let mut net = Network::new();
    let mut positions = Vec::new();
    for y in (0..H).step_by(SPACING) {
        for x in (0..W).step_by(SPACING) {
            net.add_neuron();
            positions.push((x, y));
        }
    }
    let n = positions.len();
    let mut rng = Rng(SEED);
    for i in 0..n {
        for _ in 0..3 {
            let target = rng.below(n);
            if target != i {
                let weight = if rng.frac() < 0.8 { 15.0 } else { -10.0 };
                let delay = 1 + rng.below(10);
                net.add_synapse_with_delay(i, target, weight, delay);
            }
        }
    }
    (net, positions)
}

fn seeded_field() -> GrayScott {
    let mut gs = GrayScott::new(W, H);
    gs.add_chemical(W / 2, H / 2, 1.0); // the corpse's single centre seed
    gs
}

fn run(regime: Regime, ticks: usize) -> Ledger {
    let mut gs = seeded_field();
    let (mut net, positions) = build_network();
    let neurons = positions.len();
    let synapses = net.synapses.len();
    let mut spikes: u64 = 0;

    if regime == Regime::FieldAlone {
        for _ in 0..ticks {
            gs.update(FEED, KILL, 1.0);
        }
    } else {
        let mut inputs = vec![0.0f32; neurons];
        for _ in 0..ticks {
            // 1. Read the field (or not, if deaf).
            for (i, &(x, y)) in positions.iter().enumerate() {
                inputs[i] = match regime {
                    Regime::Deaf => 10.0,
                    _ => {
                        let idx = gs.get_index(x, y).expect("neuron in bounds");
                        gs.u()[idx] * 10.0
                    }
                };
            }
            // 2. Step the network.
            net.step(&inputs);
            // 3. Inject V where neurons spiked (unless mute).
            for (i, &(x, y)) in positions.iter().enumerate() {
                if net.is_spiking(i) {
                    spikes += 1;
                    if regime != Regime::Mute {
                        gs.add_chemical(x, y, 1.0);
                    }
                }
            }
            // 4. Step the field.
            gs.update(FEED, KILL, 1.0);
        }
    }

    let v = gs.v();
    let u = gs.u();
    let cells = (W * H) as f64;
    let v_coverage = v.iter().filter(|&&c| c > 0.1).count();
    let v_mass: f64 = v.iter().map(|&c| c as f64).sum();
    let v_max = v.iter().cloned().fold(0.0f32, f32::max);
    let u_mean: f64 = u.iter().map(|&c| c as f64).sum::<f64>() / cells;
    let v_at_neurons: f64 = if regime == Regime::FieldAlone {
        // Same lattice, for comparison: where neurons would have sat.
        let mut sum = 0.0f64;
        let mut count = 0usize;
        for y in (0..H).step_by(SPACING) {
            for x in (0..W).step_by(SPACING) {
                let idx = gs.get_index(x, y).expect("site in bounds");
                sum += v[idx] as f64;
                count += 1;
            }
        }
        sum / count as f64
    } else {
        let sum: f64 = positions
            .iter()
            .map(|&(x, y)| v[gs.get_index(x, y).expect("neuron in bounds")] as f64)
            .sum();
        sum / neurons as f64
    };

    Ledger {
        regime,
        neurons: if regime == Regime::FieldAlone { 0 } else { neurons },
        synapses: if regime == Regime::FieldAlone { 0 } else { synapses },
        spikes,
        v_coverage,
        v_mass,
        v_max,
        u_mean,
        v_mean: v_mass / cells,
        v_at_neurons,
        final_v: v.to_vec(),
    }
}

fn print_ledger(l: &Ledger) {
    let name = match l.regime {
        Regime::Closed => "closed (corpse verbatim)",
        Regime::Mute => "mute (reads U, injects nothing)",
        Regime::Deaf => "deaf (constant current, injects)",
        Regime::FieldAlone => "field alone (no network)",
    };
    println!(
        "  regime '{name}': neurons {}, synapses {}, spikes {}",
        l.neurons, l.synapses, l.spikes
    );
    println!(
        "    V coverage (V > 0.1): {} of {} cells ({:.1}%), V mass {:.1}, V max {:.3}",
        l.v_coverage,
        W * H,
        100.0 * l.v_coverage as f64 / (W * H) as f64,
        l.v_mass,
        l.v_max
    );
    println!(
        "    U mean {:.4}, V mean {:.4}, V at neuron sites {:.4}",
        l.u_mean, l.v_mean, l.v_at_neurons
    );
}

fn headless() {
    println!(
        "diffusion-cortex headless: one morphogenesis loop, priced against its ablations"
    );
    println!(
        "  field {}x{}, neurons on spacing-{} lattice, {} ticks per regime, f = {FEED}, k = {KILL}",
        W, H, SPACING, TICKS
    );
    let closed = run(Regime::Closed, TICKS);
    let mute = run(Regime::Mute, TICKS);
    let deaf = run(Regime::Deaf, TICKS);
    let alone = run(Regime::FieldAlone, TICKS);
    for l in [&closed, &mute, &deaf, &alone] {
        print_ledger(l);
    }
    let p1 = closed.v_coverage > alone.v_coverage;
    let p2 = mute.final_v == alone.final_v && mute.spikes > 0;
    let p3 = closed.spikes < deaf.spikes;
    let p4 = closed.v_at_neurons > closed.v_mean;
    println!("  declared predictions, judged on the four ledgers:");
    println!(
        "    P1 the loop spreads the pattern (closed coverage > field alone): {} vs {} — {}",
        closed.v_coverage,
        alone.v_coverage,
        if p1 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 a read-only network cannot move its field (mute field == field alone, mute spikes > 0): spikes {} — {}",
        mute.spikes,
        if p2 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 reading the field throttles the spikes (closed < deaf): {} vs {} — {}",
        closed.spikes,
        deaf.spikes,
        if p3 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the pattern anchors to the lattice (closed V at neuron sites > closed V mean): {:.4} vs {:.4} — {}",
        closed.v_at_neurons,
        closed.v_mean,
        if p4 { "HELD" } else { "CRACKED" }
    );
}

fn main() -> Result<()> {
    // --headless before anything else; there is no TUI to initialise.
    if std::env::args().any(|a| a == "--headless") {
        headless();
        return Ok(());
    }
    println!("diffusion-cortex: run with --headless for the ledger (no TUI in this revival).");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lattice_and_wiring_match_the_corpse() {
        let (net, positions) = build_network();
        assert_eq!(positions.len(), 200); // 20 x 10 on spacing 4 over 80x40
        assert_eq!(net.neurons.len(), 200);
        // 600 draws, self-draws skipped: strictly fewer, but most land.
        assert!(net.synapses.len() > 500 && net.synapses.len() <= 600);
        assert_eq!(positions[0], (0, 0));
        assert_eq!(positions[199], (76, 36));
    }

    #[test]
    fn wiring_is_deterministic() {
        let (a, _) = build_network();
        let (b, _) = build_network();
        assert_eq!(a.synapses.len(), b.synapses.len());
        for (sa, sb) in a.synapses.iter().zip(b.synapses.iter()) {
            assert_eq!(sa.from, sb.from);
            assert_eq!(sa.to, sb.to);
            assert_eq!(sa.weight, sb.weight);
            assert_eq!(sa.delay, sb.delay);
        }
    }

    #[test]
    fn mute_network_cannot_move_its_field() {
        // P2's core, at test scale: the mute field is the field alone.
        let mute = run(Regime::Mute, 200);
        let alone = run(Regime::FieldAlone, 200);
        assert_eq!(mute.final_v, alone.final_v);
        assert!(mute.spikes > 0, "the mute network still spikes");
    }

    #[test]
    fn closed_loop_spikes_and_injects() {
        let closed = run(Regime::Closed, 200);
        assert!(closed.spikes > 0);
        // Injections land on neuron sites from the first ticks.
        assert!(closed.v_at_neurons > 0.0);
        assert!(closed.v_mass > 0.0);
    }

    #[test]
    fn runs_are_reproducible() {
        let a = run(Regime::Closed, 200);
        let b = run(Regime::Closed, 200);
        assert_eq!(a.spikes, b.spikes);
        assert_eq!(a.final_v, b.final_v);
    }

    #[test]
    fn deaf_network_spikes_without_a_field_to_read() {
        let deaf = run(Regime::Deaf, 200);
        assert!(deaf.spikes > 0);
        assert!(deaf.v_at_neurons > 0.0);
    }
}
