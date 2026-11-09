//! # Listener Vdrift 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, `experiments/listener-adrift`,
//! `experiments/listener-drifting`, `experiments/listener-twinwalled`,
//! `experiments/listener-twindrift`, `experiments/listener-twincurve`,
//! and `experiments/listener-census` — twindrift's closing seed, kept
//! standing through twincurve and the census, taken up verbatim: *a
//! wall whose drift changes sign mid-flight (a V in the record).*
//!
//! A wall here is a two-step trajectory: a base delay and two steps
//! (s0, s1), one per glint gap. The echo of glint j arrives after
//! delay(j) ticks, where delay(0) = base, delay(1) = base + s0, and
//! delay(2) = base + s0 + s1. The lineage's families are shapes of
//! (s0, s1): a LINEAR wall has s1 = s0; a CURVED wall has
//! s1 = s0 + accel with accel in {-1, 0, +1}. A V wall turns at the
//! joint: s1 = -s0, with s0 in {+1, +2, -2}. No V in this cave is a
//! linear wall (that would need s0 = -s0) and none is a curved wall
//! (the shallow V needs accel -2, the wide Vs carry |s0| = 2, outside
//! the curved family's drift range entirely). Three regimes are
//! surveyed, the single wall at rho 0.55:
//!
//! - **shallow V, out and back:** (6, +1, -1) — delays 6, 7, 6;
//!   echoes land at offsets 6, 11, 14.
//! - **wide V, down then up:** (7, -2, +2) — delays 7, 5, 7;
//!   echoes land at 7, 9, 15.
//! - **wide V, up then down:** (5, +2, -2) — delays 5, 7, 5;
//!   echoes land at 5, 11, 13.
//!
//! Four judges read the SAME delivered record — amplitudes and
//! arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Drift:** listener-twindrift's family, alone — a search over
//!   LINEAR trajectories (base in 3..=9, step in {-1, 0, +1}, both
//!   steps equal) and one strength on a 0.05 grid. It is never told
//!   any wall turns.
//! - **Curve:** listener-twincurve's family, alone — a search over
//!   CURVED trajectories (drift in {-1, 0, +1}, accel in {-1, 0, +1})
//!   and one strength on the same grid. It can bend a wall, but only
//!   by one tick of accel, and its first step is at most one tick.
//! - **Vdrift:** a search over JOINTED trajectories — base in
//!   3..=9, each step free in {-2, -1, 0, +1, +2}, every per-glint
//!   delay inside 1..=12 — and one strength on the same grid. The
//!   linear and curved families are subsets of this one; the joint
//!   is the novel trait. Each candidate derives the own glints it
//!   implies, predicts the whole record, and is scored by the
//!   summed absolute difference between prediction and delivery
//!   over every offset either mentions, computed sparsely with an
//!   abort bound that preserves exactness. The cheapest wins.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict
//! was right is scored by the book, never a judge.
//!
//! Predictions declared before the shipped run (re-declared
//! around one design correction — a first draft declared the
//! drift judge wounded in every regime, and the wide down-then-up
//! V returned verdict-clean under it; that draft is kept in the
//! README and the split below is what the shipped run judges):
//! - P1: the V wall wounds the naive judge in every regime — at
//!   sigma 0.00 its errors are at least 20 in each regime.
//! - P2: the drift search is a linear judge — at sigma 0.00 it
//!   names the true trajectory on zero trains in every regime
//!   (the V is not in its space), and it errs at least 20 times
//!   in the shallow V and the wide up-then-down V.
//! - P3: the wide down-then-up V is the exception that prices the
//!   rule — at sigma 0.00 the drift judge errs 0 times there
//!   despite naming the truth on zero trains, and its modal
//!   substitute is the still wall at the V's own base delay,
//!   (7, 0, 0): verdict-perfect, trajectory-blind.
//! - P4: the curve search is a one-tick-bend judge — at sigma 0.00
//!   it errs at least 20 times in each regime and names the true
//!   trajectory on zero trains in every regime (the shallow V
//!   needs accel -2; the wide Vs open with a two-tick step).
//! - P5: the vdrift search prices the jointed wall in a clean
//!   cave — at sigma 0.00 its errors are 0 in every regime, its
//!   trajectory estimate is exact on every train, and its rho
//!   error max is <= 0.011 in every regime.
//! - P6: pooled over the three regimes at sigma 0.10, the vdrift
//!   judge's errors are below the naive judge's.
//! - P7: at sigma 0.10 the vdrift judge still names the joint:
//!   its trajectory estimate is exact on at least 90% of trains
//!   pooled.
//! - P8: at sigma 0.30, pooled over the three regimes, the vdrift
//!   judge's errors are still below the naive judge's pooled
//!   errors.
//!
//! `cargo run -p listener-vdrift -- --headless` prints the ledgers
//! at sigma = 0.00, 0.10, 0.20, 0.30 for each regime, plus, at
//! sigma 0.00, the trajectory each predecessor judge most often
//! names in place of the V — what straightening looks like.
//! There is no TUI; --headless is handled before anything else
//! regardless.

