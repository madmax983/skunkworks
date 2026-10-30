//! # Cochlea Cave 🗺️
//!
//! Revival cross: `graveyard/cochlea-sim` × living
//! `crates/resonance-audio`, brought by The Cartographer of Lost
//! Things.
//!
//! The dead specimen was a complete ear: 32 channels, each a
//! band-pass filter (Q = 4) tuned log-spaced across the basilar
//! range, a hair cell (half-wave rectification + leaky smoothing),
//! and a leaky integrate-and-fire neuron. It was executed for
//! Documentation Void alone — no README, and its `main` went
//! straight to the TUI with no headless bypass, so no ledger of it
//! ever existed. Its metabolism was sound; what it lacked was a
//! *place* to hear. Its only source was a synthetic generator
//! wired straight into the filters — sound with no space in it.
//!
//! The living `resonance-audio` crate donates the place: a 2D
//! FDTD wave cave (the same field the Keeper's moths fly in),
//! where a source must cross air, reflect, and decay before it
//! reaches an ear. This revival puts the dead ear at a listening
//! post inside that cave and asks what the cave does to the
//! ear's tonotopic code:
//!
//! - **direct** — a tone fed straight into the cochlea, the dead
//!   specimen's way: the control.
//! - **cave** — the same tone driven into the FDTD grid at a
//!   source cell, sampled at an ear cell 28 cells away.
//!
//! Two honest adaptations, named here rather than hidden:
//! the dead ear's clock was 44.1 kHz; the cave steps far slower,
//! so the revival runs at a simulated 8,000 steps/sec, rescales
//! the basilar range to 50–3,000 Hz (inside the cave clock's
//! Nyquist), and rescales the neuron's ~1 ms refractory from
//! 44 samples to 8. Filter Q, hair-cell smoothing, thresholds,
//! decays, and the ×5 / ×5 gains are the corpse's own, unchanged.
//!
//! `cargo run -p cochlea-cave -- --headless` prints the ledger.
//! There is no TUI; --headless is handled before anything else.

use anyhow::Result;
use resonance_audio::PhysicsGrid;
use std::f32::consts::PI as PI32;
use std::f64::consts::PI;

// ------------------------------------------------------------------
// The cave's clock and the ear's range on it.
// ------------------------------------------------------------------

const SAMPLE_RATE: f32 = 8000.0;
const CHANNELS: usize = 32;
const MIN_FREQ: f32 = 50.0;
const MAX_FREQ: f32 = 3000.0;

/// Drive steps per scenario (0.5 s at the cave clock), then the
/// source stops and we listen to the tail for TAIL steps.
const DRIVE: usize = 4000;
const TAIL: usize = 2000;

// Cave geometry: source west, ear east, open air between.
const CAVE_W: usize = 56;
const CAVE_H: usize = 36;
const SRC: (usize, usize) = (8, 18);
const EAR_POS: (usize, usize) = (36, 18);

// ------------------------------------------------------------------
// The corpse's machinery, ported faithfully (graveyard/cochlea-sim).
// ------------------------------------------------------------------

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

struct HairCell { potential: f32, smoothing: f32 }

impl HairCell {
    fn new(smoothing: f32) -> Self { Self { potential: 0.0, smoothing } }
    fn process(&mut self, input: f32) -> f32 {
        let rectified = input.max(0.0);
        self.potential = self.potential * self.smoothing + rectified * (1.0 - self.smoothing);
        self.potential
    }
}

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
// Scenarios
// ------------------------------------------------------------------

#[derive(Default, Clone)]
struct Hearing {
    spikes_per_channel: Vec<usize>,
    drive_per_channel: Vec<usize>,
    total_spikes: usize,
    first_spike_step: Option<usize>,
    tail_spikes: usize,
    drive_total: usize,
}

impl Hearing {
    fn peak_channel(&self) -> usize {
        self.drive_per_channel.iter().enumerate()
            .max_by_key(|&(_, &c)| c).map(|(i, _)| i).unwrap_or(0)
    }
    fn concentration(&self) -> f64 {
        if self.drive_total == 0 { return 0.0; }
        let p = self.peak_channel();
        let lo = p.saturating_sub(1);
        let hi = (p + 1).min(self.drive_per_channel.len() - 1);
        let near: usize = self.drive_per_channel[lo..=hi].iter().sum();
        near as f64 / self.drive_total as f64
    }
    fn window_spikes(&self, centre: usize) -> usize {
        let lo = centre.saturating_sub(1);
        let hi = (centre + 1).min(self.drive_per_channel.len() - 1);
        self.drive_per_channel[lo..=hi].iter().sum()
    }
}

