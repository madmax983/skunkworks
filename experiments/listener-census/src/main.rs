//! # Listener Census 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, `experiments/listener-jammed`,
//! `experiments/listener-echoed`, `experiments/listener-adrift`,
//! `experiments/listener-drifting`, `experiments/listener-twinwalled`,
//! `experiments/listener-twindrift`, and
//! `experiments/listener-twincurve` — twincurve's own closing seed,
//! taken up verbatim: *the counting problem in a moving, bending
//! cave — how many walls, before pricing any.*
//!
//! Every Listener judge before this one was told, in its machinery
//! if not in its prose, how many walls the cave held: one, then
//! two. This cave does not say. Four caves are surveyed, their
//! walls curved trajectories exactly as in twincurve (base delay,
//! drift in {-1, 0, +1}, acceleration in {-1, 0, +1}; the step from
//! glint j to glint j+1 is drift + accel * j), strengths by
//! position 0.55 / 0.45 / 0.40 (the weakest wall's faintest
//! possible echo, 0.40 x 0.60, still clears the 0.22 ear — a
//! wall the medium swallows whole cannot be counted, only
//! mourned):
//!
//! - **one bending wall:** [(5, 0, +1)] — echoes at 5, 9, 14.
//! - **two walls, one bending:** [(4, +1, +1), (9, -1, 0)] —
//!   twincurve's curving + walking regime, echoes at 4, 9, 15 and
//!   9, 12, 15.
//! - **three walls, one bending:** [(4, +1, +1), (9, -1, 0),
//!   (6, 0, -1)] — the third wall's echoes land at 6, 10, 13.
//! - **three walls, all bending:** [(7, -1, -1), (5, -1, +1),
//!   (9, 0, +1)] — echoes at 7, 10, 12; 5, 8, 12; 9, 13, 18.
//!
//! The record window runs to offset 20 so every wall's full return
//! is delivered. Three judges read the SAME delivered record —
//! amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line.
//! - **Pair:** listener-twincurve's search — a joint search over
//!   an ordered pair of curved trajectories and two strengths on
//!   a 0.05 grid, whole-record score, exact abort bound. It
//!   assumes the cave holds exactly two walls, because its
//!   machinery has no way to hold any other number.
//! - **Census:** a counting search. It keeps a beam of forms:
//!   at each count up to three (the form has three lines), the
//!   BEAM best sets of that size, each expanded from the previous
//!   count's beam by every trajectory it does not already hold,
//!   with the set's strengths fitted jointly — coordinate descent
//!   over the same 0.05 grid, each wall's strength re-priced
//!   beside the others, because a wall fitted alone steals
//!   strength to cover echoes that are not its own. The count is
//!   where the buying stops: the smallest count whose successor
//!   buys less than ACCEPT_DELTA of whole-record score. The
//!   chosen form is refined — each wall re-read beside the
//!   others — while that helps, and the verdict reads the own
//!   glints the final form implies. The census is never told
//!   the true count, trajectories, or strengths.
//!
//! The perception rule is sacred: a judge reads only amplitudes
//! and arrival offsets the medium delivered. Whether a verdict —
//! or a count — was right is scored by the book, never a judge.
//!
//! Predictions declared before the first run:
//! - P1: the naive judge is wounded in every cave — at sigma 0.00
//!   its errors are at least 20 in each cave.
//! - P2: the pair judge is a two-wall judge — at sigma 0.00 it
//!   prices the two-wall cave exactly (errors 0, trajectory set
//!   exact on every train) and names the true trajectory set on
//!   zero trains in the one-wall and both three-wall caves (the
//!   true set is not in its search space: it has the wrong number
//!   of walls in it).
//! - P3: the census counts a clean cave exactly — at sigma 0.00
//!   its count is right on every train in every cave, its
//!   trajectory set is exact on every train, and its verdict
//!   errors are 0 in every cave.
//! - P4: at sigma 0.10 the census still counts: pooled over the
//!   four caves, its count is exact on at least 90% of trains.
//! - P5: pooled at sigma 0.10, the census judge's verdict errors
//!   are below the naive judge's.
//! - P6: at sigma 0.30, pooled, the census judge's verdict errors
//!   are still below the naive judge's pooled errors.
//!
//! `cargo run -p listener-census -- --headless` prints the
//! ledgers at sigma = 0.00, 0.10, 0.20, 0.30 for each cave.
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
/// Wall strengths by position in the cave.
const RHOS: [f64; 3] = [0.55, 0.45, 0.40];
/// Trains per kind per cave in the book.
const TRAINS_PER_KIND: usize = 120;
/// Widest offset any prediction or delivery can mention.
const MAX_OFFSET: usize = 32;
/// A further wall must buy at least this much whole-record score
/// before the census writes it on the form.
const ACCEPT_DELTA: f64 = 0.25;
/// The census form has three lines.
const MAX_WALLS: usize = 3;
/// Forms the census keeps in hand at each count.
const BEAM: usize = 6;

