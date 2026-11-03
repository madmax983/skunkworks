//! # Scarecrow Field 🌾
//!
//! Interruption equivalence, planted in `experiments/cochlea-cave`:
//! the Cartographer's revived ear (32 band-pass channels, hair
//! cells, LIF neurons) listening through the living FDTD cave of
//! `crates/resonance-audio` (`PhysicsGrid`, 56x36, source at
//! (8, 18), ear at (36, 18), 4,000 drive steps + 2,000 tail steps).
//! The ear machinery below is the host's own, copied faithfully;
//! the cave is the parent crate's, used as provided.
//!
//! The host keeps its state in two places. The ear's state is
//! small and listable: each channel's filter history (x1, x2,
//! y1, y2), hair-cell potential, neuron potential and refractory
//! timer. The field's state is the wave itself: the FDTD update
//! `2u - u_prev + c2 * laplacian(u)` needs TWO grids — the
//! pressure now and the pressure one step ago. The parent crate's
//! public API offers `get` (read the current pressure, cell by
//! cell) and `pluck` (add to the current pressure). It offers no
//! snapshot, and no way to read or write `u_prev` at all.
//!
//! So the checkpoint this planting can take through the public
//! API is: the whole ear, plus the field's present — and not the
//! field's past. The run stops at step 3,000, mid-drive. Three
//! rows are counted on the host's own instrument (spikes per
//! channel, drive/tail split, first spike, peak channel), plus
//! two instruments of this planting: the per-step spike mask
//! (one bit per channel) and the ear's sample each step, so a
//! resumed run can be compared against the uninterrupted run
//! step by step, not just in totals.
//!
//! - **uninterrupted** — the host run, never stopped.
//! - **ear only** — resumed from the ear checkpoint on a fresh,
//!   silent field: everything already in flight is forgotten.
//! - **ear + field now** — resumed from the ear checkpoint and
//!   the field's current pressure, restored cell by cell through
//!   `pluck`; `u_prev` starts at zero because there is no other
//!   choice the API offers.
//! - **ear + field now, past = present** — a fourth row, added
//!   after the first run cracked (see the README's Results):
//!   the still-wave assumption, reached through the public API
//!   alone — restore the present, take one priming step (which
//!   rotates the present into `u_prev`), then re-strike the
//!   present with the per-cell difference.
//!
//! `cargo run -p scarecrow-field -- --headless` prints the
//! ledger. There is no TUI; --headless is handled before
//! anything else.

use anyhow::Result;
use resonance_audio::PhysicsGrid;
use std::f32::consts::PI as PI32;
use std::f64::consts::PI;

// ------------------------------------------------------------------
// The host's clock and geometry (cochlea-cave, unchanged).
// ------------------------------------------------------------------

const SAMPLE_RATE: f32 = 8000.0;
const CHANNELS: usize = 32;
const MIN_FREQ: f32 = 50.0;
const MAX_FREQ: f32 = 3000.0;
const DRIVE: usize = 4000;
const TAIL: usize = 2000;
const STEPS: usize = DRIVE + TAIL;
const CAVE_W: usize = 56;
const CAVE_H: usize = 36;
const SRC: (usize, usize) = (8, 18);
const EAR_POS: (usize, usize) = (36, 18);

/// The winter line: the run stops here, mid-drive, and resumes.
const CHECKPOINT: usize = 3000;

/// The host's published cave ledger for a 1000 Hz tone
/// (cochlea-cave headless): total / drive / tail spikes, first
/// spike step, peak channel.
const HOST_1000: (usize, usize, usize, usize, usize) = (3312, 2347, 965, 96, 23);

// ------------------------------------------------------------------
// The host's ear, copied faithfully from cochlea-cave (the
// corpse's machinery: graveyard/cochlea-sim via the revival).
// Clone added so the ear can be written down at the checkpoint.
// ------------------------------------------------------------------

#[derive(Clone)]
struct BandPassFilter {
    b0: f32, b1: f32, b2: f32, a1: f32, a2: f32,
    x1: f32, x2: f32, y1: f32, y2: f32,
}

impl BandPassFilter {
    fn new(freq: f32, sample_rate: f32, q: f32) -> Self {
        let w0 = 2.0 * PI32 * freq / sample_rate;
        let alpha = w0.sin() / (2.0 * q);
        let cos_w0 = w0.cos();
        let a0 = 1.0 + alpha;
        Self {
            b0: alpha / a0, b1: 0.0, b2: -alpha / a0,
            a1: -2.0 * cos_w0 / a0, a2: (1.0 - alpha) / a0,
            x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0,
        }
    }
    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1; self.x1 = x;
        self.y2 = self.y1; self.y1 = y;
        y
    }
}

