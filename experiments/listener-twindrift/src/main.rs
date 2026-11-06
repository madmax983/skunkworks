//! # Listener Twindrift 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, `experiments/listener-adrift`,
//! `experiments/listener-drifting`, and
//! `experiments/listener-twinwalled` — twinwalled's own closing seed,
//! taken up verbatim: *two walls at once with either of them in
//! motion: does the joint (pair x drift) search still separate two
//! trajectories, or do two moving walls finally price each other?*
//! twinwalled took the still half of drifting's seed; this planting
//! takes the motion half that has stood named in the loop's seeds
//! since run 55.
//!
//! A wall here is a trajectory: a base delay, a drift in
//! {-1, 0, +1} ticks per glint, and a strength rho. The echo of
//! glint j arrives `base + drift * j` ticks after it. Three regimes
//! are surveyed, the near wall at rho 0.55, the far at rho 0.40:
//!
//! - **still + walking:** near (4, 0), far (8, +1) — the far wall's
//!   echoes land at 8, 13, 18.
//! - **counter-walking:** near (4, +1), far (9, -1) — the near
//!   wall's echoes land at 4, 9, 14, the far wall's at 9, 12, 15;
//!   offset 9 sums two echoes from two different walls.
//! - **both walking, same way:** near (3, -1), far (8, -1) —
//!   echoes at 3, 6, 9 and 8, 11, 14.
//!
//! The record window runs to offset 20 so both walls' full returns
//! are delivered. Three judges read the SAME delivered record —
//! amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Twin:** listener-twinwalled's search verbatim — a joint
//!   search over an ordered pair of distinct STILL delays and two
//!   strengths on a 0.05 grid. It can count to two, but neither of
//!   its walls may move. It is never told any wall moves.
//! - **Twindrift:** a joint search over an ordered pair of
//!   trajectories (base in 3..=9, drift in {-1, 0, +1}, every
//!   per-glint delay inside 1..=12) and two strengths on the same
//!   0.05 grid. Each candidate derives the own glints it implies
//!   (subtracting, in strike order, any earlier glint's echo from
//!   either trajectory that lands exactly on a later own offset),
//!   predicts the whole record, and is scored by the summed
//!   absolute difference between prediction and delivery over
//!   every offset either mentions. The cheapest candidate wins.
//!   The judge is never told the true trajectories or strengths —
//!   those are the book's ground truth, kept for scoring only.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict
//! was right is scored by the book, never a judge.
//!
//! Predictions declared before the first run:
//! - P1: two walls wound the naive judge in every regime — at
//!   sigma 0.00 its errors are at least 20 in each regime.
//! - P2: the twin search is a still-pair judge — at sigma 0.00 it
//!   errs at least 20 times in each regime, including the regime
//!   where only one wall walks.
//! - P3: the twindrift search prices both trajectories in a clean
//!   cave — at sigma 0.00 its errors are 0 in every regime, its
//!   trajectory-pair estimate is exact on every train, and its
//!   rho error max (either wall) is <= 0.011.
//! - P4: pooled over the three regimes at sigma 0.10, the
//!   twindrift judge's errors are below the naive judge's.
//! - P5: at sigma 0.10 the twindrift judge still names both
//!   trajectories: its pair estimate is exact on at least 90% of
//!   trains pooled.
//! - P6: at sigma 0.30, pooled over the three regimes, the
//!   twindrift judge's errors are still below the naive judge's
//!   pooled errors (the noise floor does not fully swallow the
//!   search).
//!
//! `cargo run -p listener-twindrift -- --headless` prints the
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30 for each regime.
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
/// Trains per kind per regime in the book.
const TRAINS_PER_KIND: usize = 120;
/// Widest offset any prediction or delivery can mention.
const MAX_OFFSET: usize = 32;

/// One wall as a trajectory: base delay, drift per glint, strength.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Wall {
    base: u64,
    drift: i64,
    rho: f64,
}

/// The delay a wall applies to glint `j`: base + drift * j.
fn wall_delay(w: Wall, j: usize) -> u64 {
    (w.base as i64 + w.drift * j as i64) as u64
}

/// The three regimes surveyed, as (near wall, far wall) without
/// strengths (strengths are RHO_A / RHO_B by position).
const REGIMES: [((u64, i64), (u64, i64)); 3] = [
    ((4, 0), (8, 1)),   // still + walking
    ((4, 1), (9, -1)),  // counter-walking
    ((3, -1), (8, -1)), // both walking, same way
];

