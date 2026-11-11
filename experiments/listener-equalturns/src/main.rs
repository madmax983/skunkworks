//! # Listener Equalturns 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, `experiments/listener-adrift`,
//! `experiments/listener-drifting`, `experiments/listener-twinwalled`,
//! `experiments/listener-twindrift`, `experiments/listener-twincurve`,
//! `experiments/listener-census`, `experiments/listener-vdrift`,
//! `experiments/listener-vpair`, and `experiments/listener-equalturns`
//! — twinjoint's own closing seed, taken up verbatim: *equal strengths
//! on two turns (both walls at one rho): the gate charged to the
//! quieter wall should vanish — does the pair's naming then gate on
//! width alone, and does either wall's exact count still equal the
//! pair's?*
//!
//! The caves are twinjoint's caves, unchanged stone for stone: a wall
//! is a two-step trajectory (base, s0, s1); a V wall turns at the joint
//! (s1 = -s0); SHALLOW steps one tick, WIDE steps two. Three regimes,
//! each holding two turning walls:
//!
//! - **shallow + shallow, opposed:** A (6, +1, -1) beside B (9, -1, +1).
//! - **wide + shallow:** A (7, -2, +2) beside B (8, +1, -1).
//! - **shallow + wide:** A (5, +1, -1) beside B (9, -2, +2).
//!
//! The ONE change from twinjoint: both walls stand at the same
//! strength, rho = 0.45 — the on-grid strength nearest below the midpoint of
//! twinjoint's 0.55 / 0.40 pair (the judge names strengths on a
//! 0.05 grid, and 0.475 would sit half a step from every nameable
//! strength, counterfeiting a rho error in a clean cave). Total
//! loudness falls slightly (0.95 to 0.90), biasing every pooled
//! comparison against this planting; only the *inequality* is
//! removed at full strength. Twinjoint's headline was a gate:
//! at sigma 0.10 the quieter (0.40) wall's exact count EQUALLED the
//! pair-exact count in every regime severally and pooled (530 = 530) —
//! whenever the quieter wall was named, the pair was named. Equalise
//! the strengths and there is no quieter wall: if the gate was the
//! inequality, the identity must break — each wall should sometimes
//! be named alone. What remains to gate the pair, if anything, is
//! width: the shallow turn stands one tick from straight, from mirror,
//! and from counterfeit; the wide turn stands alone.
//!
//! Three judges read the SAME delivered record — amplitudes and
//! arrival offsets, nothing else: Naive (first three arrivals against
//! the 0.30 flutter line), Twincurve (listener-twincurve's exhaustive
//! unordered curved-pair search, carried whole from twinjoint), and
//! the twinjoint judge itself (unordered jointed pair; coarse-lattice
//! (0.2) scoring of every pair, full-grid refit of the best 64),
//! carried whole — same search, same staging, same tests. Only the
//! cave's strengths changed, so every difference in the ledger is
//! charged to the equalising, not to a new ear.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict
//! was right is scored by the book, never a judge.
//!
//! Predictions declared before the shipped run:
//! - P1: the walled caves wound the naive judge in every regime —
//!   at sigma 0.00 its errors are at least 20 in each regime.
//! - P2: the twincurve search names the true pair on zero trains
//!   in every regime at sigma 0.00 (neither V is in its family).
//! - P3: the echo-swap shelter does not need unequal strengths —
//!   in the shallow + shallow, opposed cave at sigma 0.00 the
//!   twincurve judge still errs fewer than 20 times while naming
//!   the true pair on zero trains.
//! - P4: the twinjoint search prices the pair in a clean cave —
//!   at sigma 0.00 its errors are 0 in every regime, its pair
//!   estimate is exact on every train, and its rho error max is
//!   <= 0.011 in every regime.
//! - P5: pooled over the three regimes at sigma 0.10, the
//!   twinjoint judge's errors are below the naive judge's.
//! - P6: THE GATE IDENTITY BREAKS — pooled at sigma 0.10, NEITHER
//!   wall's exact count equals the pair-exact count; each wall is
//!   named alone on at least one train (A-only >= 1 and B-only >= 1).
//! - P7: the rent now gates on width alone — pooled at sigma 0.30,
//!   the shallow walls' exact rate is below the wide walls'.
//! - P8: at sigma 0.30, pooled, the twinjoint judge's errors are
//!   still below the naive judge's pooled errors.
//! - P9: raising the quieter wall lifts the pair — pooled pair
//!   exact at sigma 0.10 exceeds twinjoint's unequal-strength
//!   530/720, even though the louder wall fell from 0.55 to 0.45 and total loudness fell with it.
//!
//! `cargo run -p listener-equalturns -- --headless` prints the
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30 for each regime,
//! plus, at sigma 0.00, the pair the twincurve judge most often
//! names in place of the truth. There is no TUI; --headless is
//! handled before anything else regardless.

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
/// Both walls' shared strength — the on-grid strength nearest
/// below the midpoint of twinjoint's 0.55 / 0.40 pair (0.475
/// would sit half a grid step from every nameable strength).
/// Total loudness falls slightly, biasing pooled comparisons
/// against this planting; only the inequality is removed.
const RHO_A: f64 = 0.45;
/// The second wall's strength: equal to the first, by design.
const RHO_B: f64 = 0.45;
/// Trains per kind per regime in the book.
const TRAINS_PER_KIND: usize = 120;
/// Widest offset any prediction or delivery can mention.
const MAX_OFFSET: usize = 32;

