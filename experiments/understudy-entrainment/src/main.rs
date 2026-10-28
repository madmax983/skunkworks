//! # Understudy Entrainment 🎭
//!
//! Brought by The Understudy 🎭. Principal: `experiments/widow-entrainment`
//! (The Clockwork Widow 🕰️), where two Izhikevich spiking neurons —
//! a Regular Spiking driver (solo period 39.62 ticks) and a Fast
//! Spiking follower (solo period 44.52) joined by one synapse —
//! drift at no coupling, tug at weak coupling, and lock at strong.
//!
//! The Understudy performs the same role with the machinery thrown
//! out entire: no membrane, no recovery variable, no threshold, no
//! reset, no synaptic kick. Each clock is one continuous phase on a
//! ring, and the "synapse" is a smooth sinusoidal pull on the
//! follower's phase velocity — the Adler equation:
//!
//!   dθ_f/dt = ω_f + K · sin(θ_d − θ_f)
//!
//! A "beat" is a phase crossing of the next multiple of 2π. Beats
//! are then counted with the principal's own ledger definitions
//! (matched / missed / extra driver cycles, circular dispersion of
//! the follower's phase within the driver cycle), so the audience
//! compares performances, not programmes.
//!
//! Tolerances were declared in the README *before* this crate was
//! built or run. The headless ledger prints the principal's numbers
//! beside the understudy's and names each line HELD or CRACKED.
//!
//! `cargo run -p understudy-entrainment -- --headless` prints the
//! comparison ledger. There is no TUI; --headless is handled before
//! anything else regardless.

use anyhow::Result;
use std::f64::consts::TAU;

/// The role: the principal's published solo periods, in ticks.
/// These are the lines the understudy was given — the natural
/// frequencies below are built from them, and the ledger checks
/// that the crossing machinery actually reproduces them.
const DRIVER_PERIOD: f64 = 39.62;
const FOLLOWER_PERIOD: f64 = 44.52;

/// Total ticks per regime, matching the principal's window.
const TICKS: usize = 6_000;
/// Ticks discarded at the start of each regime while the pair
/// settles — the principal discards the same 1,500.
const SETTLE_TICKS: usize = 1_500;
/// Integration substeps per tick (dt = 0.05 tick). The machinery is
/// continuous; the stage is sampled finely enough that a crossing
/// time is never guessed coarser than a twentieth of a tick.
const SUBSTEPS: usize = 20;

/// Coupling teeth. The frequency gap is Δω = ω_f − ω_d ≈
/// −0.01747 rad/tick; a phase pair locks only when K > |Δω|.
/// The weak tooth sits just *under* that threshold, where the
/// Adler equation predicts frequency pulling (mean period ≈ 41.4)
/// without a lock; the strong tooth sits comfortably over it.
/// Chosen by analysis before the first run, per the README.
const COUPLINGS: [(&str, f64); 3] = [
    ("uncoupled", 0.0),
    ("weak thread", 0.0160),
    ("strong thread", 0.0600),
];

fn omega(period: f64) -> f64 {
    TAU / period
}

/// One clock measured alone: how many times it struck (phase
/// crossings), and the mean interval between strikes.
#[derive(Debug, Clone, Copy)]
struct SoloClock {
    beats: usize,
    mean_period: f64,
}

/// One coupling regime, measured with the principal's definitions.
#[derive(Debug, Clone)]
struct RegimeLedger {
    name: &'static str,
    k: f64,
    driver_beats: usize,
    follower_beats: usize,
    driver_period: f64,
    follower_period: f64,
    matched_cycles: usize,
    missed_beats: usize,
    extra_beats: usize,
    phase_dispersion: f64,
    mean_phase: f64,
    locked: bool,
}

fn mean_period(times: &[f64]) -> f64 {
    if times.len() < 2 {
        return 0.0;
    }
    let diffs: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
    diffs.iter().sum::<f64>() / diffs.len() as f64
}

