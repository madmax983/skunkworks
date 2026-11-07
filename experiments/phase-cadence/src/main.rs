//! phase-cadence — The Cartographer of Lost Things 🗺️
//!
//! Revival cross: graveyard/git-phase × experiments/firefly-cadence.
//!
//! The corpse (git-phase, executed for a Documentation Void) played
//! two step patterns derived from two revisions of one file against
//! each other Reich-style: the second pattern advances one step per
//! cycle, sweeping every relative phase offset, and nobody ever
//! measured what the sweep *did*. The living parent (firefly-cadence)
//! donates the missing organ: a Kuramoto measurement apparatus over
//! git-history cadence, with an order parameter R, a critical
//! coupling Kc, and — left as its closing seed — a P1 verdict that
//! CRACKED by tie on a five-point K grid, priced too coarsely.
//!
//! The cross: consonance gates the coupling. Sixty-four oscillators
//! carry cadence frequencies from the newest 64 commits. Oscillator
//! i hears oscillator j only where the two corpse patterns agree at
//! relative shift s: gate_ij(s) = [P_new[i] == P_old[(j+s) mod 64]].
//! Sweeping s is the Reich sweep, now with a meter on it: R(s) at
//! K = 0.08, inside the parent's tie band. Controls: uniform gate
//! (the parent's global wiring), a shuffled-pattern gate, and the
//! zero gate. A fine K grid (0.04..=0.12 by 0.01) then re-prices the
//! parent's tied Kc on this stage.

use anyhow::Result;
use std::f64::consts::TAU;

pub const STEPS: usize = 64;
pub const HISTORY: usize = 128;
pub const TICKS: u32 = 1500;
pub const DT: f64 = 0.1;
pub const OMEGA_MIN: f64 = 0.06;
pub const OMEGA_MAX: f64 = 0.16;
pub const GAP_EPS_H: f64 = 1.0 / 60.0;
/// Inside firefly-cadence's tie band (its grid jumped 0.05 -> 0.10).
pub const GATE_K: f64 = 0.08;
pub const FINE_GRID: [f64; 9] = [0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.10, 0.11, 0.12];
pub const KC_THRESHOLD: f64 = 0.8;

// ---------------------------------------------------------------------------
// The corpse's organ: step patterns from bytes (git-phase, rhythm.rs).
// Chunk sums mod 4: 0 Rest, 1 Kick, 2 Snare, 3 Hat — the corpse's own
// mapping, carried verbatim in spirit and in its unit test below.

pub const REST: u8 = 0;
pub const KICK: u8 = 1;
pub const SNARE: u8 = 2;
pub const HAT: u8 = 3;

pub fn generate_pattern(data: &[u8], length: usize) -> Vec<u8> {
    if data.is_empty() {
        return vec![REST; length];
    }
    let mut pattern = Vec::with_capacity(length);
    for i in 0..length {
        let start_idx = (i * data.len()) / length;
        let mut end_idx = ((i + 1) * data.len()) / length;
        end_idx = end_idx.max(start_idx + 1).min(data.len());
        if start_idx >= data.len() {
            pattern.push(REST);
            continue;
        }
        let sum: u32 = data[start_idx..end_idx].iter().map(|&b| b as u32).sum();
        pattern.push((sum % 4) as u8);
    }
    pattern
}

/// Consonance at relative shift s: the fraction of steps where the
/// new pattern agrees with the old pattern advanced by s. This is
/// the curve the corpse's Reich sweep traced without ever reading.
pub fn consonance(p_new: &[u8], p_old: &[u8], s: usize) -> f64 {
    let n = p_new.len();
    let hits = (0..n)
        .filter(|&i| p_new[i] == p_old[(i + s) % n])
        .count();
    hits as f64 / n as f64
}

pub fn consonance_curve(p_new: &[u8], p_old: &[u8]) -> Vec<f64> {
    (0..p_new.len()).map(|s| consonance(p_new, p_old, s)).collect()
}

// ---------------------------------------------------------------------------
// The parent's organ: cadence frequencies and phases from git history
// (firefly-cadence's recipes, re-implemented on this stage's window).

#[derive(Debug, Clone)]
pub struct CommitDatum {
    pub hash: String,
    pub timestamp: i64,
}

fn hex_byte(hash: &str, pair: usize) -> u8 {
    let b = hash.as_bytes();
    let hi = (b.get(pair * 2).copied().unwrap_or(b'0') as char)
        .to_digit(16)
        .unwrap_or(0) as u8;
    let lo = (b.get(pair * 2 + 1).copied().unwrap_or(b'0') as char)
        .to_digit(16)
        .unwrap_or(0) as u8;
    hi * 16 + lo
}

