//! # Listener Paired 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, and `experiments/listener-adaptive` —
//! the adaptive bat's own caveat, taken up as the subject.
//!
//! listener-adaptive compared two whole hunts: a fixed bat (three
//! glints, the 0.30 spread line) and an adaptive bat (a fourth glint
//! bought whenever the three-glint spread landed in the uncertain
//! band [0.20, 0.36]). At sigma 0.30 the adaptive bat made MORE
//! errors than the fixed bat (17 vs 14), and its README kept the
//! caveat honestly: whole-hunt ledgers, not paired trains — the two
//! bats' paths diverged, so the trains each bat heard were not the
//! same trains. Its closing seed named the follow-up by name: *a
//! paired-train audit on identical recorded evidence (the same
//! trains judged by both bats, so path divergence cannot confound
//! the comparison), then, if the range-inflation mechanism holds,
//! an n-corrected line that widens with the number of glints judged.*
//! This is that audit. There is no hunt and no flight here: a fixed
//! book of recorded trains — moths and stones struck at every phase
//! of their periods, in ripple's medium — is judged, train by train,
//! by three judges who read the SAME delivered amplitudes:
//!
//! - **Fixed:** the first three delivered glints, the 0.30 line.
//! - **Adaptive:** listener-adaptive's rule verbatim — if the
//!   three-glint spread lands in [0.20, 0.36], hold for the fourth
//!   glint and judge all four on the same 0.30 line.
//! - **Corrected:** the adaptive rule with one change — a four-glint
//!   judgment is made on a widened line, 0.30 * 1.20 = 0.36. The
//!   factor is the range-inflation of uniform noise itself: the
//!   expected range of n draws from U(-s, s) is 2s(n-1)/(n+1), so
//!   four draws range 1.20x wider than three, in expectation. The
//!   line widens by exactly the amount the extra glint inflates
//!   the statistic it is judged by — no more, no less.
//!
//! The medium is ripple's, unchanged: moths swing shallow
//! (0.80 + 0.20*cos, period 12), stones ripple (0.92 + 0.08*cos,
//! period 17, per-train phase), glints 4 ticks apart, per-glint
//! noise in [-sigma, +sigma] drawn deterministically (SplitMix64)
//! from the train's identity, glints under the ear threshold
//! (0.22) dropped before any judge hears them. A train with fewer
//! than three delivered glints is unattended — counted, never
//! judged, identical for all three judges.
//!
//! The perception rule is sacred, as always: every judge reads
//! only the amplitudes the medium delivered — never a clean value,
//! never a reflector's kind, never the true sigma. Whether a
//! verdict was right is scored by the book, never by the judge.
//! And the pairing is the point: because all three judges read the
//! same recorded train, any difference in their ledgers is the
//! judging rule and nothing else. Path divergence has no vote.
//!
//! `cargo run -p listener-paired -- --headless` prints the three
//! judges' ledgers at sigma = 0.00, 0.10, 0.20, 0.30. There is no
//! TUI; --headless is handled before anything else regardless.

use anyhow::Result;

// ------------------------------------------------------------------
// The medium (ripple's, unchanged) and the book of trains.
// ------------------------------------------------------------------

/// The flutter line every Listener since flutter has used.
const LINE: f64 = 0.30;
/// The uncertain band (listener-adaptive's, verbatim): a three-glint
/// spread inside it buys a fourth glint.
const BAND_LO: f64 = 0.20;
const BAND_HI: f64 = 0.36;
/// The n-correction: expected range of 4 uniform draws / of 3 draws
/// = (2s*3/5) / (2s*2/4) = 1.20.
const RANGE_CORRECTION: f64 = 1.20;
/// Glints under this amplitude never reach an ear (ripple's ear).
const EAR: f64 = 0.22;
/// Glints per train the medium sends.
const GLINTS: usize = 4;
/// Ticks between glints.
const GLINT_DT: u64 = 4;
/// Trains per kind per sigma in the book.
const TRAINS_PER_KIND: usize = 120;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Moth,
    Stone,
}

/// One recorded train: what the medium delivered, and what struck it.
struct Train {
    kind: Kind,
    /// All four clean amplitudes (the book's ground truth; judges
    /// never see these — kept in the record so the book is complete).
    #[allow(dead_code)]
    clean: [f64; GLINTS],
    /// The delivered amplitudes after noise and the ear: the only
    /// thing any judge may read.
    delivered: Vec<f64>,
}

