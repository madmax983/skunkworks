//! # Listener Jammed 🦇
//!
//! Brought by The Night Listener 🦇. Lineage: `experiments/vespertilio`
//! by way of `experiments/listener-choir`, `experiments/listener-flutter`,
//! `experiments/listener-ripple`, `experiments/listener-adaptive`, and
//! `experiments/listener-paired` — the paired audit's own closing seed,
//! taken up as the subject: *the paired audit extended to the choir:
//! foreign glints interleaved into a recorded train, judged paired, so
//! jamming is priced as evidence corruption rather than as a hunt
//! outcome.*
//!
//! listener-choir priced jamming as a hunt outcome (catches per bat:
//! solo 10.00, naive choir 5.67, polite choir 6.67) — but a hunt mixes
//! the corruption with the paths the corrupted bat then flew.
//! listener-paired removed the hunt for noise; this planting removes
//! it for jamming. A book of recorded trains — moths and stones struck
//! at every phase, in ripple's medium — is re-recorded with a second,
//! foreign reflector's glints interleaved between the train's own:
//! own glints arrive at offsets 0, 4, 8; foreign glints at offsets
//! 2, 6, 10, inside the same 12-tick listening window. The foreign
//! reflector's kind (moth-like swing or stone-like ripple) and phase
//! are fixed by the train's identity alone, never by the train's own
//! kind, so a foreign glint is as likely to steady a moth train as to
//! stir a stone train. Three judges read the SAME delivered record —
//! amplitudes and arrival offsets, nothing else:
//!
//! - **Naive:** the first three arrivals, in arrival order, spread
//!   vs the 0.30 line (flutter's judge, deaf to timing).
//! - **Gated:** only the arrivals at the train's own expected
//!   offsets (0, 4, 8) — a bat that knows when its own chirp's
//!   returns are due and refuses the rest of the window.
//! - **Greedy:** every arrival in the window, spread over all of
//!   them — a bat that uses all the evidence there is.
//!
//! The perception rule is sacred: a judge reads only amplitudes and
//! arrival offsets the medium delivered. The gated judge filters by
//! offset, never by the record's foreign flag — the flag is the
//! book's ground truth, kept for scoring and structure tests only.
//! Whether a verdict was right is scored by the book, never a judge.
//!
//! `cargo run -p listener-jammed -- --headless` prints the three
//! judges' ledgers at sigma = 0.00, 0.10, 0.20 for jamming doses of
//! 0, 1, 2, 3 foreign glints per train. There is no TUI; --headless
//! is handled before anything else regardless.

use anyhow::Result;

/// The flutter line every Listener since flutter has used.
const LINE: f64 = 0.30;
/// Glints under this amplitude never reach an ear (ripple's ear).
const EAR: f64 = 0.22;
/// Own glints per train (flutter's train).
const OWN_GLINTS: usize = 3;
/// Ticks between own glints.
const GLINT_DT: u64 = 4;
/// Foreign slots inside the window, at these offsets.
const FOREIGN_OFFSETS: [u64; 3] = [2, 6, 10];
/// Trains per kind per (sigma, dose) in the book.
const TRAINS_PER_KIND: usize = 120;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Moth,
    Stone,
}