pub fn gaps_hours(history: &[CommitDatum]) -> Vec<f64> {
    let n = history.len();
    if n == 0 {
        return Vec::new();
    }
    let mut gaps: Vec<f64> = (0..n - 1)
        .map(|i| ((history[i].timestamp - history[i + 1].timestamp) as f64 / 3600.0).max(0.0))
        .collect();
    let mut sorted = gaps.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.is_empty() {
        0.0
    } else if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    };
    gaps.push(median);
    gaps
}

pub fn omega_from_gap(gap_h: f64, ln_min: f64, ln_max: f64) -> f64 {
    if ln_max <= ln_min {
        return (OMEGA_MIN + OMEGA_MAX) / 2.0;
    }
    let s = (ln_max - (gap_h + GAP_EPS_H).ln()) / (ln_max - ln_min);
    OMEGA_MIN + (OMEGA_MAX - OMEGA_MIN) * s.clamp(0.0, 1.0)
}

pub fn cadence_omegas(history: &[CommitDatum]) -> Vec<f64> {
    let gaps = gaps_hours(history);
    let lns: Vec<f64> = gaps.iter().map(|g| (g + GAP_EPS_H).ln()).collect();
    let ln_min = lns.iter().cloned().fold(f64::INFINITY, f64::min);
    let ln_max = lns.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    gaps.iter().map(|g| omega_from_gap(*g, ln_min, ln_max)).collect()
}

pub fn opening_phase(hash: &str) -> f64 {
    let v = (hex_byte(hash, 2) as f64 * 256.0 + hex_byte(hash, 3) as f64) / 65535.0;
    v * TAU
}

// ---------------------------------------------------------------------------
// The cross: consonance-gated Kuramoto.

