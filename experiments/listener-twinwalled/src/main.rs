//! # Listener Twinwalled 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, `experiments/listener-adrift`, and
//! `experiments/listener-drifting` — drifting's own closing seed, taken
//! up as the subject: *two walls at once, either of them possibly in
//! motion — does the joint search's whole-record score still separate
//! a pair of trajectories, or do two walls price each other?* This
//! planting takes the still half of that seed (the motion half stays
//! named for a later strike): two still walls, at two unknown
//! addresses, at two unknown strengths, in the same cave at once.
//!
//! In listener-adrift one wall stood at one unknown address, and in
//! listener-drifting one wall walked. Here every train is recorded
//! behind a PAIR of walls: the first at strength RHO_A = 0.55, the
//! second at RHO_B = 0.40. Three pairs are surveyed:
//!
//! - (3, 8): the near wall interleaves (echoes at 3, 7, 11); the far
//!   wall's first echo lands exactly on the third own glint.
//! - (4, 9): the near wall coincides (echoes at 4, 8, 12); the far
//!   wall interleaves late (echoes at 9, 13, 17).
//! - (4, 8): both walls coincide — offset 8 carries the third own
//!   glint, the near wall's echo of the second glint, and the far
//!   wall's echo of the first, summed in one arrival.
//!
//! The record window runs to offset 20 so both walls' full returns
//! are delivered. Three judges read the SAME delivered record —
//! amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Single:** listener-adrift's search verbatim — a joint search
//!   over ONE fixed delay in 1..=12 ticks and rho on a 0.01 grid,
//!   scored against the whole record. It can name one wall. It is
//!   never told there are two.
//! - **Twin:** a joint search over an ordered pair of distinct
//!   delays in 1..=12 and two strengths on a 0.05 grid (the true
//!   strengths sit on that grid). Each candidate derives the own
//!   glints it implies (subtracting, in strike order, any earlier
//!   glint's echo from either wall that lands exactly on a later
//!   own offset), predicts the whole record, and is scored by the
//!   summed absolute difference between prediction and delivery
//!   over every offset either mentions. The cheapest candidate
//!   wins; its derived own glints are judged on the same line.
//!   The judge is never told the pair's true addresses or
//!   strengths — those are the book's ground truth, kept for
//!   scoring its estimates only.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict
//! was right is scored by the book, never a judge.
//!
//! Predictions declared before the first run:
//! - P1: two walls wound the naive judge at every pair — at
//!   sigma 0.00 its errors are at least 20 at each surveyed pair.
//! - P2: the single search is a one-wall judge — at sigma 0.00 it
//!   errs at least 20 times at each surveyed pair.
//! - P3: the twin search prices both walls in a clean cave — at
//!   sigma 0.00 its errors are 0 at every pair, its (delay pair)
//!   estimate is exact on every train, and its rho error max
//!   (either wall) is <= 0.011.
//! - P4: pooled over the three pairs at sigma 0.10, the twin
//!   judge's errors are below the naive judge's.
//! - P5: at sigma 0.10 the twin judge still names both walls —
//!   its delay-pair estimate is exact on at least 90% of trains
//!   pooled.
//! - P6: at sigma 0.30, pooled over the three pairs, the twin
//!   judge's errors are still below the naive judge's pooled
//!   errors (the noise floor does not fully swallow the search).
//!
//! `cargo run -p listener-twinwalled -- --headless` prints the
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30 for each pair.
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
/// The two walls' strengths: each return re-arrives at these fractions.
const RHO_A: f64 = 0.55;
const RHO_B: f64 = 0.40;
/// The three wall pairs surveyed, as (near delay, far delay).
const WALL_PAIRS: [(u64, u64); 3] = [(3, 8), (4, 9), (4, 8)];
/// Trains per kind per regime in the book.
const TRAINS_PER_KIND: usize = 120;
/// Widest offset any prediction or delivery can mention
/// (last own glint at 8, latest echo delay 12, plus margin).
const MAX_OFFSET: usize = 32;

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
/// sigma, uniform in [-sigma, +sigma]. Keyed by the train's
/// identity, the glint slot, and which channel carried the
/// contribution (0 = own glint, 1 = near wall's echo, 2 = far
/// wall's echo), so every judge — and every rerun — hears the
/// same record. The own channel's keying matches listener-adrift's
/// non-echo keying, so the unwalled control book is the lineage's
/// own book.
fn glint_noise(train_id: u64, slot: usize, channel: u64, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let mut key = train_id
        .wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ ((slot as u64 + 1).wrapping_mul(0x8CB9_2AE3_5C77_19A7))
        ^ ((sigma * 1000.0) as u64).wrapping_mul(0x1656_6679_19E3_779F);
    if channel == 1 {
        key ^= 0xF0E1_D2C3_B4A5_9687;
    } else if channel == 2 {
        key ^= 0xA5A5_5A5A_C3C3_3C3C;
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
/// 0, 4, 8 from `strike`, and — if `walls` is Some — each glint's
/// echo from each wall, `delay` ticks later at that wall's
/// strength. Contributions that land on the same offset sum in
/// the ear before the ear's threshold is applied; the record is
/// delivered sorted by arrival offset.
fn record_train(
    id: u64,
    kind: Kind,
    strike: u64,
    sigma: f64,
    walls: Option<[(u64, f64); 2]>,
) -> Train {
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
        add(GLINT_DT * j as u64, own + glint_noise(id, j, 0, sigma));
        if let Some(ws) = walls {
            for (wi, (delay, rho)) in ws.iter().enumerate() {
                add(
                    GLINT_DT * j as u64 + delay,
                    rho * own + glint_noise(id, j, (wi + 1) as u64, sigma),
                );
            }
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
/// judge at a given pair; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, walls: Option<[(u64, f64); 2]>) -> Vec<Train> {
    let mut book = Vec::new();
    let mut id: u64 = 1;
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Moth, (k % 12) as u64, sigma, walls));
        id += 1;
    }
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Stone, (k % 17) as u64, sigma, walls));
        id += 1;
    }
    book
}