/// Advance a phase pair for one substep; record any phase crossing
/// (beat) by linear interpolation inside the step, so a beat time
/// is a real time, not a step index wearing a costume.
#[allow(clippy::too_many_arguments)]
fn advance(
    theta_d: &mut f64,
    theta_f: &mut f64,
    omega_d: f64,
    omega_f: f64,
    k: f64,
    dt: f64,
    t: f64,
    driver_times: &mut Vec<f64>,
    follower_times: &mut Vec<f64>,
) {
    let pull = k * (*theta_d - *theta_f).sin();
    let nd = *theta_d + omega_d * dt;
    let nf = *theta_f + (omega_f + pull) * dt;
    for (old, new, times) in [
        (*theta_d, nd, &mut *driver_times),
        (*theta_f, nf, &mut *follower_times),
    ] {
        let before = (old / TAU).floor();
        let after = (new / TAU).floor();
        if after > before {
            let target = (before + 1.0) * TAU;
            let frac = if new > old {
                (target - old) / (new - old)
            } else {
                0.0
            };
            times.push(t + frac * dt);
        }
    }
    *theta_d = nd;
    *theta_f = nf;
}

/// Measure one clock alone (K = 0, the other clock absent).
fn measure_solo(period: f64) -> SoloClock {
    let w = omega(period);
    let dt = 1.0 / SUBSTEPS as f64;
    let mut theta = 0.0f64;
    let mut times: Vec<f64> = Vec::new();
    let mut sink_d: Vec<f64> = Vec::new();
    // Drive the solo clock through the same `advance` machinery by
    // parking a dummy partner far away with zero coupling: solo is
    // the pair with the pull switched off, measured honestly.
    let mut dummy = 0.0f64;
    for step in 0..TICKS * SUBSTEPS {
        let t = step as f64 * dt;
        let mut beats: Vec<f64> = Vec::new();
        advance(
            &mut theta,
            &mut dummy,
            w,
            0.0,
            0.0,
            dt,
            t,
            &mut beats,
            &mut sink_d,
        );
        for b in beats {
            if b >= SETTLE_TICKS as f64 {
                times.push(b);
            }
        }
    }
    SoloClock {
        beats: times.len(),
        mean_period: mean_period(&times),
    }
}

/// Run one coupling regime and take its ledger.
fn run_regime(name: &'static str, k: f64) -> RegimeLedger {
    let omega_d = omega(DRIVER_PERIOD);
    let omega_f = omega(FOLLOWER_PERIOD);
    let dt = 1.0 / SUBSTEPS as f64;
    let (mut theta_d, mut theta_f) = (0.0f64, 0.0f64);
    let mut driver_times: Vec<f64> = Vec::new();
    let mut follower_times: Vec<f64> = Vec::new();
    for step in 0..TICKS * SUBSTEPS {
        let t = step as f64 * dt;
        let (mut d, mut f) = (Vec::new(), Vec::new());
        advance(
            &mut theta_d,
            &mut theta_f,
            omega_d,
            omega_f,
            k,
            dt,
            t,
            &mut d,
            &mut f,
        );
        for b in d {
            if b >= SETTLE_TICKS as f64 {
                driver_times.push(b);
            }
        }
        for b in f {
            if b >= SETTLE_TICKS as f64 {
                follower_times.push(b);
            }
        }
    }

    // The principal's counting house, unchanged: walk the driver
    // cycles; count what the follower did inside each.
    let mut matched = 0usize;
    let mut missed = 0usize;
    let mut extra = 0usize;
    let mut phases: Vec<f64> = Vec::new();
    for w in driver_times.windows(2) {
        let (start, end) = (w[0], w[1]);
        let inside: Vec<f64> = follower_times
            .iter()
            .copied()
            .filter(|&f| f > start && f <= end)
            .collect();
        match inside.len() {
            0 => missed += 1,
            1 => matched += 1,
            n => {
                matched += 1;
                extra += n - 1;
            }
        }
        for f in inside {
            phases.push((f - start) / (end - start));
        }
    }

    // Circular statistics: phases live on a ring, not a line.
    let (mut sx, mut sy) = (0.0f64, 0.0f64);
    for p in &phases {
        let ang = p * TAU;
        sx += ang.cos();
        sy += ang.sin();
    }
    let n = phases.len().max(1) as f64;
    let r = ((sx / n).powi(2) + (sy / n).powi(2)).sqrt();
    let dispersion = 1.0 - r;
    let mut mean_phase = (sy / n).atan2(sx / n) / TAU;
    if mean_phase < 0.0 {
        mean_phase += 1.0;
    }

    let cycles = driver_times.len().saturating_sub(1).max(1);
    let locked = missed == 0 && extra == 0 && matched == cycles && dispersion < 0.05;

    RegimeLedger {
        name,
        k,
        driver_beats: driver_times.len(),
        follower_beats: follower_times.len(),
        driver_period: mean_period(&driver_times),
        follower_period: mean_period(&follower_times),
        matched_cycles: matched,
        missed_beats: missed,
        extra_beats: extra,
        phase_dispersion: dispersion,
        mean_phase,
        locked,
    }
}

