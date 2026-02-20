use crate::model::{AudioCommand, Instrument, ThreadState};
use chimera_lang::vm::ChimeraVM;
use crossbeam_channel::Sender;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

pub struct Musician {
    id: usize,
    vm: ChimeraVM,
    instruments: Vec<Instrument>,
    audio_sender: Sender<AudioCommand>,
    state_sender: Sender<(usize, ThreadState)>,
    running: Arc<AtomicBool>,
}

enum Command {
    Play { inst: usize, dur: u64 },
    Sleep { dur: u64 },
}

impl Musician {
    pub fn new(
        id: usize,
        vm: ChimeraVM,
        instruments: Vec<Instrument>,
        audio_sender: Sender<AudioCommand>,
        state_sender: Sender<(usize, ThreadState)>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            id,
            vm,
            instruments,
            audio_sender,
            state_sender,
            running,
        }
    }

    pub fn run(mut self) {
        while self.running.load(Ordering::Relaxed) {
            // Step VM
            self.vm.step();

            // Check output
            let output = std::mem::take(&mut self.vm.output);
            for line in output {
                if let Some(cmd) = self.parse_command(&line) {
                    match cmd {
                        Command::Play { inst, dur } => self.play(inst, dur),
                        Command::Sleep { dur } => {
                            let _ = self.state_sender.send((self.id, ThreadState::Sleeping));
                            thread::sleep(Duration::from_millis(dur));
                        }
                    }
                }
            }

            // Safety sleep to prevent 100% CPU on tight loops if VM halts or is fast
            if self.vm.halted {
                break;
            }
            thread::sleep(Duration::from_micros(100));
        }
        let _ = self.state_sender.send((self.id, ThreadState::Finished));
    }

    fn parse_command(&self, line: &str) -> Option<Command> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "PLAY" if parts.len() == 3 => {
                let inst = parts[1].parse().ok()?;
                let dur = parts[2].parse().ok()?;
                Some(Command::Play { inst, dur })
            }
            "SLEEP" if parts.len() == 2 => {
                let dur = parts[1].parse().ok()?;
                Some(Command::Sleep { dur })
            }
            _ => None,
        }
    }

    fn play(&mut self, inst_idx: usize, duration: u64) {
        if inst_idx >= self.instruments.len() {
            return;
        }

        let instrument = &self.instruments[inst_idx];

        // Try to lock
        match instrument.try_lock() {
            Ok(_guard) => {
                // Acquired!
                let _ = self.state_sender.send((self.id, ThreadState::Playing));
                let _ = self.audio_sender.send(AudioCommand::Play(inst_idx));

                // Sustain
                thread::sleep(Duration::from_millis(duration));

                // Reward
                self.vm.energy += 5;
            }
            Err(_) => {
                // Contention
                let _ = self.state_sender.send((self.id, ThreadState::Waiting));
                // Briefly show red state
                thread::sleep(Duration::from_millis(20));
            }
        }
        // Guard dropped here
    }
}
