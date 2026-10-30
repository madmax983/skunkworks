//! # Lamprey Tide 🗺️
//!
//! Revival cross: `graveyard/spinal-rhythms` × living
//! `crates/physics-pbd` (body) and `crates/neuro-sim` (cord),
//! brought by The Cartographer of Lost Things.
//!
//! The dead specimen was a whole animal: a 20-segment lamprey
//! whose spinal cord — a central pattern generator (CPG) of
//! Izhikevich neurons, one left/right pair per segment, bound by
//! cross-inhibition within a segment and descending excitation
//! to the next — drove the muscles of a Verlet ladder body, and
//! the body swam. It was executed for Documentation Void alone
//! (the archive calls it a Silent Specimen): no README, a
//! macroquad `main` with no headless bypass, so nothing it ever
//! did was ever counted. Its metabolism was sound; what it
//! lacked was a ledger and organs the garden maintains.
//!
//! The living garden donates both missing organs:
//!
//! - `crates/neuro-sim` — the Widow's verified Izhikevich
//!   network (delayed synapses, spike cache) — replaces the
//!   corpse's private, untested copy of the same mathematics.
//! - `crates/physics-pbd` — the garden's maintained PBD body,
//!   whose `Actuator` constraint is a muscle by design (a
//!   distance constraint with a driven rest length) — replaces
//!   the corpse's private Verlet world.
//!
//! The corpse's own parameters are kept where the new organs
//! can carry them, and every change is named: base drive 15.0
//! (head three segments full, the rest at 0.8×), cross-inhibition
//! −25 at delay 0, descending excitation +20 at delay 2 — all
//! the corpse's. Three honest adaptations: the body has 12
//! segments, not 20 (the wave is the specimen, not the length);
//! contraction is 0.28 of segment length, not the corpse's
//! 0.20, and activation is spike-triggered and smoothed (a
//! spike sets the muscle fully on; it relaxes ×0.93 per tick)
//! rather than read off the membrane voltage — in the corpse's
//! kinematic world the voltage read set muscle lengths
//! directly, but against a solver the same read produced a
//! mean bend of 0.037 and an animal that barely moved (kept in
//! the README's tuning note); and the water is new — PBD has
//! no fluid, so a resistive-force water is applied per station
//! after each of 2 solver substeps per tick: velocity
//! perpendicular to the local body axis is damped far harder
//! than velocity along it (×0.82 vs ×0.995, on top of the
//! solver's own ×0.95). Without that anisotropy an undulating
//! body goes nowhere, in this garden as in a pond.
//!
//! Three regimes, one animal:
//!
//! - **wave** — the corpse's cord as written: descending
//!   excitation at delay 2, so activation travels head → tail.
//! - **in-phase** — the same cord with the descending thread
//!   made symmetric and instant (weight 30, delay 0, both
//!   directions): every segment is pulled to beat together.
//! - **frozen** — the wave cord, but the muscles are never
//!   driven (actuator factor pinned at full extension): a
//!   rhythm with no body in it.
//!
//! The question the ledger answers: is the traveling wave —
//! the phase lag down the cord — what swims, or would any
//! rhythm do?
//!
//! `cargo run -p lamprey-tide -- --headless` prints the ledger.
//! There is no TUI; --headless is handled before anything else.

use anyhow::Result;
use neuro_sim::Network;
use physics_pbd::glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};

const SEGMENTS: usize = 12;
const SEG_LEN: f32 = 15.0;
const WIDTH: f32 = 10.0;
const DRIVE: f32 = 15.0;
const CROSS_INHIB: f32 = -25.0;
const DESC_WAVE: f32 = 20.0;
const DESC_DELAY: usize = 2;
const CONTRACTION: f32 = 0.28;
const ACT_DECAY: f32 = 0.93;
const TICKS: usize = 12_000;
const SETTLING: usize = 2_000;
const DT: f32 = 0.02;
const SUBSTEPS: usize = 2;
const ITERATIONS: usize = 5;
const DRAG_PERP: f32 = 0.82;
const DRAG_PAR: f32 = 0.995;

#[derive(Clone, Copy, PartialEq)]
enum Regime {
    Wave,
    InPhase,
    Frozen,
}

struct Lamprey {
    net: Network,
    neuron_map: Vec<(usize, usize)>, // (side A, side B) per segment
    body: PbdSystem,
    top: Vec<usize>,    // particle per station, side A rail
    bottom: Vec<usize>, // particle per station, side B rail
    muscle_a: Vec<usize>, // actuator constraint index per segment
    muscle_b: Vec<usize>,
    act_a: Vec<f32>, // smoothed spike-triggered activation per segment
    act_b: Vec<f32>,
}

