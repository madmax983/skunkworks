//! # Listener Drifting 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, and `experiments/listener-adrift` —
//! listener-adrift's own closing seed, taken up as the subject: *a
//! wall whose delay drifts within a train's flight — each glint's
//! echo at a different delay, as from a moving reflector or a sloped
//! wall — priced by a search that must give the wall a velocity,
//! not an address.*
//!
//! In listener-adrift the wall stood still at one unknown address
//! (delay 3, 4, or 8) and a joint search over (delay, rho) priced
//! it exactly in a clean cave. This planting sets the wall in
//! motion. Every wall starts at delay 4 — listener-echoed's own
//! address — at rho = 0.55, and drifts by s ticks per glint,
//! s in {-1, 0, +1}:
//!
//! - s = 0: delays 4, 4, 4 — echoes at 4, 8, 12. The still wall;
//!   its naive ledger must reproduce listener-echoed's exactly.
//! - s = +1: delays 4, 5, 6 — echoes at 4, 9, 14. The first echo
//!   coincides with the second own glint; the rest walk away.
//! - s = -1: delays 4, 3, 2 — echoes at 4, 7, 10. The first echo
//!   coincides; the rest walk back toward the strike.
//!
//! The record window runs to offset 20 so every wall's full return
//! is delivered. Three judges read the SAME delivered record —
//! amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Static:** listener-adrift's search verbatim — a joint search
//!   over one fixed delay in 1..=12 ticks and rho on a 0.01 grid,
//!   scored against the whole record. It can give the wall an
//!   address but not a velocity. It is never told the wall moves.
//! - **Drift:** a joint search over (base delay in 2..=6, drift
//!   in {-1, 0, +1}, rho on a 0.01 grid). Each candidate derives
//!   the own glints it implies (subtracting, in strike order, any
//!   earlier glint's echo that lands exactly on a later own
//!   offset), predicts the whole record, and is scored by the
//!   summed absolute difference between prediction and delivery
//!   over every offset either mentions. The cheapest candidate
//!   wins; its derived own glints are judged on the same line.
//!   The judge is never told the wall's true base, drift, or rho —
//!   those are the book's ground truth, kept for scoring its
//!   estimates only.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict
//! was right is scored by the book, never a judge.
//!
//! Predictions declared before the first run:
//! - P1: the moving wall wounds the naive judge at every drift —
//!   at sigma 0.00 its errors are at least 20 at each of s = -1, 0, +1.
//! - P2: the static search is a zero-velocity judge — at sigma 0.00
//!   it recovers the book exactly at s = 0 (errors 0) and errs at
//!   least 20 times at each of s = -1 and s = +1.
//! - P3: the drift search prices all three unknowns in a clean
//!   cave — at sigma 0.00 its errors are 0 at every drift, its
//!   (base, drift) estimate is exact on every train, and its rho
//!   error max is <= 0.011.
//! - P4: pooled over the three drifts at sigma 0.10, the drift
//!   judge's errors are below the naive judge's.
//! - P5: at sigma 0.10 the drift judge still names the wall's
//!   motion — its (base, drift) estimate is exact on at least 90%
//!   of trains pooled.
//! - P6: at sigma 0.30, pooled over the three drifts, the drift
//!   judge's errors are still below the naive judge's pooled
//!   errors (the noise floor does not fully swallow the search).
//!
//! `cargo run -p listener-drifting -- --headless` prints the
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30 for each drift.
//! There is no TUI; --headless is handled before anything else
//! regardless.

use anyhow::Result;

/// The flutter line every Listener since flutter has used.
const LINE: f64 = 0.30;
/// Glints under this amplitude never reach an ear (ripple's ear).
const EAR: f64 = 0.22;
/// Own glints per train (flutter's train).
const OWN_GLINTS: usize = 3;
/// Ticks between own glints.
const GLINT_DT: u64 = 4;
/// The walls' strength: each return re-arrives at this fraction.
const RHO: f64 = 0.55;
/// Every wall starts at this delay: listener-echoed's address.
const WALL_BASE: u64 = 4;
/// The three wall drifts surveyed, in ticks per glint.
const WALL_DRIFTS: [i64; 3] = [-1, 0, 1];
/// Trains per kind per regime in the book.
const TRAINS_PER_KIND: usize = 120;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Moth,
    Stone,
}