use anyhow::Result;
use std::collections::BTreeMap;

/// The flutter line every Listener since flutter has used.
const LINE: f64 = 0.30;
/// Glints under this amplitude never reach an ear (ripple's ear).
const EAR: f64 = 0.22;
/// Own glints per train (flutter's train).
const OWN_GLINTS: usize = 3;
/// Ticks between own glints.
const GLINT_DT: u64 = 4;
/// The single wall's strength: each return re-arrives at this fraction.
const RHO: f64 = 0.55;
/// Trains per kind per regime in the book.
const TRAINS_PER_KIND: usize = 120;
/// Widest offset any prediction or delivery can mention.
const MAX_OFFSET: usize = 32;

/// A trajectory: base delay and the two per-gap steps.
type Traj = (u64, i64, i64);

/// One wall as a jointed trajectory: base, step 0, step 1, strength.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Wall {
    base: u64,
    s0: i64,
    s1: i64,
    rho: f64,
}

/// The delay a wall applies to glint `j`.
fn wall_delay(w: Wall, j: usize) -> u64 {
    let d = match j {
        0 => 0,
        1 => w.s0,
        _ => w.s0 + w.s1,
    };
    (w.base as i64 + d) as u64
}

/// The three regimes surveyed, as trajectories without strength.
const REGIMES: [Traj; 3] = [
    (6, 1, -1), // shallow V, out and back
    (7, -2, 2), // wide V, down then up
    (5, 2, -2), // wide V, up then down
];