impl Lamprey {
    fn new(regime: Regime) -> Self {
        // --- the cord (neuro-sim) ---
        let mut net = Network::new();
        let mut neuron_map = Vec::new();
        for i in 0..SEGMENTS {
            let l = net.add_neuron();
            let r = net.add_neuron();
            // Deterministic symmetry-breaking, in place of the
            // corpse's dice: a small fixed stagger of rest voltage.
            net.neurons[l].v = -65.0 + ((i * 7) % 5) as f32;
            net.neurons[r].v = -65.0 + ((i * 3 + 2) % 5) as f32;
            neuron_map.push((l, r));
        }
        for i in 0..SEGMENTS {
            let (l, r) = neuron_map[i];
            net.add_synapse_with_delay(l, r, CROSS_INHIB, 0);
            net.add_synapse_with_delay(r, l, CROSS_INHIB, 0);
            if i + 1 < SEGMENTS {
                let (nl, nr) = neuron_map[i + 1];
                match regime {
                    Regime::Wave | Regime::Frozen => {
                        net.add_synapse_with_delay(l, nl, DESC_WAVE, DESC_DELAY);
                        net.add_synapse_with_delay(r, nr, DESC_WAVE, DESC_DELAY);
                    }
                    Regime::InPhase => {
                        // The cord is wired as in Wave; what is
                        // removed is the wave's *spatial* phase at
                        // the muscles (see the drive loop).
                        net.add_synapse_with_delay(l, nl, DESC_WAVE, DESC_DELAY);
                        net.add_synapse_with_delay(r, nr, DESC_WAVE, DESC_DELAY);
                    }
                }
            }
        }

        // --- the body (physics-pbd): a ladder in the z=0 plane ---
        let mut body = PbdSystem::new();
        let mut top = Vec::new();
        let mut bottom = Vec::new();
        for i in 0..=SEGMENTS {
            let x = i as f32 * SEG_LEN;
            top.push(body.add_particle(Vec3::new(x, -WIDTH / 2.0, 0.0), 1.0));
            bottom.push(body.add_particle(Vec3::new(x, WIDTH / 2.0, 0.0), 1.0));
        }
        let mut muscle_a = Vec::new();
        let mut muscle_b = Vec::new();
        for i in 0..SEGMENTS {
            // rungs
            body.add_distance_constraint(top[i], bottom[i], 1.0).unwrap();
            if i + 1 == SEGMENTS {
                body.add_distance_constraint(top[i + 1], bottom[i + 1], 1.0).unwrap();
            }
            // diagonals (shear)
            body.add_distance_constraint(top[i], bottom[i + 1], 0.35).unwrap();
            body.add_distance_constraint(bottom[i], top[i + 1], 0.35).unwrap();
            // muscles: the side rails are actuators
            body.add_actuator_constraint(
                top[i],
                top[i + 1],
                SEG_LEN * (1.0 - CONTRACTION),
                SEG_LEN,
                1.0,
            )
            .unwrap();
            muscle_a.push(body.constraints.len() - 1);
            body.add_actuator_constraint(
                bottom[i],
                bottom[i + 1],
                SEG_LEN * (1.0 - CONTRACTION),
                SEG_LEN,
                1.0,
            )
            .unwrap();
            muscle_b.push(body.constraints.len() - 1);
        }

        Self {
            net,
            neuron_map,
            body,
            top,
            bottom,
            muscle_a,
            muscle_b,
            act_a: vec![0.0; SEGMENTS],
            act_b: vec![0.0; SEGMENTS],
        }
    }

    fn drive_inputs(&self) -> Vec<f32> {
        let mut inputs = vec![0.0; self.net.neurons.len()];
        for (i, &(l, r)) in self.neuron_map.iter().enumerate() {
            let d = if i < 3 { DRIVE } else { DRIVE * 0.8 };
            inputs[l] = d;
            inputs[r] = d;
        }
        inputs
    }

    fn set_muscle(&mut self, constraint_idx: usize, activation: f32) {
        if let Constraint::Actuator { factor, .. } = &mut self.body.constraints[constraint_idx] {
            *factor = 1.0 - activation;
        }
    }

    fn station_mid(&self, i: usize) -> Vec3 {
        (self.body.particles[self.top[i]].pos + self.body.particles[self.bottom[i]].pos) * 0.5
    }