/// One delivered arrival: what an ear in the window actually
/// receives, after coincident contributions have summed.
struct Delivered {
    offset: u64,
    amp: f64,
}

/// One recorded train: what struck it, and what the medium delivered.
struct Train {
    kind: Kind,
    delivered: Vec<Delivered>,
}

/// SplitMix64, the garden's deterministic noise hand (ripple's).
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic noise for one contribution to one train at one
/// sigma, uniform in [-sigma, +sigma]. Keyed by the train's identity,
/// the glint slot, and whether the contribution is an echo, so every
/// judge — and every rerun — hears the same record. (Verbatim from
/// listener-adrift: the echo's noise rides with its glint, wherever
/// the moving wall delivers it.)
fn glint_noise(train_id: u64, slot: usize, echo: bool, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let mut key = train_id
        .wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ ((slot as u64 + 1).wrapping_mul(0x8CB9_2AE3_5C77_19A7))
        ^ ((sigma * 1000.0) as u64).wrapping_mul(0x1656_6679_19E3_779F);
    if echo {
        key ^= 0xF0E1_D2C3_B4A5_9687;
    }
    let bits = splitmix64(key);
    let unit = (bits >> 11) as f64 / (1u64 << 53) as f64; // [0, 1)
    (unit * 2.0 - 1.0) * sigma
}

/// The clean amplitude of a reflector's glint at tick `t`.
/// Moths swing shallow with period 12; stones ripple with period 17.
fn clean_amplitude(kind: Kind, t: u64) -> f64 {
    let tau = std::f64::consts::TAU;
    match kind {
        Kind::Moth => 0.80 + 0.20 * (tau * (t % 12) as f64 / 12.0).cos(),
        Kind::Stone => 0.92 + 0.08 * (tau * (t % 17) as f64 / 17.0).cos(),
    }
}

/// The delay the wall applies to glint `j`: base + drift * j.
fn wall_delay(base: u64, drift: i64, j: usize) -> u64 {
    (base as i64 + drift * j as i64) as u64
}

/// Record one train at one noise level: its own glints at offsets
/// 0, 4, 8 from `strike`, and — if `wall` is Some((base, drift)) —
/// each glint's wall echo `wall_delay(base, drift, j)` ticks later
/// at strength RHO. Contributions that land on the same offset sum
/// in the ear before the ear's threshold is applied; the record is
/// delivered sorted by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, wall: Option<(u64, i64)>) -> Train {
    let mut acc: Vec<(u64, f64)> = Vec::new();
    let mut add = |offset: u64, amp: f64| {
        if let Some(slot) = acc.iter_mut().find(|(o, _)| *o == offset) {
            slot.1 += amp;
        } else {
            acc.push((offset, amp));
        }
    };
    for j in 0..OWN_GLINTS {
        let own = clean_amplitude(kind, strike + GLINT_DT * j as u64);
        add(GLINT_DT * j as u64, own + glint_noise(id, j, false, sigma));
        if let Some((base, drift)) = wall {
            add(
                GLINT_DT * j as u64 + wall_delay(base, drift, j),
                RHO * own + glint_noise(id, j, true, sigma),
            );
        }
    }
    acc.sort_by_key(|(o, _)| *o);
    let delivered = acc
        .into_iter()
        .filter(|(_, a)| *a >= EAR)
        .map(|(offset, amp)| Delivered { offset, amp })
        .collect();
    Train { kind, delivered }
}

/// The book: for each kind, trains struck at every phase of the
/// reflector's period, several rounds over. Identical for every
/// judge at a given wall; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, wall: Option<(u64, i64)>) -> Vec<Train> {
    let mut book = Vec::new();
    let mut id: u64 = 1;
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Moth, (k % 12) as u64, sigma, wall));
        id += 1;
    }
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Stone, (k % 17) as u64, sigma, wall));
        id += 1;
    }
    book
}

// ------------------------------------------------------------------
// The three judges. Each reads only delivered amplitudes and offsets.
// ------------------------------------------------------------------

/// The spread statistic every Listener judge uses.
fn spread(glints: &[f64]) -> f64 {
    let max = glints.iter().cloned().fold(f64::MIN, f64::max);
    let min = glints.iter().cloned().fold(f64::MAX, f64::min);
    let mean = glints.iter().sum::<f64>() / glints.len() as f64;
    (max - min) / mean
}

