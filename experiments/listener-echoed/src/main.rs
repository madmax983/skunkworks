//! # Listener Echoed 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`,
//! `experiments/listener-paired`, and `experiments/listener-jammed` —
//! listener-jammed's own closing seed, taken up as the subject:
//! *foreign glints arriving ON the own offsets, where timing cannot
//! separate them — what channel prices those?*
//!
//! In listener-jammed the foreign reflector's glints landed between
//! the train's own (offsets 2, 6, 10 against own 0, 4, 8), and a judge
//! that simply refused every arrival not due on its own chirp's hour
//! recovered the clean book exactly. This planting closes that door:
//! a flat back wall stands behind every reflector at a range whose
//! extra round trip is exactly one glint interval (4 ticks). Every
//! return arrives twice — direct, and once more 4 ticks later at
//! strength RHO = 0.55 — so the echo of glint *j* lands exactly on
//! glint *j+1*'s offset and the two amplitudes sum in the ear. The
//! record holds four arrivals (offsets 0, 4, 8, 12): three summed
//! glints and one orphan — the last glint's echo, arriving after the
//! judged window closes, with nothing summed into it.
//!
//! Four judges read the SAME delivered record — amplitudes and
//! arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, spread vs the 0.30 line
//!   (flutter's judge).
//! - **Gated:** only arrivals at the train's own expected offsets —
//!   jammed's winner. Here every arrival is at an own offset, so its
//!   evidence is the naive judge's, sample for sample.
//! - **Tail:** un-echo the train. The orphan at offset 12 is
//!   RHO * own_2 and nothing else, so a search over RHO in
//!   [0.00, 0.95] that best predicts the orphan from the summed
//!   train prices the wall from the one arrival the wall could not
//!   hide inside another; the train is then unwound recursively
//!   (own_0 = d_0; own_j = d_j − RHO·own_{j−1}) and judged clean.
//! - **Restrike:** chirp again at glint spacing 5 instead of 4. The
//!   wall's delay is a property of the cave, not the chirp, so the
//!   echoes now land at 4, 9, 14 — between the new own offsets
//!   0, 5, 10 — and a gated judge on the second train hears its own
//!   glints alone. The cost is a second train, and the new spacing
//!   samples the moth's swing at phases the 0.30 line was not tuned
//!   on.
//!
//! The perception rule is sacred: a judge reads only amplitudes and
//! arrival offsets the medium delivered. Whether a verdict was right
//! is scored by the book, never a judge. The wall's RHO is ground
//! truth for scoring the tail judge's estimate only; the tail judge
//! is never told it.
//!
//! Predictions declared before the first run:
//! - P1: timing cannot separate coincident arrivals — gated errors
//!   equal naive errors at every sigma on contaminated evidence.
//! - P2: the summed step fools stones wholesale — at sigma 0.00 the
//!   naive judge's fooled count is at least 100 of 120 stone trains.
//! - P3: the orphan prices the wall exactly in a clean cave — at
//!   sigma 0.00 the tail judge's errors are 0 and its RHO estimate
//!   is within one grid step (0.01) of the wall's true RHO on every
//!   train.
//! - P4: the tail is one sample, and recursion amplifies its noise —
//!   at sigma 0.10 the tail judge's errors exceed the clean book's
//!   naive errors at the same sigma.
//! - P5: the re-strike escapes the wall but not the line — at sigma
//!   0.00 its errors are above 0 (spacing aliasing: some moth phases
//!   read steady at spacing 5) and far below the naive judge's, and
//!   none of them are fooled stones.
//! - P6: at sigma 0.30 the noise floor swallows every channel — the
//!   tail and re-strike judges' errors each land within 10 of the
//!   clean book's naive errors.
//!
//! `cargo run -p listener-echoed -- --headless` prints the ledgers
//! at sigma = 0.00, 0.10, 0.20, 0.30. There is no TUI; --headless is
//! handled before anything else regardless.

use anyhow::Result;

/// The flutter line every Listener since flutter has used.
const LINE: f64 = 0.30;
/// Glints under this amplitude never reach an ear (ripple's ear).
const EAR: f64 = 0.22;
/// Own glints per train (flutter's train).
const OWN_GLINTS: usize = 3;
/// Ticks between own glints in the first train.
const GLINT_DT: u64 = 4;
/// The wall's extra round trip, in ticks — exactly one glint
/// interval of the first train, so echoes land on own offsets.
const ECHO_DT: u64 = 4;
/// The wall's strength: each return re-arrives at this fraction.
const RHO: f64 = 0.55;
/// Glint spacing of the re-strike train.
const RESTRIKE_DT: u64 = 5;
/// Trains per kind per sigma in the book.
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
/// judge — and every rerun — hears the same record. The re-strike
/// train shares the first train's keys (same reflector, same cave).
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