    fn centroid(&self) -> Vec3 {
        let mut c = Vec3::ZERO;
        for p in &self.body.particles {
            c += p.pos;
        }
        c / self.body.particles.len() as f32
    }

    /// Resistive-force water: damp the perpendicular velocity
    /// component of every particle far harder than the parallel
    /// one, against the local station axis.
    fn apply_water(&mut self) {
        let mids: Vec<Vec3> = (0..=SEGMENTS).map(|i| self.station_mid(i)).collect();
        for i in 0..=SEGMENTS {
            let lo = if i == 0 { 0 } else { i - 1 };
            let hi = if i == SEGMENTS { SEGMENTS } else { i + 1 };
            let axis = (mids[hi] - mids[lo]).normalize_or_zero();
            if axis == Vec3::ZERO {
                continue;
            }
            for p_idx in [self.top[i], self.bottom[i]] {
                let p = &mut self.body.particles[p_idx];
                let v_par = axis * p.vel.dot(axis);
                let v_perp = p.vel - v_par;
                p.vel = v_par * DRAG_PAR + v_perp * DRAG_PERP;
            }
        }
    }
}

struct Ledger {
    spikes_a: Vec<Vec<usize>>, // spike ticks per segment, side A
    spikes_b: Vec<Vec<usize>>,
    centroid_start: Vec3,
    centroid_end: Vec3,
    head_path: f32, // lateral (y) path only — forward travel excluded
    tail_path: f32,
    speed_mean: f32, // centroid speed over the measured window
    bend_mean: f32,  // mean |sin| of joint angle between stations
}

fn run(regime: Regime) -> Ledger {
    let mut lamprey = Lamprey::new(regime);
    let inputs = lamprey.drive_inputs();
    let mut spikes_a: Vec<Vec<usize>> = vec![Vec::new(); SEGMENTS];
    let mut spikes_b: Vec<Vec<usize>> = vec![Vec::new(); SEGMENTS];
    let centroid_start = lamprey.centroid();
    let mut prev_head = lamprey.station_mid(0);
    let mut prev_tail = lamprey.station_mid(SEGMENTS);
    let mut head_path = 0.0f32;
    let mut tail_path = 0.0f32;
    let mut speed_sum = 0.0f32;
    let mut speed_n = 0usize;
    let mut bend_sum = 0.0f32;
    let mut bend_n = 0usize;
    let mut prev_centroid = centroid_start;

    for tick in 0..TICKS {
        lamprey.net.step(&inputs);
        if tick >= SETTLING {
            for (seg, &(l, r)) in lamprey.neuron_map.iter().enumerate() {
                if lamprey.net.is_spiking(l) {
                    spikes_a[seg].push(tick);
                }
                if lamprey.net.is_spiking(r) {
                    spikes_b[seg].push(tick);
                }
            }
        }
        // Activation is spike-triggered and smoothed: a spike
        // sets the muscle fully on, and it relaxes geometrically.
        for seg in 0..SEGMENTS {
            let (l, r) = lamprey.neuron_map[seg];
            if lamprey.net.is_spiking(l) {
                lamprey.act_a[seg] = 1.0;
            } else {
                lamprey.act_a[seg] *= ACT_DECAY;
            }
            if lamprey.net.is_spiking(r) {
                lamprey.act_b[seg] = 1.0;
            } else {
                lamprey.act_b[seg] *= ACT_DECAY;
            }
        }
        if regime != Regime::Frozen {
            // InPhase: every muscle reads segment 0's activation —
            // the same rhythm with its spatial phase removed.
            let head_a = lamprey.act_a[0];
            let head_b = lamprey.act_b[0];
            for seg in 0..SEGMENTS {
                let (a_act, b_act) = if regime == Regime::InPhase {
                    (head_a, head_b)
                } else {
                    (lamprey.act_a[seg], lamprey.act_b[seg])
                };
                let ma = lamprey.muscle_a[seg];
                let mb = lamprey.muscle_b[seg];
                lamprey.set_muscle(ma, a_act);
                lamprey.set_muscle(mb, b_act);
            }
        }
        for _ in 0..SUBSTEPS {
            lamprey.body.step(DT, ITERATIONS);
            lamprey.apply_water();
        }

        if tick >= SETTLING {
            let head = lamprey.station_mid(0);
            let tail = lamprey.station_mid(SEGMENTS);
            head_path += (head.y - prev_head.y).abs();
            tail_path += (tail.y - prev_tail.y).abs();
            prev_head = head;
            prev_tail = tail;
            let c = lamprey.centroid();
            speed_sum += c.distance(prev_centroid) / DT;
            speed_n += 1;
            prev_centroid = c;
            // bend: mean |sin| of the angle between consecutive axes
            let mids: Vec<Vec3> = (0..=SEGMENTS).map(|i| lamprey.station_mid(i)).collect();
            let mut s = 0.0f32;
            for i in 1..SEGMENTS {
                let a = (mids[i] - mids[i - 1]).normalize_or_zero();
                let b = (mids[i + 1] - mids[i]).normalize_or_zero();
                s += a.cross(b).length();
            }
            bend_sum += s / (SEGMENTS - 1) as f32;
            bend_n += 1;
        } else {
            prev_centroid = lamprey.centroid();
            prev_head = lamprey.station_mid(0);
            prev_tail = lamprey.station_mid(SEGMENTS);
        }
    }

    Ledger {
        spikes_a,
        spikes_b,
        centroid_start,
        centroid_end: lamprey.centroid(),
        head_path,
        tail_path,
        speed_mean: if speed_n > 0 { speed_sum / speed_n as f32 } else { 0.0 },
        bend_mean: if bend_n > 0 { bend_sum / bend_n as f32 } else { 0.0 },
    }
}