/// A verdict: true = "that swings — a moth", false = "steady — a stone".
fn judge_naive(delivered: &[Delivered]) -> Option<bool> {
    if delivered.len() < 3 {
        return None;
    }
    let amps: Vec<f64> = delivered[..3].iter().map(|d| d.amp).collect();
    Some(spread(&amps) > LINE)
}

/// listener-adrift's derive, verbatim: the own glints a candidate
/// fixed (delay, rho) implies, correcting for coincidence at
/// delays 4 and 8.
fn derive_own_fixed(d: [f64; 3], delay: u64, rho: f64) -> [f64; 3] {
    let o0 = d[0];
    let o1 = if delay == 4 { d[1] - rho * o0 } else { d[1] };
    let o2 = match delay {
        4 => d[2] - rho * o1,
        8 => d[2] - rho * o0,
        _ => d[2],
    };
    [o0, o1, o2]
}

/// Predict the record a set of own glints and a wall produce, and
/// score it against the delivery: summed absolute difference over
/// every offset either side mentions. Shared by both searches.
fn whole_record_score(
    delivered: &[Delivered],
    own: [f64; 3],
    delays: [u64; 3],
    rho: f64,
) -> f64 {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let mut pred: Vec<(u64, f64)> = Vec::new();
    let mut padd = |offset: u64, amp: f64| {
        if let Some(slot) = pred.iter_mut().find(|(o, _)| *o == offset) {
            slot.1 += amp;
        } else {
            pred.push((offset, amp));
        }
    };
    for j in 0..OWN_GLINTS {
        padd(GLINT_DT * j as u64, own[j]);
        padd(GLINT_DT * j as u64 + delays[j], rho * own[j]);
    }
    let mut offsets: Vec<u64> = pred.iter().map(|(o, _)| *o).collect();
    for d in delivered {
        if !offsets.contains(&d.offset) {
            offsets.push(d.offset);
        }
    }
    let mut score = 0.0;
    for o in offsets {
        let p = pred
            .iter()
            .find(|(x, _)| *x == o)
            .map(|(_, a)| *a)
            .unwrap_or(0.0);
        let d = at(o).unwrap_or(0.0);
        score += (p - d).abs();
    }
    score
}

/// The static judge: listener-adrift's search — a joint search over
/// one fixed (delay, rho), scored against the whole record. It can
/// give the wall an address but not a velocity. Returns the verdict,
/// the winning delay, and the winning rho.
fn judge_static(delivered: &[Delivered]) -> Option<(bool, u64, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let mut best: Option<(f64, u64, f64, [f64; 3])> = None; // (score, delay, rho, own)
    for delay in 1..=12u64 {
        for i in 0..=95u32 {
            let rho = i as f64 / 100.0;
            let own = derive_own_fixed(own_amps, delay, rho);
            let score = whole_record_score(delivered, own, [delay; 3], rho);
            if best.map_or(true, |(s, _, _, _)| score < s) {
                best = Some((score, delay, rho, own));
            }
        }
    }
    let (_, delay, rho, own) = best?;
    Some((spread(&own) > LINE, delay, rho))
}

/// Derive the own glints a candidate (base, drift, rho) implies:
/// walk the own offsets in strike order and subtract, from each,
/// every earlier glint's echo that lands exactly on it.
fn derive_own_drift(d: [f64; 3], base: u64, drift: i64, rho: f64) -> [f64; 3] {
    let mut own = [0.0; 3];
    for j in 0..OWN_GLINTS {
        let mut v = d[j];
        for i in 0..j {
            if GLINT_DT * i as u64 + wall_delay(base, drift, i) == GLINT_DT * j as u64 {
                v -= rho * own[i];
            }
        }
        own[j] = v;
    }
    own
}