/// One wall as a curved trajectory: base delay, drift per glint,
/// acceleration (the drift's own drift), strength.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Wall {
    base: u64,
    drift: i64,
    accel: i64,
    rho: f64,
}

/// A trajectory without its strength: (base, drift, accel).
type Traj = (u64, i64, i64);

/// The delay a wall applies to glint `j`:
/// base + drift * j + accel * j * (j - 1) / 2.
fn wall_delay(w: Wall, j: usize) -> u64 {
    (w.base as i64 + w.drift * j as i64 + w.accel * (j as i64) * (j as i64 - 1) / 2) as u64
}

/// The four caves surveyed, as trajectory lists (strengths are
/// RHOS by position).
const CAVES: [&[Traj]; 4] = [
    &[(5, 0, 1)],
    &[(4, 1, 1), (9, -1, 0)],
    &[(4, 1, 1), (9, -1, 0), (6, 0, -1)],
    &[(7, -1, -1), (5, -1, 1), (9, 0, 1)],
];

const CAVE_NAMES: [&str; 4] = [
    "one bending wall",
    "two walls, one bending",
    "three walls, one bending",
    "three walls, all bending",
];

/// A cave as recording walls (trajectories with their strengths).
fn cave_walls(cave: &[Traj]) -> Vec<Wall> {
    cave.iter()
        .enumerate()
        .map(|(i, t)| Wall { base: t.0, drift: t.1, accel: t.2, rho: RHOS[i] })
        .collect()
}

