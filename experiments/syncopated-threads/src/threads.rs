use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use crossbeam_channel::Sender;
use crate::audio::AudioCommand;
use crate::model::{ThreadState, RhythmParams, Instrument};

pub fn spawn_rhythm_thread(
    id: usize,
    instrument: Instrument,
    audio_command: AudioCommand,
    state_sender: Sender<(usize, ThreadState)>,
    audio_sender: Sender<AudioCommand>,
    params: RhythmParams,
    running: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while running.load(Ordering::Relaxed) {
            // Notify Waiting (Trying to acquire lock)
            // If lock is free, this state might be very short, visible as a flash in TUI.
            // If lock is contended, it persists.
            let _ = state_sender.send((id, ThreadState::Waiting));

            {
                let _guard = instrument.lock().unwrap();
                // Acquired lock
                let _ = state_sender.send((id, ThreadState::Playing));
                let _ = audio_sender.send(audio_command);
                thread::sleep(params.sustain);
            } // Release lock here

            // Notify Sleeping (Resting)
            let _ = state_sender.send((id, ThreadState::Sleeping));
            thread::sleep(params.rest);
        }
        let _ = state_sender.send((id, ThreadState::Finished));
    })
}