/// A trajectory: base delay and the two per-gap steps.
type Traj = (u64, i64, i64);

/// One wall as a two-step trajectory: base, step 0, step 1, strength.
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

/// The three regimes surveyed, as (first wall, second wall)
/// trajectories without strength (strengths are RHO_A / RHO_B
/// by position). Both walls turn in every regime.
const REGIMES: [(Traj, Traj); 3] = [
    ((6, 1, -1), (9, -1, 1)), // shallow + shallow, opposed
    ((7, -2, 2), (8, 1, -1)), // wide + shallow
    ((5, 1, -1), (9, -2, 2)), // shallow + wide
];

/// Whether a trajectory is a WIDE V (|s0| = 2, turning).
fn is_wide(t: Traj) -> bool {
    t.1.abs() == 2 && t.2 == -t.1
}

/// Whether a trajectory is a SHALLOW V (|s0| = 1, turning).
fn is_shallow(t: Traj) -> bool {
    t.1.abs() == 1 && t.2 == -t.1
}

/// A regime as recording walls (trajectories with their strengths).
fn regime_walls(regime: (Traj, Traj)) -> [Wall; 2] {
    [
        Wall { base: regime.0 .0, s0: regime.0 .1, s1: regime.0 .2, rho: RHO_A },
        Wall { base: regime.1 .0, s0: regime.1 .1, s1: regime.1 .2, rho: RHO_B },
    ]
}