#[derive(Clone)]
struct HairCell { potential: f32, smoothing: f32 }

impl HairCell {
    fn new(smoothing: f32) -> Self { Self { potential: 0.0, smoothing } }
    fn process(&mut self, input: f32) -> f32 {
        let rectified = input.max(0.0);
        self.potential = self.potential * self.smoothing + rectified * (1.0 - self.smoothing);
        self.potential
    }
}

#[derive(Clone)]
struct LifNeuron {
    potential: f32, threshold: f32, decay: f32,
    refractory_period: usize, refractory_timer: usize,
}

impl LifNeuron {
    fn new(threshold: f32, decay: f32) -> Self {
        Self { potential: 0.0, threshold, decay, refractory_period: 8, refractory_timer: 0 }
    }
    fn process(&mut self, input_current: f32) -> bool {
        if self.refractory_timer > 0 {
            self.refractory_timer -= 1;
            return false;
        }
        self.potential = self.potential * self.decay + input_current;
        if self.potential >= self.threshold {
            self.potential = 0.0;
            self.refractory_timer = self.refractory_period;
            true
        } else { false }
    }
}

#[derive(Clone)]
struct Channel {
    frequency: f32,
    filter: BandPassFilter,
    hair_cell: HairCell,
    neuron: LifNeuron,
}

impl Channel {
    fn new(freq: f32) -> Self {
        Self {
            frequency: freq,
            filter: BandPassFilter::new(freq, SAMPLE_RATE, 4.0),
            hair_cell: HairCell::new(0.9),
            neuron: LifNeuron::new(0.5, 0.95),
        }
    }
    fn process(&mut self, sample: f32) -> bool {
        let filtered = self.filter.process(sample);
        let potential = self.hair_cell.process(filtered * 5.0);
        self.neuron.process(potential * 5.0)
    }
}

#[derive(Clone)]
struct Cochlea { channels: Vec<Channel> }

impl Cochlea {
    fn new() -> Self {
        let mut channels = Vec::with_capacity(CHANNELS);
        for i in 0..CHANNELS {
            let t = i as f32 / (CHANNELS - 1) as f32;
            let freq = MIN_FREQ * (MAX_FREQ / MIN_FREQ).powf(t);
            channels.push(Channel::new(freq));
        }
        Self { channels }
    }
    fn freq_of(&self, idx: usize) -> f32 { self.channels[idx].frequency }
}

// ------------------------------------------------------------------
// Recording: the host's ledger plus this planting's instruments.
// ------------------------------------------------------------------

#[derive(Default, Clone)]
struct Recording {
    spikes_per_channel: Vec<usize>,
    drive_per_channel: Vec<usize>,
    total_spikes: usize,
    drive_total: usize,
    tail_spikes: usize,
    first_spike_step: Option<usize>,
    /// One bit per channel, per step.
    masks: Vec<u32>,
    /// The sample the ear heard, per step.
    ear_samples: Vec<f32>,
}

impl Recording {
    fn new() -> Self {
        Self {
            spikes_per_channel: vec![0; CHANNELS],
            drive_per_channel: vec![0; CHANNELS],
            ..Default::default()
        }
    }
    fn peak_channel(&self) -> usize {
        self.drive_per_channel.iter().enumerate()
            .max_by_key(|&(_, &c)| c).map(|(i, _)| i).unwrap_or(0)
    }
}

/// What the checkpoint can hold, through the public API only:
/// the whole ear, and the field's current pressure. Not its past.
struct Checkpoint {
    ear: Cochlea,
    field_now: Vec<f32>,
    nonzero_cells: usize,
}

fn tone_sample(freq: f64, step: usize) -> f32 {
    (2.0 * PI * freq * step as f64 / SAMPLE_RATE as f64).sin() as f32
}

fn drive_sample(freqs: &[f64], step: usize) -> f32 {
    if step < DRIVE && !freqs.is_empty() {
        freqs.iter().map(|&f| tone_sample(f, step)).sum::<f32>() * 0.05 / freqs.len() as f32
    } else {
        0.0
    }
}

