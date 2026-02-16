use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Noise,
}

pub struct Oscillator {
    pub frequency: f32,
    pub sample_rate: f32,
    pub phase: f32,
    pub waveform: Waveform,
}

impl Oscillator {
    pub fn new(frequency: f32, sample_rate: f32, waveform: Waveform) -> Self {
        Self {
            frequency,
            sample_rate,
            phase: 0.0,
            waveform,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let value = match self.waveform {
            Waveform::Sine => (self.phase * 2.0 * PI).sin(),
            Waveform::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
        };

        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        value
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    pub attack: f32,  // seconds
    pub decay: f32,   // seconds
    pub sustain: f32, // level (0.0 - 1.0)
    pub release: f32, // seconds
    pub sample_rate: f32,

    pub state: EnvelopeState,
    pub level: f32,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            sample_rate,
            state: EnvelopeState::Idle,
            level: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvelopeState::Attack;
        self.level = 0.0;
    }

    pub fn release(&mut self) {
        self.state = EnvelopeState::Release;
    }

    pub fn next_sample(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        match self.state {
            EnvelopeState::Idle => {
                self.level = 0.0;
            }
            EnvelopeState::Attack => {
                self.level += dt / self.attack;
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.state = EnvelopeState::Decay;
                }
            }
            EnvelopeState::Decay => {
                self.level -= dt / self.decay * (1.0 - self.sustain);
                if self.level <= self.sustain {
                    self.level = self.sustain;
                    self.state = EnvelopeState::Sustain;
                }
            }
            EnvelopeState::Sustain => {
                self.level = self.sustain;
            }
            EnvelopeState::Release => {
                self.level -= dt / self.release;
                if self.level <= 0.0 {
                    self.level = 0.0;
                    self.state = EnvelopeState::Idle;
                }
            }
        }

        self.level
    }

    pub fn is_active(&self) -> bool {
        self.state != EnvelopeState::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator_sine_wave() {
        let mut osc = Oscillator::new(440.0, 44100.0, Waveform::Sine);

        let sample1 = osc.next_sample();
        let sample2 = osc.next_sample();

        // Sine wave should move from 0.0 to a positive value
        assert!(sample2 > sample1);
        assert_ne!(sample2, 0.0);
    }

    #[test]
    fn test_envelope_attack() {
        let mut env = Envelope::new(0.1, 0.1, 0.5, 0.1, 100.0);
        env.trigger();
        assert_eq!(env.state, EnvelopeState::Attack);

        let val1 = env.next_sample();
        assert!(val1 > 0.0);

        // Advance 1 second (should be well past attack)
        for _ in 0..100 {
            env.next_sample();
        }

        assert_eq!(env.state, EnvelopeState::Sustain);
    }
}