/// A regime as a recording wall (trajectory with its strength).
fn regime_wall(regime: Traj) -> Wall {
    Wall { base: regime.0, s0: regime.1, s1: regime.2, rho: RHO }
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
/// identity, the glint slot, and the channel (0 = own glint,
/// 1 = the wall's echo), so every judge — and every rerun — hears
/// the same record.
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
/// 0, 4, 8 from `strike`, and — if `wall` is Some — each glint's
/// echo from the wall, arriving wall_delay(wall, j) ticks later.
/// Contributions that land on the same offset sum in the ear
/// before the ear's threshold is applied; the record is delivered
/// sorted by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, wall: Option<Wall>) -> Train {
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
        if let Some(w) = wall {
            add(
                GLINT_DT * j as u64 + wall_delay(w, j),
                w.rho * own + glint_noise(id, j, 1, sigma),
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
/// judge at a given regime; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, wall: Option<Wall>) -> Vec<Train> {
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
// The four judges. Each reads only delivered amplitudes and offsets.
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

/// Derive the own glints a candidate wall implies: walk the own
/// offsets in strike order and subtract, from each, every earlier
/// glint's echo from the candidate wall that lands exactly on it.
fn derive_own(d: [f64; 3], w: Wall) -> [f64; 3] {
    let mut own = [0.0; 3];
    for j in 0..OWN_GLINTS {
        let mut v = d[j];
        for i in 0..j {
            if GLINT_DT * i as u64 + wall_delay(w, i) == GLINT_DT * j as u64 {
                v -= w.rho * own[i];
            }
        }
        own[j] = v;
    }
    own
}

/// Predict the record a set of own glints and a candidate wall
/// produce, and score it against the delivery: summed absolute
/// difference over every offset either side mentions.
#[allow(dead_code)] // dense reference score; exercised by tests
fn whole_record_score(del: &[f64; MAX_OFFSET], own: [f64; 3], w: Wall) -> f64 {
    let mut pred = [0.0; MAX_OFFSET];
    for j in 0..OWN_GLINTS {
        pred[(GLINT_DT * j as u64) as usize] += own[j];
        pred[(GLINT_DT * j as u64 + wall_delay(w, j)) as usize] += w.rho * own[j];
    }
    let mut score = 0.0;
    for o in 0..MAX_OFFSET {
        score += (pred[o] - del[o]).abs();
    }
    score
}

/// The same whole-record score, computed sparsely with an abort
/// bound: identical to whole_record_score whenever it completes,
/// but abandons the candidate (returning f64::INFINITY) as soon as
/// the partial lower bound — the residuals already summed plus the
/// delivered mass at offsets the candidate never mentions, which
/// no later term can reduce — exceeds `limit`. Exactness is
/// preserved: a candidate is only ever abandoned when its true
/// score is provably above the incumbent's.
fn sparse_score_abort(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own: [f64; 3],
    w: Wall,
    limit: f64,
) -> f64 {
    let mut pred: [(u64, f64); 6] = [(0, 0.0); 6];
    let mut n = 0usize;
    let mut push = |offset: u64, amp: f64| {
        if let Some(slot) = pred[..n].iter_mut().find(|(o, _)| *o == offset) {
            slot.1 += amp;
        } else {
            pred[n] = (offset, amp);
            n += 1;
        }
    };
    for j in 0..OWN_GLINTS {
        push(GLINT_DT * j as u64, own[j]);
        push(GLINT_DT * j as u64 + wall_delay(w, j), w.rho * own[j]);
    }
    let mentioned: f64 = pred[..n].iter().map(|(o, _)| del[*o as usize].abs()).sum();
    let mut score = del_total - mentioned;
    if score > limit {
        return f64::INFINITY;
    }
    for (o, p) in &pred[..n] {
        score += (p - del[*o as usize]).abs();
        if score > limit {
            return f64::INFINITY;
        }
    }
    score
}

/// The plain sparse score (no abort), for tests.
#[allow(dead_code)] // exercised by tests
fn sparse_score(del: &[f64; MAX_OFFSET], del_total: f64, own: [f64; 3], w: Wall) -> f64 {
    sparse_score_abort(del, del_total, own, w, f64::INFINITY)
}

/// Every trajectory in a family, sorted by (base, s0, s1) and
/// deduplicated, with every per-glint delay inside 1..=12.
/// `steps` supplies the (s0, s1) pairs the family allows.
fn trajectories(steps: &[(i64, i64)]) -> Vec<Traj> {
    let mut v = Vec::new();
    for base in 3..=9u64 {
        for &(s0, s1) in steps {
            let w = Wall { base, s0, s1, rho: 0.0 };
            let delays = [wall_delay(w, 0), wall_delay(w, 1), wall_delay(w, 2)];
            if delays.iter().all(|d| (1..=12).contains(d)) {
                v.push((base, s0, s1));
            }
        }
    }
    v.sort();
    v.dedup();
    v
}

/// The linear family: both steps equal, in {-1, 0, +1}.
fn linear_trajectories() -> Vec<Traj> {
    let steps: Vec<(i64, i64)> = (-1..=1).map(|s| (s, s)).collect();
    trajectories(&steps)
}

/// The curved family: drift in {-1, 0, +1}, accel in {-1, 0, +1},
/// so the steps are (d, d + a).
fn curved_trajectories() -> Vec<Traj> {
    let mut steps = Vec::new();
    for d in -1..=1i64 {
        for a in -1..=1i64 {
            steps.push((d, d + a));
        }
    }
    trajectories(&steps)
}

/// The jointed family: each step free in {-2, -1, 0, +1, +2}.
fn jointed_trajectories() -> Vec<Traj> {
    let mut steps = Vec::new();
    for s0 in -2..=2i64 {
        for s1 in -2..=2i64 {
            steps.push((s0, s1));
        }
    }
    trajectories(&steps)
}

/// A search over one trajectory family: every trajectory in the
/// family and one strength on a 0.05 grid. Each candidate derives
/// its own glints, predicts the whole record, and is scored
/// sparsely with the abort bound against the incumbent. Returns
/// the verdict, the winning trajectory, and the winning strength.
fn judge_search(delivered: &[Delivered], trajs: &[Traj]) -> Option<(bool, Traj, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let mut best: Option<(f64, Traj, f64, [f64; 3])> = None;
    for &t in trajs {
        for i in 0..=19u32 {
            let r = i as f64 * 0.05;
            let w = Wall { base: t.0, s0: t.1, s1: t.2, rho: r };
            let own = derive_own(own_amps, w);
            let limit = best.map_or(f64::INFINITY, |(s, _, _, _)| s);
            let score = sparse_score_abort(&del, del_total, own, w, limit);
            if best.map_or(true, |(s, _, _, _)| score < s) {
                best = Some((score, t, r, own));
            }
        }
    }
    let (_, traj, rho, own) = best?;
    Some((spread(&own) > LINE, traj, rho))
}

/// The drift judge: the search over LINEAR trajectories only.
fn judge_drift(delivered: &[Delivered]) -> Option<(bool, Traj, f64)> {
    judge_search(delivered, &linear_trajectories())
}

/// The curve judge: the search over CURVED trajectories only.
fn judge_curve(delivered: &[Delivered]) -> Option<(bool, Traj, f64)> {
    judge_search(delivered, &curved_trajectories())
}

/// The vdrift judge: the search over JOINTED trajectories.
fn judge_vdrift(delivered: &[Delivered]) -> Option<(bool, Traj, f64)> {
    judge_search(delivered, &jointed_trajectories())
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
    drift: JudgeLedger,
    drift_exact: usize,
    curve: JudgeLedger,
    curve_exact: usize,
    vd: JudgeLedger,
    vd_exact: usize,
    vd_judged: usize,
    vd_rho_err_max: f64,
    vd_rho_err_mean: f64,
    drift_mode: (Traj, usize),
    curve_mode: (Traj, usize),
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn mode_of(counts: &BTreeMap<Traj, usize>) -> (Traj, usize) {
    counts
        .iter()
        .max_by_key(|(_, c)| *c)
        .map(|(t, c)| (*t, *c))
        .unwrap_or(((0, 0, 0), 0))
}

fn audit(sigma: f64, regime: Traj) -> Audit {
    let clean = build_book(sigma, None);
    let wall = regime_wall(regime);
    let book = build_book(sigma, Some(wall));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        drift: JudgeLedger::default(),
        drift_exact: 0,
        curve: JudgeLedger::default(),
        curve_exact: 0,
        vd: JudgeLedger::default(),
        vd_exact: 0,
        vd_judged: 0,
        vd_rho_err_max: 0.0,
        vd_rho_err_mean: 0.0,
        drift_mode: ((0, 0, 0), 0),
        curve_mode: ((0, 0, 0), 0),
    };
    for train in &clean {
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.clean_naive, v, is_moth);
        }
    }
    let mut rho_err_sum = 0.0;
    let mut rho_n = 0usize;
    let mut drift_counts: BTreeMap<Traj, usize> = BTreeMap::new();
    let mut curve_counts: BTreeMap<Traj, usize> = BTreeMap::new();
    for train in &book {
        a.arrivals += train.delivered.len();
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.naive, v, is_moth);
        }
        if let Some((v, est, _)) = judge_drift(&train.delivered) {
            score(&mut a.drift, v, is_moth);
            if est == regime {
                a.drift_exact += 1;
            }
            *drift_counts.entry(est).or_insert(0) += 1;
        }
        if let Some((v, est, _)) = judge_curve(&train.delivered) {
            score(&mut a.curve, v, is_moth);
            if est == regime {
                a.curve_exact += 1;
            }
            *curve_counts.entry(est).or_insert(0) += 1;
        }
        if let Some((v, est, r_est)) = judge_vdrift(&train.delivered) {
            score(&mut a.vd, v, is_moth);
            a.vd_judged += 1;
            if est == regime {
                a.vd_exact += 1;
            }
            let e = (r_est - RHO).abs();
            rho_err_sum += e;
            rho_n += 1;
            if e > a.vd_rho_err_max {
                a.vd_rho_err_max = e;
            }
        }
    }
    if rho_n > 0 {
        a.vd_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a.drift_mode = mode_of(&drift_counts);
    a.curve_mode = mode_of(&curve_counts);
    a
}