fn run_all() -> (SoloClock, SoloClock, Vec<RegimeLedger>) {
    let driver_solo = measure_solo(DRIVER_PERIOD);
    let follower_solo = measure_solo(FOLLOWER_PERIOD);
    let regimes = COUPLINGS
        .iter()
        .map(|&(name, k)| run_regime(name, k))
        .collect();
    (driver_solo, follower_solo, regimes)
}

fn verdict(ok: bool) -> &'static str {
    if ok {
        "HELD"
    } else {
        "CRACKED"
    }
}

fn print_ledger(driver: &SoloClock, follower: &SoloClock, regimes: &[RegimeLedger]) {
    println!("understudy-entrainment headless: the Widow's role, performed by phases — no neurons on stage");
    println!("  window: {TICKS} ticks per regime, first {SETTLE_TICKS} discarded as settling (the principal's window)");
    println!(
        "  principal's lines: driver solo 39.62 (113 spikes), follower solo 44.52 (101 spikes), ratio 1.124"
    );
    println!(
        "  understudy solo: driver {} beats, period {:.2} [{} ±0.75 of 39.62]; follower {} beats, period {:.2} [{} ±0.75 of 44.52]",
        driver.beats,
        driver.mean_period,
        verdict((driver.mean_period - 39.62).abs() <= 0.75),
        follower.beats,
        follower.mean_period,
        verdict((follower.mean_period - 44.52).abs() <= 0.75),
    );
    // Principal's published regime lines, for the comparison.
    let principal: [(&str, usize, usize, f64, f64); 3] = [
        ("uncoupled", 100, 12, 44.52, 0.9664),
        ("weak thread", 107, 5, 41.43, 0.3840),
        ("strong thread", 112, 0, 39.62, 0.0000),
    ];
    for (r, p) in regimes.iter().zip(principal.iter()) {
        println!(
            "  regime '{}' (K = {:.4}): driver {} beats (period {:.2}), follower {} beats (period {:.2})",
            r.name, r.k, r.driver_beats, r.driver_period, r.follower_beats, r.follower_period
        );
        let (held, note) = match r.name {
            "uncoupled" => {
                let ok = (r.matched_cycles as i64 - p.1 as i64).abs() <= 8
                    && r.phase_dispersion >= 0.80;
                (
                    ok,
                    format!(
                        "matched {} vs principal {} [{} ±8]; dispersion {:.4} vs {:.4} [{} ≥0.80]",
                        r.matched_cycles,
                        p.1,
                        verdict((r.matched_cycles as i64 - p.1 as i64).abs() <= 8),
                        r.phase_dispersion,
                        p.4,
                        verdict(r.phase_dispersion >= 0.80),
                    ),
                )
            }
            "weak thread" => {
                let pulled = r.follower_period > DRIVER_PERIOD && r.follower_period < FOLLOWER_PERIOD;
                let ok = (r.follower_period - p.3).abs() <= 1.75
                    && pulled
                    && r.matched_cycles >= regimes[0].matched_cycles
                    && r.matched_cycles <= 111
                    && (0.10..=0.75).contains(&r.phase_dispersion)
                    && r.phase_dispersion < regimes[0].phase_dispersion;
                (
                    ok,
                    format!(
                        "period {:.2} vs principal {:.2} [{} ±1.75, pulled {}]; matched {} vs {} [{}]; dispersion {:.4} vs {:.4} [{} in 0.10..=0.75]",
                        r.follower_period,
                        p.3,
                        verdict((r.follower_period - p.3).abs() <= 1.75),
                        pulled,
                        r.matched_cycles,
                        p.1,
                        verdict(
                            r.matched_cycles >= regimes[0].matched_cycles && r.matched_cycles <= 111
                        ),
                        r.phase_dispersion,
                        p.4,
                        verdict((0.10..=0.75).contains(&r.phase_dispersion)),
                    ),
                )
            }
            _ => {
                let ok = r.matched_cycles >= 110
                    && (r.follower_period - p.3).abs() <= 0.75
                    && r.phase_dispersion <= 0.05;
                (
                    ok,
                    format!(
                        "matched {} vs principal {} [{} ≥110]; period {:.2} vs {:.2} [{} ±0.75]; dispersion {:.4} vs {:.4} [{} ≤0.05]",
                        r.matched_cycles,
                        p.1,
                        verdict(r.matched_cycles >= 110),
                        r.follower_period,
                        p.3,
                        verdict((r.follower_period - p.3).abs() <= 0.75),
                        r.phase_dispersion,
                        p.4,
                        verdict(r.phase_dispersion <= 0.05),
                    ),
                )
            }
        };
        println!(
            "    cycles matched {}/{}, missed beats {}, extra beats {}, mean phase {:.3}, phase dispersion {:.4} — {}",
            r.matched_cycles,
            r.matched_cycles + r.missed_beats,
            r.missed_beats,
            r.extra_beats,
            r.mean_phase,
            r.phase_dispersion,
            if r.locked { "LOCKED" } else { "DRIFTING" }
        );
        println!("    comparison: {} — {}", verdict(held), note);
    }
    println!("  the crack, named in advance and kept: a synapse kicks once per cycle and coasts; a phase pull holds continuously — the lock here is smoother than the principal's, and no ledger line can hear the difference.");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --headless is handled before anything else: no TUI exists in
    // this experiment, and none is ever initialised on this path.
    let headless = args.iter().any(|a| a == "--headless");
    let (driver, follower, regimes) = run_all();
    print_ledger(&driver, &follower, &regimes);
    if !headless {
        println!("  (perform it headless any time: --headless)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_two_clocks_keep_different_time_alone() {
        let driver = measure_solo(DRIVER_PERIOD);
        let follower = measure_solo(FOLLOWER_PERIOD);
        assert!(driver.beats > 50, "driver barely turned: {}", driver.beats);
        assert!(
            (driver.mean_period - 39.62).abs() <= 0.75,
            "driver period {} off its line",
            driver.mean_period
        );
        assert!(
            (follower.mean_period - 44.52).abs() <= 0.75,
            "follower period {} off its line",
            follower.mean_period
        );
        assert!(follower.mean_period > driver.mean_period);
    }

    #[test]
    fn uncoupled_the_follower_drifts_and_slips() {
        let r = run_regime("uncoupled", 0.0);
        assert!(!r.locked);
        assert!(r.missed_beats >= 8, "only {} slips", r.missed_beats);
        assert!(
            r.phase_dispersion >= 0.80,
            "dispersion {} — a wander must wander",
            r.phase_dispersion
        );
        assert!((r.matched_cycles as i64 - 100).abs() <= 8);
    }

    #[test]
    fn weak_coupling_pulls_but_does_not_hold() {
        let (_, _, regimes) = run_all();
        let weak = &regimes[1];
        assert!(!weak.locked, "the weak tooth must not lock");
        assert!(
            weak.follower_period > DRIVER_PERIOD && weak.follower_period < FOLLOWER_PERIOD,
            "period {} was not pulled between the solos",
            weak.follower_period
        );
        assert!((weak.follower_period - 41.43).abs() <= 1.75);
        assert!(weak.matched_cycles >= regimes[0].matched_cycles);
        assert!(weak.phase_dispersion < regimes[0].phase_dispersion);
        assert!((0.10..=0.75).contains(&weak.phase_dispersion));
    }

    #[test]
    fn strong_coupling_entrains_a_phase_pair() {
        let r = run_regime("strong thread", 0.0600);
        assert!(r.locked, "dispersion {}", r.phase_dispersion);
        assert!(r.matched_cycles >= 110);
        assert!(r.missed_beats == 0 && r.extra_beats == 0);
        assert!((r.follower_period - DRIVER_PERIOD).abs() <= 0.75);
        assert!(r.phase_dispersion <= 0.05);
    }

    #[test]
    fn entrainment_is_a_boundary_not_a_switch() {
        // The performance's arc in one assertion set: dispersion
        // falls monotonically across the three teeth, and only the
        // last tooth locks.
        let (_, _, regimes) = run_all();
        assert!(regimes[0].phase_dispersion > regimes[1].phase_dispersion);
        assert!(regimes[1].phase_dispersion > regimes[2].phase_dispersion);
        assert!(!regimes[0].locked && !regimes[1].locked && regimes[2].locked);
    }
}