fn tone_sample(freq: f64, step: usize) -> f32 {
    (2.0 * PI * freq * step as f64 / SAMPLE_RATE as f64).sin() as f32
}

fn run_direct(freqs: &[f64]) -> Hearing {
    let mut cochlea = Cochlea::new();
    let mut h = Hearing { spikes_per_channel: vec![0; CHANNELS], drive_per_channel: vec![0; CHANNELS], ..Default::default() };
    for step in 0..(DRIVE + TAIL) {
        let s: f32 = if step < DRIVE && !freqs.is_empty() {
            freqs.iter().map(|&f| tone_sample(f, step)).sum::<f32>() * 0.08 / freqs.len() as f32
        } else { 0.0 };
        for (i, ch) in cochlea.channels.iter_mut().enumerate() {
            if ch.process(s) {
                h.spikes_per_channel[i] += 1;
                if step < DRIVE { h.drive_per_channel[i] += 1; }
                h.total_spikes += 1;
                if h.first_spike_step.is_none() { h.first_spike_step = Some(step); }
                if step >= DRIVE { h.tail_spikes += 1; }
            }
        }
    }
    h.drive_total = h.total_spikes - h.tail_spikes;
    h
}

fn run_cave(freqs: &[f64]) -> Hearing {
    let mut grid = PhysicsGrid::new(CAVE_W, CAVE_H);
    let mut cochlea = Cochlea::new();
    let mut h = Hearing { spikes_per_channel: vec![0; CHANNELS], drive_per_channel: vec![0; CHANNELS], ..Default::default() };
    for step in 0..(DRIVE + TAIL) {
        if step < DRIVE && !freqs.is_empty() {
            let s: f32 = freqs.iter().map(|&f| tone_sample(f, step)).sum::<f32>()
                * 0.05 / freqs.len() as f32;
            grid.pluck(SRC.0, SRC.1, s);
        }
        grid.step();
        let ear = grid.get(EAR_POS.0, EAR_POS.1);
        for (i, ch) in cochlea.channels.iter_mut().enumerate() {
            if ch.process(ear) {
                h.spikes_per_channel[i] += 1;
                if step < DRIVE { h.drive_per_channel[i] += 1; }
                h.total_spikes += 1;
                if h.first_spike_step.is_none() { h.first_spike_step = Some(step); }
                if step >= DRIVE { h.tail_spikes += 1; }
            }
        }
    }
    h.drive_total = h.total_spikes - h.tail_spikes;
    h
}

fn report(label: &str, freqs: &[f64], h: &Hearing, cochlea_freq: &[f32]) {
    let p = h.peak_channel();
    // top-3 channels by drive-window spikes approximated from totals
    let mut idx: Vec<usize> = (0..CHANNELS).collect();
    idx.sort_by_key(|&i| std::cmp::Reverse(h.drive_per_channel[i]));
    println!(
        "  {} {:?} Hz: total spikes {}, drive {}, tail {}, first spike at step {}, peak channel {} ({:.0} Hz), concentration(peak±1, drive) {:.3}, top channels {:?}",
        label, freqs, h.total_spikes, h.drive_total, h.tail_spikes,
        h.first_spike_step.map(|v| v.to_string()).unwrap_or_else(|| "none".into()), p, cochlea_freq[p], h.concentration(),
        &idx[..3.min(idx.len())]
    );
}