/// One delivered glint: what an ear in the window actually receives.
struct Delivered {
    offset: u64,
    amp: f64,
    /// The book's ground truth. Judges NEVER read this; the gated
    /// judge filters by offset alone.
    foreign: bool,
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

/// Deterministic noise for one glint of one train at one sigma,
/// uniform in [-sigma, +sigma]. Keyed by the train's identity, the
/// glint slot, and whether the glint is foreign, so every judge —
/// and every rerun — hears the same record.
fn glint_noise(train_id: u64, slot: usize, foreign: bool, sigma: f64) -> f64 {
    if sigma <= 0.0 {
        return 0.0;
    }
    let mut key = train_id
        .wrapping_mul(0xD1B5_4A32_D192_ED03)
        ^ ((slot as u64 + 1).wrapping_mul(0x8CB9_2AE3_5C77_19A7))
        ^ ((sigma * 1000.0) as u64).wrapping_mul(0x1656_6679_19E3_779F);
    if foreign {
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

/// The foreign reflector for one train: kind and phase from the
/// train's identity alone — never from the train's own kind.
fn foreign_for(id: u64) -> (Kind, u64) {
    let h = splitmix64(id ^ 0xA5A5_5A5A_C3C3_3C3C);
    let kind = if h & 1 == 0 { Kind::Moth } else { Kind::Stone };
    let base = (h >> 8) % 24;
    (kind, base)
}

/// Record one train at one noise level and one jamming dose:
/// its own glints at 0, 4, 8 and the first `dose` foreign glints
/// at 2, 6, 10, all corrupted by the medium and thinned by the ear,
/// delivered sorted by arrival offset.
fn record_train(id: u64, kind: Kind, strike: u64, sigma: f64, dose: usize) -> Train {
    let mut delivered = Vec::new();
    for j in 0..OWN_GLINTS {
        let offset = GLINT_DT * j as u64;
        let a = clean_amplitude(kind, strike + offset);
        let heard = a + glint_noise(id, j, false, sigma);
        if heard >= EAR {
            delivered.push(Delivered { offset, amp: heard, foreign: false });
        }
    }
    let (fkind, fbase) = foreign_for(id);
    for (j, &offset) in FOREIGN_OFFSETS.iter().enumerate().take(dose) {
        let a = clean_amplitude(fkind, fbase + offset);
        let heard = a + glint_noise(id, j, true, sigma);
        if heard >= EAR {
            delivered.push(Delivered { offset, amp: heard, foreign: true });
        }
    }
    delivered.sort_by_key(|d| d.offset);
    Train { kind, delivered }
}

/// The book: for each kind, trains struck at every phase of the
/// reflector's period, several rounds over. Identical for every
/// judge; the dose and the noise are the medium's, not the judges'.
fn build_book(sigma: f64, dose: usize) -> Vec<Train> {
    let mut book = Vec::new();
    let mut id: u64 = 1;
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Moth, (k % 12) as u64, sigma, dose));
        id += 1;
    }
    for k in 0..TRAINS_PER_KIND {
        book.push(record_train(id, Kind::Stone, (k % 17) as u64, sigma, dose));
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

fn judge_greedy(delivered: &[Delivered]) -> Option<bool> {
    if delivered.len() < 3 {
        return None;
    }
    let amps: Vec<f64> = delivered.iter().map(|d| d.amp).collect();
    Some(spread(&amps) > LINE)
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
    foreign_delivered: usize,
    naive: JudgeLedger,
    gated: JudgeLedger,
    greedy: JudgeLedger,
}

fn score(ledger: &mut JudgeLedger, says_moth: bool, is_moth: bool) {
    ledger.judged += 1;
    if says_moth && !is_moth {
        ledger.fooled += 1;
    } else if !says_moth && is_moth {
        ledger.wrong_refusals += 1;
    }
}

fn audit(sigma: f64, dose: usize) -> Audit {
    let book = build_book(sigma, dose);
    let mut a = Audit {
        trains: book.len(),
        foreign_delivered: 0,
        naive: JudgeLedger::default(),
        gated: JudgeLedger::default(),
        greedy: JudgeLedger::default(),
    };
    for train in &book {
        a.foreign_delivered += train.delivered.iter().filter(|d| d.foreign).count();
        let is_moth = train.kind == Kind::Moth;
        if let Some(v) = judge_naive(&train.delivered) {
            score(&mut a.naive, v, is_moth);
        }
        if let Some(v) = judge_gated(&train.delivered) {
            score(&mut a.gated, v, is_moth);
        }
        if let Some(v) = judge_greedy(&train.delivered) {
            score(&mut a.greedy, v, is_moth);
        }
    }
    a
}

fn run_headless() {
    println!("listener-jammed headless: one book of recorded trains, foreign glints interleaved, three judges, identical evidence");
    println!(
        "  book: {} trains per regime ({} moths, {} stones), own glints at offsets 0/4/8, foreign slots at 2/6/10",
        TRAINS_PER_KIND * 2,
        TRAINS_PER_KIND,
        TRAINS_PER_KIND
    );
    println!("  line {:.2}; judges: naive (first 3 arrivals), gated (own offsets only), greedy (all arrivals)", LINE);
    let mut audits = Vec::new();
    for sigma in [0.00_f64, 0.10, 0.20] {
        for dose in 0..=3usize {
            let a = audit(sigma, dose);
            println!(
                "  sigma {:.2} dose {}: foreign delivered {}, naive errors {} (wrong {}, fooled {}), gated errors {} (wrong {}, fooled {}), greedy errors {} (wrong {}, fooled {})",
                sigma,
                dose,
                a.foreign_delivered,
                a.naive.errors(),
                a.naive.wrong_refusals,
                a.naive.fooled,
                a.gated.errors(),
                a.gated.wrong_refusals,
                a.gated.fooled,
                a.greedy.errors(),
                a.greedy.wrong_refusals,
                a.greedy.fooled
            );
            audits.push((sigma, dose, a));
        }
    }
    let at = |s: f64, d: usize| &audits.iter().find(|(x, y, _)| *x == s && *y == d).unwrap().2;
    println!("  declared predictions, judged on identical interleaved evidence:");
    println!(
        "    P1 jamming alone corrupts the naive judge (sigma 0.00 naive errors dose 3 > dose 0): {} vs {} — {}",
        at(0.00, 3).naive.errors(),
        at(0.00, 0).naive.errors(),
        if at(0.00, 3).naive.errors() > at(0.00, 0).naive.errors() { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P2 the timing gate recovers exactly (gated errors at sigma 0.10 equal at doses 0 and 3, and gated errors at sigma 0.00 are 0 at dose 3): {} vs {}, {} — {}",
        at(0.10, 3).gated.errors(),
        at(0.10, 0).gated.errors(),
        at(0.00, 3).gated.errors(),
        if at(0.10, 3).gated.errors() == at(0.10, 0).gated.errors() && at(0.00, 3).gated.errors() == 0 { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P3 using everything does not recover (sigma 0.00 greedy errors at dose 3 > gated): {} vs {} — {}",
        at(0.00, 3).greedy.errors(),
        at(0.00, 3).gated.errors(),
        if at(0.00, 3).greedy.errors() > at(0.00, 3).gated.errors() { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P4 the first foreign glint does all the naive damage (sigma 0.00 naive errors equal at doses 1, 2, 3): {} / {} / {} — {}",
        at(0.00, 1).naive.errors(),
        at(0.00, 2).naive.errors(),
        at(0.00, 3).naive.errors(),
        if at(0.00, 1).naive.errors() == at(0.00, 2).naive.errors()
            && at(0.00, 2).naive.errors() == at(0.00, 3).naive.errors() { "HELD" } else { "CRACKED" }
    );
    println!(
        "    P5 the greedy judge worsens with dose (sigma 0.00 greedy errors dose 3 > dose 1): {} vs {} — {}",
        at(0.00, 3).greedy.errors(),
        at(0.00, 1).greedy.errors(),
        if at(0.00, 3).greedy.errors() > at(0.00, 1).greedy.errors() { "HELD" } else { "CRACKED" }
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
            let t = record_train(1, Kind::Moth, strike, 0.0, 0);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) > LINE, "moth strike {strike}");
        }
    }

    #[test]
    fn clean_stone_trains_stay_under_the_line_on_three_own_glints() {
        for strike in 0..17u64 {
            let t = record_train(2, Kind::Stone, strike, 0.0, 0);
            let amps: Vec<f64> = t.delivered.iter().map(|d| d.amp).collect();
            assert!(spread(&amps) <= LINE, "stone strike {strike}");
        }
    }

    #[test]
    fn clean_unjammed_book_has_zero_errors_for_every_judge() {
        let a = audit(0.0, 0);
        assert_eq!(a.naive.errors(), 0);
        assert_eq!(a.gated.errors(), 0);
        assert_eq!(a.greedy.errors(), 0);
        assert_eq!(a.foreign_delivered, 0);
    }

    #[test]
    fn the_gate_recovers_the_clean_book_at_full_dose() {
        // The gated judge filters by arrival offset alone, so at
        // sigma 0 no dose of interleaving can reach its evidence.
        let a = audit(0.0, 3);
        assert_eq!(a.gated.errors(), 0);
        assert!(a.foreign_delivered > 0);
    }

    #[test]
    fn the_naive_judges_first_three_contain_exactly_one_foreign_at_any_positive_dose() {
        // Arrivals sort 0(own), 2(foreign), 4(own), ... so a first-3
        // judge hears exactly one foreign glint whenever dose >= 1,
        // at sigma 0 where nothing is dropped by the ear.
        for dose in 1..=3usize {
            let book = build_book(0.0, dose);
            for t in &book {
                let foreign_in_first3 = t.delivered[..3].iter().filter(|d| d.foreign).count();
                assert_eq!(foreign_in_first3, 1, "dose {dose}");
            }
        }
    }

    #[test]
    fn foreign_kinds_cover_both_reflectors() {
        let mut moth = false;
        let mut stone = false;
        for id in 1..=240u64 {
            match foreign_for(id).0 {
                Kind::Moth => moth = true,
                Kind::Stone => stone = true,
            }
        }
        assert!(moth && stone);
    }

    #[test]
    fn the_book_is_paired_and_deterministic() {
        let b1 = build_book(0.20, 2);
        let b2 = build_book(0.20, 2);
        assert_eq!(b1.len(), b2.len());
        for (t1, t2) in b1.iter().zip(b2.iter()) {
            assert_eq!(t1.kind, t2.kind);
            assert_eq!(t1.delivered.len(), t2.delivered.len());
            for (d1, d2) in t1.delivered.iter().zip(t2.delivered.iter()) {
                assert_eq!(d1.offset, d2.offset);
                assert_eq!(d1.amp, d2.amp);
                assert_eq!(d1.foreign, d2.foreign);
            }
        }
    }

    #[test]
    fn noise_is_zero_at_sigma_zero_and_bounded_elsewhere() {
        for id in 1..50u64 {
            assert_eq!(glint_noise(id, 0, false, 0.0), 0.0);
            for slot in 0..3 {
                for foreign in [false, true] {
                    let n = glint_noise(id, slot, foreign, 0.30);
                    assert!(n.abs() <= 0.30, "noise {n} out of bounds");
                }
            }
        }
    }

    #[test]
    fn jamming_alone_fools_the_greedy_judge_on_clean_evidence() {
        assert!(audit(0.0, 3).greedy.errors() > 0);
    }
}