/// The drift judge: a joint search over (base, drift, rho). Each
/// candidate derives its own glints, predicts the whole record,
/// and is scored like the static judge's candidates. Candidates
/// whose per-glint delays leave 1..=12 ticks are not walls this
/// cave can hold and are skipped. Returns the verdict, the winning
/// base, the winning drift, and the winning rho.
fn judge_drift(delivered: &[Delivered]) -> Option<(bool, u64, i64, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let mut best: Option<(f64, u64, i64, f64, [f64; 3])> = None; // (score, base, drift, rho, own)
    for base in 2..=6u64 {
        for drift in [-1i64, 0, 1] {
            let delays = [
                wall_delay(base, drift, 0),
                wall_delay(base, drift, 1),
                wall_delay(base, drift, 2),
            ];
            if delays.iter().any(|d| !(1..=12).contains(d)) {
                continue;
            }
            for i in 0..=95u32 {
                let rho = i as f64 / 100.0;
                let own = derive_own_drift(own_amps, base, drift, rho);
                let score = whole_record_score(delivered, own, delays, rho);
                if best.map_or(true, |(s, _, _, _, _)| score < s) {
                    best = Some((score, base, drift, rho, own));
                }
            }
        }
    }
    let (_, base, drift, rho, own) = best?;
    Some((spread(&own) > LINE, base, drift, rho))
}

// ------------------------------------------------------------------
// The audit.
// ------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct JudgeLedger {
    judged: usize,
    wrong_refusals: usize, // moth judged stone
    fooled: usize,         // stone judged moth
}

impl JudgeLedger {
    fn errors(&self) -> usize {
        self.wrong_refusals + self.fooled
    }
}

