//! # Listener Adrift 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`, and
//! `experiments/listener-echoed` — listener-echoed's own closing seed,
//! taken up as the subject: *a wall whose delay is not one glint
//! interval — rho and delay both unknown, priced from a record whose
//! orphan may land inside the window on a later glint or beyond it
//! entirely.*
//!
//! In listener-echoed the wall stood at exactly one glint interval
//! (delay 4): every echo landed on the next own glint, and the last
//! glint's echo arrived alone at offset 12 as an orphan that priced
//! rho exactly. That judge knew the wall's address and only had to
//! read the price tag. This planting moves the wall. Three walls are
//! surveyed, at delays 3, 4, and 8 ticks, all at rho = 0.55:
//!
//! - delay 3: echoes at 3, 7, 11 — interleaved just *before* each
//!   following own glint; no arrival at offset 12 at all.
//! - delay 4: echoes at 4, 8, 12 — coincidence on the next own glint,
//!   the echoed case, orphan at 12.
//! - delay 8: echoes at 8, 12, 16 — the first echo coincides with
//!   the *last* own glint, and two orphans arrive after the window.
//!
//! The record window is extended to offset 20 so every wall's full
//! return is delivered. Three judges read the SAME delivered record
//! — amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Legacy tail:** listener-echoed's judge verbatim — it assumes
//!   delay 4, searches rho from an assumed orphan at offset 12, and
//!   unwinds. It is never told the wall moved.
//! - **Search:** a joint search over delay in 1..=12 ticks and rho
//!   in [0.00, 0.95] at 0.01 steps. For each candidate it derives the
//!   own glints the candidate implies (correcting the own offsets
//!   for coincidence when the candidate delay is 4 or 8), predicts
//!   the whole delivered record that candidate would produce, and
//!   scores the candidate by the summed absolute difference between
//!   prediction and delivery over every offset either mentions. The
//!   cheapest candidate wins; its derived own glints are judged on
//!   the same 0.30 line.
//!
//! The perception rule is sacred: a judge reads only amplitudes and
//! arrival offsets the medium delivered. Whether a verdict was right
//! is scored by the book, never a judge. The wall's true delay and
//! rho are ground truth for scoring the search judge's estimates
//! only; no judge is told them.
//!
//! Predictions declared before the first run:
//! - P1: the wall wounds the naive judge at every address — at
//!   sigma 0.00 its errors are at least 20 at each of delays 3, 4, 8.
//! - P2: the legacy tail is a one-address judge — at sigma 0.00 it
//!   recovers the book exactly at delay 4 (errors 0), judges no
//!   train at all at delay 3 (no arrival at its assumed orphan
//!   offset 12), and errs at least 20 times at delay 8.
//! - P3: the search prices both unknowns in a clean cave — at sigma
//!   0.00 its errors are 0 at every delay, its delay estimate is
//!   exact on every train, and its rho error max is <= 0.011.
//! - P4: pooled over the three walls at sigma 0.10, the search
//!   judge's errors are below the naive judge's.
//! - P5: at sigma 0.10 the search still names the wall's address —
//!   its delay estimate is exact on at least 90% of trains pooled.
//! - P6: at sigma 0.30, pooled over the three walls, the search
//!   judge's errors are still below the naive judge's pooled errors
//!   (the noise floor does not fully swallow the joint search).
//!
//! `cargo run -p listener-adrift -- --headless` prints the ledgers
//! at sigma = 0.00, 0.10, 0.20, 0.30 for each wall. There is no TUI;
//! --headless is handled before anything else regardless.

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
/// The three wall delays surveyed, in ticks.
const WALL_DELAYS: [u64; 3] = [3, 4, 8];
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
/// judge — and every rerun — hears the same record.
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

