use crate::audio::AudioCommand;
use crate::model::{Instrument, RhythmParams, ThreadState};
use crossbeam_channel::Sender;

#[cfg(not(loom))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
#[cfg(not(loom))]
use std::thread;

#[cfg(loom)]
use loom::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
#[cfg(loom)]
use loom::thread;

pub fn spawn_rhythm_thread_secondary(
    id: usize,
    primary_instrument: Instrument,
    secondary_instrument: Option<Instrument>,
    audio_command: AudioCommand,
    state_sender: Sender<(usize, ThreadState)>,
    audio_sender: Sender<AudioCommand>,
    params: RhythmParams,
    running: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while running.load(Ordering::Relaxed) {
            let _ = state_sender.send((id, ThreadState::Waiting));
            {
                let _guard1 = primary_instrument.lock().unwrap();
                let _guard2 = secondary_instrument.as_ref().map(|i: &Instrument| i.lock().unwrap());
                let _ = state_sender.send((id, ThreadState::Playing));
                let _ = audio_sender.send(audio_command);
                #[cfg(not(loom))]
                std::thread::sleep(params.sustain);
                #[cfg(loom)]
                thread::yield_now();
            }
            let _ = state_sender.send((id, ThreadState::Sleeping));
            #[cfg(not(loom))]
            std::thread::sleep(params.rest);
            #[cfg(loom)]
            thread::yield_now();
        }
        let _ = state_sender.send((id, ThreadState::Finished));
    })
}