/// The surveyed pair as recording walls (delays with their strengths).
fn pair_walls(pair: (u64, u64)) -> [(u64, f64); 2] {
    [(pair.0, RHO_A), (pair.1, RHO_B)]
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

/// The delivered record as a fixed array, for fast scoring.
fn delivered_array(delivered: &[Delivered]) -> [f64; MAX_OFFSET] {
    let mut a = [0.0; MAX_OFFSET];
    for d in delivered {
        if (d.offset as usize) < MAX_OFFSET {
            a[d.offset as usize] = d.amp;
        }
    }
    a
}

/// Derive the own glints a candidate set of walls implies: walk
/// the own offsets in strike order and subtract, from each, every
/// earlier glint's echo — from any candidate wall — that lands
/// exactly on it.
fn derive_own(d: [f64; 3], walls: &[(u64, f64)]) -> [f64; 3] {
    let mut own = [0.0; 3];
    for j in 0..OWN_GLINTS {
        let mut v = d[j];
        for i in 0..j {
            for (delay, rho) in walls {
                if GLINT_DT * i as u64 + delay == GLINT_DT * j as u64 {
                    v -= rho * own[i];
                }
            }
        }
        own[j] = v;
    }
    own
}

/// Predict the record a set of own glints and candidate walls
/// produce, and score it against the delivery: summed absolute
/// difference over every offset either side mentions.
fn whole_record_score(del: &[f64; MAX_OFFSET], own: [f64; 3], walls: &[(u64, f64)]) -> f64 {
    let mut pred = [0.0; MAX_OFFSET];
    for j in 0..OWN_GLINTS {
        pred[(GLINT_DT * j as u64) as usize] += own[j];
        for (delay, rho) in walls {
            pred[(GLINT_DT * j as u64 + delay) as usize] += rho * own[j];
        }
    }
    let mut score = 0.0;
    for o in 0..MAX_OFFSET {
        score += (pred[o] - del[o]).abs();
    }
    score
}

/// The single judge: listener-adrift's search — a joint search
/// over ONE fixed (delay, rho), scored against the whole record.
/// It can give a wall an address but cannot count to two. Returns
/// the verdict, the winning delay, and the winning rho.
fn judge_single(delivered: &[Delivered]) -> Option<(bool, u64, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let mut best: Option<(f64, u64, f64, [f64; 3])> = None;
    for delay in 1..=12u64 {
        for i in 0..=95u32 {
            let rho = i as f64 / 100.0;
            let walls = [(delay, rho)];
            let own = derive_own(own_amps, &walls);
            let score = whole_record_score(&del, own, &walls);
            if best.map_or(true, |(s, _, _, _)| score < s) {
                best = Some((score, delay, rho, own));
            }
        }
    }
    let (_, delay, rho, own) = best?;
    Some((spread(&own) > LINE, delay, rho))
}

/// The twin judge: a joint search over an ordered pair of
/// distinct delays and two strengths on a 0.05 grid. Each
/// candidate derives its own glints, predicts the whole record,
/// and is scored like the single judge's candidates. Returns the
/// verdict, the winning delay pair, and the winning strengths.
fn judge_twin(delivered: &[Delivered]) -> Option<(bool, (u64, u64), (f64, f64))> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let mut best: Option<(f64, (u64, u64), (f64, f64), [f64; 3])> = None;
    for d1 in 1..=12u64 {
        for d2 in (d1 + 1)..=12u64 {
            for i in 0..=19u32 {
                let r1 = i as f64 * 0.05;
                for k in 0..=19u32 {
                    let r2 = k as f64 * 0.05;
                    let walls = [(d1, r1), (d2, r2)];
                    let own = derive_own(own_amps, &walls);
                    let score = whole_record_score(&del, own, &walls);
                    if best.map_or(true, |(s, _, _, _)| score < s) {
                        best = Some((score, (d1, d2), (r1, r2), own));
                    }
                }
            }
        }
    }
    let (_, pair, rhos, own) = best?;
    Some((spread(&own) > LINE, pair, rhos))
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
    single: JudgeLedger,
    twin: JudgeLedger,
    twin_params_exact: usize,
    twin_judged: usize,
    twin_rho_err_max: f64,
    twin_rho_err_mean: f64,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, pair: (u64, u64)) -> Audit {
    let clean = build_book(sigma, None);
    let book = build_book(sigma, Some(pair_walls(pair)));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        single: JudgeLedger::default(),
        twin: JudgeLedger::default(),
        twin_params_exact: 0,
        twin_judged: 0,
        twin_rho_err_max: 0.0,
        twin_rho_err_mean: 0.0,
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
        if let Some((v, _, _)) = judge_single(&train.delivered) {
            score(&mut a.single, v, is_moth);
        }
        if let Some((v, pair_est, rho_est)) = judge_twin(&train.delivered) {
            score(&mut a.twin, v, is_moth);
            a.twin_judged += 1;
            if pair_est == pair {
                a.twin_params_exact += 1;
            }
            // Strengths are scored against the wall that owns the
            // estimated delay: a swapped pair still names both
            // walls, and its strengths are read by address.
            for (d_est, r_est) in [(pair_est.0, rho_est.0), (pair_est.1, rho_est.1)] {
                let truth = if d_est == pair.0 {
                    RHO_A
                } else if d_est == pair.1 {
                    RHO_B
                } else {
                    continue;
                };
                let e = (r_est - truth).abs();
                rho_err_sum += e;
                rho_n += 1;
                if e > a.twin_rho_err_max {
                    a.twin_rho_err_max = e;
                }
            }
        }
    }
    if rho_n > 0 {
        a.twin_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    println!("listener-twinwalled headless: one book of recorded trains, two walls at once, three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, wall pairs {:?}, strengths {:.2} / {:.2}, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        WALL_PAIRS,
        RHO_A,
        RHO_B
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), single (adrift's joint delay 1..=12 x rho 0.01 grid, whole-record score), twin (joint ordered delay pair x two rho on 0.05 grid, whole-record score)", LINE);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, (u64, u64), Audit)> = Vec::new();
    for sigma in sigmas {
        for pair in WALL_PAIRS {
            let a = audit(sigma, pair);
            println!(
                "  sigma {:.2} pair ({}, {}): arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), single errors {} (wrong {}, fooled {}), twin errors {} (wrong {}, fooled {}), twin pair exact {}/{}, twin rho err mean {:.3} max {:.3}",
                sigma,
                pair.0,
                pair.1,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.single.errors(),
                a.single.wrong_refusals,
                a.single.fooled,
                a.twin.errors(),
                a.twin.wrong_refusals,
                a.twin.fooled,
                a.twin_params_exact,
                a.twin_judged,
                a.twin_rho_err_mean,
                a.twin_rho_err_max
            );
            all.push((sigma, pair, a));
        }
    }
    let at = |s: f64, p: (u64, u64)| &all.iter().find(|(x, y, _)| *x == s && *y == p).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        WALL_PAIRS.iter().map(|p| pick(at(s, *p))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 two walls wound the naive judge at every pair (sigma 0.00 naive errors >= 20 at each pair): (3, 8) {}, (4, 9) {}, (4, 8) {} — {}",
        at(0.00, (3, 8)).naive.errors(),
        at(0.00, (4, 9)).naive.errors(),
        at(0.00, (4, 8)).naive.errors(),
        if WALL_PAIRS.iter().all(|p| at(0.00, *p).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the single search is a one-wall judge (sigma 0.00 single errors >= 20 at each pair): (3, 8) {}, (4, 9) {}, (4, 8) {} — {}",
        at(0.00, (3, 8)).single.errors(),
        at(0.00, (4, 9)).single.errors(),
        at(0.00, (4, 8)).single.errors(),
        if WALL_PAIRS.iter().all(|p| at(0.00, *p).single.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the twin search prices both walls in a clean cave (sigma 0.00: errors 0 at every pair, pair exact on every train, rho err max <= 0.011): errors {}/{}/{}, pair exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, (3, 8)).twin.errors(),
        at(0.00, (4, 9)).twin.errors(),
        at(0.00, (4, 8)).twin.errors(),
        at(0.00, (3, 8)).twin_params_exact,
        at(0.00, (4, 9)).twin_params_exact,
        at(0.00, (4, 8)).twin_params_exact,
        at(0.00, (3, 8)).twin_rho_err_max,
        at(0.00, (4, 9)).twin_rho_err_max,
        at(0.00, (4, 8)).twin_rho_err_max,
        if WALL_PAIRS.iter().all(|p| {
            let a = at(0.00, *p);
            a.twin.errors() == 0
                && a.twin_params_exact == a.twin_judged
                && a.twin_judged == TRAINS_PER_KIND * 2
                && a.twin_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 pooled at sigma 0.10 the twin judge errs less than the naive judge: twin {} vs naive {} — {}",
        pooled(0.10, |a| a.twin.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.twin.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let exact10: usize = WALL_PAIRS.iter().map(|p| at(0.10, *p).twin_params_exact).sum();
    let judged10: usize = WALL_PAIRS.iter().map(|p| at(0.10, *p).twin_judged).sum();
    println!(
        "    P5 at sigma 0.10 the twin judge still names both walls (pair exact on >= 90% of trains pooled): {}/{} — {}",
        exact10,
        judged10,
        if judged10 > 0 && (exact10 as f64) >= 0.90 * (judged10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 pooled the twin judge still errs less than the naive judge: twin {} vs naive {} — {}",
        pooled(0.30, |a| a.twin.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.twin.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
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
    fn the_separated_pair_interleaves_and_coincides_once() {
        // Pair (3, 8), stone struck at 3: own at 0/4/8, near echoes
        // at 3/7/11, far echoes at 8/12/16 — offset 8 is own_2
        // summed with the far wall's echo of own_0.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(pair_walls((3, 8))));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 3, 4, 7, 8, 11, 12, 16]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(8) - (own[2] + RHO_B * own[0])).abs() < 1e-12);
        assert!((at(3) - RHO_A * own[0]).abs() < 1e-12);
        assert!((at(11) - RHO_A * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_coincident_pair_sums_three_contributions_at_offset_8() {
        // Pair (4, 8), stone struck at 3: offset 8 carries own_2,
        // the near wall's echo of own_1, and the far wall's echo
        // of own_0 — three contributions, one arrival.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(pair_walls((4, 8))));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 8, 12, 16]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(8) - (own[2] + RHO_A * own[1] + RHO_B * own[0])).abs() < 1e-12);
        assert!((at(12) - (RHO_A * own[2] + RHO_B * own[1])).abs() < 1e-12);
    }

    #[test]
    fn every_pair_wounds_naive_and_single_and_is_priced_by_twin_at_sigma_zero() {
        for pair in WALL_PAIRS {
            let a = audit(0.0, pair);
            assert!(a.naive.errors() >= 20, "pair {pair:?} naive {}", a.naive.errors());
            assert!(a.single.errors() >= 20, "pair {pair:?} single {}", a.single.errors());
            assert_eq!(a.twin.errors(), 0, "pair {pair:?}");
            assert_eq!(a.twin_params_exact, TRAINS_PER_KIND * 2, "pair {pair:?}");
            assert!(a.twin_rho_err_max <= 0.011, "pair {pair:?} rho err {}", a.twin_rho_err_max);
        }
    }

    #[test]
    fn the_twin_search_names_pair_and_strengths_on_individual_trains_at_sigma_zero() {
        for pair in WALL_PAIRS {
            let book = build_book(0.0, Some(pair_walls(pair)));
            for t in &book {
                let (v, pair_est, rho_est) = judge_twin(&t.delivered).expect("twin judged");
                assert_eq!(v, t.kind == Kind::Moth, "pair {pair:?}");
                assert_eq!(pair_est, pair, "pair {pair:?}");
                assert!((rho_est.0 - RHO_A).abs() <= 0.011);
                assert!((rho_est.1 - RHO_B).abs() <= 0.011);
            }
        }
    }

    #[test]
    fn derive_own_unwinds_a_triple_summed_record() {
        // A hand-built record: own [1.0, 0.6, 1.0] behind pair
        // (4, 8). Offset 4 reads own_1 + RHO_A * own_0; offset 8
        // reads own_2 + RHO_A * own_1 + RHO_B * own_0.
        let own = [1.0, 0.6, 1.0];
        let d = [
            own[0],
            own[1] + RHO_A * own[0],
            own[2] + RHO_A * own[1] + RHO_B * own[0],
        ];
        let back = derive_own(d, &pair_walls((4, 8)));
        for j in 0..3 {
            assert!((back[j] - own[j]).abs() < 1e-12, "glint {j}");
        }
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, Some(pair_walls((4, 9))));
        let b2 = build_book(0.20, Some(pair_walls((4, 9))));
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
    fn noise_is_zero_at_sigma_zero_bounded_and_channel_separated_elsewhere() {
        for id in 1..50u64 {
            for channel in 0..=2u64 {
                assert_eq!(glint_noise(id, 0, channel, 0.0), 0.0);
            }
            for slot in 0..3 {
                let n0 = glint_noise(id, slot, 0, 0.30);
                let n1 = glint_noise(id, slot, 1, 0.30);
                let n2 = glint_noise(id, slot, 2, 0.30);
                assert!(n0.abs() <= 0.30 && n1.abs() <= 0.30 && n2.abs() <= 0.30);
                // The two walls' echoes do not share one noise draw.
                assert_ne!(n1, n2);
            }
        }
    }
}