/// Record one train at one noise level: its own glints at spacing
/// `dt` from `strike`, and — if `echo` — each glint's wall echo
/// ECHO_DT ticks later at strength RHO. Contributions that land on
/// the same offset sum in the ear before the ear's threshold is
/// applied; the record is delivered sorted by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, dt: u64, echo: bool) -> Train {
    let mut acc: Vec<(u64, f64)> = Vec::new();
    let mut add = |offset: u64, amp: f64| {
        if let Some(slot) = acc.iter_mut().find(|(o, _)| *o == offset) {
            slot.1 += amp;
        } else {
            acc.push((offset, amp));
        }
    };
    for j in 0..OWN_GLINTS {
        let own = clean_amplitude(kind, strike + dt * j as u64);
        add(dt * j as u64, own + glint_noise(id, j, false, sigma));
        if echo {
            add(
                dt * j as u64 + ECHO_DT,
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
/// judge; the noise is the medium's, not the judges'.
fn build_book(sigma: f64, dt: u64, echo: bool) -> Vec<Train> {
    let mut book = Vec::new();
    let mut id: u64 = 1;
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Moth, (k % 12) as u64, sigma, dt, echo));
        id += 1;
    }
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Stone, (k % 17) as u64, sigma, dt, echo));
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

fn judge_gated(delivered: &[Delivered]) -> Option<bool> {
    let amps: Vec<f64> = delivered
        .iter()
        .filter(|d| d.offset % GLINT_DT == 0)
        .map(|d| d.amp)
        .collect();
    if amps.len() < 3 {
        return None;
    }
    Some(spread(&amps[..3]) > LINE)
}

/// The tail judge: price the wall from the orphan arrival, unwind
/// the summation, judge the recovered own glints. Returns the
/// verdict and the judge's own RHO estimate.
fn judge_tail(delivered: &[Delivered]) -> Option<(bool, f64)> {
    let at = |offset: u64| delivered.iter().find(|d| d.offset == offset).map(|d| d.amp);
    let (d0, d1, d2, d3) = (at(0)?, at(4)?, at(8)?, at(12)?);
    // Search RHO on a 0.01 grid: the estimate whose recursively
    // unwound train best predicts the orphan d3 = RHO * own_2.
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

/// The re-strike judge is not a function of the first record: it
/// chirps again at RESTRIKE_DT and gates the second record by the
/// new spacing. Returns its verdict on the second train.
fn judge_restrike(id: u64, kind: Kind, strike: u64, sigma: f64) -> Option<bool> {
    let train = record_train(id, kind, strike, sigma, RESTRIKE_DT, true);
    let amps: Vec<f64> = train
        .delivered
        .iter()
        .filter(|d| d.offset % RESTRIKE_DT == 0)
        .map(|d| d.amp)
        .collect();
    if amps.len() < 3 {
        return None;
    }
    Some(spread(&amps[..3]) > LINE)
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
    #[allow(dead_code)]
    trains: usize,
    arrivals: usize,
    clean_naive: JudgeLedger,
    naive: JudgeLedger,
    gated: JudgeLedger,
    tail: JudgeLedger,
    tail_rho_err_max: f64,
    tail_rho_err_mean: f64,
    restrike: JudgeLedger,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64) -> Audit {
    let clean = build_book(sigma, GLINT_DT, false);
    let book = build_book(sigma, GLINT_DT, true);
    let mut a = Audit {
        trains: book.len(),
        arrivals: 0,
        clean_naive: JudgeLedger::default(),
        naive: JudgeLedger::default(),
        gated: JudgeLedger::default(),
        tail: JudgeLedger::default(),
        tail_rho_err_max: 0.0,
        tail_rho_err_mean: 0.0,
        restrike: JudgeLedger::default(),
    };
    for train in &clean {
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.clean_naive, v, is_moth);
        }
    }
    let mut rho_err_sum = 0.0;
    let mut rho_n = 0usize;
    for (idx, train) in book.iter().enumerate() {
        a.arrivals += train.delivered.len();
        let is_moth = train.kind == Kind::Moth;
        let id = idx as u64 + 1;
        let strike = match train.kind {
            Kind::Moth => (idx % TRAINS_PER_KIND) as u64 % 12,
            Kind::Stone => (idx % TRAINS_PER_KIND) as u64 % 17,
        };
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.naive, v, is_moth);
        }
        if let Some(v) = judge_gated(&train.delivered) {
            score(&mut a.gated, v, is_moth);
        }
        if let Some((v, r)) = judge_tail(&train.delivered) {
            score(&mut a.tail, v, is_moth);
            let e = (r - RHO).abs();
            rho_err_sum += e;
            rho_n += 1;
            if e > a.tail_rho_err_max {
                a.tail_rho_err_max = e;
            }
        }
        if let Some(v) = judge_restrike(id, train.kind, strike, sigma) {
            score(&mut a.restrike, v, is_moth);
        }
    }
    if rho_n > 0 {
        a.tail_rho_err_mean = rho_err_sum / rho_n as f64;
    }
    a
}