/// SplitMix64, the garden's deterministic noise hand (ripple's).
fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Deterministic noise for one glint of one train at one sigma,
/// uniform in [-sigma, +sigma]. Keyed by the train's identity alone,
/// so every judge — and every rerun — hears the same train.
fn glint_noise(train_id: u64, glint: usize, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let key = train_id
        .wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ ((glint as u64 + 1).wrapping_mul(0x8CB9_2AE3_5C77_19A7))
        ^ ((sigma * 1000.0) as u64).wrapping_mul(0x1656_6679_19E3_779F);
    let bits = splitmix64(key);
    let unit = (bits >> 11) as f64 / (1u64 << 53) as f64; // [0, 1)
    (unit * 2.0 - 1.0) * sigma
}

/// The clean amplitude of a reflector's glint at tick `t`.
/// Moths swing shallow with period 12; stones ripple with period 17
/// and a per-train phase (the train's strike offset does this work).
fn clean_amplitude(kind: Kind, t: u64) -> f64 {
    let tau = std::f64::consts::TAU;
    match kind {
        Kind::Moth => 0.80 + 0.20 * (tau * (t % 12) as f64 / 12.0).cos(),
        Kind::Stone => 0.92 + 0.08 * (tau * (t % 17) as f64 / 17.0).cos(),
    }
}

/// Strike one train: record its clean glints, then let the medium
/// corrupt and thin them into what an ear would actually receive.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64) -> Train {
    let mut clean = [0.0; GLINTS];
    let mut delivered = Vec::new();
    for j in 0..GLINTS {
        let a = clean_amplitude(kind, strike + GLINT_DT * j as u64);
        clean[j] = a;
        let heard = a + glint_noise(id, j, sigma);
        if heard >= EAR {
            delivered.push(heard);
        }
    }
    Train {
        kind,
        clean,
        delivered,
    }
}

/// The book: for each kind, trains struck at every phase of the
/// reflector's period, several rounds over. Identical for every
/// sigma except the noise the medium adds.
fn build_book(sigma: f64) -> Vec<Train> {
    let mut book = Vec::new();
    let mut id: u64 = 1;
    for k in 0..TRAINS_PER_KIND {
        // Moth strikes cover period 12; repeat over 10 rounds.
        book.push(record_train(id, Kind::Moth, (k % 12) as u64, sigma));
        id += 1;
    }
    for k in 0..TRAINS_PER_KIND {
        // Stone strikes cover period 17.
        book.push(record_train(id, Kind::Stone, (k % 17) as u64, sigma));
        id += 1;
    }
    book
}

// ------------------------------------------------------------------
// The three judges. Each reads only a train's delivered amplitudes.
// ------------------------------------------------------------------

/// The spread statistic every Listener judge uses.
fn spread(glints: &[f64]) -> f64 {
    let max = glints.iter().cloned().fold(f64::MIN, f64::max);
    let min = glints.iter().cloned().fold(f64::MAX, f64::min);
    let mean = glints.iter().sum::<f64>() / glints.len() as f64;
    (max - min) / mean
}

/// A verdict: true = "that swings — a moth", false = "steady — a stone".
fn judge_fixed(delivered: &[f64]) -> Option<bool> {
    if delivered.len() < 3 {
        return None;
    }
    Some(spread(&delivered[..3]) > LINE)
}

fn judge_adaptive(delivered: &[f64]) -> Option<(bool, bool)> {
    // Returns (verdict, extended?)
    if delivered.len() < 3 {
        return None;
    }
    let s3 = spread(&delivered[..3]);
    if (BAND_LO..=BAND_HI).contains(&s3) && delivered.len() >= 4 {
        Some((spread(&delivered[..4]) > LINE, true))
    } else {
        Some((s3 > LINE, false))
    }
}

fn judge_corrected(delivered: &[f64]) -> Option<(bool, bool)> {
    if delivered.len() < 3 {
        return None;
    }
    let s3 = spread(&delivered[..3]);
    if (BAND_LO..=BAND_HI).contains(&s3) && delivered.len() >= 4 {
        Some((spread(&delivered[..4]) > LINE * RANGE_CORRECTION, true))
    } else {
        Some((s3 > LINE, false))
    }
}