/// The true trajectory set of a cave, sorted, for scoring.
fn cave_set(cave: &[Traj]) -> Vec<Traj> {
    let mut v = cave.to_vec();
    v.sort();
    v
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
/// contribution (0 = own glint, 1 + wall index = that wall's
/// echo), so every judge — and every rerun — hears the same
/// record. Channels 1 and 2 keep listener-twincurve's keying, so
/// the two-wall cave's book is the parent's own book; channel 3
/// extends the pattern.
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
    } else if channel == 3 {
        key ^= 0x3C3C_C3C3_5A5A_A5A5;
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
/// 0, 4, 8 from `strike`, and each glint's echo from each wall in
/// `walls`. Contributions that land on the same offset sum in the
/// ear before the ear's threshold is applied; the record is
/// delivered sorted by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, walls: &[Wall]) -> Train {
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
        for (wi, w) in walls.iter().enumerate() {
            add(
                GLINT_DT * j as u64 + wall_delay(*w, j),
                w.rho * own + glint_noise(id, j, (wi + 1) as u64, sigma),
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
/// judge in a given cave; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, walls: &[Wall]) -> Vec<Train> {
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
// The judges. Each reads only delivered amplitudes and offsets.
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

/// The sparse whole-record score with an abort bound (the
/// lineage's scoring, from listener-twincurve): the delivery's
/// total absolute mass, minus the delivered mass at the predicted
/// offsets, plus the absolute residuals there; abandons the
/// candidate (f64::INFINITY) as soon as the partial lower bound
/// exceeds `limit`. Exactness is preserved: a candidate is only
/// ever abandoned when its true score is provably above the
/// incumbent's.
fn sparse_score_abort(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own: [f64; 3],
    walls: &[Wall],
    limit: f64,
) -> f64 {
    let mut pred: [(u64, f64); 12] = [(0, 0.0); 12];
    let mut n = 0usize;
    let push = |offset: u64, amp: f64, pred: &mut [(u64, f64); 12], n: &mut usize| {
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

/// Every curved trajectory: base in 3..=9, drift in {-1, 0, +1},
/// accel in {-1, 0, +1}, with every per-glint delay inside
/// 1..=12 — sorted by (base, drift, accel), the canonical order.
fn trajectories() -> Vec<Traj> {
    let mut v = Vec::new();
    for base in 3..=9u64 {
        for drift in [-1i64, 0, 1] {
            for accel in [-1i64, 0, 1] {
                let w = Wall { base, drift, accel, rho: 0.0 };
                let delays = [wall_delay(w, 0), wall_delay(w, 1), wall_delay(w, 2)];
                if delays.iter().all(|d| (1..=12).contains(d)) {
                    v.push((base, drift, accel));
                }
            }
        }
    }
    v
}

/// listener-twincurve's pair judge, carried whole: a joint search
/// over an ordered pair of distinct curved trajectories and two
/// strengths on a 0.05 grid. Returns the verdict, the winning
/// trajectory pair, and the winning strengths.
fn judge_pair(
    delivered: &[Delivered],
    trajs: &[Traj],
) -> Option<(bool, (Traj, Traj), (f64, f64))> {
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
                    let walls = [
                        Wall { base: trajs[a].0, drift: trajs[a].1, accel: trajs[a].2, rho: r1 },
                        Wall { base: trajs[b].0, drift: trajs[b].1, accel: trajs[b].2, rho: r2 },
                    ];
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

/// Fit a trajectory set's strengths jointly: coordinate descent
/// over the 0.05 grid, each wall's strength re-priced beside the
/// others in turn, until a full pass changes nothing (at most
/// three passes). A strength is only ever priced beside the
/// walls it stands with — a wall fitted alone against a two-wall
/// record steals strength to cover echoes that are not its own
/// (in the two-wall cave, the near wall alone takes rho 0.95 to
/// cover the far wall's third echo, and the frozen theft then
/// blocks the true pair). Returns the fitted walls and their
/// whole-record score.
fn fit_strengths(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own_amps: [f64; 3],
    set: &[Traj],
    seed: &[Wall],
) -> (Vec<Wall>, f64) {
    let mut walls: Vec<Wall> = set
        .iter()
        .map(|t| {
            let rho = seed
                .iter()
                .find(|w| (w.base, w.drift, w.accel) == *t)
                .map_or(0.40, |w| w.rho);
            Wall { base: t.0, drift: t.1, accel: t.2, rho }
        })
        .collect();
    let score_of = |ws: &[Wall]| {
        let own = derive_own(own_amps, ws);
        sparse_score_abort(del, del_total, own, ws, f64::INFINITY)
    };
    let mut score = score_of(&walls);
    for _ in 0..3 {
        let mut changed = false;
        for idx in 0..walls.len() {
            for i in 0..=19u32 {
                let rho = i as f64 * 0.05;
                if (rho - walls[idx].rho).abs() < 1e-12 {
                    continue;
                }
                let mut trial = walls.clone();
                trial[idx].rho = rho;
                let s = score_of(&trial);
                if s < score - 1e-12 {
                    walls = trial;
                    score = s;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    (walls, score)
}

/// Search the single best trajectory to add to `fixed` (a
/// trajectory not already among them), with the enlarged set's
/// strengths fitted jointly. Returns the fitted set and score.
fn best_addition(
    del: &[f64; MAX_OFFSET],
    del_total: f64,
    own_amps: [f64; 3],
    fixed: &[Wall],
    trajs: &[Traj],
) -> Option<(Vec<Wall>, f64)> {
    let fixed_set: Vec<Traj> = fixed.iter().map(|w| (w.base, w.drift, w.accel)).collect();
    let mut best: Option<(Vec<Wall>, f64)> = None;
    for t in trajs {
        if fixed_set.contains(t) {
            continue;
        }
        let mut set = fixed_set.clone();
        set.push(*t);
        let (walls, s) = fit_strengths(del, del_total, own_amps, &set, fixed);
        if best.as_ref().map_or(true, |(_, bs)| s < *bs) {
            best = Some((walls, s));
        }
    }
    best
}

/// The census judge: count first, then price. It keeps a beam
/// of forms — at each count up to MAX_WALLS, the BEAM best sets
/// of that size, each expanded from the previous count's beam by
/// every trajectory it does not already hold, strengths fitted
/// jointly at every step. The count is where the buying stops:
/// the smallest count whose successor buys less than
/// ACCEPT_DELTA. The chosen form is then refined — each of its
/// walls re-read beside the others — while that helps. A single
/// greedy form is hostage to its first lines (in the three-wall
/// cave a mirage pair at score 0.78 blocks every single
/// addition, though the true three walls score 0); a beam is
/// not, and the refinement is the second reading. Returns the
/// verdict, the accepted trajectory set (sorted), the accepted
/// strengths (by trajectory), and the final score.
fn judge_census(
    delivered: &[Delivered],
    trajs: &[Traj],
) -> Option<(bool, Vec<Traj>, Vec<f64>, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let own_amps = [at(0)?, at(4)?, at(8)?];
    let del = delivered_array(delivered);
    let del_total: f64 = del.iter().map(|x| x.abs()).sum();
    let empty_own = derive_own(own_amps, &[]);
    let empty_score = sparse_score_abort(&del, del_total, empty_own, &[], f64::INFINITY);
    let mut beam: Vec<(Vec<Wall>, f64)> = vec![(Vec::new(), empty_score)];
    let mut best_by_count: Vec<(Vec<Wall>, f64)> = vec![(Vec::new(), empty_score)];
    for _count in 1..=MAX_WALLS {
        let mut expanded: Vec<(Vec<Wall>, f64)> = Vec::new();
        for (form, _) in &beam {
            let form_set: Vec<Traj> = form.iter().map(|w| (w.base, w.drift, w.accel)).collect();
            for t in trajs {
                if form_set.contains(t) {
                    continue;
                }
                let mut set = form_set.clone();
                set.push(*t);
                let (walls, s) = fit_strengths(&del, del_total, own_amps, &set, form);
                expanded.push((walls, s));
            }
        }
        expanded.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut kept: Vec<(Vec<Wall>, f64)> = Vec::new();
        for cand in expanded {
            let key: Vec<Traj> = {
                let mut v: Vec<Traj> = cand.0.iter().map(|w| (w.base, w.drift, w.accel)).collect();
                v.sort();
                v
            };
            let dup = kept.iter().any(|(w, _)| {
                let mut v: Vec<Traj> = w.iter().map(|x| (x.base, x.drift, x.accel)).collect();
                v.sort();
                v == key
            });
            if !dup {
                kept.push(cand);
            }
            if kept.len() >= BEAM {
                break;
            }
        }
        beam = kept;
        best_by_count.push(beam[0].clone());
    }
    let mut chosen = MAX_WALLS;
    for k in 0..MAX_WALLS {
        if best_by_count[k].1 - best_by_count[k + 1].1 < ACCEPT_DELTA {
            chosen = k;
            break;
        }
    }
    let (mut walls, mut score) = best_by_count[chosen].clone();
    // Refinement: re-read each line of the chosen form beside
    // the others, strengths refitted jointly, while it helps.
    for _ in 0..3 {
        let mut changed = false;
        for idx in 0..walls.len() {
            let rest: Vec<Wall> = walls
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, w)| *w)
                .collect();
            if let Some((next, s)) = best_addition(&del, del_total, own_amps, &rest, trajs) {
                if s < score - 1e-12 {
                    walls = next;
                    score = s;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let own = derive_own(own_amps, &walls);
    let mut pairs: Vec<(Traj, f64)> =
        walls.iter().map(|w| ((w.base, w.drift, w.accel), w.rho)).collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let (set, rhos): (Vec<Traj>, Vec<f64>) = pairs.into_iter().unzip();
    Some((spread(&own) > LINE, set, rhos, score))
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
    pair: JudgeLedger,
    pair_set_exact: usize,
    pair_judged: usize,
    census: JudgeLedger,
    census_count_exact: usize,
    census_set_exact: usize,
    census_judged: usize,
    census_count_hist: [usize; 4], // estimated counts 0..=3
    census_rho_err_max: f64,
    census_rho_err_mean: f64,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, cave: &[Traj], trajs: &[Traj]) -> Audit {
    let clean = build_book(sigma, &[]);
    let walls = cave_walls(cave);
    let truth_set = cave_set(cave);
    let book = build_book(sigma, &walls);
    let mut a = Audit {
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        pair: JudgeLedger::default(),
        pair_set_exact: 0,
        pair_judged: 0,
        census: JudgeLedger::default(),
        census_count_exact: 0,
        census_set_exact: 0,
        census_judged: 0,
        census_count_hist: [0; 4],
        census_rho_err_max: 0.0,
        census_rho_err_mean: 0.0,
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
        if let Some((v, pair_est, _)) = judge_pair(&train.delivered, trajs) {
            score(&mut a.pair, v, is_moth);
            a.pair_judged += 1;
            let mut set_est = vec![pair_est.0, pair_est.1];
            set_est.sort();
            if set_est == truth_set {
                a.pair_set_exact += 1;
            }
        }
        if let Some((v, set_est, rho_est, _)) = judge_census(&train.delivered, trajs) {
            score(&mut a.census, v, is_moth);
            a.census_judged += 1;
            a.census_count_hist[set_est.len().min(3)] += 1;
            if set_est.len() == truth_set.len() {
                a.census_count_exact += 1;
            }
            if set_est == truth_set {
                a.census_set_exact += 1;
            }
            for (t_est, r_est) in set_est.iter().zip(rho_est.iter()) {
                if let Some(pos) = cave.iter().position(|t| t == t_est) {
                    let e = (r_est - RHOS[pos]).abs();
                    rho_err_sum += e;
                    rho_n += 1;
                    if e > a.census_rho_err_max {
                        a.census_rho_err_max = e;
                    }
                }
            }
        }
    }
    if rho_n > 0 {
        a.census_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    let trajs = trajectories();
    println!("listener-census headless: one book of recorded trains per cave, the wall count unknown to every search judge, three judges, identical evidence");
    println!(
        "  book: {} trains per cave ({} moths, {} stones), own glints at offsets 0/4/8, strengths by position {:.2} / {:.2} / {:.2}, record window to offset 20, trajectory family {} curved trajectories",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        RHOS[0],
        RHOS[1],
        RHOS[2],
        trajs.len()
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), pair (listener-twincurve's joint curved pair, exactly two walls by construction), census (beam search over counts: {} forms kept in hand per count, strengths fitted jointly, count where the next wall buys < {:.2} of whole-record score, at most {} walls)", LINE, BEAM, ACCEPT_DELTA, MAX_WALLS);
    let sigmas = [0.00_f64, 0.10, 0.20, 0.30];
    let mut all: Vec<(f64, usize, Audit)> = Vec::new();
    for sigma in sigmas {
        for (ci, cave) in CAVES.iter().enumerate() {
            let a = audit(sigma, cave, &trajs);
            println!(
                "  sigma {:.2} cave {} {:?}: arrivals {}, clean naive errors {}, naive errors {} (wrong {}, fooled {}), pair errors {} (wrong {}, fooled {}), pair set exact {}/{}, census errors {} (wrong {}, fooled {}), census count exact {}/{}, census set exact {}/{}, census counts 0/1/2/3 = {}/{}/{}/{}, census rho err mean {:.3} max {:.3}",
                sigma,
                CAVE_NAMES[ci],
                cave,
                a.arrivals,
                a.clean_naive.errors(),
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.pair.errors(),
                a.pair.wrong_refusals,
                a.pair.fooled,
                a.pair_set_exact,
                a.pair_judged,
                a.census.errors(),
                a.census.wrong_refusals,
                a.census.fooled,
                a.census_count_exact,
                a.census_judged,
                a.census_set_exact,
                a.census_judged,
                a.census_count_hist[0],
                a.census_count_hist[1],
                a.census_count_hist[2],
                a.census_count_hist[3],
                a.census_rho_err_mean,
                a.census_rho_err_max
            );
            all.push((sigma, ci, a));
        }
    }
    let at = |s: f64, ci: usize| &all.iter().find(|(x, y, _)| *x == s && *y == ci).unwrap().2;
    let pooled = |s: f64, pick: fn(&Audit) -> usize| -> usize {
        (0..4).map(|ci| pick(at(s, ci))).sum()
    };
    println!("  declared predictions, judged on identical evidence:");
    println!(
        "    P1 the naive judge is wounded in every cave (sigma 0.00 naive errors >= 20 in each cave): {} / {} / {} / {} — {}",
        at(0.00, 0).naive.errors(),
        at(0.00, 1).naive.errors(),
        at(0.00, 2).naive.errors(),
        at(0.00, 3).naive.errors(),
        if (0..4).all(|ci| at(0.00, ci).naive.errors() >= 20) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the pair judge is a two-wall judge (sigma 0.00: two-wall cave errors 0 and set exact on every train; set exact 0 in the one-wall and both three-wall caves): cave2 errors {}, set exact {}/{}, others {}/{} + {}/{} + {}/{} — {}",
        at(0.00, 1).pair.errors(),
        at(0.00, 1).pair_set_exact,
        at(0.00, 1).pair_judged,
        at(0.00, 0).pair_set_exact,
        at(0.00, 0).pair_judged,
        at(0.00, 2).pair_set_exact,
        at(0.00, 2).pair_judged,
        at(0.00, 3).pair_set_exact,
        at(0.00, 3).pair_judged,
        if at(0.00, 1).pair.errors() == 0
            && at(0.00, 1).pair_set_exact == at(0.00, 1).pair_judged
            && at(0.00, 0).pair_set_exact == 0
            && at(0.00, 2).pair_set_exact == 0
            && at(0.00, 3).pair_set_exact == 0
        { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the census counts a clean cave exactly (sigma 0.00: count exact and set exact on every train, verdict errors 0, in every cave): count {}/{}/{}/{}, set {}/{}/{}/{}, errors {}/{}/{}/{} — {}",
        at(0.00, 0).census_count_exact,
        at(0.00, 1).census_count_exact,
        at(0.00, 2).census_count_exact,
        at(0.00, 3).census_count_exact,
        at(0.00, 0).census_set_exact,
        at(0.00, 1).census_set_exact,
        at(0.00, 2).census_set_exact,
        at(0.00, 3).census_set_exact,
        at(0.00, 0).census.errors(),
        at(0.00, 1).census.errors(),
        at(0.00, 2).census.errors(),
        at(0.00, 3).census.errors(),
        if (0..4).all(|ci| {
            let a = at(0.00, ci);
            a.census_count_exact == TRAINS_PER_KIND * 2
                && a.census_set_exact == TRAINS_PER_KIND * 2
                && a.census.errors() == 0
        }) { "HELD" } else { "CRACKED" }
    );
    let c_exact_10 = pooled(0.10, |a| a.census_count_exact);
    let c_judged_10 = pooled(0.10, |a| a.census_judged);
    println!(
        "    P4 at sigma 0.10 the census still counts (count exact on >= 90% of trains pooled): {}/{} — {}",
        c_exact_10,
        c_judged_10,
        if c_judged_10 > 0 && (c_exact_10 as f64) >= 0.90 * (c_judged_10 as f64) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 pooled at sigma 0.10 the census judge errs less than the naive judge: census {} vs naive {} — {}",
        pooled(0.10, |a| a.census.errors()),
        pooled(0.10, |a| a.naive.errors()),
        if pooled(0.10, |a| a.census.errors()) < pooled(0.10, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 pooled the census judge still errs less than the naive judge: census {} vs naive {} — {}",
        pooled(0.30, |a| a.census.errors()),
        pooled(0.30, |a| a.naive.errors()),
        if pooled(0.30, |a| a.census.errors()) < pooled(0.30, |a| a.naive.errors()) { "HELD" } else { "CRACKED" }
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
            let t = record_train(1, Kind::Moth, strike, 0.0, &[]);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) > LINE, "moth strike {strike}");
        }
    }

    #[test]
    fn clean_stone_trains_stay_under_the_line_on_three_own_glints() {
        for strike in 0..17u64 {
            let t = record_train(2, Kind::Stone, strike, 0.0, &[]);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) <= LINE, "stone strike {strike}");
        }
    }

    #[test]
    fn clean_unwalled_book_has_zero_errors_for_the_naive_judge() {
        let book = build_book(0.0, &[]);
        for t in &book {
            let v = judge_naive(&t.delivered).expect("clean train judged");
            assert_eq!(v, t.kind == Kind::Moth);
        }
    }

    #[test]
    fn the_one_wall_cave_delivers_three_echoes() {
        // Cave [(5, 0, +1)], stone struck at 3: delays 5/5/6,
        // echoes at 5/9/14; own at 0/4/8.
        let t = record_train(7, Kind::Stone, 3, 0.0, &cave_walls(CAVES[0]));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 5, 8, 9, 14]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(5) - RHOS[0] * own[0]).abs() < 1e-12);
        assert!((at(9) - RHOS[0] * own[1]).abs() < 1e-12);
        assert!((at(14) - RHOS[0] * own[2]).abs() < 1e-12);
    }

    #[test]
    fn the_three_wall_cave_sums_at_shared_offsets() {
        // Cave [(4,+1,+1), (9,-1,0), (6,0,-1)], stone struck at 3:
        // wall A echoes 4/9/15, wall B 9/12/15, wall C delays
        // 6/6/5, echoes 6/10/13.
        let t = record_train(7, Kind::Stone, 3, 0.0, &cave_walls(CAVES[2]));
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 6, 8, 9, 10, 12, 13, 15]);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let at = |o: u64| t.delivered.iter().find(|d| d.offset == o).unwrap().amp;
        assert!((at(9) - (RHOS[0] * own[1] + RHOS[1] * own[0])).abs() < 1e-12);
        assert!((at(6) - RHOS[2] * own[0]).abs() < 1e-12);
        assert!((at(15) - (RHOS[0] * own[2] + RHOS[1] * own[2])).abs() < 1e-12);
    }

    #[test]
    fn the_census_counts_and_names_every_clean_cave() {
        let trajs = trajectories();
        for cave in CAVES {
            let book = build_book(0.0, &cave_walls(cave));
            for t in &book {
                let (v, set_est, _, _) = judge_census(&t.delivered, &trajs).expect("census judged");
                assert_eq!(v, t.kind == Kind::Moth, "cave {cave:?}");
                assert_eq!(set_est, cave_set(cave), "cave {cave:?}");
            }
        }
    }

    #[test]
    fn the_pair_judge_names_the_two_wall_cave_and_no_other_at_sigma_zero() {
        let trajs = trajectories();
        for (ci, cave) in CAVES.iter().enumerate() {
            let book = build_book(0.0, &cave_walls(cave));
            for t in &book {
                let (_, pair_est, _) = judge_pair(&t.delivered, &trajs).expect("pair judged");
                let mut set_est = vec![pair_est.0, pair_est.1];
                set_est.sort();
                if ci == 1 {
                    assert_eq!(set_est, cave_set(cave), "two-wall cave");
                } else {
                    assert_ne!(set_est, cave_set(cave), "cave {ci}");
                }
            }
        }
    }

    #[test]
    fn every_cave_wounds_the_naive_judge_at_sigma_zero() {
        for cave in CAVES {
            let a = audit(0.0, cave, &trajectories());
            assert!(a.naive.errors() >= 20, "cave {cave:?} naive {}", a.naive.errors());
            assert_eq!(a.census.errors(), 0, "cave {cave:?}");
            assert_eq!(a.census_count_exact, TRAINS_PER_KIND * 2, "cave {cave:?}");
            assert_eq!(a.census_set_exact, TRAINS_PER_KIND * 2, "cave {cave:?}");
        }
    }

    #[test]
    fn the_book_is_deterministic() {
        let b1 = build_book(0.20, &cave_walls(CAVES[3]));
        let b2 = build_book(0.20, &cave_walls(CAVES[3]));
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
            for channel in 0..=3u64 {
                assert_eq!(glint_noise(id, 0, channel, 0.0), 0.0);
            }
            for slot in 0..3 {
                let n1 = glint_noise(id, slot, 1, 0.30);
                let n2 = glint_noise(id, slot, 2, 0.30);
                let n3 = glint_noise(id, slot, 3, 0.30);
                assert!(n1.abs() <= 0.30 && n2.abs() <= 0.30 && n3.abs() <= 0.30);
                assert_ne!(n1, n2);
                assert_ne!(n2, n3);
                assert_ne!(n1, n3);
            }
        }
    }
}