/// Canonical (sorted) trajectory pair of a regime, for scoring an
/// unordered judge's estimate.
fn regime_pair(regime: (Traj, Traj)) -> (Traj, Traj) {
    let a = regime.0;
    let b = regime.1;
    if a <= b { (a, b) } else { (b, a) }
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
/// 1 = the V wall's echo, 2 = the second wall's echo), so every
/// judge — and every rerun — hears the same record.
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
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, walls: Option<[Wall; 2]>) -> Train {
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

/// Derive the own glints a candidate pair of walls implies: walk
/// the own offsets in strike order and subtract, from each, every
/// earlier glint's echo — from either candidate wall — that lands
/// exactly on it.
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
/// difference over every offset either side mentions.
#[allow(dead_code)] // dense reference score; exercised by tests
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
    walls: &[Wall],
    limit: f64,
) -> f64 {
    let mut pred: [(u64, f64); 9] = [(0, 0.0); 9];
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
        for w in walls {
            push(GLINT_DT * j as u64 + wall_delay(*w, j), w.rho * own[j]);
        }
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
fn sparse_score(del: &[f64; MAX_OFFSET], del_total: f64, own: [f64; 3], walls: &[Wall]) -> f64 {
    sparse_score_abort(del, del_total, own, walls, f64::INFINITY)
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
/// (Exercised by the family-nesting test; no judge here walks.)
#[allow(dead_code)]
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

/// A wall from a trajectory and a strength.
fn wall_of(t: Traj, rho: f64) -> Wall {
    Wall { base: t.0, s0: t.1, s1: t.2, rho }
}

/// Score one candidate pair at one strength pair.
fn score_pair(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own_amps: [f64; 3],
    pair: (Traj, Traj),
    rhos: (f64, f64),
    limit: f64,
) -> f64 {
    let walls = [wall_of(pair.0, rhos.0), wall_of(pair.1, rhos.1)];
    let own = derive_own(own_amps, &walls);
    sparse_score_abort(del, del_total, own, &walls, limit)
}

/// The twincurve judge: an unordered pair search over CURVED
/// trajectories and two strengths on a 0.05 grid, exhaustive —
/// listener-twincurve's search carried whole into the step
/// representation (a curved trajectory (base, d, a) there is
/// (base, d, d + a) here). Returns the verdict, the winning pair
/// in canonical order, and the winning strengths by pair position.
fn judge_twincurve(delivered: &[Delivered]) -> Option<(bool, (Traj, Traj), (f64, f64))> {
    let trajs = curved_trajectories();
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let mut best: Option<(f64, (Traj, Traj), (f64, f64), [f64; 3])> = None;
    for a in 0..trajs.len() {
        for b in (a + 1)..trajs.len() {
            for i in 0..=19u32 {
                let r1 = i as f64 * 0.05;
                for k in 0..=19u32 {
                    let r2 = k as f64 * 0.05;
                    let walls = [wall_of(trajs[a], r1), wall_of(trajs[b], r2)];
                    let own = derive_own(own_amps, &walls);
                    let limit = best.map_or(f64::INFINITY, |(s, _, _, _)| s);
                    let score = sparse_score_abort(&del, del_total, own, &walls, limit);
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

/// Fit one ordered pair's strengths by coordinate descent over
/// the 0.05 grid: two rounds of (scan the first strength holding
/// the second, then the second holding the first), starting from
/// (0, 0). Returns (score, rho_a, rho_b).
#[allow(dead_code)] // the two-round reference fit; the staged judge uses round counts directly
fn cd_fit(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own_amps: [f64; 3],
    pair: (Traj, Traj),
) -> (f64, f64, f64) {
    cd_fit_rounds(del, del_total, own_amps, pair, 2)
}

/// Coordinate descent with a chosen number of rounds.
#[allow(dead_code)] // reference fit; the staged judge ranks by lattice (see judge_twinjoint)
fn cd_fit_rounds(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own_amps: [f64; 3],
    pair: (Traj, Traj),
    rounds: usize,
) -> (f64, f64, f64) {
    let (mut ra, mut rb) = (0.0f64, 0.0f64);
    let mut best = score_pair(del, del_total, own_amps, pair, (ra, rb), f64::INFINITY);
    for _ in 0..rounds {
        for i in 0..=19u32 {
            let r = i as f64 * 0.05;
            let s = score_pair(del, del_total, own_amps, pair, (r, rb), best);
            if s < best {
                best = s;
                ra = r;
            }
        }
        for k in 0..=19u32 {
            let r = k as f64 * 0.05;
            let s = score_pair(del, del_total, own_amps, pair, (ra, r), best);
            if s < best {
                best = s;
                rb = r;
            }
        }
    }
    (best, ra, rb)
}

/// The twinjoint judge: an unordered pair search over JOINTED
/// trajectories, the two trajectories distinct, in canonical
/// order. Staged fitting: every unordered pair is fitted by ONE
/// scored on a coarse strength lattice (0.2 steps); the best 64
/// pairs are then re-fitted on the full grid and the cheapest
/// wins. (Earlier drafts ranked and re-fitted by coordinate
/// descent, as listener-vpair's judge does; the garden's own
/// staged-vs-exhaustive test caught a sampled train where the
/// exhaustive winner ranked ~175th by one-round descent and a
/// descent refit still stranded it at a local strength minimum —
/// with two turning walls the strengths interact, and only a
/// path-free ranking plus a full-grid refit is honest.) Returns the verdict, the winning pair in
/// canonical order, and the winning strengths by pair position.
fn judge_twinjoint(delivered: &[Delivered]) -> Option<(bool, (Traj, Traj), (f64, f64))> {
    let jt = jointed_trajectories();
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let mut cand: Vec<(f64, Traj, Traj)> = Vec::new();
    for a in 0..jt.len() {
        for b in (a + 1)..jt.len() {
            // Stage one ranks by a coarse strength LATTICE, not by
            // descent: with two turning walls the strengths interact
            // and a descent's path strands good pairs at bad scores
            // (the garden's own test found the exhaustive winner
            // ranked ~175th by one-round descent on a sampled train).
            // The lattice is path-free: every pair is scored at the
            // same 36 strength points and ranked by its best.
            let mut sc = f64::INFINITY;
            for i in 0..=5u32 {
                for k in 0..=5u32 {
                    let r = score_pair(
                        &del,
                        del_total,
                        own_amps,
                        (jt[a], jt[b]),
                        (i as f64 * 0.2, k as f64 * 0.2),
                        sc,
                    );
                    if r < sc {
                        sc = r;
                    }
                }
            }
            cand.push((sc, jt[a], jt[b]));
        }
    }
    cand.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap_or(std::cmp::Ordering::Equal));
    cand.truncate(64);
    let mut best: Option<(f64, (Traj, Traj), (f64, f64), [f64; 3])> = None;
    for &(_, ta, tb) in &cand {
        for i in 0..=19u32 {
            let r1 = i as f64 * 0.05;
            for k in 0..=19u32 {
                let r2 = k as f64 * 0.05;
                let walls = [wall_of(ta, r1), wall_of(tb, r2)];
                let own = derive_own(own_amps, &walls);
                let limit = best.map_or(f64::INFINITY, |(sc, _, _, _)| sc);
                let score = sparse_score_abort(&del, del_total, own, &walls, limit);
                if best.map_or(true, |(sc, _, _, _)| score < sc) {
                    best = Some((score, (ta, tb), (r1, r2), own));
                }
            }
        }
    }
    let (_, pair, rhos, own) = best?;
    Some((spread(&own) > LINE, pair, rhos))
}

/// The exhaustive unordered jointed x jointed full-grid search —
/// the reference the staged twinjoint search is checked against
/// in tests, on sampled trains only.
#[allow(dead_code)] // exercised by tests
fn judge_twinjoint_full(delivered: &[Delivered]) -> Option<(bool, (Traj, Traj), (f64, f64))> {
    let jt = jointed_trajectories();
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let mut best: Option<(f64, (Traj, Traj), (f64, f64), [f64; 3])> = None;
    for a in 0..jt.len() {
        for b in (a + 1)..jt.len() {
            let (ta, tb) = (jt[a], jt[b]);
            for i in 0..=19u32 {
                let r1 = i as f64 * 0.05;
                for k in 0..=19u32 {
                    let r2 = k as f64 * 0.05;
                    let walls = [wall_of(ta, r1), wall_of(tb, r2)];
                    let own = derive_own(own_amps, &walls);
                    let limit = best.map_or(f64::INFINITY, |(sc, _, _, _)| sc);
                    let score = sparse_score_abort(&del, del_total, own, &walls, limit);
                    if best.map_or(true, |(sc, _, _, _)| score < sc) {
                        best = Some((score, (ta, tb), (r1, r2), own));
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
    tc: JudgeLedger,
    tc_exact: usize,
    tj: JudgeLedger,
    tj_exact: usize,
    tj_judged: usize,
    tj_a_exact: usize,
    tj_b_exact: usize,
    tj_a_only: usize,
    tj_b_only: usize,
    tj_shallow_exact: usize,
    tj_shallow_total: usize,
    tj_wide_exact: usize,
    tj_wide_total: usize,
    tj_rho_err_max: f64,
    tj_rho_err_mean: f64,
    tc_mode: ((Traj, Traj), usize),
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn mode_of(counts: &BTreeMap<(Traj, Traj), usize>) -> ((Traj, Traj), usize) {
    counts
        .iter()
        .max_by_key(|(_, c)| *c)
        .map(|(t, c)| (*t, *c))
        .unwrap_or((((0, 0, 0), (0, 0, 0)), 0))
}

fn audit(sigma: f64, regime: (Traj, Traj)) -> Audit {
    let clean = build_book(sigma, None);
    let walls = regime_walls(regime);
    let truth_pair = regime_pair(regime);
    let book = build_book(sigma, Some(walls));
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        tc: JudgeLedger::default(),
        tc_exact: 0,
        tj: JudgeLedger::default(),
        tj_exact: 0,
        tj_judged: 0,
        tj_a_exact: 0,
        tj_b_exact: 0,
        tj_a_only: 0,
        tj_b_only: 0,
        tj_shallow_exact: 0,
        tj_shallow_total: 0,
        tj_wide_exact: 0,
        tj_wide_total: 0,
        tj_rho_err_max: 0.0,
        tj_rho_err_mean: 0.0,
        tc_mode: (((0, 0, 0), (0, 0, 0)), 0),
    };
    for train in &clean {
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.clean_naive, v, is_moth);
        }
    }
    let mut rho_err_sum = 0.0;
    let mut rho_n = 0usize;
    let mut tc_counts: BTreeMap<(Traj, Traj), usize> = BTreeMap::new();
    for train in &book {
        a.arrivals += train.delivered.len();
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.naive, v, is_moth);
        }
        if let Some((v, pair_est, _)) = judge_twincurve(&train.delivered) {
            score(&mut a.tc, v, is_moth);
            if pair_est == truth_pair {
                a.tc_exact += 1;
            }
            *tc_counts.entry(pair_est).or_insert(0) += 1;
        }
        if let Some((v, pair_est, rho_est)) = judge_twinjoint(&train.delivered) {
            score(&mut a.tj, v, is_moth);
            a.tj_judged += 1;
            if pair_est == truth_pair {
                a.tj_exact += 1;
            }
            // Per-wall naming is set membership: the pair is
            // unordered, a wall is named if it stands in the pair.
            let named = [pair_est.0, pair_est.1];
            if named.contains(&regime.0) {
                a.tj_a_exact += 1;
            }
            if named.contains(&regime.1) {
                a.tj_b_exact += 1;
            }
            if named.contains(&regime.0) && !named.contains(&regime.1) {
                a.tj_a_only += 1;
            }
            if named.contains(&regime.1) && !named.contains(&regime.0) {
                a.tj_b_only += 1;
            }
            for (t_true, _) in [(regime.0, RHO_A), (regime.1, RHO_B)] {
                let hit = named.contains(&t_true);
                if is_wide(t_true) {
                    a.tj_wide_total += 1;
                    if hit {
                        a.tj_wide_exact += 1;
                    }
                } else if is_shallow(t_true) {
                    a.tj_shallow_total += 1;
                    if hit {
                        a.tj_shallow_exact += 1;
                    }
                }
            }
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
                if e > a.tj_rho_err_max {
                    a.tj_rho_err_max = e;
                }
            }
        }
    }
    if rho_n > 0 {
        a.tj_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a.tc_mode = mode_of(&tc_counts);
    a
}

fn run_headless() {
    println!("listener-equalturns headless: twinjoint's caves with both walls at one strength (rho 0.45), three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, regimes {:?}, strengths {:.3} (first) / {:.3} (second) — EQUAL, record window to offset 20",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        REGIMES,
        RHO_A,
        RHO_B
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), twincurve (unordered curved pair x two rhos on 0.05 grid, exhaustive), twinjoint (unordered jointed pair; coarse-lattice (0.2) scoring of every pair, full-grid refit of the best 64)", LINE);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, (Traj, Traj), Audit)> = Vec::new();
    for sigma in sigmas {
        for regime in REGIMES {
            let a = audit(sigma, regime);
            println!(
                "  sigma {:.2} regime A ({}, {:+}, {:+}) + B ({}, {:+}, {:+}): arrivals {}, clean naive errors {} (wrong {}, fooled {}), naive errors {} (wrong {}, fooled {}), twincurve errors {} (wrong {}, fooled {}), twincurve pair exact {}, twinjoint errors {} (wrong {}, fooled {}), twinjoint pair exact {}/{}, twinjoint A-wall exact {}, twinjoint B-wall exact {}, A-only {}, B-only {}, shallow exact {}/{}, wide exact {}/{}, twinjoint rho err mean {:.3} max {:.3}",
                sigma,
                regime.0 .0,
                regime.0 .1,
                regime.0 .2,
                regime.1 .0,
                regime.1 .1,
                regime.1 .2,
                a.arrivals,
                a.clean_naive.errors(),
                a.clean_naive.wrong_refusals,
                a.clean_naive.fooled,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.tc.errors(),
                a.tc.wrong_refusals,
                a.tc.fooled,
                a.tc_exact,
                a.tj.errors(),
                a.tj.wrong_refusals,
                a.tj.fooled,
                a.tj_exact,
                a.tj_judged,
                a.tj_a_exact,
                a.tj_b_exact,
                a.tj_a_only,
                a.tj_b_only,
                a.tj_shallow_exact,
                a.tj_shallow_total,
                a.tj_wide_exact,
                a.tj_wide_total,
                a.tj_rho_err_mean,
                a.tj_rho_err_max
            );
            if sigma == 0.0 {
                println!(
                    "    sigma 0.00 substitutes for regime A ({}, {:+}, {:+}) + B ({}, {:+}, {:+}): twincurve most often names (({}, {:+}, {:+}), ({}, {:+}, {:+})) on {} trains",
                    regime.0 .0,
                    regime.0 .1,
                    regime.0 .2,
                    regime.1 .0,
                    regime.1 .1,
                    regime.1 .2,
                    a.tc_mode.0 .0 .0,
                    a.tc_mode.0 .0 .1,
                    a.tc_mode.0 .0 .2,
                    a.tc_mode.0 .1 .0,
                    a.tc_mode.0 .1 .1,
                    a.tc_mode.0 .1 .2,
                    a.tc_mode.1
                );
            }
            all.push((sigma, regime, a));
        }
    }
    let at = |s: f64, r: (Traj, Traj)| &all.iter().find(|(x, y, _)| *x == s && *y == r).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        REGIMES.iter().map(|r| pick(at(s, *r))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the walled caves wound the naive judge in every regime (sigma 0.00 naive errors >= 20 in each regime): {} / {} / {} — {}",
        at(0.00, REGIMES[0]).naive.errors(),
        at(0.00, REGIMES[1]).naive.errors(),
        at(0.00, REGIMES[2]).naive.errors(),
        if REGIMES.iter().all(|r| at(0.00, *r).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the twincurve search names the true pair on zero trains in every regime at sigma 0.00: exact {} / {} / {} — {}",
        at(0.00, REGIMES[0]).tc_exact,
        at(0.00, REGIMES[1]).tc_exact,
        at(0.00, REGIMES[2]).tc_exact,
        if REGIMES.iter().all(|r| at(0.00, *r).tc_exact == 0) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the echo-swap shelter does not need unequal strengths (sigma 0.00, shallow+shallow opposed: twincurve errors < 20 while its pair exact is 0): errors {}, exact {} — {}",
        at(0.00, REGIMES[0]).tc.errors(),
        at(0.00, REGIMES[0]).tc_exact,
        if at(0.00, REGIMES[0]).tc.errors() < 20 && at(0.00, REGIMES[0]).tc_exact == 0 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the twinjoint search prices the pair in a clean cave (sigma 0.00: errors 0 in every regime, pair exact on every train, rho err max <= 0.011): errors {}/{}/{}, pair exact {}/{}/{}, rho err max {:.3}/{:.3}/{:.3} — {}",
        at(0.00, REGIMES[0]).tj.errors(),
        at(0.00, REGIMES[1]).tj.errors(),
        at(0.00, REGIMES[2]).tj.errors(),
        at(0.00, REGIMES[0]).tj_exact,
        at(0.00, REGIMES[1]).tj_exact,
        at(0.00, REGIMES[2]).tj_exact,
        at(0.00, REGIMES[0]).tj_rho_err_max,
        at(0.00, REGIMES[1]).tj_rho_err_max,
        at(0.00, REGIMES[2]).tj_rho_err_max,
        if REGIMES.iter().all(|r| {
            let a = at(0.00, *r);
            a.tj.errors() == 0
                && a.tj_exact == a.tj_judged
                && a.tj_judged == TRAINS_PER_KIND * 2
                && a.tj_rho_err_max <= 0.011
        }) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 pooled at sigma 0.10 the twinjoint judge errs less than the naive judge: twinjoint {} vs naive {} — {}",
        pooled(0.10, |a| a.tj.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.tj.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    let pair10 = pooled(0.10, |a| a.tj_exact);
    let a10 = pooled(0.10, |a| a.tj_a_exact);
    let b10 = pooled(0.10, |a| a.tj_b_exact);
    let aonly10 = pooled(0.10, |a| a.tj_a_only);
    let bonly10 = pooled(0.10, |a| a.tj_b_only);
    println!(
        "    P6 THE GATE IDENTITY BREAKS (sigma 0.10 pooled: pair exact {}, A exact {}, B exact {}, A-only {}, B-only {}; held only if neither wall equals the pair and both solos >= 1) — {}",
        pair10, a10, b10, aonly10, bonly10,
        if a10 != pair10 && b10 != pair10 && aonly10 >= 1 && bonly10 >= 1 { "HELD" } else { "CRACKED" }
    );
    let sh30: usize = REGIMES.iter().map(|r| at(0.30, *r).tj_shallow_exact).sum();
    let sht30: usize = REGIMES.iter().map(|r| at(0.30, *r).tj_shallow_total).sum();
    let wi30: usize = REGIMES.iter().map(|r| at(0.30, *r).tj_wide_exact).sum();
    let wit30: usize = REGIMES.iter().map(|r| at(0.30, *r).tj_wide_total).sum();
    println!(
        "    P7 the rent gates on width alone (sigma 0.30 pooled: shallow exact rate {}/{} = {:.3} < wide exact rate {}/{} = {:.3}) — {}",
        sh30,
        sht30,
        if sht30 > 0 { sh30 as f64 / sht30 as f64 } else { 0.0 },
        wi30,
        wit30,
        if wit30 > 0 { wi30 as f64 / wit30 as f64 } else { 0.0 },
        if sht30 > 0 && wit30 > 0 && (sh30 as f64 / sht30 as f64) < (wi30 as f64 / wit30 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P8 at sigma 0.30 pooled the twinjoint judge still errs less than the naive judge: twinjoint {} vs naive {} — {}",
        pooled(0.30, |a| a.tj.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.tj.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P9 raising the quieter wall lifts the pair (sigma 0.10 pooled pair exact {} > twinjoint unequal 530) — {}",
        pair10,
        if pair10 > 530 { "HELD" } else { "CRACKED" }
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
    fn regime_walls_both_turn_as_declared() {
        let delays = |t: Traj| {
            let w = wall_of(t, 0.0);
            [wall_delay(w, 0), wall_delay(w, 1), wall_delay(w, 2)]
        };
        // Both walls turn in every regime: second step negates first.
        for r in REGIMES {
            for w in [r.0, r.1] {
                assert_eq!(w.2, -w.1, "wall {w:?} turns");
                assert_ne!(w.1, 0);
                assert!(is_wide(w) || is_shallow(w), "wall {w:?} is a V");
            }
        }
        assert_eq!(delays(REGIMES[0].0), [6, 7, 6]);
        assert_eq!(delays(REGIMES[0].1), [9, 8, 9]);
        assert_eq!(delays(REGIMES[1].0), [7, 5, 7]);
        assert_eq!(delays(REGIMES[1].1), [8, 9, 8]);
        assert_eq!(delays(REGIMES[2].0), [5, 6, 5]);
        assert_eq!(delays(REGIMES[2].1), [9, 7, 9]);
        // Both walls stand at ONE strength: the inequality is gone.
        assert_eq!(RHO_A, RHO_B);
        assert!((RHO_A - 0.45).abs() < 1e-12);
        for r in REGIMES {
            let ws = regime_walls(r);
            assert_eq!(ws[0].rho, ws[1].rho);
        }
        // Width is balanced across positions: the mixed caves put
        // each width in each position exactly once.
        assert!(is_shallow(REGIMES[0].0) && is_shallow(REGIMES[0].1));
        assert!(is_wide(REGIMES[1].0) && is_shallow(REGIMES[1].1));
        assert!(is_shallow(REGIMES[2].0) && is_wide(REGIMES[2].1));
    }

    #[test]
    fn no_regime_wall_is_in_the_curved_family_but_both_are_in_the_twinjoint_space() {
        let cur = curved_trajectories();
        let joi = jointed_trajectories();
        let lin = linear_trajectories();
        for r in REGIMES {
            for w in [r.0, r.1] {
                assert!(!cur.contains(&w), "V wall {w:?} in curved family");
                assert!(joi.contains(&w), "V wall {w:?} not in jointed family");
            }
        }
        // The families nest on steps: linear ⊂ curved ⊂ jointed.
        for t in &lin {
            assert!(cur.contains(t), "linear {t:?} not in curved");
            assert!(joi.contains(t), "linear {t:?} not in jointed");
        }
        for t in &cur {
            assert!(joi.contains(t), "curved {t:?} not in jointed");
        }
    }

    #[test]
    fn regime_echoes_land_where_declared() {
        // Regime 1, stone struck at 3: own at 0/4/8; A echoes at
        // 6/11/14; B echoes at 9/12/17 — no coincidences.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_walls(REGIMES[0])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 6, 8, 9, 11, 12, 14, 17]);
        // Regime 2, stone at 3: A echoes at 7/9/15; B echoes at
        // 8/13/16 — offset 8 rides the third own glint.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_walls(REGIMES[1])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 7, 8, 9, 13, 15, 16]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(8) - (own[2] + RHO_B * own[0])).abs() < 1e-12);
        assert!((at(7) - RHO_A * own[0]).abs() < 1e-12);
        // Regime 3, stone at 3: A echoes at 5/10/13; B echoes at
        // 9/11/17.
        let t = record_train(7, Kind::Stone, 3, 0.0, Some(regime_walls(REGIMES[2])));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 5, 8, 9, 10, 11, 13, 17]);
    }

    #[test]
    fn sparse_score_equals_whole_record_score() {
        let book = build_book(0.20, Some(regime_walls(REGIMES[1])));
        let walls = [wall_of((5, -1, 2), 0.35), wall_of((8, 0, 0), 0.50)];
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
    fn derive_own_unwinds_echoes_from_two_walls_on_later_glints() {
        // Hand-built: own [1.0, 0.6, 1.0]; wall A still at delay 4
        // (steps 0, 0) at RHO_A; wall B still at delay 8 at RHO_B.
        // Offset 4 reads own_1 + RHO_A * own_0; offset 8 reads
        // own_2 + RHO_A * own_1 + RHO_B * own_0.
        let own = [1.0, 0.6, 1.0];
        let d = [
            own[0],
            own[1] + RHO_A * own[0],
            own[2] + RHO_A * own[1] + RHO_B * own[0],
        ];
        let back = derive_own(d, &[wall_of((4, 0, 0), RHO_A), wall_of((8, 0, 0), RHO_B)]);
        for j in 0..3 {
            assert!((back[j] - own[j]).abs() < 1e-12, "glint {j}");
        }
    }

    #[test]
    fn the_staged_twinjoint_search_matches_the_exhaustive_search_on_sampled_trains() {
        for regime in REGIMES {
            let book = build_book(0.20, Some(regime_walls(regime)));
            for t in book.iter().take(4) {
                let fast = judge_twinjoint(&t.delivered).expect("twinjoint judged");
                let full = judge_twinjoint_full(&t.delivered).expect("full judged");
                assert_eq!(fast.0, full.0, "verdict differs, regime {regime:?}");
                assert_eq!(fast.1, full.1, "pair differs, regime {regime:?}");
                assert_eq!(fast.2, full.2, "rhos differ, regime {regime:?}");
            }
        }
    }

    #[test]
    fn every_regime_wounds_naive_and_is_priced_by_twinjoint_at_sigma_zero() {
        for regime in REGIMES {
            let a = audit(0.0, regime);
            assert!(a.naive.errors() >= 20, "regime {regime:?} naive {}", a.naive.errors());
            assert_eq!(a.tc_exact, 0, "regime {regime:?}");
            assert_eq!(a.tj.errors(), 0, "regime {regime:?}");
            assert_eq!(a.tj_exact, TRAINS_PER_KIND * 2, "regime {regime:?}");
            assert!(a.tj_rho_err_max <= 0.011, "regime {regime:?} rho err {}", a.tj_rho_err_max);
        }
    }

    #[test]
    fn the_book_is_deterministic() {
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
    fn noise_is_zero_at_sigma_zero_and_bounded_elsewhere() {
        for id in 1..50u64 {
            for channel in 0..=2u64 {
                assert_eq!(glint_noise(id, 0, channel, 0.0), 0.0);
            }
            for slot in 0..3 {
                let n0 = glint_noise(id, slot, 0, 0.30);
                let n1 = glint_noise(id, slot, 1, 0.30);
                let n2 = glint_noise(id, slot, 2, 0.30);
                assert!(n0.abs() <= 0.30 && n1.abs() <= 0.30 && n2.abs() <= 0.30);
                assert_ne!(n0, n1);
                assert_ne!(n0, n2);
                assert_ne!(n1, n2);
            }
        }
    }
}