/// A regime as recording walls (trajectories with their strengths).
fn regime_walls(regime: ((u64, i64), (u64, i64))) -> [Wall; 2] {
    [
        Wall { base: regime.0 .0, drift: regime.0 .1, rho: RHO_A },
        Wall { base: regime.1 .0, drift: regime.1 .1, rho: RHO_B },
    ]
}

/// Canonical (sorted) trajectory pair of a regime, for scoring a
/// judge's estimate: trajectories ordered by (base, drift).
fn regime_pair(regime: ((u64, i64), (u64, i64))) -> ((u64, i64), (u64, i64)) {
    let a = regime.0;
    let b = regime.1;
    if (a.0, a.1) <= (b.0, b.1) { (a, b) } else { (b, a) }
}

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
/// echo from each wall, arriving wall_delay(wall, j) ticks later.
/// Contributions that land on the same offset sum in the ear
/// before the ear's threshold is applied; the record is delivered
/// sorted by arrival offset.
fn record_train(
    id: u64,
    kind: Kind,
    strike: u64,
    sigma: f64,
    walls: Option<[Wall; 2]>,
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
            for (wi, w) in ws.iter().enumerate() {
                add(
                    GLINT_DT * j as u64 + wall_delay(*w, j),
                    w.rho * own + glint_noise(id, j, (wi + 1) as u64, sigma),
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
/// judge at a given regime; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, walls: Option<[Wall; 2]>) -> Vec<Train> {
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
/// earlier glint's echo — from any candidate wall, at that
/// glint's own drifted delay — that lands exactly on it.
fn derive_own(d: [f64; 3], walls: &[Wall]) -> [f64; 3] {
    let mut own = [0.0; 3];
    for j in 0..OWN_GLINTS {
        let mut v = d[j];
        for i in 0..j {
            for w in walls {
                if GLINT_DT * i as u64 + wall_delay(*w, i) == GLINT_DT * j as u64 {
                    v -= w.rho * own[i];
                }
            }
        }
        own[j] = v;
    }
    own
}

/// Predict the record a set of own glints and candidate walls
/// produce, and score it against the delivery: summed absolute
/// difference over every offset either side mentions. (The twin
/// judge's scoring, verbatim in shape.)
fn whole_record_score(del: &[f64; MAX_OFFSET], own: [f64; 3], walls: &[Wall]) -> f64 {
    let mut pred = [0.0; MAX_OFFSET];
    for j in 0..OWN_GLINTS {
        pred[(GLINT_DT * j as u64) as usize] += own[j];
        for w in walls {
            pred[(GLINT_DT * j as u64 + wall_delay(*w, j)) as usize] += w.rho * own[j];
        }
    }
    let mut score = 0.0;
    for o in 0..MAX_OFFSET {
        score += (pred[o] - del[o]).abs();
    }
    score
}

/// The same whole-record score, computed sparsely: a candidate
/// record mentions at most 9 offsets, so the score is the
/// delivery's total absolute mass, minus the delivered mass at
/// the predicted offsets, plus the absolute residuals there.
/// Identical value to whole_record_score (asserted by test).
fn sparse_score(del: &[f64; MAX_OFFSET], del_total: f64, own: [f64; 3], walls: &[Wall]) -> f64 {
    let mut pred: [(u64, f64); 9] = [(0, 0.0); 9];
    let mut n = 0usize;
    let mut push = |offset: u64, amp: f64, pred: &mut [(u64, f64); 9], n: &mut usize| {
        if let Some(slot) = pred[..*n].iter_mut().find(|(o, _)| *o == offset) {
            slot.1 += amp;
        } else {
            pred[*n] = (offset, amp);
            *n += 1;
        }
    };
    for j in 0..OWN_GLINTS {
        push(GLINT_DT * j as u64, own[j], &mut pred, &mut n);
        for w in walls {
            push(GLINT_DT * j as u64 + wall_delay(*w, j), w.rho * own[j], &mut pred, &mut n);
        }
    }
    let mut score = del_total;
    for (o, p) in &pred[..n] {
        let d = del[*o as usize];
        score += (p - d).abs() - d.abs();
    }
    score
}

/// The twin judge: listener-twinwalled's search — a joint search
/// over an ordered pair of distinct STILL delays and two strengths
/// on a 0.05 grid. It can count to two but neither wall may move.
/// Returns the verdict, the winning delay pair, and the strengths.
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
                    let walls = [
                        Wall { base: d1, drift: 0, rho: r1 },
                        Wall { base: d2, drift: 0, rho: r2 },
                    ];
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

/// Every trajectory a twindrift candidate wall may take: base in
/// 3..=9, drift in {-1, 0, +1}, with every per-glint delay inside
/// 1..=12 — sorted by (base, drift), the canonical order.
fn trajectories() -> Vec<(u64, i64)> {
    let mut v = Vec::new();
    for base in 3..=9u64 {
        for drift in [-1i64, 0, 1] {
            let delays = [
                wall_delay(Wall { base, drift, rho: 0.0 }, 0),
                wall_delay(Wall { base, drift, rho: 0.0 }, 1),
                wall_delay(Wall { base, drift, rho: 0.0 }, 2),
            ];
            if delays.iter().all(|d| (1..=12).contains(d)) {
                v.push((base, drift));
            }
        }
    }
    v
}

/// The twindrift judge: a joint search over an ordered pair of
/// distinct trajectories and two strengths on a 0.05 grid. Each
/// candidate derives its own glints, predicts the whole record,
/// and is scored like the twin judge's candidates (computed
/// sparsely; the value is the same). Returns the verdict, the
/// winning trajectory pair, and the winning strengths.
fn judge_twindrift(
    delivered: &[Delivered],
) -> Option<(bool, ((u64, i64), (u64, i64)), (f64, f64))> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let trajs = trajectories();
    let mut best: Option<(f64, ((u64, i64), (u64, i64)), (f64, f64), [f64; 3])> = None;
    for a in 0..trajs.len() {
        for b in (a + 1)..trajs.len() {
            for i in 0..=19u32 {
                let r1 = i as f64 * 0.05;
                for k in 0..=19u32 {
                    let r2 = k as f64 * 0.05;
                    let walls = [
                        Wall { base: trajs[a].0, drift: trajs[a].1, rho: r1 },
                        Wall { base: trajs[b].0, drift: trajs[b].1, rho: r2 },
                    ];
                    let own = derive_own(own_amps, &walls);
                    let score = sparse_score(&del, del_total, own, &walls);
                    if best.map_or(true, |(s, _, _, _)| score < s) {
                        best = Some((score, (trajs[a], trajs[b]), (r1, r2), own));
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
    twin: JudgeLedger,
    td: JudgeLedger,
    td_params_exact: usize,
    td_judged: usize,
    td_rho_err_max: f64,
    td_rho_err_mean: f64,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, regime: ((u64, i64), (u64, i64))) -> Audit {
    let clean = build_book(sigma, None);
    let walls = regime_walls(regime);
    let truth_pair = regime_pair(regime);
    let book = build_book(sigma, Some(walls));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        twin: JudgeLedger::default(),
        td: JudgeLedger::default(),
        td_params_exact: 0,
        td_judged: 0,
        td_rho_err_max: 0.0,
        td_rho_err_mean: 0.0,
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
        if let Some((v, _, _)) = judge_twin(&train.delivered) {
            score(&mut a.twin, v, is_moth);
        }
        if let Some((v, pair_est, rho_est)) = judge_twindrift(&train.delivered) {
            score(&mut a.td, v, is_moth);
            a.td_judged += 1;
            if pair_est == truth_pair {
                a.td_params_exact += 1;
            }
            // Strengths are scored against the wall that owns the
            // estimated trajectory: a swapped pair still names both
            // walls, and its strengths are read by trajectory.
            for (t_est, r_est) in [(pair_est.0, rho_est.0), (pair_est.1, rho_est.1)] {
                let truth = if t_est == regime.0 {
                    RHO_A
                } else if t_est == regime.1 {
                    RHO_B
                } else {
                    continue;
                };
                let e = (r_est - truth).abs();
                rho_err_sum += e;
                rho_n += 1;
                if e > a.td_rho_err_max {
                    a.td_rho_err_max = e;
                }
            }
        }
    }
    if rho_n > 0 {
        a.td_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    println!("listener-twindrift headless: one book of recorded trains, two walls as trajectories, three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, regimes {:?}, strengths {:.2} / {:.2}, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        REGIMES,
        RHO_A,
        RHO_B
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), twin (twinwalled's joint still delay pair x two rho on 0.05 grid, whole-record score), twindrift (joint trajectory pair, base 3..=9 x drift -1/0/+1, x two rho on 0.05 grid, whole-record score)", LINE);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, ((u64, i64), (u64, i64)), Audit)> = Vec::new();
    for sigma in sigmas {
        for regime in REGIMES {
            let a = audit(sigma, regime);
            println!(
                "  sigma {:.2} regime (({}, {:+}), ({}, {:+})): arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), twin errors {} (wrong {}, fooled {}), twindrift errors {} (wrong {}, fooled {}), twindrift traj exact {}/{}, twindrift rho err mean {:.3} max {:.3}",
                sigma,
                regime.0 .0,
                regime.0 .1,
                regime.1 .0,
                regime.1 .1,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.twin.errors(),
                a.twin.wrong_refusals,
                a.twin.fooled,
                a.td.errors(),
                a.td.wrong_refusals,
                a.td.fooled,
                a.td_params_exact,
                a.td_judged,
                a.td_rho_err_mean,
                a.td_rho_err_max
            );
            all.push((sigma, regime, a));
        }
    }
    let at = |s: f64, r: ((u64, i64), (u64, i64))| {
        &all.iter().find(|(x, y, _)| *x == s && *y == r).unwrap().2
    };
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        REGIMES.iter().map(|r| pick(at(s, *r))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 two walls wound the naive judge in every regime (sigma 0.00 naive errors >= 20 in each regime): {} / {} / {} — {}",
        at(0.00, REGIMES[0]).naive.errors(),
        at(0.00, REGIMES[1]).naive.errors(),
        at(0.00, REGIMES[2]).naive.errors(),
        if REGIMES.iter().all(|r| at(0.00, *r).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the twin search is a still-pair judge (sigma 0.00 twin errors >= 20 in each regime): {} / {} / {} — {}",
        at(0.00, REGIMES[0]).twin.errors(),
        at(0.00, REGIMES[1]).twin.errors(),
        at(0.00, REGIMES[2]).twin.errors(),
        if REGIMES.iter().all(|r| at(0.00, *r).twin.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the twindrift search prices both trajectories in a clean cave (sigma 0.00: errors 0 in every regime, traj exact on every train, rho err max <= 0.011): errors {}/{}/{}, traj exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, REGIMES[0]).td.errors(),
        at(0.00, REGIMES[1]).td.errors(),
        at(0.00, REGIMES[2]).td.errors(),
        at(0.00, REGIMES[0]).td_params_exact,
        at(0.00, REGIMES[1]).td_params_exact,
        at(0.00, REGIMES[2]).td_params_exact,
        at(0.00, REGIMES[0]).td_rho_err_max,
        at(0.00, REGIMES[1]).td_rho_err_max,
        at(0.00, REGIMES[2]).td_rho_err_max,
        if REGIMES.iter().all(|r| {
            let a = at(0.00, *r);
            a.td.errors() == 0
                && a.td_params_exact == a.td_judged
                && a.td_judged == TRAINS_PER_KIND * 2
                && a.td_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 pooled at sigma 0.10 the twindrift judge errs less than the naive judge: twindrift {} vs naive {} — {}",
        pooled(0.10, |a| a.td.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.td.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let exact10: usize = REGIMES.iter().map(|r| at(0.10, *r).td_params_exact).sum();
    let judged10: usize = REGIMES.iter().map(|r| at(0.10, *r).td_judged).sum();
    println!(
        "    P5 at sigma 0.10 the twindrift judge still names both trajectories (traj exact on >= 90% of trains pooled): {}/{} — {}",
        exact10,
        judged10,
        if judged10 > 0 && (exact10 as f64) >= 0.90 * (judged10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 pooled the twindrift judge still errs less than the naive judge: twindrift {} vs naive {} — {}",
        pooled(0.30, |a| a.td.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.td.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
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
    fn the_counter_walking_regime_sums_two_walls_at_offset_9() {
        // Regime ((4, +1), (9, -1)), stone struck at 3: own at
        // 0/4/8; near echoes at 4/9/14; far echoes at 9/12/15.
        // Offset 4 is own_1 + near echo of own_0; offset 9 is the
        // near echo of own_1 summed with the far echo of own_0.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_walls(REGIMES[1])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 8, 9, 12, 14, 15]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(4) - (own[1] + RHO_A * own[0])).abs() < 1e-12);
        assert!((at(9) - (RHO_A * own[1] + RHO_B * own[0])).abs() < 1e-12);
        assert!((at(14) - RHO_A * own[2]).abs() < 1e-12);
        assert!((at(15) - RHO_B * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_still_plus_walking_regime_walks_only_the_far_wall() {
        // Regime ((4, 0), (8, +1)), stone struck at 3: near echoes
        // at 4/8/12 (the still wall, coincident), far echoes at
        // 8/13/18 — offset 8 sums own_2, the near echo of own_1,
        // and the far echo of own_0.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_walls(REGIMES[0])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 8, 12, 13, 18]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(8) - (own[2] + RHO_A * own[1] + RHO_B * own[0])).abs() < 1e-12);
        assert!((at(13) - RHO_B * own[1]).abs() < 1e-12);
    }

    #[test]
    fn sparse_score_equals_whole_record_score() {
        let book = build_book(0.20, Some(regime_walls(REGIMES[1])));
        let walls = [
            Wall { base: 5, drift: -1, rho: 0.35 },
            Wall { base: 8, drift: 1, rho: 0.60 },
        ];
        for t in book.iter().take(12) {
            let del = delivered_array(&t.delivered);
            let total: f64 = del.iter().map(|x| x.abs()).sum();
            let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).map(|d| d.amp).unwrap_or(0.0);
            let own = derive_own([at(0), at(4), at(8)], &walls);
            let a = whole_record_score(&del, own, &walls);
            let b = sparse_score(&del, total, own, &walls);
            assert!((a - b).abs() < 1e-9, "whole {a} sparse {b}");
        }
    }

    #[test]
    fn every_regime_wounds_naive_and_twin_and_is_priced_by_twindrift_at_sigma_zero() {
        for regime in REGIMES {
            let a = audit(0.0, regime);
            assert!(a.naive.errors() >= 20, "regime {regime:?} naive {}", a.naive.errors());
            assert!(a.twin.errors() >= 20, "regime {regime:?} twin {}", a.twin.errors());
            assert_eq!(a.td.errors(), 0, "regime {regime:?}");
            assert_eq!(a.td_params_exact, TRAINS_PER_KIND * 2, "regime {regime:?}");
            assert!(a.td_rho_err_max <= 0.011, "regime {regime:?} rho err {}", a.td_rho_err_max);
        }
    }

    #[test]
    fn the_twindrift_search_names_trajectories_and_strengths_on_individual_trains_at_sigma_zero() {
        for regime in REGIMES {
            let book = build_book(0.0, Some(regime_walls(regime)));
            for t in &book {
                let (v, pair_est, rho_est) = judge_twindrift(&t.delivered).expect("td judged");
                assert_eq!(v, t.kind == Kind::Moth, "regime {regime:?}");
                assert_eq!(pair_est, regime_pair(regime), "regime {regime:?}");
                // Strengths are read by trajectory, as in the audit.
                for (t_est, r_est) in [(pair_est.0, rho_est.0), (pair_est.1, rho_est.1)] {
                    let truth = if t_est == regime.0 { RHO_A } else { RHO_B };
                    assert!((r_est - truth).abs() <= 0.011, "regime {regime:?} traj {t_est:?}");
                }
            }
        }
    }

    #[test]
    fn derive_own_unwinds_a_counter_walking_record() {
        // Hand-built: own [1.0, 0.6, 1.0] behind regime
        // ((4, +1), (9, -1)). Offset 4 reads own_1 + RHO_A * own_0
        // (near wall, glint 0, delay 4); no echo lands on offset 8.
        let own = [1.0, 0.6, 1.0];
        let d = [own[0], own[1] + RHO_A * own[0], own[2]];
        let back = derive_own(d, &regime_walls(REGIMES[1]));
        for j in 0..3 {
            assert!((back[j] - own[j]).abs() < 1e-12, "glint {j}");
        }
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, Some(regime_walls(REGIMES[2])));
        let b2 = build_book(0.20, Some(regime_walls(REGIMES[2])));
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
                assert_ne!(n1, n2);
            }
        }
    }
}