fn headless() {
    let probe = Cochlea::new();
    let freqs: Vec<f32> = (0..CHANNELS).map(|i| probe.freq_of(i)).collect();
    println!("cochlea-cave headless: a dead ear, re-housed in a living cave");
    println!(
        "  ear: {} channels, log-spaced {:.0}-{:.0} Hz, Q=4.0, hair-cell smoothing 0.9, LIF threshold 0.5 decay 0.95 refractory 8 steps, sample rate {:.0} steps/sec",
        CHANNELS, MIN_FREQ, MAX_FREQ, SAMPLE_RATE
    );
    println!(
        "  cave: {}x{} FDTD, source {:?}, ear {:?} ({} cells apart), drive {} steps, tail {} steps",
        CAVE_W, CAVE_H, SRC, EAR_POS, EAR_POS.0 - SRC.0, DRIVE, TAIL
    );
    for &f in &[150.0_f64, 400.0, 1000.0] {
        let d = run_direct(&[f]);
        report("direct", &[f], &d, &freqs);
        let c = run_cave(&[f]);
        report("cave  ", &[f], &c, &freqs);
    }
    let chord_d = run_direct(&[150.0, 1000.0]);
    report("direct chord", &[150.0, 1000.0], &chord_d, &freqs);
    let chord_c = run_cave(&[150.0, 1000.0]);
    report("cave chord  ", &[150.0, 1000.0], &chord_c, &freqs);
    println!(
        "  chord humps (drive spikes within ±1 of each single-tone peak): direct 150-peak {} / 1000-peak {}; cave 150-peak {} / 1000-peak {}",
        chord_d.window_spikes(8), chord_d.window_spikes(23),
        chord_c.window_spikes(8), chord_c.window_spikes(23)
    );
    let silence = run_cave(&[]);
    println!(
        "  cave silence: total spikes {}, first spike {}",
        silence.total_spikes,
        silence.first_spike_step.map(|v| v.to_string()).unwrap_or_else(|| "none".into())
    );
    // The comparisons the experiment exists to make
    let d150 = run_direct(&[150.0]); let c150 = run_cave(&[150.0]);
    let d1000 = run_direct(&[1000.0]); let c1000 = run_cave(&[1000.0]);
    println!(
        "  tonotopy: peak channel 150 Hz direct {} -> cave {}; 1000 Hz direct {} -> cave {}",
        d150.peak_channel(), c150.peak_channel(), d1000.peak_channel(), c1000.peak_channel()
    );
    println!(
        "  onset delay (first spike, cave - direct): 150 Hz {} steps, 400 Hz {} steps, 1000 Hz {} steps",
        c150.first_spike_step.unwrap() - d150.first_spike_step.unwrap(),
        run_cave(&[400.0]).first_spike_step.unwrap() - run_direct(&[400.0]).first_spike_step.unwrap(),
        c1000.first_spike_step.unwrap() - d1000.first_spike_step.unwrap()
    );
    println!(
        "  reverb tail (1000 Hz): direct tail spikes {}, cave tail spikes {}",
        d1000.tail_spikes, c1000.tail_spikes
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
    fn channels_are_log_spaced_and_monotonic() {
        let c = Cochlea::new();
        assert_eq!(c.channels.len(), 32);
        assert!((c.freq_of(0) - 50.0).abs() < 0.01);
        assert!((c.freq_of(31) - 3000.0).abs() < 1.0);
        for i in 1..32 { assert!(c.freq_of(i) > c.freq_of(i - 1)); }
        let ratio = c.freq_of(1) / c.freq_of(0);
        for i in 2..32 {
            assert!((c.freq_of(i) / c.freq_of(i - 1) - ratio).abs() < 0.01);
        }
    }

    #[test]
    fn hair_cell_rectifies() {
        let mut hc = HairCell::new(0.9);
        let neg = hc.process(-3.0);
        assert_eq!(neg, 0.0);
        let pos = hc.process(1.0);
        assert!(pos > 0.0 && pos < 1.0);
    }

    #[test]
    fn silence_spikes_nothing() {
        let h = run_cave(&[]);
        assert_eq!(h.total_spikes, 0);
        let d = run_direct(&[]);
        assert_eq!(d.total_spikes, 0);
    }

    #[test]
    fn tonotopy_peak_rises_with_frequency_direct() {
        let lo = run_direct(&[150.0]);
        let mid = run_direct(&[400.0]);
        let hi = run_direct(&[1000.0]);
        assert!(lo.total_spikes > 0 && mid.total_spikes > 0 && hi.total_spikes > 0);
        assert!(lo.peak_channel() < mid.peak_channel());
        assert!(mid.peak_channel() < hi.peak_channel());
    }

    #[test]
    fn cave_preserves_tonotopic_order() {
        let lo = run_cave(&[150.0]);
        let hi = run_cave(&[1000.0]);
        assert!(lo.total_spikes > 0 && hi.total_spikes > 0);
        assert!(lo.peak_channel() < hi.peak_channel());
    }

    #[test]
    fn cave_delays_first_spike() {
        let d = run_direct(&[1000.0]);
        let c = run_cave(&[1000.0]);
        assert!(c.first_spike_step.unwrap() > d.first_spike_step.unwrap());
    }

    #[test]
    fn chord_has_two_humps_in_cave() {
        let h = run_cave(&[150.0, 1000.0]);
        let lo = run_cave(&[150.0]).peak_channel();
        let hi = run_cave(&[1000.0]).peak_channel();
        // spikes near both single-tone peaks must be substantial
        assert!(h.window_spikes(lo) > 20);
        assert!(h.window_spikes(hi) > 20);
    }
}