fn run_headless() {
    println!("listener-echoed headless: one book of recorded trains, every return echoed onto the next own offset by a back wall, four judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, wall echo +{} ticks at rho {:.2}, orphan at offset 12",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        ECHO_DT,
        RHO
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), gated (own offsets only), tail (un-echo via the orphan), restrike (second train at spacing {}, gated)", LINE, RESTRIKE_DT);
    let mut audits = Vec::new();
    for sigma in [0.00_f64, 0.10, 0.20, 0.30] {
        let a = audit(sigma);
        println!(
            "  sigma {:.2}: arrivals {}, clean naive errors {} (wrong {}, fooled {}), echoed naive errors {} (wrong {}, fooled {}), gated errors {} (wrong {}, fooled {}), tail errors {} (wrong {}, fooled {}), tail rho err mean {:.3} max {:.3}, restrike errors {} (wrong {}, fooled {})",
            sigma,
            a.arrivals,
            a.clean_naive.errors(),
            a.clean_naive.wrong_refusals,
            a.clean_naive.fooled,
            a.naive.errors(),
            a.naive.wrong_refusals,
            a.naive.fooled,
            a.gated.errors(),
            a.gated.wrong_refusals,
            a.gated.fooled,
            a.tail.errors(),
            a.tail.wrong_refusals,
            a.tail.fooled,
            a.tail_rho_err_mean,
            a.tail_rho_err_max,
            a.restrike.errors(),
            a.restrike.wrong_refusals,
            a.restrike.fooled
        );
        audits.push((sigma, a));
    }
    let at = |s: f64| &audits.iter().find(|(x, _)| *x == s).unwrap().1;
    println!("  declared predictions, judged on identical echoed evidence:");
    println!(
        "    P1 timing cannot separate coincident arrivals (gated errors == naive errors at every sigma): {} / {} / {} / {} vs {} / {} / {} / {} — {}",
        at(0.00).gated.errors(), at(0.10).gated.errors(), at(0.20).gated.errors(), at(0.30).gated.errors(),
        at(0.00).naive.errors(), at(0.10).naive.errors(), at(0.20).naive.errors(), at(0.30).naive.errors(),
        if [0.00, 0.10, 0.20, 0.30].iter().all(|s| at(*s).gated.errors() == at(*s).naive.errors()) { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the summed step fools stones wholesale (sigma 0.00 naive fooled >= 100 of {}): {} — {}",
        TRAINS_PER_KIND,
        at(0.00).naive.fooled,
        if at(0.00).naive.fooled >= 100 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 the orphan prices the wall exactly in a clean cave (sigma 0.00 tail errors 0, rho err max <= 0.011): errors {}, rho err max {:.3} — {}",
        at(0.00).tail.errors(),
        at(0.00).tail_rho_err_max,
        if at(0.00).tail.errors() == 0 && at(0.00).tail_rho_err_max <= 0.011 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the tail amplifies noise (sigma 0.10 tail errors > clean naive errors): {} vs {} — {}",
        at(0.10).tail.errors(),
        at(0.10).clean_naive.errors(),
        if at(0.10).tail.errors() > at(0.10).clean_naive.errors() { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 the re-strike escapes the wall but not the line (sigma 0.00 restrike errors > 0, < naive, fooled 0): {} vs naive {}, fooled {} — {}",
        at(0.00).restrike.errors(),
        at(0.00).naive.errors(),
        at(0.00).restrike.fooled,
        if at(0.00).restrike.errors() > 0 && at(0.00).restrike.errors() < at(0.00).naive.errors() && at(0.00).restrike.fooled == 0 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P6 at sigma 0.30 every channel sits on the noise floor (tail and restrike errors within 10 of clean naive): tail {} , restrike {}, clean {} — {}",
        at(0.30).tail.errors(),
        at(0.30).restrike.errors(),
        at(0.30).clean_naive.errors(),
        if (at(0.30).tail.errors() as i64 - at(0.30).clean_naive.errors() as i64).abs() <= 10
            && (at(0.30).restrike.errors() as i64 - at(0.30).clean_naive.errors() as i64).abs() <= 10 { "HELD" } else { "CRACKED" }
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
            let t = record_train(1, Kind::Moth, strike, 0.0, GLINT_DT, false);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) > LINE, "moth strike {strike}");
        }
    }

    #[test]
    fn clean_stone_trains_stay_under_the_line_on_three_own_glints() {
        for strike in 0..17u64 {
            let t = record_train(2, Kind::Stone, strike, 0.0, GLINT_DT, false);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) <= LINE, "stone strike {strike}");
        }
    }

    #[test]
    fn clean_unwalled_book_has_zero_errors_for_the_naive_judge() {
        let book = build_book(0.0, GLINT_DT, false);
        for t in &book {
            let v = judge_naive(&t.delivered).expect("clean train judged");
            assert_eq!(v, t.kind == Kind::Moth);
        }
    }

    #[test]
    fn the_wall_sums_each_echo_onto_the_next_own_offset() {
        // At sigma 0 the record is exactly four arrivals: own_0,
        // own_1 + RHO*own_0, own_2 + RHO*own_1, and the orphan
        // RHO*own_2 at offset 12, with nothing summed into it.
        let t = record_train(7, Kind::Stone, 3, 0.0, GLINT_DT, true);
        assert_eq!(t.delivered.len(), 4);
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 3 + 4 * j)).collect();
        let expect = [own[0], own[1] + RHO * own[0], own[2] + RHO * own[1], RHO * own[2]];
        for (d, e) in t.delivered.iter().zip(expect.iter()) {
            assert!((d.amp - e).abs() < 1e-12, "amp {} vs expected {}", d.amp, e);
        }
        assert_eq!(t.delivered[3].offset, 12);
    }

    #[test]
    fn every_stone_train_is_fooled_by_the_summed_step_at_sigma_zero() {
        let book = build_book(0.0, GLINT_DT, true);
        for t in book.iter().filter(|t| t.kind == Kind::Stone) {
            assert_eq!(judge_naive(&t.delivered), Some(true), "stone train not fooled");
        }
    }

    #[test]
    fn a_moth_at_its_peak_reads_steady_through_the_wall() {
        // The named flattening: struck at phase 0 the moth's own
        // glints fall (1.00, 0.70, 0.70) and the echo step fills the
        // fall in — the summed train's spread drops under the line.
        let t = record_train(1, Kind::Moth, 0, 0.0, GLINT_DT, true);
        assert_eq!(judge_naive(&t.delivered), Some(false));
    }

    #[test]
    fn the_gate_and_the_naive_judge_agree_on_every_echoed_train() {
        for sigma in [0.0, 0.10, 0.20, 0.30] {
            let book = build_book(sigma, GLINT_DT, true);
            for t in &book {
                assert_eq!(judge_gated(&t.delivered), judge_naive(&t.delivered), "sigma {sigma}");
            }
        }
    }

    #[test]
    fn the_tail_judge_recovers_the_wall_and_the_book_at_sigma_zero() {
        let book = build_book(0.0, GLINT_DT, true);
        for t in &book {
            let (v, r) = judge_tail(&t.delivered).expect("tail judged");
            assert_eq!(v, t.kind == Kind::Moth);
            assert!((r - RHO).abs() <= 0.011, "rho estimate {r}");
        }
    }

    #[test]
    fn the_restrike_moves_the_echoes_off_the_own_offsets() {
        // At spacing 5 the wall's echoes land at 4, 9, 14 — none of
        // them an own offset — and the gated selection on the second
        // train is exactly the three clean own glints at sigma 0.
        let t = record_train(3, Kind::Stone, 5, 0.0, RESTRIKE_DT, true);
        let offsets: Vec<u64> = t.delivered.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0, 4, 5, 9, 10, 14]);
        let gated: Vec<f64> = t
            .delivered
            .iter()
            .filter(|d| d.offset % RESTRIKE_DT == 0)
            .map(|d| d.amp)
            .collect();
        let own: Vec<f64> = (0..3).map(|j| clean_amplitude(Kind::Stone, 5 + 5 * j)).collect();
        assert_eq!(gated, own);
    }

    #[test]
    fn the_restrike_fools_no_stone_at_sigma_zero_but_loses_some_moths() {
        let a = audit(0.0);
        assert_eq!(a.restrike.fooled, 0);
        assert!(a.restrike.wrong_refusals > 0);
        assert!(a.restrike.errors() < a.naive.errors());
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, GLINT_DT, true);
        let b2 = build_book(0.20, GLINT_DT, true);
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