// ------------------------------------------------------------------
// The audit.
// ------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct JudgeLedger {
    judged: usize,
    wrong_refusals: usize, // moth judged stone
    fooled: usize,         // stone judged moth
    extensions: usize,
}

impl JudgeLedger {
    fn errors(&self) -> usize {
        self.wrong_refusals + self.fooled
    }
}

struct Audit {
    trains: usize,
    unattended: usize,
    dropped_glints: usize,
    fixed: JudgeLedger,
    adaptive: JudgeLedger,
    corrected: JudgeLedger,
}

fn audit(sigma: f64) -> Audit {
    let book = build_book(sigma);
    let mut a = Audit {
        trains: book.len(),
        unattended: 0,
        dropped_glints: 0,
        fixed: JudgeLedger::default(),
        adaptive: JudgeLedger::default(),
        corrected: JudgeLedger::default(),
    };
    for train in &book {
        a.dropped_glints += GLINTS - train.delivered.len();
        let is_moth = train.kind == Kind::Moth;
        match judge_fixed(&train.delivered) {
            None => a.unattended += 1,
            Some(says_moth) => {
                a.fixed.judged += 1;
                if says_moth && !is_moth {
                    a.fixed.fooled += 1;
                } else if !says_moth && is_moth {
                    a.fixed.wrong_refusals += 1;
                }
            }
        }
        if let Some((says_moth, ext)) = judge_adaptive(&train.delivered) {
            a.adaptive.judged += 1;
            if ext {
                a.adaptive.extensions += 1;
            }
            if says_moth && !is_moth {
                a.adaptive.fooled += 1;
            } else if !says_moth && is_moth {
                a.adaptive.wrong_refusals += 1;
            }
        }
        if let Some((says_moth, ext)) = judge_corrected(&train.delivered) {
            a.corrected.judged += 1;
            if ext {
                a.corrected.extensions += 1;
            }
            if says_moth && !is_moth {
                a.corrected.fooled += 1;
            } else if !says_moth && is_moth {
                a.corrected.wrong_refusals += 1;
            }
        }
    }
    a
}