fn run_headless() {
    println!("listener-vdrift headless: one book of recorded trains, one wall that turns at the joint, four judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, regimes {:?}, strength {:.2}, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        REGIMES,
        RHO
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), drift (linear trajectories x rho on 0.05 grid), curve (curved trajectories, accel -1/0/+1, x rho), vdrift (jointed trajectories, steps free in -2..=+2, x rho); all searches whole-record score", LINE);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, Traj, Audit)> = Vec::new();
    for sigma in sigmas {
        for regime in REGIMES {
            let a = audit(sigma, regime);
            println!(
                "  sigma {:.2} regime ({}, {:+}, {:+}): arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), drift errors {} (wrong {}, fooled {}), drift traj exact {}, curve errors {} (wrong {}, fooled {}), curve traj exact {}, vdrift errors {} (wrong {}, fooled {}), vdrift traj exact {}/{}, vdrift rho err mean {:.3} max {:.3}",
                sigma,
                regime.0,
                regime.1,
                regime.2,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.drift.errors(),
                a.drift.wrong_refusals,
                a.drift.fooled,
                a.drift_exact,
                a.curve.errors(),
                a.curve.wrong_refusals,
                a.curve.fooled,
                a.curve_exact,
                a.vd.errors(),
                a.vd.wrong_refusals,
                a.vd.fooled,
                a.vd_exact,
                a.vd_judged,
                a.vd_rho_err_mean,
                a.vd_rho_err_max
            );
            if sigma == 0.0 {
                println!(
                    "    sigma 0.00 substitutes for regime ({}, {:+}, {:+}): drift most often names ({}, {:+}, {:+}) on {} trains; curve most often names ({}, {:+}, {:+}) on {} trains",
                    regime.0,
                    regime.1,
                    regime.2,
                    a.drift_mode.0 .0,
                    a.drift_mode.0 .1,
                    a.drift_mode.0 .2,
                    a.drift_mode.1,
                    a.curve_mode.0 .0,
                    a.curve_mode.0 .1,
                    a.curve_mode.0 .2,
                    a.curve_mode.1
                );
            }
            all.push((sigma, regime, a));
        }
    }
    let at = |s: f64, r: Traj| &all.iter().find(|(x, y, _)| *x == s && *y == r).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        REGIMES.iter().map(|r| pick(at(s, *r))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the V wall wounds the naive judge in every regime (sigma 0.00 naive errors >= 20 in each regime): {} / {} / {} — {}",
        at(0.00, REGIMES[0]).naive.errors(),
        at(0.00, REGIMES[1]).naive.errors(),
        at(0.00, REGIMES[2]).naive.errors(),
        if REGIMES.iter().all(|r| at(0.00, *r).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the drift search is a linear judge (sigma 0.00: traj exact 0 in each regime; errors >= 20 in the shallow V and the wide up-then-down V): errors {} / {} / {}, exact {} / {} / {} — {}",
        at(0.00, REGIMES[0]).drift.errors(),
        at(0.00, REGIMES[1]).drift.errors(),
        at(0.00, REGIMES[2]).drift.errors(),
        at(0.00, REGIMES[0]).drift_exact,
        at(0.00, REGIMES[1]).drift_exact,
        at(0.00, REGIMES[2]).drift_exact,
        if REGIMES.iter().all(|r| at(0.00, *r).drift_exact == 0)
            && at(0.00, REGIMES[0]).drift.errors() >= 20
            && at(0.00, REGIMES[2]).drift.errors() >= 20
        { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the wide down-then-up V hides from the linear judge's verdicts (sigma 0.00: drift errors 0 there, modal substitute the still wall (7, 0, 0)): errors {}, mode ({}, {:+}, {:+}) on {} trains — {}",
        at(0.00, REGIMES[1]).drift.errors(),
        at(0.00, REGIMES[1]).drift_mode.0 .0,
        at(0.00, REGIMES[1]).drift_mode.0 .1,
        at(0.00, REGIMES[1]).drift_mode.0 .2,
        at(0.00, REGIMES[1]).drift_mode.1,
        if at(0.00, REGIMES[1]).drift.errors() == 0
            && at(0.00, REGIMES[1]).drift_mode.0 == (7, 0, 0)
        { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the curve search is a one-tick-bend judge (sigma 0.00: errors >= 20 in each regime, traj exact 0 in each): errors {} / {} / {}, exact {} / {} / {} — {}",
        at(0.00, REGIMES[0]).curve.errors(),
        at(0.00, REGIMES[1]).curve.errors(),
        at(0.00, REGIMES[2]).curve.errors(),
        at(0.00, REGIMES[0]).curve_exact,
        at(0.00, REGIMES[1]).curve_exact,
        at(0.00, REGIMES[2]).curve_exact,
        if REGIMES.iter().all(|r| {
            let a = at(0.00, *r);
            a.curve.errors() >= 20 && a.curve_exact == 0
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 the vdrift search prices the jointed wall in a clean cave (sigma 0.00: errors 0 in every regime, traj exact on every train, rho err max <= 0.011): errors {}/{}/{}, traj exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, REGIMES[0]).vd.errors(),
        at(0.00, REGIMES[1]).vd.errors(),
        at(0.00, REGIMES[2]).vd.errors(),
        at(0.00, REGIMES[0]).vd_exact,
        at(0.00, REGIMES[1]).vd_exact,
        at(0.00, REGIMES[2]).vd_exact,
        at(0.00, REGIMES[0]).vd_rho_err_max,
        at(0.00, REGIMES[1]).vd_rho_err_max,
        at(0.00, REGIMES[2]).vd_rho_err_max,
        if REGIMES.iter().all(|r| {
            let a = at(0.00, *r);
            a.vd.errors() == 0
                && a.vd_exact == a.vd_judged
                && a.vd_judged == TRAINS_PER_KIND * 2
                && a.vd_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 pooled at sigma 0.10 the vdrift judge errs less than the naive judge: vdrift {} vs naive {} — {}",
        pooled(0.10, |a| a.vd.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.vd.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let exact10: usize = REGIMES.iter().map(|r| at(0.10, *r).vd_exact).sum();
    let judged10: usize = REGIMES.iter().map(|r| at(0.10, *r).vd_judged).sum();
    println!(
        "    P7 at sigma 0.10 the vdrift judge still names the joint (traj exact on >= 90% of trains pooled): {}/{} — {}",
        exact10,
        judged10,
        if judged10 > 0 && (exact10 as f64) >= 0.90 * (judged10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P8 at sigma 0.30 pooled the vdrift judge still errs less than the naive judge: vdrift {} vs naive {} — {}",
        pooled(0.30, |a| a.vd.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.vd.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
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
    fn v_wall_delays_turn_at_the_joint() {
        let delays = |t: Traj| {
            let w = regime_wall(t);
            [wall_delay(w, 0), wall_delay(w, 1), wall_delay(w, 2)]
        };
        assert_eq!(delays(REGIMES[0]), [6, 7, 6]);
        assert_eq!(delays(REGIMES[1]), [7, 5, 7]);
        assert_eq!(delays(REGIMES[2]), [5, 7, 5]);
        for t in REGIMES {
            assert_eq!(t.2, -t.1, "regime {t:?} is a V: second step negates the first");
            assert_ne!(t.1, 0);
        }
    }

    #[test]
    fn no_regime_trajectory_is_in_the_linear_or_curved_family() {
        let lin = linear_trajectories();
        let cur = curved_trajectories();
        let joi = jointed_trajectories();
        for t in REGIMES {
            assert!(!lin.contains(&t), "regime {t:?} in linear family");
            assert!(!cur.contains(&t), "regime {t:?} in curved family");
            assert!(joi.contains(&t), "regime {t:?} not in jointed family");
        }
        // And the families nest: linear ⊂ curved ⊂ jointed.
        for t in &lin {
            assert!(cur.contains(t), "linear {t:?} not in curved");
            assert!(joi.contains(t), "linear {t:?} not in jointed");
        }
        for t in &cur {
            assert!(joi.contains(t), "curved {t:?} not in jointed");
        }
    }

    #[test]
    fn the_shallow_v_regime_echoes_land_at_6_11_14() {
        // Regime (6, +1, -1), stone struck at 3: own at 0/4/8;
        // echoes at 0+6, 4+7, 8+6 = 6/11/14, none on an own offset.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_wall(REGIMES[0])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 6, 8, 11, 14]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(6) - RHO * own[0]).abs() < 1e-12);
        assert!((at(11) - RHO * own[1]).abs() < 1e-12);
        assert!((at(14) - RHO * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_wide_v_regimes_echo_where_declared() {
        // Regime (7, -2, +2), stone at 3: echoes at 7, 4+5, 8+7 = 7/9/15.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_wall(REGIMES[1])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 7, 8, 9, 15]);
        // Regime (5, +2, -2), stone at 3: echoes at 5, 4+7, 8+5 = 5/11/13.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_wall(REGIMES[2])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 5, 8, 11, 13]);
    }

    #[test]
    fn sparse_score_equals_whole_record_score() {
        let book = build_book(0.20, Some(regime_wall(REGIMES[1])));
        let w = Wall { base: 5, s0: -1, s1: 2, rho: 0.35 };
        for t in book.iter().take(12) {
            let del = delivered_array(&t.delivered);
            let total: f64 = del.iter().map(|x| x.abs()).sum();
            let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).map(|d| d.amp).unwrap_or(0.0);
            let own = derive_own([at(0), at(4), at(8)], w);
            let a = whole_record_score(&del, own, w);
            let b = sparse_score(&del, total, own, w);
            assert!((a - b).abs() < 1e-9, "whole {a} sparse {b}");
        }
    }

    #[test]
    fn derive_own_unwinds_an_echo_that_lands_on_a_later_glint() {
        // Hand-built: own [1.0, 0.6, 1.0] behind a still wall at
        // delay 4 (steps 0, 0). Offset 4 reads own_1 + RHO * own_0;
        // offset 8 reads own_2 + RHO * own_1.
        let own = [1.0, 0.6, 1.0];
        let d = [own[0], own[1] + RHO * own[0], own[2] + RHO * own[1]];
        let back = derive_own(d, Wall { base: 4, s0: 0, s1: 0, rho: RHO });
        for j in 0..3 {
            assert!((back[j] - own[j]).abs() < 1e-12, "glint {j}");
        }
    }

    #[test]
    fn every_regime_wounds_naive_drift_and_curve_and_is_priced_by_vdrift_at_sigma_zero() {
        for regime in REGIMES {
            let a = audit(0.0, regime);
            assert!(a.naive.errors() >= 20, "regime {regime:?} naive {}", a.naive.errors());
            assert_eq!(a.drift_exact, 0, "regime {regime:?}");
            if regime == REGIMES[1] {
                // The wide down-then-up V hides from the linear
                // judge's verdicts: errors 0, and the modal
                // substitute is the still wall at its own base.
                assert_eq!(a.drift.errors(), 0, "regime {regime:?} drift {}", a.drift.errors());
                assert_eq!(a.drift_mode.0, (7, 0, 0), "regime {regime:?}");
            } else {
                assert!(a.drift.errors() >= 20, "regime {regime:?} drift {}", a.drift.errors());
            }
            assert!(a.curve.errors() >= 20, "regime {regime:?} curve {}", a.curve.errors());
            assert_eq!(a.curve_exact, 0, "regime {regime:?}");
            assert_eq!(a.vd.errors(), 0, "regime {regime:?}");
            assert_eq!(a.vd_exact, TRAINS_PER_KIND * 2, "regime {regime:?}");
            assert!(a.vd_rho_err_max <= 0.011, "regime {regime:?} rho err {}", a.vd_rho_err_max);
        }
    }

    #[test]
    fn the_vdrift_search_names_trajectory_and_strength_on_individual_trains_at_sigma_zero() {
        for regime in REGIMES {
            let book = build_book(0.0, Some(regime_wall(regime)));
            for t in &book {
                let (v, est, r_est) = judge_vdrift(&t.delivered).expect("vd judged");
                assert_eq!(v, t.kind == Kind::Moth, "regime {regime:?}");
                assert_eq!(est, regime, "regime {regime:?}");
                assert!((r_est - RHO).abs() <= 0.011, "regime {regime:?}");
            }
        }
    }

    #[test]
    fn the_book_is_deterministic() {
        let b1 = build_book(0.20, Some(regime_wall(REGIMES[2])));
        let b2 = build_book(0.20, Some(regime_wall(REGIMES[2])));
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
            for channel in 0..=1u64 {
                assert_eq!(glint_noise(id, 0, channel, 0.0), 0.0);
            }
            for slot in 0..3 {
                let n0 = glint_noise(id, slot, 0, 0.30);
                let n1 = glint_noise(id, slot, 1, 0.30);
                assert!(n0.abs() <= 0.30 && n1.abs() <= 0.30);
                assert_ne!(n0, n1);
            }
        }
    }
}