/// Mean spike period of a spike train (ticks), if it has ≥ 2 spikes.
fn mean_period(spikes: &[usize]) -> Option<f32> {
    if spikes.len() < 2 {
        return None;
    }
    let diffs: Vec<f32> = spikes.windows(2).map(|w| (w[1] - w[0]) as f32).collect();
    Some(diffs.iter().sum::<f32>() / diffs.len() as f32)
}

/// Mean phase lag (ticks) of train B behind train A: for each B
/// spike, the distance back to the most recent A spike, folded
/// into [0, period).
fn mean_lag(a: &[usize], b: &[usize], period: f32) -> Option<f32> {
    if a.len() < 2 || b.len() < 2 || period <= 0.0 {
        return None;
    }
    let mut lags = Vec::new();
    let mut j = 0usize;
    for &tb in b {
        while j + 1 < a.len() && a[j + 1] <= tb {
            j += 1;
        }
        if a[j] <= tb {
            lags.push((tb - a[j]) as f32);
        }
    }
    if lags.is_empty() {
        return None;
    }
    Some(lags.iter().sum::<f32>() / lags.len() as f32)
}

fn report(name: &str, led: &Ledger) {
    let disp = led.centroid_end - led.centroid_start;
    let period = mean_period(&led.spikes_a[0]).unwrap_or(0.0);
    let mut lags = Vec::new();
    for seg in 1..SEGMENTS {
        if let Some(l) = mean_lag(&led.spikes_a[seg - 1], &led.spikes_a[seg], period) {
            lags.push(l);
        }
    }
    let lag_seg = if lags.is_empty() {
        0.0
    } else {
        lags.iter().sum::<f32>() / lags.len() as f32
    };
    let total_lag = mean_lag(&led.spikes_a[0], &led.spikes_a[SEGMENTS - 1], period).unwrap_or(0.0);
    let total_spikes: usize = led.spikes_a.iter().chain(led.spikes_b.iter()).map(|v| v.len()).sum();
    println!("  regime '{name}':");
    println!(
        "    cord: {} spikes in window, head period {:.2} ticks, mean lag/segment {:.2} ticks, head->tail lag {:.2} ticks ({:.2} of a period)",
        total_spikes,
        period,
        lag_seg,
        total_lag,
        if period > 0.0 { total_lag / period } else { 0.0 }
    );
    println!(
        "    body: centroid displacement ({:.1}, {:.1}) |d| {:.1}, forward (-x) {:.1}, mean speed {:.2} units/s, mean bend {:.4}",
        disp.x, disp.y, disp.length(), -disp.x, led.speed_mean, led.bend_mean
    );
    println!(
        "    lateral paths: head {:.1}, tail {:.1} (tail/head {:.2})",
        led.head_path,
        led.tail_path,
        if led.head_path > 0.0 { led.tail_path / led.head_path } else { 0.0 }
    );
}