fn run_headless() {
    println!("listener-paired headless: one book of recorded trains, three judges, identical evidence");
    println!(
        "  book: {} trains per sigma ({} moths, {} stones), {} glints each, {} ticks apart",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND,
        GLINTS,
        GLINT_DT
    );
    println!(
        "  line {:.2}; adaptive band [{:.2}, {:.2}]; corrected four-glint line {:.2} (x{:.2} range correction)",
        LINE,
        BAND_LO,
        BAND_HI,
        LINE * RANGE_CORRECTION,
        RANGE_CORRECTION
    );
    let mut audits = Vec::new();
    for sigma in [0.00_f64, 0.10, 0.20, 0.30] {
        let a = audit(sigma);
        println!("  sigma {:.2}:", sigma);
        println!(
            "    trains {}, unattended {}, glints dropped {}",
            a.trains, a.unattended, a.dropped_glints
        );
        println!(
            "    fixed:     judged {}, errors {} (wrong refusals {}, fooled {})",
            a.fixed.judged,
            a.fixed.errors(),
            a.fixed.wrong_refusals,
            a.fixed.fooled
        );
        println!(
            "    adaptive:  judged {}, errors {} (wrong refusals {}, fooled {}), extensions {}",
            a.adaptive.judged,
            a.adaptive.errors(),
            a.adaptive.wrong_refusals,
            a.adaptive.fooled,
            a.adaptive.extensions
        );
        println!(
            "    corrected: judged {}, errors {} (wrong refusals {}, fooled {}), extensions {}",
            a.corrected.judged,
            a.corrected.errors(),
            a.corrected.wrong_refusals,
            a.corrected.fooled,
            a.corrected.extensions
        );
        audits.push((sigma, a));
    }
    let at = |s: f64| &audits.iter().find(|(x, _)| *x == s).unwrap().1;
    println!("  declared predictions, judged on paired evidence:");
    println!(
        "    P1 noise prices the fixed judge (fixed errors sigma 0.30 > sigma 0.00): {} vs {} — {}",
        at(0.30).fixed.errors(),
        at(0.00).fixed.errors(),
        if at(0.30).fixed.errors() > at(0.00).fixed.errors() {
            "HELD"
        } else {
            "CRACKED"
        }
    );
    println!(
        "    P2 the fourth glint pays at sigma 0.10 (adaptive errors < fixed): {} vs {} — {}",
        at(0.10).adaptive.errors(),
        at(0.10).fixed.errors(),
        if at(0.10).adaptive.errors() < at(0.10).fixed.errors() {
            "HELD"
        } else {
            "CRACKED"
        }
    );
    println!(
        "    P3 range inflation replicates paired at sigma 0.30 (adaptive errors >= fixed): {} vs {} — {}",
        at(0.30).adaptive.errors(),
        at(0.30).fixed.errors(),
        if at(0.30).adaptive.errors() >= at(0.30).fixed.errors() {
            "HELD"
        } else {
            "CRACKED"
        }
    );
    println!(
        "    P4 the corrected line beats the adaptive at sigma 0.30 (corrected errors < adaptive): {} vs {} — {}",
        at(0.30).corrected.errors(),
        at(0.30).adaptive.errors(),
        if at(0.30).corrected.errors() < at(0.30).adaptive.errors() {
            "HELD"
        } else {
            "CRACKED"
        }
    );
    println!(
        "    P5 the correction costs nothing clean (corrected errors at sigma 0.00 = 0): {} — {}",
        at(0.00).corrected.errors(),
        if at(0.00).corrected.errors() == 0 {
            "HELD"
        } else {
            "CRACKED"
        }
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
    fn clean_moth_trains_clear_the_line_on_three_glints() {
        for strike in 0..12u64 {
            let t = record_train(1, Kind::Moth, strike, 0.0);
            let s = spread(&t.delivered[..3]);
            assert!(s > LINE, "moth strike {strike} spread {s}");
        }
    }

    #[test]
    fn clean_stone_trains_stay_under_the_line_on_three_glints() {
        for strike in 0..17u64 {
            let t = record_train(2, Kind::Stone, strike, 0.0);
            let s = spread(&t.delivered[..3]);
            assert!(s <= LINE, "stone strike {strike} spread {s}");
        }
    }

    #[test]
    fn no_clean_train_enters_the_uncertain_band() {
        // The band is the adaptive judge's purchase trigger; if a
        // clean train could enter it, extensions would fire clean.
        for strike in 0..12u64 {
            let t = record_train(3, Kind::Moth, strike, 0.0);
            let s = spread(&t.delivered[..3]);
            assert!(!(BAND_LO..=BAND_HI).contains(&s), "moth {strike} {s}");
        }
        for strike in 0..17u64 {
            let t = record_train(4, Kind::Stone, strike, 0.0);
            let s = spread(&t.delivered[..3]);
            assert!(!(BAND_LO..=BAND_HI).contains(&s), "stone {strike} {s}");
        }
    }

    #[test]
    fn clean_book_has_zero_errors_for_every_judge() {
        let a = audit(0.0);
        assert_eq!(a.fixed.errors(), 0);
        assert_eq!(a.adaptive.errors(), 0);
        assert_eq!(a.corrected.errors(), 0);
        assert_eq!(a.adaptive.extensions, 0);
        assert_eq!(a.unattended, 0);
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        // Same sigma, built twice: every delivered amplitude identical,
        // so the judges' ledgers are functions of the rules alone.
        let b1 = build_book(0.20);
        let b2 = build_book(0.20);
        assert_eq!(b1.len(), b2.len());
        for (t1, t2) in b1.iter().zip(b2.iter()) {
            assert_eq!(t1.kind, t2.kind);
            assert_eq!(t1.delivered, t2.delivered);
        }
    }

    #[test]
    fn noise_is_zero_at_sigma_zero_and_bounded_elsewhere() {
        for id in 1..50u64 {
            assert_eq!(glint_noise(id, 0, 0.0), 0.0);
            for j in 0..GLINTS {
                let n = glint_noise(id, j, 0.30);
                assert!(n.abs() <= 0.30, "noise {n} out of bounds");
            }
        }
    }

    #[test]
    fn noise_grows_errors_for_the_fixed_judge() {
        assert!(audit(0.30).fixed.errors() > audit(0.0).fixed.errors());
    }
}