/// gate_ij(s): oscillator i (playing the new pattern) hears
/// oscillator j (playing the old) only where they agree at shift s.
pub fn gate_matrix(p_new: &[u8], p_old: &[u8], s: usize) -> Vec<Vec<f64>> {
    let n = p_new.len();
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if p_new[i] == p_old[(j + s) % n] {
                        1.0
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect()
}

pub fn order_parameter(phases: &[f64]) -> f64 {
    let n = phases.len() as f64;
    let (mut re, mut im) = (0.0, 0.0);
    for &p in phases {
        re += p.cos();
        im += p.sin();
    }
    ((re / n).powi(2) + (im / n).powi(2)).sqrt()
}

/// Run the gated swarm; return mean R over the second half of ticks.
pub fn run_gated(omegas: &[f64], opening: &[f64], gate: &[Vec<f64>], k: f64) -> f64 {
    let n = omegas.len();
    let mut phases: Vec<f64> = opening.to_vec();
    let mut next = vec![0.0; n];
    let mut r_sum = 0.0;
    let mut r_count = 0u32;
    for tick in 0..TICKS {
        for i in 0..n {
            let mut coup = 0.0;
            for j in 0..n {
                if gate[i][j] != 0.0 {
                    coup += gate[i][j] * (phases[j] - phases[i]).sin();
                }
            }
            next[i] = phases[i] + (omegas[i] + k / n as f64 * coup) * DT;
        }
        std::mem::swap(&mut phases, &mut next);
        if tick >= TICKS / 2 {
            r_sum += order_parameter(&phases);
            r_count += 1;
        }
    }
    r_sum / r_count as f64
}

pub fn critical_k(grid: &[f64], rs: &[f64]) -> Option<f64> {
    grid.iter()
        .zip(rs)
        .find(|(_, r)| **r >= KC_THRESHOLD)
        .map(|(k, _)| *k)
}

fn argmax(v: &[f64]) -> usize {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn argmin(v: &[f64]) -> usize {
    v.iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn circ_dist(a: usize, b: usize, n: usize) -> usize {
    let d = a.abs_diff(b);
    d.min(n - d)
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

pub fn shuffled_pattern(p: &[u8], seed: u64) -> Vec<u8> {
    let mut out = p.to_vec();
    let mut rng = SplitMix(seed);
    for i in (1..out.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        out.swap(i, j);
    }
    out
}

/// Deterministic synthetic history, rung only when no repository can
/// be opened (the parent's vacuum rule): 128 commits, gaps alternating
/// burst blocks and lulls, hashes mixed from the index.
pub fn synthetic_history() -> Vec<CommitDatum> {
    let mut out = Vec::new();
    let mut t: i64 = 1_760_000_000;
    for i in (0..HISTORY).rev() {
        let gap_h = match (i / 8) % 4 {
            0 => 0.05,
            1 => 0.4,
            2 => 3.0,
            _ => 26.0,
        };
        t -= (gap_h * 3600.0) as i64;
        let mut rng = SplitMix(0x5EED_0000 + i as u64);
        let hash = format!("{:016x}{:016x}{:08x}", rng.next(), rng.next(), rng.next() as u32);
        out.push(CommitDatum { hash, timestamp: t });
    }
    out.reverse();
    out
}

pub fn load_history(limit: usize) -> Vec<CommitDatum> {
    let output = std::process::Command::new("git")
        .args(["log", &format!("-{limit}"), "--format=%H %ct"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            let mut v: Vec<CommitDatum> = text
                .lines()
                .filter_map(|line| {
                    let mut parts = line.split_whitespace();
                    let hash = parts.next()?.to_string();
                    let timestamp: i64 = parts.next()?.parse().ok()?;
                    Some(CommitDatum { hash, timestamp })
                })
                .collect();
            if v.len() >= 32 {
                v.truncate(limit);
                return v;
            }
        }
    }
    synthetic_history()
}

/// Patterns for the two halves of the window: the newer half plays
/// P_new, the older half P_old, each read off concatenated hash bytes
/// by the corpse's mapping at STEPS length.
pub fn half_patterns(history: &[CommitDatum]) -> (Vec<u8>, Vec<u8>) {
    let half = history.len() / 2;
    let bytes_of = |slice: &[CommitDatum]| -> Vec<u8> {
        slice.iter().flat_map(|c| c.hash.bytes()).collect()
    };
    let p_new = generate_pattern(&bytes_of(&history[..half]), STEPS);
    let p_old = generate_pattern(&bytes_of(&history[half..]), STEPS);
    (p_new, p_old)
}

fn hist_counts(p: &[u8]) -> [usize; 4] {
    let mut c = [0usize; 4];
    for &v in p {
        c[v as usize] += 1;
    }
    c
}

fn main() -> Result<()> {
    let headless = std::env::args().any(|a| a == "--headless");
    if !headless {
        println!("phase-cadence: run with --headless for the ledger (no TUI in this specimen).");
    }
    let history = load_history(HISTORY);
    let n_commits = history.len();
    let (p_new, p_old) = half_patterns(&history);
    let newest = &history[..STEPS.min(history.len())];
    let omegas = cadence_omegas(newest);
    let opening: Vec<f64> = newest.iter().map(|c| opening_phase(&c.hash)).collect();

    let curve = consonance_curve(&p_new, &p_old);
    let c_max_s = argmax(&curve);
    let c_min_s = argmin(&curve);
    let c_max = curve[c_max_s];
    let c_min = curve[c_min_s];
    let c_mean = curve.iter().sum::<f64>() / curve.len() as f64;

    // The Reich sweep, metered — two instruments. The raw sweep runs
    // every gate at fixed GATE_K; its ledger is a tax ledger, because
    // a gate at shift s keeps only C(s) of the links, so its swarm is
    // coupled at GATE_K * C(s) in effect. The priced sweep rescales
    // each shift's K by its own density (K_s = GATE_K / C(s)), so
    // every shift spends the same total coupling and the only thing
    // left to differ is *which* links survive. (First-build tuning
    // note: the fixed-K-only draft's P3 was unwinnable by arithmetic;
    // the rescale is the fairness the comparison needed.)
    let mut r_curve = Vec::with_capacity(STEPS);
    let mut r_scaled = Vec::with_capacity(STEPS);
    for s in 0..STEPS {
        let gate = gate_matrix(&p_new, &p_old, s);
        r_curve.push(run_gated(&omegas, &opening, &gate, GATE_K));
        let k_s = GATE_K / curve[s].max(0.02);
        r_scaled.push(run_gated(&omegas, &opening, &gate, k_s));
    }
    let r_max_s = argmax(&r_scaled);
    let r_max = r_scaled[r_max_s];
    let r_at_cmin = r_scaled[c_min_s];
    let r_mean = r_scaled.iter().sum::<f64>() / STEPS as f64;
    let raw_max = r_curve.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let raw_mean = r_curve.iter().sum::<f64>() / STEPS as f64;

    // Controls at GATE_K. The shuffled gate is rescaled by its own
    // density at the consonance-peak shift, like the priced sweep.
    let uniform_gate = vec![vec![1.0; STEPS]; STEPS];
    let r_uniform = run_gated(&omegas, &opening, &uniform_gate, GATE_K);
    let p_shuf = shuffled_pattern(&p_old, 0xCA27);
    let shuf_gate = gate_matrix(&p_new, &p_shuf, c_max_s);
    let shuf_density = consonance(&p_new, &p_shuf, c_max_s).max(0.02);
    let r_shuffled = run_gated(&omegas, &opening, &shuf_gate, GATE_K / shuf_density);
    let zero_gate = vec![vec![0.0; STEPS]; STEPS];
    let r_zero = run_gated(&omegas, &opening, &zero_gate, GATE_K);

    // The fine grid re-prices the parent's tie on this stage.
    let fine_uniform: Vec<f64> = FINE_GRID
        .iter()
        .map(|&k| run_gated(&omegas, &opening, &uniform_gate, k))
        .collect();
    let kc_fine = critical_k(&FINE_GRID, &fine_uniform);
    let best_gate = gate_matrix(&p_new, &p_old, r_max_s);
    let fine_gated: Vec<f64> = FINE_GRID
        .iter()
        .map(|&k| run_gated(&omegas, &opening, &best_gate, k))
        .collect();
    let kc_gated = critical_k(&FINE_GRID, &fine_gated);

    let fmt_rs = |rs: &[f64]| {
        rs.iter()
            .map(|r| format!("{r:.3}"))
            .collect::<Vec<_>>()
            .join("/")
    };
    let fmt_kc = |kc: Option<f64>| match kc {
        Some(k) => format!("{k:.2}"),
        None => "none".to_string(),
    };
    let hn = hist_counts(&p_new);
    let ho = hist_counts(&p_old);

    println!("phase-cadence ledger (Cartographer: git-phase x firefly-cadence)");
    println!("  history: {n_commits} commits, newest {STEPS} carry the cadence");
    println!(
        "  cadence omega: min {:.4}, max {:.4} (parent band 0.0600-0.1600)",
        omegas.iter().cloned().fold(f64::INFINITY, f64::min),
        omegas.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    );
    println!(
        "  patterns (Rest/Kick/Snare/Hat): new {}/{}/{}/{} , old {}/{}/{}/{}",
        hn[0], hn[1], hn[2], hn[3], ho[0], ho[1], ho[2], ho[3]
    );
    println!(
        "  consonance C(s): mean {c_mean:.3}, min {c_min:.3} at s={c_min_s}, max {c_max:.3} at s={c_max_s}, range {:.3}",
        c_max - c_min
    );
    println!(
        "  Reich sweep, raw (fixed K={GATE_K:.2}, the tax ledger): mean {raw_mean:.3}, max {raw_max:.3}",
    );
    println!(
        "  Reich sweep, priced (K rescaled per shift by 1/C(s)): mean {r_mean:.3}, max {r_max:.3} at s={r_max_s}, R at consonance peak (s={c_max_s}) {:.3}, R at consonance trough (s={c_min_s}) {r_at_cmin:.3}",
        r_scaled[c_max_s]
    );
    println!(
        "  controls: uniform at K={GATE_K:.2} {r_uniform:.3}, shuffled-gate (rescaled) {r_shuffled:.3}, zero gate at K={GATE_K:.2} {r_zero:.3}"
    );
    println!(
        "  fine grid K = [{}]",
        FINE_GRID.iter().map(|k| format!("{k:.2}")).collect::<Vec<_>>().join("/")
    );
    println!(
        "  uniform wiring R(K) = [{}], Kc = {}",
        fmt_rs(&fine_uniform),
        fmt_kc(kc_fine)
    );
    println!(
        "  gated (s={r_max_s}) R(K) = [{}], Kc = {}",
        fmt_rs(&fine_gated),
        fmt_kc(kc_gated)
    );
    let d_peak = circ_dist(r_max_s, c_max_s, STEPS);
    let p1 = c_max - c_min >= 0.10;
    let p2 = d_peak <= 3;
    let p3 = r_max >= r_uniform + 0.05 && r_max >= r_shuffled + 0.05;
    let p4 = r_at_cmin <= r_shuffled;
    let p5 = matches!(kc_fine, Some(k) if (0.055..0.095).contains(&k));
    let verdict = |b: bool| if b { "HELD" } else { "CRACKED" };
    println!("  predictions:");
    println!(
        "    P1 the two revisions genuinely phase (C range >= 0.10): {:.3} — {}",
        c_max - c_min,
        verdict(p1)
    );
    println!(
        "    P2 the priced R(s) peaks within 3 steps of the consonance peak: distance {d_peak} (R peak s={r_max_s}, C peak s={c_max_s}) — {}",
        verdict(p2)
    );
    println!(
        "    P3 the best priced gate beats uniform and shuffled by >= 0.05: {r_max:.3} vs {r_uniform:.3} / {r_shuffled:.3} — {}",
        verdict(p3)
    );
    println!(
        "    P4 the consonance trough (priced) locks no better than the shuffled gate: {r_at_cmin:.3} vs {r_shuffled:.3} — {}",
        verdict(p4)
    );
    println!(
        "    P5 the fine grid resolves the parent's tied Kc strictly inside (0.05, 0.10): Kc = {} — {}",
        fmt_kc(kc_fine),
        verdict(p5)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpse_mapping_matches_its_own_test_vector() {
        // git-phase's own unit test, carried across the grave:
        // data [1,2,3,4] at length 4 -> Kick, Snare, Hat, Rest.
        let p = generate_pattern(&[1, 2, 3, 4], 4);
        assert_eq!(p, vec![KICK, SNARE, HAT, REST]);
    }

    #[test]
    fn empty_data_is_all_rest() {
        assert_eq!(generate_pattern(&[], 8), vec![REST; 8]);
    }

    #[test]
    fn consonance_with_self_at_zero_shift_is_one() {
        let p = generate_pattern(b"the cartographer reads the death certificates", STEPS);
        assert!((consonance(&p, &p, 0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn consonance_curve_covers_every_shift() {
        let a = generate_pattern(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 16);
        let b = generate_pattern(b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", 16);
        let curve = consonance_curve(&a, &b);
        assert_eq!(curve.len(), 16);
        assert!(curve.iter().all(|&c| (0.0..=1.0).contains(&c)));
    }

    #[test]
    fn gate_matrix_reads_the_shifted_old_pattern() {
        let p_new = vec![KICK, REST, SNARE, HAT];
        let p_old = vec![REST, KICK, HAT, SNARE];
        let g = gate_matrix(&p_new, &p_old, 1);
        // i=0 (Kick) hears j where p_old[(j+1)%4]==Kick -> j=0 only.
        assert_eq!(g[0], vec![1.0, 0.0, 0.0, 0.0]);
        // i=1 (Rest) hears j where p_old[(j+1)%4]==Rest -> j=3 only.
        assert_eq!(g[1], vec![0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn order_parameter_extremes() {
        assert!((order_parameter(&[0.3; 8]) - 1.0).abs() < 1e-9);
        let spread: Vec<f64> = (0..8).map(|i| i as f64 * TAU / 8.0).collect();
        assert!(order_parameter(&spread) < 0.01);
    }

    #[test]
    fn zero_gate_never_locks() {
        let omegas = vec![0.06, 0.09, 0.12, 0.16];
        let opening = vec![0.0, 1.7, 3.4, 5.1];
        let zero = vec![vec![0.0; 4]; 4];
        assert!(run_gated(&omegas, &opening, &zero, 0.4) < 0.6);
    }

    #[test]
    fn uniform_gate_at_strong_coupling_locks() {
        let omegas = vec![0.09, 0.10, 0.11, 0.10];
        let opening = vec![0.0, 1.7, 3.4, 5.1];
        let uni = vec![vec![1.0; 4]; 4];
        assert!(run_gated(&omegas, &opening, &uni, 0.4) > 0.9);
    }

    #[test]
    fn cadence_endpoints_hit_the_parent_band() {
        let h = vec![
            CommitDatum { hash: "aa".repeat(20), timestamp: 7200 },
            CommitDatum { hash: "bb".repeat(20), timestamp: 3600 },
            CommitDatum { hash: "cc".repeat(20), timestamp: 0 },
        ];
        let om = cadence_omegas(&h);
        assert!(om.iter().all(|&o| (OMEGA_MIN..=OMEGA_MAX).contains(&o)));
    }

    #[test]
    fn synthetic_history_is_deterministic() {
        let a = synthetic_history();
        let b = synthetic_history();
        assert_eq!(a.len(), HISTORY);
        assert!(a.iter().zip(&b).all(|(x, y)| x.hash == y.hash && x.timestamp == y.timestamp));
    }

    #[test]
    fn critical_k_uses_the_parents_threshold() {
        let rs = [0.1, 0.5, 0.85, 0.95];
        assert_eq!(critical_k(&[0.04, 0.05, 0.06, 0.07], &rs), Some(0.06));
        assert_eq!(critical_k(&[0.04], &[0.5]), None);
    }

    #[test]
    fn shuffle_preserves_the_histogram() {
        let p = generate_pattern(b"nothing here is waste, some of it is just early", STEPS);
        let s = shuffled_pattern(&p, 7);
        assert_eq!(hist_counts(&p), hist_counts(&s));
    }
}