/// Record one train at one noise level: its own glints at offsets
/// 0, 4, 8 from `strike`, and — if `wall` is Some(delay) — each
/// glint's wall echo `delay` ticks later at strength RHO.
/// Contributions that land on the same offset sum in the ear before
/// the ear's threshold is applied; the record is delivered sorted
/// by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, wall: Option<u64>) -> Train {
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
        if let Some(delay) = wall {
            add(
                GLINT_DT * j as u64 + delay,
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
fn build_book(sigma: f64, wall: Option<u64>) -> Vec<Train> {
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

/// The legacy tail judge, listener-echoed's judge verbatim: it
/// assumes the wall stands at delay 4, searches rho on a 0.01 grid
/// so the unwound train best predicts an assumed orphan at offset
/// 12, unwinds, and judges. Returns the verdict and its rho estimate.
fn judge_legacy(delivered: &[Delivered]) -> Option<(bool, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let (d0, d1, d2, d3) = (at(0)?, at(4)?, at(8)?, at(12)?);
    let mut best: Option<(f64, f64)> = None; // (misprediction, rho)
    for i in 0..=95u32 {
        let r = i as f64 / 100.0;
        let o0 = d0;
        let o1 = d1 - r * o0;
        let o2 = d2 - r * o1;
        let err = (r * o2 - d3).abs();
        if best.map_or(true, |(e, _)| err < e) {
            best = Some((err, r));
        }
    }
    let r = best?.1;
    let own = [d0, d1 - r * d0, d2 - r * (d1 - r * d0)];
    Some((spread(&own) > LINE, r))
}

/// Derive the own glints a candidate (delay, rho) implies from the
/// delivered own-offset amplitudes, correcting for coincidence:
/// at delay 4 each own glint carries the previous glint's echo;
/// at delay 8 the last own glint carries the first glint's echo;
/// at every other delay the own offsets are clean.
fn derive_own(d: [f64; 3], delay: u64, rho: f64) -> [f64; 3] {
    let o0 = d[0];
    let o1 = if delay == 4 { d[1] - rho * o0 } else { d[1] };
    let o2 = match delay {
        4 => d[2] - rho * o1,
        8 => d[2] - rho * o0,
        _ => d[2],
    };
    [o0, o1, o2]
}

/// The search judge: a joint search over (delay, rho). Each
/// candidate derives its own glints, predicts the whole record
/// (own arrivals plus one echo per glint at the candidate delay),
/// and is scored by the summed absolute difference between its
/// prediction and the delivery over every offset either mentions.
/// Returns the verdict, the winning delay, and the winning rho.
fn judge_search(delivered: &[Delivered]) -> Option<(bool, u64, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let mut best: Option<(f64, u64, f64, [f64; 3])> = None; // (score, delay, rho, own)
    for delay in 1..=12u64 {
        for i in 0..=95u32 {
            let rho = i as f64 / 100.0;
            let own = derive_own(own_amps, delay, rho);
            // Predict the record this candidate would deliver.
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
                padd(GLINT_DT * j as u64 + delay, rho * own[j]);
            }
            // Score over the union of predicted and delivered offsets.
            let mut offsets: Vec<u64> = pred.iter().map(|(o, _)| *o).collect();
            for d in delivered {
                if !offsets.contains(&d.offset) {
                    offsets.push(d.offset);
                }
            }
            let mut score = 0.0;
            for o in offsets {
                let p = pred.iter().find(|(x, _)| *x == o).map(|(_, a)| *a).unwrap_or(0.0);
                let d = at(o).unwrap_or(0.0);
                score += (p - d).abs();
            }
            if best.map_or(true, |(s, _, _, _)| score < s) {
                best = Some((score, delay, rho, own));
            }
        }
    }
    let (_, delay, rho, own) = best?;
    Some((spread(&own) > LINE, delay, rho))
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
    legacy: JudgeLedger,
    search: JudgeLedger,
    search_delay_exact: usize,
    search_delay_judged: usize,
    search_rho_err_max: f64,
    search_rho_err_mean: f64,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, wall_delay: u64) -> Audit {
    let clean = build_book(sigma, None);
    let book = build_book(sigma, Some(wall_delay));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        legacy: JudgeLedger::default(),
        search: JudgeLedger::default(),
        search_delay_exact: 0,
        search_delay_judged: 0,
        search_rho_err_max: 0.0,
        search_rho_err_mean: 0.0,
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
        if let Some((v, _)) = judge_legacy(&train.delivered) {
            score(&mut a.legacy, v, is_moth);
        }
        if let Some((v, delay_est, rho_est)) = judge_search(&train.delivered) {
            score(&mut a.search, v, is_moth);
            a.search_delay_judged += 1;
            if delay_est == wall_delay {
                a.search_delay_exact += 1;
            }
            let e = (rho_est - RHO).abs();
            rho_err_sum += e;
            rho_n += 1;
            if e > a.search_rho_err_max {
                a.search_rho_err_max = e;
            }
        }
    }
    if rho_n > 0 {
        a.search_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    println!("listener-adrift headless: one book of recorded trains, three walls at unknown delays, three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, walls at delays {:?} ticks, rho {:.2}, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        WALL_DELAYS,
        RHO
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), legacy tail (assumes delay 4, orphan at 12), search (joint delay 1..=12 x rho grid, whole-record score)", LINE);
    // audits[sigma][delay]
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, u64, Audit)> = Vec::new();
    for sigma in sigmas {
        for delay in WALL_DELAYS {
            let a = audit(sigma, delay);
            println!(
                "  sigma {:.2} delay {}: arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), legacy errors {} (judged {}, wrong {}, fooled {}), search errors {} (wrong {}, fooled {}), search delay exact {}/{}, search rho err mean {:.3} max {:.3}",
                sigma,
                delay,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.legacy.errors(),
                a.legacy.judged,
                a.legacy.wrong_refusals,
                a.legacy.fooled,
                a.search.errors(),
                a.search.wrong_refusals,
                a.search.fooled,
                a.search_delay_exact,
                a.search_delay_judged,
                a.search_rho_err_mean,
                a.search_rho_err_max
            );
            all.push((sigma, delay, a));
        }
    }
    let at = |s: f64, d: u64| &all.iter().find(|(x, y, _)| *x == s && *y == d).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        WALL_DELAYS.iter().map(|d| pick(at(s, *d))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the wall wounds the naive judge at every address (sigma 0.00 naive errors >= 20 at each delay): delay 3 {}, delay 4 {}, delay 8 {} — {}",
        at(0.00, 3).naive.errors(),
        at(0.00, 4).naive.errors(),
        at(0.00, 8).naive.errors(),
        if WALL_DELAYS.iter().all(|d| at(0.00, *d).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the legacy tail is a one-address judge (sigma 0.00: errors 0 at delay 4, judged 0 at delay 3, errors >= 20 at delay 8): delay 4 errors {}, delay 3 judged {}, delay 8 errors {} — {}",
        at(0.00, 4).legacy.errors(),
        at(0.00, 3).legacy.judged,
        at(0.00, 8).legacy.errors(),
        if at(0.00, 4).legacy.errors() == 0 && at(0.00, 3).legacy.judged == 0 && at(0.00, 8).legacy.errors() >= 20 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the search prices both unknowns in a clean cave (sigma 0.00: errors 0 at every delay, delay exact on every train, rho err max <= 0.011): errors {}/{}/{}, delay exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, 3).search.errors(),
        at(0.00, 4).search.errors(),
        at(0.00, 8).search.errors(),
        at(0.00, 3).search_delay_exact,
        at(0.00, 4).search_delay_exact,
        at(0.00, 8).search_delay_exact,
        at(0.00, 3).search_rho_err_max,
        at(0.00, 4).search_rho_err_max,
        at(0.00, 8).search_rho_err_max,
        if WALL_DELAYS.iter().all(|d| {
            let a = at(0.00, *d);
            a.search.errors() == 0
                && a.search_delay_exact == a.search_delay_judged
                && a.search_delay_judged == TRAINS_PER_KIND * 2
                && a.search_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 pooled at sigma 0.10 the search errs less than the naive judge: search {} vs naive {} — {}",
        pooled(0.10, |a| a.search.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.search.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let exact10: usize = WALL_DELAYS.iter().map(|d| at(0.10, *d).search_delay_exact).sum();
    let judged10: usize = WALL_DELAYS.iter().map(|d| at(0.10, *d).search_delay_judged).sum();
    println!(
        "    P5 at sigma 0.10 the search still names the wall's address (delay exact on >= 90% of trains pooled): {}/{} — {}",
        exact10,
        judged10,
        if judged10 > 0 && (exact10 as f64) >= 0.90 * (judged10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 pooled the search still errs less than the naive judge: search {} vs naive {} — {}",
        pooled(0.30, |a| a.search.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.search.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
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
    fn the_delay3_wall_interleaves_its_echoes_before_the_next_own_glints() {
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(3));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 3, 4, 7, 8, 11]);
        // No arrival at the legacy judge's assumed orphan offset.
        assert!(t.delivered.iter().all(|d| d.offset != 12));
    }

    #[test]
    fn the_delay8_wall_coincides_once_and_orphans_twice() {
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(8));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 8, 12, 16]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        // Offset 8 is own_2 summed with the echo of own_0.
        assert!((t.delivered[2].amp - (own[2] + RHO * own[0])).abs() < 1e-12);
        // Offset 12 is the orphan echo of own_1, alone.
        assert!((t.delivered[3].amp - RHO * own[1]).abs() < 1e-12);
    }

    #[test]
    fn the_delay4_wall_reproduces_listener_echoeds_naive_ledger() {
        let a = audit(0.0, 4);
        assert_eq!(a.naive.errors(), 140);
        assert_eq!(a.naive.fooled, 120);
    }

    #[test]
    fn the_legacy_tail_judge_recovers_only_its_own_address_at_sigma_zero() {
        assert_eq!(audit(0.0, 4).legacy.errors(), 0);
        assert_eq!(audit(0.0, 3).legacy.judged, 0);
        assert!(audit(0.0, 8).legacy.errors() >= 20);
    }

    #[test]
    fn the_search_judge_recovers_every_wall_at_sigma_zero() {
        for delay in WALL_DELAYS {
            let a = audit(0.0, delay);
            assert_eq!(a.search.errors(), 0, "delay {delay}");
            assert_eq!(a.search_delay_exact, TRAINS_PER_KIND * 2, "delay {delay}");
            assert!(a.search_rho_err_max <= 0.011, "delay {delay} rho err {}", a.search_rho_err_max);
        }
    }

    #[test]
    fn the_search_judge_names_the_delay_on_individual_trains_at_sigma_zero() {
        for delay in WALL_DELAYS {
            let book = build_book(0.0, Some(delay));
            for t in &book {
                let (v, d_est, r_est) = judge_search(&t.delivered).expect("search judged");
                assert_eq!(v, t.kind == Kind::Moth, "delay {delay}");
                assert_eq!(d_est, delay);
                assert!((r_est - RHO).abs() <= 0.011);
            }
        }
    }

    #[test]
    fn the_naive_judge_is_wounded_at_every_wall_at_sigma_zero() {
        for delay in WALL_DELAYS {
            assert!(audit(0.0, delay).naive.errors() >= 20, "delay {delay}");
        }
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, Some(3));
        let b2 = build_book(0.20, Some(3));
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