/// One step of the host's cave run, exactly as cochlea-cave does
/// it: pluck the source, step the field, feed the ear cell to
/// every channel, record.
fn cave_step(grid: &mut PhysicsGrid, ear: &mut Cochlea, rec: &mut Recording, freqs: &[f64], step: usize) {
    if step < DRIVE && !freqs.is_empty() {
        grid.pluck(SRC.0, SRC.1, drive_sample(freqs, step));
    }
    grid.step();
    let sample = grid.get(EAR_POS.0, EAR_POS.1);
    rec.ear_samples.push(sample);
    let mut mask: u32 = 0;
    for (i, ch) in ear.channels.iter_mut().enumerate() {
        if ch.process(sample) {
            mask |= 1 << i;
            rec.spikes_per_channel[i] += 1;
            if step < DRIVE { rec.drive_per_channel[i] += 1; rec.drive_total += 1; }
            rec.total_spikes += 1;
            if rec.first_spike_step.is_none() { rec.first_spike_step = Some(step); }
            if step >= DRIVE { rec.tail_spikes += 1; }
        }
    }
    rec.masks.push(mask);
}

/// The uninterrupted run. Also takes the checkpoint at
/// CHECKPOINT, after that many steps have completed, the way a
/// real stop would find the system.
fn run_uninterrupted(freqs: &[f64]) -> (Recording, Checkpoint) {
    let mut grid = PhysicsGrid::new(CAVE_W, CAVE_H);
    let mut ear = Cochlea::new();
    let mut rec = Recording::new();
    let mut ckpt = None;
    for step in 0..STEPS {
        if step == CHECKPOINT {
            let mut field_now = Vec::with_capacity(CAVE_W * CAVE_H);
            let mut nonzero = 0;
            for y in 0..CAVE_H {
                for x in 0..CAVE_W {
                    let v = grid.get(x, y);
                    if v.abs() > 1e-6 { nonzero += 1; }
                    field_now.push(v);
                }
            }
            ckpt = Some(Checkpoint { ear: ear.clone(), field_now, nonzero_cells: nonzero });
        }
        cave_step(&mut grid, &mut ear, &mut rec, freqs, step);
    }
    (rec, ckpt.expect("checkpoint taken"))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Restore {
    /// Fresh silent field; only the ear resumes.
    EarOnly,
    /// The field's present restored cell by cell through
    /// `pluck`; `u_prev` starts at zero — the API offers no
    /// other choice, and zero is not an absence: the update
    /// reads it as a past that was silent.
    FieldNow,
    /// The field's present restored, and its past set equal to
    /// its present (the still-wave assumption), reached through
    /// the public API alone: restore the present, take one
    /// priming step (which rotates the present into `u_prev`),
    /// then re-strike the present cell by cell with the
    /// difference. The ear is not fed during the priming step;
    /// it is bookkeeping, not hearing.
    FieldNowPastCopied,
}

/// A resumed run, steps CHECKPOINT..STEPS, from the checkpoint.
/// The field is written back into a fresh grid, cell by cell,
/// through `pluck` — the only door the API offers. Nothing in
/// the public API can write `u_prev` directly.
fn run_resumed(freqs: &[f64], ckpt: &Checkpoint, restore: Restore) -> Recording {
    let mut grid = PhysicsGrid::new(CAVE_W, CAVE_H);
    if restore != Restore::EarOnly {
        for y in 0..CAVE_H {
            for x in 0..CAVE_W {
                let v = ckpt.field_now[y * CAVE_W + x];
                if v != 0.0 {
                    grid.pluck(x, y, v);
                }
            }
        }
        if restore == Restore::FieldNowPastCopied {
            grid.step();
            for y in 0..CAVE_H {
                for x in 0..CAVE_W {
                    let want = ckpt.field_now[y * CAVE_W + x];
                    let delta = want - grid.get(x, y);
                    if delta != 0.0 {
                        grid.pluck(x, y, delta);
                    }
                }
            }
        }
    }
    let mut ear = ckpt.ear.clone();
    let mut rec = Recording::new();
    for step in CHECKPOINT..STEPS {
        cave_step(&mut grid, &mut ear, &mut rec, freqs, step);
    }
    rec
}

/// The comparison: first post-checkpoint step whose spike mask
/// differs from the uninterrupted run's, and the ear-sample
/// error over the resumed span.
fn compare(full: &Recording, resumed: &Recording) -> (Option<usize>, f64, f32) {
    let mut first_div = None;
    let mut sq = 0.0f64;
    let mut max_err = 0.0f32;
    for (i, (&fm, &fe)) in full.masks[CHECKPOINT..].iter().zip(full.ear_samples[CHECKPOINT..].iter()).enumerate() {
        let rm = resumed.masks[i];
        if first_div.is_none() && rm != fm {
            first_div = Some(CHECKPOINT + i);
        }
        let e = resumed.ear_samples[i] - fe;
        sq += (e as f64) * (e as f64);
        if e.abs() > max_err { max_err = e.abs(); }
    }
    let rms = (sq / (STEPS - CHECKPOINT) as f64).sqrt();
    (first_div, rms, max_err)
}

fn scenario(freqs: &[f64], label: &str) {
    let (full, ckpt) = run_uninterrupted(freqs);
    let ear_only = run_resumed(freqs, &ckpt, Restore::EarOnly);
    let field_now = run_resumed(freqs, &ckpt, Restore::FieldNow);
    let past_copied = run_resumed(freqs, &ckpt, Restore::FieldNowPastCopied);
    let (div_e, rms_e, max_e) = compare(&full, &ear_only);
    let (div_f, rms_f, max_f) = compare(&full, &field_now);
    let (div_p, rms_p, max_p) = compare(&full, &past_copied);
    let post_full = full.total_spikes - full.masks[..CHECKPOINT].iter().map(|m| m.count_ones() as usize).sum::<usize>();
    println!("  scenario {} {:?} Hz:", label, freqs);
    println!(
        "    uninterrupted: total {}, drive {}, tail {}, first spike {}, peak channel {} ({:.0} Hz)",
        full.total_spikes, full.drive_total, full.tail_spikes,
        full.first_spike_step.map(|v| v.to_string()).unwrap_or_else(|| "none".into()),
        full.peak_channel(), full_peak_freq(full.peak_channel())
    );
    println!(
        "    checkpoint at step {}: ear written down whole ({} channels); field present captured in {} values, {} nonzero cells; field past (u_prev) captured in 0 values — no API",
        CHECKPOINT, CHANNELS, ckpt.field_now.len(), ckpt.nonzero_cells
    );
    println!(
        "    post-checkpoint spikes: uninterrupted {}, ear only {}, ear + field now {}, ear + field now/past copied {}",
        post_full, ear_only.total_spikes, field_now.total_spikes, past_copied.total_spikes
    );
    println!(
        "    ear only:        first divergence {}, post-checkpoint spike diff {:+}, ear-sample RMS err {:.6}, max {:.6}",
        div_e.map(|v| v.to_string()).unwrap_or_else(|| "none".into()),
        ear_only.total_spikes as i64 - post_full as i64, rms_e, max_e
    );
    println!(
        "    ear + field now: first divergence {}, post-checkpoint spike diff {:+}, ear-sample RMS err {:.6}, max {:.6}",
        div_f.map(|v| v.to_string()).unwrap_or_else(|| "none".into()),
        field_now.total_spikes as i64 - post_full as i64, rms_f, max_f
    );
    println!(
        "    ear + field now, past = present: first divergence {}, post-checkpoint spike diff {:+}, ear-sample RMS err {:.6}, max {:.6}",
        div_p.map(|v| v.to_string()).unwrap_or_else(|| "none".into()),
        past_copied.total_spikes as i64 - post_full as i64, rms_p, max_p
    );
    println!(
        "    full-run totals after resume: ear only {} (diff {:+}), ear + field now {} (diff {:+}), past copied {} (diff {:+})",
        full.total_spikes - post_full + ear_only.total_spikes,
        (full.total_spikes - post_full + ear_only.total_spikes) as i64 - full.total_spikes as i64,
        full.total_spikes - post_full + field_now.total_spikes,
        (full.total_spikes - post_full + field_now.total_spikes) as i64 - full.total_spikes as i64,
        full.total_spikes - post_full + past_copied.total_spikes,
        (full.total_spikes - post_full + past_copied.total_spikes) as i64 - full.total_spikes as i64
    );
}

fn full_peak_freq(idx: usize) -> f32 {
    Cochlea::new().freq_of(idx)
}

fn headless() {
    println!("scarecrow-field headless: the ear written down, the field half written down");
    println!(
        "  host: cochlea-cave's ear ({} channels, {:.0}-{:.0} Hz, Q=4) in resonance-audio's cave {}x{}, source {:?}, ear {:?}, drive {} + tail {} steps",
        CHANNELS, MIN_FREQ, MAX_FREQ, CAVE_W, CAVE_H, SRC, EAR_POS, DRIVE, TAIL
    );
    scenario(&[1000.0], "tone");
    scenario(&[150.0, 1000.0], "chord");
    // Host check, on the tone scenario's uninterrupted row.
    let (full, _) = run_uninterrupted(&[1000.0]);
    let held = (full.total_spikes, full.drive_total, full.tail_spikes,
        full.first_spike_step.unwrap_or(0), full.peak_channel()) == HOST_1000;
    println!(
        "  host check (1000 Hz cave, cochlea-cave's published ledger): total {} drive {} tail {} first {} peak {} — {}",
        full.total_spikes, full.drive_total, full.tail_spikes,
        full.first_spike_step.unwrap_or(0), full.peak_channel(),
        if held { "HELD exact" } else { "CRACKED" }
    );
}

fn main() -> Result<()> {
    // --headless handled BEFORE anything else; there is no TUI.
    let _headless = std::env::args().any(|a| a == "--headless");
    headless();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_run_reproduces_cochlea_cave_ledger() {
        let (full, _) = run_uninterrupted(&[1000.0]);
        assert_eq!(
            (full.total_spikes, full.drive_total, full.tail_spikes,
             full.first_spike_step, Some(full.peak_channel())),
            (HOST_1000.0, HOST_1000.1, HOST_1000.2, Some(HOST_1000.3), Some(HOST_1000.4))
        );
    }

    #[test]
    fn checkpoint_field_is_really_in_flight() {
        // At the checkpoint the cave is loud with travelling
        // sound: the captured present must be widely nonzero, or
        // the comparison is vacuous.
        let (_, ckpt) = run_uninterrupted(&[1000.0]);
        assert_eq!(ckpt.field_now.len(), CAVE_W * CAVE_H);
        assert!(ckpt.nonzero_cells > 500, "only {} nonzero cells", ckpt.nonzero_cells);
    }

    #[test]
    fn no_resume_reproduces_the_tone_ledger() {
        // The falsification, pinned: every resume diverges, at
        // the measured step, by the measured count.
        let (full, ckpt) = run_uninterrupted(&[1000.0]);
        let post = full.total_spikes
            - full.masks[..CHECKPOINT].iter().map(|m| m.count_ones() as usize).sum::<usize>();
        assert_eq!(post, 1743);
        let ear_only = run_resumed(&[1000.0], &ckpt, Restore::EarOnly);
        let field_now = run_resumed(&[1000.0], &ckpt, Restore::FieldNow);
        let copied = run_resumed(&[1000.0], &ckpt, Restore::FieldNowPastCopied);
        assert_eq!(compare(&full, &ear_only).0, Some(3007));
        assert_eq!(compare(&full, &field_now).0, Some(3001));
        assert_eq!(compare(&full, &copied).0, Some(3014));
        assert_eq!(ear_only.total_spikes as i64 - post as i64, -413);
        assert_eq!(field_now.total_spikes as i64 - post as i64, -271);
        assert_eq!(copied.total_spikes as i64 - post as i64, 75);
    }

    #[test]
    fn half_a_field_is_worse_than_none_on_the_chord() {
        // The crack, pinned: restoring the present with a
        // silent past invents a strike (+1777 spikes), while
        // restoring nothing lands within 1 of the true count —
        // and the past-copied resume has the smallest sample
        // error of the three.
        let (full, ckpt) = run_uninterrupted(&[150.0, 1000.0]);
        assert_eq!(full.total_spikes, 2440);
        let post = full.total_spikes
            - full.masks[..CHECKPOINT].iter().map(|m| m.count_ones() as usize).sum::<usize>();
        assert_eq!(post, 1208);
        let ear_only = run_resumed(&[150.0, 1000.0], &ckpt, Restore::EarOnly);
        let field_now = run_resumed(&[150.0, 1000.0], &ckpt, Restore::FieldNow);
        let copied = run_resumed(&[150.0, 1000.0], &ckpt, Restore::FieldNowPastCopied);
        assert_eq!(field_now.total_spikes as i64 - post as i64, 1777);
        assert_eq!(ear_only.total_spikes as i64 - post as i64, 1);
        assert_eq!(copied.total_spikes as i64 - post as i64, 87);
        let rms_e = compare(&full, &ear_only).1;
        let rms_f = compare(&full, &field_now).1;
        let rms_p = compare(&full, &copied).1;
        assert!(rms_p < rms_e && rms_e < rms_f);
        assert_eq!(compare(&full, &copied).0, Some(3016));
    }

    #[test]
    fn silence_resumes_as_silence() {
        let (full, ckpt) = run_uninterrupted(&[]);
        assert_eq!(full.total_spikes, 0);
        let resumed = run_resumed(&[], &ckpt, Restore::FieldNow);
        assert_eq!(resumed.total_spikes, 0);
    }
}