struct Audit {
    arrivals: usize,
    clean_naive: JudgeLedger,
    naive: JudgeLedger,
    static_ledger: JudgeLedger,
    drift_ledger: JudgeLedger,
    drift_params_exact: usize,
    drift_judged: usize,
    drift_rho_err_max: f64,
    drift_rho_err_mean: f64,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, drift: i64) -> Audit {
    let clean = build_book(sigma, None);
    let book = build_book(sigma, Some((WALL_BASE, drift)));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        static_ledger: JudgeLedger::default(),
        drift_ledger: JudgeLedger::default(),
        drift_params_exact: 0,
        drift_judged: 0,
        drift_rho_err_max: 0.0,
        drift_rho_err_mean: 0.0,
    };
    for train in &clean {
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.clean_naive, v, is_moth);
        }
    }
    let mut rho_err_sum = 0.0;
    let mut rho_n = 0usize;
    for train in &book {
        a.arrivals += train.delivered.len();
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.naive, v, is_moth);
        }
        if let Some((v, _, _)) = judge_static(&train.delivered) {
            score(&mut a.static_ledger, v, is_moth);
        }
        if let Some((v, base_est, drift_est, rho_est)) = judge_drift(&train.delivered) {
            score(&mut a.drift_ledger, v, is_moth);
            a.drift_judged += 1;
            if base_est == WALL_BASE && drift_est == drift {
                a.drift_params_exact += 1;
            }
            let e = (rho_est - RHO).abs();
            rho_err_sum += e;
            rho_n += 1;
            if e > a.drift_rho_err_max {
                a.drift_rho_err_max = e;
            }
        }
    }
    if rho_n > 0 {
        a.drift_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    println!("listener-drifting headless: one book of recorded trains, one wall in motion, three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, wall base delay {} ticks, drifts {:?} ticks per glint, rho {:.2}, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        WALL_BASE,
        WALL_DRIFTS,
        RHO
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), static (adrift's joint delay 1..=12 x rho grid, whole-record score), drift (joint base 2..=6 x drift -1/0/+1 x rho grid, whole-record score)", LINE);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, i64, Audit)> = Vec::new();
    for sigma in sigmas {
        for drift in WALL_DRIFTS {
            let a = audit(sigma, drift);
            println!(
                "  sigma {:.2} drift {:+}: arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), static errors {} (wrong {}, fooled {}), drift errors {} (wrong {}, fooled {}), drift (base, drift) exact {}/{}, drift rho err mean {:.3} max {:.3}",
                sigma,
                drift,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.static_ledger.errors(),
                a.static_ledger.wrong_refusals,
                a.static_ledger.fooled,
                a.drift_ledger.errors(),
                a.drift_ledger.wrong_refusals,
                a.drift_ledger.fooled,
                a.drift_params_exact,
                a.drift_judged,
                a.drift_rho_err_mean,
                a.drift_rho_err_max
            );
            all.push((sigma, drift, a));
        }
    }
    let at = |s: f64, d: i64| &all.iter().find(|(x, y, _)| *x == s && *y == d).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        WALL_DRIFTS.iter().map(|d| pick(at(s, *d))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the moving wall wounds the naive judge at every drift (sigma 0.00 naive errors >= 20 at each drift): drift -1 {}, drift 0 {}, drift +1 {} — {}",
        at(0.00, -1).naive.errors(),
        at(0.00, 0).naive.errors(),
        at(0.00, 1).naive.errors(),
        if WALL_DRIFTS.iter().all(|d| at(0.00, *d).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the static search is a zero-velocity judge (sigma 0.00: errors 0 at drift 0, errors >= 20 at drifts -1 and +1): drift 0 errors {}, drift -1 errors {}, drift +1 errors {} — {}",
        at(0.00, 0).static_ledger.errors(),
        at(0.00, -1).static_ledger.errors(),
        at(0.00, 1).static_ledger.errors(),
        if at(0.00, 0).static_ledger.errors() == 0
            && at(0.00, -1).static_ledger.errors() >= 20
            && at(0.00, 1).static_ledger.errors() >= 20
        { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the drift search prices all three unknowns in a clean cave (sigma 0.00: errors 0 at every drift, (base, drift) exact on every train, rho err max <= 0.011): errors {}/{}/{}, params exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, -1).drift_ledger.errors(),
        at(0.00, 0).drift_ledger.errors(),
        at(0.00, 1).drift_ledger.errors(),
        at(0.00, -1).drift_params_exact,
        at(0.00, 0).drift_params_exact,
        at(0.00, 1).drift_params_exact,
        at(0.00, -1).drift_rho_err_max,
        at(0.00, 0).drift_rho_err_max,
        at(0.00, 1).drift_rho_err_max,
        if WALL_DRIFTS.iter().all(|d| {
            let a = at(0.00, *d);
            a.drift_ledger.errors() == 0
                && a.drift_params_exact == a.drift_judged
                && a.drift_judged == TRAINS_PER_KIND * 2
                && a.drift_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 pooled at sigma 0.10 the drift judge errs less than the naive judge: drift {} vs naive {} — {}",
        pooled(0.10, |a| a.drift_ledger.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.drift_ledger.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let exact10: usize = WALL_DRIFTS.iter().map(|d| at(0.10, *d).drift_params_exact).sum();
    let judged10: usize = WALL_DRIFTS.iter().map(|d| at(0.10, *d).drift_judged).sum();
    println!(
        "    P5 at sigma 0.10 the drift judge still names the wall's motion ((base, drift) exact on >= 90% of trains pooled): {}/{} — {}",
        exact10,
        judged10,
        if judged10 > 0 && (exact10 as f64) >= 0.90 * (judged10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 pooled the drift judge still errs less than the naive judge: drift {} vs naive {} — {}",
        pooled(0.30, |a| a.drift_ledger.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.drift_ledger.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
}

fn main() -> Result<()> {
    // --headless is handled before anything else; there is no TUI.
    let _args: Vec<String> = std::env::args().collect();
    run_headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_moth_trains_clear_the_line_on_three_own_glints() {
        for strike in 0..12u64 {
            let t = record_train(1, Kind::Moth, strike, 0.0, None);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) > LINE, "moth strike {strike}");
        }
    }

    #[test]
    fn clean_stone_trains_stay_under_the_line_on_three_own_glints() {
        for strike in 0..17u64 {
            let t = record_train(2, Kind::Stone, strike, 0.0, None);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) <= LINE, "stone strike {strike}");
        }
    }

    #[test]
    fn clean_unwalled_book_has_zero_errors_for_the_naive_judge() {
        let book = build_book(0.0, None);
        for t in &book {
            let v = judge_naive(&t.delivered).expect("clean train judged");
            assert_eq!(v, t.kind == Kind::Moth);
        }
    }

    #[test]
    fn the_still_wall_reproduces_listener_echoeds_naive_ledger() {
        // Drift 0 at base 4 IS listener-echoed's wall: every stone
        // fooled by the summed step, twenty moths refused.
        let a = audit(0.0, 0);
        assert_eq!(a.naive.errors(), 140);
        assert_eq!(a.naive.fooled, 120);
    }

    #[test]
    fn the_forward_drift_wall_coincides_once_then_walks_away() {
        let t = record_train(7, Kind::Stone, 3, 0.0, Some((WALL_BASE, 1)));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 8, 9, 14]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        // Offset 4 is own_1 summed with the echo of own_0.
        assert!((t.delivered[1].amp - (own[1] + RHO * own[0])).abs() < 1e-12);
        // Offset 9 is the echo of own_1, alone, one tick late.
        assert!((t.delivered[3].amp - RHO * own[1]).abs() < 1e-12);
        // Offset 14 is the echo of own_2, alone, two ticks late.
        assert!((t.delivered[4].amp - RHO * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_backward_drift_wall_coincides_once_then_walks_back() {
        let t = record_train(7, Kind::Stone, 3, 0.0, Some((WALL_BASE, -1)));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 7, 8, 10]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        // Offset 7 is the echo of own_1, alone, one tick early.
        assert!((t.delivered[2].amp - RHO * own[1]).abs() < 1e-12);
        // Offset 10 is the echo of own_2, alone, two ticks early.
        assert!((t.delivered[4].amp - RHO * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_static_search_recovers_only_the_still_wall_at_sigma_zero() {
        assert_eq!(audit(0.0, 0).static_ledger.errors(), 0);
        assert!(audit(0.0, -1).static_ledger.errors() >= 20);
        assert!(audit(0.0, 1).static_ledger.errors() >= 20);
    }

    #[test]
    fn the_drift_search_recovers_every_drift_at_sigma_zero() {
        for drift in WALL_DRIFTS {
            let a = audit(0.0, drift);
            assert_eq!(a.drift_ledger.errors(), 0, "drift {drift}");
            assert_eq!(a.drift_params_exact, TRAINS_PER_KIND * 2, "drift {drift}");
            assert!(a.drift_rho_err_max <= 0.011, "drift {drift} rho err {}", a.drift_rho_err_max);
        }
    }

    #[test]
    fn the_drift_search_names_base_drift_and_rho_on_individual_trains_at_sigma_zero() {
        for drift in WALL_DRIFTS {
            let book = build_book(0.0, Some((WALL_BASE, drift)));
            for t in &book {
                let (v, b_est, s_est, r_est) =
                    judge_drift(&t.delivered).expect("drift judged");
                assert_eq!(v, t.kind == Kind::Moth, "drift {drift}");
                assert_eq!(b_est, WALL_BASE);
                assert_eq!(s_est, drift);
                assert!((r_est - RHO).abs() <= 0.011);
            }
        }
    }

    #[test]
    fn the_naive_judge_is_wounded_at_every_drift_at_sigma_zero() {
        for drift in WALL_DRIFTS {
            assert!(audit(0.0, drift).naive.errors() >= 20, "drift {drift}");
        }
    }

    #[test]
    fn derive_own_drift_unwinds_a_coincident_first_echo() {
        // A hand-built record: own [1.0, 0.6, 1.0] behind the
        // forward-drift wall. Offset 4 must read 0.6 + RHO * 1.0.
        let own = [1.0, 0.6, 1.0];
        let d = [own[0], own[1] + RHO * own[0], own[2]];
        let back = derive_own_drift(d, WALL_BASE, 1, RHO);
        for j in 0..3 {
            assert!((back[j] - own[j]).abs() < 1e-12, "glint {j}");
        }
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, Some((WALL_BASE, -1)));
        let b2 = build_book(0.20, Some((WALL_BASE, -1)));
        assert_eq!(b1.len(), b2.len());
        for (t1, t2) in b1.iter().zip(b2.iter()) {
            assert_eq!(t1.kind, t2.kind);
            assert_eq!(t1.delivered.len(), t2.delivered.len());
            for (d1, d2) in t1.delivered.iter().zip(t2.delivered.iter()) {
                assert_eq!(d1.offset, d2.offset);
                assert_eq!(d1.amp, d2.amp);
            }
        }
    }

    #[test]
    fn noise_is_zero_at_sigma_zero_and_bounded_elsewhere() {
        for id in 1..50u64 {
            assert_eq!(glint_noise(id, 0, false, 0.0), 0.0);
            for slot in 0..3 {
                for echo in [false, true] {
                    let n = glint_noise(id, slot, echo, 0.30);
                    assert!(n.abs() <= 0.30, "noise {n} out of bounds");
                }
            }
        }
    }
}