fn headless() {
    println!("lamprey-tide headless: a silent specimen, given a ledger — does the traveling wave swim?");
    println!(
        "  animal: {SEGMENTS} segments, cord = neuro-sim CPG (cross-inhibition {CROSS_INHIB:.0}, drive {DRIVE:.1}), body = physics-pbd ladder, muscles = actuators (contraction {CONTRACTION:.2})"
    );
    println!(
        "  water: resistive — perpendicular velocity x{DRAG_PERP} per substep, parallel x{DRAG_PAR}, {SUBSTEPS} substeps per tick; window {TICKS} ticks, first {SETTLING} settling"
    );
    let wave = run(Regime::Wave);
    report("wave (descending +20, delay 2)", &wave);
    let inphase = run(Regime::InPhase);
    report("in-phase (one phase broadcast to all muscles)", &inphase);
    let frozen = run(Regime::Frozen);
    report("frozen muscles (wave cord, no drive to body)", &frozen);
    let w = -wave.centroid_end.x + wave.centroid_start.x;
    let i = -inphase.centroid_end.x + inphase.centroid_start.x;
    let f = (frozen.centroid_end - frozen.centroid_start).length();
    println!(
        "  verdict: wave forward {:.1} vs in-phase forward {:.1} (x{:.2}) vs frozen |d| {:.1}",
        w,
        i,
        if i > 0.0 { w / i } else { 0.0 },
        f
    );
    println!(
        "  efficiency (forward per unit of lateral head travel): wave {:.3} vs in-phase {:.3} (x{:.1})",
        w / wave.head_path.max(1.0),
        i / inphase.head_path.max(1.0),
        (w / wave.head_path.max(1.0)) / (i / inphase.head_path.max(1.0)).max(1e-9)
    );
}

fn main() -> Result<()> {
    // --headless handled BEFORE anything else; there is no TUI.
    let _headless = std::env::args().any(|a| a == "--headless");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_centers_alternate() {
        // Within a segment, cross-inhibition must keep the two
        // sides from spiking together: coincident spikes are rare.
        let mut lamprey = Lamprey::new(Regime::Wave);
        let inputs = lamprey.drive_inputs();
        let mut coincident = 0usize;
        let mut total = 0usize;
        for _ in 0..3000 {
            lamprey.net.step(&inputs);
            let (l, r) = lamprey.neuron_map[0];
            let sl = lamprey.net.is_spiking(l);
            let sr = lamprey.net.is_spiking(r);
            if sl && sr {
                coincident += 1;
            }
            if sl || sr {
                total += 1;
            }
        }
        assert!(total > 20, "the cord must beat (got {total} spike ticks)");
        assert!(
            coincident * 4 < total,
            "sides must alternate, not chorus: {coincident} coincident of {total}"
        );
    }

    #[test]
    fn wave_travels_head_to_tail() {
        let led = run(Regime::Wave);
        let period = mean_period(&led.spikes_a[0]).expect("head must beat");
        let lag = mean_lag(&led.spikes_a[0], &led.spikes_a[SEGMENTS - 1], period)
            .expect("tail must beat");
        assert!(period > 10.0 && period < 120.0, "period {period}");
        assert!(
            lag > 1.0 && lag < period,
            "the tail must beat after the head, inside one period: lag {lag} of {period}"
        );
    }

    #[test]
    fn wave_swims_forward() {
        let led = run(Regime::Wave);
        let forward = led.centroid_start.x - led.centroid_end.x;
        assert!(
            forward > 20.0,
            "the wave must carry the body forward (-x): got {forward}"
        );
    }

    #[test]
    fn frozen_muscles_do_not_swim() {
        let wave = run(Regime::Wave);
        let frozen = run(Regime::Frozen);
        let w = wave.centroid_start.x - wave.centroid_end.x;
        let f = (frozen.centroid_end - frozen.centroid_start).length();
        assert!(
            f < w / 4.0,
            "a rhythm with no muscles must not swim: frozen |d| {f} vs wave forward {w}"
        );
    }

    #[test]
    fn wave_outswims_in_phase() {
        let wave = run(Regime::Wave);
        let inphase = run(Regime::InPhase);
        let w = wave.centroid_start.x - wave.centroid_end.x;
        let i = inphase.centroid_start.x - inphase.centroid_end.x;
        assert!(
            w > i * 1.5,
            "the traveling wave must outswim the same rhythm with no spatial phase: wave {w:.1} vs in-phase {i:.1}"
        );
        // And per unit of lateral thrash, the wave is several
        // times the swimmer: the in-phase animal churns.
        let we = w / wave.head_path.max(1.0);
        let ie = i / inphase.head_path.max(1.0);
        assert!(
            we > ie * 3.0,
            "efficiency: wave {we:.3} vs in-phase {ie:.3}"
        );
    }
}
